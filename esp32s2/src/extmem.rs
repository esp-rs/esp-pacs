#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pro_dcache_ctrl: PRO_DCACHE_CTRL,
    pro_dcache_ctrl1: PRO_DCACHE_CTRL1,
    pro_dcache_tag_power_ctrl: PRO_DCACHE_TAG_POWER_CTRL,
    pro_dcache_lock0_addr: PRO_DCACHE_LOCK0_ADDR,
    pro_dcache_lock0_size: PRO_DCACHE_LOCK0_SIZE,
    pro_dcache_lock1_addr: PRO_DCACHE_LOCK1_ADDR,
    pro_dcache_lock1_size: PRO_DCACHE_LOCK1_SIZE,
    pro_dcache_mem_sync0: PRO_DCACHE_MEM_SYNC0,
    pro_dcache_mem_sync1: PRO_DCACHE_MEM_SYNC1,
    pro_dcache_preload_addr: PRO_DCACHE_PRELOAD_ADDR,
    pro_dcache_preload_size: PRO_DCACHE_PRELOAD_SIZE,
    pro_dcache_autoload_cfg: PRO_DCACHE_AUTOLOAD_CFG,
    pro_dcache_autoload_section0_addr: PRO_DCACHE_AUTOLOAD_SECTION0_ADDR,
    pro_dcache_autoload_section0_size: PRO_DCACHE_AUTOLOAD_SECTION0_SIZE,
    pro_dcache_autoload_section1_addr: PRO_DCACHE_AUTOLOAD_SECTION1_ADDR,
    pro_dcache_autoload_section1_size: PRO_DCACHE_AUTOLOAD_SECTION1_SIZE,
    pro_icache_ctrl: PRO_ICACHE_CTRL,
    pro_icache_ctrl1: PRO_ICACHE_CTRL1,
    pro_icache_tag_power_ctrl: PRO_ICACHE_TAG_POWER_CTRL,
    pro_icache_lock0_addr: PRO_ICACHE_LOCK0_ADDR,
    pro_icache_lock0_size: PRO_ICACHE_LOCK0_SIZE,
    pro_icache_lock1_addr: PRO_ICACHE_LOCK1_ADDR,
    pro_icache_lock1_size: PRO_ICACHE_LOCK1_SIZE,
    pro_icache_mem_sync0: PRO_ICACHE_MEM_SYNC0,
    pro_icache_mem_sync1: PRO_ICACHE_MEM_SYNC1,
    pro_icache_preload_addr: PRO_ICACHE_PRELOAD_ADDR,
    pro_icache_preload_size: PRO_ICACHE_PRELOAD_SIZE,
    pro_icache_autoload_cfg: PRO_ICACHE_AUTOLOAD_CFG,
    pro_icache_autoload_section0_addr: PRO_ICACHE_AUTOLOAD_SECTION0_ADDR,
    pro_icache_autoload_section0_size: PRO_ICACHE_AUTOLOAD_SECTION0_SIZE,
    pro_icache_autoload_section1_addr: PRO_ICACHE_AUTOLOAD_SECTION1_ADDR,
    pro_icache_autoload_section1_size: PRO_ICACHE_AUTOLOAD_SECTION1_SIZE,
    ic_preload_cnt: IC_PRELOAD_CNT,
    ic_preload_miss_cnt: IC_PRELOAD_MISS_CNT,
    ibus2_abandon_cnt: IBUS2_ABANDON_CNT,
    ibus1_abandon_cnt: IBUS1_ABANDON_CNT,
    ibus0_abandon_cnt: IBUS0_ABANDON_CNT,
    ibus2_acs_miss_cnt: IBUS2_ACS_MISS_CNT,
    ibus1_acs_miss_cnt: IBUS1_ACS_MISS_CNT,
    ibus0_acs_miss_cnt: IBUS0_ACS_MISS_CNT,
    ibus2_acs_cnt: IBUS2_ACS_CNT,
    ibus1_acs_cnt: IBUS1_ACS_CNT,
    ibus0_acs_cnt: IBUS0_ACS_CNT,
    dc_preload_cnt: DC_PRELOAD_CNT,
    dc_preload_evict_cnt: DC_PRELOAD_EVICT_CNT,
    dc_preload_miss_cnt: DC_PRELOAD_MISS_CNT,
    dbus2_abandon_cnt: DBUS2_ABANDON_CNT,
    dbus1_abandon_cnt: DBUS1_ABANDON_CNT,
    dbus0_abandon_cnt: DBUS0_ABANDON_CNT,
    dbus2_acs_wb_cnt: DBUS2_ACS_WB_CNT,
    dbus1_acs_wb_cnt: DBUS1_ACS_WB_CNT,
    dbus0_acs_wb_cnt: DBUS0_ACS_WB_CNT,
    dbus2_acs_miss_cnt: DBUS2_ACS_MISS_CNT,
    dbus1_acs_miss_cnt: DBUS1_ACS_MISS_CNT,
    dbus0_acs_miss_cnt: DBUS0_ACS_MISS_CNT,
    dbus2_acs_cnt: DBUS2_ACS_CNT,
    dbus1_acs_cnt: DBUS1_ACS_CNT,
    dbus0_acs_cnt: DBUS0_ACS_CNT,
    cache_dbg_int_ena: CACHE_DBG_INT_ENA,
    cache_dbg_int_clr: CACHE_DBG_INT_CLR,
    cache_dbg_status0: CACHE_DBG_STATUS0,
    cache_dbg_status1: CACHE_DBG_STATUS1,
    pro_cache_acs_cnt_clr: PRO_CACHE_ACS_CNT_CLR,
    pro_dcache_reject_st: PRO_DCACHE_REJECT_ST,
    pro_dcache_reject_vaddr: PRO_DCACHE_REJECT_VADDR,
    pro_icache_reject_st: PRO_ICACHE_REJECT_ST,
    pro_icache_reject_vaddr: PRO_ICACHE_REJECT_VADDR,
    pro_cache_mmu_fault_content: PRO_CACHE_MMU_FAULT_CONTENT,
    pro_cache_mmu_fault_vaddr: PRO_CACHE_MMU_FAULT_VADDR,
    pro_cache_wrap_around_ctrl: PRO_CACHE_WRAP_AROUND_CTRL,
    pro_cache_mmu_power_ctrl: PRO_CACHE_MMU_POWER_CTRL,
    pro_cache_state: PRO_CACHE_STATE,
    cache_encrypt_decrypt_record_disable: CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE,
    cache_encrypt_decrypt_clk_force_on: CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON,
    cache_bridge_arbiter_ctrl: CACHE_BRIDGE_ARBITER_CTRL,
    cache_preload_int_ctrl: CACHE_PRELOAD_INT_CTRL,
    cache_sync_int_ctrl: CACHE_SYNC_INT_CTRL,
    cache_conf_misc: CACHE_CONF_MISC,
    clock_gate: CLOCK_GATE,
    _reserved79: [u8; 0x02c0],
    pro_extmem_reg_date: PRO_EXTMEM_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_ctrl(&self) -> &PRO_DCACHE_CTRL {
        &self.pro_dcache_ctrl
    }
    #[doc = "0x04 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_ctrl1(&self) -> &PRO_DCACHE_CTRL1 {
        &self.pro_dcache_ctrl1
    }
    #[doc = "0x08 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_tag_power_ctrl(&self) -> &PRO_DCACHE_TAG_POWER_CTRL {
        &self.pro_dcache_tag_power_ctrl
    }
    #[doc = "0x0c - register description"]
    #[inline(always)]
    pub const fn pro_dcache_lock0_addr(&self) -> &PRO_DCACHE_LOCK0_ADDR {
        &self.pro_dcache_lock0_addr
    }
    #[doc = "0x10 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_lock0_size(&self) -> &PRO_DCACHE_LOCK0_SIZE {
        &self.pro_dcache_lock0_size
    }
    #[doc = "0x14 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_lock1_addr(&self) -> &PRO_DCACHE_LOCK1_ADDR {
        &self.pro_dcache_lock1_addr
    }
    #[doc = "0x18 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_lock1_size(&self) -> &PRO_DCACHE_LOCK1_SIZE {
        &self.pro_dcache_lock1_size
    }
    #[doc = "0x1c - register description"]
    #[inline(always)]
    pub const fn pro_dcache_mem_sync0(&self) -> &PRO_DCACHE_MEM_SYNC0 {
        &self.pro_dcache_mem_sync0
    }
    #[doc = "0x20 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_mem_sync1(&self) -> &PRO_DCACHE_MEM_SYNC1 {
        &self.pro_dcache_mem_sync1
    }
    #[doc = "0x24 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_preload_addr(&self) -> &PRO_DCACHE_PRELOAD_ADDR {
        &self.pro_dcache_preload_addr
    }
    #[doc = "0x28 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_preload_size(&self) -> &PRO_DCACHE_PRELOAD_SIZE {
        &self.pro_dcache_preload_size
    }
    #[doc = "0x2c - register description"]
    #[inline(always)]
    pub const fn pro_dcache_autoload_cfg(&self) -> &PRO_DCACHE_AUTOLOAD_CFG {
        &self.pro_dcache_autoload_cfg
    }
    #[doc = "0x30 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_autoload_section0_addr(&self) -> &PRO_DCACHE_AUTOLOAD_SECTION0_ADDR {
        &self.pro_dcache_autoload_section0_addr
    }
    #[doc = "0x34 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_autoload_section0_size(&self) -> &PRO_DCACHE_AUTOLOAD_SECTION0_SIZE {
        &self.pro_dcache_autoload_section0_size
    }
    #[doc = "0x38 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_autoload_section1_addr(&self) -> &PRO_DCACHE_AUTOLOAD_SECTION1_ADDR {
        &self.pro_dcache_autoload_section1_addr
    }
    #[doc = "0x3c - register description"]
    #[inline(always)]
    pub const fn pro_dcache_autoload_section1_size(&self) -> &PRO_DCACHE_AUTOLOAD_SECTION1_SIZE {
        &self.pro_dcache_autoload_section1_size
    }
    #[doc = "0x40 - register description"]
    #[inline(always)]
    pub const fn pro_icache_ctrl(&self) -> &PRO_ICACHE_CTRL {
        &self.pro_icache_ctrl
    }
    #[doc = "0x44 - register description"]
    #[inline(always)]
    pub const fn pro_icache_ctrl1(&self) -> &PRO_ICACHE_CTRL1 {
        &self.pro_icache_ctrl1
    }
    #[doc = "0x48 - register description"]
    #[inline(always)]
    pub const fn pro_icache_tag_power_ctrl(&self) -> &PRO_ICACHE_TAG_POWER_CTRL {
        &self.pro_icache_tag_power_ctrl
    }
    #[doc = "0x4c - register description"]
    #[inline(always)]
    pub const fn pro_icache_lock0_addr(&self) -> &PRO_ICACHE_LOCK0_ADDR {
        &self.pro_icache_lock0_addr
    }
    #[doc = "0x50 - register description"]
    #[inline(always)]
    pub const fn pro_icache_lock0_size(&self) -> &PRO_ICACHE_LOCK0_SIZE {
        &self.pro_icache_lock0_size
    }
    #[doc = "0x54 - register description"]
    #[inline(always)]
    pub const fn pro_icache_lock1_addr(&self) -> &PRO_ICACHE_LOCK1_ADDR {
        &self.pro_icache_lock1_addr
    }
    #[doc = "0x58 - register description"]
    #[inline(always)]
    pub const fn pro_icache_lock1_size(&self) -> &PRO_ICACHE_LOCK1_SIZE {
        &self.pro_icache_lock1_size
    }
    #[doc = "0x5c - register description"]
    #[inline(always)]
    pub const fn pro_icache_mem_sync0(&self) -> &PRO_ICACHE_MEM_SYNC0 {
        &self.pro_icache_mem_sync0
    }
    #[doc = "0x60 - register description"]
    #[inline(always)]
    pub const fn pro_icache_mem_sync1(&self) -> &PRO_ICACHE_MEM_SYNC1 {
        &self.pro_icache_mem_sync1
    }
    #[doc = "0x64 - register description"]
    #[inline(always)]
    pub const fn pro_icache_preload_addr(&self) -> &PRO_ICACHE_PRELOAD_ADDR {
        &self.pro_icache_preload_addr
    }
    #[doc = "0x68 - register description"]
    #[inline(always)]
    pub const fn pro_icache_preload_size(&self) -> &PRO_ICACHE_PRELOAD_SIZE {
        &self.pro_icache_preload_size
    }
    #[doc = "0x6c - register description"]
    #[inline(always)]
    pub const fn pro_icache_autoload_cfg(&self) -> &PRO_ICACHE_AUTOLOAD_CFG {
        &self.pro_icache_autoload_cfg
    }
    #[doc = "0x70 - register description"]
    #[inline(always)]
    pub const fn pro_icache_autoload_section0_addr(&self) -> &PRO_ICACHE_AUTOLOAD_SECTION0_ADDR {
        &self.pro_icache_autoload_section0_addr
    }
    #[doc = "0x74 - register description"]
    #[inline(always)]
    pub const fn pro_icache_autoload_section0_size(&self) -> &PRO_ICACHE_AUTOLOAD_SECTION0_SIZE {
        &self.pro_icache_autoload_section0_size
    }
    #[doc = "0x78 - register description"]
    #[inline(always)]
    pub const fn pro_icache_autoload_section1_addr(&self) -> &PRO_ICACHE_AUTOLOAD_SECTION1_ADDR {
        &self.pro_icache_autoload_section1_addr
    }
    #[doc = "0x7c - register description"]
    #[inline(always)]
    pub const fn pro_icache_autoload_section1_size(&self) -> &PRO_ICACHE_AUTOLOAD_SECTION1_SIZE {
        &self.pro_icache_autoload_section1_size
    }
    #[doc = "0x80 - register description"]
    #[inline(always)]
    pub const fn ic_preload_cnt(&self) -> &IC_PRELOAD_CNT {
        &self.ic_preload_cnt
    }
    #[doc = "0x84 - register description"]
    #[inline(always)]
    pub const fn ic_preload_miss_cnt(&self) -> &IC_PRELOAD_MISS_CNT {
        &self.ic_preload_miss_cnt
    }
    #[doc = "0x88 - register description"]
    #[inline(always)]
    pub const fn ibus2_abandon_cnt(&self) -> &IBUS2_ABANDON_CNT {
        &self.ibus2_abandon_cnt
    }
    #[doc = "0x8c - register description"]
    #[inline(always)]
    pub const fn ibus1_abandon_cnt(&self) -> &IBUS1_ABANDON_CNT {
        &self.ibus1_abandon_cnt
    }
    #[doc = "0x90 - register description"]
    #[inline(always)]
    pub const fn ibus0_abandon_cnt(&self) -> &IBUS0_ABANDON_CNT {
        &self.ibus0_abandon_cnt
    }
    #[doc = "0x94 - register description"]
    #[inline(always)]
    pub const fn ibus2_acs_miss_cnt(&self) -> &IBUS2_ACS_MISS_CNT {
        &self.ibus2_acs_miss_cnt
    }
    #[doc = "0x98 - register description"]
    #[inline(always)]
    pub const fn ibus1_acs_miss_cnt(&self) -> &IBUS1_ACS_MISS_CNT {
        &self.ibus1_acs_miss_cnt
    }
    #[doc = "0x9c - register description"]
    #[inline(always)]
    pub const fn ibus0_acs_miss_cnt(&self) -> &IBUS0_ACS_MISS_CNT {
        &self.ibus0_acs_miss_cnt
    }
    #[doc = "0xa0 - register description"]
    #[inline(always)]
    pub const fn ibus2_acs_cnt(&self) -> &IBUS2_ACS_CNT {
        &self.ibus2_acs_cnt
    }
    #[doc = "0xa4 - register description"]
    #[inline(always)]
    pub const fn ibus1_acs_cnt(&self) -> &IBUS1_ACS_CNT {
        &self.ibus1_acs_cnt
    }
    #[doc = "0xa8 - register description"]
    #[inline(always)]
    pub const fn ibus0_acs_cnt(&self) -> &IBUS0_ACS_CNT {
        &self.ibus0_acs_cnt
    }
    #[doc = "0xac - register description"]
    #[inline(always)]
    pub const fn dc_preload_cnt(&self) -> &DC_PRELOAD_CNT {
        &self.dc_preload_cnt
    }
    #[doc = "0xb0 - register description"]
    #[inline(always)]
    pub const fn dc_preload_evict_cnt(&self) -> &DC_PRELOAD_EVICT_CNT {
        &self.dc_preload_evict_cnt
    }
    #[doc = "0xb4 - register description"]
    #[inline(always)]
    pub const fn dc_preload_miss_cnt(&self) -> &DC_PRELOAD_MISS_CNT {
        &self.dc_preload_miss_cnt
    }
    #[doc = "0xb8 - register description"]
    #[inline(always)]
    pub const fn dbus2_abandon_cnt(&self) -> &DBUS2_ABANDON_CNT {
        &self.dbus2_abandon_cnt
    }
    #[doc = "0xbc - register description"]
    #[inline(always)]
    pub const fn dbus1_abandon_cnt(&self) -> &DBUS1_ABANDON_CNT {
        &self.dbus1_abandon_cnt
    }
    #[doc = "0xc0 - register description"]
    #[inline(always)]
    pub const fn dbus0_abandon_cnt(&self) -> &DBUS0_ABANDON_CNT {
        &self.dbus0_abandon_cnt
    }
    #[doc = "0xc4 - register description"]
    #[inline(always)]
    pub const fn dbus2_acs_wb_cnt(&self) -> &DBUS2_ACS_WB_CNT {
        &self.dbus2_acs_wb_cnt
    }
    #[doc = "0xc8 - register description"]
    #[inline(always)]
    pub const fn dbus1_acs_wb_cnt(&self) -> &DBUS1_ACS_WB_CNT {
        &self.dbus1_acs_wb_cnt
    }
    #[doc = "0xcc - register description"]
    #[inline(always)]
    pub const fn dbus0_acs_wb_cnt(&self) -> &DBUS0_ACS_WB_CNT {
        &self.dbus0_acs_wb_cnt
    }
    #[doc = "0xd0 - register description"]
    #[inline(always)]
    pub const fn dbus2_acs_miss_cnt(&self) -> &DBUS2_ACS_MISS_CNT {
        &self.dbus2_acs_miss_cnt
    }
    #[doc = "0xd4 - register description"]
    #[inline(always)]
    pub const fn dbus1_acs_miss_cnt(&self) -> &DBUS1_ACS_MISS_CNT {
        &self.dbus1_acs_miss_cnt
    }
    #[doc = "0xd8 - register description"]
    #[inline(always)]
    pub const fn dbus0_acs_miss_cnt(&self) -> &DBUS0_ACS_MISS_CNT {
        &self.dbus0_acs_miss_cnt
    }
    #[doc = "0xdc - register description"]
    #[inline(always)]
    pub const fn dbus2_acs_cnt(&self) -> &DBUS2_ACS_CNT {
        &self.dbus2_acs_cnt
    }
    #[doc = "0xe0 - register description"]
    #[inline(always)]
    pub const fn dbus1_acs_cnt(&self) -> &DBUS1_ACS_CNT {
        &self.dbus1_acs_cnt
    }
    #[doc = "0xe4 - register description"]
    #[inline(always)]
    pub const fn dbus0_acs_cnt(&self) -> &DBUS0_ACS_CNT {
        &self.dbus0_acs_cnt
    }
    #[doc = "0xe8 - register description"]
    #[inline(always)]
    pub const fn cache_dbg_int_ena(&self) -> &CACHE_DBG_INT_ENA {
        &self.cache_dbg_int_ena
    }
    #[doc = "0xec - register description"]
    #[inline(always)]
    pub const fn cache_dbg_int_clr(&self) -> &CACHE_DBG_INT_CLR {
        &self.cache_dbg_int_clr
    }
    #[doc = "0xf0 - register description"]
    #[inline(always)]
    pub const fn cache_dbg_status0(&self) -> &CACHE_DBG_STATUS0 {
        &self.cache_dbg_status0
    }
    #[doc = "0xf4 - register description"]
    #[inline(always)]
    pub const fn cache_dbg_status1(&self) -> &CACHE_DBG_STATUS1 {
        &self.cache_dbg_status1
    }
    #[doc = "0xf8 - register description"]
    #[inline(always)]
    pub const fn pro_cache_acs_cnt_clr(&self) -> &PRO_CACHE_ACS_CNT_CLR {
        &self.pro_cache_acs_cnt_clr
    }
    #[doc = "0xfc - register description"]
    #[inline(always)]
    pub const fn pro_dcache_reject_st(&self) -> &PRO_DCACHE_REJECT_ST {
        &self.pro_dcache_reject_st
    }
    #[doc = "0x100 - register description"]
    #[inline(always)]
    pub const fn pro_dcache_reject_vaddr(&self) -> &PRO_DCACHE_REJECT_VADDR {
        &self.pro_dcache_reject_vaddr
    }
    #[doc = "0x104 - register description"]
    #[inline(always)]
    pub const fn pro_icache_reject_st(&self) -> &PRO_ICACHE_REJECT_ST {
        &self.pro_icache_reject_st
    }
    #[doc = "0x108 - register description"]
    #[inline(always)]
    pub const fn pro_icache_reject_vaddr(&self) -> &PRO_ICACHE_REJECT_VADDR {
        &self.pro_icache_reject_vaddr
    }
    #[doc = "0x10c - register description"]
    #[inline(always)]
    pub const fn pro_cache_mmu_fault_content(&self) -> &PRO_CACHE_MMU_FAULT_CONTENT {
        &self.pro_cache_mmu_fault_content
    }
    #[doc = "0x110 - register description"]
    #[inline(always)]
    pub const fn pro_cache_mmu_fault_vaddr(&self) -> &PRO_CACHE_MMU_FAULT_VADDR {
        &self.pro_cache_mmu_fault_vaddr
    }
    #[doc = "0x114 - register description"]
    #[inline(always)]
    pub const fn pro_cache_wrap_around_ctrl(&self) -> &PRO_CACHE_WRAP_AROUND_CTRL {
        &self.pro_cache_wrap_around_ctrl
    }
    #[doc = "0x118 - register description"]
    #[inline(always)]
    pub const fn pro_cache_mmu_power_ctrl(&self) -> &PRO_CACHE_MMU_POWER_CTRL {
        &self.pro_cache_mmu_power_ctrl
    }
    #[doc = "0x11c - register description"]
    #[inline(always)]
    pub const fn pro_cache_state(&self) -> &PRO_CACHE_STATE {
        &self.pro_cache_state
    }
    #[doc = "0x120 - register description"]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_record_disable(
        &self,
    ) -> &CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {
        &self.cache_encrypt_decrypt_record_disable
    }
    #[doc = "0x124 - register description"]
    #[inline(always)]
    pub const fn cache_encrypt_decrypt_clk_force_on(&self) -> &CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {
        &self.cache_encrypt_decrypt_clk_force_on
    }
    #[doc = "0x128 - register description"]
    #[inline(always)]
    pub const fn cache_bridge_arbiter_ctrl(&self) -> &CACHE_BRIDGE_ARBITER_CTRL {
        &self.cache_bridge_arbiter_ctrl
    }
    #[doc = "0x12c - register description"]
    #[inline(always)]
    pub const fn cache_preload_int_ctrl(&self) -> &CACHE_PRELOAD_INT_CTRL {
        &self.cache_preload_int_ctrl
    }
    #[doc = "0x130 - register description"]
    #[inline(always)]
    pub const fn cache_sync_int_ctrl(&self) -> &CACHE_SYNC_INT_CTRL {
        &self.cache_sync_int_ctrl
    }
    #[doc = "0x134 - register description"]
    #[inline(always)]
    pub const fn cache_conf_misc(&self) -> &CACHE_CONF_MISC {
        &self.cache_conf_misc
    }
    #[doc = "0x138 - register description"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - register description"]
    #[inline(always)]
    pub const fn pro_extmem_reg_date(&self) -> &PRO_EXTMEM_REG_DATE {
        &self.pro_extmem_reg_date
    }
}
#[doc = "PRO_DCACHE_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_ctrl`] module"]
pub type PRO_DCACHE_CTRL = crate::Reg<pro_dcache_ctrl::PRO_DCACHE_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_ctrl;
#[doc = "PRO_DCACHE_CTRL1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_ctrl1`] module"]
pub type PRO_DCACHE_CTRL1 = crate::Reg<pro_dcache_ctrl1::PRO_DCACHE_CTRL1_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_ctrl1;
#[doc = "PRO_DCACHE_TAG_POWER_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_tag_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_tag_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_tag_power_ctrl`] module"]
pub type PRO_DCACHE_TAG_POWER_CTRL =
    crate::Reg<pro_dcache_tag_power_ctrl::PRO_DCACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_tag_power_ctrl;
#[doc = "PRO_DCACHE_LOCK0_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_lock0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_lock0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_lock0_addr`] module"]
pub type PRO_DCACHE_LOCK0_ADDR = crate::Reg<pro_dcache_lock0_addr::PRO_DCACHE_LOCK0_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_lock0_addr;
#[doc = "PRO_DCACHE_LOCK0_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_lock0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_lock0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_lock0_size`] module"]
pub type PRO_DCACHE_LOCK0_SIZE = crate::Reg<pro_dcache_lock0_size::PRO_DCACHE_LOCK0_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_lock0_size;
#[doc = "PRO_DCACHE_LOCK1_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_lock1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_lock1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_lock1_addr`] module"]
pub type PRO_DCACHE_LOCK1_ADDR = crate::Reg<pro_dcache_lock1_addr::PRO_DCACHE_LOCK1_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_lock1_addr;
#[doc = "PRO_DCACHE_LOCK1_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_lock1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_lock1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_lock1_size`] module"]
pub type PRO_DCACHE_LOCK1_SIZE = crate::Reg<pro_dcache_lock1_size::PRO_DCACHE_LOCK1_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_lock1_size;
#[doc = "PRO_DCACHE_MEM_SYNC0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_mem_sync0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_mem_sync0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_mem_sync0`] module"]
pub type PRO_DCACHE_MEM_SYNC0 = crate::Reg<pro_dcache_mem_sync0::PRO_DCACHE_MEM_SYNC0_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_mem_sync0;
#[doc = "PRO_DCACHE_MEM_SYNC1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_mem_sync1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_mem_sync1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_mem_sync1`] module"]
pub type PRO_DCACHE_MEM_SYNC1 = crate::Reg<pro_dcache_mem_sync1::PRO_DCACHE_MEM_SYNC1_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_mem_sync1;
#[doc = "PRO_DCACHE_PRELOAD_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_preload_addr`] module"]
pub type PRO_DCACHE_PRELOAD_ADDR =
    crate::Reg<pro_dcache_preload_addr::PRO_DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_preload_addr;
#[doc = "PRO_DCACHE_PRELOAD_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_preload_size`] module"]
pub type PRO_DCACHE_PRELOAD_SIZE =
    crate::Reg<pro_dcache_preload_size::PRO_DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_preload_size;
#[doc = "PRO_DCACHE_AUTOLOAD_CFG (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_autoload_cfg`] module"]
pub type PRO_DCACHE_AUTOLOAD_CFG =
    crate::Reg<pro_dcache_autoload_cfg::PRO_DCACHE_AUTOLOAD_CFG_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_autoload_cfg;
#[doc = "PRO_DCACHE_AUTOLOAD_SECTION0_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_section0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_section0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_autoload_section0_addr`] module"]
pub type PRO_DCACHE_AUTOLOAD_SECTION0_ADDR =
    crate::Reg<pro_dcache_autoload_section0_addr::PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_autoload_section0_addr;
#[doc = "PRO_DCACHE_AUTOLOAD_SECTION0_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_section0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_section0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_autoload_section0_size`] module"]
pub type PRO_DCACHE_AUTOLOAD_SECTION0_SIZE =
    crate::Reg<pro_dcache_autoload_section0_size::PRO_DCACHE_AUTOLOAD_SECTION0_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_autoload_section0_size;
#[doc = "PRO_DCACHE_AUTOLOAD_SECTION1_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_section1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_section1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_autoload_section1_addr`] module"]
pub type PRO_DCACHE_AUTOLOAD_SECTION1_ADDR =
    crate::Reg<pro_dcache_autoload_section1_addr::PRO_DCACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_autoload_section1_addr;
#[doc = "PRO_DCACHE_AUTOLOAD_SECTION1_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_section1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_section1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_autoload_section1_size`] module"]
pub type PRO_DCACHE_AUTOLOAD_SECTION1_SIZE =
    crate::Reg<pro_dcache_autoload_section1_size::PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_autoload_section1_size;
#[doc = "PRO_ICACHE_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_ctrl`] module"]
pub type PRO_ICACHE_CTRL = crate::Reg<pro_icache_ctrl::PRO_ICACHE_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_icache_ctrl;
#[doc = "PRO_ICACHE_CTRL1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_ctrl1`] module"]
pub type PRO_ICACHE_CTRL1 = crate::Reg<pro_icache_ctrl1::PRO_ICACHE_CTRL1_SPEC>;
#[doc = "register description"]
pub mod pro_icache_ctrl1;
#[doc = "PRO_ICACHE_TAG_POWER_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_tag_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_tag_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_tag_power_ctrl`] module"]
pub type PRO_ICACHE_TAG_POWER_CTRL =
    crate::Reg<pro_icache_tag_power_ctrl::PRO_ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_icache_tag_power_ctrl;
#[doc = "PRO_ICACHE_LOCK0_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_lock0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_lock0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_lock0_addr`] module"]
pub type PRO_ICACHE_LOCK0_ADDR = crate::Reg<pro_icache_lock0_addr::PRO_ICACHE_LOCK0_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_lock0_addr;
#[doc = "PRO_ICACHE_LOCK0_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_lock0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_lock0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_lock0_size`] module"]
pub type PRO_ICACHE_LOCK0_SIZE = crate::Reg<pro_icache_lock0_size::PRO_ICACHE_LOCK0_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_icache_lock0_size;
#[doc = "PRO_ICACHE_LOCK1_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_lock1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_lock1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_lock1_addr`] module"]
pub type PRO_ICACHE_LOCK1_ADDR = crate::Reg<pro_icache_lock1_addr::PRO_ICACHE_LOCK1_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_lock1_addr;
#[doc = "PRO_ICACHE_LOCK1_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_lock1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_lock1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_lock1_size`] module"]
pub type PRO_ICACHE_LOCK1_SIZE = crate::Reg<pro_icache_lock1_size::PRO_ICACHE_LOCK1_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_icache_lock1_size;
#[doc = "PRO_ICACHE_MEM_SYNC0 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_mem_sync0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_mem_sync0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_mem_sync0`] module"]
pub type PRO_ICACHE_MEM_SYNC0 = crate::Reg<pro_icache_mem_sync0::PRO_ICACHE_MEM_SYNC0_SPEC>;
#[doc = "register description"]
pub mod pro_icache_mem_sync0;
#[doc = "PRO_ICACHE_MEM_SYNC1 (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_mem_sync1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_mem_sync1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_mem_sync1`] module"]
pub type PRO_ICACHE_MEM_SYNC1 = crate::Reg<pro_icache_mem_sync1::PRO_ICACHE_MEM_SYNC1_SPEC>;
#[doc = "register description"]
pub mod pro_icache_mem_sync1;
#[doc = "PRO_ICACHE_PRELOAD_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_preload_addr`] module"]
pub type PRO_ICACHE_PRELOAD_ADDR =
    crate::Reg<pro_icache_preload_addr::PRO_ICACHE_PRELOAD_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_preload_addr;
#[doc = "PRO_ICACHE_PRELOAD_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_preload_size`] module"]
pub type PRO_ICACHE_PRELOAD_SIZE =
    crate::Reg<pro_icache_preload_size::PRO_ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_icache_preload_size;
#[doc = "PRO_ICACHE_AUTOLOAD_CFG (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_autoload_cfg`] module"]
pub type PRO_ICACHE_AUTOLOAD_CFG =
    crate::Reg<pro_icache_autoload_cfg::PRO_ICACHE_AUTOLOAD_CFG_SPEC>;
#[doc = "register description"]
pub mod pro_icache_autoload_cfg;
#[doc = "PRO_ICACHE_AUTOLOAD_SECTION0_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_section0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_section0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_autoload_section0_addr`] module"]
pub type PRO_ICACHE_AUTOLOAD_SECTION0_ADDR =
    crate::Reg<pro_icache_autoload_section0_addr::PRO_ICACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_autoload_section0_addr;
#[doc = "PRO_ICACHE_AUTOLOAD_SECTION0_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_section0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_section0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_autoload_section0_size`] module"]
pub type PRO_ICACHE_AUTOLOAD_SECTION0_SIZE =
    crate::Reg<pro_icache_autoload_section0_size::PRO_ICACHE_AUTOLOAD_SECTION0_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_icache_autoload_section0_size;
#[doc = "PRO_ICACHE_AUTOLOAD_SECTION1_ADDR (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_section1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_section1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_autoload_section1_addr`] module"]
pub type PRO_ICACHE_AUTOLOAD_SECTION1_ADDR =
    crate::Reg<pro_icache_autoload_section1_addr::PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_autoload_section1_addr;
#[doc = "PRO_ICACHE_AUTOLOAD_SECTION1_SIZE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_section1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_section1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_autoload_section1_size`] module"]
pub type PRO_ICACHE_AUTOLOAD_SECTION1_SIZE =
    crate::Reg<pro_icache_autoload_section1_size::PRO_ICACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
#[doc = "register description"]
pub mod pro_icache_autoload_section1_size;
#[doc = "IC_PRELOAD_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_preload_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic_preload_cnt`] module"]
pub type IC_PRELOAD_CNT = crate::Reg<ic_preload_cnt::IC_PRELOAD_CNT_SPEC>;
#[doc = "register description"]
pub mod ic_preload_cnt;
#[doc = "IC_PRELOAD_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_preload_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic_preload_miss_cnt`] module"]
pub type IC_PRELOAD_MISS_CNT = crate::Reg<ic_preload_miss_cnt::IC_PRELOAD_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod ic_preload_miss_cnt;
#[doc = "IBUS2_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_abandon_cnt`] module"]
pub type IBUS2_ABANDON_CNT = crate::Reg<ibus2_abandon_cnt::IBUS2_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus2_abandon_cnt;
#[doc = "IBUS1_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_abandon_cnt`] module"]
pub type IBUS1_ABANDON_CNT = crate::Reg<ibus1_abandon_cnt::IBUS1_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus1_abandon_cnt;
#[doc = "IBUS0_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_abandon_cnt`] module"]
pub type IBUS0_ABANDON_CNT = crate::Reg<ibus0_abandon_cnt::IBUS0_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus0_abandon_cnt;
#[doc = "IBUS2_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_miss_cnt`] module"]
pub type IBUS2_ACS_MISS_CNT = crate::Reg<ibus2_acs_miss_cnt::IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus2_acs_miss_cnt;
#[doc = "IBUS1_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_miss_cnt`] module"]
pub type IBUS1_ACS_MISS_CNT = crate::Reg<ibus1_acs_miss_cnt::IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus1_acs_miss_cnt;
#[doc = "IBUS0_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_miss_cnt`] module"]
pub type IBUS0_ACS_MISS_CNT = crate::Reg<ibus0_acs_miss_cnt::IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus0_acs_miss_cnt;
#[doc = "IBUS2_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_cnt`] module"]
pub type IBUS2_ACS_CNT = crate::Reg<ibus2_acs_cnt::IBUS2_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus2_acs_cnt;
#[doc = "IBUS1_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_cnt`] module"]
pub type IBUS1_ACS_CNT = crate::Reg<ibus1_acs_cnt::IBUS1_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus1_acs_cnt;
#[doc = "IBUS0_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_cnt`] module"]
pub type IBUS0_ACS_CNT = crate::Reg<ibus0_acs_cnt::IBUS0_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod ibus0_acs_cnt;
#[doc = "DC_PRELOAD_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_preload_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_preload_cnt`] module"]
pub type DC_PRELOAD_CNT = crate::Reg<dc_preload_cnt::DC_PRELOAD_CNT_SPEC>;
#[doc = "register description"]
pub mod dc_preload_cnt;
#[doc = "DC_PRELOAD_EVICT_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_preload_evict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_preload_evict_cnt`] module"]
pub type DC_PRELOAD_EVICT_CNT = crate::Reg<dc_preload_evict_cnt::DC_PRELOAD_EVICT_CNT_SPEC>;
#[doc = "register description"]
pub mod dc_preload_evict_cnt;
#[doc = "DC_PRELOAD_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_preload_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_preload_miss_cnt`] module"]
pub type DC_PRELOAD_MISS_CNT = crate::Reg<dc_preload_miss_cnt::DC_PRELOAD_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod dc_preload_miss_cnt;
#[doc = "DBUS2_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_abandon_cnt`] module"]
pub type DBUS2_ABANDON_CNT = crate::Reg<dbus2_abandon_cnt::DBUS2_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus2_abandon_cnt;
#[doc = "DBUS1_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_abandon_cnt`] module"]
pub type DBUS1_ABANDON_CNT = crate::Reg<dbus1_abandon_cnt::DBUS1_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus1_abandon_cnt;
#[doc = "DBUS0_ABANDON_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_abandon_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_abandon_cnt`] module"]
pub type DBUS0_ABANDON_CNT = crate::Reg<dbus0_abandon_cnt::DBUS0_ABANDON_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus0_abandon_cnt;
#[doc = "DBUS2_ACS_WB_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_wb_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_wb_cnt`] module"]
pub type DBUS2_ACS_WB_CNT = crate::Reg<dbus2_acs_wb_cnt::DBUS2_ACS_WB_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus2_acs_wb_cnt;
#[doc = "DBUS1_ACS_WB_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_wb_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_wb_cnt`] module"]
pub type DBUS1_ACS_WB_CNT = crate::Reg<dbus1_acs_wb_cnt::DBUS1_ACS_WB_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus1_acs_wb_cnt;
#[doc = "DBUS0_ACS_WB_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_wb_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_wb_cnt`] module"]
pub type DBUS0_ACS_WB_CNT = crate::Reg<dbus0_acs_wb_cnt::DBUS0_ACS_WB_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus0_acs_wb_cnt;
#[doc = "DBUS2_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_miss_cnt`] module"]
pub type DBUS2_ACS_MISS_CNT = crate::Reg<dbus2_acs_miss_cnt::DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus2_acs_miss_cnt;
#[doc = "DBUS1_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_miss_cnt`] module"]
pub type DBUS1_ACS_MISS_CNT = crate::Reg<dbus1_acs_miss_cnt::DBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus1_acs_miss_cnt;
#[doc = "DBUS0_ACS_MISS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_miss_cnt`] module"]
pub type DBUS0_ACS_MISS_CNT = crate::Reg<dbus0_acs_miss_cnt::DBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus0_acs_miss_cnt;
#[doc = "DBUS2_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_cnt`] module"]
pub type DBUS2_ACS_CNT = crate::Reg<dbus2_acs_cnt::DBUS2_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus2_acs_cnt;
#[doc = "DBUS1_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_cnt`] module"]
pub type DBUS1_ACS_CNT = crate::Reg<dbus1_acs_cnt::DBUS1_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus1_acs_cnt;
#[doc = "DBUS0_ACS_CNT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_cnt`] module"]
pub type DBUS0_ACS_CNT = crate::Reg<dbus0_acs_cnt::DBUS0_ACS_CNT_SPEC>;
#[doc = "register description"]
pub mod dbus0_acs_cnt;
#[doc = "CACHE_DBG_INT_ENA (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_dbg_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_dbg_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dbg_int_ena`] module"]
pub type CACHE_DBG_INT_ENA = crate::Reg<cache_dbg_int_ena::CACHE_DBG_INT_ENA_SPEC>;
#[doc = "register description"]
pub mod cache_dbg_int_ena;
#[doc = "CACHE_DBG_INT_CLR (w) register accessor: register description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_dbg_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dbg_int_clr`] module"]
pub type CACHE_DBG_INT_CLR = crate::Reg<cache_dbg_int_clr::CACHE_DBG_INT_CLR_SPEC>;
#[doc = "register description"]
pub mod cache_dbg_int_clr;
#[doc = "CACHE_DBG_STATUS0 (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_dbg_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dbg_status0`] module"]
pub type CACHE_DBG_STATUS0 = crate::Reg<cache_dbg_status0::CACHE_DBG_STATUS0_SPEC>;
#[doc = "register description"]
pub mod cache_dbg_status0;
#[doc = "CACHE_DBG_STATUS1 (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_dbg_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_dbg_status1`] module"]
pub type CACHE_DBG_STATUS1 = crate::Reg<cache_dbg_status1::CACHE_DBG_STATUS1_SPEC>;
#[doc = "register description"]
pub mod cache_dbg_status1;
#[doc = "PRO_CACHE_ACS_CNT_CLR (w) register accessor: register description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cache_acs_cnt_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_acs_cnt_clr`] module"]
pub type PRO_CACHE_ACS_CNT_CLR = crate::Reg<pro_cache_acs_cnt_clr::PRO_CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "register description"]
pub mod pro_cache_acs_cnt_clr;
#[doc = "PRO_DCACHE_REJECT_ST (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_reject_st`] module"]
pub type PRO_DCACHE_REJECT_ST = crate::Reg<pro_dcache_reject_st::PRO_DCACHE_REJECT_ST_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_reject_st;
#[doc = "PRO_DCACHE_REJECT_VADDR (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_dcache_reject_vaddr`] module"]
pub type PRO_DCACHE_REJECT_VADDR =
    crate::Reg<pro_dcache_reject_vaddr::PRO_DCACHE_REJECT_VADDR_SPEC>;
#[doc = "register description"]
pub mod pro_dcache_reject_vaddr;
#[doc = "PRO_ICACHE_REJECT_ST (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_reject_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_reject_st`] module"]
pub type PRO_ICACHE_REJECT_ST = crate::Reg<pro_icache_reject_st::PRO_ICACHE_REJECT_ST_SPEC>;
#[doc = "register description"]
pub mod pro_icache_reject_st;
#[doc = "PRO_ICACHE_REJECT_VADDR (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_reject_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_icache_reject_vaddr`] module"]
pub type PRO_ICACHE_REJECT_VADDR =
    crate::Reg<pro_icache_reject_vaddr::PRO_ICACHE_REJECT_VADDR_SPEC>;
#[doc = "register description"]
pub mod pro_icache_reject_vaddr;
#[doc = "PRO_CACHE_MMU_FAULT_CONTENT (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_mmu_fault_content::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_mmu_fault_content`] module"]
pub type PRO_CACHE_MMU_FAULT_CONTENT =
    crate::Reg<pro_cache_mmu_fault_content::PRO_CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "register description"]
pub mod pro_cache_mmu_fault_content;
#[doc = "PRO_CACHE_MMU_FAULT_VADDR (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_mmu_fault_vaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_mmu_fault_vaddr`] module"]
pub type PRO_CACHE_MMU_FAULT_VADDR =
    crate::Reg<pro_cache_mmu_fault_vaddr::PRO_CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "register description"]
pub mod pro_cache_mmu_fault_vaddr;
#[doc = "PRO_CACHE_WRAP_AROUND_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_wrap_around_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_wrap_around_ctrl`] module"]
pub type PRO_CACHE_WRAP_AROUND_CTRL =
    crate::Reg<pro_cache_wrap_around_ctrl::PRO_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_cache_wrap_around_ctrl;
#[doc = "PRO_CACHE_MMU_POWER_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cache_mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_mmu_power_ctrl`] module"]
pub type PRO_CACHE_MMU_POWER_CTRL =
    crate::Reg<pro_cache_mmu_power_ctrl::PRO_CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "register description"]
pub mod pro_cache_mmu_power_ctrl;
#[doc = "PRO_CACHE_STATE (r) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pro_cache_state`] module"]
pub type PRO_CACHE_STATE = crate::Reg<pro_cache_state::PRO_CACHE_STATE_SPEC>;
#[doc = "register description"]
pub mod pro_cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_encrypt_decrypt_record_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_record_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_record_disable`] module"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "register description"]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_encrypt_decrypt_clk_force_on::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_encrypt_decrypt_clk_force_on::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_encrypt_decrypt_clk_force_on`] module"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "register description"]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_BRIDGE_ARBITER_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_bridge_arbiter_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_bridge_arbiter_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_bridge_arbiter_ctrl`] module"]
pub type CACHE_BRIDGE_ARBITER_CTRL =
    crate::Reg<cache_bridge_arbiter_ctrl::CACHE_BRIDGE_ARBITER_CTRL_SPEC>;
#[doc = "register description"]
pub mod cache_bridge_arbiter_ctrl;
#[doc = "CACHE_PRELOAD_INT_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_int_ctrl`] module"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "register description"]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_int_ctrl`] module"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "register description"]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_CONF_MISC (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_conf_misc`] module"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "register description"]
pub mod cache_conf_misc;
#[doc = "CLOCK_GATE (rw) register accessor: register description\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
pub use crate::aes::date as pro_extmem_reg_date;
pub use crate::aes::DATE as PRO_EXTMEM_REG_DATE;
