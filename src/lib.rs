//!
//! Rust wrappers for the [USB ID Repository](http://www.linux-usb.org/usb-ids.html).
//!
//! The USB ID Repository is the canonical source of USB device information for most
//! Linux userspaces; this crate vendors the USB ID database to allow non-Linux hosts to
//! access the same canonical information.
//!
//! # Usage
//!
//! Iterating over all known vendors:
//!
//! ```rust
//! use usb_ids::Vendors;
//!
//! for vendor in Vendors::iter() {
//!     for device in vendor.devices() {
//!         println!("vendor: {}, device: {}", vendor.name(), device.name());
//!     }
//! }
//! ```
//!
//! See the individual documentation for each structure for more details.
//!

#![warn(missing_docs)]

// Codegen: introduces USB_IDS, a phf::Map<u16, Vendor>.
include!(concat!(env!("OUT_DIR"), "/usb_ids.cg.rs"));

/// An abstraction for iterating over all vendors in the USB database.
pub struct Vendors;
impl Vendors {
    /// Returns an iterator over all vendors in the USB database.
    pub fn iter() -> impl Iterator<Item = &'static Vendor> {
        USB_IDS.values()
    }
}

/// Represents a USB device vendor in the USB database.
///
/// Every device vendor has a vendor ID, a pretty name, and a
/// list of associated [`Device`]s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vendor {
    id: u16,
    name: &'static str,
    devices: &'static [Device],
}

impl Vendor {
    /// Returns the vendor's ID.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the vendor's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns an iterator over the vendor's devices.
    pub fn devices(&self) -> impl Iterator<Item = &'static Device> {
        self.devices.iter()
    }
}

/// Represents a single device in the USB database.
///
/// Every device has a corresponding vendor, a device ID, a pretty name,
/// and a list of associated [`Interface`]s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Device {
    vendor_id: u16,
    id: u16,
    name: &'static str,
    interfaces: &'static [Interface],
}

impl Device {
    /// Returns the [`Device`] corresponding to the given vendor and product IDs,
    /// or `None` if no such device exists in the DB.
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<&'static Device> {
        let vendor = Vendor::from_id(vid);

        vendor.and_then(|v| v.devices().find(|d| d.id == pid))
    }

    /// Returns the [`Vendor`] that this device belongs to.
    ///
    /// Looking up a vendor by device is cheap (`O(1)`).
    pub fn vendor(&self) -> &'static Vendor {
        USB_IDS.get(&self.vendor_id).unwrap()
    }

    /// Returns a tuple of (vendor id, device/"product" id) for this device.
    ///
    /// This is convenient for interactions with other USB libraries.
    pub fn as_vid_pid(&self) -> (u16, u16) {
        (self.vendor_id, self.id)
    }

    /// Returns the device's ID.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the device's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns an iterator over the device's interfaces.
    ///
    /// **NOTE**: The USB database does not include interface information for
    /// most devices. This list is not authoritative.
    pub fn interfaces(&self) -> impl Iterator<Item = &'static Interface> {
        self.interfaces.iter()
    }
}

/// Represents an interface to a USB device in the USB database.
///
/// Every interface has an interface ID (which is an index on the device)
/// and a pretty name.
///
/// **NOTE**: The USB database is not a canonical or authoritative source
/// of interface information for devices. Users who wish to discover interfaces
/// on their USB devices should query those devices directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interface {
    id: u8,
    name: &'static str,
}

impl Interface {
    /// Returns the interface's ID.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the interface's name.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// A convenience trait for retrieving a top-level entity (like a [`Vendor`]) from the USB
/// database by its unique ID.
// NOTE(ww): This trait will be generally useful once we support other top-level
// entities in `usb.ids` (like language, country code, HID codes, etc).
pub trait FromId<T> {
    /// Returns the entity corresponding to `id`, or `None` if none exists.
    fn from_id(id: T) -> Option<&'static Self>;
}

impl FromId<u16> for Vendor {
    fn from_id(id: u16) -> Option<&'static Self> {
        USB_IDS.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_id() {
        let vendor = Vendor::from_id(0x1d6b).unwrap();

        assert_eq!(vendor.name(), "Linux Foundation");
        assert_eq!(vendor.id(), 0x1d6b);
    }

    #[test]
    fn test_vendor_devices() {
        let vendor = Vendor::from_id(0x1d6b).unwrap();

        for device in vendor.devices() {
            assert_eq!(device.vendor(), vendor);
            assert!(!device.name().is_empty());
        }
    }

    #[test]
    fn test_from_vid_pid() {
        let device = Device::from_vid_pid(0x1d6b, 0x0003).unwrap();

        assert_eq!(device.name(), "3.0 root hub");

        let (vid, pid) = device.as_vid_pid();

        assert_eq!(vid, device.vendor().id());
        assert_eq!(pid, device.id());

        let device2 = Device::from_vid_pid(vid, pid).unwrap();

        assert_eq!(device, device2);
    }
}
