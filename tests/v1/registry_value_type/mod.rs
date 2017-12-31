mod parse;

use registry_pol::v1::RegistryValueType;


#[test]
fn qword_little_endian() {
    assert_eq!(RegistryValueType::REG_QWORD_LITTLE_ENDIAN(), RegistryValueType::REG_QWORD);
}

#[test]
#[cfg(target_endian = "little")]
fn dword_little_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_LITTLE_ENDIAN(), RegistryValueType::REG_DWORD);
}

#[test]
#[cfg(target_endian = "big")]
fn dword_little_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_LITTLE_ENDIAN(), RegistryValueType::REG_DWORD_ALT);
}

#[test]
#[cfg(target_endian = "little")]
fn dword_big_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_BIG_ENDIAN(), RegistryValueType::REG_DWORD_ALT);
}

#[test]
#[cfg(target_endian = "big")]
fn dword_big_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_BIG_ENDIAN(), RegistryValueType::REG_DWORD);
}
