#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - ******* Description ***********"]
    pub dcache_ctrl: DCACHE_CTRL,
    #[doc = "0x04 - ******* Description ***********"]
    pub dcache_ctrl1: DCACHE_CTRL1,
    #[doc = "0x08 - ******* Description ***********"]
    pub dcache_tag_power_ctrl: DCACHE_TAG_POWER_CTRL,
    #[doc = "0x0c - ******* Description ***********"]
    pub dcache_prelock_ctrl: DCACHE_PRELOCK_CTRL,
    #[doc = "0x10 - ******* Description ***********"]
    pub dcache_prelock_sct0_addr: DCACHE_PRELOCK_SCT0_ADDR,
    #[doc = "0x14 - ******* Description ***********"]
    pub dcache_prelock_sct1_addr: DCACHE_PRELOCK_SCT1_ADDR,
    #[doc = "0x18 - ******* Description ***********"]
    pub dcache_prelock_sct_size: DCACHE_PRELOCK_SCT_SIZE,
    #[doc = "0x1c - ******* Description ***********"]
    pub dcache_lock_ctrl: DCACHE_LOCK_CTRL,
    #[doc = "0x20 - ******* Description ***********"]
    pub dcache_lock_addr: DCACHE_LOCK_ADDR,
    #[doc = "0x24 - ******* Description ***********"]
    pub dcache_lock_size: DCACHE_LOCK_SIZE,
    #[doc = "0x28 - ******* Description ***********"]
    pub dcache_sync_ctrl: DCACHE_SYNC_CTRL,
    #[doc = "0x2c - ******* Description ***********"]
    pub dcache_sync_addr: DCACHE_SYNC_ADDR,
    #[doc = "0x30 - ******* Description ***********"]
    pub dcache_sync_size: DCACHE_SYNC_SIZE,
    #[doc = "0x34 - ******* Description ***********"]
    pub dcache_occupy_ctrl: DCACHE_OCCUPY_CTRL,
    #[doc = "0x38 - ******* Description ***********"]
    pub dcache_occupy_addr: DCACHE_OCCUPY_ADDR,
    #[doc = "0x3c - ******* Description ***********"]
    pub dcache_occupy_size: DCACHE_OCCUPY_SIZE,
    #[doc = "0x40 - ******* Description ***********"]
    pub dcache_preload_ctrl: DCACHE_PRELOAD_CTRL,
    #[doc = "0x44 - ******* Description ***********"]
    pub dcache_preload_addr: DCACHE_PRELOAD_ADDR,
    #[doc = "0x48 - ******* Description ***********"]
    pub dcache_preload_size: DCACHE_PRELOAD_SIZE,
    #[doc = "0x4c - ******* Description ***********"]
    pub dcache_autoload_ctrl: DCACHE_AUTOLOAD_CTRL,
    #[doc = "0x50 - ******* Description ***********"]
    pub dcache_autoload_sct0_addr: DCACHE_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x54 - ******* Description ***********"]
    pub dcache_autoload_sct0_size: DCACHE_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x58 - ******* Description ***********"]
    pub dcache_autoload_sct1_addr: DCACHE_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x5c - ******* Description ***********"]
    pub dcache_autoload_sct1_size: DCACHE_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x60 - ******* Description ***********"]
    pub icache_ctrl: ICACHE_CTRL,
    #[doc = "0x64 - ******* Description ***********"]
    pub icache_ctrl1: ICACHE_CTRL1,
    #[doc = "0x68 - ******* Description ***********"]
    pub icache_tag_power_ctrl: ICACHE_TAG_POWER_CTRL,
    #[doc = "0x6c - ******* Description ***********"]
    pub icache_prelock_ctrl: ICACHE_PRELOCK_CTRL,
    #[doc = "0x70 - ******* Description ***********"]
    pub icache_prelock_sct0_addr: ICACHE_PRELOCK_SCT0_ADDR,
    #[doc = "0x74 - ******* Description ***********"]
    pub icache_prelock_sct1_addr: ICACHE_PRELOCK_SCT1_ADDR,
    #[doc = "0x78 - ******* Description ***********"]
    pub icache_prelock_sct_size: ICACHE_PRELOCK_SCT_SIZE,
    #[doc = "0x7c - ******* Description ***********"]
    pub icache_lock_ctrl: ICACHE_LOCK_CTRL,
    #[doc = "0x80 - ******* Description ***********"]
    pub icache_lock_addr: ICACHE_LOCK_ADDR,
    #[doc = "0x84 - ******* Description ***********"]
    pub icache_lock_size: ICACHE_LOCK_SIZE,
    #[doc = "0x88 - ******* Description ***********"]
    pub icache_sync_ctrl: ICACHE_SYNC_CTRL,
    #[doc = "0x8c - ******* Description ***********"]
    pub icache_sync_addr: ICACHE_SYNC_ADDR,
    #[doc = "0x90 - ******* Description ***********"]
    pub icache_sync_size: ICACHE_SYNC_SIZE,
    #[doc = "0x94 - ******* Description ***********"]
    pub icache_preload_ctrl: ICACHE_PRELOAD_CTRL,
    #[doc = "0x98 - ******* Description ***********"]
    pub icache_preload_addr: ICACHE_PRELOAD_ADDR,
    #[doc = "0x9c - ******* Description ***********"]
    pub icache_preload_size: ICACHE_PRELOAD_SIZE,
    #[doc = "0xa0 - ******* Description ***********"]
    pub icache_autoload_ctrl: ICACHE_AUTOLOAD_CTRL,
    #[doc = "0xa4 - ******* Description ***********"]
    pub icache_autoload_sct0_addr: ICACHE_AUTOLOAD_SCT0_ADDR,
    #[doc = "0xa8 - ******* Description ***********"]
    pub icache_autoload_sct0_size: ICACHE_AUTOLOAD_SCT0_SIZE,
    #[doc = "0xac - ******* Description ***********"]
    pub icache_autoload_sct1_addr: ICACHE_AUTOLOAD_SCT1_ADDR,
    #[doc = "0xb0 - ******* Description ***********"]
    pub icache_autoload_sct1_size: ICACHE_AUTOLOAD_SCT1_SIZE,
    #[doc = "0xb4 - ******* Description ***********"]
    pub ibus_to_flash_start_vaddr: IBUS_TO_FLASH_START_VADDR,
    #[doc = "0xb8 - ******* Description ***********"]
    pub ibus_to_flash_end_vaddr: IBUS_TO_FLASH_END_VADDR,
    #[doc = "0xbc - ******* Description ***********"]
    pub dbus_to_flash_start_vaddr: DBUS_TO_FLASH_START_VADDR,
    #[doc = "0xc0 - ******* Description ***********"]
    pub dbus_to_flash_end_vaddr: DBUS_TO_FLASH_END_VADDR,
    #[doc = "0xc4 - ******* Description ***********"]
    pub cache_acs_cnt_clr: CACHE_ACS_CNT_CLR,
    #[doc = "0xc8 - ******* Description ***********"]
    pub ibus_acs_miss_cnt: IBUS_ACS_MISS_CNT,
    #[doc = "0xcc - ******* Description ***********"]
    pub ibus_acs_cnt: IBUS_ACS_CNT,
    #[doc = "0xd0 - ******* Description ***********"]
    pub dbus_acs_flash_miss_cnt: DBUS_ACS_FLASH_MISS_CNT,
    #[doc = "0xd4 - ******* Description ***********"]
    pub dbus_acs_spiram_miss_cnt: DBUS_ACS_SPIRAM_MISS_CNT,
    #[doc = "0xd8 - ******* Description ***********"]
    pub dbus_acs_cnt: DBUS_ACS_CNT,
    #[doc = "0xdc - ******* Description ***********"]
    pub cache_ilg_int_ena: CACHE_ILG_INT_ENA,
    #[doc = "0xe0 - ******* Description ***********"]
    pub cache_ilg_int_clr: CACHE_ILG_INT_CLR,
    #[doc = "0xe4 - ******* Description ***********"]
    pub cache_ilg_int_st: CACHE_ILG_INT_ST,
    #[doc = "0xe8 - ******* Description ***********"]
    pub core0_acs_cache_int_ena: CORE0_ACS_CACHE_INT_ENA,
    #[doc = "0xec - ******* Description ***********"]
    pub core0_acs_cache_int_clr: CORE0_ACS_CACHE_INT_CLR,
    #[doc = "0xf0 - ******* Description ***********"]
    pub core0_acs_cache_int_st: CORE0_ACS_CACHE_INT_ST,
    #[doc = "0xf4 - ******* Description ***********"]
    pub core1_acs_cache_int_ena: CORE1_ACS_CACHE_INT_ENA,
    #[doc = "0xf8 - ******* Description ***********"]
    pub core1_acs_cache_int_clr: CORE1_ACS_CACHE_INT_CLR,
    #[doc = "0xfc - ******* Description ***********"]
    pub core1_acs_cache_int_st: CORE1_ACS_CACHE_INT_ST,
    #[doc = "0x100 - ******* Description ***********"]
    pub core0_dbus_reject_st: CORE0_DBUS_REJECT_ST,
    #[doc = "0x104 - ******* Description ***********"]
    pub core0_dbus_reject_vaddr: CORE0_DBUS_REJECT_VADDR,
    #[doc = "0x108 - ******* Description ***********"]
    pub core0_ibus_reject_st: CORE0_IBUS_REJECT_ST,
    #[doc = "0x10c - ******* Description ***********"]
    pub core0_ibus_reject_vaddr: CORE0_IBUS_REJECT_VADDR,
    #[doc = "0x110 - ******* Description ***********"]
    pub core1_dbus_reject_st: CORE1_DBUS_REJECT_ST,
    #[doc = "0x114 - ******* Description ***********"]
    pub core1_dbus_reject_vaddr: CORE1_DBUS_REJECT_VADDR,
    #[doc = "0x118 - ******* Description ***********"]
    pub core1_ibus_reject_st: CORE1_IBUS_REJECT_ST,
    #[doc = "0x11c - ******* Description ***********"]
    pub core1_ibus_reject_vaddr: CORE1_IBUS_REJECT_VADDR,
    #[doc = "0x120 - ******* Description ***********"]
    pub cache_mmu_fault_content: CACHE_MMU_FAULT_CONTENT,
    #[doc = "0x124 - ******* Description ***********"]
    pub cache_mmu_fault_vaddr: CACHE_MMU_FAULT_VADDR,
    #[doc = "0x128 - ******* Description ***********"]
    pub cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    #[doc = "0x12c - ******* Description ***********"]
    pub cache_mmu_power_ctrl: CACHE_MMU_POWER_CTRL,
    #[doc = "0x130 - ******* Description ***********"]
    pub cache_state: CACHE_STATE,
    #[doc = "0x134 - ******* Description ***********"]
    pub cache_encrypt_decrypt_record_disable: CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    #[doc = "0x138 - ******* Description ***********"]
    pub cache_encrypt_decrypt_clk_force_on: CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    #[doc = "0x13c - ******* Description ***********"]
    pub cache_bridge_arbiter_ctrl: CACHE_BRIDGE_ARBITER_CTRL,
    #[doc = "0x140 - ******* Description ***********"]
    pub cache_preload_int_ctrl: CACHE_PRELOAD_INT_CTRL,
    #[doc = "0x144 - ******* Description ***********"]
    pub cache_sync_int_ctrl: CACHE_SYNC_INT_CTRL,
    #[doc = "0x148 - ******* Description ***********"]
    pub cache_mmu_owner: CACHE_MMU_OWNER,
    #[doc = "0x14c - ******* Description ***********"]
    pub cache_conf_misc: CACHE_CONF_MISC,
    #[doc = "0x150 - ******* Description ***********"]
    pub dcache_freeze: DCACHE_FREEZE,
    #[doc = "0x154 - ******* Description ***********"]
    pub icache_freeze: ICACHE_FREEZE,
    #[doc = "0x158 - ******* Description ***********"]
    pub icache_atomic_operate_ena: ICACHE_ATOMIC_OPERATE_ENA,
    #[doc = "0x15c - ******* Description ***********"]
    pub dcache_atomic_operate_ena: DCACHE_ATOMIC_OPERATE_ENA,
    #[doc = "0x160 - ******* Description ***********"]
    pub cache_request: CACHE_REQUEST,
    #[doc = "0x164 - ******* Description ***********"]
    pub clock_gate: CLOCK_GATE,
    _reserved90: [u8; 0x18],
    #[doc = "0x180 - ******* Description ***********"]
    pub cache_tag_object_ctrl: CACHE_TAG_OBJECT_CTRL,
    #[doc = "0x184 - ******* Description ***********"]
    pub cache_tag_way_object: CACHE_TAG_WAY_OBJECT,
    #[doc = "0x188 - ******* Description ***********"]
    pub cache_vaddr: CACHE_VADDR,
    #[doc = "0x18c - ******* Description ***********"]
    pub cache_tag_content: CACHE_TAG_CONTENT,
    _reserved94: [u8; 0x026c],
    #[doc = "0x3fc - ******* Description ***********"]
    pub date: DATE,
}
#[doc = "DCACHE_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_ctrl`] module"]
pub type DCACHE_CTRL = crate::Reg<dcache_ctrl::DCACHE_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_ctrl;
#[doc = "DCACHE_CTRL1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_ctrl1`] module"]
pub type DCACHE_CTRL1 = crate::Reg<dcache_ctrl1::DCACHE_CTRL1_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_ctrl1;
#[doc = "DCACHE_TAG_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_tag_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_tag_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_tag_power_ctrl`] module"]
pub type DCACHE_TAG_POWER_CTRL = crate::Reg<dcache_tag_power_ctrl::DCACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_tag_power_ctrl;
#[doc = "DCACHE_PRELOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_prelock_ctrl`] module"]
pub type DCACHE_PRELOCK_CTRL = crate::Reg<dcache_prelock_ctrl::DCACHE_PRELOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_ctrl;
#[doc = "DCACHE_PRELOCK_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_prelock_sct0_addr`] module"]
pub type DCACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<dcache_prelock_sct0_addr::DCACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct0_addr;
#[doc = "DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_prelock_sct1_addr`] module"]
pub type DCACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<dcache_prelock_sct1_addr::DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct1_addr;
#[doc = "DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_prelock_sct_size`] module"]
pub type DCACHE_PRELOCK_SCT_SIZE =
    crate::Reg<dcache_prelock_sct_size::DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct_size;
#[doc = "DCACHE_LOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_lock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_lock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_lock_ctrl`] module"]
pub type DCACHE_LOCK_CTRL = crate::Reg<dcache_lock_ctrl::DCACHE_LOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_ctrl;
#[doc = "DCACHE_LOCK_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_lock_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_lock_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_lock_addr`] module"]
pub type DCACHE_LOCK_ADDR = crate::Reg<dcache_lock_addr::DCACHE_LOCK_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_addr;
#[doc = "DCACHE_LOCK_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_lock_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_lock_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_lock_size`] module"]
pub type DCACHE_LOCK_SIZE = crate::Reg<dcache_lock_size::DCACHE_LOCK_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_size;
#[doc = "DCACHE_SYNC_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_sync_ctrl`] module"]
pub type DCACHE_SYNC_CTRL = crate::Reg<dcache_sync_ctrl::DCACHE_SYNC_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_ctrl;
#[doc = "DCACHE_SYNC_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_sync_addr`] module"]
pub type DCACHE_SYNC_ADDR = crate::Reg<dcache_sync_addr::DCACHE_SYNC_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_addr;
#[doc = "DCACHE_SYNC_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_sync_size`] module"]
pub type DCACHE_SYNC_SIZE = crate::Reg<dcache_sync_size::DCACHE_SYNC_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_size;
#[doc = "DCACHE_OCCUPY_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_occupy_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_occupy_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_occupy_ctrl`] module"]
pub type DCACHE_OCCUPY_CTRL = crate::Reg<dcache_occupy_ctrl::DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_ctrl;
#[doc = "DCACHE_OCCUPY_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_occupy_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_occupy_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_occupy_addr`] module"]
pub type DCACHE_OCCUPY_ADDR = crate::Reg<dcache_occupy_addr::DCACHE_OCCUPY_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_addr;
#[doc = "DCACHE_OCCUPY_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_occupy_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_occupy_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_occupy_size`] module"]
pub type DCACHE_OCCUPY_SIZE = crate::Reg<dcache_occupy_size::DCACHE_OCCUPY_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_size;
#[doc = "DCACHE_PRELOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_preload_ctrl`] module"]
pub type DCACHE_PRELOAD_CTRL = crate::Reg<dcache_preload_ctrl::DCACHE_PRELOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_ctrl;
#[doc = "DCACHE_PRELOAD_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_preload_addr`] module"]
pub type DCACHE_PRELOAD_ADDR = crate::Reg<dcache_preload_addr::DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_addr;
#[doc = "DCACHE_PRELOAD_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_preload_size`] module"]
pub type DCACHE_PRELOAD_SIZE = crate::Reg<dcache_preload_size::DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_size;
#[doc = "DCACHE_AUTOLOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_autoload_ctrl`] module"]
pub type DCACHE_AUTOLOAD_CTRL = crate::Reg<dcache_autoload_ctrl::DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_ctrl;
#[doc = "DCACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_autoload_sct0_addr`] module"]
pub type DCACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<dcache_autoload_sct0_addr::DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct0_addr;
#[doc = "DCACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_autoload_sct0_size`] module"]
pub type DCACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<dcache_autoload_sct0_size::DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct0_size;
#[doc = "DCACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_autoload_sct1_addr`] module"]
pub type DCACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<dcache_autoload_sct1_addr::DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct1_addr;
#[doc = "DCACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_autoload_sct1_size`] module"]
pub type DCACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<dcache_autoload_sct1_size::DCACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct1_size;
#[doc = "ICACHE_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_ctrl`] module"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_ctrl;
#[doc = "ICACHE_CTRL1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_ctrl1`] module"]
pub type ICACHE_CTRL1 = crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_ctrl1;
#[doc = "ICACHE_TAG_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_tag_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_tag_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_tag_power_ctrl`] module"]
pub type ICACHE_TAG_POWER_CTRL = crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_tag_power_ctrl;
#[doc = "ICACHE_PRELOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_prelock_ctrl`] module"]
pub type ICACHE_PRELOCK_CTRL = crate::Reg<icache_prelock_ctrl::ICACHE_PRELOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_ctrl;
#[doc = "ICACHE_PRELOCK_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_prelock_sct0_addr`] module"]
pub type ICACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<icache_prelock_sct0_addr::ICACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct0_addr;
#[doc = "ICACHE_PRELOCK_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_prelock_sct1_addr`] module"]
pub type ICACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<icache_prelock_sct1_addr::ICACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct1_addr;
#[doc = "ICACHE_PRELOCK_SCT_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_prelock_sct_size`] module"]
pub type ICACHE_PRELOCK_SCT_SIZE =
    crate::Reg<icache_prelock_sct_size::ICACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct_size;
#[doc = "ICACHE_LOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_lock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_lock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_lock_ctrl`] module"]
pub type ICACHE_LOCK_CTRL = crate::Reg<icache_lock_ctrl::ICACHE_LOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_ctrl;
#[doc = "ICACHE_LOCK_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_lock_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_lock_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_lock_addr`] module"]
pub type ICACHE_LOCK_ADDR = crate::Reg<icache_lock_addr::ICACHE_LOCK_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_addr;
#[doc = "ICACHE_LOCK_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_lock_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_lock_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_lock_size`] module"]
pub type ICACHE_LOCK_SIZE = crate::Reg<icache_lock_size::ICACHE_LOCK_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_size;
#[doc = "ICACHE_SYNC_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_sync_ctrl`] module"]
pub type ICACHE_SYNC_CTRL = crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_ctrl;
#[doc = "ICACHE_SYNC_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_sync_addr`] module"]
pub type ICACHE_SYNC_ADDR = crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_addr;
#[doc = "ICACHE_SYNC_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_sync_size`] module"]
pub type ICACHE_SYNC_SIZE = crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_size;
#[doc = "ICACHE_PRELOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_preload_ctrl`] module"]
pub type ICACHE_PRELOAD_CTRL = crate::Reg<icache_preload_ctrl::ICACHE_PRELOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_ctrl;
#[doc = "ICACHE_PRELOAD_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_preload_addr`] module"]
pub type ICACHE_PRELOAD_ADDR = crate::Reg<icache_preload_addr::ICACHE_PRELOAD_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_addr;
#[doc = "ICACHE_PRELOAD_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_preload_size`] module"]
pub type ICACHE_PRELOAD_SIZE = crate::Reg<icache_preload_size::ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_size;
#[doc = "ICACHE_AUTOLOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_autoload_ctrl`] module"]
pub type ICACHE_AUTOLOAD_CTRL = crate::Reg<icache_autoload_ctrl::ICACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_ctrl;
#[doc = "ICACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_autoload_sct0_addr`] module"]
pub type ICACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache_autoload_sct0_addr::ICACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct0_addr;
#[doc = "ICACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_autoload_sct0_size`] module"]
pub type ICACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache_autoload_sct0_size::ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct0_size;
#[doc = "ICACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_autoload_sct1_addr`] module"]
pub type ICACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache_autoload_sct1_addr::ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct1_addr;
#[doc = "ICACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_autoload_sct1_size`] module"]
pub type ICACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache_autoload_sct1_size::ICACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct1_size;
#[doc = "IBUS_TO_FLASH_START_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_to_flash_start_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ibus_to_flash_start_vaddr`] module"]
pub type IBUS_TO_FLASH_START_VADDR =
    crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_to_flash_start_vaddr;
#[doc = "IBUS_TO_FLASH_END_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_to_flash_end_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ibus_to_flash_end_vaddr`] module"]
pub type IBUS_TO_FLASH_END_VADDR =
    crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_to_flash_end_vaddr;
#[doc = "DBUS_TO_FLASH_START_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_to_flash_start_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbus_to_flash_start_vaddr`] module"]
pub type DBUS_TO_FLASH_START_VADDR =
    crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_to_flash_start_vaddr;
#[doc = "DBUS_TO_FLASH_END_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_to_flash_end_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbus_to_flash_end_vaddr`] module"]
pub type DBUS_TO_FLASH_END_VADDR =
    crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_to_flash_end_vaddr;
#[doc = "CACHE_ACS_CNT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_acs_cnt_clr`] module"]
pub type CACHE_ACS_CNT_CLR = crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_acs_cnt_clr;
#[doc = "IBUS_ACS_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ibus_acs_miss_cnt`] module"]
pub type IBUS_ACS_MISS_CNT = crate::Reg<ibus_acs_miss_cnt::IBUS_ACS_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_acs_miss_cnt;
#[doc = "IBUS_ACS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_acs_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ibus_acs_cnt`] module"]
pub type IBUS_ACS_CNT = crate::Reg<ibus_acs_cnt::IBUS_ACS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_acs_cnt;
#[doc = "DBUS_ACS_FLASH_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_acs_flash_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbus_acs_flash_miss_cnt`] module"]
pub type DBUS_ACS_FLASH_MISS_CNT =
    crate::Reg<dbus_acs_flash_miss_cnt::DBUS_ACS_FLASH_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_flash_miss_cnt;
#[doc = "DBUS_ACS_SPIRAM_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_acs_spiram_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbus_acs_spiram_miss_cnt`] module"]
pub type DBUS_ACS_SPIRAM_MISS_CNT =
    crate::Reg<dbus_acs_spiram_miss_cnt::DBUS_ACS_SPIRAM_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_spiram_miss_cnt;
#[doc = "DBUS_ACS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_acs_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dbus_acs_cnt`] module"]
pub type DBUS_ACS_CNT = crate::Reg<dbus_acs_cnt::DBUS_ACS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_cnt;
#[doc = "CACHE_ILG_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_ilg_int_ena`] module"]
pub type CACHE_ILG_INT_ENA = crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_ena;
#[doc = "CACHE_ILG_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ilg_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_ilg_int_clr`] module"]
pub type CACHE_ILG_INT_CLR = crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_clr;
#[doc = "CACHE_ILG_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ilg_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_ilg_int_st`] module"]
pub type CACHE_ILG_INT_ST = crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_st;
#[doc = "CORE0_ACS_CACHE_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_acs_cache_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_acs_cache_int_ena`] module"]
pub type CORE0_ACS_CACHE_INT_ENA =
    crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_ena;
#[doc = "CORE0_ACS_CACHE_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_acs_cache_int_clr`] module"]
pub type CORE0_ACS_CACHE_INT_CLR =
    crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_clr;
#[doc = "CORE0_ACS_CACHE_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_acs_cache_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_acs_cache_int_st`] module"]
pub type CORE0_ACS_CACHE_INT_ST = crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_st;
#[doc = "CORE1_ACS_CACHE_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_acs_cache_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core1_acs_cache_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_acs_cache_int_ena`] module"]
pub type CORE1_ACS_CACHE_INT_ENA =
    crate::Reg<core1_acs_cache_int_ena::CORE1_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_ena;
#[doc = "CORE1_ACS_CACHE_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core1_acs_cache_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_acs_cache_int_clr`] module"]
pub type CORE1_ACS_CACHE_INT_CLR =
    crate::Reg<core1_acs_cache_int_clr::CORE1_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_clr;
#[doc = "CORE1_ACS_CACHE_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_acs_cache_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_acs_cache_int_st`] module"]
pub type CORE1_ACS_CACHE_INT_ST = crate::Reg<core1_acs_cache_int_st::CORE1_ACS_CACHE_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_st;
#[doc = "CORE0_DBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_dbus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_dbus_reject_st`] module"]
pub type CORE0_DBUS_REJECT_ST = crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_dbus_reject_st;
#[doc = "CORE0_DBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_dbus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_dbus_reject_vaddr`] module"]
pub type CORE0_DBUS_REJECT_VADDR =
    crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_dbus_reject_vaddr;
#[doc = "CORE0_IBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_ibus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_ibus_reject_st`] module"]
pub type CORE0_IBUS_REJECT_ST = crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_ibus_reject_st;
#[doc = "CORE0_IBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_ibus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core0_ibus_reject_vaddr`] module"]
pub type CORE0_IBUS_REJECT_VADDR =
    crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_ibus_reject_vaddr;
#[doc = "CORE1_DBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_dbus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_dbus_reject_st`] module"]
pub type CORE1_DBUS_REJECT_ST = crate::Reg<core1_dbus_reject_st::CORE1_DBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_dbus_reject_st;
#[doc = "CORE1_DBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_dbus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_dbus_reject_vaddr`] module"]
pub type CORE1_DBUS_REJECT_VADDR =
    crate::Reg<core1_dbus_reject_vaddr::CORE1_DBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_dbus_reject_vaddr;
#[doc = "CORE1_IBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_ibus_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_ibus_reject_st`] module"]
pub type CORE1_IBUS_REJECT_ST = crate::Reg<core1_ibus_reject_st::CORE1_IBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_ibus_reject_st;
#[doc = "CORE1_IBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_ibus_reject_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core1_ibus_reject_vaddr`] module"]
pub type CORE1_IBUS_REJECT_VADDR =
    crate::Reg<core1_ibus_reject_vaddr::CORE1_IBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_ibus_reject_vaddr;
#[doc = "CACHE_MMU_FAULT_CONTENT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_fault_content::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_mmu_fault_content`] module"]
pub type CACHE_MMU_FAULT_CONTENT =
    crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_fault_content;
#[doc = "CACHE_MMU_FAULT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_fault_vaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_mmu_fault_vaddr`] module"]
pub type CACHE_MMU_FAULT_VADDR = crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_fault_vaddr;
#[doc = "CACHE_WRAP_AROUND_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_wrap_around_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_wrap_around_ctrl`] module"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MMU_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_mmu_power_ctrl`] module"]
pub type CACHE_MMU_POWER_CTRL = crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_power_ctrl;
#[doc = "CACHE_STATE (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_state`] module"]
pub type CACHE_STATE = crate::Reg<cache_state::CACHE_STATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_encrypt_decrypt_record_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_record_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_encrypt_decrypt_record_disable`] module"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_encrypt_decrypt_clk_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_clk_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_encrypt_decrypt_clk_force_on`] module"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_BRIDGE_ARBITER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_bridge_arbiter_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_bridge_arbiter_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_bridge_arbiter_ctrl`] module"]
pub type CACHE_BRIDGE_ARBITER_CTRL =
    crate::Reg<cache_bridge_arbiter_ctrl::CACHE_BRIDGE_ARBITER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_bridge_arbiter_ctrl;
#[doc = "CACHE_PRELOAD_INT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_preload_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_preload_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_preload_int_ctrl`] module"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_sync_int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sync_int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_sync_int_ctrl`] module"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_MMU_OWNER (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mmu_owner::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mmu_owner::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_mmu_owner`] module"]
pub type CACHE_MMU_OWNER = crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_owner;
#[doc = "CACHE_CONF_MISC (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_conf_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_conf_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_conf_misc`] module"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_conf_misc;
#[doc = "DCACHE_FREEZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_freeze`] module"]
pub type DCACHE_FREEZE = crate::Reg<dcache_freeze::DCACHE_FREEZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_freeze;
#[doc = "ICACHE_FREEZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_freeze::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_freeze::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_freeze`] module"]
pub type ICACHE_FREEZE = crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_freeze;
#[doc = "ICACHE_ATOMIC_OPERATE_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_atomic_operate_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_atomic_operate_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_atomic_operate_ena`] module"]
pub type ICACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_atomic_operate_ena;
#[doc = "DCACHE_ATOMIC_OPERATE_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_atomic_operate_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_atomic_operate_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_atomic_operate_ena`] module"]
pub type DCACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<dcache_atomic_operate_ena::DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_atomic_operate_ena;
#[doc = "CACHE_REQUEST (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_request`] module"]
pub type CACHE_REQUEST = crate::Reg<cache_request::CACHE_REQUEST_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_request;
#[doc = "CLOCK_GATE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod clock_gate;
#[doc = "CACHE_TAG_OBJECT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_object_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_object_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_tag_object_ctrl`] module"]
pub type CACHE_TAG_OBJECT_CTRL = crate::Reg<cache_tag_object_ctrl::CACHE_TAG_OBJECT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_object_ctrl;
#[doc = "CACHE_TAG_WAY_OBJECT (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_way_object::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_way_object::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_tag_way_object`] module"]
pub type CACHE_TAG_WAY_OBJECT = crate::Reg<cache_tag_way_object::CACHE_TAG_WAY_OBJECT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_way_object;
#[doc = "CACHE_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_vaddr`] module"]
pub type CACHE_VADDR = crate::Reg<cache_vaddr::CACHE_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_vaddr;
#[doc = "CACHE_TAG_CONTENT (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_content::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_content::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_tag_content`] module"]
pub type CACHE_TAG_CONTENT = crate::Reg<cache_tag_content::CACHE_TAG_CONTENT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_content;
#[doc = "DATE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod date;
