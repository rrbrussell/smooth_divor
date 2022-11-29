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
#[br(big, magic = br"\xed\xab\xee\xdb")]
pub struct RPMLead {
    magic: [u8; 4],
    major: u8,
    minor: u8,
    rpmtype: u16,
    archnum: u16,
    name: [u8; 66],
    osnum: u16,
    signature_type: u16,
    reserved: [u8; 16],
}

impl RPMLead {
    pub fn from_u8_buffer(buffer: &[u8]) -> Result<Self, errors::Error> {
        if buffer.len() < RPM_LEAD_SIZE {
            return Err(Error::InputBufferToSmall);
        }

        let in_magic: [u8; 4] = [buffer[0], buffer[1], buffer[2], buffer[3]];
        let in_major: u8 = buffer[4];
        let in_minor: u8 = buffer[5];
        let in_rpmtype: u16 = u16::from_be_bytes([buffer[6], buffer[7]]);
        let in_archnum: u16 = u16::from_be_bytes([buffer[8], buffer[9]]);
        let in_name: [u8; 66] = buffer[10..76].try_into().unwrap();
        let in_osnum: u16 = u16::from_be_bytes([buffer[76], buffer[77]]);
        let in_signature_type: u16 = u16::from_be_bytes([buffer[78], buffer[79]]);
        let in_reserved: [u8; 16] = buffer[80..].try_into().unwrap();
        return Ok(RPMLead {
            magic: in_magic,
            major: in_major,
            minor: in_minor,
            rpmtype: in_rpmtype,
            archnum: in_archnum,
            name: in_name,
            osnum: in_osnum,
            signature_type: in_signature_type,
            reserved: in_reserved,
        });
    }

    pub fn is_binary_package(self: &Self) -> bool {
        return self.rpmtype == 0000;
    }

    pub fn is_source_package(self: &Self) -> bool {
        return self.rpmtype == 0001;
    }

    pub fn is_valid(self: &Self) -> bool {
        let valid_magic = self.magic == RPM_FILE_MAGIC;
        let valid_major = self.major == 03;
        let valid_minor = self.minor == 00;
        let valid_rpmtype = self.rpmtype == 0000 || self.rpmtype == 0001;
        let valid_name = self.name[65] == 00;
        let valid_signature_type = self.signature_type == 0005;
        return valid_magic
            && valid_major
            && valid_minor
            && valid_rpmtype
            && valid_name
            && valid_signature_type;
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
