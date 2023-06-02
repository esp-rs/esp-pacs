#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - register description"]
    pub rom_table_lock: ROM_TABLE_LOCK,
    #[doc = "0x04 - register description"]
    pub rom_table: ROM_TABLE,
    #[doc = "0x08 - register description"]
    pub apb_peripheral_access_0: APB_PERIPHERAL_ACCESS_0,
    #[doc = "0x0c - register description"]
    pub apb_peripheral_access_1: APB_PERIPHERAL_ACCESS_1,
    #[doc = "0x10 - register description"]
    pub internal_sram_usage_0: INTERNAL_SRAM_USAGE_0,
    #[doc = "0x14 - register description"]
    pub internal_sram_usage_1: INTERNAL_SRAM_USAGE_1,
    #[doc = "0x18 - register description"]
    pub internal_sram_usage_3: INTERNAL_SRAM_USAGE_3,
    #[doc = "0x1c - register description"]
    pub cache_tag_access_0: CACHE_TAG_ACCESS_0,
    #[doc = "0x20 - register description"]
    pub cache_tag_access_1: CACHE_TAG_ACCESS_1,
    #[doc = "0x24 - register description"]
    pub cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    #[doc = "0x28 - register description"]
    pub cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    #[doc = "0x2c - register description"]
    pub pif_access_monitor_0: PIF_ACCESS_MONITOR_0,
    #[doc = "0x30 - register description"]
    pub pif_access_monitor_1: PIF_ACCESS_MONITOR_1,
    #[doc = "0x34 - register description"]
    pub pif_access_monitor_2: PIF_ACCESS_MONITOR_2,
    #[doc = "0x38 - register description"]
    pub pif_access_monitor_3: PIF_ACCESS_MONITOR_3,
    #[doc = "0x3c - register description"]
    pub xts_aes_key_update: XTS_AES_KEY_UPDATE,
    #[doc = "0x40 - register description"]
    pub clock_gate: CLOCK_GATE,
    _reserved17: [u8; 0x0fb8],
    #[doc = "0xffc - register description"]
    pub sensitive_reg_date: SENSITIVE_REG_DATE,
}
#[doc = "ROM_TABLE_LOCK (rw) register accessor: an alias for `Reg<ROM_TABLE_LOCK_SPEC>`"]
pub type ROM_TABLE_LOCK = crate::Reg<rom_table_lock::ROM_TABLE_LOCK_SPEC>;
#[doc = "register description"]
pub mod rom_table_lock;
#[doc = "ROM_TABLE (rw) register accessor: an alias for `Reg<ROM_TABLE_SPEC>`"]
pub type ROM_TABLE = crate::Reg<rom_table::ROM_TABLE_SPEC>;
#[doc = "register description"]
pub mod rom_table;
#[doc = "APB_PERIPHERAL_ACCESS_0 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_0_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_0 =
    crate::Reg<apb_peripheral_access_0::APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod apb_peripheral_access_0;
#[doc = "APB_PERIPHERAL_ACCESS_1 (rw) register accessor: an alias for `Reg<APB_PERIPHERAL_ACCESS_1_SPEC>`"]
pub type APB_PERIPHERAL_ACCESS_1 =
    crate::Reg<apb_peripheral_access_1::APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod apb_peripheral_access_1;
#[doc = "INTERNAL_SRAM_USAGE_0 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_0_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_0 = crate::Reg<internal_sram_usage_0::INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_0;
#[doc = "INTERNAL_SRAM_USAGE_1 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_1_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_1 = crate::Reg<internal_sram_usage_1::INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_1;
#[doc = "INTERNAL_SRAM_USAGE_3 (rw) register accessor: an alias for `Reg<INTERNAL_SRAM_USAGE_3_SPEC>`"]
pub type INTERNAL_SRAM_USAGE_3 = crate::Reg<internal_sram_usage_3::INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_3;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_0_SPEC>`"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_TAG_ACCESS_1_SPEC>`"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_0_SPEC>`"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: an alias for `Reg<CACHE_MMU_ACCESS_1_SPEC>`"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod cache_mmu_access_1;
#[doc = "PIF_ACCESS_MONITOR_0 (rw) register accessor: an alias for `Reg<PIF_ACCESS_MONITOR_0_SPEC>`"]
pub type PIF_ACCESS_MONITOR_0 = crate::Reg<pif_access_monitor_0::PIF_ACCESS_MONITOR_0_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_0;
#[doc = "PIF_ACCESS_MONITOR_1 (rw) register accessor: an alias for `Reg<PIF_ACCESS_MONITOR_1_SPEC>`"]
pub type PIF_ACCESS_MONITOR_1 = crate::Reg<pif_access_monitor_1::PIF_ACCESS_MONITOR_1_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_1;
#[doc = "PIF_ACCESS_MONITOR_2 (r) register accessor: an alias for `Reg<PIF_ACCESS_MONITOR_2_SPEC>`"]
pub type PIF_ACCESS_MONITOR_2 = crate::Reg<pif_access_monitor_2::PIF_ACCESS_MONITOR_2_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_2;
#[doc = "PIF_ACCESS_MONITOR_3 (r) register accessor: an alias for `Reg<PIF_ACCESS_MONITOR_3_SPEC>`"]
pub type PIF_ACCESS_MONITOR_3 = crate::Reg<pif_access_monitor_3::PIF_ACCESS_MONITOR_3_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_3;
#[doc = "XTS_AES_KEY_UPDATE (rw) register accessor: an alias for `Reg<XTS_AES_KEY_UPDATE_SPEC>`"]
pub type XTS_AES_KEY_UPDATE = crate::Reg<xts_aes_key_update::XTS_AES_KEY_UPDATE_SPEC>;
#[doc = "register description"]
pub mod xts_aes_key_update;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "SENSITIVE_REG_DATE (rw) register accessor: an alias for `Reg<SENSITIVE_REG_DATE_SPEC>`"]
pub type SENSITIVE_REG_DATE = crate::Reg<sensitive_reg_date::SENSITIVE_REG_DATE_SPEC>;
#[doc = "register description"]
pub mod sensitive_reg_date;
