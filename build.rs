use std::env;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use phf_codegen::Map;
use regex::Regex;

/* This build script contains a "parser" for the USB ID database.
 * "Parser" is in scare-quotes because it's really a line matcher with a small amount
 * of context needed for pairing nested entities (e.g. devices) with their parents (e.g. vendors).
 */

type VMap = Map<u16>;

struct CGVendor {
    id: u16,
    name: String,
    devices: Vec<CGDevice>,
}

struct CGDevice {
    id: u16,
    name: String,
    interfaces: Vec<CGInterface>,
}

struct CGInterface {
    id: u8,
    name: String,
}

#[allow(clippy::redundant_field_names)]
fn main() {
    // Regexp line patterns.
    let vendor_line = Regex::new(r"^(?P<id>[[:xdigit:]]{4})\s{2}(?P<name>.+)$").unwrap();
    let device_line = Regex::new(r"^\t(?P<id>[[:xdigit:]]{4})\s{2}(?P<name>.+)$").unwrap();
    let interface_line = Regex::new(r"^\t\t(?P<id>[[:xdigit]]{2})\s{2}(?P<name>.+)$").unwrap();

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
    let mut prev_vendor: Option<CGVendor> = None;
    let mut curr_vendor: Option<CGVendor> = None;
    let mut curr_device_id = 0u16;

    let mut map = emit_prologue(&mut output);

    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(captures) = vendor_line.captures(&line) {
            let id = u16::from_str_radix(&captures["id"], 16).unwrap();
            let name = &captures["name"];

            // If there was a previous vendor, emit it.
            if let Some(vendor) = prev_vendor {
                emit_vendor(&mut map, &vendor);
            }

            // Set our new vendor as the current vendor.
            prev_vendor = curr_vendor;
            curr_vendor = Some(CGVendor {
                id: id,
                name: name.into(),
                devices: vec![],
            });
        } else if let Some(captures) = device_line.captures(&line) {
            let id = u16::from_str_radix(&captures["id"], 16).unwrap();
            let name = &captures["name"];

            // We should always have a current vendor; failure here indicates a malformed input.
            let curr_vendor = curr_vendor.as_mut().unwrap();
            curr_vendor.devices.push(CGDevice {
                id: id,
                name: name.into(),
                interfaces: vec![],
            });
            curr_device_id = id;
        } else if let Some(captures) = interface_line.captures(&line) {
            let id = u8::from_str_radix(&captures["id"], 16).unwrap();
            let name = &captures["name"];

            // We should always have a current vendor; failure here indicates a malformed input.
            // Similarly, our current vendor should always have a device corresponding
            // to the current device id.
            let curr_vendor = curr_vendor.as_mut().unwrap();
            let curr_device = curr_vendor
                .devices
                .iter_mut()
                .find(|d| d.id == curr_device_id)
                .unwrap();

            curr_device.interfaces.push(CGInterface {
                id: id,
                name: name.into(),
            });
        } else {
            // TODO: Lots of other things that could be parsed out:
            // Language, dialect, country code, HID types, ...
            break;
        }
    }

    emit_epilogue(&mut output, map);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/usb.ids");
}

fn emit_prologue(output: &mut impl Write) -> VMap {
    writeln!(output, "static USB_IDS: phf::Map<u16, Vendor> = ").unwrap();

    Map::new()
}

fn emit_vendor(map: &mut VMap, vendor: &CGVendor) {
    map.entry(
        vendor.id,
        format!(
            "Vendor {{ id: {}, name: r###\"{}\"###, devices: &[{}] }}",
            vendor.id,
            &vendor.name,
            build_device_list(vendor.id, &vendor.devices)
        )
        .as_str(),
    );
}

fn build_device_list(vendor_id: u16, devices: &[CGDevice]) -> String {
    // SWAG: Each device repr is probably around 64 bytes (including commas),
    // so give ourselves about that much space per device.
    let mut list = String::with_capacity(64 * devices.len());
    for device in devices.iter() {
        list.push_str(build_device(vendor_id, &device).as_str());
        list.push_str(", ");
    }

    list
}

fn build_device(vendor_id: u16, device: &CGDevice) -> String {
    format!(
        "Device {{ vendor_id: {}, id: {}, name: r###\"{}\"###, interfaces: &[{}] }}",
        vendor_id,
        device.id,
        &device.name,
        build_interface_list(&device.interfaces)
    )
}

fn build_interface_list(interfaces: &[CGInterface]) -> String {
    // SWAG: Each interfaces repr is probably around 64 bytes (including commas),
    // so give ourselves about that much space per interfaces.
    let mut list = String::with_capacity(64 * interfaces.len());
    for interface in interfaces.iter() {
        list.push_str(
            format!(
                "Interface {{ id: {}, name: r###\"{}\"### }}",
                interface.id, &interface.name
            )
            .as_str(),
        );
        list.push_str(", ");
    }

    list
}

fn emit_epilogue(output: &mut impl Write, map: VMap) {
    writeln!(output, "{};", map.build()).unwrap();
}
