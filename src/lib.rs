use binrw::BinRead;
use std::fmt;

// reexports
pub use constants::{RPM_FILE_MAGIC, RPM_HEADER_SIZE, RPM_LEAD_SIZE};
pub use enums::{HeaderTags, IndexFormats, SignatureTags};
pub use errors::Error;

// Internal modules
pub mod cli_support;
mod constants;
mod enums;
mod errors;

/// RPMLead is the first part of an RPM file.
#[derive(Debug, BinRead)]
#[br(big, magic = 0xedabeedb_u32)]
pub struct RPMLead {
    major: u8,
    minor: u8,
    rpmtype: u16,
    _archnum: u16,
    name: [u8; 66],
    _osnum: u16,
    signature_type: u16,
    _reserved: [u8; 16],
}

impl RPMLead {
    pub fn is_binary_package(self: &Self) -> bool {
        return self.rpmtype == 0000;
    }

    pub fn is_source_package(self: &Self) -> bool {
        return self.rpmtype == 0001;
    }

    pub fn is_valid(self: &Self) -> bool {
        let valid_major = self.major == 03;
        let valid_minor = self.minor == 00;
        let valid_rpmtype = self.rpmtype == 0000 || self.rpmtype == 0001;
        let valid_name = self.name[65] == 00;
        let valid_signature_type = self.signature_type == 0005;
        return valid_major && valid_minor && valid_rpmtype && valid_name && valid_signature_type;
    }

    pub fn name(self: &Self) -> String {
        return String::from_utf8_lossy(&self.name[..]).to_string();
    }
}

/// Used for both the Signature and Header Sections on an RPM file.
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug, BinRead)]
#[br(big, magic = b"\x8e\xad\xe8\x01")]
pub struct RPMHeader {
    _reserved: [u8; 4],
    number_of_index_entries: u32,
    hsize: u32,
}

impl RPMHeader {
    pub fn header_size(self: &Self) -> u32 {
        return self.hsize;
    }

    pub fn is_valid(self: &Self) -> bool {
        return self.number_of_index_entries >= 1 && self.hsize > 1;
    }

    pub fn number_of_index_entries(self: &Self) -> u32 {
        return self.number_of_index_entries;
    }
}

/// Used for both the Signature and Header Sections on an RPM file.
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug, BinRead)]
#[br(big, magic = b"\x8e\xad\xe8\x01")]
pub struct RPMSignature {
    _reserved: [u8; 4],
    number_of_index_entries: u32,
    hsize: u32,
}

impl RPMSignature {
    pub fn header_size(self: &Self) -> u32 {
        return self.hsize;
    }

    pub fn is_valid(self: &Self) -> bool {
        return self.number_of_index_entries >= 1 && self.hsize > 1;
    }

    pub fn number_of_index_entries(self: &Self) -> u32 {
        return self.number_of_index_entries;
    }
}

/// A Signature or Header index item.
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug, BinRead)]
#[br(big)]
pub struct RPMHeaderIndexEntry {
    tag: HeaderTags,
    format: IndexFormats,
    offset: u32,
    count: u32,
}

impl fmt::Display for RPMHeaderIndexEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{} is a {} with {} elements at {:#X}",
            self.tag, self.format, self.count, self.offset
        );
    }
}

/// A Signature or Header index item.
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug, BinRead)]
#[br(big)]
pub struct RPMSignatureIndexEntry {
    tag: SignatureTags,
    format: IndexFormats,
    offset: u32,
    count: u32,
}

impl fmt::Display for RPMSignatureIndexEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{} is a {} with {} elements at {:#X}",
            self.tag, self.format, self.count, self.offset
        );
    }
}
