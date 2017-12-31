use registry_pol::v1::{RegistryValueType, RegistryValue, parse};


static EMPTY_DATA: &[u8] = include_bytes!("../../test-data/Empty.pol");
static MACHINE_REGISTRY_DATA: &[u8] = include_bytes!("../../test-data/Machine-Registry.pol");
static USER_REGISTRY_DATA: &[u8] = include_bytes!("../../test-data/User-Registry.pol");


#[test]
fn empty() {
    assert_eq!(parse(EMPTY_DATA), Ok(vec![]));
}

#[test]
fn machine() {
    assert_eq!(parse(MACHINE_REGISTRY_DATA),
               Ok(vec![RegistryValue {
                           key: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer".to_string(),
                           value: Some("NoDriveTypeAutorun".to_string()),
                           data_type: Some(RegistryValueType::REG_DWORD),
                           data: Some(vec![0x9E, 0x00, 0x00, 0x00]),
                       }]));
}

#[test]
fn user() {
    assert_eq!(parse(USER_REGISTRY_DATA),
               Ok(vec![RegistryValue {
                           key: r"Software\Microsoft\Windows\CurrentVersion\Policies\ActiveDesktop".to_string(),
                           value: Some("**del.NoChangingWallPaper".to_string()),
                           data_type: Some(RegistryValueType::REG_SZ),
                           data: Some(vec![0x20, 0x00, 0x00, 0x00]),
                       }]));
}
