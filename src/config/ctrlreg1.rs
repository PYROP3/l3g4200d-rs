use bitmask::bitmask;

#[derive(PartialEq, Debug)]
pub struct DataRateBandwidth(u8);

impl DataRateBandwidth {
    pub const ODR_100_CUT_OFF_12_5: DataRateBandwidth = DataRateBandwidth {0: 0x00};
    pub const ODR_100_CUT_OFF_25:   DataRateBandwidth = DataRateBandwidth {0: 0x10};
    pub const ODR_200_CUT_OFF_12_5: DataRateBandwidth = DataRateBandwidth {0: 0x40};
    pub const ODR_200_CUT_OFF_25:   DataRateBandwidth = DataRateBandwidth {0: 0x50};
    pub const ODR_200_CUT_OFF_50:   DataRateBandwidth = DataRateBandwidth {0: 0x60};
    pub const ODR_200_CUT_OFF_70:   DataRateBandwidth = DataRateBandwidth {0: 0x70};
    pub const ODR_400_CUT_OFF_20:   DataRateBandwidth = DataRateBandwidth {0: 0x80};
    pub const ODR_400_CUT_OFF_25:   DataRateBandwidth = DataRateBandwidth {0: 0x90};
    pub const ODR_400_CUT_OFF_50:   DataRateBandwidth = DataRateBandwidth {0: 0xA0};
    pub const ODR_400_CUT_OFF_110:  DataRateBandwidth = DataRateBandwidth {0: 0xB0};
    pub const ODR_800_CUT_OFF_30:   DataRateBandwidth = DataRateBandwidth {0: 0xC0};
    pub const ODR_800_CUT_OFF_35:   DataRateBandwidth = DataRateBandwidth {0: 0xD0};
    pub const ODR_800_CUT_OFF_50:   DataRateBandwidth = DataRateBandwidth {0: 0xE0};
    pub const ODR_800_CUT_OFF_110:  DataRateBandwidth = DataRateBandwidth {0: 0xF0};
}

#[derive(PartialEq, Debug)]
pub struct PowerDownMode(bool);

impl PowerDownMode {
    pub const POWER_DOWN_MODE_ENABLE: PowerDownMode = PowerDownMode {0: false};
    pub const NORMAL_MODE:            PowerDownMode = PowerDownMode {0: true};
    pub const SLEEP_MODE:             PowerDownMode = PowerDownMode {0: true};
}

bitmask! {
    mask Attrs: u8 where flags Flags {
        Drb = 0xF0,
        Pow = 0x08,
        Zen = 0x04,
        Yen = 0x02,
        Xen = 0x01,
    }
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub dr_bw: DataRateBandwidth, 
    pub power_down_mode_enable: PowerDownMode, 
    pub x_enable: bool, 
    pub y_enable: bool, 
    pub z_enable: bool,
}

impl Value {
    pub fn to_value(self) -> u8 {
        let pd:  u8 = if self.power_down_mode_enable.0 { *Flags::Pow } else { 0 };
        let xen: u8 = if self.x_enable { *Flags::Xen } else { 0 };
        let yen: u8 = if self.y_enable { *Flags::Yen } else { 0 };
        let zen: u8 = if self.z_enable { *Flags::Zen } else { 0 };
        self.dr_bw.0 | pd | xen | yen | zen
    }
}

impl Default for Value {
    fn default() -> Self { 
        Value {
            dr_bw: DataRateBandwidth::ODR_200_CUT_OFF_50,
            power_down_mode_enable: PowerDownMode::POWER_DOWN_MODE_ENABLE,
            x_enable: true,
            y_enable: true,
            z_enable: true, 
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value {
            dr_bw: DataRateBandwidth(value & *Flags::Drb),
            power_down_mode_enable: PowerDownMode(value & *Flags::Pow != 0),
            x_enable: *Flags::Xen & value != 0,
            y_enable: *Flags::Yen & value != 0,
            z_enable: *Flags::Zen & value != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_reg_1() {
        let config: Value = Value::default();
        assert_eq!(config.to_value(), 0x67);
    }

    #[test]
    fn parse_config_reg_1_1() {
        let config: Value = Value {
            dr_bw: DataRateBandwidth::ODR_200_CUT_OFF_70,
            power_down_mode_enable: PowerDownMode::NORMAL_MODE,
            x_enable: false,
            y_enable: false,
            z_enable: true,
        };
        assert_eq!(config.to_value(), 0x7C);
    }

    #[test]
    fn parse_config_reg_1_from_u8_1() {
        let config: Value = Value::from(0x6F);
        //assert_eq!(config.to_value(), 0x6F);
        println!("{:?}", config);
    }
}