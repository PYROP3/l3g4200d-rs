
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

use crate::config;
use crate::registers::L3G4200DRegister;
use crate::errors::L3G4200DError;

const GYRO_K: i16 = 240; // 21845. / 90.;
pub const L3G4200D_ADDR: u16 = 0x69;
pub const L3G4200D_DEV_ID: u8 = 0xD3;

pub struct L3G4200D {
    i2c: LinuxI2CDevice,
    coords: [i16; 3],
    drift_compensation: [i16; 3],
    temp_compensation: i8,
}

macro_rules! write_reg_fn {
    ( $f:ident, $reg:expr, $cfg:ty ) => {
        pub fn $f(&mut self, value: $cfg) -> Result<(), L3G4200DError> {
            self.write_byte_data($reg, value.to_value())
        }
    };
}

macro_rules! read_reg_fn {
    ( $f:ident, $reg:expr, $cfg:ty ) => {
        pub fn $f(&mut self) -> Result<$cfg, L3G4200DError> {
            self.read_byte_data($reg).map(|raw| raw.into())
        }
    };
}

impl L3G4200D {
    pub fn new(channel: u8) -> Self {
        L3G4200D { 
            i2c: LinuxI2CDevice::new(format!("/dev/i2c-{}", channel), L3G4200D_ADDR).expect("Failed to open I2C device"),
            coords: [0, 0, 0],
            drift_compensation: [0, 0, 0],
            temp_compensation: 0,
        }
    }

    pub fn new_safe(channel: u8) -> Result<Self, String> {
        if let Ok(device) = LinuxI2CDevice::new(format!("/dev/i2c-{}", channel), L3G4200D_ADDR) {
            Ok(L3G4200D { 
                i2c: device,
                coords: [0, 0, 0],
                drift_compensation: [0, 0, 0],
                temp_compensation: 0
            })
        } else {
            Err("Failed to open I2C device".to_string())
        }
    }

    pub fn write_byte_data(&mut self, reg: L3G4200DRegister, value: u8) -> Result<(), L3G4200DError> {
        reg.enforce_write_protect(value)?;
        self.i2c.smbus_write_byte_data(reg.reg(), value)
            .map_err(From::from)
    }

    pub fn read_byte_data(&mut self, reg: L3G4200DRegister) -> Result<u8, L3G4200DError> {
        self.i2c.smbus_read_byte_data(reg.reg())
            .map_err(From::from)
    }

    fn _to_u16(low: u8, high: u8) -> u16 {
        (high as u16) << 8 | low as u16
    }

    fn _from_twos(value: u16) -> i16 {
        if value & (1 << 15) != 0 {-((!value +1) as i16)} else { value as i16 }
    }

    fn _from_twos_u8(value: u8) -> i8 {
        if value & (1 << 7) != 0 {-((!value +1) as i8)} else { value as i8 }
    }

    pub fn read_raw_temperature_delta(&mut self) -> Result<u8, L3G4200DError> {
        self.read_byte_data(L3G4200DRegister::OUT_TEMP)
    }

    pub fn read_temperature(&mut self) -> Result<i8, L3G4200DError> {
        self.read_raw_temperature_delta().map(Self::_from_twos_u8).map(|temp| -1 * temp + self.temp_compensation)
    }

    pub fn read_raw_delta(&mut self) -> Result<(i16, i16, i16), L3G4200DError> {
        let x_l = self.read_byte_data(L3G4200DRegister::OUT_X_L)?;
        let x_h = self.read_byte_data(L3G4200DRegister::OUT_X_H)?;
        let y_l = self.read_byte_data(L3G4200DRegister::OUT_Y_L)?;
        let y_h = self.read_byte_data(L3G4200DRegister::OUT_Y_H)?;
        let z_l = self.read_byte_data(L3G4200DRegister::OUT_Z_L)?;
        let z_h = self.read_byte_data(L3G4200DRegister::OUT_Z_H)?;

        let x = Self::_from_twos(Self::_to_u16(x_l, x_h)) + self.drift_compensation[0];
        let y = Self::_from_twos(Self::_to_u16(y_l, y_h)) + self.drift_compensation[1];
        let z = Self::_from_twos(Self::_to_u16(z_l, z_h)) + self.drift_compensation[2];

        Ok((x, y, z))
    }

    pub fn read_position(&mut self) -> Result<(i16, i16, i16), L3G4200DError> {
        let (x, y, z) = self.read_raw_delta()?;
        self.coords[0] += x / GYRO_K;
        self.coords[1] += y / GYRO_K;
        self.coords[2] += z / GYRO_K;
        Ok((self.coords[0], self.coords[1], self.coords[2]))
    }

    pub fn read_delta_filtered<F>(&mut self, filter: F) -> Result<(i16, i16, i16), L3G4200DError> where F: Fn(i16) -> i16 {
        let (mut x, mut y, mut z) = self.read_raw_delta()?;

        x = filter(x);
        y = filter(y);
        z = filter(z);

        Ok((x, y, z))
    }

    pub fn read_position_filtered<F>(&mut self, filter: F) -> Result<(i16, i16, i16), L3G4200DError> where F: Fn(i16) -> i16 {
        let (x, y, z) = self.read_delta_filtered(filter)?;
        self.coords[0] += x / GYRO_K;
        self.coords[1] += y / GYRO_K;
        self.coords[2] += z / GYRO_K;
        Ok((self.coords[0], self.coords[1], self.coords[2]))
    }

    pub fn callibrate_drift(&mut self, iterations: i32) -> Result<[i32; 3], L3G4200DError> {
        let mut drift: [i32; 3] = [0, 0, 0];
        for _ in 0..iterations {
            let (x, y, z) = self.read_raw_delta()?;
            drift[0] += x as i32;
            drift[1] += y as i32;
            drift[2] += z as i32;
        }
        drift[0] /= iterations;
        drift[1] /= iterations;
        drift[2] /= iterations;

        self.drift_compensation[0] = -1 * (drift[0] as i16);
        self.drift_compensation[1] = -1 * (drift[1] as i16);
        self.drift_compensation[2] = -1 * (drift[2] as i16);

        Ok(drift)
    }

    pub fn callibrate_temperature(&mut self, current_temperature: Option<i8>) -> Result<i8, L3G4200DError> {
        self.temp_compensation = (-1 * self.read_temperature()?) + current_temperature.unwrap_or(0);

        Ok(-1 * self.temp_compensation)
    }

    pub fn common_setup(&mut self) -> Result <(), L3G4200DError> {
        use crate::config::*;

        self.write_config_reg_1(ctrlreg1::Value {
            dr_bw: ctrlreg1::DataRateBandwidth::ODR_200_CUT_OFF_50,
            power_down_mode_enable: ctrlreg1::PowerDownMode::NORMAL_MODE,
            ..Default::default()
        })?;
    
        self.write_config_reg_4(ctrlreg4::Value {
            full_scale_select: ctrlreg4::FullScaleSelection::FSS_2000_DPS,
            ..Default::default()
        })?;

        Ok(())
    }

    pub fn who_am_i(&mut self) -> Result<u8, L3G4200DError> {
        self.read_byte_data(L3G4200DRegister::WHO_AM_I)
    }

    write_reg_fn!{write_config_reg_1, L3G4200DRegister::CTRL_REG1, config::ctrlreg1::Value}
    write_reg_fn!{write_config_reg_2, L3G4200DRegister::CTRL_REG2, config::ctrlreg2::Value}
    write_reg_fn!{write_config_reg_3, L3G4200DRegister::CTRL_REG3, config::ctrlreg3::Value}
    write_reg_fn!{write_config_reg_4, L3G4200DRegister::CTRL_REG4, config::ctrlreg4::Value}
    write_reg_fn!{write_config_reg_5, L3G4200DRegister::CTRL_REG5, config::ctrlreg5::Value}

    read_reg_fn!{read_config_reg_1, L3G4200DRegister::CTRL_REG1, config::ctrlreg1::Value}
    read_reg_fn!{read_config_reg_2, L3G4200DRegister::CTRL_REG2, config::ctrlreg2::Value}
    read_reg_fn!{read_config_reg_3, L3G4200DRegister::CTRL_REG3, config::ctrlreg3::Value}
    read_reg_fn!{read_config_reg_4, L3G4200DRegister::CTRL_REG4, config::ctrlreg4::Value}
    read_reg_fn!{read_config_reg_5, L3G4200DRegister::CTRL_REG5, config::ctrlreg5::Value}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_twos_positive() {
        assert_eq!(L3G4200D::_from_twos(0x1), 1, "Mathematical error");
    }

    #[test]
    fn test_from_twos_negative() {
        assert_eq!(L3G4200D::_from_twos(0xFFFF), -1, "Mathematical error");
    }

    #[test]
    fn test_to_u16() {
        assert_eq!(L3G4200D::_to_u16(0xAB, 0xCD), 0xCDAB, "Mathematical error");
    }
}
