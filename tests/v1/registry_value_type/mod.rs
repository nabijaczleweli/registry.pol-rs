mod parse;

use registry_pol::v1::RegistryValueType;


#[test]
fn dword_little_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_LITTLE_ENDIAN, RegistryValueType::REG_DWORD);
}

#[test]
fn qword_little_endian() {
    assert_eq!(RegistryValueType::REG_QWORD_LITTLE_ENDIAN, RegistryValueType::REG_QWORD);
}
