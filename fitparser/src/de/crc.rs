//! Helper functions to calclate the crc value from a slice of bytes
//! These were converted from `fit_crc.c` in the FIT SDK.

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800, 0xB401,
    0x5000, 0x9C01, 0x8801, 0x4400,
];

/// calculate a CRC from a slice of bytes.
pub fn caculate_crc(data: &[u8]) -> u16 {
    update_crc(0, data)
}

/// Update a pre-existing CRC value with more data, to calculate a new CRC use 0 as the initial
/// value passed in.
pub fn update_crc(crc: u16, data: &[u8]) -> u16 {
    data.iter().fold(crc, |acc, byte| get_crc(acc, *byte))
}

#[inline]
/// Calcuate the checksum for the byte provided
fn get_crc(crc: u16, byte: u8) -> u16 {
    // compute checksum of lower four bits of byte
    let mut tmp = CRC_TABLE[(crc & 0xF) as usize];
    let mut crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ CRC_TABLE[(byte & 0xF) as usize];

    // now compute checksum of upper four bits of byte
    tmp = CRC_TABLE[(crc & 0xF) as usize];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0xF) as usize];

    crc
}
