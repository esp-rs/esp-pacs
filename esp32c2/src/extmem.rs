#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    icache_ctrl: ICACHE_CTRL,
    icache_ctrl1: ICACHE_CTRL1,
    icache_tag_power_ctrl: ICACHE_TAG_POWER_CTRL,
    _reserved3: [u8; 0x1c],
    icache_sync_ctrl: ICACHE_SYNC_CTRL,
    icache_sync_addr: ICACHE_SYNC_ADDR,
    icache_sync_size: ICACHE_SYNC_SIZE,
    _reserved6: [u8; 0x20],
    ibus_to_flash_start_vaddr: IBUS_TO_FLASH_START_VADDR,
    ibus_to_flash_end_vaddr: IBUS_TO_FLASH_END_VADDR,
    dbus_to_flash_start_vaddr: DBUS_TO_FLASH_START_VADDR,
    dbus_to_flash_end_vaddr: DBUS_TO_FLASH_END_VADDR,
    cache_acs_cnt_clr: CACHE_ACS_CNT_CLR,
    _reserved11: [u8; 0x10],
    cache_ilg_int_ena: CACHE_ILG_INT_ENA,
    cache_ilg_int_clr: CACHE_ILG_INT_CLR,
    cache_ilg_int_st: CACHE_ILG_INT_ST,
    core0_acs_cache_int_ena: CORE0_ACS_CACHE_INT_ENA,
    core0_acs_cache_int_clr: CORE0_ACS_CACHE_INT_CLR,
    core0_acs_cache_int_st: CORE0_ACS_CACHE_INT_ST,
    core0_dbus_reject_st: CORE0_DBUS_REJECT_ST,
    core0_dbus_reject_vaddr: CORE0_DBUS_REJECT_VADDR,
    core0_ibus_reject_st: CORE0_IBUS_REJECT_ST,
    core0_ibus_reject_vaddr: CORE0_IBUS_REJECT_VADDR,
    cache_mmu_fault_content: CACHE_MMU_FAULT_CONTENT,
    cache_mmu_fault_vaddr: CACHE_MMU_FAULT_VADDR,
    cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    cache_mmu_power_ctrl: CACHE_MMU_POWER_CTRL,
    cache_state: CACHE_STATE,
    cache_encrypt_decrypt_record_disable: CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    cache_encrypt_decrypt_clk_force_on: CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    cache_preload_int_ctrl: CACHE_PRELOAD_INT_CTRL,
    cache_sync_int_ctrl: CACHE_SYNC_INT_CTRL,
    cache_mmu_owner: CACHE_MMU_OWNER,
    cache_conf_misc: CACHE_CONF_MISC,
    icache_freeze: ICACHE_FREEZE,
    icache_atomic_operate_ena: ICACHE_ATOMIC_OPERATE_ENA,
    cache_request: CACHE_REQUEST,
    _reserved35: [u8; 0x28],
    clock_gate: CLOCK_GATE,
    _reserved36: [u8; 0x02f8],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_ctrl(&self) -> &ICACHE_CTRL {
        &self.icache_ctrl
    }
    #[doc = "0x04 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_ctrl1(&self) -> &ICACHE_CTRL1 {
        &self.icache_ctrl1
    }
    #[doc = "0x08 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_tag_power_ctrl(&self) -> &ICACHE_TAG_POWER_CTRL {
        &self.icache_tag_power_ctrl
    }
    #[doc = "0x28 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_sync_ctrl(&self) -> &ICACHE_SYNC_CTRL {
        &self.icache_sync_ctrl
    }
    #[doc = "0x2c - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_sync_addr(&self) -> &ICACHE_SYNC_ADDR {
        &self.icache_sync_addr
    }
    #[doc = "0x30 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_sync_size(&self) -> &ICACHE_SYNC_SIZE {
        &self.icache_sync_size
    }
    #[doc = "0x54 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn ibus_to_flash_start_vaddr(&self) -> &IBUS_TO_FLASH_START_VADDR {
        &self.ibus_to_flash_start_vaddr
    }
    #[doc = "0x58 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn ibus_to_flash_end_vaddr(&self) -> &IBUS_TO_FLASH_END_VADDR {
        &self.ibus_to_flash_end_vaddr
    }
    #[doc = "0x5c - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn dbus_to_flash_start_vaddr(&self) -> &DBUS_TO_FLASH_START_VADDR {
        &self.dbus_to_flash_start_vaddr
    }
    #[doc = "0x60 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn dbus_to_flash_end_vaddr(&self) -> &DBUS_TO_FLASH_END_VADDR {
        &self.dbus_to_flash_end_vaddr
    }
    #[doc = "0x64 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_acs_cnt_clr(&self) -> &CACHE_ACS_CNT_CLR {
        &self.cache_acs_cnt_clr
    }
    #[doc = "0x78 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_ilg_int_ena(&self) -> &CACHE_ILG_INT_ENA {
        &self.cache_ilg_int_ena
    }
    #[doc = "0x7c - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_ilg_int_clr(&self) -> &CACHE_ILG_INT_CLR {
        &self.cache_ilg_int_clr
    }
    #[doc = "0x80 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_ilg_int_st(&self) -> &CACHE_ILG_INT_ST {
        &self.cache_ilg_int_st
    }
    #[doc = "0x84 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_acs_cache_int_ena(&self) -> &CORE0_ACS_CACHE_INT_ENA {
        &self.core0_acs_cache_int_ena
    }
    #[doc = "0x88 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_acs_cache_int_clr(&self) -> &CORE0_ACS_CACHE_INT_CLR {
        &self.core0_acs_cache_int_clr
    }
    #[doc = "0x8c - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_acs_cache_int_st(&self) -> &CORE0_ACS_CACHE_INT_ST {
        &self.core0_acs_cache_int_st
    }
    #[doc = "0x90 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_dbus_reject_st(&self) -> &CORE0_DBUS_REJECT_ST {
        &self.core0_dbus_reject_st
    }
    #[doc = "0x94 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_dbus_reject_vaddr(&self) -> &CORE0_DBUS_REJECT_VADDR {
        &self.core0_dbus_reject_vaddr
    }
    #[doc = "0x98 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_ibus_reject_st(&self) -> &CORE0_IBUS_REJECT_ST {
        &self.core0_ibus_reject_st
    }
    #[doc = "0x9c - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn core0_ibus_reject_vaddr(&self) -> &CORE0_IBUS_REJECT_VADDR {
        &self.core0_ibus_reject_vaddr
    }
    #[doc = "0xa0 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_mmu_fault_content(&self) -> &CACHE_MMU_FAULT_CONTENT {
        &self.cache_mmu_fault_content
    }
    #[doc = "0xa4 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_mmu_fault_vaddr(&self) -> &CACHE_MMU_FAULT_VADDR {
        &self.cache_mmu_fault_vaddr
    }
    #[doc = "0xa8 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_wrap_around_ctrl(&self) -> &CACHE_WRAP_AROUND_CTRL {
        &self.cache_wrap_around_ctrl
    }
    #[doc = "0xac - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_mmu_power_ctrl(&self) -> &CACHE_MMU_POWER_CTRL {
        &self.cache_mmu_power_ctrl
    }
    #[doc = "0xb0 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_state(&self) -> &CACHE_STATE {
        &self.cache_state
    }
    #[doc = "0xb4 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_record_disable(
        &self,
    ) -> &CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {
        &self.cache_encrypt_decrypt_record_disable
    }
    #[doc = "0xb8 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_clk_force_on(&self) -> &CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {
        &self.cache_encrypt_decrypt_clk_force_on
    }
    #[doc = "0xbc - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_preload_int_ctrl(&self) -> &CACHE_PRELOAD_INT_CTRL {
        &self.cache_preload_int_ctrl
    }
    #[doc = "0xc0 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_sync_int_ctrl(&self) -> &CACHE_SYNC_INT_CTRL {
        &self.cache_sync_int_ctrl
    }
    #[doc = "0xc4 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_mmu_owner(&self) -> &CACHE_MMU_OWNER {
        &self.cache_mmu_owner
    }
    #[doc = "0xc8 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_conf_misc(&self) -> &CACHE_CONF_MISC {
        &self.cache_conf_misc
    }
    #[doc = "0xcc - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_freeze(&self) -> &ICACHE_FREEZE {
        &self.icache_freeze
    }
    #[doc = "0xd0 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn icache_atomic_operate_ena(&self) -> &ICACHE_ATOMIC_OPERATE_ENA {
        &self.icache_atomic_operate_ena
    }
    #[doc = "0xd4 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn cache_request(&self) -> &CACHE_REQUEST {
        &self.cache_request
    }
    #[doc = "0x100 - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - This description will be updated in the near future."]
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
#[doc = "ICACHE_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl`] module"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_ctrl;
#[doc = "ICACHE_CTRL1 (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl1`] module"]
pub type ICACHE_CTRL1 = crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_ctrl1;
#[doc = "ICACHE_TAG_POWER_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_tag_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_tag_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_tag_power_ctrl`] module"]
pub type ICACHE_TAG_POWER_CTRL = crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_tag_power_ctrl;
#[doc = "ICACHE_SYNC_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_ctrl`] module"]
pub type ICACHE_SYNC_CTRL = crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_ctrl;
#[doc = "ICACHE_SYNC_ADDR (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_addr`] module"]
pub type ICACHE_SYNC_ADDR = crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_addr;
#[doc = "ICACHE_SYNC_SIZE (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_size`] module"]
pub type ICACHE_SYNC_SIZE = crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_size;
#[doc = "IBUS_TO_FLASH_START_VADDR (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_to_flash_start_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_to_flash_start_vaddr`] module"]
pub type IBUS_TO_FLASH_START_VADDR =
    crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod ibus_to_flash_start_vaddr;
#[doc = "IBUS_TO_FLASH_END_VADDR (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_to_flash_end_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_to_flash_end_vaddr`] module"]
pub type IBUS_TO_FLASH_END_VADDR =
    crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod ibus_to_flash_end_vaddr;
#[doc = "DBUS_TO_FLASH_START_VADDR (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_to_flash_start_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_to_flash_start_vaddr`] module"]
pub type DBUS_TO_FLASH_START_VADDR =
    crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod dbus_to_flash_start_vaddr;
#[doc = "DBUS_TO_FLASH_END_VADDR (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_to_flash_end_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_to_flash_end_vaddr`] module"]
pub type DBUS_TO_FLASH_END_VADDR =
    crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod dbus_to_flash_end_vaddr;
#[doc = "CACHE_ACS_CNT_CLR (w) register accessor: This description will be updated in the near future.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_clr`] module"]
pub type CACHE_ACS_CNT_CLR = crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_acs_cnt_clr;
#[doc = "CACHE_ILG_INT_ENA (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_ena`] module"]
pub type CACHE_ILG_INT_ENA = crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_ena;
#[doc = "CACHE_ILG_INT_CLR (w) register accessor: This description will be updated in the near future.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_clr`] module"]
pub type CACHE_ILG_INT_CLR = crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_clr;
#[doc = "CACHE_ILG_INT_ST (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_st`] module"]
pub type CACHE_ILG_INT_ST = crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_st;
#[doc = "CORE0_ACS_CACHE_INT_ENA (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_acs_cache_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_ena`] module"]
pub type CORE0_ACS_CACHE_INT_ENA =
    crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_ena;
#[doc = "CORE0_ACS_CACHE_INT_CLR (w) register accessor: This description will be updated in the near future.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_clr`] module"]
pub type CORE0_ACS_CACHE_INT_CLR =
    crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_clr;
#[doc = "CORE0_ACS_CACHE_INT_ST (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_acs_cache_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_st`] module"]
pub type CORE0_ACS_CACHE_INT_ST = crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_st;
#[doc = "CORE0_DBUS_REJECT_ST (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_dbus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_dbus_reject_st`] module"]
pub type CORE0_DBUS_REJECT_ST = crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_dbus_reject_st;
#[doc = "CORE0_DBUS_REJECT_VADDR (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_dbus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_dbus_reject_vaddr`] module"]
pub type CORE0_DBUS_REJECT_VADDR =
    crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_dbus_reject_vaddr;
#[doc = "CORE0_IBUS_REJECT_ST (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_ibus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_ibus_reject_st`] module"]
pub type CORE0_IBUS_REJECT_ST = crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_ibus_reject_st;
#[doc = "CORE0_IBUS_REJECT_VADDR (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_ibus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_ibus_reject_vaddr`] module"]
pub type CORE0_IBUS_REJECT_VADDR =
    crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_ibus_reject_vaddr;
#[doc = "CACHE_MMU_FAULT_CONTENT (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_fault_content::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_fault_content`] module"]
pub type CACHE_MMU_FAULT_CONTENT =
    crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_fault_content;
#[doc = "CACHE_MMU_FAULT_VADDR (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_fault_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_fault_vaddr`] module"]
pub type CACHE_MMU_FAULT_VADDR = crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_fault_vaddr;
#[doc = "CACHE_WRAP_AROUND_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_wrap_around_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_wrap_around_ctrl`] module"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MMU_POWER_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_power_ctrl`] module"]
pub type CACHE_MMU_POWER_CTRL = crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_power_ctrl;
#[doc = "CACHE_STATE (r) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_state`] module"]
pub type CACHE_STATE = crate::Reg<cache_state::CACHE_STATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_encrypt_decrypt_record_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_record_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_record_disable`] module"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_encrypt_decrypt_clk_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_clk_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_clk_force_on`] module"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_PRELOAD_INT_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_preload_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_preload_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_int_ctrl`] module"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_sync_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sync_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_int_ctrl`] module"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_MMU_OWNER (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_owner::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_owner::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_owner`] module"]
pub type CACHE_MMU_OWNER = crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_owner;
#[doc = "CACHE_CONF_MISC (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_conf_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_conf_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_conf_misc`] module"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_conf_misc;
#[doc = "ICACHE_FREEZE (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_freeze`] module"]
pub type ICACHE_FREEZE = crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_freeze;
#[doc = "ICACHE_ATOMIC_OPERATE_ENA (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_atomic_operate_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_atomic_operate_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_atomic_operate_ena`] module"]
pub type ICACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_atomic_operate_ena;
#[doc = "CACHE_REQUEST (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_request`] module"]
pub type CACHE_REQUEST = crate::Reg<cache_request::CACHE_REQUEST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_request;
#[doc = "CLOCK_GATE (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod clock_gate;
#[doc = "REG_DATE (rw) register accessor: This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_date`] module"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod reg_date;
