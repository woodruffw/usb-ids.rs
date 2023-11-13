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

include!(concat!(env!("OUT_DIR"), "/usb_ids.cg.rs"));

const AT_ID: u8 = 1;
const HID_ID: u8 = 2;
const HID_TYPE_ID: u8 = 3;
const HID_USAGE_ID: u8 = 4;
const BIAS_ID: u8 = 5;
const PHY_ID: u8 = 6;
const DIALECT_ID: u8 = 7;
const HCC_ID: u8 = 8;
const VT_ID: u8 = 9;

/// An abstraction for iterating over all vendors in the USB database.
pub struct Vendors;
impl Vendors {
    /// Returns an iterator over all vendors in the USB database.
    pub fn iter() -> impl Iterator<Item = &'static Vendor> {
        USB_IDS.values()
    }
}

/// An abstraction for iterating over all classes in the USB database.
pub struct Classes;
impl Classes {
    /// Returns an iterator over all classes in the USB database.
    pub fn iter() -> impl Iterator<Item = &'static Class> {
        USB_CLASSES.values()
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
pub trait FromId<T> {
    /// Returns the entity corresponding to `id`, or `None` if none exists.
    fn from_id(id: T) -> Option<&'static Self>;
}

impl FromId<u16> for Vendor {
    fn from_id(id: u16) -> Option<&'static Self> {
        USB_IDS.get(&id)
    }
}

impl FromId<u8> for Class {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_CLASSES.get(&id)
    }
}

impl FromId<u16> for AudioTerminal {
    fn from_id(id: u16) -> Option<&'static Self> {
        USB_AUDIO_TERMINALS.get(&id)
    }
}

impl FromId<u8> for Hid {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_HID_IDS.get(&id)
    }
}

impl FromId<u8> for HidItemType {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_HID_R_TYPES.get(&id)
    }
}

impl FromId<u8> for HidUsagePage {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_HUTS.get(&id)
    }
}

impl FromId<u8> for Bias {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_BIASES.get(&id)
    }
}

impl FromId<u8> for Phy {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_PHYS.get(&id)
    }
}

impl FromId<u16> for Language {
    fn from_id(id: u16) -> Option<&'static Self> {
        USB_LANGS.get(&id)
    }
}

impl FromId<u8> for HidCountryCode {
    fn from_id(id: u8) -> Option<&'static Self> {
        USB_HID_CCS.get(&id)
    }
}

impl FromId<u16> for VideoTerminal {
    fn from_id(id: u16) -> Option<&'static Self> {
        USB_VIDEO_TERMINALS.get(&id)
    }
}

/// Represents a USB device class in the USB database.
///
/// Every device class has a class ID, a pretty name, and a
/// list of associated [`SubClass`]s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Class {
    id: u8,
    name: &'static str,
    sub_classes: &'static [SubClass],
}

impl Class {
    /// Returns the class's ID.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the class's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns an iterator over the class's subclasses.
    pub fn sub_classes(&self) -> impl Iterator<Item = &'static SubClass> {
        self.sub_classes.iter()
    }
}

/// Represents a class subclass in the USB database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubClass {
    class_id: u8,
    id: u8,
    name: &'static str,
    protocols: &'static [Protocol],
}

impl SubClass {
    /// Returns the [`SubClass`] corresponding to the given class and subclass IDs,
    /// or `None` if no such subclass exists in the DB.
    pub fn from_cid_scid(class_id: u8, id: u8) -> Option<&'static Self> {
        let class = Class::from_id(class_id);

        class.and_then(|c| c.sub_classes().find(|s| s.id == id))
    }

    /// Returns the [`Class`] that this subclass belongs to.
    ///
    /// Looking up a class by subclass is cheap (`O(1)`).
    pub fn class(&self) -> &'static Class {
        USB_CLASSES.get(&self.class_id).unwrap()
    }

    /// Returns a tuple of (class id, subclass id) for this subclass.
    ///
    /// This is convenient for interactions with other USB libraries.
    pub fn as_cid_scid(&self) -> (u8, u8) {
        (self.class_id, self.id)
    }

    /// Returns the subclass' ID.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the subclass' name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns an iterator over the subclasses's protocols.
    ///
    /// **NOTE**: The USB database nor USB-IF includes protocol information for
    /// all subclassess. This list is not authoritative.
    pub fn protocols(&self) -> impl Iterator<Item = &'static Protocol> {
        self.protocols.iter()
    }
}

const PROTOCOL_ID: u8 = 0;
/// Represents a subclass protocol in the USB database.
pub type Protocol = UsbId<PROTOCOL_ID, u8>;

impl Protocol {
    /// Returns the [`Protocol`] corresponding to the given class, subclass, and protocol IDs,
    /// or `None` if no such protocol exists in the DB.
    pub fn from_cid_scid_pid(class_id: u8, subclass_id: u8, id: u8) -> Option<&'static Self> {
        let subclass = SubClass::from_cid_scid(class_id, subclass_id);

        subclass.and_then(|s| s.protocols().find(|p| p.id == id))
    }
}

/// Represents a generic USB ID in the USB database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbId<const ID: u8, T> {
    id: T,
    name: &'static str,
}

impl<const ID: u8, T: Copy> UsbId<ID, T> {
    /// Returns the type's ID.
    pub fn id(&self) -> T {
        self.id
    }

    /// Returns the type's name.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// Represents a generic USB ID in the USB database with children IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbIdWithChildren<T: Copy, C: 'static> {
    id: T,
    name: &'static str,
    children: &'static [C],
}

impl<T: Copy, C: 'static> UsbIdWithChildren<T, C> {
    /// Returns the type's ID.
    pub fn id(&self) -> T {
        self.id
    }

    /// Returns the type's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns an iterator over the type's children.
    fn children(&self) -> impl Iterator<Item = &'static C> {
        self.children.iter()
    }
}

/// Represents an audio terminal in the USB database.
pub type AudioTerminal = UsbId<AT_ID, u16>;

/// Represents a HID descriptor type in the USB database.
pub type Hid = UsbId<HID_ID, u8>;

/// Represents a HID descriptor item type in the USB database.
pub type HidItemType = UsbId<HID_TYPE_ID, u8>;

/// Represents a HID usage page in the USB database.
pub type HidUsagePage = UsbIdWithChildren<u8, HidUsage>;

impl HidUsagePage {
    /// Returns an iterator over the page's usages.
    pub fn usages(&self) -> impl Iterator<Item = &'static HidUsage> {
        self.children()
    }
}

/// Represents a HID usage in the USB database.
pub type HidUsage = UsbId<HID_USAGE_ID, u16>;

impl HidUsage {
    /// Returns the [`HidUsage`] corresponding to the given usage page and usage ID,
    /// or `None` if no such usage exists in the DB.
    pub fn from_pageid_uid(page_id: u8, id: u16) -> Option<&'static Self> {
        let page = HidUsagePage::from_id(page_id)?;

        page.children().find(|u| u.id() == id)
    }
}

/// Physical descriptor bias types
pub type Bias = UsbId<BIAS_ID, u8>;

/// Physical descriptor item types
pub type Phy = UsbId<PHY_ID, u8>;

/// Represents a language in the USB database.
pub type Language = UsbIdWithChildren<u16, Dialect>;

impl Language {
    /// Returns an iterator over the language's dialects.
    pub fn dialects(&self) -> impl Iterator<Item = &'static Dialect> {
        self.children()
    }
}

/// Represents a language dialect in the USB database.
pub type Dialect = UsbId<DIALECT_ID, u8>;

impl Dialect {
    /// Returns the [`Dialect`] corresponding to the given language and dialect IDs,
    /// or `None` if no such dialect exists in the DB.
    pub fn from_lid_did(language_id: u16, id: u8) -> Option<&'static Self> {
        let language = Language::from_id(language_id)?;

        language.children().find(|d| d.id() == id)
    }
}

/// Represents a HID descriptor country code in the USB database.
pub type HidCountryCode = UsbId<HCC_ID, u8>;

/// Represents a video class terminal type in the USB database.
pub type VideoTerminal = UsbId<VT_ID, u16>;

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

        let last_device = Device::from_vid_pid(0xffee, 0x0100).unwrap();
        assert_eq!(last_device.name(), "Card Reader Controller RTS5101/RTS5111/RTS5116");
    }

    #[test]
    fn test_class_from_id() {
        let class = Class::from_id(0x03).unwrap();

        assert_eq!(class.name(), "Human Interface Device");
        assert_eq!(class.id(), 0x03);
    }

    #[test]
    fn test_subclass_from_cid_scid() {
        let subclass = SubClass::from_cid_scid(0x03, 0x01).unwrap();

        assert_eq!(subclass.name(), "Boot Interface Subclass");
        assert_eq!(subclass.id(), 0x01);
    }

    #[test]
    fn test_protocol_from_cid_scid_pid() {
        let protocol = Protocol::from_cid_scid_pid(0x03, 0x01, 0x01).unwrap();

        assert_eq!(protocol.name(), "Keyboard");
        assert_eq!(protocol.id(), 0x01);

        let protocol = Protocol::from_cid_scid_pid(0x07, 0x01, 0x03).unwrap();

        assert_eq!(protocol.name(), "IEEE 1284.4 compatible bidirectional");
        assert_eq!(protocol.id(), 0x03);

        let protocol = Protocol::from_cid_scid_pid(0xff, 0xff, 0xff).unwrap();

        // check last entry for parsing
        assert_eq!(protocol.name(), "Vendor Specific Protocol");
        assert_eq!(protocol.id(), 0xff);
    }

    #[test]
    fn test_at_from_id() {
        let at = AudioTerminal::from_id(0x0713).unwrap();

        assert_eq!(at.name(), "Synthesizer");
        assert_eq!(at.id(), 0x0713);
    }

    #[test]
    fn test_hid_from_id() {
        let hid = Hid::from_id(0x23).unwrap();

        assert_eq!(hid.name(), "Physical");
        assert_eq!(hid.id(), 0x23);
    }

    #[test]
    fn test_hid_type_from_id() {
        let hid_type = HidItemType::from_id(0xc0).unwrap();

        assert_eq!(hid_type.name(), "End Collection");
        assert_eq!(hid_type.id(), 0xc0);
    }

    #[test]
    fn test_bias_from_id() {
        let bias = Bias::from_id(0x04).unwrap();

        assert_eq!(bias.name(), "Either Hand");
        assert_eq!(bias.id(), 0x04);
    }

    #[test]
    fn test_phy_from_id() {
        let phy = Phy::from_id(0x27).unwrap();

        assert_eq!(phy.name(), "Cheek");
        assert_eq!(phy.id(), 0x27);
    }

    #[test]
    fn test_hid_usages_from_id() {
        let hid_usage_page = HidUsagePage::from_id(0x0d).unwrap();

        assert_eq!(hid_usage_page.name(), "Digitizer");
        assert_eq!(hid_usage_page.id(), 0x0d);

        let hid_usage = HidUsage::from_pageid_uid(0x0d, 0x01).unwrap();

        assert_eq!(hid_usage.name(), "Digitizer");
        assert_eq!(hid_usage.id(), 0x01);
    }

    #[test]
    fn test_language_from_id() {
        let language = Language::from_id(0x0007).unwrap();

        assert_eq!(language.name(), "German");
        assert_eq!(language.id(), 0x0007);

        let dialect = language.dialects().find(|d| d.id() == 0x02).unwrap();

        assert_eq!(dialect.name(), "Swiss");
        assert_eq!(dialect.id(), 0x02);
    }

    #[test]
    fn test_hid_country_code_from_id() {
        let hid_country_code = HidCountryCode::from_id(0x29).unwrap();

        assert_eq!(hid_country_code.name(), "Switzerland");
        assert_eq!(hid_country_code.id(), 0x29);

        let hid_country_code = HidCountryCode::from_id(0x00).unwrap();
        assert_eq!(hid_country_code.name(), "Not supported");
    }

    #[test]
    fn test_video_terminal_from_id() {
        let video_terminal = VideoTerminal::from_id(0x0100).unwrap();

        assert_eq!(video_terminal.name(), "USB Vendor Specific");
        assert_eq!(video_terminal.id(), 0x0100);

        let video_terminal = VideoTerminal::from_id(0x0403).unwrap();
        assert_eq!(video_terminal.name(), "Component Video");
    }
}
