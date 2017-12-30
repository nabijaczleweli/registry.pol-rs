//! Registry Policy File format version 1.


/// Second part of header.
pub const REGISTRY_FILE_VERSION: u32 = 1;


/// The data type field can contain any of the registry value types defined in WinNT.h.
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RegistryValueType {
    REG_NONE = 0,
    REG_SZ = 1,
    REG_EXPAND_SZ = 2,
    REG_BINARY = 3,
    REG_DWORD = 4,
    /// The non-native byte ordering for a `u32`.
    REG_DWORD_ALT = 5,
    REG_LINK = 6,
    REG_MULTI_SZ = 7,
    REG_QWORD = 11,
}

#[allow(non_snake_case)]
impl RegistryValueType {
    pub fn REG_DWORD_LITTLE_ENDIAN() -> RegistryValueType {
        if cfg!(target_endian = "little") {
            RegistryValueType::REG_DWORD
        } else {
            RegistryValueType::REG_DWORD_ALT
        }
    }

    pub fn REG_DWORD_BIG_ENDIAN() -> RegistryValueType {
        if cfg!(target_endian = "little") {
            RegistryValueType::REG_DWORD_ALT
        } else {
            RegistryValueType::REG_DWORD
        }
    }

    pub fn REG_QWORD_LITTLE_ENDIAN() -> RegistryValueType {
        RegistryValueType::REG_QWORD
    }
}


/// A representation of registry value version 1.
///
/// If value, type, size, or data are missing or zero, only the registry key is created.
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
