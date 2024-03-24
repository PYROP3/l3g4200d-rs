use bitmask::bitmask;

#[derive(PartialEq, Debug)]
pub struct RebootMemoryContent(bool);

impl RebootMemoryContent {
    pub const NORMAL_MODE:           RebootMemoryContent = RebootMemoryContent {0: false};
    pub const REBOOT_MEMORY_CONTENT: RebootMemoryContent = RebootMemoryContent {0: true};
}

// pub struct Int1SelectionConfiguration(u8);

// impl Int1SelectionConfiguration {
//     pub const NORMAL_MODE:           RebootMemoryContent = RebootMemoryContent {0: 0x00};
//     pub const REBOOT_MEMORY_CONTENT: RebootMemoryContent = RebootMemoryContent {0: 0x80};
// }

// pub struct OutSelectionConfiguration(u8);

// impl OutSelectionConfiguration {
//     pub const NO_FILTERING:         OutSelectionConfiguration = OutSelectionConfiguration {0: 0x00};
//     pub const NO_FILTERING:         OutSelectionConfiguration = OutSelectionConfiguration {0: 0x00};
//     pub const NO_FILTERING:         OutSelectionConfiguration = OutSelectionConfiguration {0: 0x00};
// }

#[derive(PartialEq, Debug)]
pub struct FilteringConfiguration(u8);

impl FilteringConfiguration {
    // High pass filter disabled
    pub const OUT_LPF1_INT_LPF1:                   FilteringConfiguration = FilteringConfiguration {0: 0x00};     // 0 00 00
    pub const OUT_LPF1_HPF_INT_LPF1:               FilteringConfiguration = FilteringConfiguration {0: 0x01};     // 0 00 01
    pub const OUT_LPF1_LPF2_INT_LPF1:              FilteringConfiguration = FilteringConfiguration {0: 0x02};     // 0 00 10
    //pub const OUT_LPF1_LPF2_INT_LPF1:              FilteringConfiguration = FilteringConfiguration {0: 0x03};     // 0 00 11
    pub const OUT_LPF1_INT_LPF1_HPF:               FilteringConfiguration = FilteringConfiguration {0: 0x04};     // 0 01 00
    pub const OUT_LPF1_HPF_INT_LPF1_HPF:           FilteringConfiguration = FilteringConfiguration {0: 0x05};     // 0 01 01
    pub const OUT_LPF1_LPF2_INT_LPF1_HPF:          FilteringConfiguration = FilteringConfiguration {0: 0x06};     // 0 01 10
    //pub const OUT_LPF1_LPF2_INT_LPF1_HPF:          FilteringConfiguration = FilteringConfiguration {0: 0x07};     // 0 01 11
    pub const OUT_LPF1_INT_LPF1_LPF2:              FilteringConfiguration = FilteringConfiguration {0: 0x08};     // 0 10 00
    pub const OUT_LPF1_HPF_INT_LPF1_LPF2:          FilteringConfiguration = FilteringConfiguration {0: 0x09};     // 0 10 01
    pub const OUT_LPF1_LPF2_INT_LPF1_LPF2:         FilteringConfiguration = FilteringConfiguration {0: 0x0A};     // 0 10 10
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x0B};     // 0 10 11
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x0C};     // 0 11 00
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x0D};     // 0 11 01
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x0E};     // 0 11 10
    ////pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x0F};     // 0 11 11

    // High pass filter enable
    //pub const XXXXXXXXXXXXXXXXXXXXX:               FilteringConfiguration = FilteringConfiguration {0: 0x10};     // 1 00 00
    //pub const XXXXXXXXXXXXXXXXXXXXX:               FilteringConfiguration = FilteringConfiguration {0: 0x11};     // 1 00 01
    pub const OUT_LPF1_HPF_LPF2_INT_LPF1:          FilteringConfiguration = FilteringConfiguration {0: 0x12};     // 1 00 10
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x13};     // 1 00 11
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x14};     // 1 01 00
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x15};     // 1 01 01
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x16};     // 1 01 10
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x17};     // 1 01 11
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x18};     // 1 10 00
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x19};     // 1 10 01
    pub const OUT_LPF1_HPF_LPF2_INT_LPF1_HPF_LPF2: FilteringConfiguration = FilteringConfiguration {0: 0x1A};     // 1 10 10
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x1B};     // 1 10 11
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x1C};     // 1 11 00
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x1D};     // 1 11 01
    //pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x1E};     // 1 11 10
    ////pub const XXXXXXXXXXXXXXXXX:                   FilteringConfiguration = FilteringConfiguration {0: 0x1F};     // 1 11 11
}

bitmask! {
    mask Attrs: u8 where flags Flags {
        Boot = 0x80,
        FifoEnable = 0x40,
        Filtering = 0x1F,
    }
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub reboot_memory_content: RebootMemoryContent,
    pub filtering_configuration: FilteringConfiguration,
    pub fifo_enable: bool,
}

impl Value {
    pub fn to_value(self) -> u8 {
        let reboot_memory_content: u8 = if self.reboot_memory_content.0 { *Flags::Boot } else { 0 };
        let filtering_configuration: u8 = self.filtering_configuration.0;
        let fifo_enable: u8 = if self.fifo_enable { *Flags::FifoEnable } else { 0 };
        reboot_memory_content | filtering_configuration | fifo_enable
    }
}

impl Default for Value {
    fn default() -> Self { 
        Value {
            reboot_memory_content: RebootMemoryContent::NORMAL_MODE,
            filtering_configuration: FilteringConfiguration::OUT_LPF1_INT_LPF1,
            fifo_enable: false,
        }
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value {
            reboot_memory_content: RebootMemoryContent(value & *Flags::Boot != 0),
            filtering_configuration: FilteringConfiguration(value & *Flags::Filtering),
            fifo_enable: value & *Flags::FifoEnable != 0,
        }
    }
}
