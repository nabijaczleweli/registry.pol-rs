//! The Group Policy Object Editor stores registry-based configuration settings in two Registry.pol files, stored in folders
//! under the `<drive>:\Windows\System32\GroupPolicy\` folder. One file contains computer settings and the other file contains
//! user settings. The Group Policy Object Editor saves the settings to these files on exit, and imports the settings on
//! startup.
//!
//! A `Registry.pol` file is a text file that consists of a header and a body.
//! The header contains two DWORD values that indicate the file signature and version. These values are defined as follows.


use self::super::{REGISTRY_FILE_VERSION, RegistryValueType, RegistryValue};
use nom::{ErrorKind, IResult, le_u16, le_u32};
use self::super::super::REGFILE_SIGNATURE;
use std::mem::transmute;


lazy_static! {
    static ref REGFILE_SIGNATURE_LE: [u8; 4] = unsafe { transmute(REGFILE_SIGNATURE.to_le()) };
    static ref REGISTRY_FILE_VERSION_LE: [u8; 4] = unsafe { transmute(REGISTRY_FILE_VERSION.to_le()) };
}
const UTF16_LE_OPEN_SQUARE_BRACKET: [u8; 2] = [0x5B, 0x00];
const UTF16_LE_CLOSE_SQUARE_BRACKET: [u8; 2] = [0x5D, 0x00];
const UTF16_LE_SEMICOLON: [u8; 2] = [0x3B, 0x00];
const UTF16_LE_NUL: [u8; 2] = [0x00, 0x00];


fn nul_terminated_utf16(input: &[u8]) -> IResult<&[u8], String> {
    fn utf16_nul(input: &[u8]) -> IResult<&[u8], &[u8]> {
        tag!(input, UTF16_LE_NUL)
    }
    named!(utf16_bytes<(Vec<u16>, &[u8])>, many_till!(le_u16, utf16_nul));

    match utf16_bytes(input) {
        IResult::Done(input, (chars, zero)) => {
            match String::from_utf16(&chars) {
                Ok(s) => {
                    println!("nul_terminated_utf16: {:?}, {:?}", s, zero);
                    IResult::Done(input, s)
                }
                Err(_) => IResult::Error(error_code!(ErrorKind::Custom(16))),
            }
        }
        IResult::Error(e) => IResult::Error(e),
        IResult::Incomplete(n) => IResult::Incomplete(n),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(data_type<Option<RegistryValueType>>, do_parse!(
    tp: le_u32 >>
    (RegistryValueType::parse(tp))
));

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(header<()>, do_parse!(
    tag!(*REGFILE_SIGNATURE_LE) >>
    tag!(*REGISTRY_FILE_VERSION_LE) >>
    ()
));

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(value<RegistryValue>, do_parse!(
    key: nul_terminated_utf16 >>
    tag!(UTF16_LE_SEMICOLON) >>
    value: nul_terminated_utf16 >>
    tag!(UTF16_LE_SEMICOLON) >>
    tp: data_type >>
    tag!(UTF16_LE_SEMICOLON) >>
    size: le_u32 >>
    tag!(UTF16_LE_SEMICOLON) >>
    data: take!(size) >>
    (RegistryValue{
        key: key,
        value: Some(value),
        data_type: tp,
        data: Some(data.to_vec()),
    })
));

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub parse<Vec<RegistryValue>>,
  do_parse!(
    header >>
    values: many0!(delimited!(tag!(UTF16_LE_OPEN_SQUARE_BRACKET), value, tag!(UTF16_LE_CLOSE_SQUARE_BRACKET))) >>
    (values)
  )
);
