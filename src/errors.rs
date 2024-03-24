use thiserror::Error;
use i2cdev::linux::LinuxI2CError;

#[derive(Debug, Error)]
pub enum L3G4200DError {
    #[error("Register is not writable")]
    NotWritableRegister(),
    #[error("Attempt to write to protected bits")]
    WriteProtectedRegister(),
    #[error("I2C error")]
    I2CError(#[from] LinuxI2CError),
}