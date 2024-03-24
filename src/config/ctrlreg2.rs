#![allow(non_upper_case_globals)]

use bitmask::bitmask;

#[derive(PartialEq, Debug)]
pub struct HighPassFilterMode(u8);

impl HighPassFilterMode {
    pub const HP_RESET_FILTER:  HighPassFilterMode = HighPassFilterMode {0: 0x00};
    pub const REFERENCE_SIGNAL: HighPassFilterMode = HighPassFilterMode {0: 0x10};
    pub const NORMAL_MODE:      HighPassFilterMode = HighPassFilterMode {0: 0x20};
    pub const AUTORESET_ON_INT: HighPassFilterMode = HighPassFilterMode {0: 0x30};
}

#[derive(PartialEq, Debug)]
pub struct HighPassFilterCutOff(u8);

impl HighPassFilterCutOff {
    pub const ODR_100_8_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x00};
    pub const ODR_200_15_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x00};
    pub const ODR_400_30_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x00};
    pub const ODR_800_46_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x00};

    pub const ODR_100_4_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x01};
    pub const ODR_200_8_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x01};
    pub const ODR_400_15_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x01};
    pub const ODR_800_30_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x01};

    pub const ODR_100_2_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x02};
    pub const ODR_200_4_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x02};
    pub const ODR_400_8_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x02};
    pub const ODR_800_15_HZ:   HighPassFilterCutOff = HighPassFilterCutOff {0: 0x02};

    pub const ODR_100_1_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x03};
    pub const ODR_200_2_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x03};
    pub const ODR_400_4_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x03};
    pub const ODR_800_8_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x03};

    pub const ODR_100_500_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x04};
    pub const ODR_200_1_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x04};
    pub const ODR_400_2_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x04};
    pub const ODR_800_4_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x04};

    pub const ODR_100_200_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x05};
    pub const ODR_200_500_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x05};
    pub const ODR_400_1_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x05};
    pub const ODR_800_2_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x05};

    pub const ODR_100_100_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x06};
    pub const ODR_200_200_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x06};
    pub const ODR_400_500_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x06};
    pub const ODR_800_1_HZ:    HighPassFilterCutOff = HighPassFilterCutOff {0: 0x06};

    pub const ODR_100_50_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x07};
    pub const ODR_200_100_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x07};
    pub const ODR_400_200_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x07};
    pub const ODR_800_500_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x07};

    pub const ODR_100_20_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x08};
    pub const ODR_200_50_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x08};
    pub const ODR_400_100_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x08};
    pub const ODR_800_200_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x08};

    pub const ODR_100_10_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x09};
    pub const ODR_200_20_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x09};
    pub const ODR_400_50_mHZ:  HighPassFilterCutOff = HighPassFilterCutOff {0: 0x09};
    pub const ODR_800_100_mHZ: HighPassFilterCutOff = HighPassFilterCutOff {0: 0x09};
}

bitmask! {
    mask Attrs: u8 where flags Flags {
        Hpm = 0x30,
        Hpcf = 0x0F,
    }
}

#[derive(PartialEq, Debug)]
pub struct Value {
    hpm: HighPassFilterMode,
    hpcf: HighPassFilterCutOff,
}

impl Value {
    pub fn to_value(self) -> u8 {
        self.hpm.0 | self.hpcf.0
    }
}

impl Default for Value {
    fn default() -> Self { 
        Value {
            hpm: HighPassFilterMode::NORMAL_MODE,
            hpcf: HighPassFilterCutOff::ODR_100_8_HZ,
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value {
            hpm: HighPassFilterMode(value & *Flags::Hpm),
            hpcf: HighPassFilterCutOff(value & *Flags::Hpcf),
        }
    }
}