#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    icache_ctrl: ICACHE_CTRL,
    cache_ctrl: CACHE_CTRL,
    _reserved2: [u8; 0x08],
    icache_cachesize_conf: ICACHE_CACHESIZE_CONF,
    icache_blocksize_conf: ICACHE_BLOCKSIZE_CONF,
    cache_cachesize_conf: CACHE_CACHESIZE_CONF,
    cache_blocksize_conf: CACHE_BLOCKSIZE_CONF,
    cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    cache_miss_access_ctrl: CACHE_MISS_ACCESS_CTRL,
    cache_freeze_ctrl: CACHE_FREEZE_CTRL,
    cache_data_mem_acs_conf: CACHE_DATA_MEM_ACS_CONF,
    cache_tag_mem_acs_conf: CACHE_TAG_MEM_ACS_CONF,
    _reserved11: [u8; 0x20],
    icache2_prelock_conf: ICACHE2_PRELOCK_CONF,
    icache2_prelock_sct0_addr: ICACHE2_PRELOCK_SCT0_ADDR,
    icache2_prelock_sct1_addr: ICACHE2_PRELOCK_SCT1_ADDR,
    icache2_prelock_sct_size: ICACHE2_PRELOCK_SCT_SIZE,
    _reserved15: [u8; 0x10],
    cache_prelock_conf: CACHE_PRELOCK_CONF,
    cache_prelock_sct0_addr: CACHE_PRELOCK_SCT0_ADDR,
    dcache_prelock_sct1_addr: DCACHE_PRELOCK_SCT1_ADDR,
    dcache_prelock_sct_size: DCACHE_PRELOCK_SCT_SIZE,
    lock_ctrl: LOCK_CTRL,
    lock_map: LOCK_MAP,
    lock_addr: LOCK_ADDR,
    lock_size: LOCK_SIZE,
    sync_ctrl: SYNC_CTRL,
    sync_map: SYNC_MAP,
    sync_addr: SYNC_ADDR,
    sync_size: SYNC_SIZE,
    _reserved27: [u8; 0x18],
    icache2_preload_ctrl: ICACHE2_PRELOAD_CTRL,
    icache2_preload_addr: ICACHE2_PRELOAD_ADDR,
    icache2_preload_size: ICACHE2_PRELOAD_SIZE,
    _reserved30: [u8; 0x0c],
    cache_preload_ctrl: CACHE_PRELOAD_CTRL,
    dcache_preload_addr: DCACHE_PRELOAD_ADDR,
    dcache_preload_size: DCACHE_PRELOAD_SIZE,
    _reserved33: [u8; 0x28],
    icache2_autoload_ctrl: ICACHE2_AUTOLOAD_CTRL,
    icache2_autoload_sct0_addr: ICACHE2_AUTOLOAD_SCT0_ADDR,
    icache2_autoload_sct0_size: ICACHE2_AUTOLOAD_SCT0_SIZE,
    icache2_autoload_sct1_addr: ICACHE2_AUTOLOAD_SCT1_ADDR,
    icache2_autoload_sct1_size: ICACHE2_AUTOLOAD_SCT1_SIZE,
    _reserved38: [u8; 0x14],
    cache_autoload_ctrl: CACHE_AUTOLOAD_CTRL,
    cache_autoload_sct0_addr: CACHE_AUTOLOAD_SCT0_ADDR,
    cache_autoload_sct0_size: CACHE_AUTOLOAD_SCT0_SIZE,
    cache_autoload_sct1_addr: CACHE_AUTOLOAD_SCT1_ADDR,
    cache_autoload_sct1_size: CACHE_AUTOLOAD_SCT1_SIZE,
    _reserved43: [u8; 0x10],
    cache_acs_cnt_int_ena: CACHE_ACS_CNT_INT_ENA,
    cache_acs_cnt_int_clr: CACHE_ACS_CNT_INT_CLR,
    cache_acs_cnt_int_raw: CACHE_ACS_CNT_INT_RAW,
    cache_acs_cnt_int_st: CACHE_ACS_CNT_INT_ST,
    cache_acs_fail_ctrl: CACHE_ACS_FAIL_CTRL,
    cache_acs_fail_int_ena: CACHE_ACS_FAIL_INT_ENA,
    cache_acs_fail_int_clr: CACHE_ACS_FAIL_INT_CLR,
    cache_acs_fail_int_raw: CACHE_ACS_FAIL_INT_RAW,
    cache_acs_fail_int_st: CACHE_ACS_FAIL_INT_ST,
    cache_acs_cnt_ctrl: CACHE_ACS_CNT_CTRL,
    _reserved53: [u8; 0x20],
    ibus2_acs_hit_cnt: IBUS2_ACS_HIT_CNT,
    ibus2_acs_miss_cnt: IBUS2_ACS_MISS_CNT,
    ibus2_acs_conflict_cnt: IBUS2_ACS_CONFLICT_CNT,
    ibus2_acs_nxtlvl_rd_cnt: IBUS2_ACS_NXTLVL_RD_CNT,
    ibus3_acs_hit_cnt: IBUS3_ACS_HIT_CNT,
    ibus3_acs_miss_cnt: IBUS3_ACS_MISS_CNT,
    ibus3_acs_conflict_cnt: IBUS3_ACS_CONFLICT_CNT,
    ibus3_acs_nxtlvl_rd_cnt: IBUS3_ACS_NXTLVL_RD_CNT,
    bus0_acs_hit_cnt: BUS0_ACS_HIT_CNT,
    bus0_acs_miss_cnt: BUS0_ACS_MISS_CNT,
    bus0_acs_conflict_cnt: BUS0_ACS_CONFLICT_CNT,
    dbus0_acs_nxtlvl_rd_cnt: DBUS0_ACS_NXTLVL_RD_CNT,
    dbus0_acs_nxtlvl_wr_cnt: DBUS0_ACS_NXTLVL_WR_CNT,
    bus1_acs_hit_cnt: BUS1_ACS_HIT_CNT,
    bus1_acs_miss_cnt: BUS1_ACS_MISS_CNT,
    bus1_acs_conflict_cnt: BUS1_ACS_CONFLICT_CNT,
    dbus1_acs_nxtlvl_rd_cnt: DBUS1_ACS_NXTLVL_RD_CNT,
    dbus1_acs_nxtlvl_wr_cnt: DBUS1_ACS_NXTLVL_WR_CNT,
    _reserved71: [u8; 0x38],
    icache2_acs_fail_id_attr: ICACHE2_ACS_FAIL_ID_ATTR,
    icache2_acs_fail_addr: ICACHE2_ACS_FAIL_ADDR,
    _reserved73: [u8; 0x08],
    dcache_acs_fail_id_attr: DCACHE_ACS_FAIL_ID_ATTR,
    dcache_acs_fail_addr: DCACHE_ACS_FAIL_ADDR,
    cache_sync_preload_int_ena: CACHE_SYNC_PRELOAD_INT_ENA,
    cache_sync_preload_int_clr: CACHE_SYNC_PRELOAD_INT_CLR,
    cache_sync_preload_int_raw: CACHE_SYNC_PRELOAD_INT_RAW,
    cache_sync_preload_int_st: CACHE_SYNC_PRELOAD_INT_ST,
    cache_sync_preload_exception: CACHE_SYNC_PRELOAD_EXCEPTION,
    cache_sync_rst_ctrl: CACHE_SYNC_RST_CTRL,
    cache_preload_rst_ctrl: CACHE_PRELOAD_RST_CTRL,
    cache_autoload_buf_clr_ctrl: CACHE_AUTOLOAD_BUF_CLR_CTRL,
    unallocate_buffer_clear: UNALLOCATE_BUFFER_CLEAR,
    cache_object_ctrl: CACHE_OBJECT_CTRL,
    cache_way_object: CACHE_WAY_OBJECT,
    cache_addr: CACHE_ADDR,
    cache_debug_bus: CACHE_DEBUG_BUS,
    _reserved88: [u8; 0x0168],
    trace_ena: TRACE_ENA,
    _reserved89: [u8; 0x28],
    cache_date: CACHE_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Instruction cache (ICache) control register"]
    #[inline(always)]
    pub const fn icache_ctrl(&self) -> &ICACHE_CTRL {
        &self.icache_ctrl
    }
    #[doc = "0x04 - Data cache control register"]
    #[inline(always)]
    pub const fn cache_ctrl(&self) -> &CACHE_CTRL {
        &self.cache_ctrl
    }
    #[doc = "0x10 - Instruction Cache CacheSize mode configure register"]
    #[inline(always)]
    pub const fn icache_cachesize_conf(&self) -> &ICACHE_CACHESIZE_CONF {
        &self.icache_cachesize_conf
    }
    #[doc = "0x14 - Instruction Cache BlockSize mode configure register"]
    #[inline(always)]
    pub const fn icache_blocksize_conf(&self) -> &ICACHE_BLOCKSIZE_CONF {
        &self.icache_blocksize_conf
    }
    #[doc = "0x18 - Data Cache CacheSize mode configure register"]
    #[inline(always)]
    pub const fn cache_cachesize_conf(&self) -> &CACHE_CACHESIZE_CONF {
        &self.cache_cachesize_conf
    }
    #[doc = "0x1c - Data Cache BlockSize mode configure register"]
    #[inline(always)]
    pub const fn cache_blocksize_conf(&self) -> &CACHE_BLOCKSIZE_CONF {
        &self.cache_blocksize_conf
    }
    #[doc = "0x20 - Cache wrap around control register"]
    #[inline(always)]
    pub const fn cache_wrap_around_ctrl(&self) -> &CACHE_WRAP_AROUND_CTRL {
        &self.cache_wrap_around_ctrl
    }
    #[doc = "0x24 - Cache wrap around control register"]
    #[inline(always)]
    pub const fn cache_miss_access_ctrl(&self) -> &CACHE_MISS_ACCESS_CTRL {
        &self.cache_miss_access_ctrl
    }
    #[doc = "0x28 - Cache Freeze control register"]
    #[inline(always)]
    pub const fn cache_freeze_ctrl(&self) -> &CACHE_FREEZE_CTRL {
        &self.cache_freeze_ctrl
    }
    #[doc = "0x2c - Cache data memory access configure register"]
    #[inline(always)]
    pub const fn cache_data_mem_acs_conf(&self) -> &CACHE_DATA_MEM_ACS_CONF {
        &self.cache_data_mem_acs_conf
    }
    #[doc = "0x30 - Cache tag memory access configure register"]
    #[inline(always)]
    pub const fn cache_tag_mem_acs_conf(&self) -> &CACHE_TAG_MEM_ACS_CONF {
        &self.cache_tag_mem_acs_conf
    }
    #[doc = "0x54 - Instruction Cache 2 prelock configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_conf(&self) -> &ICACHE2_PRELOCK_CONF {
        &self.icache2_prelock_conf
    }
    #[doc = "0x58 - Instruction Cache 2 prelock section0 address configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct0_addr(&self) -> &ICACHE2_PRELOCK_SCT0_ADDR {
        &self.icache2_prelock_sct0_addr
    }
    #[doc = "0x5c - Instruction Cache 2 prelock section1 address configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct1_addr(&self) -> &ICACHE2_PRELOCK_SCT1_ADDR {
        &self.icache2_prelock_sct1_addr
    }
    #[doc = "0x60 - Instruction Cache 2 prelock section size configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct_size(&self) -> &ICACHE2_PRELOCK_SCT_SIZE {
        &self.icache2_prelock_sct_size
    }
    #[doc = "0x74 - Cache prelock configure register"]
    #[inline(always)]
    pub const fn cache_prelock_conf(&self) -> &CACHE_PRELOCK_CONF {
        &self.cache_prelock_conf
    }
    #[doc = "0x78 - Cache prelock section0 address configure register"]
    #[inline(always)]
    pub const fn cache_prelock_sct0_addr(&self) -> &CACHE_PRELOCK_SCT0_ADDR {
        &self.cache_prelock_sct0_addr
    }
    #[doc = "0x7c - Cache prelock section1 address configure register"]
    #[inline(always)]
    pub const fn dcache_prelock_sct1_addr(&self) -> &DCACHE_PRELOCK_SCT1_ADDR {
        &self.dcache_prelock_sct1_addr
    }
    #[doc = "0x80 - Cache prelock section size configure register"]
    #[inline(always)]
    pub const fn dcache_prelock_sct_size(&self) -> &DCACHE_PRELOCK_SCT_SIZE {
        &self.dcache_prelock_sct_size
    }
    #[doc = "0x84 - Lock-class (manual lock) operation control register"]
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> &LOCK_CTRL {
        &self.lock_ctrl
    }
    #[doc = "0x88 - Lock (manual lock) map configure register"]
    #[inline(always)]
    pub const fn lock_map(&self) -> &LOCK_MAP {
        &self.lock_map
    }
    #[doc = "0x8c - Lock (manual lock) address configure register"]
    #[inline(always)]
    pub const fn lock_addr(&self) -> &LOCK_ADDR {
        &self.lock_addr
    }
    #[doc = "0x90 - Lock (manual lock) size configure register"]
    #[inline(always)]
    pub const fn lock_size(&self) -> &LOCK_SIZE {
        &self.lock_size
    }
    #[doc = "0x94 - Sync-class operation control register"]
    #[inline(always)]
    pub const fn sync_ctrl(&self) -> &SYNC_CTRL {
        &self.sync_ctrl
    }
    #[doc = "0x98 - Sync map configure register"]
    #[inline(always)]
    pub const fn sync_map(&self) -> &SYNC_MAP {
        &self.sync_map
    }
    #[doc = "0x9c - Sync address configure register"]
    #[inline(always)]
    pub const fn sync_addr(&self) -> &SYNC_ADDR {
        &self.sync_addr
    }
    #[doc = "0xa0 - Sync size configure register"]
    #[inline(always)]
    pub const fn sync_size(&self) -> &SYNC_SIZE {
        &self.sync_size
    }
    #[doc = "0xbc - Instruction Cache 2 preload-operation control register"]
    #[inline(always)]
    pub const fn icache2_preload_ctrl(&self) -> &ICACHE2_PRELOAD_CTRL {
        &self.icache2_preload_ctrl
    }
    #[doc = "0xc0 - Instruction Cache 2 preload address configure register"]
    #[inline(always)]
    pub const fn icache2_preload_addr(&self) -> &ICACHE2_PRELOAD_ADDR {
        &self.icache2_preload_addr
    }
    #[doc = "0xc4 - Instruction Cache 2 preload size configure register"]
    #[inline(always)]
    pub const fn icache2_preload_size(&self) -> &ICACHE2_PRELOAD_SIZE {
        &self.icache2_preload_size
    }
    #[doc = "0xd4 - Cache preload-operation control register"]
    #[inline(always)]
    pub const fn cache_preload_ctrl(&self) -> &CACHE_PRELOAD_CTRL {
        &self.cache_preload_ctrl
    }
    #[doc = "0xd8 - Cache preload address configure register"]
    #[inline(always)]
    pub const fn dcache_preload_addr(&self) -> &DCACHE_PRELOAD_ADDR {
        &self.dcache_preload_addr
    }
    #[doc = "0xdc - Cache preload size configure register"]
    #[inline(always)]
    pub const fn dcache_preload_size(&self) -> &DCACHE_PRELOAD_SIZE {
        &self.dcache_preload_size
    }
    #[doc = "0x108 - Instruction Cache 2 autoload-operation control register"]
    #[inline(always)]
    pub const fn icache2_autoload_ctrl(&self) -> &ICACHE2_AUTOLOAD_CTRL {
        &self.icache2_autoload_ctrl
    }
    #[doc = "0x10c - Instruction Cache 2 autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct0_addr(&self) -> &ICACHE2_AUTOLOAD_SCT0_ADDR {
        &self.icache2_autoload_sct0_addr
    }
    #[doc = "0x110 - Instruction Cache 2 autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct0_size(&self) -> &ICACHE2_AUTOLOAD_SCT0_SIZE {
        &self.icache2_autoload_sct0_size
    }
    #[doc = "0x114 - Instruction Cache 2 autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct1_addr(&self) -> &ICACHE2_AUTOLOAD_SCT1_ADDR {
        &self.icache2_autoload_sct1_addr
    }
    #[doc = "0x118 - Instruction Cache 2 autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct1_size(&self) -> &ICACHE2_AUTOLOAD_SCT1_SIZE {
        &self.icache2_autoload_sct1_size
    }
    #[doc = "0x130 - Cache autoload-operation control register"]
    #[inline(always)]
    pub const fn cache_autoload_ctrl(&self) -> &CACHE_AUTOLOAD_CTRL {
        &self.cache_autoload_ctrl
    }
    #[doc = "0x134 - Cache autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct0_addr(&self) -> &CACHE_AUTOLOAD_SCT0_ADDR {
        &self.cache_autoload_sct0_addr
    }
    #[doc = "0x138 - Cache autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct0_size(&self) -> &CACHE_AUTOLOAD_SCT0_SIZE {
        &self.cache_autoload_sct0_size
    }
    #[doc = "0x13c - Cache autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct1_addr(&self) -> &CACHE_AUTOLOAD_SCT1_ADDR {
        &self.cache_autoload_sct1_addr
    }
    #[doc = "0x140 - Cache autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct1_size(&self) -> &CACHE_AUTOLOAD_SCT1_SIZE {
        &self.cache_autoload_sct1_size
    }
    #[doc = "0x154 - Cache Access Counter Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_acs_cnt_int_ena(&self) -> &CACHE_ACS_CNT_INT_ENA {
        &self.cache_acs_cnt_int_ena
    }
    #[doc = "0x158 - Cache Access Counter Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_acs_cnt_int_clr(&self) -> &CACHE_ACS_CNT_INT_CLR {
        &self.cache_acs_cnt_int_clr
    }
    #[doc = "0x15c - Cache Access Counter Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_acs_cnt_int_raw(&self) -> &CACHE_ACS_CNT_INT_RAW {
        &self.cache_acs_cnt_int_raw
    }
    #[doc = "0x160 - Cache Access Counter Interrupt status register"]
    #[inline(always)]
    pub const fn cache_acs_cnt_int_st(&self) -> &CACHE_ACS_CNT_INT_ST {
        &self.cache_acs_cnt_int_st
    }
    #[doc = "0x164 - Cache Access Fail Configuration register"]
    #[inline(always)]
    pub const fn cache_acs_fail_ctrl(&self) -> &CACHE_ACS_FAIL_CTRL {
        &self.cache_acs_fail_ctrl
    }
    #[doc = "0x168 - Cache Access Fail Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_acs_fail_int_ena(&self) -> &CACHE_ACS_FAIL_INT_ENA {
        &self.cache_acs_fail_int_ena
    }
    #[doc = "0x16c - Cache Access Fail Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_acs_fail_int_clr(&self) -> &CACHE_ACS_FAIL_INT_CLR {
        &self.cache_acs_fail_int_clr
    }
    #[doc = "0x170 - Cache Access Fail Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_acs_fail_int_raw(&self) -> &CACHE_ACS_FAIL_INT_RAW {
        &self.cache_acs_fail_int_raw
    }
    #[doc = "0x174 - Cache Access Fail Interrupt status register"]
    #[inline(always)]
    pub const fn cache_acs_fail_int_st(&self) -> &CACHE_ACS_FAIL_INT_ST {
        &self.cache_acs_fail_int_st
    }
    #[doc = "0x178 - Cache Access Counter enable and clear register"]
    #[inline(always)]
    pub const fn cache_acs_cnt_ctrl(&self) -> &CACHE_ACS_CNT_CTRL {
        &self.cache_acs_cnt_ctrl
    }
    #[doc = "0x19c - ICache bus2 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_hit_cnt(&self) -> &IBUS2_ACS_HIT_CNT {
        &self.ibus2_acs_hit_cnt
    }
    #[doc = "0x1a0 - ICache bus2 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_miss_cnt(&self) -> &IBUS2_ACS_MISS_CNT {
        &self.ibus2_acs_miss_cnt
    }
    #[doc = "0x1a4 - ICache bus2 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_conflict_cnt(&self) -> &IBUS2_ACS_CONFLICT_CNT {
        &self.ibus2_acs_conflict_cnt
    }
    #[doc = "0x1a8 - ICache bus2 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_nxtlvl_rd_cnt(&self) -> &IBUS2_ACS_NXTLVL_RD_CNT {
        &self.ibus2_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1ac - ICache bus3 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_hit_cnt(&self) -> &IBUS3_ACS_HIT_CNT {
        &self.ibus3_acs_hit_cnt
    }
    #[doc = "0x1b0 - ICache bus3 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_miss_cnt(&self) -> &IBUS3_ACS_MISS_CNT {
        &self.ibus3_acs_miss_cnt
    }
    #[doc = "0x1b4 - ICache bus3 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_conflict_cnt(&self) -> &IBUS3_ACS_CONFLICT_CNT {
        &self.ibus3_acs_conflict_cnt
    }
    #[doc = "0x1b8 - ICache bus3 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_nxtlvl_rd_cnt(&self) -> &IBUS3_ACS_NXTLVL_RD_CNT {
        &self.ibus3_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1bc - Cache bus0 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_hit_cnt(&self) -> &BUS0_ACS_HIT_CNT {
        &self.bus0_acs_hit_cnt
    }
    #[doc = "0x1c0 - Cache bus0 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_miss_cnt(&self) -> &BUS0_ACS_MISS_CNT {
        &self.bus0_acs_miss_cnt
    }
    #[doc = "0x1c4 - Cache bus0 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_conflict_cnt(&self) -> &BUS0_ACS_CONFLICT_CNT {
        &self.bus0_acs_conflict_cnt
    }
    #[doc = "0x1c8 - Cache bus0 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus0_acs_nxtlvl_rd_cnt(&self) -> &DBUS0_ACS_NXTLVL_RD_CNT {
        &self.dbus0_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1cc - DCache bus0 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus0_acs_nxtlvl_wr_cnt(&self) -> &DBUS0_ACS_NXTLVL_WR_CNT {
        &self.dbus0_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x1d0 - Cache bus1 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_hit_cnt(&self) -> &BUS1_ACS_HIT_CNT {
        &self.bus1_acs_hit_cnt
    }
    #[doc = "0x1d4 - Cache bus1 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_miss_cnt(&self) -> &BUS1_ACS_MISS_CNT {
        &self.bus1_acs_miss_cnt
    }
    #[doc = "0x1d8 - Cache bus1 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_conflict_cnt(&self) -> &BUS1_ACS_CONFLICT_CNT {
        &self.bus1_acs_conflict_cnt
    }
    #[doc = "0x1dc - DCache bus1 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus1_acs_nxtlvl_rd_cnt(&self) -> &DBUS1_ACS_NXTLVL_RD_CNT {
        &self.dbus1_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1e0 - DCache bus1 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus1_acs_nxtlvl_wr_cnt(&self) -> &DBUS1_ACS_NXTLVL_WR_CNT {
        &self.dbus1_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x21c - ICache0 Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn icache2_acs_fail_id_attr(&self) -> &ICACHE2_ACS_FAIL_ID_ATTR {
        &self.icache2_acs_fail_id_attr
    }
    #[doc = "0x220 - ICache0 Access Fail Address information register"]
    #[inline(always)]
    pub const fn icache2_acs_fail_addr(&self) -> &ICACHE2_ACS_FAIL_ADDR {
        &self.icache2_acs_fail_addr
    }
    #[doc = "0x22c - Cache Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn dcache_acs_fail_id_attr(&self) -> &DCACHE_ACS_FAIL_ID_ATTR {
        &self.dcache_acs_fail_id_attr
    }
    #[doc = "0x230 - Cache Access Fail Address information register"]
    #[inline(always)]
    pub const fn dcache_acs_fail_addr(&self) -> &DCACHE_ACS_FAIL_ADDR {
        &self.dcache_acs_fail_addr
    }
    #[doc = "0x234 - Cache Access Fail Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_sync_preload_int_ena(&self) -> &CACHE_SYNC_PRELOAD_INT_ENA {
        &self.cache_sync_preload_int_ena
    }
    #[doc = "0x238 - Sync Preload operation Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_sync_preload_int_clr(&self) -> &CACHE_SYNC_PRELOAD_INT_CLR {
        &self.cache_sync_preload_int_clr
    }
    #[doc = "0x23c - Sync Preload operation Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_sync_preload_int_raw(&self) -> &CACHE_SYNC_PRELOAD_INT_RAW {
        &self.cache_sync_preload_int_raw
    }
    #[doc = "0x240 - Cache Access Fail Interrupt status register"]
    #[inline(always)]
    pub const fn cache_sync_preload_int_st(&self) -> &CACHE_SYNC_PRELOAD_INT_ST {
        &self.cache_sync_preload_int_st
    }
    #[doc = "0x244 - Cache Sync/Preload Operation exception register"]
    #[inline(always)]
    pub const fn cache_sync_preload_exception(&self) -> &CACHE_SYNC_PRELOAD_EXCEPTION {
        &self.cache_sync_preload_exception
    }
    #[doc = "0x248 - Cache Sync Reset control register"]
    #[inline(always)]
    pub const fn cache_sync_rst_ctrl(&self) -> &CACHE_SYNC_RST_CTRL {
        &self.cache_sync_rst_ctrl
    }
    #[doc = "0x24c - Cache Preload Reset control register"]
    #[inline(always)]
    pub const fn cache_preload_rst_ctrl(&self) -> &CACHE_PRELOAD_RST_CTRL {
        &self.cache_preload_rst_ctrl
    }
    #[doc = "0x250 - Cache Autoload buffer clear control register"]
    #[inline(always)]
    pub const fn cache_autoload_buf_clr_ctrl(&self) -> &CACHE_AUTOLOAD_BUF_CLR_CTRL {
        &self.cache_autoload_buf_clr_ctrl
    }
    #[doc = "0x254 - Unallocate request buffer clear registers"]
    #[inline(always)]
    pub const fn unallocate_buffer_clear(&self) -> &UNALLOCATE_BUFFER_CLEAR {
        &self.unallocate_buffer_clear
    }
    #[doc = "0x258 - Cache Tag and Data memory Object control register"]
    #[inline(always)]
    pub const fn cache_object_ctrl(&self) -> &CACHE_OBJECT_CTRL {
        &self.cache_object_ctrl
    }
    #[doc = "0x25c - Cache Tag and Data memory way register"]
    #[inline(always)]
    pub const fn cache_way_object(&self) -> &CACHE_WAY_OBJECT {
        &self.cache_way_object
    }
    #[doc = "0x260 - Cache address register"]
    #[inline(always)]
    pub const fn cache_addr(&self) -> &CACHE_ADDR {
        &self.cache_addr
    }
    #[doc = "0x264 - Cache Tag/data memory content register"]
    #[inline(always)]
    pub const fn cache_debug_bus(&self) -> &CACHE_DEBUG_BUS {
        &self.cache_debug_bus
    }
    #[doc = "0x3d0 - Clock gate control register"]
    #[inline(always)]
    pub const fn trace_ena(&self) -> &TRACE_ENA {
        &self.trace_ena
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn cache_date(&self) -> &CACHE_DATE {
        &self.cache_date
    }
}
#[doc = "ICACHE_CTRL (rw) register accessor: Instruction cache (ICache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl`] module"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "Instruction cache (ICache) control register"]
pub mod icache_ctrl;
#[doc = "CACHE_CTRL (rw) register accessor: Data cache control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ctrl`] module"]
pub type CACHE_CTRL = crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>;
#[doc = "Data cache control register"]
pub mod cache_ctrl;
#[doc = "ICACHE_CACHESIZE_CONF (r) register accessor: Instruction Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_cachesize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_cachesize_conf`] module"]
pub type ICACHE_CACHESIZE_CONF = crate::Reg<icache_cachesize_conf::ICACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Instruction Cache CacheSize mode configure register"]
pub mod icache_cachesize_conf;
#[doc = "ICACHE_BLOCKSIZE_CONF (r) register accessor: Instruction Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_blocksize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_blocksize_conf`] module"]
pub type ICACHE_BLOCKSIZE_CONF = crate::Reg<icache_blocksize_conf::ICACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Instruction Cache BlockSize mode configure register"]
pub mod icache_blocksize_conf;
#[doc = "CACHE_CACHESIZE_CONF (r) register accessor: Data Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_cachesize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_cachesize_conf`] module"]
pub type CACHE_CACHESIZE_CONF = crate::Reg<cache_cachesize_conf::CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Data Cache CacheSize mode configure register"]
pub mod cache_cachesize_conf;
#[doc = "CACHE_BLOCKSIZE_CONF (r) register accessor: Data Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_blocksize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_blocksize_conf`] module"]
pub type CACHE_BLOCKSIZE_CONF = crate::Reg<cache_blocksize_conf::CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "Data Cache BlockSize mode configure register"]
pub mod cache_blocksize_conf;
#[doc = "CACHE_WRAP_AROUND_CTRL (rw) register accessor: Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_wrap_around_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_wrap_around_ctrl`] module"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MISS_ACCESS_CTRL (rw) register accessor: Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_miss_access_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_miss_access_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_miss_access_ctrl`] module"]
pub type CACHE_MISS_ACCESS_CTRL = crate::Reg<cache_miss_access_ctrl::CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod cache_miss_access_ctrl;
#[doc = "CACHE_FREEZE_CTRL (rw) register accessor: Cache Freeze control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_freeze_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_freeze_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_freeze_ctrl`] module"]
pub type CACHE_FREEZE_CTRL = crate::Reg<cache_freeze_ctrl::CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Cache Freeze control register"]
pub mod cache_freeze_ctrl;
#[doc = "CACHE_DATA_MEM_ACS_CONF (rw) register accessor: Cache data memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_data_mem_acs_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_data_mem_acs_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_data_mem_acs_conf`] module"]
pub type CACHE_DATA_MEM_ACS_CONF =
    crate::Reg<cache_data_mem_acs_conf::CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Cache data memory access configure register"]
pub mod cache_data_mem_acs_conf;
#[doc = "CACHE_TAG_MEM_ACS_CONF (rw) register accessor: Cache tag memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_mem_acs_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_mem_acs_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_tag_mem_acs_conf`] module"]
pub type CACHE_TAG_MEM_ACS_CONF = crate::Reg<cache_tag_mem_acs_conf::CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Cache tag memory access configure register"]
pub mod cache_tag_mem_acs_conf;
#[doc = "ICACHE2_PRELOCK_CONF (rw) register accessor: Instruction Cache 2 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_conf`] module"]
pub type ICACHE2_PRELOCK_CONF = crate::Reg<icache2_prelock_conf::ICACHE2_PRELOCK_CONF_SPEC>;
#[doc = "Instruction Cache 2 prelock configure register"]
pub mod icache2_prelock_conf;
#[doc = "ICACHE2_PRELOCK_SCT0_ADDR (rw) register accessor: Instruction Cache 2 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct0_addr`] module"]
pub type ICACHE2_PRELOCK_SCT0_ADDR =
    crate::Reg<icache2_prelock_sct0_addr::ICACHE2_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Instruction Cache 2 prelock section0 address configure register"]
pub mod icache2_prelock_sct0_addr;
#[doc = "ICACHE2_PRELOCK_SCT1_ADDR (rw) register accessor: Instruction Cache 2 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct1_addr`] module"]
pub type ICACHE2_PRELOCK_SCT1_ADDR =
    crate::Reg<icache2_prelock_sct1_addr::ICACHE2_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Instruction Cache 2 prelock section1 address configure register"]
pub mod icache2_prelock_sct1_addr;
#[doc = "ICACHE2_PRELOCK_SCT_SIZE (rw) register accessor: Instruction Cache 2 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_sct_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct_size`] module"]
pub type ICACHE2_PRELOCK_SCT_SIZE =
    crate::Reg<icache2_prelock_sct_size::ICACHE2_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Instruction Cache 2 prelock section size configure register"]
pub mod icache2_prelock_sct_size;
#[doc = "CACHE_PRELOCK_CONF (rw) register accessor: Cache prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_prelock_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_prelock_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_prelock_conf`] module"]
pub type CACHE_PRELOCK_CONF = crate::Reg<cache_prelock_conf::CACHE_PRELOCK_CONF_SPEC>;
#[doc = "Cache prelock configure register"]
pub mod cache_prelock_conf;
#[doc = "CACHE_PRELOCK_SCT0_ADDR (rw) register accessor: Cache prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_prelock_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_prelock_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_prelock_sct0_addr`] module"]
pub type CACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<cache_prelock_sct0_addr::CACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Cache prelock section0 address configure register"]
pub mod cache_prelock_sct0_addr;
#[doc = "DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: Cache prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct1_addr`] module"]
pub type DCACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<dcache_prelock_sct1_addr::DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Cache prelock section1 address configure register"]
pub mod dcache_prelock_sct1_addr;
#[doc = "DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: Cache prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct_size`] module"]
pub type DCACHE_PRELOCK_SCT_SIZE =
    crate::Reg<dcache_prelock_sct_size::DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Cache prelock section size configure register"]
pub mod dcache_prelock_sct_size;
#[doc = "LOCK_CTRL (rw) register accessor: Lock-class (manual lock) operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_ctrl`] module"]
pub type LOCK_CTRL = crate::Reg<lock_ctrl::LOCK_CTRL_SPEC>;
#[doc = "Lock-class (manual lock) operation control register"]
pub mod lock_ctrl;
#[doc = "LOCK_MAP (rw) register accessor: Lock (manual lock) map configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_map`] module"]
pub type LOCK_MAP = crate::Reg<lock_map::LOCK_MAP_SPEC>;
#[doc = "Lock (manual lock) map configure register"]
pub mod lock_map;
#[doc = "LOCK_ADDR (rw) register accessor: Lock (manual lock) address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_addr`] module"]
pub type LOCK_ADDR = crate::Reg<lock_addr::LOCK_ADDR_SPEC>;
#[doc = "Lock (manual lock) address configure register"]
pub mod lock_addr;
#[doc = "LOCK_SIZE (rw) register accessor: Lock (manual lock) size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_size`] module"]
pub type LOCK_SIZE = crate::Reg<lock_size::LOCK_SIZE_SPEC>;
#[doc = "Lock (manual lock) size configure register"]
pub mod lock_size;
#[doc = "SYNC_CTRL (rw) register accessor: Sync-class operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ctrl`] module"]
pub type SYNC_CTRL = crate::Reg<sync_ctrl::SYNC_CTRL_SPEC>;
#[doc = "Sync-class operation control register"]
pub mod sync_ctrl;
#[doc = "SYNC_MAP (rw) register accessor: Sync map configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_map`] module"]
pub type SYNC_MAP = crate::Reg<sync_map::SYNC_MAP_SPEC>;
#[doc = "Sync map configure register"]
pub mod sync_map;
#[doc = "SYNC_ADDR (rw) register accessor: Sync address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_addr`] module"]
pub type SYNC_ADDR = crate::Reg<sync_addr::SYNC_ADDR_SPEC>;
#[doc = "Sync address configure register"]
pub mod sync_addr;
#[doc = "SYNC_SIZE (rw) register accessor: Sync size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_size`] module"]
pub type SYNC_SIZE = crate::Reg<sync_size::SYNC_SIZE_SPEC>;
#[doc = "Sync size configure register"]
pub mod sync_size;
#[doc = "ICACHE2_PRELOAD_CTRL (rw) register accessor: Instruction Cache 2 preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_ctrl`] module"]
pub type ICACHE2_PRELOAD_CTRL = crate::Reg<icache2_preload_ctrl::ICACHE2_PRELOAD_CTRL_SPEC>;
#[doc = "Instruction Cache 2 preload-operation control register"]
pub mod icache2_preload_ctrl;
#[doc = "ICACHE2_PRELOAD_ADDR (rw) register accessor: Instruction Cache 2 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_addr`] module"]
pub type ICACHE2_PRELOAD_ADDR = crate::Reg<icache2_preload_addr::ICACHE2_PRELOAD_ADDR_SPEC>;
#[doc = "Instruction Cache 2 preload address configure register"]
pub mod icache2_preload_addr;
#[doc = "ICACHE2_PRELOAD_SIZE (rw) register accessor: Instruction Cache 2 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_size`] module"]
pub type ICACHE2_PRELOAD_SIZE = crate::Reg<icache2_preload_size::ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "Instruction Cache 2 preload size configure register"]
pub mod icache2_preload_size;
#[doc = "CACHE_PRELOAD_CTRL (rw) register accessor: Cache preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_ctrl`] module"]
pub type CACHE_PRELOAD_CTRL = crate::Reg<cache_preload_ctrl::CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "Cache preload-operation control register"]
pub mod cache_preload_ctrl;
#[doc = "DCACHE_PRELOAD_ADDR (rw) register accessor: Cache preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_addr`] module"]
pub type DCACHE_PRELOAD_ADDR = crate::Reg<dcache_preload_addr::DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "Cache preload address configure register"]
pub mod dcache_preload_addr;
#[doc = "DCACHE_PRELOAD_SIZE (rw) register accessor: Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_size`] module"]
pub type DCACHE_PRELOAD_SIZE = crate::Reg<dcache_preload_size::DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Cache preload size configure register"]
pub mod dcache_preload_size;
#[doc = "ICACHE2_AUTOLOAD_CTRL (rw) register accessor: Instruction Cache 2 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_ctrl`] module"]
pub type ICACHE2_AUTOLOAD_CTRL = crate::Reg<icache2_autoload_ctrl::ICACHE2_AUTOLOAD_CTRL_SPEC>;
#[doc = "Instruction Cache 2 autoload-operation control register"]
pub mod icache2_autoload_ctrl;
#[doc = "ICACHE2_AUTOLOAD_SCT0_ADDR (rw) register accessor: Instruction Cache 2 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct0_addr`] module"]
pub type ICACHE2_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache2_autoload_sct0_addr::ICACHE2_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "Instruction Cache 2 autoload section 0 address configure register"]
pub mod icache2_autoload_sct0_addr;
#[doc = "ICACHE2_AUTOLOAD_SCT0_SIZE (rw) register accessor: Instruction Cache 2 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_sct0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct0_size`] module"]
pub type ICACHE2_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache2_autoload_sct0_size::ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Instruction Cache 2 autoload section 0 size configure register"]
pub mod icache2_autoload_sct0_size;
#[doc = "ICACHE2_AUTOLOAD_SCT1_ADDR (rw) register accessor: Instruction Cache 2 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct1_addr`] module"]
pub type ICACHE2_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache2_autoload_sct1_addr::ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "Instruction Cache 2 autoload section 1 address configure register"]
pub mod icache2_autoload_sct1_addr;
#[doc = "ICACHE2_AUTOLOAD_SCT1_SIZE (rw) register accessor: Instruction Cache 2 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_sct1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct1_size`] module"]
pub type ICACHE2_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache2_autoload_sct1_size::ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Instruction Cache 2 autoload section 1 size configure register"]
pub mod icache2_autoload_sct1_size;
#[doc = "CACHE_AUTOLOAD_CTRL (rw) register accessor: Cache autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_ctrl`] module"]
pub type CACHE_AUTOLOAD_CTRL = crate::Reg<cache_autoload_ctrl::CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Cache autoload-operation control register"]
pub mod cache_autoload_ctrl;
#[doc = "CACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: Cache autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct0_addr`] module"]
pub type CACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<cache_autoload_sct0_addr::CACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "Cache autoload section 0 address configure register"]
pub mod cache_autoload_sct0_addr;
#[doc = "CACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: Cache autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct0_size`] module"]
pub type CACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<cache_autoload_sct0_size::CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Cache autoload section 0 size configure register"]
pub mod cache_autoload_sct0_size;
#[doc = "CACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: Cache autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct1_addr`] module"]
pub type CACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<cache_autoload_sct1_addr::CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "Cache autoload section 1 address configure register"]
pub mod cache_autoload_sct1_addr;
#[doc = "CACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct1_size`] module"]
pub type CACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<cache_autoload_sct1_size::CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Cache autoload section 1 size configure register"]
pub mod cache_autoload_sct1_size;
#[doc = "CACHE_ACS_CNT_INT_ENA (rw) register accessor: Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_ena`] module"]
pub type CACHE_ACS_CNT_INT_ENA = crate::Reg<cache_acs_cnt_int_ena::CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Cache Access Counter Interrupt enable register"]
pub mod cache_acs_cnt_int_ena;
#[doc = "CACHE_ACS_CNT_INT_CLR (w) register accessor: Cache Access Counter Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_clr`] module"]
pub type CACHE_ACS_CNT_INT_CLR = crate::Reg<cache_acs_cnt_int_clr::CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Cache Access Counter Interrupt clear register"]
pub mod cache_acs_cnt_int_clr;
#[doc = "CACHE_ACS_CNT_INT_RAW (rw) register accessor: Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_raw`] module"]
pub type CACHE_ACS_CNT_INT_RAW = crate::Reg<cache_acs_cnt_int_raw::CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Cache Access Counter Interrupt raw register"]
pub mod cache_acs_cnt_int_raw;
#[doc = "CACHE_ACS_CNT_INT_ST (r) register accessor: Cache Access Counter Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_st`] module"]
pub type CACHE_ACS_CNT_INT_ST = crate::Reg<cache_acs_cnt_int_st::CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Cache Access Counter Interrupt status register"]
pub mod cache_acs_cnt_int_st;
#[doc = "CACHE_ACS_FAIL_CTRL (rw) register accessor: Cache Access Fail Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_ctrl`] module"]
pub type CACHE_ACS_FAIL_CTRL = crate::Reg<cache_acs_fail_ctrl::CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Cache Access Fail Configuration register"]
pub mod cache_acs_fail_ctrl;
#[doc = "CACHE_ACS_FAIL_INT_ENA (rw) register accessor: Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_int_ena`] module"]
pub type CACHE_ACS_FAIL_INT_ENA = crate::Reg<cache_acs_fail_int_ena::CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Cache Access Fail Interrupt enable register"]
pub mod cache_acs_fail_int_ena;
#[doc = "CACHE_ACS_FAIL_INT_CLR (w) register accessor: Cache Access Fail Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_int_clr`] module"]
pub type CACHE_ACS_FAIL_INT_CLR = crate::Reg<cache_acs_fail_int_clr::CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "Cache Access Fail Interrupt clear register"]
pub mod cache_acs_fail_int_clr;
#[doc = "CACHE_ACS_FAIL_INT_RAW (rw) register accessor: Cache Access Fail Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_int_raw`] module"]
pub type CACHE_ACS_FAIL_INT_RAW = crate::Reg<cache_acs_fail_int_raw::CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Cache Access Fail Interrupt raw register"]
pub mod cache_acs_fail_int_raw;
#[doc = "CACHE_ACS_FAIL_INT_ST (r) register accessor: Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_int_st`] module"]
pub type CACHE_ACS_FAIL_INT_ST = crate::Reg<cache_acs_fail_int_st::CACHE_ACS_FAIL_INT_ST_SPEC>;
#[doc = "Cache Access Fail Interrupt status register"]
pub mod cache_acs_fail_int_st;
#[doc = "CACHE_ACS_CNT_CTRL (rw) register accessor: Cache Access Counter enable and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_ctrl`] module"]
pub type CACHE_ACS_CNT_CTRL = crate::Reg<cache_acs_cnt_ctrl::CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Cache Access Counter enable and clear register"]
pub mod cache_acs_cnt_ctrl;
#[doc = "IBUS2_ACS_HIT_CNT (r) register accessor: ICache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_hit_cnt`] module"]
pub type IBUS2_ACS_HIT_CNT = crate::Reg<ibus2_acs_hit_cnt::IBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "ICache bus2 Hit-Access Counter register"]
pub mod ibus2_acs_hit_cnt;
#[doc = "IBUS2_ACS_MISS_CNT (r) register accessor: ICache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_miss_cnt`] module"]
pub type IBUS2_ACS_MISS_CNT = crate::Reg<ibus2_acs_miss_cnt::IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "ICache bus2 Miss-Access Counter register"]
pub mod ibus2_acs_miss_cnt;
#[doc = "IBUS2_ACS_CONFLICT_CNT (r) register accessor: ICache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_conflict_cnt`] module"]
pub type IBUS2_ACS_CONFLICT_CNT = crate::Reg<ibus2_acs_conflict_cnt::IBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "ICache bus2 Conflict-Access Counter register"]
pub mod ibus2_acs_conflict_cnt;
#[doc = "IBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: ICache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS2_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus2_acs_nxtlvl_rd_cnt::IBUS2_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "ICache bus2 Next-Level-Access Counter register"]
pub mod ibus2_acs_nxtlvl_rd_cnt;
#[doc = "IBUS3_ACS_HIT_CNT (r) register accessor: ICache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_hit_cnt`] module"]
pub type IBUS3_ACS_HIT_CNT = crate::Reg<ibus3_acs_hit_cnt::IBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "ICache bus3 Hit-Access Counter register"]
pub mod ibus3_acs_hit_cnt;
#[doc = "IBUS3_ACS_MISS_CNT (r) register accessor: ICache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_miss_cnt`] module"]
pub type IBUS3_ACS_MISS_CNT = crate::Reg<ibus3_acs_miss_cnt::IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "ICache bus3 Miss-Access Counter register"]
pub mod ibus3_acs_miss_cnt;
#[doc = "IBUS3_ACS_CONFLICT_CNT (r) register accessor: ICache bus3 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_conflict_cnt`] module"]
pub type IBUS3_ACS_CONFLICT_CNT = crate::Reg<ibus3_acs_conflict_cnt::IBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "ICache bus3 Conflict-Access Counter register"]
pub mod ibus3_acs_conflict_cnt;
#[doc = "IBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: ICache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS3_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus3_acs_nxtlvl_rd_cnt::IBUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "ICache bus3 Next-Level-Access Counter register"]
pub mod ibus3_acs_nxtlvl_rd_cnt;
#[doc = "BUS0_ACS_HIT_CNT (r) register accessor: Cache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_hit_cnt`] module"]
pub type BUS0_ACS_HIT_CNT = crate::Reg<bus0_acs_hit_cnt::BUS0_ACS_HIT_CNT_SPEC>;
#[doc = "Cache bus0 Hit-Access Counter register"]
pub mod bus0_acs_hit_cnt;
#[doc = "BUS0_ACS_MISS_CNT (r) register accessor: Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_miss_cnt`] module"]
pub type BUS0_ACS_MISS_CNT = crate::Reg<bus0_acs_miss_cnt::BUS0_ACS_MISS_CNT_SPEC>;
#[doc = "Cache bus0 Miss-Access Counter register"]
pub mod bus0_acs_miss_cnt;
#[doc = "BUS0_ACS_CONFLICT_CNT (r) register accessor: Cache bus0 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_conflict_cnt`] module"]
pub type BUS0_ACS_CONFLICT_CNT = crate::Reg<bus0_acs_conflict_cnt::BUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "Cache bus0 Conflict-Access Counter register"]
pub mod bus0_acs_conflict_cnt;
#[doc = "DBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS0_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus0_acs_nxtlvl_rd_cnt::DBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "Cache bus0 Next-Level-Access Counter register"]
pub mod dbus0_acs_nxtlvl_rd_cnt;
#[doc = "DBUS0_ACS_NXTLVL_WR_CNT (r) register accessor: DCache bus0 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS0_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus0_acs_nxtlvl_wr_cnt::DBUS0_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "DCache bus0 WB-Access Counter register"]
pub mod dbus0_acs_nxtlvl_wr_cnt;
#[doc = "BUS1_ACS_HIT_CNT (r) register accessor: Cache bus1 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_hit_cnt`] module"]
pub type BUS1_ACS_HIT_CNT = crate::Reg<bus1_acs_hit_cnt::BUS1_ACS_HIT_CNT_SPEC>;
#[doc = "Cache bus1 Hit-Access Counter register"]
pub mod bus1_acs_hit_cnt;
#[doc = "BUS1_ACS_MISS_CNT (r) register accessor: Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_miss_cnt`] module"]
pub type BUS1_ACS_MISS_CNT = crate::Reg<bus1_acs_miss_cnt::BUS1_ACS_MISS_CNT_SPEC>;
#[doc = "Cache bus1 Miss-Access Counter register"]
pub mod bus1_acs_miss_cnt;
#[doc = "BUS1_ACS_CONFLICT_CNT (r) register accessor: Cache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_conflict_cnt`] module"]
pub type BUS1_ACS_CONFLICT_CNT = crate::Reg<bus1_acs_conflict_cnt::BUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "Cache bus1 Conflict-Access Counter register"]
pub mod bus1_acs_conflict_cnt;
#[doc = "DBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: DCache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS1_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus1_acs_nxtlvl_rd_cnt::DBUS1_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "DCache bus1 Next-Level-Access Counter register"]
pub mod dbus1_acs_nxtlvl_rd_cnt;
#[doc = "DBUS1_ACS_NXTLVL_WR_CNT (r) register accessor: DCache bus1 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS1_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus1_acs_nxtlvl_wr_cnt::DBUS1_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "DCache bus1 WB-Access Counter register"]
pub mod dbus1_acs_nxtlvl_wr_cnt;
#[doc = "ICACHE2_ACS_FAIL_ID_ATTR (r) register accessor: ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_acs_fail_id_attr`] module"]
pub type ICACHE2_ACS_FAIL_ID_ATTR =
    crate::Reg<icache2_acs_fail_id_attr::ICACHE2_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "ICache0 Access Fail ID/attribution information register"]
pub mod icache2_acs_fail_id_attr;
#[doc = "ICACHE2_ACS_FAIL_ADDR (r) register accessor: ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_acs_fail_addr`] module"]
pub type ICACHE2_ACS_FAIL_ADDR = crate::Reg<icache2_acs_fail_addr::ICACHE2_ACS_FAIL_ADDR_SPEC>;
#[doc = "ICache0 Access Fail Address information register"]
pub mod icache2_acs_fail_addr;
#[doc = "DCACHE_ACS_FAIL_ID_ATTR (r) register accessor: Cache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_acs_fail_id_attr`] module"]
pub type DCACHE_ACS_FAIL_ID_ATTR =
    crate::Reg<dcache_acs_fail_id_attr::DCACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "Cache Access Fail ID/attribution information register"]
pub mod dcache_acs_fail_id_attr;
#[doc = "DCACHE_ACS_FAIL_ADDR (r) register accessor: Cache Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_acs_fail_addr`] module"]
pub type DCACHE_ACS_FAIL_ADDR = crate::Reg<dcache_acs_fail_addr::DCACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "Cache Access Fail Address information register"]
pub mod dcache_acs_fail_addr;
#[doc = "CACHE_SYNC_PRELOAD_INT_ENA (rw) register accessor: Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_ena`] module"]
pub type CACHE_SYNC_PRELOAD_INT_ENA =
    crate::Reg<cache_sync_preload_int_ena::CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "Cache Access Fail Interrupt enable register"]
pub mod cache_sync_preload_int_ena;
#[doc = "CACHE_SYNC_PRELOAD_INT_CLR (w) register accessor: Sync Preload operation Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_clr`] module"]
pub type CACHE_SYNC_PRELOAD_INT_CLR =
    crate::Reg<cache_sync_preload_int_clr::CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Sync Preload operation Interrupt clear register"]
pub mod cache_sync_preload_int_clr;
#[doc = "CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: Sync Preload operation Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_raw`] module"]
pub type CACHE_SYNC_PRELOAD_INT_RAW =
    crate::Reg<cache_sync_preload_int_raw::CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Sync Preload operation Interrupt raw register"]
pub mod cache_sync_preload_int_raw;
#[doc = "CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_st`] module"]
pub type CACHE_SYNC_PRELOAD_INT_ST =
    crate::Reg<cache_sync_preload_int_st::CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "Cache Access Fail Interrupt status register"]
pub mod cache_sync_preload_int_st;
#[doc = "CACHE_SYNC_PRELOAD_EXCEPTION (r) register accessor: Cache Sync/Preload Operation exception register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_exception::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_exception`] module"]
pub type CACHE_SYNC_PRELOAD_EXCEPTION =
    crate::Reg<cache_sync_preload_exception::CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Cache Sync/Preload Operation exception register"]
pub mod cache_sync_preload_exception;
#[doc = "CACHE_SYNC_RST_CTRL (rw) register accessor: Cache Sync Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_rst_ctrl`] module"]
pub type CACHE_SYNC_RST_CTRL = crate::Reg<cache_sync_rst_ctrl::CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Cache Sync Reset control register"]
pub mod cache_sync_rst_ctrl;
#[doc = "CACHE_PRELOAD_RST_CTRL (rw) register accessor: Cache Preload Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_rst_ctrl`] module"]
pub type CACHE_PRELOAD_RST_CTRL = crate::Reg<cache_preload_rst_ctrl::CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Cache Preload Reset control register"]
pub mod cache_preload_rst_ctrl;
#[doc = "CACHE_AUTOLOAD_BUF_CLR_CTRL (rw) register accessor: Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_buf_clr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_buf_clr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_buf_clr_ctrl`] module"]
pub type CACHE_AUTOLOAD_BUF_CLR_CTRL =
    crate::Reg<cache_autoload_buf_clr_ctrl::CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Cache Autoload buffer clear control register"]
pub mod cache_autoload_buf_clr_ctrl;
#[doc = "UNALLOCATE_BUFFER_CLEAR (rw) register accessor: Unallocate request buffer clear registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unallocate_buffer_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unallocate_buffer_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unallocate_buffer_clear`] module"]
pub type UNALLOCATE_BUFFER_CLEAR =
    crate::Reg<unallocate_buffer_clear::UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Unallocate request buffer clear registers"]
pub mod unallocate_buffer_clear;
#[doc = "CACHE_OBJECT_CTRL (rw) register accessor: Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_object_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_object_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_object_ctrl`] module"]
pub type CACHE_OBJECT_CTRL = crate::Reg<cache_object_ctrl::CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Cache Tag and Data memory Object control register"]
pub mod cache_object_ctrl;
#[doc = "CACHE_WAY_OBJECT (rw) register accessor: Cache Tag and Data memory way register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_way_object::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_way_object::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_way_object`] module"]
pub type CACHE_WAY_OBJECT = crate::Reg<cache_way_object::CACHE_WAY_OBJECT_SPEC>;
#[doc = "Cache Tag and Data memory way register"]
pub mod cache_way_object;
#[doc = "CACHE_ADDR (rw) register accessor: Cache address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_addr`] module"]
pub type CACHE_ADDR = crate::Reg<cache_addr::CACHE_ADDR_SPEC>;
#[doc = "Cache address register"]
pub mod cache_addr;
#[doc = "CACHE_DEBUG_BUS (rw) register accessor: Cache Tag/data memory content register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_debug_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_debug_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_debug_bus`] module"]
pub type CACHE_DEBUG_BUS = crate::Reg<cache_debug_bus::CACHE_DEBUG_BUS_SPEC>;
#[doc = "Cache Tag/data memory content register"]
pub mod cache_debug_bus;
#[doc = "TRACE_ENA (rw) register accessor: Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_ena`] module"]
pub type TRACE_ENA = crate::Reg<trace_ena::TRACE_ENA_SPEC>;
#[doc = "Clock gate control register"]
pub mod trace_ena;
pub use crate::aes::{date as cache_date, DATE as CACHE_DATE};
