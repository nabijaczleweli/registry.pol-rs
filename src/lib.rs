//! The Group Policy Object Editor stores registry-based configuration settings in two Registry.pol files, stored in folders
//! under the `<drive>:\Windows\System32\GroupPolicy\` folder. One file contains computer settings and the other file contains
//! user settings. The Group Policy Object Editor saves the settings to these files on exit, and imports the settings on
//! startup.


#[macro_use]
extern crate nom;
#[macro_use]
extern crate lazy_static;

pub mod v1;

pub use self::v1::*;


/// First part of header, equivalent to "PReg" in little-endian.
pub const REGFILE_SIGNATURE: u32 = 0x67655250;
