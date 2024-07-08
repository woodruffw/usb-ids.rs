use std::env;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use phf_codegen::Map;
use quote::quote;

/* This build script contains a "parser" for the USB ID database.
 * "Parser" is in scare-quotes because it's really a line matcher with a small amount
 * of context needed for pairing nested entities (e.g. devices) with their parents (e.g. vendors).
 */

// these are the definitions for the generated maps that will be written to the source file
const VENDOR_PROLOGUE: &str = "static USB_IDS: phf::Map<u16, Vendor> = ";
const CLASS_PROLOGUE: &str = "static USB_CLASSES: phf::Map<u8, Class> = ";
const AUDIO_TERMINAL_PROLOGUE: &str = "static USB_AUDIO_TERMINALS: phf::Map<u16, AudioTerminal> = ";
const HID_ID_PROLOGUE: &str = "static USB_HID_IDS: phf::Map<u8, Hid> = ";
const HID_R_PROLOGUE: &str = "static USB_HID_R_TYPES: phf::Map<u8, HidItemType> = ";
const BIAS_PROLOGUE: &str = "static USB_BIASES: phf::Map<u8, Bias> = ";
const PHY_PROLOGUE: &str = "static USB_PHYS: phf::Map<u8, Phy> = ";
const HUT_PROLOGUE: &str = "static USB_HUTS: phf::Map<u8, HidUsagePage> = ";
const LANG_PROLOGUE: &str = "static USB_LANGS: phf::Map<u16, Language> = ";
const HID_CC_PROLOGUE: &str = "static USB_HID_CCS: phf::Map<u8, HidCountryCode> = ";
const TERMINAL_PROLOGUE: &str = "static USB_VIDEO_TERMINALS: phf::Map<u16, VideoTerminal> = ";

trait CgEntry<T> {
    fn id(&self) -> T;
}

struct CgVendor {
    id: u16,
    name: String,
    devices: Vec<CgDevice>,
}

struct CgDevice {
    id: u16,
    name: String,
    interfaces: Vec<CgInterface>,
}

struct CgClass {
    id: u8,
    name: String,
    sub_classes: Vec<CgSubClass>,
}

type CgSubClass = CgParentType<u8, CgProtocol>;

struct CgParentType<T, C> {
    id: T,
    name: String,
    children: Vec<C>,
}

impl<T: Copy, C: CgEntry<T>> CgEntry<T> for CgParentType<T, C> {
    fn id(&self) -> T {
        self.id
    }
}

struct CgType<T> {
    id: T,
    name: String,
}

impl<T: Copy> CgEntry<T> for CgType<T> {
    fn id(&self) -> T {
        self.id
    }
}

type CgInterface = CgType<u8>;
type CgProtocol = CgType<u8>;
type CgAtType = CgType<u16>;
type CgHidType = CgType<u8>;
type CgRType = CgType<u8>;
type CgRBiasType = CgType<u8>;
type CgPhyType = CgType<u8>;
type CgHidUsage = CgType<u16>;
type CgHut = CgParentType<u8, CgHidUsage>;
type CgDialect = CgType<u8>;
type CgLang = CgParentType<u16, CgDialect>;
type CgCountryCode = CgType<u8>;
type CgTerminalType = CgType<u16>;

/// Parser state parses only the type for the current section, this is because some
/// parsers are ambiguous without context; device.interface == subclass.protocol for example.
enum ParserState {
    Vendors(Map<u16>, Option<CgVendor>, u16),
    Classes(Map<u8>, Option<CgClass>, u8),
    AtType(Map<u16>, Option<CgAtType>),
    HidType(Map<u8>, Option<CgHidType>),
    RType(Map<u8>, Option<CgRType>),
    BiasType(Map<u8>, Option<CgRBiasType>),
    PhyType(Map<u8>, Option<CgPhyType>),
    HutType(Map<u8>, Option<CgHut>),
    Lang(Map<u16>, Option<CgLang>),
    CountryCode(Map<u8>, Option<CgCountryCode>),
    TerminalType(Map<u16>, Option<CgTerminalType>),
}

impl ParserState {
    /// Return the prologue string for the current state; the type definition
    fn prologue_str(&self) -> &'static str {
        match self {
            ParserState::Vendors(_, _, _) => VENDOR_PROLOGUE,
            ParserState::Classes(_, _, _) => CLASS_PROLOGUE,
            ParserState::AtType(_, _) => AUDIO_TERMINAL_PROLOGUE,
            ParserState::HidType(_, _) => HID_ID_PROLOGUE,
            ParserState::RType(_, _) => HID_R_PROLOGUE,
            ParserState::BiasType(_, _) => BIAS_PROLOGUE,
            ParserState::PhyType(_, _) => PHY_PROLOGUE,
            ParserState::HutType(_, _) => HUT_PROLOGUE,
            ParserState::Lang(_, _) => LANG_PROLOGUE,
            ParserState::CountryCode(_, _) => HID_CC_PROLOGUE,
            ParserState::TerminalType(_, _) => TERMINAL_PROLOGUE,
        }
    }

    /// Emit any pending entries to the map
    fn emit(&mut self) {
        match self {
            ParserState::Vendors(m, Some(vendor), _) => {
                m.entry(vendor.id, &quote!(#vendor).to_string());
            }
            ParserState::Classes(m, Some(class), _) => {
                m.entry(class.id, &quote!(#class).to_string());
            }
            ParserState::AtType(m, Some(t)) | ParserState::TerminalType(m, Some(t)) => {
                m.entry(t.id(), &quote!(#t).to_string());
            }
            ParserState::HidType(m, Some(t))
            | ParserState::RType(m, Some(t))
            | ParserState::BiasType(m, Some(t))
            | ParserState::CountryCode(m, Some(t))
            | ParserState::PhyType(m, Some(t)) => {
                m.entry(t.id(), &quote!(#t).to_string());
            }
            ParserState::HutType(m, Some(t)) => {
                m.entry(t.id, &quote!(#t).to_string());
            }
            ParserState::Lang(m, Some(t)) => {
                m.entry(t.id, &quote!(#t).to_string());
            }
            _ => {}
        }
    }

    /// Detects the next state based on the header line
    ///
    /// Not very efficient but since it only checks # lines and required length it is not terrible
    fn next_from_header(&mut self, line: &str, output: &mut impl Write) -> Option<ParserState> {
        if line.len() < 7 || !line.starts_with('#') {
            return None;
        }

        match &line[..7] {
            "# C cla" => {
                self.finalize(output);
                Some(ParserState::Classes(Map::<u8>::new(), None, 0u8))
            }
            "# AT te" => {
                self.finalize(output);
                Some(ParserState::AtType(Map::<u16>::new(), None))
            }
            "# HID d" => {
                self.finalize(output);
                Some(ParserState::HidType(Map::<u8>::new(), None))
            }
            "# R ite" => {
                self.finalize(output);
                Some(ParserState::RType(Map::<u8>::new(), None))
            }
            "# BIAS " => {
                self.finalize(output);
                Some(ParserState::BiasType(Map::<u8>::new(), None))
            }
            "# PHY i" => {
                self.finalize(output);
                Some(ParserState::PhyType(Map::<u8>::new(), None))
            }
            "# HUT h" => {
                self.finalize(output);
                Some(ParserState::HutType(Map::<u8>::new(), None))
            }
            "# L lan" => {
                self.finalize(output);
                Some(ParserState::Lang(Map::<u16>::new(), None))
            }
            "# HCC c" => {
                self.finalize(output);
                Some(ParserState::CountryCode(Map::<u8>::new(), None))
            }
            "# VT te" => {
                self.finalize(output);
                Some(ParserState::TerminalType(Map::<u16>::new(), None))
            }
            _ => None,
        }
    }

    /// Process a line of input for the current state
    fn process(&mut self, line: &str) {
        if line.is_empty() || line.starts_with('#') {
            return;
        }

        // Switch parser state based on line prefix and current state
        // this relies on ordering of classes and types in the file...
        match self {
            ParserState::Vendors(m, ref mut curr_vendor, ref mut curr_device_id) => {
                if let Ok((name, id)) = parser::vendor(line) {
                    if let Some(cv) = curr_vendor {
                        m.entry(cv.id, &quote!(#cv).to_string());
                    }

                    // Set our new vendor as the current vendor.
                    *curr_vendor = Some(CgVendor {
                        id,
                        name: name.into(),
                        devices: vec![],
                    });
                // We should always have a current vendor; failure here indicates a malformed input.
                } else {
                    let curr_vendor = curr_vendor
                        .as_mut()
                        .expect("No parent vendor whilst parsing vendors");
                    if let Ok((name, id)) = parser::device(line) {
                        curr_vendor.devices.push(CgDevice {
                            id,
                            name: name.into(),
                            interfaces: vec![],
                        });
                        *curr_device_id = id;
                    } else if let Ok((name, id)) = parser::interface(line) {
                        let curr_device = curr_vendor
                            .devices
                            .iter_mut()
                            .find(|d| d.id == *curr_device_id)
                            .expect("No parent device whilst parsing interfaces");

                        curr_device.interfaces.push(CgInterface {
                            id,
                            name: name.into(),
                        });
                    }
                }
            }
            ParserState::Classes(m, ref mut curr_class, ref mut curr_class_id) => {
                if let Ok((name, id)) = parser::class(line) {
                    if let Some(cv) = curr_class {
                        m.entry(cv.id, &quote!(#cv).to_string());
                    }

                    // Set our new class as the current class.
                    *curr_class = Some(CgClass {
                        id,
                        name: name.into(),
                        sub_classes: vec![],
                    });
                } else {
                    let curr_class = curr_class
                        .as_mut()
                        .expect("No parent class whilst parsing classes");
                    if let Ok((name, id)) = parser::sub_class(line) {
                        curr_class.sub_classes.push(CgSubClass {
                            id,
                            name: name.into(),
                            children: vec![],
                        });
                        *curr_class_id = id;
                    } else if let Ok((name, id)) = parser::protocol(line) {
                        let curr_device = curr_class
                            .sub_classes
                            .iter_mut()
                            .find(|d| d.id == *curr_class_id)
                            .expect("No parent sub-class whilst parsing protocols");

                        curr_device.children.push(CgProtocol {
                            id,
                            name: name.into(),
                        });
                    }
                }
            }
            ParserState::AtType(m, ref mut current) => {
                let (name, id) =
                    parser::audio_terminal_type(line).expect("Invalid audio terminal line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgAtType {
                    id,
                    name: name.into(),
                });
            }
            ParserState::HidType(m, ref mut current) => {
                let (name, id) = parser::hid_type(line).expect("Invalid hid type line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgHidType {
                    id,
                    name: name.into(),
                });
            }
            ParserState::RType(m, ref mut current) => {
                let (name, id) = parser::hid_item_type(line).expect("Invalid hid item type line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgRType {
                    id,
                    name: name.into(),
                });
            }
            ParserState::BiasType(m, ref mut current) => {
                let (name, id) = parser::bias_type(line).expect("Invalid bias type line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgRBiasType {
                    id,
                    name: name.into(),
                });
            }
            ParserState::PhyType(m, ref mut current) => {
                let (name, id) = parser::phy_type(line).expect("Invalid phy type line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgPhyType {
                    id,
                    name: name.into(),
                });
            }
            ParserState::HutType(m, ref mut current) => {
                if let Ok((name, id)) = parser::hut_type(line) {
                    if let Some(cv) = current {
                        m.entry(cv.id, &quote!(#cv).to_string());
                    }

                    // Set our new class as the current class.
                    *current = Some(CgHut {
                        id,
                        name: name.into(),
                        children: vec![],
                    });
                } else {
                    let curr_hut = current.as_mut().expect("No parent hut whilst parsing huts");
                    if let Ok((name, id)) = parser::hid_usage_name(line) {
                        curr_hut.children.push(CgHidUsage {
                            id,
                            name: name.into(),
                        });
                    }
                }
            }
            ParserState::Lang(m, ref mut current) => {
                if let Ok((name, id)) = parser::language(line) {
                    if let Some(cv) = current {
                        m.entry(cv.id, &quote!(#cv).to_string());
                    }

                    // Set our new class as the current class.
                    *current = Some(CgLang {
                        id,
                        name: name.into(),
                        children: vec![],
                    });
                } else {
                    let curr_lang = current
                        .as_mut()
                        .expect("No parent lang whilst parsing langs");
                    if let Ok((name, id)) = parser::dialect(line) {
                        curr_lang.children.push(CgDialect {
                            id,
                            name: name.into(),
                        });
                    }
                }
            }
            ParserState::CountryCode(m, ref mut current) => {
                let (name, id) = parser::country_code(line).expect("Invalid country code line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgCountryCode {
                    id,
                    name: name.into(),
                });
            }
            ParserState::TerminalType(m, ref mut current) => {
                let (name, id) = parser::terminal_type(line).expect("Invalid terminal type line");
                if let Some(cv) = current {
                    m.entry(cv.id, &quote!(#cv).to_string());
                }

                // Set our new class as the current class.
                *current = Some(CgTerminalType {
                    id,
                    name: name.into(),
                });
            }
        }
    }

    /// Emit the prologue and map to the output file.
    ///
    /// Should only be called once per state, used before switching.
    fn finalize(&mut self, output: &mut impl Write) {
        // Emit any pending contained within
        self.emit();

        // Write the prologue
        writeln!(output, "{}", self.prologue_str()).unwrap();

        // And the map itself
        match self {
            ParserState::Vendors(m, _, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
            ParserState::Classes(m, _, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
            ParserState::AtType(m, _) | ParserState::TerminalType(m, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
            ParserState::HidType(m, _)
            | ParserState::RType(m, _)
            | ParserState::BiasType(m, _)
            | ParserState::CountryCode(m, _)
            | ParserState::PhyType(m, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
            ParserState::HutType(m, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
            ParserState::Lang(m, _) => {
                writeln!(output, "{};", m.build()).unwrap();
            }
        }
    }

    /// Return the next state for the current state based on the standard ordering of the file
    ///
    /// Not as robust as the next_from_header but at lot less overhead. The issue is reliably detecting the end of a section; # comments are not reliable as there are some '# typo?' strings
    #[allow(dead_code)]
    fn next(&mut self, output: &mut impl Write) -> Option<ParserState> {
        self.finalize(output);
        match self {
            ParserState::Vendors(_, _, _) => {
                Some(ParserState::Classes(Map::<u8>::new(), None, 0u8))
            }
            ParserState::Classes(_, _, _) => Some(ParserState::AtType(Map::<u16>::new(), None)),
            ParserState::AtType(_, _) => Some(ParserState::HidType(Map::<u8>::new(), None)),
            ParserState::HidType(_, _) => Some(ParserState::RType(Map::<u8>::new(), None)),
            ParserState::RType(_, _) => Some(ParserState::BiasType(Map::<u8>::new(), None)),
            ParserState::BiasType(_, _) => Some(ParserState::PhyType(Map::<u8>::new(), None)),
            ParserState::PhyType(_, _) => Some(ParserState::HutType(Map::<u8>::new(), None)),
            ParserState::HutType(_, _) => Some(ParserState::Lang(Map::<u16>::new(), None)),
            ParserState::Lang(_, _) => Some(ParserState::CountryCode(Map::<u8>::new(), None)),
            ParserState::CountryCode(_, _) => {
                Some(ParserState::TerminalType(Map::<u16>::new(), None))
            }
            ParserState::TerminalType(_, _) => None,
        }
    }
}

#[allow(clippy::redundant_field_names)]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src_path = Path::new("src/usb.ids");
    let dest_path = Path::new(&out_dir).join("usb_ids.cg.rs");
    let input = {
        let f = fs::File::open(src_path).unwrap();
        BufReader::new(f)
    };
    let mut output = {
        let f = fs::File::create(dest_path).unwrap();
        BufWriter::new(f)
    };

    // Parser state machine starts with vendors (first in file)
    let mut parser_state: ParserState = ParserState::Vendors(Map::<u16>::new(), None, 0u16);

    #[allow(clippy::lines_filter_map_ok)]
    for line in input.lines().flatten() {
        // Check for a state change based on the header comments
        if let Some(next_state) = parser_state.next_from_header(&line, &mut output) {
            parser_state = next_state;
        }

        // Process line for current parser
        parser_state.process(&line);
    }

    // Last call for last parser in file
    parser_state.finalize(&mut output);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/usb.ids");
}

mod parser {
    use std::num::ParseIntError;

    use nom::bytes::complete::{tag, take};
    use nom::character::complete::{hex_digit1, tab};
    use nom::combinator::{all_consuming, map_parser, map_res};
    use nom::sequence::{delimited, terminated};
    use nom::IResult;

    fn id<T, F>(size: usize, from_str_radix: F) -> impl Fn(&str) -> IResult<&str, T>
    where
        F: Fn(&str, u32) -> Result<T, ParseIntError>,
    {
        move |input| {
            map_res(map_parser(take(size), all_consuming(hex_digit1)), |input| {
                from_str_radix(input, 16)
            })(input)
        }
    }

    pub fn vendor(input: &str) -> IResult<&str, u16> {
        let id = id(4, u16::from_str_radix);
        terminated(id, tag("  "))(input)
    }

    pub fn device(input: &str) -> IResult<&str, u16> {
        let id = id(4, u16::from_str_radix);
        delimited(tab, id, tag("  "))(input)
    }

    pub fn interface(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("\t\t"), id, tag("  "))(input)
    }

    pub fn class(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("C "), id, tag("  "))(input)
    }

    pub fn sub_class(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tab, id, tag("  "))(input)
    }

    pub fn protocol(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("\t\t"), id, tag("  "))(input)
    }

    pub fn audio_terminal_type(input: &str) -> IResult<&str, u16> {
        let id = id(4, u16::from_str_radix);
        delimited(tag("AT "), id, tag("  "))(input)
    }

    pub fn hid_type(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("HID "), id, tag("  "))(input)
    }

    pub fn hid_item_type(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("R "), id, tag("  "))(input)
    }

    pub fn bias_type(input: &str) -> IResult<&str, u8> {
        let id = id(1, u8::from_str_radix);
        delimited(tag("BIAS "), id, tag("  "))(input)
    }

    pub fn phy_type(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("PHY "), id, tag("  "))(input)
    }

    pub fn hut_type(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("HUT "), id, tag("  "))(input)
    }

    pub fn hid_usage_name(input: &str) -> IResult<&str, u16> {
        let id = id(3, u16::from_str_radix);
        delimited(tab, id, tag("  "))(input)
    }

    pub fn language(input: &str) -> IResult<&str, u16> {
        let id = id(4, u16::from_str_radix);
        delimited(tag("L "), id, tag("  "))(input)
    }

    pub fn dialect(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tab, id, tag("  "))(input)
    }

    pub fn country_code(input: &str) -> IResult<&str, u8> {
        let id = id(2, u8::from_str_radix);
        delimited(tag("HCC "), id, tag("  "))(input)
    }

    pub fn terminal_type(input: &str) -> IResult<&str, u16> {
        let id = id(4, u16::from_str_radix);
        delimited(tag("VT "), id, tag("  "))(input)
    }
}

impl quote::ToTokens for CgVendor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CgVendor {
            id: vendor_id,
            name,
            devices,
        } = self;

        let devices = devices.iter().map(|CgDevice { id, name, interfaces }| {
            quote!{
                Device { vendor_id: #vendor_id, id: #id, name: #name, interfaces: &[#(#interfaces),*] }
            }
        });
        tokens.extend(quote! {
            Vendor { id: #vendor_id, name: #name, devices: &[#(#devices),*] }
        });
    }
}

impl quote::ToTokens for CgClass {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CgClass {
            id: class_id,
            name,
            sub_classes,
        } = self;

        let sub_classes = sub_classes.iter().map(|CgSubClass { id, name, children }| {
            quote! {
                SubClass { class_id: #class_id, id: #id, name: #name, protocols: &[#(#children),*] }
            }
        });
        tokens.extend(quote! {
            Class { id: #class_id, name: #name, sub_classes: &[#(#sub_classes),*] }
        });
    }
}

impl<T: quote::ToTokens, C: quote::ToTokens> quote::ToTokens for CgParentType<T, C> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CgParentType { id, name, children } = self;
        tokens.extend(quote! {
            UsbIdWithChildren { id: #id, name: #name, children: &[#(#children),*] }
        });
    }
}

impl<T: quote::ToTokens> quote::ToTokens for CgType<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CgType { id, name } = self;
        tokens.extend(quote! {
            UsbId { id: #id, name: #name }
        });
    }
}
