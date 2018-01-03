mod parse;
#[cfg(target_os = "windows")]
mod winapi_compare;

use registry_pol::v1::RegistryValueType;


#[test]
fn reg_dword_little_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_LITTLE_ENDIAN, RegistryValueType::REG_DWORD);
}

#[test]
fn reg_qword_little_endian() {
    assert_eq!(RegistryValueType::REG_QWORD_LITTLE_ENDIAN, RegistryValueType::REG_QWORD);
}
