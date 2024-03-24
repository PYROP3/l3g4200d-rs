use bitmask::bitmask;

#[derive(PartialEq, Debug)]
pub struct BlockDataUpdate(bool);

impl BlockDataUpdate {
    pub const CONTINUOUS_UPDATE: BlockDataUpdate = BlockDataUpdate {0: false};
    pub const WAIT_FOR_READING:  BlockDataUpdate = BlockDataUpdate {0: true};
}

#[derive(PartialEq, Debug)]
pub struct BigLittleEndian(bool);

impl BigLittleEndian {
    pub const LITTLE_ENDIAN: BigLittleEndian = BigLittleEndian {0: false};
    pub const BIG_ENDIAN:    BigLittleEndian = BigLittleEndian {0: true};
}

#[derive(PartialEq, Debug)]
pub struct FullScaleSelection(u8);

impl FullScaleSelection {
    pub const FSS_250_DPS:  FullScaleSelection = FullScaleSelection {0: 0x00};
    pub const FSS_500_DPS:  FullScaleSelection = FullScaleSelection {0: 0x10};
    pub const FSS_2000_DPS: FullScaleSelection = FullScaleSelection {0: 0x20};
}

#[derive(PartialEq, Debug)]
pub struct SelfTestEnabled(u8);

impl SelfTestEnabled {
    pub const NORMAL_MODE: SelfTestEnabled = SelfTestEnabled {0: 0x00};
    pub const SELF_TEST_0: SelfTestEnabled = SelfTestEnabled {0: 0x02};
    pub const SELF_TEST_1: SelfTestEnabled = SelfTestEnabled {0: 0x06};
}

#[derive(PartialEq, Debug)]
pub struct SerialInterfaceMode(bool);

impl SerialInterfaceMode {
    pub const SPI_4_WIRE: SerialInterfaceMode = SerialInterfaceMode {0: false};
    pub const SPI_3_WIRE: SerialInterfaceMode = SerialInterfaceMode {0: true};
}

bitmask! {
    mask Attrs: u8 where flags Flags {
        Bdu = 0x80,
        Ble = 0x40,
        Fss = 0x30,
        Ste = 0x06,
        Sim = 0x01,
    }
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub block_data_update:     BlockDataUpdate,
    pub big_little_endian:     BigLittleEndian,
    pub full_scale_select:     FullScaleSelection,
    pub self_test_enabled:     SelfTestEnabled,
    pub serial_interface_mode: SerialInterfaceMode,
}

impl Value {
    pub fn to_value(self) -> u8 {
        let block_data_update:     u8 = if self.block_data_update.0     { *Flags::Bdu } else { 0 };
        let big_little_endian:     u8 = if self.big_little_endian.0     { *Flags::Ble } else { 0 };
        let full_scale_select:     u8 = self.full_scale_select.0;
        let self_test_enabled:     u8 = self.self_test_enabled.0;
        let serial_interface_mode: u8 = if self.serial_interface_mode.0 { *Flags::Sim } else { 0 };

        block_data_update | big_little_endian | full_scale_select | self_test_enabled | serial_interface_mode
    }
}

impl Default for Value {
    fn default() -> Self { 
        Value {
            block_data_update: BlockDataUpdate::CONTINUOUS_UPDATE,
            big_little_endian: BigLittleEndian::LITTLE_ENDIAN,
            full_scale_select: FullScaleSelection::FSS_250_DPS,
            self_test_enabled: SelfTestEnabled::NORMAL_MODE,
            serial_interface_mode: SerialInterfaceMode::SPI_4_WIRE,
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value {
            block_data_update: BlockDataUpdate(value & *Flags::Bdu != 0),
            big_little_endian: BigLittleEndian(value & *Flags::Ble != 0),
            full_scale_select: FullScaleSelection(value & *Flags::Fss),
            self_test_enabled: SelfTestEnabled(value & *Flags::Ste),
            serial_interface_mode: SerialInterfaceMode(value & *Flags::Sim != 0),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_reg_4() {
        let config: Value = Value::default();
        assert_eq!(config.to_value(), 0x0);
    }

    #[test]
    fn parse_config_reg_4_from_u8_1() {
        let config: Value = Value::from(0x20);
        assert_eq!(config.to_value(), 0x20);
    }
}