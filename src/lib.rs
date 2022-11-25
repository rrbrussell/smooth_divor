use enums::IndexFormats;

mod enums;

/// RPMLead is the first part of an RPM file.
#[derive(Debug)]
pub struct RPMLead {
    pub magic: [u8; 4],
    pub major: u8,
    pub minor: u8,
    pub rpmtype: i16,
    pub archnum: i16,
    pub name: [u8; 66],
    pub osnum: i16,
    pub signature_type: i16,
    pub reserved: [u8; 16],
}

impl RPMLead {
    pub fn is_binary_package(self: &Self) -> bool {
        return self.rpmtype == 0000;
    }

    pub fn is_source_package(self: &Self) -> bool {
        return self.rpmtype == 0001;
    }

    pub fn is_valid(self: &Self) -> bool {
        let valid_magic = self.magic == [0xed, 0xab, 0xee, 0xdb];
        let valid_major = self.major == 03;
        let valid_minor = self.minor == 00;
        let valid_rpmtype = self.rpmtype == 0000 || self.rpmtype == 0001;
        let valid_archnum = self.archnum == 0001;
        let valid_name = self.name[65] == 00;
        let valid_osnum = self.osnum == 0001;
        let valid_signature_type = self.signature_type == 0005;
        return valid_magic
            && valid_major
            && valid_minor
            && valid_rpmtype
            && valid_archnum
            && valid_name
            && valid_osnum
            && valid_signature_type;
    }
}

/// Used for both the Signature and Header Sections on an RPM file.
#[derive(Debug)]
pub struct RPMHeader {
    magic: [u8; 3],
    version: u8,
    reserved: [u8; 4],
    nindex: u32,
    hsize: u32,
}

#[derive(Debug)]
pub struct RPMIndex {
    tag: i32,
    format: IndexFormats,
    offset: i32,
    count: i32,
}
