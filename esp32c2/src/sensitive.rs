#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    rom_table_lock: ROM_TABLE_LOCK,
    rom_table: ROM_TABLE,
    apb_peripheral_access_0: APB_PERIPHERAL_ACCESS_0,
    apb_peripheral_access_1: APB_PERIPHERAL_ACCESS_1,
    internal_sram_usage_0: INTERNAL_SRAM_USAGE_0,
    internal_sram_usage_1: INTERNAL_SRAM_USAGE_1,
    internal_sram_usage_3: INTERNAL_SRAM_USAGE_3,
    cache_tag_access_0: CACHE_TAG_ACCESS_0,
    cache_tag_access_1: CACHE_TAG_ACCESS_1,
    cache_mmu_access_0: CACHE_MMU_ACCESS_0,
    cache_mmu_access_1: CACHE_MMU_ACCESS_1,
    pif_access_monitor_0: PIF_ACCESS_MONITOR_0,
    pif_access_monitor_1: PIF_ACCESS_MONITOR_1,
    pif_access_monitor_2: PIF_ACCESS_MONITOR_2,
    pif_access_monitor_3: PIF_ACCESS_MONITOR_3,
    xts_aes_key_update: XTS_AES_KEY_UPDATE,
    clock_gate: CLOCK_GATE,
    _reserved17: [u8; 0x0fb8],
    sensitive_reg_date: SENSITIVE_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - register description"]
    #[inline(always)]
    pub const fn rom_table_lock(&self) -> &ROM_TABLE_LOCK {
        &self.rom_table_lock
    }
    #[doc = "0x04 - register description"]
    #[inline(always)]
    pub const fn rom_table(&self) -> &ROM_TABLE {
        &self.rom_table
    }
    #[doc = "0x08 - register description"]
    #[inline(always)]
    pub const fn apb_peripheral_access_0(&self) -> &APB_PERIPHERAL_ACCESS_0 {
        &self.apb_peripheral_access_0
    }
    #[doc = "0x0c - register description"]
    #[inline(always)]
    pub const fn apb_peripheral_access_1(&self) -> &APB_PERIPHERAL_ACCESS_1 {
        &self.apb_peripheral_access_1
    }
    #[doc = "0x10 - register description"]
    #[inline(always)]
    pub const fn internal_sram_usage_0(&self) -> &INTERNAL_SRAM_USAGE_0 {
        &self.internal_sram_usage_0
    }
    #[doc = "0x14 - register description"]
    #[inline(always)]
    pub const fn internal_sram_usage_1(&self) -> &INTERNAL_SRAM_USAGE_1 {
        &self.internal_sram_usage_1
    }
    #[doc = "0x18 - register description"]
    #[inline(always)]
    pub const fn internal_sram_usage_3(&self) -> &INTERNAL_SRAM_USAGE_3 {
        &self.internal_sram_usage_3
    }
    #[doc = "0x1c - register description"]
    #[inline(always)]
    pub const fn cache_tag_access_0(&self) -> &CACHE_TAG_ACCESS_0 {
        &self.cache_tag_access_0
    }
    #[doc = "0x20 - register description"]
    #[inline(always)]
    pub const fn cache_tag_access_1(&self) -> &CACHE_TAG_ACCESS_1 {
        &self.cache_tag_access_1
    }
    #[doc = "0x24 - register description"]
    #[inline(always)]
    pub const fn cache_mmu_access_0(&self) -> &CACHE_MMU_ACCESS_0 {
        &self.cache_mmu_access_0
    }
    #[doc = "0x28 - register description"]
    #[inline(always)]
    pub const fn cache_mmu_access_1(&self) -> &CACHE_MMU_ACCESS_1 {
        &self.cache_mmu_access_1
    }
    #[doc = "0x2c - register description"]
    #[inline(always)]
    pub const fn pif_access_monitor_0(&self) -> &PIF_ACCESS_MONITOR_0 {
        &self.pif_access_monitor_0
    }
    #[doc = "0x30 - register description"]
    #[inline(always)]
    pub const fn pif_access_monitor_1(&self) -> &PIF_ACCESS_MONITOR_1 {
        &self.pif_access_monitor_1
    }
    #[doc = "0x34 - register description"]
    #[inline(always)]
    pub const fn pif_access_monitor_2(&self) -> &PIF_ACCESS_MONITOR_2 {
        &self.pif_access_monitor_2
    }
    #[doc = "0x38 - register description"]
    #[inline(always)]
    pub const fn pif_access_monitor_3(&self) -> &PIF_ACCESS_MONITOR_3 {
        &self.pif_access_monitor_3
    }
    #[doc = "0x3c - register description"]
    #[inline(always)]
    pub const fn xts_aes_key_update(&self) -> &XTS_AES_KEY_UPDATE {
        &self.xts_aes_key_update
    }
    #[doc = "0x40 - register description"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - register description"]
    #[inline(always)]
    pub const fn sensitive_reg_date(&self) -> &SENSITIVE_REG_DATE {
        &self.sensitive_reg_date
    }
}
#[doc = "ROM_TABLE_LOCK (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_table_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_table_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_table_lock`] module"]
pub type ROM_TABLE_LOCK = crate::Reg<rom_table_lock::ROM_TABLE_LOCK_SPEC>;
#[doc = "register description"]
pub mod rom_table_lock;
#[doc = "ROM_TABLE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_table::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_table::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_table`] module"]
pub type ROM_TABLE = crate::Reg<rom_table::ROM_TABLE_SPEC>;
#[doc = "register description"]
pub mod rom_table;
#[doc = "APB_PERIPHERAL_ACCESS_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_peripheral_access_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_peripheral_access_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_access_0`] module"]
pub type APB_PERIPHERAL_ACCESS_0 =
    crate::Reg<apb_peripheral_access_0::APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod apb_peripheral_access_0;
#[doc = "APB_PERIPHERAL_ACCESS_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_peripheral_access_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_peripheral_access_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_peripheral_access_1`] module"]
pub type APB_PERIPHERAL_ACCESS_1 =
    crate::Reg<apb_peripheral_access_1::APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod apb_peripheral_access_1;
#[doc = "INTERNAL_SRAM_USAGE_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`internal_sram_usage_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internal_sram_usage_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_0`] module"]
pub type INTERNAL_SRAM_USAGE_0 = crate::Reg<internal_sram_usage_0::INTERNAL_SRAM_USAGE_0_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_0;
#[doc = "INTERNAL_SRAM_USAGE_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`internal_sram_usage_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internal_sram_usage_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_1`] module"]
pub type INTERNAL_SRAM_USAGE_1 = crate::Reg<internal_sram_usage_1::INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_1;
#[doc = "INTERNAL_SRAM_USAGE_3 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`internal_sram_usage_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internal_sram_usage_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@internal_sram_usage_3`] module"]
pub type INTERNAL_SRAM_USAGE_3 = crate::Reg<internal_sram_usage_3::INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "register description"]
pub mod internal_sram_usage_3;
#[doc = "CACHE_TAG_ACCESS_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_access_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_access_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_0`] module"]
pub type CACHE_TAG_ACCESS_0 = crate::Reg<cache_tag_access_0::CACHE_TAG_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod cache_tag_access_0;
#[doc = "CACHE_TAG_ACCESS_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_access_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_access_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_access_1`] module"]
pub type CACHE_TAG_ACCESS_1 = crate::Reg<cache_tag_access_1::CACHE_TAG_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod cache_tag_access_1;
#[doc = "CACHE_MMU_ACCESS_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_access_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mmu_access_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_0`] module"]
pub type CACHE_MMU_ACCESS_0 = crate::Reg<cache_mmu_access_0::CACHE_MMU_ACCESS_0_SPEC>;
#[doc = "register description"]
pub mod cache_mmu_access_0;
#[doc = "CACHE_MMU_ACCESS_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_access_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mmu_access_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_access_1`] module"]
pub type CACHE_MMU_ACCESS_1 = crate::Reg<cache_mmu_access_1::CACHE_MMU_ACCESS_1_SPEC>;
#[doc = "register description"]
pub mod cache_mmu_access_1;
#[doc = "PIF_ACCESS_MONITOR_0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_access_monitor_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pif_access_monitor_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pif_access_monitor_0`] module"]
pub type PIF_ACCESS_MONITOR_0 = crate::Reg<pif_access_monitor_0::PIF_ACCESS_MONITOR_0_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_0;
#[doc = "PIF_ACCESS_MONITOR_1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_access_monitor_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pif_access_monitor_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pif_access_monitor_1`] module"]
pub type PIF_ACCESS_MONITOR_1 = crate::Reg<pif_access_monitor_1::PIF_ACCESS_MONITOR_1_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_1;
#[doc = "PIF_ACCESS_MONITOR_2 (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_access_monitor_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pif_access_monitor_2`] module"]
pub type PIF_ACCESS_MONITOR_2 = crate::Reg<pif_access_monitor_2::PIF_ACCESS_MONITOR_2_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_2;
#[doc = "PIF_ACCESS_MONITOR_3 (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_access_monitor_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pif_access_monitor_3`] module"]
pub type PIF_ACCESS_MONITOR_3 = crate::Reg<pif_access_monitor_3::PIF_ACCESS_MONITOR_3_SPEC>;
#[doc = "register description"]
pub mod pif_access_monitor_3;
#[doc = "XTS_AES_KEY_UPDATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_aes_key_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_aes_key_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_aes_key_update`] module"]
pub type XTS_AES_KEY_UPDATE = crate::Reg<xts_aes_key_update::XTS_AES_KEY_UPDATE_SPEC>;
#[doc = "register description"]
pub mod xts_aes_key_update;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
pub use crate::apb_ctrl::date as sensitive_reg_date;
pub use crate::apb_ctrl::DATE as SENSITIVE_REG_DATE;
