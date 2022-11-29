use binrw::BinRead;
// reexports
pub use constants::{RPM_FILE_MAGIC, RPM_LEAD_SIZE};
pub use enums::HeaderTags;
pub use enums::IndexFormats;
pub use errors::Error;

// Internal modules
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
#[derive(Debug)]
#[repr(C)]
pub struct RPMHeader {
    magic: [u8; 3],
    version: u8,
    reserved: [u8; 4],
    number_of_index_entries: u32,
    hsize: u32,
}

/// A Signature or Header index item.
#[cfg_attr(debug_assertions, allow(dead_code))]
#[derive(Debug)]
#[repr(C)]
pub struct RPMHeaderIndexEntry {
    tag: HeaderTags,
    format: IndexFormats,
    offset: u32,
    count: u32,
}

impl RPMHeaderIndexEntry {
    pub fn from_u8_buffer(buffer: &[u8]) -> Result<Self, errors::Error> {
        if buffer.len() < 16 {
            return Err(Error::InputBufferToSmall);
        }
        todo!();
    }
}
