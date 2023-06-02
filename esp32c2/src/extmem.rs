#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - This description will be updated in the near future."]
    pub icache_ctrl: ICACHE_CTRL,
    #[doc = "0x04 - This description will be updated in the near future."]
    pub icache_ctrl1: ICACHE_CTRL1,
    #[doc = "0x08 - This description will be updated in the near future."]
    pub icache_tag_power_ctrl: ICACHE_TAG_POWER_CTRL,
    _reserved3: [u8; 0x1c],
    #[doc = "0x28 - This description will be updated in the near future."]
    pub icache_sync_ctrl: ICACHE_SYNC_CTRL,
    #[doc = "0x2c - This description will be updated in the near future."]
    pub icache_sync_addr: ICACHE_SYNC_ADDR,
    #[doc = "0x30 - This description will be updated in the near future."]
    pub icache_sync_size: ICACHE_SYNC_SIZE,
    _reserved6: [u8; 0x20],
    #[doc = "0x54 - This description will be updated in the near future."]
    pub ibus_to_flash_start_vaddr: IBUS_TO_FLASH_START_VADDR,
    #[doc = "0x58 - This description will be updated in the near future."]
    pub ibus_to_flash_end_vaddr: IBUS_TO_FLASH_END_VADDR,
    #[doc = "0x5c - This description will be updated in the near future."]
    pub dbus_to_flash_start_vaddr: DBUS_TO_FLASH_START_VADDR,
    #[doc = "0x60 - This description will be updated in the near future."]
    pub dbus_to_flash_end_vaddr: DBUS_TO_FLASH_END_VADDR,
    #[doc = "0x64 - This description will be updated in the near future."]
    pub cache_acs_cnt_clr: CACHE_ACS_CNT_CLR,
    _reserved11: [u8; 0x10],
    #[doc = "0x78 - This description will be updated in the near future."]
    pub cache_ilg_int_ena: CACHE_ILG_INT_ENA,
    #[doc = "0x7c - This description will be updated in the near future."]
    pub cache_ilg_int_clr: CACHE_ILG_INT_CLR,
    #[doc = "0x80 - This description will be updated in the near future."]
    pub cache_ilg_int_st: CACHE_ILG_INT_ST,
    #[doc = "0x84 - This description will be updated in the near future."]
    pub core0_acs_cache_int_ena: CORE0_ACS_CACHE_INT_ENA,
    #[doc = "0x88 - This description will be updated in the near future."]
    pub core0_acs_cache_int_clr: CORE0_ACS_CACHE_INT_CLR,
    #[doc = "0x8c - This description will be updated in the near future."]
    pub core0_acs_cache_int_st: CORE0_ACS_CACHE_INT_ST,
    #[doc = "0x90 - This description will be updated in the near future."]
    pub core0_dbus_reject_st: CORE0_DBUS_REJECT_ST,
    #[doc = "0x94 - This description will be updated in the near future."]
    pub core0_dbus_reject_vaddr: CORE0_DBUS_REJECT_VADDR,
    #[doc = "0x98 - This description will be updated in the near future."]
    pub core0_ibus_reject_st: CORE0_IBUS_REJECT_ST,
    #[doc = "0x9c - This description will be updated in the near future."]
    pub core0_ibus_reject_vaddr: CORE0_IBUS_REJECT_VADDR,
    #[doc = "0xa0 - This description will be updated in the near future."]
    pub cache_mmu_fault_content: CACHE_MMU_FAULT_CONTENT,
    #[doc = "0xa4 - This description will be updated in the near future."]
    pub cache_mmu_fault_vaddr: CACHE_MMU_FAULT_VADDR,
    #[doc = "0xa8 - This description will be updated in the near future."]
    pub cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    #[doc = "0xac - This description will be updated in the near future."]
    pub cache_mmu_power_ctrl: CACHE_MMU_POWER_CTRL,
    #[doc = "0xb0 - This description will be updated in the near future."]
    pub cache_state: CACHE_STATE,
    #[doc = "0xb4 - This description will be updated in the near future."]
    pub cache_encrypt_decrypt_record_disable: CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    #[doc = "0xb8 - This description will be updated in the near future."]
    pub cache_encrypt_decrypt_clk_force_on: CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    #[doc = "0xbc - This description will be updated in the near future."]
    pub cache_preload_int_ctrl: CACHE_PRELOAD_INT_CTRL,
    #[doc = "0xc0 - This description will be updated in the near future."]
    pub cache_sync_int_ctrl: CACHE_SYNC_INT_CTRL,
    #[doc = "0xc4 - This description will be updated in the near future."]
    pub cache_mmu_owner: CACHE_MMU_OWNER,
    #[doc = "0xc8 - This description will be updated in the near future."]
    pub cache_conf_misc: CACHE_CONF_MISC,
    #[doc = "0xcc - This description will be updated in the near future."]
    pub icache_freeze: ICACHE_FREEZE,
    #[doc = "0xd0 - This description will be updated in the near future."]
    pub icache_atomic_operate_ena: ICACHE_ATOMIC_OPERATE_ENA,
    #[doc = "0xd4 - This description will be updated in the near future."]
    pub cache_request: CACHE_REQUEST,
    _reserved35: [u8; 0x28],
    #[doc = "0x100 - This description will be updated in the near future."]
    pub clock_gate: CLOCK_GATE,
    _reserved36: [u8; 0x02f8],
    #[doc = "0x3fc - This description will be updated in the near future."]
    pub reg_date: REG_DATE,
}
#[doc = "ICACHE_CTRL (rw) register accessor: an alias for `Reg<ICACHE_CTRL_SPEC>`"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_ctrl;
#[doc = "ICACHE_CTRL1 (rw) register accessor: an alias for `Reg<ICACHE_CTRL1_SPEC>`"]
pub type ICACHE_CTRL1 = crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_ctrl1;
#[doc = "ICACHE_TAG_POWER_CTRL (rw) register accessor: an alias for `Reg<ICACHE_TAG_POWER_CTRL_SPEC>`"]
pub type ICACHE_TAG_POWER_CTRL = crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_tag_power_ctrl;
#[doc = "ICACHE_SYNC_CTRL (rw) register accessor: an alias for `Reg<ICACHE_SYNC_CTRL_SPEC>`"]
pub type ICACHE_SYNC_CTRL = crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_ctrl;
#[doc = "ICACHE_SYNC_ADDR (rw) register accessor: an alias for `Reg<ICACHE_SYNC_ADDR_SPEC>`"]
pub type ICACHE_SYNC_ADDR = crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_addr;
#[doc = "ICACHE_SYNC_SIZE (rw) register accessor: an alias for `Reg<ICACHE_SYNC_SIZE_SPEC>`"]
pub type ICACHE_SYNC_SIZE = crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_sync_size;
#[doc = "IBUS_TO_FLASH_START_VADDR (rw) register accessor: an alias for `Reg<IBUS_TO_FLASH_START_VADDR_SPEC>`"]
pub type IBUS_TO_FLASH_START_VADDR =
    crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod ibus_to_flash_start_vaddr;
#[doc = "IBUS_TO_FLASH_END_VADDR (rw) register accessor: an alias for `Reg<IBUS_TO_FLASH_END_VADDR_SPEC>`"]
pub type IBUS_TO_FLASH_END_VADDR =
    crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod ibus_to_flash_end_vaddr;
#[doc = "DBUS_TO_FLASH_START_VADDR (rw) register accessor: an alias for `Reg<DBUS_TO_FLASH_START_VADDR_SPEC>`"]
pub type DBUS_TO_FLASH_START_VADDR =
    crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod dbus_to_flash_start_vaddr;
#[doc = "DBUS_TO_FLASH_END_VADDR (rw) register accessor: an alias for `Reg<DBUS_TO_FLASH_END_VADDR_SPEC>`"]
pub type DBUS_TO_FLASH_END_VADDR =
    crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod dbus_to_flash_end_vaddr;
#[doc = "CACHE_ACS_CNT_CLR (w) register accessor: an alias for `Reg<CACHE_ACS_CNT_CLR_SPEC>`"]
pub type CACHE_ACS_CNT_CLR = crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_acs_cnt_clr;
#[doc = "CACHE_ILG_INT_ENA (rw) register accessor: an alias for `Reg<CACHE_ILG_INT_ENA_SPEC>`"]
pub type CACHE_ILG_INT_ENA = crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_ena;
#[doc = "CACHE_ILG_INT_CLR (w) register accessor: an alias for `Reg<CACHE_ILG_INT_CLR_SPEC>`"]
pub type CACHE_ILG_INT_CLR = crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_clr;
#[doc = "CACHE_ILG_INT_ST (r) register accessor: an alias for `Reg<CACHE_ILG_INT_ST_SPEC>`"]
pub type CACHE_ILG_INT_ST = crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_ilg_int_st;
#[doc = "CORE0_ACS_CACHE_INT_ENA (rw) register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_ENA_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_ENA =
    crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_ena;
#[doc = "CORE0_ACS_CACHE_INT_CLR (w) register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_CLR_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_CLR =
    crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_clr;
#[doc = "CORE0_ACS_CACHE_INT_ST (r) register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_ST_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_ST = crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_acs_cache_int_st;
#[doc = "CORE0_DBUS_REJECT_ST (r) register accessor: an alias for `Reg<CORE0_DBUS_REJECT_ST_SPEC>`"]
pub type CORE0_DBUS_REJECT_ST = crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_dbus_reject_st;
#[doc = "CORE0_DBUS_REJECT_VADDR (r) register accessor: an alias for `Reg<CORE0_DBUS_REJECT_VADDR_SPEC>`"]
pub type CORE0_DBUS_REJECT_VADDR =
    crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_dbus_reject_vaddr;
#[doc = "CORE0_IBUS_REJECT_ST (r) register accessor: an alias for `Reg<CORE0_IBUS_REJECT_ST_SPEC>`"]
pub type CORE0_IBUS_REJECT_ST = crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_ibus_reject_st;
#[doc = "CORE0_IBUS_REJECT_VADDR (r) register accessor: an alias for `Reg<CORE0_IBUS_REJECT_VADDR_SPEC>`"]
pub type CORE0_IBUS_REJECT_VADDR =
    crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod core0_ibus_reject_vaddr;
#[doc = "CACHE_MMU_FAULT_CONTENT (r) register accessor: an alias for `Reg<CACHE_MMU_FAULT_CONTENT_SPEC>`"]
pub type CACHE_MMU_FAULT_CONTENT =
    crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_fault_content;
#[doc = "CACHE_MMU_FAULT_VADDR (r) register accessor: an alias for `Reg<CACHE_MMU_FAULT_VADDR_SPEC>`"]
pub type CACHE_MMU_FAULT_VADDR = crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_fault_vaddr;
#[doc = "CACHE_WRAP_AROUND_CTRL (rw) register accessor: an alias for `Reg<CACHE_WRAP_AROUND_CTRL_SPEC>`"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MMU_POWER_CTRL (rw) register accessor: an alias for `Reg<CACHE_MMU_POWER_CTRL_SPEC>`"]
pub type CACHE_MMU_POWER_CTRL = crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_power_ctrl;
#[doc = "CACHE_STATE (r) register accessor: an alias for `Reg<CACHE_STATE_SPEC>`"]
pub type CACHE_STATE = crate::Reg<cache_state::CACHE_STATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE (rw) register accessor: an alias for `Reg<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>`"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON (rw) register accessor: an alias for `Reg<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>`"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_PRELOAD_INT_CTRL (rw) register accessor: an alias for `Reg<CACHE_PRELOAD_INT_CTRL_SPEC>`"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL (rw) register accessor: an alias for `Reg<CACHE_SYNC_INT_CTRL_SPEC>`"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_MMU_OWNER (rw) register accessor: an alias for `Reg<CACHE_MMU_OWNER_SPEC>`"]
pub type CACHE_MMU_OWNER = crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_mmu_owner;
#[doc = "CACHE_CONF_MISC (rw) register accessor: an alias for `Reg<CACHE_CONF_MISC_SPEC>`"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_conf_misc;
#[doc = "ICACHE_FREEZE (rw) register accessor: an alias for `Reg<ICACHE_FREEZE_SPEC>`"]
pub type ICACHE_FREEZE = crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_freeze;
#[doc = "ICACHE_ATOMIC_OPERATE_ENA (rw) register accessor: an alias for `Reg<ICACHE_ATOMIC_OPERATE_ENA_SPEC>`"]
pub type ICACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod icache_atomic_operate_ena;
#[doc = "CACHE_REQUEST (rw) register accessor: an alias for `Reg<CACHE_REQUEST_SPEC>`"]
pub type CACHE_REQUEST = crate::Reg<cache_request::CACHE_REQUEST_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod cache_request;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod clock_gate;
#[doc = "REG_DATE (rw) register accessor: an alias for `Reg<REG_DATE_SPEC>`"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "This description will be updated in the near future."]
pub mod reg_date;
