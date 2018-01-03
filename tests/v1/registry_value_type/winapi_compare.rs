use registry_pol::v1::RegistryValueType;
use winapi::um::winnt;


#[test]
fn reg_none() {
    assert_eq!(RegistryValueType::REG_NONE as u32, winnt::REG_NONE);
}

#[test]
fn reg_sz() {
    assert_eq!(RegistryValueType::REG_SZ as u32, winnt::REG_SZ);
}

#[test]
fn reg_expand_sz() {
    assert_eq!(RegistryValueType::REG_EXPAND_SZ as u32, winnt::REG_EXPAND_SZ);
}

#[test]
fn reg_binary() {
    assert_eq!(RegistryValueType::REG_BINARY as u32, winnt::REG_BINARY);
}

#[test]
fn reg_dword() {
    assert_eq!(RegistryValueType::REG_DWORD as u32, winnt::REG_DWORD);
}

#[test]
fn reg_dword_alt() {
    assert_eq!(RegistryValueType::REG_DWORD_BIG_ENDIAN as u32, winnt::REG_DWORD_BIG_ENDIAN);
}

#[test]
fn reg_link() {
    assert_eq!(RegistryValueType::REG_LINK as u32, winnt::REG_LINK);
}

#[test]
fn reg_multi_sz() {
    assert_eq!(RegistryValueType::REG_MULTI_SZ as u32, winnt::REG_MULTI_SZ);
}

#[test]
fn reg_qword() {
    assert_eq!(RegistryValueType::REG_QWORD as u32, winnt::REG_QWORD);
}

#[test]
fn reg_dword_little_endian() {
    assert_eq!(RegistryValueType::REG_DWORD_LITTLE_ENDIAN as u32, winnt::REG_DWORD_LITTLE_ENDIAN);
}

#[test]
fn reg_qword_little_endian() {
    assert_eq!(RegistryValueType::REG_QWORD_LITTLE_ENDIAN as u32, winnt::REG_QWORD_LITTLE_ENDIAN);
}
