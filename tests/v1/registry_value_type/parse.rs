use registry_pol::v1::RegistryValueType;


#[test]
fn reg_none() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_NONE as u32), Some(RegistryValueType::REG_NONE));
}

#[test]
fn reg_sz() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_SZ as u32), Some(RegistryValueType::REG_SZ));
}

#[test]
fn reg_expand_sz() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_EXPAND_SZ as u32),
               Some(RegistryValueType::REG_EXPAND_SZ));
}

#[test]
fn reg_binary() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_BINARY as u32),
               Some(RegistryValueType::REG_BINARY));
}

#[test]
fn reg_dword() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_DWORD as u32),
               Some(RegistryValueType::REG_DWORD));
}

#[test]
fn reg_dword_alt() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_DWORD_ALT as u32),
               Some(RegistryValueType::REG_DWORD_ALT));
}

#[test]
fn reg_link() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_LINK as u32), Some(RegistryValueType::REG_LINK));
}

#[test]
fn reg_multi_sz() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_MULTI_SZ as u32),
               Some(RegistryValueType::REG_MULTI_SZ));
}

#[test]
fn reg_qword() {
    assert_eq!(RegistryValueType::parse(RegistryValueType::REG_QWORD as u32),
               Some(RegistryValueType::REG_QWORD));
}

#[test]
fn invalid() {
    for i in 8..11 {
        assert_eq!(RegistryValueType::parse(i), None);
    }
    for i in 12..0xFFFF {
        assert_eq!(RegistryValueType::parse(i), None);
    }
}
