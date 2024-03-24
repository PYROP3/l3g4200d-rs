use std::fmt::{Debug, Display};

use crate::errors::L3G4200DError;

#[non_exhaustive]
#[derive(Debug)]
pub struct L3G4200DRegister {
    register: u8,
    read_only: bool,
    write_protect: u8,
}

impl Display for L3G4200DRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register)
    }
}

impl L3G4200DRegister {
    pub const WHO_AM_I:      L3G4200DRegister = L3G4200DRegister::readonly(0x0F);
    pub const CTRL_REG1:     L3G4200DRegister = L3G4200DRegister::writeable(0x20);
    pub const CTRL_REG2:     L3G4200DRegister = L3G4200DRegister { register: 0x21, write_protect: 0xC0, read_only: false };
    pub const CTRL_REG3:     L3G4200DRegister = L3G4200DRegister::writeable(0x22);
    pub const CTRL_REG4:     L3G4200DRegister = L3G4200DRegister::writeable(0x23);
    pub const CTRL_REG5:     L3G4200DRegister = L3G4200DRegister::writeable(0x24);
    pub const REFERENCE:     L3G4200DRegister = L3G4200DRegister::writeable(0x25);
    pub const OUT_TEMP:      L3G4200DRegister = L3G4200DRegister::readonly(0x26);
    pub const STATUS_REG:    L3G4200DRegister = L3G4200DRegister::readonly(0x27);
    pub const OUT_X_L:       L3G4200DRegister = L3G4200DRegister::readonly(0x28);
    pub const OUT_X_H:       L3G4200DRegister = L3G4200DRegister::readonly(0x29);
    pub const OUT_Y_L:       L3G4200DRegister = L3G4200DRegister::readonly(0x2A);
    pub const OUT_Y_H:       L3G4200DRegister = L3G4200DRegister::readonly(0x2B);
    pub const OUT_Z_L:       L3G4200DRegister = L3G4200DRegister::readonly(0x2C);
    pub const OUT_Z_H:       L3G4200DRegister = L3G4200DRegister::readonly(0x2D);
    pub const FIFO_CTRL:     L3G4200DRegister = L3G4200DRegister::writeable(0x2E);
    pub const FIFO_SRC:      L3G4200DRegister = L3G4200DRegister::readonly(0x2F);
    pub const INT1_CFG:      L3G4200DRegister = L3G4200DRegister::writeable(0x30);
    pub const INT1_SRC:      L3G4200DRegister = L3G4200DRegister::readonly(0x31);
    pub const INT1_THS_XH:   L3G4200DRegister = L3G4200DRegister::writeable(0x32);
    pub const INT1_THS_XL:   L3G4200DRegister = L3G4200DRegister::writeable(0x33);
    pub const INT1_THS_YH:   L3G4200DRegister = L3G4200DRegister::writeable(0x34);
    pub const INT1_THS_YL:   L3G4200DRegister = L3G4200DRegister::writeable(0x35);
    pub const INT1_THS_ZH:   L3G4200DRegister = L3G4200DRegister::writeable(0x36);
    pub const INT1_THS_ZL:   L3G4200DRegister = L3G4200DRegister::writeable(0x37);
    pub const INT1_DURATION: L3G4200DRegister = L3G4200DRegister::writeable(0x38);

    const fn readonly(register: u8) -> Self {
        Self { register, write_protect: 0xFF, read_only: true }
    }

    const fn writeable(register: u8) -> Self {
        Self { register, write_protect: 0x0, read_only: false }
    }

    pub fn enforce_write_protect(&self, value: u8) -> Result<(), L3G4200DError> {
        if self.read_only {
            return Err(L3G4200DError::NotWritableRegister());
        }
        if value & self.write_protect != 0 {
            return Err(L3G4200DError::WriteProtectedRegister());
        }
        Ok(())
    }

    pub fn reg(&self) -> u8 { self.register }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn enforce_write_protect_WHO_AM_I() {
        assert!(L3G4200DRegister::WHO_AM_I.enforce_write_protect(0x0).is_err(), "WHO_AM_I should be write-protected");
    }

    #[test]
    fn enforce_write_protect_OUT_X_L() {
        assert!(L3G4200DRegister::OUT_X_L.enforce_write_protect(0x0).is_err(), "OUT_X_L should be write-protected")
    }

    #[test]
    fn enforce_write_protect_OUT_X_H() {
        assert!(L3G4200DRegister::OUT_X_H.enforce_write_protect(0x0).is_err(), "OUT_X_H should be write-protected")
    }

    #[test]
    fn enforce_write_protect_OUT_Y_L() {
        assert!(L3G4200DRegister::OUT_Y_L.enforce_write_protect(0x0).is_err(), "OUT_Y_L should be write-protected")
    }

    #[test]
    fn enforce_write_protect_OUT_Y_H() {
        assert!(L3G4200DRegister::OUT_Y_H.enforce_write_protect(0x0).is_err(), "OUT_Y_H should be write-protected")
    }

    #[test]
    fn enforce_write_protect_OUT_Z_L() {
        assert!(L3G4200DRegister::OUT_Z_L.enforce_write_protect(0x0).is_err(), "OUT_Z_L should be write-protected")
    }

    #[test]
    fn enforce_write_protect_OUT_Z_H() {
        assert!(L3G4200DRegister::OUT_Z_H.enforce_write_protect(0x0).is_err(), "OUT_Z_H should be write-protected")
    }

}