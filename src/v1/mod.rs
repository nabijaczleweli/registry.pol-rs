//! Registry Policy File format version 1.


mod parser;

use nom;


/// Second part of header.
pub const REGISTRY_FILE_VERSION: u32 = 1;


/// The data type field can contain any of the registry value types defined in WinNT.h.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RegistryValueType {
    REG_NONE = 0,
    REG_SZ = 1,
    REG_EXPAND_SZ = 2,
    REG_BINARY = 3,
    REG_DWORD = 4,
    REG_DWORD_BIG_ENDIAN = 5,
    REG_LINK = 6,
    REG_MULTI_SZ = 7,
    REG_QWORD = 11,
}

#[allow(non_snake_case)]
impl RegistryValueType {
    /// Same as `REG_DWORD`
    pub fn REG_DWORD_LITTLE_ENDIAN() -> RegistryValueType {
        RegistryValueType::REG_DWORD
    }

    pub fn REG_QWORD_LITTLE_ENDIAN() -> RegistryValueType {
        // Not sure if that's always that but my WinNT.h says so :v
        RegistryValueType::REG_QWORD
    }
}

impl RegistryValueType {
    /// Get a `RegistryValueType` from an integer representation, or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use registry_pol::v1::RegistryValueType;
    /// assert_eq!(RegistryValueType::parse(RegistryValueType::REG_DWORD as u32),
    ///            Some(RegistryValueType::REG_DWORD));
    /// assert_eq!(RegistryValueType::parse(420), None);
    /// ```
    pub fn parse(tp: u32) -> Option<RegistryValueType> {
        match tp {
            0 => Some(RegistryValueType::REG_NONE),
            1 => Some(RegistryValueType::REG_SZ),
            2 => Some(RegistryValueType::REG_EXPAND_SZ),
            3 => Some(RegistryValueType::REG_BINARY),
            4 => Some(RegistryValueType::REG_DWORD),
            5 => Some(RegistryValueType::REG_DWORD_BIG_ENDIAN),
            6 => Some(RegistryValueType::REG_LINK),
            7 => Some(RegistryValueType::REG_MULTI_SZ),
            11 => Some(RegistryValueType::REG_QWORD),
            _ => None,
        }
    }
}


/// A representation of registry value version 1.
///
/// If value, type, size, or data are missing or zero, only the registry key is created.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RegistryValue {
    /// Path to the registry key.
    ///
    /// Do not include `HKEY_LOCAL_MACHINE` or `HKEY_CURRENT_USER` in the registry path.
    /// The location of the file determines which of these keys are used.
    pub key: String,

    /// The name of the registry value.
    ///
    /// The following values have special meaning for this field:
    ///
    /// | Value | Meaning |
    /// |-------|---------|
    /// | `**DeleteValues` | A semicolon-delimited list of values to delete. Use as a value of the associated key. |
    /// | `**Del.<valuename>` | Deletes a single value. Use as a value of the associated key. |
    /// | `**DelVals` | Deletes all values in a key. Use as a value of the associated key. |
    /// | `**DeleteKeys` | A semicolon-delimited list of keys to delete. |
    /// | `**SecureKey` | See [Access Rights and Access Masks](//msdn.microsoft.com/en-us/library/aa374902%28v=vs.85%29.aspx). |
    pub value: Option<String>,

    /// The data type.
    pub data_type: Option<RegistryValueType>,

    /// The user-supplied data.
    pub data: Option<Vec<u8>>,
}


/// Parse the [Windows Registry Policy File format](https://msdn.microsoft.com/en-us/library/aa374407%28v=vs.85%29.aspx) from
/// the specified bytes.
pub fn parse(data: &[u8]) -> Result<Vec<RegistryValue>, nom::IError> {
    parser::parse(data).to_full_result()
}
