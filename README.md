usb-ids
=======

[![Build Status](https://img.shields.io/github/workflow/status/woodruffw/usb-ids.rs/CI/main)](https://github.com/woodruffw/usb-ids.rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/usb-ids)](https://crates.io/crates/usb-ids)

Cross-platform Rust wrappers for the [USB ID Repository](http://www.linux-usb.org/usb-ids.html).

This library bundles the USB ID database, allowing platforms other than Linux to query it
as a source of canonical USB metadata.

## Usage

Iterating over all known vendors:

```rust
use usb_ids::Vendors;

for vendor in Vendors::iter() {
    for device in vendor.devices() {
        println!("vendor: {}, device: {}", vendor.name(), device.name());
    }
}
```

See [the documentation](https://docs.rs/usb-ids) for more details.
