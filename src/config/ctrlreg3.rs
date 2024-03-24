use bitmask::bitmask;

bitmask! {
    mask Attrs: u8 where flags Flags {
        I1Int1 = 0x80,
        I1Boot1 = 0x40,
        HLActive = 0x20,
        PushPullOpenDrain = 0x10,
        I2DataReady = 0x08,
        I2Watermark = 0x04,
        I2Overrun = 0x02,
        I2Empty = 0x01,
    }
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub i1_int1: bool,
    pub i1_boot1: bool,
    pub h_lactive: bool,
    pub pp_od: bool,
    pub i2_drdy: bool,
    pub i2_wtm: bool,
    pub i2_orun: bool,
    pub i2_empty: bool
}

impl Value {
    pub fn to_value(self) -> u8 {
        let i1_int1:   u8 = if self.i1_int1  { *Flags::I1Int1 }            else { 0 };
        let i1_boot1:  u8 = if self.i1_boot1 { *Flags::I1Boot1 }           else { 0 };
        let h_lactive: u8 = if self.h_lactive{ *Flags::HLActive }          else { 0 };
        let pp_od:     u8 = if self.pp_od    { *Flags::PushPullOpenDrain } else { 0 };
        let i2_drdy:   u8 = if self.i2_drdy  { *Flags::I2DataReady }       else { 0 };
        let i2_wtm:    u8 = if self.i2_wtm   { *Flags::I2Watermark }       else { 0 };
        let i2_orun:   u8 = if self.i2_orun  { *Flags::I2Overrun }         else { 0 };
        let i2_empty:  u8 = if self.i2_empty { *Flags::I2Empty }           else { 0 };
        i1_int1 | i1_boot1 | h_lactive | pp_od | i2_drdy | i2_wtm | i2_orun | i2_empty
    }
}

impl Default for Value {
    fn default() -> Self { 
        Value {
            i1_int1: false,
            i1_boot1: false,
            h_lactive: false,
            pp_od: false,
            i2_drdy: false,
            i2_wtm: false,
            i2_orun: false,
            i2_empty: false,
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value {
            i1_int1: *Flags::I1Int1 & value != 0,
            i1_boot1: *Flags::I1Boot1 & value != 0,
            h_lactive: *Flags::HLActive & value != 0,
            pp_od: *Flags::PushPullOpenDrain & value != 0,
            i2_drdy: *Flags::I2DataReady & value != 0,
            i2_wtm: *Flags::I2Watermark & value != 0,
            i2_orun: *Flags::I2Overrun & value != 0,
            i2_empty: *Flags::I2Empty & value != 0,
        }
    }
}