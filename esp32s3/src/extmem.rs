#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcache_ctrl: DCACHE_CTRL,
    dcache_ctrl1: DCACHE_CTRL1,
    dcache_tag_power_ctrl: DCACHE_TAG_POWER_CTRL,
    dcache_prelock_ctrl: DCACHE_PRELOCK_CTRL,
    dcache_prelock_sct0_addr: DCACHE_PRELOCK_SCT0_ADDR,
    dcache_prelock_sct1_addr: DCACHE_PRELOCK_SCT1_ADDR,
    dcache_prelock_sct_size: DCACHE_PRELOCK_SCT_SIZE,
    dcache_lock_ctrl: DCACHE_LOCK_CTRL,
    dcache_lock_addr: DCACHE_LOCK_ADDR,
    dcache_lock_size: DCACHE_LOCK_SIZE,
    dcache_sync_ctrl: DCACHE_SYNC_CTRL,
    dcache_sync_addr: DCACHE_SYNC_ADDR,
    dcache_sync_size: DCACHE_SYNC_SIZE,
    dcache_occupy_ctrl: DCACHE_OCCUPY_CTRL,
    dcache_occupy_addr: DCACHE_OCCUPY_ADDR,
    dcache_occupy_size: DCACHE_OCCUPY_SIZE,
    dcache_preload_ctrl: DCACHE_PRELOAD_CTRL,
    dcache_preload_addr: DCACHE_PRELOAD_ADDR,
    dcache_preload_size: DCACHE_PRELOAD_SIZE,
    dcache_autoload_ctrl: DCACHE_AUTOLOAD_CTRL,
    dcache_autoload_sct0_addr: DCACHE_AUTOLOAD_SCT0_ADDR,
    dcache_autoload_sct0_size: DCACHE_AUTOLOAD_SCT0_SIZE,
    dcache_autoload_sct1_addr: DCACHE_AUTOLOAD_SCT1_ADDR,
    dcache_autoload_sct1_size: DCACHE_AUTOLOAD_SCT1_SIZE,
    icache_ctrl: ICACHE_CTRL,
    icache_ctrl1: ICACHE_CTRL1,
    icache_tag_power_ctrl: ICACHE_TAG_POWER_CTRL,
    icache_prelock_ctrl: ICACHE_PRELOCK_CTRL,
    icache_prelock_sct0_addr: ICACHE_PRELOCK_SCT0_ADDR,
    icache_prelock_sct1_addr: ICACHE_PRELOCK_SCT1_ADDR,
    icache_prelock_sct_size: ICACHE_PRELOCK_SCT_SIZE,
    icache_lock_ctrl: ICACHE_LOCK_CTRL,
    icache_lock_addr: ICACHE_LOCK_ADDR,
    icache_lock_size: ICACHE_LOCK_SIZE,
    icache_sync_ctrl: ICACHE_SYNC_CTRL,
    icache_sync_addr: ICACHE_SYNC_ADDR,
    icache_sync_size: ICACHE_SYNC_SIZE,
    icache_preload_ctrl: ICACHE_PRELOAD_CTRL,
    icache_preload_addr: ICACHE_PRELOAD_ADDR,
    icache_preload_size: ICACHE_PRELOAD_SIZE,
    icache_autoload_ctrl: ICACHE_AUTOLOAD_CTRL,
    icache_autoload_sct0_addr: ICACHE_AUTOLOAD_SCT0_ADDR,
    icache_autoload_sct0_size: ICACHE_AUTOLOAD_SCT0_SIZE,
    icache_autoload_sct1_addr: ICACHE_AUTOLOAD_SCT1_ADDR,
    icache_autoload_sct1_size: ICACHE_AUTOLOAD_SCT1_SIZE,
    ibus_to_flash_start_vaddr: IBUS_TO_FLASH_START_VADDR,
    ibus_to_flash_end_vaddr: IBUS_TO_FLASH_END_VADDR,
    dbus_to_flash_start_vaddr: DBUS_TO_FLASH_START_VADDR,
    dbus_to_flash_end_vaddr: DBUS_TO_FLASH_END_VADDR,
    cache_acs_cnt_clr: CACHE_ACS_CNT_CLR,
    ibus_acs_miss_cnt: IBUS_ACS_MISS_CNT,
    ibus_acs_cnt: IBUS_ACS_CNT,
    dbus_acs_flash_miss_cnt: DBUS_ACS_FLASH_MISS_CNT,
    dbus_acs_spiram_miss_cnt: DBUS_ACS_SPIRAM_MISS_CNT,
    dbus_acs_cnt: DBUS_ACS_CNT,
    cache_ilg_int_ena: CACHE_ILG_INT_ENA,
    cache_ilg_int_clr: CACHE_ILG_INT_CLR,
    cache_ilg_int_st: CACHE_ILG_INT_ST,
    core0_acs_cache_int_ena: CORE0_ACS_CACHE_INT_ENA,
    core0_acs_cache_int_clr: CORE0_ACS_CACHE_INT_CLR,
    core0_acs_cache_int_st: CORE0_ACS_CACHE_INT_ST,
    core1_acs_cache_int_ena: CORE1_ACS_CACHE_INT_ENA,
    core1_acs_cache_int_clr: CORE1_ACS_CACHE_INT_CLR,
    core1_acs_cache_int_st: CORE1_ACS_CACHE_INT_ST,
    core0_dbus_reject_st: CORE0_DBUS_REJECT_ST,
    core0_dbus_reject_vaddr: CORE0_DBUS_REJECT_VADDR,
    core0_ibus_reject_st: CORE0_IBUS_REJECT_ST,
    core0_ibus_reject_vaddr: CORE0_IBUS_REJECT_VADDR,
    core1_dbus_reject_st: CORE1_DBUS_REJECT_ST,
    core1_dbus_reject_vaddr: CORE1_DBUS_REJECT_VADDR,
    core1_ibus_reject_st: CORE1_IBUS_REJECT_ST,
    core1_ibus_reject_vaddr: CORE1_IBUS_REJECT_VADDR,
    cache_mmu_fault_content: CACHE_MMU_FAULT_CONTENT,
    cache_mmu_fault_vaddr: CACHE_MMU_FAULT_VADDR,
    cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    cache_mmu_power_ctrl: CACHE_MMU_POWER_CTRL,
    cache_state: CACHE_STATE,
    cache_encrypt_decrypt_record_disable: CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    cache_encrypt_decrypt_clk_force_on: CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    cache_bridge_arbiter_ctrl: CACHE_BRIDGE_ARBITER_CTRL,
    cache_preload_int_ctrl: CACHE_PRELOAD_INT_CTRL,
    cache_sync_int_ctrl: CACHE_SYNC_INT_CTRL,
    cache_mmu_owner: CACHE_MMU_OWNER,
    cache_conf_misc: CACHE_CONF_MISC,
    dcache_freeze: DCACHE_FREEZE,
    icache_freeze: ICACHE_FREEZE,
    icache_atomic_operate_ena: ICACHE_ATOMIC_OPERATE_ENA,
    dcache_atomic_operate_ena: DCACHE_ATOMIC_OPERATE_ENA,
    cache_request: CACHE_REQUEST,
    clock_gate: CLOCK_GATE,
    _reserved90: [u8; 0x18],
    cache_tag_object_ctrl: CACHE_TAG_OBJECT_CTRL,
    cache_tag_way_object: CACHE_TAG_WAY_OBJECT,
    cache_vaddr: CACHE_VADDR,
    cache_tag_content: CACHE_TAG_CONTENT,
    _reserved94: [u8; 0x026c],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_ctrl(&self) -> &DCACHE_CTRL {
        &self.dcache_ctrl
    }
    #[doc = "0x04 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_ctrl1(&self) -> &DCACHE_CTRL1 {
        &self.dcache_ctrl1
    }
    #[doc = "0x08 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_tag_power_ctrl(&self) -> &DCACHE_TAG_POWER_CTRL {
        &self.dcache_tag_power_ctrl
    }
    #[doc = "0x0c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_prelock_ctrl(&self) -> &DCACHE_PRELOCK_CTRL {
        &self.dcache_prelock_ctrl
    }
    #[doc = "0x10 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_prelock_sct0_addr(&self) -> &DCACHE_PRELOCK_SCT0_ADDR {
        &self.dcache_prelock_sct0_addr
    }
    #[doc = "0x14 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_prelock_sct1_addr(&self) -> &DCACHE_PRELOCK_SCT1_ADDR {
        &self.dcache_prelock_sct1_addr
    }
    #[doc = "0x18 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_prelock_sct_size(&self) -> &DCACHE_PRELOCK_SCT_SIZE {
        &self.dcache_prelock_sct_size
    }
    #[doc = "0x1c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_lock_ctrl(&self) -> &DCACHE_LOCK_CTRL {
        &self.dcache_lock_ctrl
    }
    #[doc = "0x20 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_lock_addr(&self) -> &DCACHE_LOCK_ADDR {
        &self.dcache_lock_addr
    }
    #[doc = "0x24 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_lock_size(&self) -> &DCACHE_LOCK_SIZE {
        &self.dcache_lock_size
    }
    #[doc = "0x28 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_sync_ctrl(&self) -> &DCACHE_SYNC_CTRL {
        &self.dcache_sync_ctrl
    }
    #[doc = "0x2c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_sync_addr(&self) -> &DCACHE_SYNC_ADDR {
        &self.dcache_sync_addr
    }
    #[doc = "0x30 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_sync_size(&self) -> &DCACHE_SYNC_SIZE {
        &self.dcache_sync_size
    }
    #[doc = "0x34 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_occupy_ctrl(&self) -> &DCACHE_OCCUPY_CTRL {
        &self.dcache_occupy_ctrl
    }
    #[doc = "0x38 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_occupy_addr(&self) -> &DCACHE_OCCUPY_ADDR {
        &self.dcache_occupy_addr
    }
    #[doc = "0x3c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_occupy_size(&self) -> &DCACHE_OCCUPY_SIZE {
        &self.dcache_occupy_size
    }
    #[doc = "0x40 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_preload_ctrl(&self) -> &DCACHE_PRELOAD_CTRL {
        &self.dcache_preload_ctrl
    }
    #[doc = "0x44 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_preload_addr(&self) -> &DCACHE_PRELOAD_ADDR {
        &self.dcache_preload_addr
    }
    #[doc = "0x48 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_preload_size(&self) -> &DCACHE_PRELOAD_SIZE {
        &self.dcache_preload_size
    }
    #[doc = "0x4c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_autoload_ctrl(&self) -> &DCACHE_AUTOLOAD_CTRL {
        &self.dcache_autoload_ctrl
    }
    #[doc = "0x50 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_autoload_sct0_addr(&self) -> &DCACHE_AUTOLOAD_SCT0_ADDR {
        &self.dcache_autoload_sct0_addr
    }
    #[doc = "0x54 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_autoload_sct0_size(&self) -> &DCACHE_AUTOLOAD_SCT0_SIZE {
        &self.dcache_autoload_sct0_size
    }
    #[doc = "0x58 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_autoload_sct1_addr(&self) -> &DCACHE_AUTOLOAD_SCT1_ADDR {
        &self.dcache_autoload_sct1_addr
    }
    #[doc = "0x5c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_autoload_sct1_size(&self) -> &DCACHE_AUTOLOAD_SCT1_SIZE {
        &self.dcache_autoload_sct1_size
    }
    #[doc = "0x60 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_ctrl(&self) -> &ICACHE_CTRL {
        &self.icache_ctrl
    }
    #[doc = "0x64 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_ctrl1(&self) -> &ICACHE_CTRL1 {
        &self.icache_ctrl1
    }
    #[doc = "0x68 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_tag_power_ctrl(&self) -> &ICACHE_TAG_POWER_CTRL {
        &self.icache_tag_power_ctrl
    }
    #[doc = "0x6c - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_prelock_ctrl(&self) -> &ICACHE_PRELOCK_CTRL {
        &self.icache_prelock_ctrl
    }
    #[doc = "0x70 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_prelock_sct0_addr(&self) -> &ICACHE_PRELOCK_SCT0_ADDR {
        &self.icache_prelock_sct0_addr
    }
    #[doc = "0x74 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_prelock_sct1_addr(&self) -> &ICACHE_PRELOCK_SCT1_ADDR {
        &self.icache_prelock_sct1_addr
    }
    #[doc = "0x78 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_prelock_sct_size(&self) -> &ICACHE_PRELOCK_SCT_SIZE {
        &self.icache_prelock_sct_size
    }
    #[doc = "0x7c - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_lock_ctrl(&self) -> &ICACHE_LOCK_CTRL {
        &self.icache_lock_ctrl
    }
    #[doc = "0x80 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_lock_addr(&self) -> &ICACHE_LOCK_ADDR {
        &self.icache_lock_addr
    }
    #[doc = "0x84 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_lock_size(&self) -> &ICACHE_LOCK_SIZE {
        &self.icache_lock_size
    }
    #[doc = "0x88 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_sync_ctrl(&self) -> &ICACHE_SYNC_CTRL {
        &self.icache_sync_ctrl
    }
    #[doc = "0x8c - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_sync_addr(&self) -> &ICACHE_SYNC_ADDR {
        &self.icache_sync_addr
    }
    #[doc = "0x90 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_sync_size(&self) -> &ICACHE_SYNC_SIZE {
        &self.icache_sync_size
    }
    #[doc = "0x94 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_preload_ctrl(&self) -> &ICACHE_PRELOAD_CTRL {
        &self.icache_preload_ctrl
    }
    #[doc = "0x98 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_preload_addr(&self) -> &ICACHE_PRELOAD_ADDR {
        &self.icache_preload_addr
    }
    #[doc = "0x9c - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_preload_size(&self) -> &ICACHE_PRELOAD_SIZE {
        &self.icache_preload_size
    }
    #[doc = "0xa0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_autoload_ctrl(&self) -> &ICACHE_AUTOLOAD_CTRL {
        &self.icache_autoload_ctrl
    }
    #[doc = "0xa4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_autoload_sct0_addr(&self) -> &ICACHE_AUTOLOAD_SCT0_ADDR {
        &self.icache_autoload_sct0_addr
    }
    #[doc = "0xa8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_autoload_sct0_size(&self) -> &ICACHE_AUTOLOAD_SCT0_SIZE {
        &self.icache_autoload_sct0_size
    }
    #[doc = "0xac - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_autoload_sct1_addr(&self) -> &ICACHE_AUTOLOAD_SCT1_ADDR {
        &self.icache_autoload_sct1_addr
    }
    #[doc = "0xb0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_autoload_sct1_size(&self) -> &ICACHE_AUTOLOAD_SCT1_SIZE {
        &self.icache_autoload_sct1_size
    }
    #[doc = "0xb4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn ibus_to_flash_start_vaddr(&self) -> &IBUS_TO_FLASH_START_VADDR {
        &self.ibus_to_flash_start_vaddr
    }
    #[doc = "0xb8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn ibus_to_flash_end_vaddr(&self) -> &IBUS_TO_FLASH_END_VADDR {
        &self.ibus_to_flash_end_vaddr
    }
    #[doc = "0xbc - ******* Description ***********"]
    #[inline(always)]
    pub const fn dbus_to_flash_start_vaddr(&self) -> &DBUS_TO_FLASH_START_VADDR {
        &self.dbus_to_flash_start_vaddr
    }
    #[doc = "0xc0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dbus_to_flash_end_vaddr(&self) -> &DBUS_TO_FLASH_END_VADDR {
        &self.dbus_to_flash_end_vaddr
    }
    #[doc = "0xc4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_acs_cnt_clr(&self) -> &CACHE_ACS_CNT_CLR {
        &self.cache_acs_cnt_clr
    }
    #[doc = "0xc8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn ibus_acs_miss_cnt(&self) -> &IBUS_ACS_MISS_CNT {
        &self.ibus_acs_miss_cnt
    }
    #[doc = "0xcc - ******* Description ***********"]
    #[inline(always)]
    pub const fn ibus_acs_cnt(&self) -> &IBUS_ACS_CNT {
        &self.ibus_acs_cnt
    }
    #[doc = "0xd0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dbus_acs_flash_miss_cnt(&self) -> &DBUS_ACS_FLASH_MISS_CNT {
        &self.dbus_acs_flash_miss_cnt
    }
    #[doc = "0xd4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dbus_acs_spiram_miss_cnt(&self) -> &DBUS_ACS_SPIRAM_MISS_CNT {
        &self.dbus_acs_spiram_miss_cnt
    }
    #[doc = "0xd8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dbus_acs_cnt(&self) -> &DBUS_ACS_CNT {
        &self.dbus_acs_cnt
    }
    #[doc = "0xdc - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_ilg_int_ena(&self) -> &CACHE_ILG_INT_ENA {
        &self.cache_ilg_int_ena
    }
    #[doc = "0xe0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_ilg_int_clr(&self) -> &CACHE_ILG_INT_CLR {
        &self.cache_ilg_int_clr
    }
    #[doc = "0xe4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_ilg_int_st(&self) -> &CACHE_ILG_INT_ST {
        &self.cache_ilg_int_st
    }
    #[doc = "0xe8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_acs_cache_int_ena(&self) -> &CORE0_ACS_CACHE_INT_ENA {
        &self.core0_acs_cache_int_ena
    }
    #[doc = "0xec - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_acs_cache_int_clr(&self) -> &CORE0_ACS_CACHE_INT_CLR {
        &self.core0_acs_cache_int_clr
    }
    #[doc = "0xf0 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_acs_cache_int_st(&self) -> &CORE0_ACS_CACHE_INT_ST {
        &self.core0_acs_cache_int_st
    }
    #[doc = "0xf4 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_acs_cache_int_ena(&self) -> &CORE1_ACS_CACHE_INT_ENA {
        &self.core1_acs_cache_int_ena
    }
    #[doc = "0xf8 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_acs_cache_int_clr(&self) -> &CORE1_ACS_CACHE_INT_CLR {
        &self.core1_acs_cache_int_clr
    }
    #[doc = "0xfc - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_acs_cache_int_st(&self) -> &CORE1_ACS_CACHE_INT_ST {
        &self.core1_acs_cache_int_st
    }
    #[doc = "0x100 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_dbus_reject_st(&self) -> &CORE0_DBUS_REJECT_ST {
        &self.core0_dbus_reject_st
    }
    #[doc = "0x104 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_dbus_reject_vaddr(&self) -> &CORE0_DBUS_REJECT_VADDR {
        &self.core0_dbus_reject_vaddr
    }
    #[doc = "0x108 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_ibus_reject_st(&self) -> &CORE0_IBUS_REJECT_ST {
        &self.core0_ibus_reject_st
    }
    #[doc = "0x10c - ******* Description ***********"]
    #[inline(always)]
    pub const fn core0_ibus_reject_vaddr(&self) -> &CORE0_IBUS_REJECT_VADDR {
        &self.core0_ibus_reject_vaddr
    }
    #[doc = "0x110 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_dbus_reject_st(&self) -> &CORE1_DBUS_REJECT_ST {
        &self.core1_dbus_reject_st
    }
    #[doc = "0x114 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_dbus_reject_vaddr(&self) -> &CORE1_DBUS_REJECT_VADDR {
        &self.core1_dbus_reject_vaddr
    }
    #[doc = "0x118 - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_ibus_reject_st(&self) -> &CORE1_IBUS_REJECT_ST {
        &self.core1_ibus_reject_st
    }
    #[doc = "0x11c - ******* Description ***********"]
    #[inline(always)]
    pub const fn core1_ibus_reject_vaddr(&self) -> &CORE1_IBUS_REJECT_VADDR {
        &self.core1_ibus_reject_vaddr
    }
    #[doc = "0x120 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_mmu_fault_content(&self) -> &CACHE_MMU_FAULT_CONTENT {
        &self.cache_mmu_fault_content
    }
    #[doc = "0x124 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_mmu_fault_vaddr(&self) -> &CACHE_MMU_FAULT_VADDR {
        &self.cache_mmu_fault_vaddr
    }
    #[doc = "0x128 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_wrap_around_ctrl(&self) -> &CACHE_WRAP_AROUND_CTRL {
        &self.cache_wrap_around_ctrl
    }
    #[doc = "0x12c - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_mmu_power_ctrl(&self) -> &CACHE_MMU_POWER_CTRL {
        &self.cache_mmu_power_ctrl
    }
    #[doc = "0x130 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_state(&self) -> &CACHE_STATE {
        &self.cache_state
    }
    #[doc = "0x134 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_record_disable(
        &self,
    ) -> &CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {
        &self.cache_encrypt_decrypt_record_disable
    }
    #[doc = "0x138 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_clk_force_on(&self) -> &CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {
        &self.cache_encrypt_decrypt_clk_force_on
    }
    #[doc = "0x13c - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_bridge_arbiter_ctrl(&self) -> &CACHE_BRIDGE_ARBITER_CTRL {
        &self.cache_bridge_arbiter_ctrl
    }
    #[doc = "0x140 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_preload_int_ctrl(&self) -> &CACHE_PRELOAD_INT_CTRL {
        &self.cache_preload_int_ctrl
    }
    #[doc = "0x144 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_sync_int_ctrl(&self) -> &CACHE_SYNC_INT_CTRL {
        &self.cache_sync_int_ctrl
    }
    #[doc = "0x148 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_mmu_owner(&self) -> &CACHE_MMU_OWNER {
        &self.cache_mmu_owner
    }
    #[doc = "0x14c - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_conf_misc(&self) -> &CACHE_CONF_MISC {
        &self.cache_conf_misc
    }
    #[doc = "0x150 - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_freeze(&self) -> &DCACHE_FREEZE {
        &self.dcache_freeze
    }
    #[doc = "0x154 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_freeze(&self) -> &ICACHE_FREEZE {
        &self.icache_freeze
    }
    #[doc = "0x158 - ******* Description ***********"]
    #[inline(always)]
    pub const fn icache_atomic_operate_ena(&self) -> &ICACHE_ATOMIC_OPERATE_ENA {
        &self.icache_atomic_operate_ena
    }
    #[doc = "0x15c - ******* Description ***********"]
    #[inline(always)]
    pub const fn dcache_atomic_operate_ena(&self) -> &DCACHE_ATOMIC_OPERATE_ENA {
        &self.dcache_atomic_operate_ena
    }
    #[doc = "0x160 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_request(&self) -> &CACHE_REQUEST {
        &self.cache_request
    }
    #[doc = "0x164 - ******* Description ***********"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x180 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_tag_object_ctrl(&self) -> &CACHE_TAG_OBJECT_CTRL {
        &self.cache_tag_object_ctrl
    }
    #[doc = "0x184 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_tag_way_object(&self) -> &CACHE_TAG_WAY_OBJECT {
        &self.cache_tag_way_object
    }
    #[doc = "0x188 - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_vaddr(&self) -> &CACHE_VADDR {
        &self.cache_vaddr
    }
    #[doc = "0x18c - ******* Description ***********"]
    #[inline(always)]
    pub const fn cache_tag_content(&self) -> &CACHE_TAG_CONTENT {
        &self.cache_tag_content
    }
    #[doc = "0x3fc - ******* Description ***********"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "DCACHE_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_ctrl`] module"]
pub type DCACHE_CTRL = crate::Reg<dcache_ctrl::DCACHE_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_ctrl;
#[doc = "DCACHE_CTRL1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_ctrl1`] module"]
pub type DCACHE_CTRL1 = crate::Reg<dcache_ctrl1::DCACHE_CTRL1_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_ctrl1;
#[doc = "DCACHE_TAG_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_tag_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_tag_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_tag_power_ctrl`] module"]
pub type DCACHE_TAG_POWER_CTRL = crate::Reg<dcache_tag_power_ctrl::DCACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_tag_power_ctrl;
#[doc = "DCACHE_PRELOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_ctrl`] module"]
pub type DCACHE_PRELOCK_CTRL = crate::Reg<dcache_prelock_ctrl::DCACHE_PRELOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_ctrl;
#[doc = "DCACHE_PRELOCK_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct0_addr`] module"]
pub type DCACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<dcache_prelock_sct0_addr::DCACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct0_addr;
#[doc = "DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct1_addr`] module"]
pub type DCACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<dcache_prelock_sct1_addr::DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct1_addr;
#[doc = "DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct_size`] module"]
pub type DCACHE_PRELOCK_SCT_SIZE =
    crate::Reg<dcache_prelock_sct_size::DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_prelock_sct_size;
#[doc = "DCACHE_LOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_lock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_lock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_lock_ctrl`] module"]
pub type DCACHE_LOCK_CTRL = crate::Reg<dcache_lock_ctrl::DCACHE_LOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_ctrl;
#[doc = "DCACHE_LOCK_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_lock_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_lock_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_lock_addr`] module"]
pub type DCACHE_LOCK_ADDR = crate::Reg<dcache_lock_addr::DCACHE_LOCK_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_addr;
#[doc = "DCACHE_LOCK_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_lock_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_lock_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_lock_size`] module"]
pub type DCACHE_LOCK_SIZE = crate::Reg<dcache_lock_size::DCACHE_LOCK_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_lock_size;
#[doc = "DCACHE_SYNC_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_sync_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_sync_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_sync_ctrl`] module"]
pub type DCACHE_SYNC_CTRL = crate::Reg<dcache_sync_ctrl::DCACHE_SYNC_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_ctrl;
#[doc = "DCACHE_SYNC_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_sync_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_sync_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_sync_addr`] module"]
pub type DCACHE_SYNC_ADDR = crate::Reg<dcache_sync_addr::DCACHE_SYNC_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_addr;
#[doc = "DCACHE_SYNC_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_sync_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_sync_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_sync_size`] module"]
pub type DCACHE_SYNC_SIZE = crate::Reg<dcache_sync_size::DCACHE_SYNC_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_sync_size;
#[doc = "DCACHE_OCCUPY_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_occupy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_occupy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_occupy_ctrl`] module"]
pub type DCACHE_OCCUPY_CTRL = crate::Reg<dcache_occupy_ctrl::DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_ctrl;
#[doc = "DCACHE_OCCUPY_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_occupy_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_occupy_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_occupy_addr`] module"]
pub type DCACHE_OCCUPY_ADDR = crate::Reg<dcache_occupy_addr::DCACHE_OCCUPY_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_addr;
#[doc = "DCACHE_OCCUPY_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_occupy_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_occupy_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_occupy_size`] module"]
pub type DCACHE_OCCUPY_SIZE = crate::Reg<dcache_occupy_size::DCACHE_OCCUPY_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_occupy_size;
#[doc = "DCACHE_PRELOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_ctrl`] module"]
pub type DCACHE_PRELOAD_CTRL = crate::Reg<dcache_preload_ctrl::DCACHE_PRELOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_ctrl;
#[doc = "DCACHE_PRELOAD_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_addr`] module"]
pub type DCACHE_PRELOAD_ADDR = crate::Reg<dcache_preload_addr::DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_addr;
#[doc = "DCACHE_PRELOAD_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_size`] module"]
pub type DCACHE_PRELOAD_SIZE = crate::Reg<dcache_preload_size::DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_preload_size;
#[doc = "DCACHE_AUTOLOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_autoload_ctrl`] module"]
pub type DCACHE_AUTOLOAD_CTRL = crate::Reg<dcache_autoload_ctrl::DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_ctrl;
#[doc = "DCACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_autoload_sct0_addr`] module"]
pub type DCACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<dcache_autoload_sct0_addr::DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct0_addr;
#[doc = "DCACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_sct0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_sct0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_autoload_sct0_size`] module"]
pub type DCACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<dcache_autoload_sct0_size::DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct0_size;
#[doc = "DCACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_autoload_sct1_addr`] module"]
pub type DCACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<dcache_autoload_sct1_addr::DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct1_addr;
#[doc = "DCACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_sct1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_sct1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_autoload_sct1_size`] module"]
pub type DCACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<dcache_autoload_sct1_size::DCACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_autoload_sct1_size;
#[doc = "ICACHE_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl`] module"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_ctrl;
#[doc = "ICACHE_CTRL1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl1`] module"]
pub type ICACHE_CTRL1 = crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_ctrl1;
#[doc = "ICACHE_TAG_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_tag_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_tag_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_tag_power_ctrl`] module"]
pub type ICACHE_TAG_POWER_CTRL = crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_tag_power_ctrl;
#[doc = "ICACHE_PRELOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_prelock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_prelock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_prelock_ctrl`] module"]
pub type ICACHE_PRELOCK_CTRL = crate::Reg<icache_prelock_ctrl::ICACHE_PRELOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_ctrl;
#[doc = "ICACHE_PRELOCK_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_prelock_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_prelock_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_prelock_sct0_addr`] module"]
pub type ICACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<icache_prelock_sct0_addr::ICACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct0_addr;
#[doc = "ICACHE_PRELOCK_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_prelock_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_prelock_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_prelock_sct1_addr`] module"]
pub type ICACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<icache_prelock_sct1_addr::ICACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct1_addr;
#[doc = "ICACHE_PRELOCK_SCT_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_prelock_sct_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_prelock_sct_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_prelock_sct_size`] module"]
pub type ICACHE_PRELOCK_SCT_SIZE =
    crate::Reg<icache_prelock_sct_size::ICACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_prelock_sct_size;
#[doc = "ICACHE_LOCK_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_lock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_lock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_lock_ctrl`] module"]
pub type ICACHE_LOCK_CTRL = crate::Reg<icache_lock_ctrl::ICACHE_LOCK_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_ctrl;
#[doc = "ICACHE_LOCK_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_lock_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_lock_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_lock_addr`] module"]
pub type ICACHE_LOCK_ADDR = crate::Reg<icache_lock_addr::ICACHE_LOCK_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_addr;
#[doc = "ICACHE_LOCK_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_lock_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_lock_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_lock_size`] module"]
pub type ICACHE_LOCK_SIZE = crate::Reg<icache_lock_size::ICACHE_LOCK_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_lock_size;
#[doc = "ICACHE_SYNC_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_sync_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_sync_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_ctrl`] module"]
pub type ICACHE_SYNC_CTRL = crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_ctrl;
#[doc = "ICACHE_SYNC_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_sync_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_sync_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_addr`] module"]
pub type ICACHE_SYNC_ADDR = crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_addr;
#[doc = "ICACHE_SYNC_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_sync_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_sync_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_sync_size`] module"]
pub type ICACHE_SYNC_SIZE = crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_sync_size;
#[doc = "ICACHE_PRELOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_preload_ctrl`] module"]
pub type ICACHE_PRELOAD_CTRL = crate::Reg<icache_preload_ctrl::ICACHE_PRELOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_ctrl;
#[doc = "ICACHE_PRELOAD_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_preload_addr`] module"]
pub type ICACHE_PRELOAD_ADDR = crate::Reg<icache_preload_addr::ICACHE_PRELOAD_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_addr;
#[doc = "ICACHE_PRELOAD_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_preload_size`] module"]
pub type ICACHE_PRELOAD_SIZE = crate::Reg<icache_preload_size::ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_preload_size;
#[doc = "ICACHE_AUTOLOAD_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_autoload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_autoload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_autoload_ctrl`] module"]
pub type ICACHE_AUTOLOAD_CTRL = crate::Reg<icache_autoload_ctrl::ICACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_ctrl;
#[doc = "ICACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_autoload_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_autoload_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_autoload_sct0_addr`] module"]
pub type ICACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache_autoload_sct0_addr::ICACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct0_addr;
#[doc = "ICACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_autoload_sct0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_autoload_sct0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_autoload_sct0_size`] module"]
pub type ICACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache_autoload_sct0_size::ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct0_size;
#[doc = "ICACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_autoload_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_autoload_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_autoload_sct1_addr`] module"]
pub type ICACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache_autoload_sct1_addr::ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct1_addr;
#[doc = "ICACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_autoload_sct1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_autoload_sct1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_autoload_sct1_size`] module"]
pub type ICACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache_autoload_sct1_size::ICACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_autoload_sct1_size;
#[doc = "IBUS_TO_FLASH_START_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus_to_flash_start_vaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_to_flash_start_vaddr`] module"]
pub type IBUS_TO_FLASH_START_VADDR =
    crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_to_flash_start_vaddr;
#[doc = "IBUS_TO_FLASH_END_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus_to_flash_end_vaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_to_flash_end_vaddr`] module"]
pub type IBUS_TO_FLASH_END_VADDR =
    crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_to_flash_end_vaddr;
#[doc = "DBUS_TO_FLASH_START_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus_to_flash_start_vaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbus_to_flash_start_vaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_to_flash_start_vaddr`] module"]
pub type DBUS_TO_FLASH_START_VADDR =
    crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_to_flash_start_vaddr;
#[doc = "DBUS_TO_FLASH_END_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus_to_flash_end_vaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbus_to_flash_end_vaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_to_flash_end_vaddr`] module"]
pub type DBUS_TO_FLASH_END_VADDR =
    crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_to_flash_end_vaddr;
#[doc = "CACHE_ACS_CNT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_clr`] module"]
pub type CACHE_ACS_CNT_CLR = crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_acs_cnt_clr;
#[doc = "IBUS_ACS_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_acs_miss_cnt`] module"]
pub type IBUS_ACS_MISS_CNT = crate::Reg<ibus_acs_miss_cnt::IBUS_ACS_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_acs_miss_cnt;
#[doc = "IBUS_ACS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus_acs_cnt`] module"]
pub type IBUS_ACS_CNT = crate::Reg<ibus_acs_cnt::IBUS_ACS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod ibus_acs_cnt;
#[doc = "DBUS_ACS_FLASH_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus_acs_flash_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_acs_flash_miss_cnt`] module"]
pub type DBUS_ACS_FLASH_MISS_CNT =
    crate::Reg<dbus_acs_flash_miss_cnt::DBUS_ACS_FLASH_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_flash_miss_cnt;
#[doc = "DBUS_ACS_SPIRAM_MISS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus_acs_spiram_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_acs_spiram_miss_cnt`] module"]
pub type DBUS_ACS_SPIRAM_MISS_CNT =
    crate::Reg<dbus_acs_spiram_miss_cnt::DBUS_ACS_SPIRAM_MISS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_spiram_miss_cnt;
#[doc = "DBUS_ACS_CNT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus_acs_cnt`] module"]
pub type DBUS_ACS_CNT = crate::Reg<dbus_acs_cnt::DBUS_ACS_CNT_SPEC>;
#[doc = "******* Description ***********"]
pub mod dbus_acs_cnt;
#[doc = "CACHE_ILG_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ilg_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ilg_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_ena`] module"]
pub type CACHE_ILG_INT_ENA = crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_ena;
#[doc = "CACHE_ILG_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ilg_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_clr`] module"]
pub type CACHE_ILG_INT_CLR = crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_clr;
#[doc = "CACHE_ILG_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ilg_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ilg_int_st`] module"]
pub type CACHE_ILG_INT_ST = crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_ilg_int_st;
#[doc = "CORE0_ACS_CACHE_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_acs_cache_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_acs_cache_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_ena`] module"]
pub type CORE0_ACS_CACHE_INT_ENA =
    crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_ena;
#[doc = "CORE0_ACS_CACHE_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_acs_cache_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_clr`] module"]
pub type CORE0_ACS_CACHE_INT_CLR =
    crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_clr;
#[doc = "CORE0_ACS_CACHE_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_acs_cache_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_acs_cache_int_st`] module"]
pub type CORE0_ACS_CACHE_INT_ST = crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_acs_cache_int_st;
#[doc = "CORE1_ACS_CACHE_INT_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_acs_cache_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_acs_cache_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_acs_cache_int_ena`] module"]
pub type CORE1_ACS_CACHE_INT_ENA =
    crate::Reg<core1_acs_cache_int_ena::CORE1_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_ena;
#[doc = "CORE1_ACS_CACHE_INT_CLR (w) register accessor: ******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_acs_cache_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_acs_cache_int_clr`] module"]
pub type CORE1_ACS_CACHE_INT_CLR =
    crate::Reg<core1_acs_cache_int_clr::CORE1_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_clr;
#[doc = "CORE1_ACS_CACHE_INT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_acs_cache_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_acs_cache_int_st`] module"]
pub type CORE1_ACS_CACHE_INT_ST = crate::Reg<core1_acs_cache_int_st::CORE1_ACS_CACHE_INT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_acs_cache_int_st;
#[doc = "CORE0_DBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_dbus_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_dbus_reject_st`] module"]
pub type CORE0_DBUS_REJECT_ST = crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_dbus_reject_st;
#[doc = "CORE0_DBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_dbus_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_dbus_reject_vaddr`] module"]
pub type CORE0_DBUS_REJECT_VADDR =
    crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_dbus_reject_vaddr;
#[doc = "CORE0_IBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_ibus_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_ibus_reject_st`] module"]
pub type CORE0_IBUS_REJECT_ST = crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_ibus_reject_st;
#[doc = "CORE0_IBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_ibus_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_ibus_reject_vaddr`] module"]
pub type CORE0_IBUS_REJECT_VADDR =
    crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core0_ibus_reject_vaddr;
#[doc = "CORE1_DBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_dbus_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_dbus_reject_st`] module"]
pub type CORE1_DBUS_REJECT_ST = crate::Reg<core1_dbus_reject_st::CORE1_DBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_dbus_reject_st;
#[doc = "CORE1_DBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_dbus_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_dbus_reject_vaddr`] module"]
pub type CORE1_DBUS_REJECT_VADDR =
    crate::Reg<core1_dbus_reject_vaddr::CORE1_DBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_dbus_reject_vaddr;
#[doc = "CORE1_IBUS_REJECT_ST (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_ibus_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_ibus_reject_st`] module"]
pub type CORE1_IBUS_REJECT_ST = crate::Reg<core1_ibus_reject_st::CORE1_IBUS_REJECT_ST_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_ibus_reject_st;
#[doc = "CORE1_IBUS_REJECT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_ibus_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_ibus_reject_vaddr`] module"]
pub type CORE1_IBUS_REJECT_VADDR =
    crate::Reg<core1_ibus_reject_vaddr::CORE1_IBUS_REJECT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod core1_ibus_reject_vaddr;
#[doc = "CACHE_MMU_FAULT_CONTENT (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_fault_content::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_fault_content`] module"]
pub type CACHE_MMU_FAULT_CONTENT =
    crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_fault_content;
#[doc = "CACHE_MMU_FAULT_VADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_fault_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_fault_vaddr`] module"]
pub type CACHE_MMU_FAULT_VADDR = crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_fault_vaddr;
#[doc = "CACHE_WRAP_AROUND_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_wrap_around_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_wrap_around_ctrl`] module"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MMU_POWER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_power_ctrl`] module"]
pub type CACHE_MMU_POWER_CTRL = crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_power_ctrl;
#[doc = "CACHE_STATE (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_state`] module"]
pub type CACHE_STATE = crate::Reg<cache_state::CACHE_STATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_encrypt_decrypt_record_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_record_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_record_disable`] module"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_encrypt_decrypt_clk_force_on::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_clk_force_on::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_clk_force_on`] module"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_BRIDGE_ARBITER_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_bridge_arbiter_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_bridge_arbiter_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_bridge_arbiter_ctrl`] module"]
pub type CACHE_BRIDGE_ARBITER_CTRL =
    crate::Reg<cache_bridge_arbiter_ctrl::CACHE_BRIDGE_ARBITER_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_bridge_arbiter_ctrl;
#[doc = "CACHE_PRELOAD_INT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_int_ctrl`] module"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_int_ctrl`] module"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_MMU_OWNER (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mmu_owner::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mmu_owner::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mmu_owner`] module"]
pub type CACHE_MMU_OWNER = crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_mmu_owner;
#[doc = "CACHE_CONF_MISC (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_conf_misc`] module"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_conf_misc;
#[doc = "DCACHE_FREEZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_freeze`] module"]
pub type DCACHE_FREEZE = crate::Reg<dcache_freeze::DCACHE_FREEZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_freeze;
#[doc = "ICACHE_FREEZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_freeze::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_freeze::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_freeze`] module"]
pub type ICACHE_FREEZE = crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_freeze;
#[doc = "ICACHE_ATOMIC_OPERATE_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_atomic_operate_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_atomic_operate_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_atomic_operate_ena`] module"]
pub type ICACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod icache_atomic_operate_ena;
#[doc = "DCACHE_ATOMIC_OPERATE_ENA (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_atomic_operate_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_atomic_operate_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_atomic_operate_ena`] module"]
pub type DCACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<dcache_atomic_operate_ena::DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "******* Description ***********"]
pub mod dcache_atomic_operate_ena;
#[doc = "CACHE_REQUEST (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_request::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_request::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_request`] module"]
pub type CACHE_REQUEST = crate::Reg<cache_request::CACHE_REQUEST_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_request;
#[doc = "CLOCK_GATE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod clock_gate;
#[doc = "CACHE_TAG_OBJECT_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_object_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_object_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_object_ctrl`] module"]
pub type CACHE_TAG_OBJECT_CTRL = crate::Reg<cache_tag_object_ctrl::CACHE_TAG_OBJECT_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_object_ctrl;
#[doc = "CACHE_TAG_WAY_OBJECT (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_way_object::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_way_object::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_way_object`] module"]
pub type CACHE_TAG_WAY_OBJECT = crate::Reg<cache_tag_way_object::CACHE_TAG_WAY_OBJECT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_way_object;
#[doc = "CACHE_VADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_vaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_vaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_vaddr`] module"]
pub type CACHE_VADDR = crate::Reg<cache_vaddr::CACHE_VADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_vaddr;
#[doc = "CACHE_TAG_CONTENT (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_content::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_content::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_content`] module"]
pub type CACHE_TAG_CONTENT = crate::Reg<cache_tag_content::CACHE_TAG_CONTENT_SPEC>;
#[doc = "******* Description ***********"]
pub mod cache_tag_content;
pub use crate::aes::date;
pub use crate::aes::DATE;
