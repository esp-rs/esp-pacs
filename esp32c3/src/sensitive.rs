#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SENSITIVE_ROM_TABLE_LOCK_REG"]
    pub rom_table_lock: ROM_TABLE_LOCK,
    #[doc = "0x04 - SENSITIVE_ROM_TABLE_REG"]
    pub rom_table: ROM_TABLE,
    #[doc = "0x08 - SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG"]
    pub privilege_mode_sel_lock: PRIVILEGE_MODE_SEL_LOCK,
    #[doc = "0x0c - SENSITIVE_PRIVILEGE_MODE_SEL_REG"]
    pub privilege_mode_sel: PRIVILEGE_MODE_SEL,
    #[doc = "0x10 - SENSITIVE_APB_PERIPHERAL_ACCESS_0_REG"]
    pub apb_peripheral_access_0: APB_PERIPHERAL_ACCESS_0,
    #[doc = "0x14 - SENSITIVE_APB_PERIPHERAL_ACCESS_1_REG"]
    pub apb_peripheral_access_1: APB_PERIPHERAL_ACCESS_1,
    #[doc = "0x18 - SENSITIVE_INTERNAL_SRAM_USAGE_0_REG"]
    pub internal_sram_usage_0: INTERNAL_SRAM_USAGE_0,
    #[doc = "0x1c - SENSITIVE_INTERNAL_SRAM_USAGE_1_REG"]
    pub internal_sram_usage_1: INTERNAL_SRAM_USAGE_1,
    #[doc = "0x20 - SENSITIVE_INTERNAL_SRAM_USAGE_3_REG"]
    pub internal_sram_usage_3: INTERNAL_SRAM_USAGE_3,
    #[doc = "0x24 - SENSITIVE_INTERNAL_SRAM_USAGE_4_REG"]
    pub internal_sram_usage_4: INTERNAL_SRAM_USAGE_4,
    #[doc = "0x28 - SENSITIVE_CACHE_TAG_ACCESS_0_REG"]
    pub cache_tag_access_0: CACHE_TAG_ACCESS_0,
    #[doc = "0x2c - SENSITIVE_CACHE_TAG_ACCESS_1_REG"]
    pub cache_tag_access_1: CACHE_TAG_ACCESS_1,
    #[doc = "0x30 - SENSITIVE_CACHE_MMU_ACCESS_0_REG"]
    pub cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    #[doc = "0x34 - SENSITIVE_CACHE_MMU_ACCESS_1_REG"]
    pub cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    #[doc = "0x38 - SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_spi2_pms_constrain_0: DMA_APBPERI_SPI2_PMS_CONSTRAIN_0,
    #[doc = "0x3c - SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_spi2_pms_constrain_1: DMA_APBPERI_SPI2_PMS_CONSTRAIN_1,
    #[doc = "0x40 - SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_uchi0_pms_constrain_0: DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0,
    #[doc = "0x44 - SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_uchi0_pms_constrain_1: DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1,
    #[doc = "0x48 - SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_i2s0_pms_constrain_0: DMA_APBPERI_I2S0_PMS_CONSTRAIN_0,
    #[doc = "0x4c - SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_i2s0_pms_constrain_1: DMA_APBPERI_I2S0_PMS_CONSTRAIN_1,
    #[doc = "0x50 - SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_mac_pms_constrain_0: DMA_APBPERI_MAC_PMS_CONSTRAIN_0,
    #[doc = "0x54 - SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_mac_pms_constrain_1: DMA_APBPERI_MAC_PMS_CONSTRAIN_1,
    #[doc = "0x58 - SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_backup_pms_constrain_0: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0,
    #[doc = "0x5c - SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_backup_pms_constrain_1: DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1,
    #[doc = "0x60 - SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_lc_pms_constrain_0: DMA_APBPERI_LC_PMS_CONSTRAIN_0,
    #[doc = "0x64 - SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_lc_pms_constrain_1: DMA_APBPERI_LC_PMS_CONSTRAIN_1,
    #[doc = "0x68 - SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_aes_pms_constrain_0: DMA_APBPERI_AES_PMS_CONSTRAIN_0,
    #[doc = "0x6c - SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_aes_pms_constrain_1: DMA_APBPERI_AES_PMS_CONSTRAIN_1,
    #[doc = "0x70 - SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_sha_pms_constrain_0: DMA_APBPERI_SHA_PMS_CONSTRAIN_0,
    #[doc = "0x74 - SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_sha_pms_constrain_1: DMA_APBPERI_SHA_PMS_CONSTRAIN_1,
    #[doc = "0x78 - SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_REG"]
    pub dma_apbperi_adc_dac_pms_constrain_0: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0,
    #[doc = "0x7c - SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_REG"]
    pub dma_apbperi_adc_dac_pms_constrain_1: DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1,
    #[doc = "0x80 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_0_REG"]
    pub dma_apbperi_pms_monitor_0: DMA_APBPERI_PMS_MONITOR_0,
    #[doc = "0x84 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_1_REG"]
    pub dma_apbperi_pms_monitor_1: DMA_APBPERI_PMS_MONITOR_1,
    #[doc = "0x88 - SENSITIVE_DMA_APBPERI_PMS_MONITOR_2_REG"]
    pub dma_apbperi_pms_monitor_2: DMA_APBPERI_PMS_MONITOR_2,
    #[doc = "0x8c - SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG"]
    pub dma_apbperi_pms_monitor_3: DMA_APBPERI_PMS_MONITOR_3,
    #[doc = "0x90 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_0:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0,
    #[doc = "0x94 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_1:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1,
    #[doc = "0x98 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_2:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2,
    #[doc = "0x9c - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_3:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3,
    #[doc = "0xa0 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_4:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4,
    #[doc = "0xa4 - SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_REG"]
    pub core_x_iram0_dram0_dma_split_line_constrain_5:
        CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5,
    #[doc = "0xa8 - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0_REG"]
    pub core_x_iram0_pms_constrain_0: CORE_X_IRAM0_PMS_CONSTRAIN_0,
    #[doc = "0xac - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1_REG"]
    pub core_x_iram0_pms_constrain_1: CORE_X_IRAM0_PMS_CONSTRAIN_1,
    #[doc = "0xb0 - SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2_REG"]
    pub core_x_iram0_pms_constrain_2: CORE_X_IRAM0_PMS_CONSTRAIN_2,
    #[doc = "0xb4 - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0_REG"]
    pub core_0_iram0_pms_monitor_0: CORE_0_IRAM0_PMS_MONITOR_0,
    #[doc = "0xb8 - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1_REG"]
    pub core_0_iram0_pms_monitor_1: CORE_0_IRAM0_PMS_MONITOR_1,
    #[doc = "0xbc - SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2_REG"]
    pub core_0_iram0_pms_monitor_2: CORE_0_IRAM0_PMS_MONITOR_2,
    #[doc = "0xc0 - SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0_REG"]
    pub core_x_dram0_pms_constrain_0: CORE_X_DRAM0_PMS_CONSTRAIN_0,
    #[doc = "0xc4 - SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1_REG"]
    pub core_x_dram0_pms_constrain_1: CORE_X_DRAM0_PMS_CONSTRAIN_1,
    #[doc = "0xc8 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0_REG"]
    pub core_0_dram0_pms_monitor_0: CORE_0_DRAM0_PMS_MONITOR_0,
    #[doc = "0xcc - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1_REG"]
    pub core_0_dram0_pms_monitor_1: CORE_0_DRAM0_PMS_MONITOR_1,
    #[doc = "0xd0 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2_REG"]
    pub core_0_dram0_pms_monitor_2: CORE_0_DRAM0_PMS_MONITOR_2,
    #[doc = "0xd4 - SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3_REG"]
    pub core_0_dram0_pms_monitor_3: CORE_0_DRAM0_PMS_MONITOR_3,
    #[doc = "0xd8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0_REG"]
    pub core_0_pif_pms_constrain_0: CORE_0_PIF_PMS_CONSTRAIN_0,
    #[doc = "0xdc - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG"]
    pub core_0_pif_pms_constrain_1: CORE_0_PIF_PMS_CONSTRAIN_1,
    #[doc = "0xe0 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2_REG"]
    pub core_0_pif_pms_constrain_2: CORE_0_PIF_PMS_CONSTRAIN_2,
    #[doc = "0xe4 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3_REG"]
    pub core_0_pif_pms_constrain_3: CORE_0_PIF_PMS_CONSTRAIN_3,
    #[doc = "0xe8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4_REG"]
    pub core_0_pif_pms_constrain_4: CORE_0_PIF_PMS_CONSTRAIN_4,
    #[doc = "0xec - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5_REG"]
    pub core_0_pif_pms_constrain_5: CORE_0_PIF_PMS_CONSTRAIN_5,
    #[doc = "0xf0 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6_REG"]
    pub core_0_pif_pms_constrain_6: CORE_0_PIF_PMS_CONSTRAIN_6,
    #[doc = "0xf4 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7_REG"]
    pub core_0_pif_pms_constrain_7: CORE_0_PIF_PMS_CONSTRAIN_7,
    #[doc = "0xf8 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG"]
    pub core_0_pif_pms_constrain_8: CORE_0_PIF_PMS_CONSTRAIN_8,
    #[doc = "0xfc - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG"]
    pub core_0_pif_pms_constrain_9: CORE_0_PIF_PMS_CONSTRAIN_9,
    #[doc = "0x100 - SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10_REG"]
    pub core_0_pif_pms_constrain_10: CORE_0_PIF_PMS_CONSTRAIN_10,
    #[doc = "0x104 - SENSITIVE_REGION_PMS_CONSTRAIN_0_REG"]
    pub region_pms_constrain_0: REGION_PMS_CONSTRAIN_0,
    #[doc = "0x108 - SENSITIVE_REGION_PMS_CONSTRAIN_1_REG"]
    pub region_pms_constrain_1: REGION_PMS_CONSTRAIN_1,
    #[doc = "0x10c - SENSITIVE_REGION_PMS_CONSTRAIN_2_REG"]
    pub region_pms_constrain_2: REGION_PMS_CONSTRAIN_2,
    #[doc = "0x110 - SENSITIVE_REGION_PMS_CONSTRAIN_3_REG"]
    pub region_pms_constrain_3: REGION_PMS_CONSTRAIN_3,
    #[doc = "0x114 - SENSITIVE_REGION_PMS_CONSTRAIN_4_REG"]
    pub region_pms_constrain_4: REGION_PMS_CONSTRAIN_4,
    #[doc = "0x118 - SENSITIVE_REGION_PMS_CONSTRAIN_5_REG"]
    pub region_pms_constrain_5: REGION_PMS_CONSTRAIN_5,
    #[doc = "0x11c - SENSITIVE_REGION_PMS_CONSTRAIN_6_REG"]
    pub region_pms_constrain_6: REGION_PMS_CONSTRAIN_6,
    #[doc = "0x120 - SENSITIVE_REGION_PMS_CONSTRAIN_7_REG"]
    pub region_pms_constrain_7: REGION_PMS_CONSTRAIN_7,
    #[doc = "0x124 - SENSITIVE_REGION_PMS_CONSTRAIN_8_REG"]
    pub region_pms_constrain_8: REGION_PMS_CONSTRAIN_8,
    #[doc = "0x128 - SENSITIVE_REGION_PMS_CONSTRAIN_9_REG"]
    pub region_pms_constrain_9: REGION_PMS_CONSTRAIN_9,
    #[doc = "0x12c - SENSITIVE_REGION_PMS_CONSTRAIN_10_REG"]
    pub region_pms_constrain_10: REGION_PMS_CONSTRAIN_10,
    #[doc = "0x130 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_0_REG"]
    pub core_0_pif_pms_monitor_0: CORE_0_PIF_PMS_MONITOR_0,
    #[doc = "0x134 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_1_REG"]
    pub core_0_pif_pms_monitor_1: CORE_0_PIF_PMS_MONITOR_1,
    #[doc = "0x138 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_2_REG"]
    pub core_0_pif_pms_monitor_2: CORE_0_PIF_PMS_MONITOR_2,
    #[doc = "0x13c - SENSITIVE_CORE_0_PIF_PMS_MONITOR_3_REG"]
    pub core_0_pif_pms_monitor_3: CORE_0_PIF_PMS_MONITOR_3,
    #[doc = "0x140 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_4_REG"]
    pub core_0_pif_pms_monitor_4: CORE_0_PIF_PMS_MONITOR_4,
    #[doc = "0x144 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_5_REG"]
    pub core_0_pif_pms_monitor_5: CORE_0_PIF_PMS_MONITOR_5,
    #[doc = "0x148 - SENSITIVE_CORE_0_PIF_PMS_MONITOR_6_REG"]
    pub core_0_pif_pms_monitor_6: CORE_0_PIF_PMS_MONITOR_6,
    #[doc = "0x14c - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0_REG"]
    pub backup_bus_pms_constrain_0: BACKUP_BUS_PMS_CONSTRAIN_0,
    #[doc = "0x150 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG"]
    pub backup_bus_pms_constrain_1: BACKUP_BUS_PMS_CONSTRAIN_1,
    #[doc = "0x154 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2_REG"]
    pub backup_bus_pms_constrain_2: BACKUP_BUS_PMS_CONSTRAIN_2,
    #[doc = "0x158 - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG"]
    pub backup_bus_pms_constrain_3: BACKUP_BUS_PMS_CONSTRAIN_3,
    #[doc = "0x15c - SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG"]
    pub backup_bus_pms_constrain_4: BACKUP_BUS_PMS_CONSTRAIN_4,
    #[doc = "0x160 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_0_REG"]
    pub backup_bus_pms_monitor_0: BACKUP_BUS_PMS_MONITOR_0,
    #[doc = "0x164 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_1_REG"]
    pub backup_bus_pms_monitor_1: BACKUP_BUS_PMS_MONITOR_1,
    #[doc = "0x168 - SENSITIVE_BACKUP_BUS_PMS_MONITOR_2_REG"]
    pub backup_bus_pms_monitor_2: BACKUP_BUS_PMS_MONITOR_2,
    #[doc = "0x16c - SENSITIVE_BACKUP_BUS_PMS_MONITOR_3_REG"]
    pub backup_bus_pms_monitor_3: BACKUP_BUS_PMS_MONITOR_3,
    #[doc = "0x170 - SENSITIVE_CLOCK_GATE_REG_REG"]
    pub clock_gate: CLOCK_GATE,
    _reserved93: [u8; 0x0e88],
    #[doc = "0xffc - SENSITIVE_DATE_REG"]
    pub date: DATE,
}
#[doc = "ROM_TABLE_LOCK (rw) register accessor: an alias for `Reg<ROM_TABLE_LOCK_SPEC>`"]
pub type ROM_TABLE_LOCK = crate::Reg<rom_table_lock::ROM_TABLE_LOCK_SPEC>;
#[doc = "SENSITIVE_ROM_TABLE_LOCK_REG"]
pub mod rom_table_lock;
#[doc = "ROM_TABLE (rw) register accessor: an alias for `Reg<ROM_TABLE_SPEC>`"]
pub type ROM_TABLE = crate::Reg<rom_table::ROM_TABLE_SPEC>;
#[doc = "SENSITIVE_ROM_TABLE_REG"]
pub mod rom_table;
#[doc = "PRIVILEGE_MODE_SEL_LOCK (rw) register accessor: an alias for `Reg<PRIVILEGE_MODE_SEL_LOCK_SPEC>`"]
pub type PRIVILEGE_MODE_SEL_LOCK =
    crate::Reg<privilege_mode_sel_lock::PRIVILEGE_MODE_SEL_LOCK_SPEC>;
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG"]
pub mod privilege_mode_sel_lock;
#[doc = "PRIVILEGE_MODE_SEL (rw) register accessor: an alias for `Reg<PRIVILEGE_MODE_SEL_SPEC>`"]
pub type PRIVILEGE_MODE_SEL = crate::Reg<privilege_mode_sel::PRIVILEGE_MODE_SEL_SPEC>;
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_REG"]
pub mod privilege_mode_sel;
#[doc = "APB_PERIPHERAL_ACCESS_0 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_0_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_0 =
    crate::Reg<apb_peripheral_access_0::APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0_REG"]
pub mod apb_peripheral_access_0;
#[doc = "APB_PERIPHERAL_ACCESS_1 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_1_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_1 =
    crate::Reg<apb_peripheral_access_1::APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1_REG"]
pub mod apb_peripheral_access_1;
#[doc = "INTERNAL_SRAM_USAGE_0 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_0_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_0 = crate::Reg<internal_sram_usage_0::INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0_REG"]
pub mod internal_sram_usage_0;
#[doc = "INTERNAL_SRAM_USAGE_1 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_1_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_1 = crate::Reg<internal_sram_usage_1::INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1_REG"]
pub mod internal_sram_usage_1;
#[doc = "INTERNAL_SRAM_USAGE_3 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_3_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_3 = crate::Reg<internal_sram_usage_3::INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3_REG"]
pub mod internal_sram_usage_3;
#[doc = "INTERNAL_SRAM_USAGE_4 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_4_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_4 = crate::Reg<internal_sram_usage_4::INTERNAL_SRAM_USAGE_4_SPEC>;
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_4_REG"]
pub mod internal_sram_usage_4;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_0_SPEC>`"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_0_REG"]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_1_SPEC>`"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_1_REG"]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_0_SPEC>`"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_0_REG"]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_1_SPEC>`"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_1_REG"]
pub mod cache_mmu_access_1;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_0::DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_spi2_pms_constrain_0;
#[doc = "DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_spi2_pms_constrain_1::DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_spi2_pms_constrain_1;
#[doc = "DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_uchi0_pms_constrain_0::DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_uchi0_pms_constrain_0;
#[doc = "DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_uchi0_pms_constrain_1::DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_uchi0_pms_constrain_1;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_0::DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_i2s0_pms_constrain_0;
#[doc = "DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_i2s0_pms_constrain_1::DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_i2s0_pms_constrain_1;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_MAC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_mac_pms_constrain_0::DMA_APBPERI_MAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_mac_pms_constrain_0;
#[doc = "DMA_APBPERI_MAC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_MAC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_MAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_mac_pms_constrain_1::DMA_APBPERI_MAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_mac_pms_constrain_1;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_backup_pms_constrain_0::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_backup_pms_constrain_0;
#[doc = "DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_backup_pms_constrain_1::DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_backup_pms_constrain_1;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_lc_pms_constrain_0::DMA_APBPERI_LC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_lc_pms_constrain_0;
#[doc = "DMA_APBPERI_LC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_LC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_lc_pms_constrain_1::DMA_APBPERI_LC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_lc_pms_constrain_1;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_AES_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_aes_pms_constrain_0::DMA_APBPERI_AES_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_aes_pms_constrain_0;
#[doc = "DMA_APBPERI_AES_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_AES_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_AES_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_aes_pms_constrain_1::DMA_APBPERI_AES_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_aes_pms_constrain_1;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SHA_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_sha_pms_constrain_0::DMA_APBPERI_SHA_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_sha_pms_constrain_0;
#[doc = "DMA_APBPERI_SHA_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_SHA_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_sha_pms_constrain_1::DMA_APBPERI_SHA_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_sha_pms_constrain_1;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_SPEC>`"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_0::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_REG"]
pub mod dma_apbperi_adc_dac_pms_constrain_0;
#[doc = "DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>`"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 =
    crate::Reg<dma_apbperi_adc_dac_pms_constrain_1::DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_REG"]
pub mod dma_apbperi_adc_dac_pms_constrain_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_0_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_0 =
    crate::Reg<dma_apbperi_pms_monitor_0::DMA_APBPERI_PMS_MONITOR_0_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0_REG"]
pub mod dma_apbperi_pms_monitor_0;
#[doc = "DMA_APBPERI_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_1_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_1 =
    crate::Reg<dma_apbperi_pms_monitor_1::DMA_APBPERI_PMS_MONITOR_1_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1_REG"]
pub mod dma_apbperi_pms_monitor_1;
#[doc = "DMA_APBPERI_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_2_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_2 =
    crate::Reg<dma_apbperi_pms_monitor_2::DMA_APBPERI_PMS_MONITOR_2_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2_REG"]
pub mod dma_apbperi_pms_monitor_2;
#[doc = "DMA_APBPERI_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_3_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_3 =
    crate::Reg<dma_apbperi_pms_monitor_3::DMA_APBPERI_PMS_MONITOR_3_SPEC>;
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG"]
pub mod dma_apbperi_pms_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_0 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_0;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_1 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_1;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_2 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_2;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_3 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_3;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_4 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_4;
#[doc = "CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 = crate :: Reg < core_x_iram0_dram0_dma_split_line_constrain_5 :: CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC > ;
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_REG"]
pub mod core_x_iram0_dram0_dma_split_line_constrain_5;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_iram0_pms_constrain_0::CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0_REG"]
pub mod core_x_iram0_pms_constrain_0;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_iram0_pms_constrain_1::CORE_X_IRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1_REG"]
pub mod core_x_iram0_pms_constrain_1;
#[doc = "CORE_X_IRAM0_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_X_IRAM0_PMS_CONSTRAIN_2 =
    crate::Reg<core_x_iram0_pms_constrain_2::CORE_X_IRAM0_PMS_CONSTRAIN_2_SPEC>;
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2_REG"]
pub mod core_x_iram0_pms_constrain_2;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_iram0_pms_monitor_0::CORE_0_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0_REG"]
pub mod core_0_iram0_pms_monitor_0;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_iram0_pms_monitor_1::CORE_0_IRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1_REG"]
pub mod core_0_iram0_pms_monitor_1;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_iram0_pms_monitor_2::CORE_0_IRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2_REG"]
pub mod core_0_iram0_pms_monitor_2;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_0 =
    crate::Reg<core_x_dram0_pms_constrain_0::CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0_REG"]
pub mod core_x_dram0_pms_constrain_0;
#[doc = "CORE_X_DRAM0_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_1 =
    crate::Reg<core_x_dram0_pms_constrain_1::CORE_X_DRAM0_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1_REG"]
pub mod core_x_dram0_pms_constrain_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_0 =
    crate::Reg<core_0_dram0_pms_monitor_0::CORE_0_DRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0_REG"]
pub mod core_0_dram0_pms_monitor_0;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_1 =
    crate::Reg<core_0_dram0_pms_monitor_1::CORE_0_DRAM0_PMS_MONITOR_1_SPEC>;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1_REG"]
pub mod core_0_dram0_pms_monitor_1;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_2 =
    crate::Reg<core_0_dram0_pms_monitor_2::CORE_0_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2_REG"]
pub mod core_0_dram0_pms_monitor_2;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_3 =
    crate::Reg<core_0_dram0_pms_monitor_3::CORE_0_DRAM0_PMS_MONITOR_3_SPEC>;
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3_REG"]
pub mod core_0_dram0_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_0_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_0 =
    crate::Reg<core_0_pif_pms_constrain_0::CORE_0_PIF_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0_REG"]
pub mod core_0_pif_pms_constrain_0;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_1 =
    crate::Reg<core_0_pif_pms_constrain_1::CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG"]
pub mod core_0_pif_pms_constrain_1;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_2 =
    crate::Reg<core_0_pif_pms_constrain_2::CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2_REG"]
pub mod core_0_pif_pms_constrain_2;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_3 =
    crate::Reg<core_0_pif_pms_constrain_3::CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3_REG"]
pub mod core_0_pif_pms_constrain_3;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_4 =
    crate::Reg<core_0_pif_pms_constrain_4::CORE_0_PIF_PMS_CONSTRAIN_4_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4_REG"]
pub mod core_0_pif_pms_constrain_4;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_5 =
    crate::Reg<core_0_pif_pms_constrain_5::CORE_0_PIF_PMS_CONSTRAIN_5_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5_REG"]
pub mod core_0_pif_pms_constrain_5;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_6 =
    crate::Reg<core_0_pif_pms_constrain_6::CORE_0_PIF_PMS_CONSTRAIN_6_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6_REG"]
pub mod core_0_pif_pms_constrain_6;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_7 =
    crate::Reg<core_0_pif_pms_constrain_7::CORE_0_PIF_PMS_CONSTRAIN_7_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7_REG"]
pub mod core_0_pif_pms_constrain_7;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_8 =
    crate::Reg<core_0_pif_pms_constrain_8::CORE_0_PIF_PMS_CONSTRAIN_8_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG"]
pub mod core_0_pif_pms_constrain_8;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_9 =
    crate::Reg<core_0_pif_pms_constrain_9::CORE_0_PIF_PMS_CONSTRAIN_9_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG"]
pub mod core_0_pif_pms_constrain_9;
#[doc = "CORE_0_PIF_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>`"]
pub type CORE_0_PIF_PMS_CONSTRAIN_10 =
    crate::Reg<core_0_pif_pms_constrain_10::CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10_REG"]
pub mod core_0_pif_pms_constrain_10;
#[doc = "REGION_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_0_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_0 = crate::Reg<region_pms_constrain_0::REGION_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_0_REG"]
pub mod region_pms_constrain_0;
#[doc = "REGION_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_1_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_1 = crate::Reg<region_pms_constrain_1::REGION_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_1_REG"]
pub mod region_pms_constrain_1;
#[doc = "REGION_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_2_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_2 = crate::Reg<region_pms_constrain_2::REGION_PMS_CONSTRAIN_2_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2_REG"]
pub mod region_pms_constrain_2;
#[doc = "REGION_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_3_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_3 = crate::Reg<region_pms_constrain_3::REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3_REG"]
pub mod region_pms_constrain_3;
#[doc = "REGION_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_4_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_4 = crate::Reg<region_pms_constrain_4::REGION_PMS_CONSTRAIN_4_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_4_REG"]
pub mod region_pms_constrain_4;
#[doc = "REGION_PMS_CONSTRAIN_5 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_5_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_5 = crate::Reg<region_pms_constrain_5::REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_5_REG"]
pub mod region_pms_constrain_5;
#[doc = "REGION_PMS_CONSTRAIN_6 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_6_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_6 = crate::Reg<region_pms_constrain_6::REGION_PMS_CONSTRAIN_6_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6_REG"]
pub mod region_pms_constrain_6;
#[doc = "REGION_PMS_CONSTRAIN_7 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_7_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_7 = crate::Reg<region_pms_constrain_7::REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_7_REG"]
pub mod region_pms_constrain_7;
#[doc = "REGION_PMS_CONSTRAIN_8 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_8_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_8 = crate::Reg<region_pms_constrain_8::REGION_PMS_CONSTRAIN_8_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_8_REG"]
pub mod region_pms_constrain_8;
#[doc = "REGION_PMS_CONSTRAIN_9 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_9_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_9 = crate::Reg<region_pms_constrain_9::REGION_PMS_CONSTRAIN_9_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_9_REG"]
pub mod region_pms_constrain_9;
#[doc = "REGION_PMS_CONSTRAIN_10 (rw) register accessor: an alias for `Reg<REGION_PMS_CONSTRAIN_10_SPEC>`"]
pub type REGION_PMS_CONSTRAIN_10 =
    crate::Reg<region_pms_constrain_10::REGION_PMS_CONSTRAIN_10_SPEC>;
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_10_REG"]
pub mod region_pms_constrain_10;
#[doc = "CORE_0_PIF_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_0_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_0 =
    crate::Reg<core_0_pif_pms_monitor_0::CORE_0_PIF_PMS_MONITOR_0_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_0_REG"]
pub mod core_0_pif_pms_monitor_0;
#[doc = "CORE_0_PIF_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_1_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_1 =
    crate::Reg<core_0_pif_pms_monitor_1::CORE_0_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_1_REG"]
pub mod core_0_pif_pms_monitor_1;
#[doc = "CORE_0_PIF_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_2_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_2 =
    crate::Reg<core_0_pif_pms_monitor_2::CORE_0_PIF_PMS_MONITOR_2_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2_REG"]
pub mod core_0_pif_pms_monitor_2;
#[doc = "CORE_0_PIF_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_3_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_3 =
    crate::Reg<core_0_pif_pms_monitor_3::CORE_0_PIF_PMS_MONITOR_3_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_3_REG"]
pub mod core_0_pif_pms_monitor_3;
#[doc = "CORE_0_PIF_PMS_MONITOR_4 (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_4_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_4 =
    crate::Reg<core_0_pif_pms_monitor_4::CORE_0_PIF_PMS_MONITOR_4_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_4_REG"]
pub mod core_0_pif_pms_monitor_4;
#[doc = "CORE_0_PIF_PMS_MONITOR_5 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_5_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_5 =
    crate::Reg<core_0_pif_pms_monitor_5::CORE_0_PIF_PMS_MONITOR_5_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5_REG"]
pub mod core_0_pif_pms_monitor_5;
#[doc = "CORE_0_PIF_PMS_MONITOR_6 (r) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_6_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_6 =
    crate::Reg<core_0_pif_pms_monitor_6::CORE_0_PIF_PMS_MONITOR_6_SPEC>;
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6_REG"]
pub mod core_0_pif_pms_monitor_6;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_0 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_0 =
    crate::Reg<backup_bus_pms_constrain_0::BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0_REG"]
pub mod backup_bus_pms_constrain_0;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_1 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_1 =
    crate::Reg<backup_bus_pms_constrain_1::BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG"]
pub mod backup_bus_pms_constrain_1;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_2 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_2 =
    crate::Reg<backup_bus_pms_constrain_2::BACKUP_BUS_PMS_CONSTRAIN_2_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2_REG"]
pub mod backup_bus_pms_constrain_2;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_3 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_3 =
    crate::Reg<backup_bus_pms_constrain_3::BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG"]
pub mod backup_bus_pms_constrain_3;
#[doc = "BACKUP_BUS_PMS_CONSTRAIN_4 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>`"]
pub type BACKUP_BUS_PMS_CONSTRAIN_4 =
    crate::Reg<backup_bus_pms_constrain_4::BACKUP_BUS_PMS_CONSTRAIN_4_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG"]
pub mod backup_bus_pms_constrain_4;
#[doc = "BACKUP_BUS_PMS_MONITOR_0 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_0_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_0 =
    crate::Reg<backup_bus_pms_monitor_0::BACKUP_BUS_PMS_MONITOR_0_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_0_REG"]
pub mod backup_bus_pms_monitor_0;
#[doc = "BACKUP_BUS_PMS_MONITOR_1 (rw) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_1_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_1 =
    crate::Reg<backup_bus_pms_monitor_1::BACKUP_BUS_PMS_MONITOR_1_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1_REG"]
pub mod backup_bus_pms_monitor_1;
#[doc = "BACKUP_BUS_PMS_MONITOR_2 (r) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_2_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_2 =
    crate::Reg<backup_bus_pms_monitor_2::BACKUP_BUS_PMS_MONITOR_2_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2_REG"]
pub mod backup_bus_pms_monitor_2;
#[doc = "BACKUP_BUS_PMS_MONITOR_3 (r) register accessor: an alias for `Reg<BACKUP_BUS_PMS_MONITOR_3_SPEC>`"]
pub type BACKUP_BUS_PMS_MONITOR_3 =
    crate::Reg<backup_bus_pms_monitor_3::BACKUP_BUS_PMS_MONITOR_3_SPEC>;
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3_REG"]
pub mod backup_bus_pms_monitor_3;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SENSITIVE_CLOCK_GATE_REG_REG"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SENSITIVE_DATE_REG"]
pub mod date;
