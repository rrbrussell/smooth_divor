/// The 4 bytes that identify an RPM file.
pub const RPM_FILE_MAGIC: [u8; 4] = [0xed, 0xab, 0xee, 0xdb];

#[cfg_attr(debug_assertions, allow(dead_code))]
/// The 3 bytes that identify the start of a header in an RPM file.
pub const RPM_HEADER_MAGIC: [u8; 3] = [0x8e, 0xad, 0xe8];

/// How large is the lead element of an RPM file.
pub const RPM_LEAD_SIZE: usize = 96;
