/// Parse FIT files
///
/// Logic largely based on: https://github.com/dtcooper/python-fitparse
///
/// Notes:
///   All fields in these files are stored in little endian format.
///   I'll want to eventually determine a way to check CRCs if it's not too messy
///   Apparently FIT files can be chained? If so I'll need to conditionally rerun the parser
use std::fmt::Display;
use nom::IResult;
use nom::number::complete::{le_u8, le_u16, le_u32};

// FIT file header format should be 12-14 bytes
// header_size = u8,
// protocol_ver_enc = u8,
// profile_ver_enc = u16
// data_size = u32
// literal ".FIT" = [u8; 4]
// CRC = u16 (if the header_size is 14 bytes)
#[derive(Clone,Debug)]
pub struct FitFileHeader {
  pub header_size: u8,
  pub protocol_ver_enc: f32,
  pub profile_ver_enc: f32,
  pub data_size: u32,
  pub crc: Option<u16>
}

named!(pub header<FitFileHeader>,
  do_parse!(
    header_size: le_u8      >>
    protocol_ver_enc: le_u8 >>
    profile_ver_enc: le_u16 >>
    data_size: le_u32       >>
    tag!(".FIT")            >>
    crc: cond!(header_size - 12 >= 2, le_u16) >> // consume crc if it's available
    cond!(header_size > 14, take!(header_size - 14)) >> // consume any extra header bytes
    (FitFileHeader {
        header_size: header_size,
        protocol_ver_enc: split_decimal_to_float(protocol_ver_enc >> 4, protocol_ver_enc & ((1 << 4) - 1)),
        profile_ver_enc: split_decimal_to_float(profile_ver_enc / 100, profile_ver_enc % 100),
        data_size:  data_size,
        crc: crc
    })
  )
);

/// Convert a split decimal style value with fix precisions into a single floating point value
///
/// this function should never fail as long as integer values are passed in as the arguments
fn split_decimal_to_float<T: Display>(left: T, right: T) -> f32 {
    format!("{}.{}", left, right).parse().unwrap()
}

#[test]
fn header_test() {
  let data = include_bytes!("../test/fixtures/Activity.fit");
  let hdr = header(data).unwrap().1;
  assert_eq!(hdr.header_size, 12);
  assert_eq!(hdr.protocol_ver_enc, 1.0);
  assert_eq!(hdr.profile_ver_enc, 1.0);
  assert_eq!(hdr.data_size, 757);
  assert_eq!(hdr.crc, None);
}
