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

type VMap = Map<u16>;

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

struct CgInterface {
    id: u8,
    name: String,
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

    // Parser state.
    let mut curr_vendor: Option<CgVendor> = None;
    let mut curr_device_id = 0u16;

    let mut map = emit_prologue(&mut output);

    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Ok((name, id)) = parser::vendor(&line) {
            // If there was a previous vendor, emit it.
            if let Some(vendor) = curr_vendor.take() {
                emit_vendor(&mut map, &vendor);
            }

            // Set our new vendor as the current vendor.
            curr_vendor = Some(CgVendor {
                id,
                name: name.into(),
                devices: vec![],
            });
        } else if let Ok((name, id)) = parser::device(&line) {
            // We should always have a current vendor; failure here indicates a malformed input.
            let curr_vendor = curr_vendor.as_mut().unwrap();
            curr_vendor.devices.push(CgDevice {
                id,
                name: name.into(),
                interfaces: vec![],
            });
            curr_device_id = id;
        } else if let Ok((name, id)) = parser::interface(&line) {
            // We should always have a current vendor; failure here indicates a malformed input.
            // Similarly, our current vendor should always have a device corresponding
            // to the current device id.
            let curr_vendor = curr_vendor.as_mut().unwrap();
            let curr_device = curr_vendor
                .devices
                .iter_mut()
                .find(|d| d.id == curr_device_id)
                .unwrap();

            curr_device.interfaces.push(CgInterface {
                id,
                name: name.into(),
            });
        } else {
            // TODO: Lots of other things that could be parsed out:
            // Language, dialect, country code, HID types, ...
            break;
        }
    }
    if let Some(vendor) = curr_vendor.take() {
        emit_vendor(&mut map, &vendor);
    }

    emit_epilogue(&mut output, map);

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
}

fn emit_prologue(output: &mut impl Write) -> VMap {
    writeln!(output, "static USB_IDS: phf::Map<u16, Vendor> = ").unwrap();

    Map::new()
}

fn emit_vendor(map: &mut VMap, vendor: &CgVendor) {
    map.entry(vendor.id, &quote!(#vendor).to_string());
}

fn emit_epilogue(output: &mut impl Write, map: VMap) {
    writeln!(output, "{};", map.build()).unwrap();
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

impl quote::ToTokens for CgInterface {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let CgInterface { id, name } = self;
        tokens.extend(quote! {
            Interface { id: #id, name: #name }
        });
    }
}
