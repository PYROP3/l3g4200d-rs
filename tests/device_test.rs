use l3g4200d::gyro::L3G4200D;

#[test]
fn init() {
    let result = L3G4200D::new_safe(1);
    assert!(result.is_ok(), "Device should be available (is /dev/mem read/writeable by the current user?)");
}

#[test]
fn init_unknown() {
    let result = L3G4200D::new_safe(99);
    assert!(result.is_err(), "Device should not exist at non-existent I2C channel");
}

#[test]
fn who_am_i() {
    let result = L3G4200D::new(1).who_am_i().unwrap_or(0);
    assert_eq!(result, 0xD3, "WHO_AM_I constant is incorrect (are you using the correct sensor?)");
}

#[test]
fn read_position() {
    let result = L3G4200D::new(1).read_position();
    assert!(result.is_ok(), "Position data should be available");
}