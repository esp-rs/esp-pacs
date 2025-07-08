#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    icache_ctrl: ICACHE_CTRL,
    cache_ctrl: CACHE_CTRL,
    bypass_cache_conf: BYPASS_CACHE_CONF,
    cache_atomic_conf: CACHE_ATOMIC_CONF,
    icache_cachesize_conf: ICACHE_CACHESIZE_CONF,
    icache_blocksize_conf: ICACHE_BLOCKSIZE_CONF,
    cache_cachesize_conf: CACHE_CACHESIZE_CONF,
    cache_blocksize_conf: CACHE_BLOCKSIZE_CONF,
    cache_wrap_around_ctrl: CACHE_WRAP_AROUND_CTRL,
    cache_miss_access_ctrl: CACHE_MISS_ACCESS_CTRL,
    cache_freeze_ctrl: CACHE_FREEZE_CTRL,
    cache_data_mem_acs_conf: CACHE_DATA_MEM_ACS_CONF,
    cache_tag_mem_acs_conf: CACHE_TAG_MEM_ACS_CONF,
    icache0_prelock_conf: ICACHE0_PRELOCK_CONF,
    icache0_prelock_sct0_addr: ICACHE0_PRELOCK_SCT0_ADDR,
    icache0_prelock_sct1_addr: ICACHE0_PRELOCK_SCT1_ADDR,
    icache0_prelock_sct_size: ICACHE0_PRELOCK_SCT_SIZE,
    icache1_prelock_conf: ICACHE1_PRELOCK_CONF,
    icache1_prelock_sct0_addr: ICACHE1_PRELOCK_SCT0_ADDR,
    icache1_prelock_sct1_addr: ICACHE1_PRELOCK_SCT1_ADDR,
    icache1_prelock_sct_size: ICACHE1_PRELOCK_SCT_SIZE,
    icache2_prelock_conf: ICACHE2_PRELOCK_CONF,
    icache2_prelock_sct0_addr: ICACHE2_PRELOCK_SCT0_ADDR,
    icache2_prelock_sct1_addr: ICACHE2_PRELOCK_SCT1_ADDR,
    icache2_prelock_sct_size: ICACHE2_PRELOCK_SCT_SIZE,
    icache3_prelock_conf: ICACHE3_PRELOCK_CONF,
    icache3_prelock_sct0_addr: ICACHE3_PRELOCK_SCT0_ADDR,
    icache3_prelock_sct1_addr: ICACHE3_PRELOCK_SCT1_ADDR,
    icache3_prelock_sct_size: ICACHE3_PRELOCK_SCT_SIZE,
    cache_prelock_conf: CACHE_PRELOCK_CONF,
    cache_prelock_sct0_addr: CACHE_PRELOCK_SCT0_ADDR,
    dcache_prelock_sct1_addr: DCACHE_PRELOCK_SCT1_ADDR,
    dcache_prelock_sct_size: DCACHE_PRELOCK_SCT_SIZE,
    cache_lock_ctrl: CACHE_LOCK_CTRL,
    cache_lock_map: CACHE_LOCK_MAP,
    cache_lock_addr: CACHE_LOCK_ADDR,
    cache_lock_size: CACHE_LOCK_SIZE,
    cache_sync_ctrl: CACHE_SYNC_CTRL,
    cache_sync_map: CACHE_SYNC_MAP,
    cache_sync_addr: CACHE_SYNC_ADDR,
    cache_sync_size: CACHE_SYNC_SIZE,
    icache0_preload_ctrl: ICACHE0_PRELOAD_CTRL,
    icache0_preload_addr: ICACHE0_PRELOAD_ADDR,
    icache0_preload_size: ICACHE0_PRELOAD_SIZE,
    icache1_preload_ctrl: ICACHE1_PRELOAD_CTRL,
    icache1_preload_addr: ICACHE1_PRELOAD_ADDR,
    icache1_preload_size: ICACHE1_PRELOAD_SIZE,
    icache2_preload_ctrl: ICACHE2_PRELOAD_CTRL,
    icache2_preload_addr: ICACHE2_PRELOAD_ADDR,
    icache2_preload_size: ICACHE2_PRELOAD_SIZE,
    icache3_preload_ctrl: ICACHE3_PRELOAD_CTRL,
    icache3_preload_addr: ICACHE3_PRELOAD_ADDR,
    icache3_preload_size: ICACHE3_PRELOAD_SIZE,
    cache_preload_ctrl: CACHE_PRELOAD_CTRL,
    dcache_preload_addr: DCACHE_PRELOAD_ADDR,
    dcache_preload_size: DCACHE_PRELOAD_SIZE,
    icache0_autoload_ctrl: ICACHE0_AUTOLOAD_CTRL,
    icache0_autoload_sct0_addr: ICACHE0_AUTOLOAD_SCT0_ADDR,
    icache0_autoload_sct0_size: ICACHE0_AUTOLOAD_SCT0_SIZE,
    icache0_autoload_sct1_addr: ICACHE0_AUTOLOAD_SCT1_ADDR,
    icache0_autoload_sct1_size: ICACHE0_AUTOLOAD_SCT1_SIZE,
    icache1_autoload_ctrl: ICACHE1_AUTOLOAD_CTRL,
    icache1_autoload_sct0_addr: ICACHE1_AUTOLOAD_SCT0_ADDR,
    icache1_autoload_sct0_size: ICACHE1_AUTOLOAD_SCT0_SIZE,
    icache1_autoload_sct1_addr: ICACHE1_AUTOLOAD_SCT1_ADDR,
    icache1_autoload_sct1_size: ICACHE1_AUTOLOAD_SCT1_SIZE,
    icache2_autoload_ctrl: ICACHE2_AUTOLOAD_CTRL,
    icache2_autoload_sct0_addr: ICACHE2_AUTOLOAD_SCT0_ADDR,
    icache2_autoload_sct0_size: ICACHE2_AUTOLOAD_SCT0_SIZE,
    icache2_autoload_sct1_addr: ICACHE2_AUTOLOAD_SCT1_ADDR,
    icache2_autoload_sct1_size: ICACHE2_AUTOLOAD_SCT1_SIZE,
    icache3_autoload_ctrl: ICACHE3_AUTOLOAD_CTRL,
    icache3_autoload_sct0_addr: ICACHE3_AUTOLOAD_SCT0_ADDR,
    icache3_autoload_sct0_size: ICACHE3_AUTOLOAD_SCT0_SIZE,
    icache3_autoload_sct1_addr: ICACHE3_AUTOLOAD_SCT1_ADDR,
    icache3_autoload_sct1_size: ICACHE3_AUTOLOAD_SCT1_SIZE,
    cache_autoload_ctrl: CACHE_AUTOLOAD_CTRL,
    cache_autoload_sct0_addr: CACHE_AUTOLOAD_SCT0_ADDR,
    cache_autoload_sct0_size: CACHE_AUTOLOAD_SCT0_SIZE,
    cache_autoload_sct1_addr: CACHE_AUTOLOAD_SCT1_ADDR,
    cache_autoload_sct1_size: CACHE_AUTOLOAD_SCT1_SIZE,
    cache_autoload_sct2_addr: CACHE_AUTOLOAD_SCT2_ADDR,
    cache_autoload_sct2_size: CACHE_AUTOLOAD_SCT2_SIZE,
    cache_autoload_sct3_addr: CACHE_AUTOLOAD_SCT3_ADDR,
    cache_autoload_sct3_size: CACHE_AUTOLOAD_SCT3_SIZE,
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
    ibus0_acs_hit_cnt: IBUS0_ACS_HIT_CNT,
    ibus0_acs_miss_cnt: IBUS0_ACS_MISS_CNT,
    ibus0_acs_conflict_cnt: IBUS0_ACS_CONFLICT_CNT,
    ibus0_acs_nxtlvl_rd_cnt: IBUS0_ACS_NXTLVL_RD_CNT,
    ibus1_acs_hit_cnt: IBUS1_ACS_HIT_CNT,
    ibus1_acs_miss_cnt: IBUS1_ACS_MISS_CNT,
    ibus1_acs_conflict_cnt: IBUS1_ACS_CONFLICT_CNT,
    ibus1_acs_nxtlvl_rd_cnt: IBUS1_ACS_NXTLVL_RD_CNT,
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
    dbus2_acs_hit_cnt: DBUS2_ACS_HIT_CNT,
    dbus2_acs_miss_cnt: DBUS2_ACS_MISS_CNT,
    dbus2_acs_conflict_cnt: DBUS2_ACS_CONFLICT_CNT,
    dbus2_acs_nxtlvl_rd_cnt: DBUS2_ACS_NXTLVL_RD_CNT,
    dbus2_acs_nxtlvl_wr_cnt: DBUS2_ACS_NXTLVL_WR_CNT,
    dbus3_acs_hit_cnt: DBUS3_ACS_HIT_CNT,
    dbus3_acs_miss_cnt: DBUS3_ACS_MISS_CNT,
    dbus3_acs_conflict_cnt: DBUS3_ACS_CONFLICT_CNT,
    dbus3_acs_nxtlvl_rd_cnt: DBUS3_ACS_NXTLVL_RD_CNT,
    dbus3_acs_nxtlvl_wr_cnt: DBUS3_ACS_NXTLVL_WR_CNT,
    icache0_acs_fail_id_attr: ICACHE0_ACS_FAIL_ID_ATTR,
    icache0_acs_fail_addr: ICACHE0_ACS_FAIL_ADDR,
    icache1_acs_fail_id_attr: ICACHE1_ACS_FAIL_ID_ATTR,
    icache1_acs_fail_addr: ICACHE1_ACS_FAIL_ADDR,
    icache2_acs_fail_id_attr: ICACHE2_ACS_FAIL_ID_ATTR,
    icache2_acs_fail_addr: ICACHE2_ACS_FAIL_ADDR,
    icache3_acs_fail_id_attr: ICACHE3_ACS_FAIL_ID_ATTR,
    icache3_acs_fail_addr: ICACHE3_ACS_FAIL_ADDR,
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
    cache_level_split0: CACHE_LEVEL_SPLIT0,
    cache_l2_cache_ctrl: CACHE_L2_CACHE_CTRL,
    cache_l2_bypass_cache_conf: CACHE_L2_BYPASS_CACHE_CONF,
    cache_l2_cache_cachesize_conf: CACHE_L2_CACHE_CACHESIZE_CONF,
    cache_l2_cache_blocksize_conf: CACHE_L2_CACHE_BLOCKSIZE_CONF,
    cache_l2_cache_wrap_around_ctrl: CACHE_L2_CACHE_WRAP_AROUND_CTRL,
    cache_l2_cache_miss_access_ctrl: CACHE_L2_CACHE_MISS_ACCESS_CTRL,
    cache_l2_cache_freeze_ctrl: CACHE_L2_CACHE_FREEZE_CTRL,
    cache_l2_cache_data_mem_acs_conf: CACHE_L2_CACHE_DATA_MEM_ACS_CONF,
    cache_l2_cache_tag_mem_acs_conf: CACHE_L2_CACHE_TAG_MEM_ACS_CONF,
    cache_l2_cache_prelock_conf: CACHE_L2_CACHE_PRELOCK_CONF,
    cache_l2_cache_prelock_sct0_addr: CACHE_L2_CACHE_PRELOCK_SCT0_ADDR,
    cache_l2_cache_prelock_sct1_addr: CACHE_L2_CACHE_PRELOCK_SCT1_ADDR,
    cache_l2_cache_prelock_sct_size: CACHE_L2_CACHE_PRELOCK_SCT_SIZE,
    cache_l2_cache_preload_ctrl: CACHE_L2_CACHE_PRELOAD_CTRL,
    cache_l2_cache_preload_addr: CACHE_L2_CACHE_PRELOAD_ADDR,
    cache_l2_cache_preload_size: CACHE_L2_CACHE_PRELOAD_SIZE,
    cache_l2_cache_autoload_ctrl: CACHE_L2_CACHE_AUTOLOAD_CTRL,
    cache_l2_cache_autoload_sct0_addr: CACHE_L2_CACHE_AUTOLOAD_SCT0_ADDR,
    cache_l2_cache_autoload_sct0_size: CACHE_L2_CACHE_AUTOLOAD_SCT0_SIZE,
    cache_l2_cache_autoload_sct1_addr: CACHE_L2_CACHE_AUTOLOAD_SCT1_ADDR,
    cache_l2_cache_autoload_sct1_size: CACHE_L2_CACHE_AUTOLOAD_SCT1_SIZE,
    cache_l2_cache_autoload_sct2_addr: CACHE_L2_CACHE_AUTOLOAD_SCT2_ADDR,
    cache_l2_cache_autoload_sct2_size: CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE,
    cache_l2_cache_autoload_sct3_addr: CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR,
    cache_l2_cache_autoload_sct3_size: CACHE_L2_CACHE_AUTOLOAD_SCT3_SIZE,
    cache_l2_cache_acs_cnt_int_ena: CACHE_L2_CACHE_ACS_CNT_INT_ENA,
    cache_l2_cache_acs_cnt_int_clr: CACHE_L2_CACHE_ACS_CNT_INT_CLR,
    cache_l2_cache_acs_cnt_int_raw: CACHE_L2_CACHE_ACS_CNT_INT_RAW,
    cache_l2_cache_acs_cnt_int_st: CACHE_L2_CACHE_ACS_CNT_INT_ST,
    cache_l2_cache_acs_fail_ctrl: CACHE_L2_CACHE_ACS_FAIL_CTRL,
    cache_l2_cache_acs_fail_int_ena: CACHE_L2_CACHE_ACS_FAIL_INT_ENA,
    cache_l2_cache_acs_fail_int_clr: CACHE_L2_CACHE_ACS_FAIL_INT_CLR,
    cache_l2_cache_acs_fail_int_raw: CACHE_L2_CACHE_ACS_FAIL_INT_RAW,
    cache_l2_cache_acs_fail_int_st: CACHE_L2_CACHE_ACS_FAIL_INT_ST,
    cache_l2_cache_acs_cnt_ctrl: CACHE_L2_CACHE_ACS_CNT_CTRL,
    cache_l2_ibus0_acs_hit_cnt: CACHE_L2_IBUS0_ACS_HIT_CNT,
    cache_l2_ibus0_acs_miss_cnt: CACHE_L2_IBUS0_ACS_MISS_CNT,
    cache_l2_ibus0_acs_conflict_cnt: CACHE_L2_IBUS0_ACS_CONFLICT_CNT,
    cache_l2_ibus0_acs_nxtlvl_rd_cnt: CACHE_L2_IBUS0_ACS_NXTLVL_RD_CNT,
    cache_l2_ibus1_acs_hit_cnt: CACHE_L2_IBUS1_ACS_HIT_CNT,
    cache_l2_ibus1_acs_miss_cnt: CACHE_L2_IBUS1_ACS_MISS_CNT,
    cache_l2_ibus1_acs_conflict_cnt: CACHE_L2_IBUS1_ACS_CONFLICT_CNT,
    cache_l2_ibus1_acs_nxtlvl_rd_cnt: CACHE_L2_IBUS1_ACS_NXTLVL_RD_CNT,
    cache_l2_ibus2_acs_hit_cnt: CACHE_L2_IBUS2_ACS_HIT_CNT,
    cache_l2_ibus2_acs_miss_cnt: CACHE_L2_IBUS2_ACS_MISS_CNT,
    cache_l2_ibus2_acs_conflict_cnt: CACHE_L2_IBUS2_ACS_CONFLICT_CNT,
    cache_l2_ibus2_acs_nxtlvl_rd_cnt: CACHE_L2_IBUS2_ACS_NXTLVL_RD_CNT,
    cache_l2_ibus3_acs_hit_cnt: CACHE_L2_IBUS3_ACS_HIT_CNT,
    cache_l2_ibus3_acs_miss_cnt: CACHE_L2_IBUS3_ACS_MISS_CNT,
    cache_l2_ibus3_acs_conflict_cnt: CACHE_L2_IBUS3_ACS_CONFLICT_CNT,
    cache_l2_ibus3_acs_nxtlvl_rd_cnt: CACHE_L2_IBUS3_ACS_NXTLVL_RD_CNT,
    cache_l2_dbus0_acs_hit_cnt: CACHE_L2_DBUS0_ACS_HIT_CNT,
    cache_l2_dbus0_acs_miss_cnt: CACHE_L2_DBUS0_ACS_MISS_CNT,
    cache_l2_dbus0_acs_conflict_cnt: CACHE_L2_DBUS0_ACS_CONFLICT_CNT,
    cache_l2_dbus0_acs_nxtlvl_rd_cnt: CACHE_L2_DBUS0_ACS_NXTLVL_RD_CNT,
    cache_l2_dbus0_acs_nxtlvl_wr_cnt: CACHE_L2_DBUS0_ACS_NXTLVL_WR_CNT,
    cache_l2_dbus1_acs_hit_cnt: CACHE_L2_DBUS1_ACS_HIT_CNT,
    cache_l2_dbus1_acs_miss_cnt: CACHE_L2_DBUS1_ACS_MISS_CNT,
    cache_l2_dbus1_acs_conflict_cnt: CACHE_L2_DBUS1_ACS_CONFLICT_CNT,
    cache_l2_dbus1_acs_nxtlvl_rd_cnt: CACHE_L2_DBUS1_ACS_NXTLVL_RD_CNT,
    cache_l2_dbus1_acs_nxtlvl_wr_cnt: CACHE_L2_DBUS1_ACS_NXTLVL_WR_CNT,
    cache_l2_dbus2_acs_hit_cnt: CACHE_L2_DBUS2_ACS_HIT_CNT,
    cache_l2_dbus2_acs_miss_cnt: CACHE_L2_DBUS2_ACS_MISS_CNT,
    cache_l2_dbus2_acs_conflict_cnt: CACHE_L2_DBUS2_ACS_CONFLICT_CNT,
    cache_l2_dbus2_acs_nxtlvl_rd_cnt: CACHE_L2_DBUS2_ACS_NXTLVL_RD_CNT,
    cache_l2_dbus2_acs_nxtlvl_wr_cnt: CACHE_L2_DBUS2_ACS_NXTLVL_WR_CNT,
    cache_l2_dbus3_acs_hit_cnt: CACHE_L2_DBUS3_ACS_HIT_CNT,
    cache_l2_dbus3_acs_miss_cnt: CACHE_L2_DBUS3_ACS_MISS_CNT,
    cache_l2_dbus3_acs_conflict_cnt: CACHE_L2_DBUS3_ACS_CONFLICT_CNT,
    cache_l2_dbus3_acs_nxtlvl_rd_cnt: CACHE_L2_DBUS3_ACS_NXTLVL_RD_CNT,
    cache_l2_dbus3_acs_nxtlvl_wr_cnt: CACHE_L2_DBUS3_ACS_NXTLVL_WR_CNT,
    cache_l2_cache_acs_fail_id_attr: CACHE_L2_CACHE_ACS_FAIL_ID_ATTR,
    cache_l2_cache_acs_fail_addr: CACHE_L2_CACHE_ACS_FAIL_ADDR,
    cache_l2_cache_sync_preload_int_ena: CACHE_L2_CACHE_SYNC_PRELOAD_INT_ENA,
    cache_l2_cache_sync_preload_int_clr: CACHE_L2_CACHE_SYNC_PRELOAD_INT_CLR,
    cache_l2_cache_sync_preload_int_raw: CACHE_L2_CACHE_SYNC_PRELOAD_INT_RAW,
    cache_l2_cache_sync_preload_int_st: CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST,
    cache_l2_cache_sync_preload_exception: CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION,
    cache_l2_cache_sync_rst_ctrl: CACHE_L2_CACHE_SYNC_RST_CTRL,
    cache_l2_cache_preload_rst_ctrl: CACHE_L2_CACHE_PRELOAD_RST_CTRL,
    cache_l2_cache_autoload_buf_clr_ctrl: CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL,
    cache_l2_unallocate_buffer_clear: CACHE_L2_UNALLOCATE_BUFFER_CLEAR,
    cache_l2_cache_access_attr_ctrl: CACHE_L2_CACHE_ACCESS_ATTR_CTRL,
    cache_l2_cache_object_ctrl: CACHE_L2_CACHE_OBJECT_CTRL,
    cache_l2_cache_way_object: CACHE_L2_CACHE_WAY_OBJECT,
    cache_l2_cache_addr: CACHE_L2_CACHE_ADDR,
    cache_l2_cache_debug_bus: CACHE_L2_CACHE_DEBUG_BUS,
    cache_level_split1: CACHE_LEVEL_SPLIT1,
    cache_clock_gate: CACHE_CLOCK_GATE,
    cache_trace_ena: CACHE_TRACE_ENA,
    cache_redundancy_sig0: CACHE_REDUNDANCY_SIG0,
    cache_redundancy_sig1: CACHE_REDUNDANCY_SIG1,
    cache_redundancy_sig2: CACHE_REDUNDANCY_SIG2,
    cache_redundancy_sig3: CACHE_REDUNDANCY_SIG3,
    cache_redundancy_sig4: CACHE_REDUNDANCY_SIG4,
    _reserved250: [u8; 0x14],
    cache_date: CACHE_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - L1 instruction Cache(L1-ICache) control register"]
    #[inline(always)]
    pub const fn icache_ctrl(&self) -> &ICACHE_CTRL {
        &self.icache_ctrl
    }
    #[doc = "0x04 - L1 data Cache(L1-Cache) control register"]
    #[inline(always)]
    pub const fn cache_ctrl(&self) -> &CACHE_CTRL {
        &self.cache_ctrl
    }
    #[doc = "0x08 - Bypass Cache configure register"]
    #[inline(always)]
    pub const fn bypass_cache_conf(&self) -> &BYPASS_CACHE_CONF {
        &self.bypass_cache_conf
    }
    #[doc = "0x0c - L1 Cache atomic feature configure register"]
    #[inline(always)]
    pub const fn cache_atomic_conf(&self) -> &CACHE_ATOMIC_CONF {
        &self.cache_atomic_conf
    }
    #[doc = "0x10 - L1 instruction Cache CacheSize mode configure register"]
    #[inline(always)]
    pub const fn icache_cachesize_conf(&self) -> &ICACHE_CACHESIZE_CONF {
        &self.icache_cachesize_conf
    }
    #[doc = "0x14 - L1 instruction Cache BlockSize mode configure register"]
    #[inline(always)]
    pub const fn icache_blocksize_conf(&self) -> &ICACHE_BLOCKSIZE_CONF {
        &self.icache_blocksize_conf
    }
    #[doc = "0x18 - L1 data Cache CacheSize mode configure register"]
    #[inline(always)]
    pub const fn cache_cachesize_conf(&self) -> &CACHE_CACHESIZE_CONF {
        &self.cache_cachesize_conf
    }
    #[doc = "0x1c - L1 data Cache BlockSize mode configure register"]
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
    #[doc = "0x34 - L1 instruction Cache 0 prelock configure register"]
    #[inline(always)]
    pub const fn icache0_prelock_conf(&self) -> &ICACHE0_PRELOCK_CONF {
        &self.icache0_prelock_conf
    }
    #[doc = "0x38 - L1 instruction Cache 0 prelock section0 address configure register"]
    #[inline(always)]
    pub const fn icache0_prelock_sct0_addr(&self) -> &ICACHE0_PRELOCK_SCT0_ADDR {
        &self.icache0_prelock_sct0_addr
    }
    #[doc = "0x3c - L1 instruction Cache 0 prelock section1 address configure register"]
    #[inline(always)]
    pub const fn icache0_prelock_sct1_addr(&self) -> &ICACHE0_PRELOCK_SCT1_ADDR {
        &self.icache0_prelock_sct1_addr
    }
    #[doc = "0x40 - L1 instruction Cache 0 prelock section size configure register"]
    #[inline(always)]
    pub const fn icache0_prelock_sct_size(&self) -> &ICACHE0_PRELOCK_SCT_SIZE {
        &self.icache0_prelock_sct_size
    }
    #[doc = "0x44 - L1 instruction Cache 1 prelock configure register"]
    #[inline(always)]
    pub const fn icache1_prelock_conf(&self) -> &ICACHE1_PRELOCK_CONF {
        &self.icache1_prelock_conf
    }
    #[doc = "0x48 - L1 instruction Cache 1 prelock section0 address configure register"]
    #[inline(always)]
    pub const fn icache1_prelock_sct0_addr(&self) -> &ICACHE1_PRELOCK_SCT0_ADDR {
        &self.icache1_prelock_sct0_addr
    }
    #[doc = "0x4c - L1 instruction Cache 1 prelock section1 address configure register"]
    #[inline(always)]
    pub const fn icache1_prelock_sct1_addr(&self) -> &ICACHE1_PRELOCK_SCT1_ADDR {
        &self.icache1_prelock_sct1_addr
    }
    #[doc = "0x50 - L1 instruction Cache 1 prelock section size configure register"]
    #[inline(always)]
    pub const fn icache1_prelock_sct_size(&self) -> &ICACHE1_PRELOCK_SCT_SIZE {
        &self.icache1_prelock_sct_size
    }
    #[doc = "0x54 - L1 instruction Cache 2 prelock configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_conf(&self) -> &ICACHE2_PRELOCK_CONF {
        &self.icache2_prelock_conf
    }
    #[doc = "0x58 - L1 instruction Cache 2 prelock section0 address configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct0_addr(&self) -> &ICACHE2_PRELOCK_SCT0_ADDR {
        &self.icache2_prelock_sct0_addr
    }
    #[doc = "0x5c - L1 instruction Cache 2 prelock section1 address configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct1_addr(&self) -> &ICACHE2_PRELOCK_SCT1_ADDR {
        &self.icache2_prelock_sct1_addr
    }
    #[doc = "0x60 - L1 instruction Cache 2 prelock section size configure register"]
    #[inline(always)]
    pub const fn icache2_prelock_sct_size(&self) -> &ICACHE2_PRELOCK_SCT_SIZE {
        &self.icache2_prelock_sct_size
    }
    #[doc = "0x64 - L1 instruction Cache 3 prelock configure register"]
    #[inline(always)]
    pub const fn icache3_prelock_conf(&self) -> &ICACHE3_PRELOCK_CONF {
        &self.icache3_prelock_conf
    }
    #[doc = "0x68 - L1 instruction Cache 3 prelock section0 address configure register"]
    #[inline(always)]
    pub const fn icache3_prelock_sct0_addr(&self) -> &ICACHE3_PRELOCK_SCT0_ADDR {
        &self.icache3_prelock_sct0_addr
    }
    #[doc = "0x6c - L1 instruction Cache 3 prelock section1 address configure register"]
    #[inline(always)]
    pub const fn icache3_prelock_sct1_addr(&self) -> &ICACHE3_PRELOCK_SCT1_ADDR {
        &self.icache3_prelock_sct1_addr
    }
    #[doc = "0x70 - L1 instruction Cache 3 prelock section size configure register"]
    #[inline(always)]
    pub const fn icache3_prelock_sct_size(&self) -> &ICACHE3_PRELOCK_SCT_SIZE {
        &self.icache3_prelock_sct_size
    }
    #[doc = "0x74 - L1 Cache prelock configure register"]
    #[inline(always)]
    pub const fn cache_prelock_conf(&self) -> &CACHE_PRELOCK_CONF {
        &self.cache_prelock_conf
    }
    #[doc = "0x78 - L1 Cache prelock section0 address configure register"]
    #[inline(always)]
    pub const fn cache_prelock_sct0_addr(&self) -> &CACHE_PRELOCK_SCT0_ADDR {
        &self.cache_prelock_sct0_addr
    }
    #[doc = "0x7c - L1 Cache prelock section1 address configure register"]
    #[inline(always)]
    pub const fn dcache_prelock_sct1_addr(&self) -> &DCACHE_PRELOCK_SCT1_ADDR {
        &self.dcache_prelock_sct1_addr
    }
    #[doc = "0x80 - L1 Cache prelock section size configure register"]
    #[inline(always)]
    pub const fn dcache_prelock_sct_size(&self) -> &DCACHE_PRELOCK_SCT_SIZE {
        &self.dcache_prelock_sct_size
    }
    #[doc = "0x84 - Lock-class (manual lock) operation control register"]
    #[inline(always)]
    pub const fn cache_lock_ctrl(&self) -> &CACHE_LOCK_CTRL {
        &self.cache_lock_ctrl
    }
    #[doc = "0x88 - Lock (manual lock) map configure register"]
    #[inline(always)]
    pub const fn cache_lock_map(&self) -> &CACHE_LOCK_MAP {
        &self.cache_lock_map
    }
    #[doc = "0x8c - Lock (manual lock) address configure register"]
    #[inline(always)]
    pub const fn cache_lock_addr(&self) -> &CACHE_LOCK_ADDR {
        &self.cache_lock_addr
    }
    #[doc = "0x90 - Lock (manual lock) size configure register"]
    #[inline(always)]
    pub const fn cache_lock_size(&self) -> &CACHE_LOCK_SIZE {
        &self.cache_lock_size
    }
    #[doc = "0x94 - Sync-class operation control register"]
    #[inline(always)]
    pub const fn cache_sync_ctrl(&self) -> &CACHE_SYNC_CTRL {
        &self.cache_sync_ctrl
    }
    #[doc = "0x98 - Sync map configure register"]
    #[inline(always)]
    pub const fn cache_sync_map(&self) -> &CACHE_SYNC_MAP {
        &self.cache_sync_map
    }
    #[doc = "0x9c - Sync address configure register"]
    #[inline(always)]
    pub const fn cache_sync_addr(&self) -> &CACHE_SYNC_ADDR {
        &self.cache_sync_addr
    }
    #[doc = "0xa0 - Sync size configure register"]
    #[inline(always)]
    pub const fn cache_sync_size(&self) -> &CACHE_SYNC_SIZE {
        &self.cache_sync_size
    }
    #[doc = "0xa4 - L1 instruction Cache 0 preload-operation control register"]
    #[inline(always)]
    pub const fn icache0_preload_ctrl(&self) -> &ICACHE0_PRELOAD_CTRL {
        &self.icache0_preload_ctrl
    }
    #[doc = "0xa8 - L1 instruction Cache 0 preload address configure register"]
    #[inline(always)]
    pub const fn icache0_preload_addr(&self) -> &ICACHE0_PRELOAD_ADDR {
        &self.icache0_preload_addr
    }
    #[doc = "0xac - L1 instruction Cache 0 preload size configure register"]
    #[inline(always)]
    pub const fn icache0_preload_size(&self) -> &ICACHE0_PRELOAD_SIZE {
        &self.icache0_preload_size
    }
    #[doc = "0xb0 - L1 instruction Cache 1 preload-operation control register"]
    #[inline(always)]
    pub const fn icache1_preload_ctrl(&self) -> &ICACHE1_PRELOAD_CTRL {
        &self.icache1_preload_ctrl
    }
    #[doc = "0xb4 - L1 instruction Cache 1 preload address configure register"]
    #[inline(always)]
    pub const fn icache1_preload_addr(&self) -> &ICACHE1_PRELOAD_ADDR {
        &self.icache1_preload_addr
    }
    #[doc = "0xb8 - L1 instruction Cache 1 preload size configure register"]
    #[inline(always)]
    pub const fn icache1_preload_size(&self) -> &ICACHE1_PRELOAD_SIZE {
        &self.icache1_preload_size
    }
    #[doc = "0xbc - L1 instruction Cache 2 preload-operation control register"]
    #[inline(always)]
    pub const fn icache2_preload_ctrl(&self) -> &ICACHE2_PRELOAD_CTRL {
        &self.icache2_preload_ctrl
    }
    #[doc = "0xc0 - L1 instruction Cache 2 preload address configure register"]
    #[inline(always)]
    pub const fn icache2_preload_addr(&self) -> &ICACHE2_PRELOAD_ADDR {
        &self.icache2_preload_addr
    }
    #[doc = "0xc4 - L1 instruction Cache 2 preload size configure register"]
    #[inline(always)]
    pub const fn icache2_preload_size(&self) -> &ICACHE2_PRELOAD_SIZE {
        &self.icache2_preload_size
    }
    #[doc = "0xc8 - L1 instruction Cache 3 preload-operation control register"]
    #[inline(always)]
    pub const fn icache3_preload_ctrl(&self) -> &ICACHE3_PRELOAD_CTRL {
        &self.icache3_preload_ctrl
    }
    #[doc = "0xcc - L1 instruction Cache 3 preload address configure register"]
    #[inline(always)]
    pub const fn icache3_preload_addr(&self) -> &ICACHE3_PRELOAD_ADDR {
        &self.icache3_preload_addr
    }
    #[doc = "0xd0 - L1 instruction Cache 3 preload size configure register"]
    #[inline(always)]
    pub const fn icache3_preload_size(&self) -> &ICACHE3_PRELOAD_SIZE {
        &self.icache3_preload_size
    }
    #[doc = "0xd4 - L1 Cache preload-operation control register"]
    #[inline(always)]
    pub const fn cache_preload_ctrl(&self) -> &CACHE_PRELOAD_CTRL {
        &self.cache_preload_ctrl
    }
    #[doc = "0xd8 - L1 Cache preload address configure register"]
    #[inline(always)]
    pub const fn dcache_preload_addr(&self) -> &DCACHE_PRELOAD_ADDR {
        &self.dcache_preload_addr
    }
    #[doc = "0xdc - L1 Cache preload size configure register"]
    #[inline(always)]
    pub const fn dcache_preload_size(&self) -> &DCACHE_PRELOAD_SIZE {
        &self.dcache_preload_size
    }
    #[doc = "0xe0 - L1 instruction Cache 0 autoload-operation control register"]
    #[inline(always)]
    pub const fn icache0_autoload_ctrl(&self) -> &ICACHE0_AUTOLOAD_CTRL {
        &self.icache0_autoload_ctrl
    }
    #[doc = "0xe4 - L1 instruction Cache 0 autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn icache0_autoload_sct0_addr(&self) -> &ICACHE0_AUTOLOAD_SCT0_ADDR {
        &self.icache0_autoload_sct0_addr
    }
    #[doc = "0xe8 - L1 instruction Cache 0 autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn icache0_autoload_sct0_size(&self) -> &ICACHE0_AUTOLOAD_SCT0_SIZE {
        &self.icache0_autoload_sct0_size
    }
    #[doc = "0xec - L1 instruction Cache 0 autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn icache0_autoload_sct1_addr(&self) -> &ICACHE0_AUTOLOAD_SCT1_ADDR {
        &self.icache0_autoload_sct1_addr
    }
    #[doc = "0xf0 - L1 instruction Cache 0 autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn icache0_autoload_sct1_size(&self) -> &ICACHE0_AUTOLOAD_SCT1_SIZE {
        &self.icache0_autoload_sct1_size
    }
    #[doc = "0xf4 - L1 instruction Cache 1 autoload-operation control register"]
    #[inline(always)]
    pub const fn icache1_autoload_ctrl(&self) -> &ICACHE1_AUTOLOAD_CTRL {
        &self.icache1_autoload_ctrl
    }
    #[doc = "0xf8 - L1 instruction Cache 1 autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn icache1_autoload_sct0_addr(&self) -> &ICACHE1_AUTOLOAD_SCT0_ADDR {
        &self.icache1_autoload_sct0_addr
    }
    #[doc = "0xfc - L1 instruction Cache 1 autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn icache1_autoload_sct0_size(&self) -> &ICACHE1_AUTOLOAD_SCT0_SIZE {
        &self.icache1_autoload_sct0_size
    }
    #[doc = "0x100 - L1 instruction Cache 1 autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn icache1_autoload_sct1_addr(&self) -> &ICACHE1_AUTOLOAD_SCT1_ADDR {
        &self.icache1_autoload_sct1_addr
    }
    #[doc = "0x104 - L1 instruction Cache 1 autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn icache1_autoload_sct1_size(&self) -> &ICACHE1_AUTOLOAD_SCT1_SIZE {
        &self.icache1_autoload_sct1_size
    }
    #[doc = "0x108 - L1 instruction Cache 2 autoload-operation control register"]
    #[inline(always)]
    pub const fn icache2_autoload_ctrl(&self) -> &ICACHE2_AUTOLOAD_CTRL {
        &self.icache2_autoload_ctrl
    }
    #[doc = "0x10c - L1 instruction Cache 2 autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct0_addr(&self) -> &ICACHE2_AUTOLOAD_SCT0_ADDR {
        &self.icache2_autoload_sct0_addr
    }
    #[doc = "0x110 - L1 instruction Cache 2 autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct0_size(&self) -> &ICACHE2_AUTOLOAD_SCT0_SIZE {
        &self.icache2_autoload_sct0_size
    }
    #[doc = "0x114 - L1 instruction Cache 2 autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct1_addr(&self) -> &ICACHE2_AUTOLOAD_SCT1_ADDR {
        &self.icache2_autoload_sct1_addr
    }
    #[doc = "0x118 - L1 instruction Cache 2 autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn icache2_autoload_sct1_size(&self) -> &ICACHE2_AUTOLOAD_SCT1_SIZE {
        &self.icache2_autoload_sct1_size
    }
    #[doc = "0x11c - L1 instruction Cache 3 autoload-operation control register"]
    #[inline(always)]
    pub const fn icache3_autoload_ctrl(&self) -> &ICACHE3_AUTOLOAD_CTRL {
        &self.icache3_autoload_ctrl
    }
    #[doc = "0x120 - L1 instruction Cache 3 autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn icache3_autoload_sct0_addr(&self) -> &ICACHE3_AUTOLOAD_SCT0_ADDR {
        &self.icache3_autoload_sct0_addr
    }
    #[doc = "0x124 - L1 instruction Cache 3 autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn icache3_autoload_sct0_size(&self) -> &ICACHE3_AUTOLOAD_SCT0_SIZE {
        &self.icache3_autoload_sct0_size
    }
    #[doc = "0x128 - L1 instruction Cache 3 autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn icache3_autoload_sct1_addr(&self) -> &ICACHE3_AUTOLOAD_SCT1_ADDR {
        &self.icache3_autoload_sct1_addr
    }
    #[doc = "0x12c - L1 instruction Cache 3 autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn icache3_autoload_sct1_size(&self) -> &ICACHE3_AUTOLOAD_SCT1_SIZE {
        &self.icache3_autoload_sct1_size
    }
    #[doc = "0x130 - L1 Cache autoload-operation control register"]
    #[inline(always)]
    pub const fn cache_autoload_ctrl(&self) -> &CACHE_AUTOLOAD_CTRL {
        &self.cache_autoload_ctrl
    }
    #[doc = "0x134 - L1 Cache autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct0_addr(&self) -> &CACHE_AUTOLOAD_SCT0_ADDR {
        &self.cache_autoload_sct0_addr
    }
    #[doc = "0x138 - L1 Cache autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct0_size(&self) -> &CACHE_AUTOLOAD_SCT0_SIZE {
        &self.cache_autoload_sct0_size
    }
    #[doc = "0x13c - L1 Cache autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct1_addr(&self) -> &CACHE_AUTOLOAD_SCT1_ADDR {
        &self.cache_autoload_sct1_addr
    }
    #[doc = "0x140 - L1 Cache autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct1_size(&self) -> &CACHE_AUTOLOAD_SCT1_SIZE {
        &self.cache_autoload_sct1_size
    }
    #[doc = "0x144 - L1 Cache autoload section 2 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct2_addr(&self) -> &CACHE_AUTOLOAD_SCT2_ADDR {
        &self.cache_autoload_sct2_addr
    }
    #[doc = "0x148 - L1 Cache autoload section 2 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct2_size(&self) -> &CACHE_AUTOLOAD_SCT2_SIZE {
        &self.cache_autoload_sct2_size
    }
    #[doc = "0x14c - L1 Cache autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct3_addr(&self) -> &CACHE_AUTOLOAD_SCT3_ADDR {
        &self.cache_autoload_sct3_addr
    }
    #[doc = "0x150 - L1 Cache autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn cache_autoload_sct3_size(&self) -> &CACHE_AUTOLOAD_SCT3_SIZE {
        &self.cache_autoload_sct3_size
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
    #[doc = "0x16c - L1-Cache Access Fail Interrupt clear register"]
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
    #[doc = "0x17c - L1-ICache bus0 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus0_acs_hit_cnt(&self) -> &IBUS0_ACS_HIT_CNT {
        &self.ibus0_acs_hit_cnt
    }
    #[doc = "0x180 - L1-ICache bus0 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus0_acs_miss_cnt(&self) -> &IBUS0_ACS_MISS_CNT {
        &self.ibus0_acs_miss_cnt
    }
    #[doc = "0x184 - L1-ICache bus0 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus0_acs_conflict_cnt(&self) -> &IBUS0_ACS_CONFLICT_CNT {
        &self.ibus0_acs_conflict_cnt
    }
    #[doc = "0x188 - L1-ICache bus0 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus0_acs_nxtlvl_rd_cnt(&self) -> &IBUS0_ACS_NXTLVL_RD_CNT {
        &self.ibus0_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x18c - L1-ICache bus1 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus1_acs_hit_cnt(&self) -> &IBUS1_ACS_HIT_CNT {
        &self.ibus1_acs_hit_cnt
    }
    #[doc = "0x190 - L1-ICache bus1 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus1_acs_miss_cnt(&self) -> &IBUS1_ACS_MISS_CNT {
        &self.ibus1_acs_miss_cnt
    }
    #[doc = "0x194 - L1-ICache bus1 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus1_acs_conflict_cnt(&self) -> &IBUS1_ACS_CONFLICT_CNT {
        &self.ibus1_acs_conflict_cnt
    }
    #[doc = "0x198 - L1-ICache bus1 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus1_acs_nxtlvl_rd_cnt(&self) -> &IBUS1_ACS_NXTLVL_RD_CNT {
        &self.ibus1_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x19c - L1-ICache bus2 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_hit_cnt(&self) -> &IBUS2_ACS_HIT_CNT {
        &self.ibus2_acs_hit_cnt
    }
    #[doc = "0x1a0 - L1-ICache bus2 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_miss_cnt(&self) -> &IBUS2_ACS_MISS_CNT {
        &self.ibus2_acs_miss_cnt
    }
    #[doc = "0x1a4 - L1-ICache bus2 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_conflict_cnt(&self) -> &IBUS2_ACS_CONFLICT_CNT {
        &self.ibus2_acs_conflict_cnt
    }
    #[doc = "0x1a8 - L1-ICache bus2 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus2_acs_nxtlvl_rd_cnt(&self) -> &IBUS2_ACS_NXTLVL_RD_CNT {
        &self.ibus2_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1ac - L1-ICache bus3 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_hit_cnt(&self) -> &IBUS3_ACS_HIT_CNT {
        &self.ibus3_acs_hit_cnt
    }
    #[doc = "0x1b0 - L1-ICache bus3 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_miss_cnt(&self) -> &IBUS3_ACS_MISS_CNT {
        &self.ibus3_acs_miss_cnt
    }
    #[doc = "0x1b4 - L1-ICache bus3 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_conflict_cnt(&self) -> &IBUS3_ACS_CONFLICT_CNT {
        &self.ibus3_acs_conflict_cnt
    }
    #[doc = "0x1b8 - L1-ICache bus3 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn ibus3_acs_nxtlvl_rd_cnt(&self) -> &IBUS3_ACS_NXTLVL_RD_CNT {
        &self.ibus3_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1bc - L1-Cache bus0 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_hit_cnt(&self) -> &BUS0_ACS_HIT_CNT {
        &self.bus0_acs_hit_cnt
    }
    #[doc = "0x1c0 - L1-Cache bus0 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_miss_cnt(&self) -> &BUS0_ACS_MISS_CNT {
        &self.bus0_acs_miss_cnt
    }
    #[doc = "0x1c4 - L1-Cache bus0 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn bus0_acs_conflict_cnt(&self) -> &BUS0_ACS_CONFLICT_CNT {
        &self.bus0_acs_conflict_cnt
    }
    #[doc = "0x1c8 - L1-Cache bus0 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus0_acs_nxtlvl_rd_cnt(&self) -> &DBUS0_ACS_NXTLVL_RD_CNT {
        &self.dbus0_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1cc - L1-DCache bus0 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus0_acs_nxtlvl_wr_cnt(&self) -> &DBUS0_ACS_NXTLVL_WR_CNT {
        &self.dbus0_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x1d0 - L1-Cache bus1 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_hit_cnt(&self) -> &BUS1_ACS_HIT_CNT {
        &self.bus1_acs_hit_cnt
    }
    #[doc = "0x1d4 - L1-Cache bus1 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_miss_cnt(&self) -> &BUS1_ACS_MISS_CNT {
        &self.bus1_acs_miss_cnt
    }
    #[doc = "0x1d8 - L1-Cache bus1 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn bus1_acs_conflict_cnt(&self) -> &BUS1_ACS_CONFLICT_CNT {
        &self.bus1_acs_conflict_cnt
    }
    #[doc = "0x1dc - L1-DCache bus1 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus1_acs_nxtlvl_rd_cnt(&self) -> &DBUS1_ACS_NXTLVL_RD_CNT {
        &self.dbus1_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1e0 - L1-DCache bus1 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus1_acs_nxtlvl_wr_cnt(&self) -> &DBUS1_ACS_NXTLVL_WR_CNT {
        &self.dbus1_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x1e4 - L1-DCache bus2 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn dbus2_acs_hit_cnt(&self) -> &DBUS2_ACS_HIT_CNT {
        &self.dbus2_acs_hit_cnt
    }
    #[doc = "0x1e8 - L1-DCache bus2 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn dbus2_acs_miss_cnt(&self) -> &DBUS2_ACS_MISS_CNT {
        &self.dbus2_acs_miss_cnt
    }
    #[doc = "0x1ec - L1-DCache bus2 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn dbus2_acs_conflict_cnt(&self) -> &DBUS2_ACS_CONFLICT_CNT {
        &self.dbus2_acs_conflict_cnt
    }
    #[doc = "0x1f0 - L1-DCache bus2 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus2_acs_nxtlvl_rd_cnt(&self) -> &DBUS2_ACS_NXTLVL_RD_CNT {
        &self.dbus2_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x1f4 - L1-DCache bus2 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus2_acs_nxtlvl_wr_cnt(&self) -> &DBUS2_ACS_NXTLVL_WR_CNT {
        &self.dbus2_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x1f8 - L1-DCache bus3 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn dbus3_acs_hit_cnt(&self) -> &DBUS3_ACS_HIT_CNT {
        &self.dbus3_acs_hit_cnt
    }
    #[doc = "0x1fc - L1-DCache bus3 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn dbus3_acs_miss_cnt(&self) -> &DBUS3_ACS_MISS_CNT {
        &self.dbus3_acs_miss_cnt
    }
    #[doc = "0x200 - L1-DCache bus3 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn dbus3_acs_conflict_cnt(&self) -> &DBUS3_ACS_CONFLICT_CNT {
        &self.dbus3_acs_conflict_cnt
    }
    #[doc = "0x204 - L1-DCache bus3 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn dbus3_acs_nxtlvl_rd_cnt(&self) -> &DBUS3_ACS_NXTLVL_RD_CNT {
        &self.dbus3_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x208 - L1-DCache bus3 WB-Access Counter register"]
    #[inline(always)]
    pub const fn dbus3_acs_nxtlvl_wr_cnt(&self) -> &DBUS3_ACS_NXTLVL_WR_CNT {
        &self.dbus3_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x20c - L1-ICache0 Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn icache0_acs_fail_id_attr(&self) -> &ICACHE0_ACS_FAIL_ID_ATTR {
        &self.icache0_acs_fail_id_attr
    }
    #[doc = "0x210 - L1-ICache0 Access Fail Address information register"]
    #[inline(always)]
    pub const fn icache0_acs_fail_addr(&self) -> &ICACHE0_ACS_FAIL_ADDR {
        &self.icache0_acs_fail_addr
    }
    #[doc = "0x214 - L1-ICache0 Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn icache1_acs_fail_id_attr(&self) -> &ICACHE1_ACS_FAIL_ID_ATTR {
        &self.icache1_acs_fail_id_attr
    }
    #[doc = "0x218 - L1-ICache0 Access Fail Address information register"]
    #[inline(always)]
    pub const fn icache1_acs_fail_addr(&self) -> &ICACHE1_ACS_FAIL_ADDR {
        &self.icache1_acs_fail_addr
    }
    #[doc = "0x21c - L1-ICache0 Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn icache2_acs_fail_id_attr(&self) -> &ICACHE2_ACS_FAIL_ID_ATTR {
        &self.icache2_acs_fail_id_attr
    }
    #[doc = "0x220 - L1-ICache0 Access Fail Address information register"]
    #[inline(always)]
    pub const fn icache2_acs_fail_addr(&self) -> &ICACHE2_ACS_FAIL_ADDR {
        &self.icache2_acs_fail_addr
    }
    #[doc = "0x224 - L1-ICache0 Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn icache3_acs_fail_id_attr(&self) -> &ICACHE3_ACS_FAIL_ID_ATTR {
        &self.icache3_acs_fail_id_attr
    }
    #[doc = "0x228 - L1-ICache0 Access Fail Address information register"]
    #[inline(always)]
    pub const fn icache3_acs_fail_addr(&self) -> &ICACHE3_ACS_FAIL_ADDR {
        &self.icache3_acs_fail_addr
    }
    #[doc = "0x22c - L1-Cache Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn dcache_acs_fail_id_attr(&self) -> &DCACHE_ACS_FAIL_ID_ATTR {
        &self.dcache_acs_fail_id_attr
    }
    #[doc = "0x230 - L1-Cache Access Fail Address information register"]
    #[inline(always)]
    pub const fn dcache_acs_fail_addr(&self) -> &DCACHE_ACS_FAIL_ADDR {
        &self.dcache_acs_fail_addr
    }
    #[doc = "0x234 - L1-Cache Access Fail Interrupt enable register"]
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
    #[doc = "0x240 - L1-Cache Access Fail Interrupt status register"]
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
    #[doc = "0x268 - USED TO SPLIT L1 CACHE AND L2 CACHE"]
    #[inline(always)]
    pub const fn cache_level_split0(&self) -> &CACHE_LEVEL_SPLIT0 {
        &self.cache_level_split0
    }
    #[doc = "0x26c - L2 Cache(L2-Cache) control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_ctrl(&self) -> &CACHE_L2_CACHE_CTRL {
        &self.cache_l2_cache_ctrl
    }
    #[doc = "0x270 - Bypass Cache configure register"]
    #[inline(always)]
    pub const fn cache_l2_bypass_cache_conf(&self) -> &CACHE_L2_BYPASS_CACHE_CONF {
        &self.cache_l2_bypass_cache_conf
    }
    #[doc = "0x274 - L2 Cache CacheSize mode configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_cachesize_conf(&self) -> &CACHE_L2_CACHE_CACHESIZE_CONF {
        &self.cache_l2_cache_cachesize_conf
    }
    #[doc = "0x278 - L2 Cache BlockSize mode configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_blocksize_conf(&self) -> &CACHE_L2_CACHE_BLOCKSIZE_CONF {
        &self.cache_l2_cache_blocksize_conf
    }
    #[doc = "0x27c - Cache wrap around control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_wrap_around_ctrl(&self) -> &CACHE_L2_CACHE_WRAP_AROUND_CTRL {
        &self.cache_l2_cache_wrap_around_ctrl
    }
    #[doc = "0x280 - Cache wrap around control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_miss_access_ctrl(&self) -> &CACHE_L2_CACHE_MISS_ACCESS_CTRL {
        &self.cache_l2_cache_miss_access_ctrl
    }
    #[doc = "0x284 - Cache Freeze control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_freeze_ctrl(&self) -> &CACHE_L2_CACHE_FREEZE_CTRL {
        &self.cache_l2_cache_freeze_ctrl
    }
    #[doc = "0x288 - Cache data memory access configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_data_mem_acs_conf(&self) -> &CACHE_L2_CACHE_DATA_MEM_ACS_CONF {
        &self.cache_l2_cache_data_mem_acs_conf
    }
    #[doc = "0x28c - Cache tag memory access configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_tag_mem_acs_conf(&self) -> &CACHE_L2_CACHE_TAG_MEM_ACS_CONF {
        &self.cache_l2_cache_tag_mem_acs_conf
    }
    #[doc = "0x290 - L2 Cache prelock configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_prelock_conf(&self) -> &CACHE_L2_CACHE_PRELOCK_CONF {
        &self.cache_l2_cache_prelock_conf
    }
    #[doc = "0x294 - L2 Cache prelock section0 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_prelock_sct0_addr(&self) -> &CACHE_L2_CACHE_PRELOCK_SCT0_ADDR {
        &self.cache_l2_cache_prelock_sct0_addr
    }
    #[doc = "0x298 - L2 Cache prelock section1 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_prelock_sct1_addr(&self) -> &CACHE_L2_CACHE_PRELOCK_SCT1_ADDR {
        &self.cache_l2_cache_prelock_sct1_addr
    }
    #[doc = "0x29c - L2 Cache prelock section size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_prelock_sct_size(&self) -> &CACHE_L2_CACHE_PRELOCK_SCT_SIZE {
        &self.cache_l2_cache_prelock_sct_size
    }
    #[doc = "0x2a0 - L2 Cache preload-operation control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_preload_ctrl(&self) -> &CACHE_L2_CACHE_PRELOAD_CTRL {
        &self.cache_l2_cache_preload_ctrl
    }
    #[doc = "0x2a4 - L2 Cache preload address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_preload_addr(&self) -> &CACHE_L2_CACHE_PRELOAD_ADDR {
        &self.cache_l2_cache_preload_addr
    }
    #[doc = "0x2a8 - L2 Cache preload size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_preload_size(&self) -> &CACHE_L2_CACHE_PRELOAD_SIZE {
        &self.cache_l2_cache_preload_size
    }
    #[doc = "0x2ac - L2 Cache autoload-operation control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_ctrl(&self) -> &CACHE_L2_CACHE_AUTOLOAD_CTRL {
        &self.cache_l2_cache_autoload_ctrl
    }
    #[doc = "0x2b0 - L2 Cache autoload section 0 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct0_addr(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT0_ADDR {
        &self.cache_l2_cache_autoload_sct0_addr
    }
    #[doc = "0x2b4 - L2 Cache autoload section 0 size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct0_size(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT0_SIZE {
        &self.cache_l2_cache_autoload_sct0_size
    }
    #[doc = "0x2b8 - L2 Cache autoload section 1 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct1_addr(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT1_ADDR {
        &self.cache_l2_cache_autoload_sct1_addr
    }
    #[doc = "0x2bc - L2 Cache autoload section 1 size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct1_size(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT1_SIZE {
        &self.cache_l2_cache_autoload_sct1_size
    }
    #[doc = "0x2c0 - L2 Cache autoload section 2 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct2_addr(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT2_ADDR {
        &self.cache_l2_cache_autoload_sct2_addr
    }
    #[doc = "0x2c4 - L2 Cache autoload section 2 size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct2_size(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE {
        &self.cache_l2_cache_autoload_sct2_size
    }
    #[doc = "0x2c8 - L2 Cache autoload section 3 address configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct3_addr(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR {
        &self.cache_l2_cache_autoload_sct3_addr
    }
    #[doc = "0x2cc - L2 Cache autoload section 3 size configure register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_sct3_size(&self) -> &CACHE_L2_CACHE_AUTOLOAD_SCT3_SIZE {
        &self.cache_l2_cache_autoload_sct3_size
    }
    #[doc = "0x2d0 - Cache Access Counter Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_cnt_int_ena(&self) -> &CACHE_L2_CACHE_ACS_CNT_INT_ENA {
        &self.cache_l2_cache_acs_cnt_int_ena
    }
    #[doc = "0x2d4 - Cache Access Counter Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_cnt_int_clr(&self) -> &CACHE_L2_CACHE_ACS_CNT_INT_CLR {
        &self.cache_l2_cache_acs_cnt_int_clr
    }
    #[doc = "0x2d8 - Cache Access Counter Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_cnt_int_raw(&self) -> &CACHE_L2_CACHE_ACS_CNT_INT_RAW {
        &self.cache_l2_cache_acs_cnt_int_raw
    }
    #[doc = "0x2dc - Cache Access Counter Interrupt status register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_cnt_int_st(&self) -> &CACHE_L2_CACHE_ACS_CNT_INT_ST {
        &self.cache_l2_cache_acs_cnt_int_st
    }
    #[doc = "0x2e0 - Cache Access Fail Configuration register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_ctrl(&self) -> &CACHE_L2_CACHE_ACS_FAIL_CTRL {
        &self.cache_l2_cache_acs_fail_ctrl
    }
    #[doc = "0x2e4 - Cache Access Fail Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_int_ena(&self) -> &CACHE_L2_CACHE_ACS_FAIL_INT_ENA {
        &self.cache_l2_cache_acs_fail_int_ena
    }
    #[doc = "0x2e8 - L1-Cache Access Fail Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_int_clr(&self) -> &CACHE_L2_CACHE_ACS_FAIL_INT_CLR {
        &self.cache_l2_cache_acs_fail_int_clr
    }
    #[doc = "0x2ec - Cache Access Fail Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_int_raw(&self) -> &CACHE_L2_CACHE_ACS_FAIL_INT_RAW {
        &self.cache_l2_cache_acs_fail_int_raw
    }
    #[doc = "0x2f0 - Cache Access Fail Interrupt status register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_int_st(&self) -> &CACHE_L2_CACHE_ACS_FAIL_INT_ST {
        &self.cache_l2_cache_acs_fail_int_st
    }
    #[doc = "0x2f4 - Cache Access Counter enable and clear register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_cnt_ctrl(&self) -> &CACHE_L2_CACHE_ACS_CNT_CTRL {
        &self.cache_l2_cache_acs_cnt_ctrl
    }
    #[doc = "0x2f8 - L2-Cache bus0 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus0_acs_hit_cnt(&self) -> &CACHE_L2_IBUS0_ACS_HIT_CNT {
        &self.cache_l2_ibus0_acs_hit_cnt
    }
    #[doc = "0x2fc - L2-Cache bus0 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus0_acs_miss_cnt(&self) -> &CACHE_L2_IBUS0_ACS_MISS_CNT {
        &self.cache_l2_ibus0_acs_miss_cnt
    }
    #[doc = "0x300 - L2-Cache bus0 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus0_acs_conflict_cnt(&self) -> &CACHE_L2_IBUS0_ACS_CONFLICT_CNT {
        &self.cache_l2_ibus0_acs_conflict_cnt
    }
    #[doc = "0x304 - L2-Cache bus0 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus0_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_IBUS0_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_ibus0_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x308 - L2-Cache bus1 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus1_acs_hit_cnt(&self) -> &CACHE_L2_IBUS1_ACS_HIT_CNT {
        &self.cache_l2_ibus1_acs_hit_cnt
    }
    #[doc = "0x30c - L2-Cache bus1 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus1_acs_miss_cnt(&self) -> &CACHE_L2_IBUS1_ACS_MISS_CNT {
        &self.cache_l2_ibus1_acs_miss_cnt
    }
    #[doc = "0x310 - L2-Cache bus1 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus1_acs_conflict_cnt(&self) -> &CACHE_L2_IBUS1_ACS_CONFLICT_CNT {
        &self.cache_l2_ibus1_acs_conflict_cnt
    }
    #[doc = "0x314 - L2-Cache bus1 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus1_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_IBUS1_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_ibus1_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x318 - L2-Cache bus2 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus2_acs_hit_cnt(&self) -> &CACHE_L2_IBUS2_ACS_HIT_CNT {
        &self.cache_l2_ibus2_acs_hit_cnt
    }
    #[doc = "0x31c - L2-Cache bus2 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus2_acs_miss_cnt(&self) -> &CACHE_L2_IBUS2_ACS_MISS_CNT {
        &self.cache_l2_ibus2_acs_miss_cnt
    }
    #[doc = "0x320 - L2-Cache bus2 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus2_acs_conflict_cnt(&self) -> &CACHE_L2_IBUS2_ACS_CONFLICT_CNT {
        &self.cache_l2_ibus2_acs_conflict_cnt
    }
    #[doc = "0x324 - L2-Cache bus2 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus2_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_IBUS2_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_ibus2_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x328 - L2-Cache bus3 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus3_acs_hit_cnt(&self) -> &CACHE_L2_IBUS3_ACS_HIT_CNT {
        &self.cache_l2_ibus3_acs_hit_cnt
    }
    #[doc = "0x32c - L2-Cache bus3 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus3_acs_miss_cnt(&self) -> &CACHE_L2_IBUS3_ACS_MISS_CNT {
        &self.cache_l2_ibus3_acs_miss_cnt
    }
    #[doc = "0x330 - L2-Cache bus3 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus3_acs_conflict_cnt(&self) -> &CACHE_L2_IBUS3_ACS_CONFLICT_CNT {
        &self.cache_l2_ibus3_acs_conflict_cnt
    }
    #[doc = "0x334 - L2-Cache bus3 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_ibus3_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_IBUS3_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_ibus3_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x338 - L2-Cache bus0 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus0_acs_hit_cnt(&self) -> &CACHE_L2_DBUS0_ACS_HIT_CNT {
        &self.cache_l2_dbus0_acs_hit_cnt
    }
    #[doc = "0x33c - L2-Cache bus0 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus0_acs_miss_cnt(&self) -> &CACHE_L2_DBUS0_ACS_MISS_CNT {
        &self.cache_l2_dbus0_acs_miss_cnt
    }
    #[doc = "0x340 - L2-Cache bus0 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus0_acs_conflict_cnt(&self) -> &CACHE_L2_DBUS0_ACS_CONFLICT_CNT {
        &self.cache_l2_dbus0_acs_conflict_cnt
    }
    #[doc = "0x344 - L2-Cache bus0 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus0_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_DBUS0_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_dbus0_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x348 - L2-Cache bus0 WB-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus0_acs_nxtlvl_wr_cnt(&self) -> &CACHE_L2_DBUS0_ACS_NXTLVL_WR_CNT {
        &self.cache_l2_dbus0_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x34c - L2-Cache bus1 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus1_acs_hit_cnt(&self) -> &CACHE_L2_DBUS1_ACS_HIT_CNT {
        &self.cache_l2_dbus1_acs_hit_cnt
    }
    #[doc = "0x350 - L2-Cache bus1 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus1_acs_miss_cnt(&self) -> &CACHE_L2_DBUS1_ACS_MISS_CNT {
        &self.cache_l2_dbus1_acs_miss_cnt
    }
    #[doc = "0x354 - L2-Cache bus1 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus1_acs_conflict_cnt(&self) -> &CACHE_L2_DBUS1_ACS_CONFLICT_CNT {
        &self.cache_l2_dbus1_acs_conflict_cnt
    }
    #[doc = "0x358 - L2-Cache bus1 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus1_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_DBUS1_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_dbus1_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x35c - L2-Cache bus1 WB-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus1_acs_nxtlvl_wr_cnt(&self) -> &CACHE_L2_DBUS1_ACS_NXTLVL_WR_CNT {
        &self.cache_l2_dbus1_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x360 - L2-Cache bus2 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus2_acs_hit_cnt(&self) -> &CACHE_L2_DBUS2_ACS_HIT_CNT {
        &self.cache_l2_dbus2_acs_hit_cnt
    }
    #[doc = "0x364 - L2-Cache bus2 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus2_acs_miss_cnt(&self) -> &CACHE_L2_DBUS2_ACS_MISS_CNT {
        &self.cache_l2_dbus2_acs_miss_cnt
    }
    #[doc = "0x368 - L2-Cache bus2 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus2_acs_conflict_cnt(&self) -> &CACHE_L2_DBUS2_ACS_CONFLICT_CNT {
        &self.cache_l2_dbus2_acs_conflict_cnt
    }
    #[doc = "0x36c - L2-Cache bus2 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus2_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_DBUS2_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_dbus2_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x370 - L2-Cache bus2 WB-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus2_acs_nxtlvl_wr_cnt(&self) -> &CACHE_L2_DBUS2_ACS_NXTLVL_WR_CNT {
        &self.cache_l2_dbus2_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x374 - L2-Cache bus3 Hit-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus3_acs_hit_cnt(&self) -> &CACHE_L2_DBUS3_ACS_HIT_CNT {
        &self.cache_l2_dbus3_acs_hit_cnt
    }
    #[doc = "0x378 - L2-Cache bus3 Miss-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus3_acs_miss_cnt(&self) -> &CACHE_L2_DBUS3_ACS_MISS_CNT {
        &self.cache_l2_dbus3_acs_miss_cnt
    }
    #[doc = "0x37c - L2-Cache bus3 Conflict-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus3_acs_conflict_cnt(&self) -> &CACHE_L2_DBUS3_ACS_CONFLICT_CNT {
        &self.cache_l2_dbus3_acs_conflict_cnt
    }
    #[doc = "0x380 - L2-Cache bus3 Next-Level-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus3_acs_nxtlvl_rd_cnt(&self) -> &CACHE_L2_DBUS3_ACS_NXTLVL_RD_CNT {
        &self.cache_l2_dbus3_acs_nxtlvl_rd_cnt
    }
    #[doc = "0x384 - L2-Cache bus3 WB-Access Counter register"]
    #[inline(always)]
    pub const fn cache_l2_dbus3_acs_nxtlvl_wr_cnt(&self) -> &CACHE_L2_DBUS3_ACS_NXTLVL_WR_CNT {
        &self.cache_l2_dbus3_acs_nxtlvl_wr_cnt
    }
    #[doc = "0x388 - L2-Cache Access Fail ID/attribution information register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_id_attr(&self) -> &CACHE_L2_CACHE_ACS_FAIL_ID_ATTR {
        &self.cache_l2_cache_acs_fail_id_attr
    }
    #[doc = "0x38c - L2-Cache Access Fail Address information register"]
    #[inline(always)]
    pub const fn cache_l2_cache_acs_fail_addr(&self) -> &CACHE_L2_CACHE_ACS_FAIL_ADDR {
        &self.cache_l2_cache_acs_fail_addr
    }
    #[doc = "0x390 - L1-Cache Access Fail Interrupt enable register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_preload_int_ena(
        &self,
    ) -> &CACHE_L2_CACHE_SYNC_PRELOAD_INT_ENA {
        &self.cache_l2_cache_sync_preload_int_ena
    }
    #[doc = "0x394 - Sync Preload operation Interrupt clear register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_preload_int_clr(
        &self,
    ) -> &CACHE_L2_CACHE_SYNC_PRELOAD_INT_CLR {
        &self.cache_l2_cache_sync_preload_int_clr
    }
    #[doc = "0x398 - Sync Preload operation Interrupt raw register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_preload_int_raw(
        &self,
    ) -> &CACHE_L2_CACHE_SYNC_PRELOAD_INT_RAW {
        &self.cache_l2_cache_sync_preload_int_raw
    }
    #[doc = "0x39c - L1-Cache Access Fail Interrupt status register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_preload_int_st(&self) -> &CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST {
        &self.cache_l2_cache_sync_preload_int_st
    }
    #[doc = "0x3a0 - Cache Sync/Preload Operation exception register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_preload_exception(
        &self,
    ) -> &CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION {
        &self.cache_l2_cache_sync_preload_exception
    }
    #[doc = "0x3a4 - Cache Sync Reset control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_sync_rst_ctrl(&self) -> &CACHE_L2_CACHE_SYNC_RST_CTRL {
        &self.cache_l2_cache_sync_rst_ctrl
    }
    #[doc = "0x3a8 - Cache Preload Reset control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_preload_rst_ctrl(&self) -> &CACHE_L2_CACHE_PRELOAD_RST_CTRL {
        &self.cache_l2_cache_preload_rst_ctrl
    }
    #[doc = "0x3ac - Cache Autoload buffer clear control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_autoload_buf_clr_ctrl(
        &self,
    ) -> &CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL {
        &self.cache_l2_cache_autoload_buf_clr_ctrl
    }
    #[doc = "0x3b0 - Unallocate request buffer clear registers"]
    #[inline(always)]
    pub const fn cache_l2_unallocate_buffer_clear(&self) -> &CACHE_L2_UNALLOCATE_BUFFER_CLEAR {
        &self.cache_l2_unallocate_buffer_clear
    }
    #[doc = "0x3b4 - L2 cache access attribute control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_access_attr_ctrl(&self) -> &CACHE_L2_CACHE_ACCESS_ATTR_CTRL {
        &self.cache_l2_cache_access_attr_ctrl
    }
    #[doc = "0x3b8 - Cache Tag and Data memory Object control register"]
    #[inline(always)]
    pub const fn cache_l2_cache_object_ctrl(&self) -> &CACHE_L2_CACHE_OBJECT_CTRL {
        &self.cache_l2_cache_object_ctrl
    }
    #[doc = "0x3bc - Cache Tag and Data memory way register"]
    #[inline(always)]
    pub const fn cache_l2_cache_way_object(&self) -> &CACHE_L2_CACHE_WAY_OBJECT {
        &self.cache_l2_cache_way_object
    }
    #[doc = "0x3c0 - Cache address register"]
    #[inline(always)]
    pub const fn cache_l2_cache_addr(&self) -> &CACHE_L2_CACHE_ADDR {
        &self.cache_l2_cache_addr
    }
    #[doc = "0x3c4 - Cache Tag/data memory content register"]
    #[inline(always)]
    pub const fn cache_l2_cache_debug_bus(&self) -> &CACHE_L2_CACHE_DEBUG_BUS {
        &self.cache_l2_cache_debug_bus
    }
    #[doc = "0x3c8 - USED TO SPLIT L1 CACHE AND L2 CACHE"]
    #[inline(always)]
    pub const fn cache_level_split1(&self) -> &CACHE_LEVEL_SPLIT1 {
        &self.cache_level_split1
    }
    #[doc = "0x3cc - Clock gate control register"]
    #[inline(always)]
    pub const fn cache_clock_gate(&self) -> &CACHE_CLOCK_GATE {
        &self.cache_clock_gate
    }
    #[doc = "0x3d0 - Clock gate control register"]
    #[inline(always)]
    pub const fn cache_trace_ena(&self) -> &CACHE_TRACE_ENA {
        &self.cache_trace_ena
    }
    #[doc = "0x3d4 - Cache redundancy signal 0 register"]
    #[inline(always)]
    pub const fn cache_redundancy_sig0(&self) -> &CACHE_REDUNDANCY_SIG0 {
        &self.cache_redundancy_sig0
    }
    #[doc = "0x3d8 - Cache redundancy signal 1 register"]
    #[inline(always)]
    pub const fn cache_redundancy_sig1(&self) -> &CACHE_REDUNDANCY_SIG1 {
        &self.cache_redundancy_sig1
    }
    #[doc = "0x3dc - Cache redundancy signal 2 register"]
    #[inline(always)]
    pub const fn cache_redundancy_sig2(&self) -> &CACHE_REDUNDANCY_SIG2 {
        &self.cache_redundancy_sig2
    }
    #[doc = "0x3e0 - Cache redundancy signal 3 register"]
    #[inline(always)]
    pub const fn cache_redundancy_sig3(&self) -> &CACHE_REDUNDANCY_SIG3 {
        &self.cache_redundancy_sig3
    }
    #[doc = "0x3e4 - Cache redundancy signal 0 register"]
    #[inline(always)]
    pub const fn cache_redundancy_sig4(&self) -> &CACHE_REDUNDANCY_SIG4 {
        &self.cache_redundancy_sig4
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn cache_date(&self) -> &CACHE_DATE {
        &self.cache_date
    }
}
#[doc = "ICACHE_CTRL (rw) register accessor: L1 instruction Cache(L1-ICache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_ctrl`] module"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "L1 instruction Cache(L1-ICache) control register"]
pub mod icache_ctrl;
#[doc = "CACHE_CTRL (rw) register accessor: L1 data Cache(L1-Cache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ctrl`] module"]
pub type CACHE_CTRL = crate::Reg<cache_ctrl::CACHE_CTRL_SPEC>;
#[doc = "L1 data Cache(L1-Cache) control register"]
pub mod cache_ctrl;
#[doc = "BYPASS_CACHE_CONF (r) register accessor: Bypass Cache configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`bypass_cache_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bypass_cache_conf`] module"]
pub type BYPASS_CACHE_CONF = crate::Reg<bypass_cache_conf::BYPASS_CACHE_CONF_SPEC>;
#[doc = "Bypass Cache configure register"]
pub mod bypass_cache_conf;
#[doc = "CACHE_ATOMIC_CONF (r) register accessor: L1 Cache atomic feature configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_atomic_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_atomic_conf`] module"]
pub type CACHE_ATOMIC_CONF = crate::Reg<cache_atomic_conf::CACHE_ATOMIC_CONF_SPEC>;
#[doc = "L1 Cache atomic feature configure register"]
pub mod cache_atomic_conf;
#[doc = "ICACHE_CACHESIZE_CONF (r) register accessor: L1 instruction Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_cachesize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_cachesize_conf`] module"]
pub type ICACHE_CACHESIZE_CONF = crate::Reg<icache_cachesize_conf::ICACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L1 instruction Cache CacheSize mode configure register"]
pub mod icache_cachesize_conf;
#[doc = "ICACHE_BLOCKSIZE_CONF (r) register accessor: L1 instruction Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_blocksize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache_blocksize_conf`] module"]
pub type ICACHE_BLOCKSIZE_CONF = crate::Reg<icache_blocksize_conf::ICACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L1 instruction Cache BlockSize mode configure register"]
pub mod icache_blocksize_conf;
#[doc = "CACHE_CACHESIZE_CONF (r) register accessor: L1 data Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_cachesize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_cachesize_conf`] module"]
pub type CACHE_CACHESIZE_CONF = crate::Reg<cache_cachesize_conf::CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L1 data Cache CacheSize mode configure register"]
pub mod cache_cachesize_conf;
#[doc = "CACHE_BLOCKSIZE_CONF (r) register accessor: L1 data Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_blocksize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_blocksize_conf`] module"]
pub type CACHE_BLOCKSIZE_CONF = crate::Reg<cache_blocksize_conf::CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L1 data Cache BlockSize mode configure register"]
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
#[doc = "ICACHE0_PRELOCK_CONF (r) register accessor: L1 instruction Cache 0 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_prelock_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_prelock_conf`] module"]
pub type ICACHE0_PRELOCK_CONF = crate::Reg<icache0_prelock_conf::ICACHE0_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 0 prelock configure register"]
pub mod icache0_prelock_conf;
#[doc = "ICACHE0_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 0 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_prelock_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_prelock_sct0_addr`] module"]
pub type ICACHE0_PRELOCK_SCT0_ADDR =
    crate::Reg<icache0_prelock_sct0_addr::ICACHE0_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section0 address configure register"]
pub mod icache0_prelock_sct0_addr;
#[doc = "ICACHE0_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 0 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_prelock_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_prelock_sct1_addr`] module"]
pub type ICACHE0_PRELOCK_SCT1_ADDR =
    crate::Reg<icache0_prelock_sct1_addr::ICACHE0_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section1 address configure register"]
pub mod icache0_prelock_sct1_addr;
#[doc = "ICACHE0_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 0 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_prelock_sct_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_prelock_sct_size`] module"]
pub type ICACHE0_PRELOCK_SCT_SIZE =
    crate::Reg<icache0_prelock_sct_size::ICACHE0_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section size configure register"]
pub mod icache0_prelock_sct_size;
#[doc = "ICACHE1_PRELOCK_CONF (r) register accessor: L1 instruction Cache 1 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_prelock_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_prelock_conf`] module"]
pub type ICACHE1_PRELOCK_CONF = crate::Reg<icache1_prelock_conf::ICACHE1_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 1 prelock configure register"]
pub mod icache1_prelock_conf;
#[doc = "ICACHE1_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 1 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_prelock_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_prelock_sct0_addr`] module"]
pub type ICACHE1_PRELOCK_SCT0_ADDR =
    crate::Reg<icache1_prelock_sct0_addr::ICACHE1_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section0 address configure register"]
pub mod icache1_prelock_sct0_addr;
#[doc = "ICACHE1_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 1 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_prelock_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_prelock_sct1_addr`] module"]
pub type ICACHE1_PRELOCK_SCT1_ADDR =
    crate::Reg<icache1_prelock_sct1_addr::ICACHE1_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section1 address configure register"]
pub mod icache1_prelock_sct1_addr;
#[doc = "ICACHE1_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 1 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_prelock_sct_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_prelock_sct_size`] module"]
pub type ICACHE1_PRELOCK_SCT_SIZE =
    crate::Reg<icache1_prelock_sct_size::ICACHE1_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section size configure register"]
pub mod icache1_prelock_sct_size;
#[doc = "ICACHE2_PRELOCK_CONF (r) register accessor: L1 instruction Cache 2 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_conf`] module"]
pub type ICACHE2_PRELOCK_CONF = crate::Reg<icache2_prelock_conf::ICACHE2_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 2 prelock configure register"]
pub mod icache2_prelock_conf;
#[doc = "ICACHE2_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 2 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct0_addr`] module"]
pub type ICACHE2_PRELOCK_SCT0_ADDR =
    crate::Reg<icache2_prelock_sct0_addr::ICACHE2_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section0 address configure register"]
pub mod icache2_prelock_sct0_addr;
#[doc = "ICACHE2_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 2 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct1_addr`] module"]
pub type ICACHE2_PRELOCK_SCT1_ADDR =
    crate::Reg<icache2_prelock_sct1_addr::ICACHE2_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section1 address configure register"]
pub mod icache2_prelock_sct1_addr;
#[doc = "ICACHE2_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 2 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_prelock_sct_size`] module"]
pub type ICACHE2_PRELOCK_SCT_SIZE =
    crate::Reg<icache2_prelock_sct_size::ICACHE2_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section size configure register"]
pub mod icache2_prelock_sct_size;
#[doc = "ICACHE3_PRELOCK_CONF (r) register accessor: L1 instruction Cache 3 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_prelock_conf`] module"]
pub type ICACHE3_PRELOCK_CONF = crate::Reg<icache3_prelock_conf::ICACHE3_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 3 prelock configure register"]
pub mod icache3_prelock_conf;
#[doc = "ICACHE3_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 3 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_prelock_sct0_addr`] module"]
pub type ICACHE3_PRELOCK_SCT0_ADDR =
    crate::Reg<icache3_prelock_sct0_addr::ICACHE3_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section0 address configure register"]
pub mod icache3_prelock_sct0_addr;
#[doc = "ICACHE3_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 3 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_prelock_sct1_addr`] module"]
pub type ICACHE3_PRELOCK_SCT1_ADDR =
    crate::Reg<icache3_prelock_sct1_addr::ICACHE3_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section1 address configure register"]
pub mod icache3_prelock_sct1_addr;
#[doc = "ICACHE3_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 3 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_sct_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_prelock_sct_size`] module"]
pub type ICACHE3_PRELOCK_SCT_SIZE =
    crate::Reg<icache3_prelock_sct_size::ICACHE3_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section size configure register"]
pub mod icache3_prelock_sct_size;
#[doc = "CACHE_PRELOCK_CONF (rw) register accessor: L1 Cache prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_prelock_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_prelock_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_prelock_conf`] module"]
pub type CACHE_PRELOCK_CONF = crate::Reg<cache_prelock_conf::CACHE_PRELOCK_CONF_SPEC>;
#[doc = "L1 Cache prelock configure register"]
pub mod cache_prelock_conf;
#[doc = "CACHE_PRELOCK_SCT0_ADDR (rw) register accessor: L1 Cache prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_prelock_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_prelock_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_prelock_sct0_addr`] module"]
pub type CACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<cache_prelock_sct0_addr::CACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 Cache prelock section0 address configure register"]
pub mod cache_prelock_sct0_addr;
#[doc = "DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: L1 Cache prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct1_addr`] module"]
pub type DCACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<dcache_prelock_sct1_addr::DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 Cache prelock section1 address configure register"]
pub mod dcache_prelock_sct1_addr;
#[doc = "DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: L1 Cache prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_prelock_sct_size`] module"]
pub type DCACHE_PRELOCK_SCT_SIZE =
    crate::Reg<dcache_prelock_sct_size::DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 Cache prelock section size configure register"]
pub mod dcache_prelock_sct_size;
#[doc = "CACHE_LOCK_CTRL (rw) register accessor: Lock-class (manual lock) operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_lock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_lock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_lock_ctrl`] module"]
pub type CACHE_LOCK_CTRL = crate::Reg<cache_lock_ctrl::CACHE_LOCK_CTRL_SPEC>;
#[doc = "Lock-class (manual lock) operation control register"]
pub mod cache_lock_ctrl;
#[doc = "CACHE_LOCK_MAP (rw) register accessor: Lock (manual lock) map configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_lock_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_lock_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_lock_map`] module"]
pub type CACHE_LOCK_MAP = crate::Reg<cache_lock_map::CACHE_LOCK_MAP_SPEC>;
#[doc = "Lock (manual lock) map configure register"]
pub mod cache_lock_map;
#[doc = "CACHE_LOCK_ADDR (rw) register accessor: Lock (manual lock) address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_lock_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_lock_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_lock_addr`] module"]
pub type CACHE_LOCK_ADDR = crate::Reg<cache_lock_addr::CACHE_LOCK_ADDR_SPEC>;
#[doc = "Lock (manual lock) address configure register"]
pub mod cache_lock_addr;
#[doc = "CACHE_LOCK_SIZE (rw) register accessor: Lock (manual lock) size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_lock_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_lock_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_lock_size`] module"]
pub type CACHE_LOCK_SIZE = crate::Reg<cache_lock_size::CACHE_LOCK_SIZE_SPEC>;
#[doc = "Lock (manual lock) size configure register"]
pub mod cache_lock_size;
#[doc = "CACHE_SYNC_CTRL (rw) register accessor: Sync-class operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_ctrl`] module"]
pub type CACHE_SYNC_CTRL = crate::Reg<cache_sync_ctrl::CACHE_SYNC_CTRL_SPEC>;
#[doc = "Sync-class operation control register"]
pub mod cache_sync_ctrl;
#[doc = "CACHE_SYNC_MAP (rw) register accessor: Sync map configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_map`] module"]
pub type CACHE_SYNC_MAP = crate::Reg<cache_sync_map::CACHE_SYNC_MAP_SPEC>;
#[doc = "Sync map configure register"]
pub mod cache_sync_map;
#[doc = "CACHE_SYNC_ADDR (rw) register accessor: Sync address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_addr`] module"]
pub type CACHE_SYNC_ADDR = crate::Reg<cache_sync_addr::CACHE_SYNC_ADDR_SPEC>;
#[doc = "Sync address configure register"]
pub mod cache_sync_addr;
#[doc = "CACHE_SYNC_SIZE (rw) register accessor: Sync size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_size`] module"]
pub type CACHE_SYNC_SIZE = crate::Reg<cache_sync_size::CACHE_SYNC_SIZE_SPEC>;
#[doc = "Sync size configure register"]
pub mod cache_sync_size;
#[doc = "ICACHE0_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 0 preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache0_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_preload_ctrl`] module"]
pub type ICACHE0_PRELOAD_CTRL = crate::Reg<icache0_preload_ctrl::ICACHE0_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 0 preload-operation control register"]
pub mod icache0_preload_ctrl;
#[doc = "ICACHE0_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 0 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_preload_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_preload_addr`] module"]
pub type ICACHE0_PRELOAD_ADDR = crate::Reg<icache0_preload_addr::ICACHE0_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 preload address configure register"]
pub mod icache0_preload_addr;
#[doc = "ICACHE0_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 0 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_preload_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_preload_size`] module"]
pub type ICACHE0_PRELOAD_SIZE = crate::Reg<icache0_preload_size::ICACHE0_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 preload size configure register"]
pub mod icache0_preload_size;
#[doc = "ICACHE1_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 1 preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache1_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_preload_ctrl`] module"]
pub type ICACHE1_PRELOAD_CTRL = crate::Reg<icache1_preload_ctrl::ICACHE1_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 1 preload-operation control register"]
pub mod icache1_preload_ctrl;
#[doc = "ICACHE1_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 1 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_preload_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_preload_addr`] module"]
pub type ICACHE1_PRELOAD_ADDR = crate::Reg<icache1_preload_addr::ICACHE1_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 preload address configure register"]
pub mod icache1_preload_addr;
#[doc = "ICACHE1_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 1 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_preload_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_preload_size`] module"]
pub type ICACHE1_PRELOAD_SIZE = crate::Reg<icache1_preload_size::ICACHE1_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 preload size configure register"]
pub mod icache1_preload_size;
#[doc = "ICACHE2_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 2 preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_ctrl`] module"]
pub type ICACHE2_PRELOAD_CTRL = crate::Reg<icache2_preload_ctrl::ICACHE2_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 2 preload-operation control register"]
pub mod icache2_preload_ctrl;
#[doc = "ICACHE2_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 2 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_addr`] module"]
pub type ICACHE2_PRELOAD_ADDR = crate::Reg<icache2_preload_addr::ICACHE2_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 preload address configure register"]
pub mod icache2_preload_addr;
#[doc = "ICACHE2_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 2 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_preload_size`] module"]
pub type ICACHE2_PRELOAD_SIZE = crate::Reg<icache2_preload_size::ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 preload size configure register"]
pub mod icache2_preload_size;
#[doc = "ICACHE3_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 3 preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache3_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_preload_ctrl`] module"]
pub type ICACHE3_PRELOAD_CTRL = crate::Reg<icache3_preload_ctrl::ICACHE3_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 3 preload-operation control register"]
pub mod icache3_preload_ctrl;
#[doc = "ICACHE3_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 3 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_preload_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_preload_addr`] module"]
pub type ICACHE3_PRELOAD_ADDR = crate::Reg<icache3_preload_addr::ICACHE3_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 preload address configure register"]
pub mod icache3_preload_addr;
#[doc = "ICACHE3_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 3 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_preload_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_preload_size`] module"]
pub type ICACHE3_PRELOAD_SIZE = crate::Reg<icache3_preload_size::ICACHE3_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 preload size configure register"]
pub mod icache3_preload_size;
#[doc = "CACHE_PRELOAD_CTRL (rw) register accessor: L1 Cache preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_preload_ctrl`] module"]
pub type CACHE_PRELOAD_CTRL = crate::Reg<cache_preload_ctrl::CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "L1 Cache preload-operation control register"]
pub mod cache_preload_ctrl;
#[doc = "DCACHE_PRELOAD_ADDR (rw) register accessor: L1 Cache preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_addr`] module"]
pub type DCACHE_PRELOAD_ADDR = crate::Reg<dcache_preload_addr::DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "L1 Cache preload address configure register"]
pub mod dcache_preload_addr;
#[doc = "DCACHE_PRELOAD_SIZE (rw) register accessor: L1 Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_preload_size`] module"]
pub type DCACHE_PRELOAD_SIZE = crate::Reg<dcache_preload_size::DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "L1 Cache preload size configure register"]
pub mod dcache_preload_size;
#[doc = "ICACHE0_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 0 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_autoload_ctrl`] module"]
pub type ICACHE0_AUTOLOAD_CTRL = crate::Reg<icache0_autoload_ctrl::ICACHE0_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 0 autoload-operation control register"]
pub mod icache0_autoload_ctrl;
#[doc = "ICACHE0_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 0 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_autoload_sct0_addr`] module"]
pub type ICACHE0_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache0_autoload_sct0_addr::ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 0 address configure register"]
pub mod icache0_autoload_sct0_addr;
#[doc = "ICACHE0_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 0 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_sct0_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_autoload_sct0_size`] module"]
pub type ICACHE0_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache0_autoload_sct0_size::ICACHE0_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 0 size configure register"]
pub mod icache0_autoload_sct0_size;
#[doc = "ICACHE0_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 0 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_autoload_sct1_addr`] module"]
pub type ICACHE0_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache0_autoload_sct1_addr::ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 1 address configure register"]
pub mod icache0_autoload_sct1_addr;
#[doc = "ICACHE0_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 0 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_sct1_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_autoload_sct1_size`] module"]
pub type ICACHE0_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache0_autoload_sct1_size::ICACHE0_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 1 size configure register"]
pub mod icache0_autoload_sct1_size;
#[doc = "ICACHE1_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 1 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_autoload_ctrl`] module"]
pub type ICACHE1_AUTOLOAD_CTRL = crate::Reg<icache1_autoload_ctrl::ICACHE1_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 1 autoload-operation control register"]
pub mod icache1_autoload_ctrl;
#[doc = "ICACHE1_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 1 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_autoload_sct0_addr`] module"]
pub type ICACHE1_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache1_autoload_sct0_addr::ICACHE1_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 0 address configure register"]
pub mod icache1_autoload_sct0_addr;
#[doc = "ICACHE1_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 1 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_sct0_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_autoload_sct0_size`] module"]
pub type ICACHE1_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache1_autoload_sct0_size::ICACHE1_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 0 size configure register"]
pub mod icache1_autoload_sct0_size;
#[doc = "ICACHE1_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 1 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_autoload_sct1_addr`] module"]
pub type ICACHE1_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache1_autoload_sct1_addr::ICACHE1_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 1 address configure register"]
pub mod icache1_autoload_sct1_addr;
#[doc = "ICACHE1_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 1 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_sct1_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_autoload_sct1_size`] module"]
pub type ICACHE1_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache1_autoload_sct1_size::ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 1 size configure register"]
pub mod icache1_autoload_sct1_size;
#[doc = "ICACHE2_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 2 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_ctrl`] module"]
pub type ICACHE2_AUTOLOAD_CTRL = crate::Reg<icache2_autoload_ctrl::ICACHE2_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 2 autoload-operation control register"]
pub mod icache2_autoload_ctrl;
#[doc = "ICACHE2_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 2 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct0_addr`] module"]
pub type ICACHE2_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache2_autoload_sct0_addr::ICACHE2_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 0 address configure register"]
pub mod icache2_autoload_sct0_addr;
#[doc = "ICACHE2_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 2 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct0_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct0_size`] module"]
pub type ICACHE2_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache2_autoload_sct0_size::ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 0 size configure register"]
pub mod icache2_autoload_sct0_size;
#[doc = "ICACHE2_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 2 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct1_addr`] module"]
pub type ICACHE2_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache2_autoload_sct1_addr::ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 1 address configure register"]
pub mod icache2_autoload_sct1_addr;
#[doc = "ICACHE2_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 2 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct1_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_autoload_sct1_size`] module"]
pub type ICACHE2_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache2_autoload_sct1_size::ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 1 size configure register"]
pub mod icache2_autoload_sct1_size;
#[doc = "ICACHE3_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 3 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_autoload_ctrl`] module"]
pub type ICACHE3_AUTOLOAD_CTRL = crate::Reg<icache3_autoload_ctrl::ICACHE3_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 3 autoload-operation control register"]
pub mod icache3_autoload_ctrl;
#[doc = "ICACHE3_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 3 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_autoload_sct0_addr`] module"]
pub type ICACHE3_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache3_autoload_sct0_addr::ICACHE3_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 0 address configure register"]
pub mod icache3_autoload_sct0_addr;
#[doc = "ICACHE3_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 3 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_sct0_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_autoload_sct0_size`] module"]
pub type ICACHE3_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache3_autoload_sct0_size::ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 0 size configure register"]
pub mod icache3_autoload_sct0_size;
#[doc = "ICACHE3_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 3 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_autoload_sct1_addr`] module"]
pub type ICACHE3_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache3_autoload_sct1_addr::ICACHE3_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 1 address configure register"]
pub mod icache3_autoload_sct1_addr;
#[doc = "ICACHE3_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 3 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_sct1_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_autoload_sct1_size`] module"]
pub type ICACHE3_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache3_autoload_sct1_size::ICACHE3_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 1 size configure register"]
pub mod icache3_autoload_sct1_size;
#[doc = "CACHE_AUTOLOAD_CTRL (rw) register accessor: L1 Cache autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_ctrl`] module"]
pub type CACHE_AUTOLOAD_CTRL = crate::Reg<cache_autoload_ctrl::CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 Cache autoload-operation control register"]
pub mod cache_autoload_ctrl;
#[doc = "CACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: L1 Cache autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct0_addr`] module"]
pub type CACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<cache_autoload_sct0_addr::CACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 0 address configure register"]
pub mod cache_autoload_sct0_addr;
#[doc = "CACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: L1 Cache autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct0_size`] module"]
pub type CACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<cache_autoload_sct0_size::CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 0 size configure register"]
pub mod cache_autoload_sct0_size;
#[doc = "CACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: L1 Cache autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct1_addr`] module"]
pub type CACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<cache_autoload_sct1_addr::CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 1 address configure register"]
pub mod cache_autoload_sct1_addr;
#[doc = "CACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: L1 Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct1_size`] module"]
pub type CACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<cache_autoload_sct1_size::CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 1 size configure register"]
pub mod cache_autoload_sct1_size;
#[doc = "CACHE_AUTOLOAD_SCT2_ADDR (r) register accessor: L1 Cache autoload section 2 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct2_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct2_addr`] module"]
pub type CACHE_AUTOLOAD_SCT2_ADDR =
    crate::Reg<cache_autoload_sct2_addr::CACHE_AUTOLOAD_SCT2_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 2 address configure register"]
pub mod cache_autoload_sct2_addr;
#[doc = "CACHE_AUTOLOAD_SCT2_SIZE (r) register accessor: L1 Cache autoload section 2 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct2_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct2_size`] module"]
pub type CACHE_AUTOLOAD_SCT2_SIZE =
    crate::Reg<cache_autoload_sct2_size::CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 2 size configure register"]
pub mod cache_autoload_sct2_size;
#[doc = "CACHE_AUTOLOAD_SCT3_ADDR (r) register accessor: L1 Cache autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct3_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct3_addr`] module"]
pub type CACHE_AUTOLOAD_SCT3_ADDR =
    crate::Reg<cache_autoload_sct3_addr::CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 1 address configure register"]
pub mod cache_autoload_sct3_addr;
#[doc = "CACHE_AUTOLOAD_SCT3_SIZE (r) register accessor: L1 Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct3_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_autoload_sct3_size`] module"]
pub type CACHE_AUTOLOAD_SCT3_SIZE =
    crate::Reg<cache_autoload_sct3_size::CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 1 size configure register"]
pub mod cache_autoload_sct3_size;
#[doc = "CACHE_ACS_CNT_INT_ENA (rw) register accessor: Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_ena`] module"]
pub type CACHE_ACS_CNT_INT_ENA = crate::Reg<cache_acs_cnt_int_ena::CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Cache Access Counter Interrupt enable register"]
pub mod cache_acs_cnt_int_ena;
#[doc = "CACHE_ACS_CNT_INT_CLR (rw) register accessor: Cache Access Counter Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_cnt_int_clr`] module"]
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
#[doc = "CACHE_ACS_FAIL_INT_CLR (w) register accessor: L1-Cache Access Fail Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_fail_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_acs_fail_int_clr`] module"]
pub type CACHE_ACS_FAIL_INT_CLR = crate::Reg<cache_acs_fail_int_clr::CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt clear register"]
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
#[doc = "IBUS0_ACS_HIT_CNT (r) register accessor: L1-ICache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_hit_cnt`] module"]
pub type IBUS0_ACS_HIT_CNT = crate::Reg<ibus0_acs_hit_cnt::IBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus0 Hit-Access Counter register"]
pub mod ibus0_acs_hit_cnt;
#[doc = "IBUS0_ACS_MISS_CNT (r) register accessor: L1-ICache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_miss_cnt`] module"]
pub type IBUS0_ACS_MISS_CNT = crate::Reg<ibus0_acs_miss_cnt::IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus0 Miss-Access Counter register"]
pub mod ibus0_acs_miss_cnt;
#[doc = "IBUS0_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus0 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_conflict_cnt`] module"]
pub type IBUS0_ACS_CONFLICT_CNT = crate::Reg<ibus0_acs_conflict_cnt::IBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus0 Conflict-Access Counter register"]
pub mod ibus0_acs_conflict_cnt;
#[doc = "IBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus0_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS0_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus0_acs_nxtlvl_rd_cnt::IBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-ICache bus0 Next-Level-Access Counter register"]
pub mod ibus0_acs_nxtlvl_rd_cnt;
#[doc = "IBUS1_ACS_HIT_CNT (r) register accessor: L1-ICache bus1 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_hit_cnt`] module"]
pub type IBUS1_ACS_HIT_CNT = crate::Reg<ibus1_acs_hit_cnt::IBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus1 Hit-Access Counter register"]
pub mod ibus1_acs_hit_cnt;
#[doc = "IBUS1_ACS_MISS_CNT (r) register accessor: L1-ICache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_miss_cnt`] module"]
pub type IBUS1_ACS_MISS_CNT = crate::Reg<ibus1_acs_miss_cnt::IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus1 Miss-Access Counter register"]
pub mod ibus1_acs_miss_cnt;
#[doc = "IBUS1_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_conflict_cnt`] module"]
pub type IBUS1_ACS_CONFLICT_CNT = crate::Reg<ibus1_acs_conflict_cnt::IBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus1 Conflict-Access Counter register"]
pub mod ibus1_acs_conflict_cnt;
#[doc = "IBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus1_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS1_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus1_acs_nxtlvl_rd_cnt::IBUS1_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-ICache bus1 Next-Level-Access Counter register"]
pub mod ibus1_acs_nxtlvl_rd_cnt;
#[doc = "IBUS2_ACS_HIT_CNT (r) register accessor: L1-ICache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_hit_cnt`] module"]
pub type IBUS2_ACS_HIT_CNT = crate::Reg<ibus2_acs_hit_cnt::IBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus2 Hit-Access Counter register"]
pub mod ibus2_acs_hit_cnt;
#[doc = "IBUS2_ACS_MISS_CNT (r) register accessor: L1-ICache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_miss_cnt`] module"]
pub type IBUS2_ACS_MISS_CNT = crate::Reg<ibus2_acs_miss_cnt::IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus2 Miss-Access Counter register"]
pub mod ibus2_acs_miss_cnt;
#[doc = "IBUS2_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_conflict_cnt`] module"]
pub type IBUS2_ACS_CONFLICT_CNT = crate::Reg<ibus2_acs_conflict_cnt::IBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus2 Conflict-Access Counter register"]
pub mod ibus2_acs_conflict_cnt;
#[doc = "IBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus2_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus2_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS2_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus2_acs_nxtlvl_rd_cnt::IBUS2_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-ICache bus2 Next-Level-Access Counter register"]
pub mod ibus2_acs_nxtlvl_rd_cnt;
#[doc = "IBUS3_ACS_HIT_CNT (r) register accessor: L1-ICache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_hit_cnt`] module"]
pub type IBUS3_ACS_HIT_CNT = crate::Reg<ibus3_acs_hit_cnt::IBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus3 Hit-Access Counter register"]
pub mod ibus3_acs_hit_cnt;
#[doc = "IBUS3_ACS_MISS_CNT (r) register accessor: L1-ICache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_miss_cnt`] module"]
pub type IBUS3_ACS_MISS_CNT = crate::Reg<ibus3_acs_miss_cnt::IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus3 Miss-Access Counter register"]
pub mod ibus3_acs_miss_cnt;
#[doc = "IBUS3_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus3 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_conflict_cnt`] module"]
pub type IBUS3_ACS_CONFLICT_CNT = crate::Reg<ibus3_acs_conflict_cnt::IBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus3 Conflict-Access Counter register"]
pub mod ibus3_acs_conflict_cnt;
#[doc = "IBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibus3_acs_nxtlvl_rd_cnt`] module"]
pub type IBUS3_ACS_NXTLVL_RD_CNT =
    crate::Reg<ibus3_acs_nxtlvl_rd_cnt::IBUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-ICache bus3 Next-Level-Access Counter register"]
pub mod ibus3_acs_nxtlvl_rd_cnt;
#[doc = "BUS0_ACS_HIT_CNT (r) register accessor: L1-Cache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_hit_cnt`] module"]
pub type BUS0_ACS_HIT_CNT = crate::Reg<bus0_acs_hit_cnt::BUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L1-Cache bus0 Hit-Access Counter register"]
pub mod bus0_acs_hit_cnt;
#[doc = "BUS0_ACS_MISS_CNT (r) register accessor: L1-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_miss_cnt`] module"]
pub type BUS0_ACS_MISS_CNT = crate::Reg<bus0_acs_miss_cnt::BUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L1-Cache bus0 Miss-Access Counter register"]
pub mod bus0_acs_miss_cnt;
#[doc = "BUS0_ACS_CONFLICT_CNT (r) register accessor: L1-Cache bus0 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus0_acs_conflict_cnt`] module"]
pub type BUS0_ACS_CONFLICT_CNT = crate::Reg<bus0_acs_conflict_cnt::BUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-Cache bus0 Conflict-Access Counter register"]
pub mod bus0_acs_conflict_cnt;
#[doc = "DBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L1-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS0_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus0_acs_nxtlvl_rd_cnt::DBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-Cache bus0 Next-Level-Access Counter register"]
pub mod dbus0_acs_nxtlvl_rd_cnt;
#[doc = "DBUS0_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus0 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus0_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus0_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS0_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus0_acs_nxtlvl_wr_cnt::DBUS0_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L1-DCache bus0 WB-Access Counter register"]
pub mod dbus0_acs_nxtlvl_wr_cnt;
#[doc = "BUS1_ACS_HIT_CNT (r) register accessor: L1-Cache bus1 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_hit_cnt`] module"]
pub type BUS1_ACS_HIT_CNT = crate::Reg<bus1_acs_hit_cnt::BUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L1-Cache bus1 Hit-Access Counter register"]
pub mod bus1_acs_hit_cnt;
#[doc = "BUS1_ACS_MISS_CNT (r) register accessor: L1-Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_miss_cnt`] module"]
pub type BUS1_ACS_MISS_CNT = crate::Reg<bus1_acs_miss_cnt::BUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L1-Cache bus1 Miss-Access Counter register"]
pub mod bus1_acs_miss_cnt;
#[doc = "BUS1_ACS_CONFLICT_CNT (r) register accessor: L1-Cache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus1_acs_conflict_cnt`] module"]
pub type BUS1_ACS_CONFLICT_CNT = crate::Reg<bus1_acs_conflict_cnt::BUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-Cache bus1 Conflict-Access Counter register"]
pub mod bus1_acs_conflict_cnt;
#[doc = "DBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS1_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus1_acs_nxtlvl_rd_cnt::DBUS1_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-DCache bus1 Next-Level-Access Counter register"]
pub mod dbus1_acs_nxtlvl_rd_cnt;
#[doc = "DBUS1_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus1 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus1_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus1_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS1_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus1_acs_nxtlvl_wr_cnt::DBUS1_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L1-DCache bus1 WB-Access Counter register"]
pub mod dbus1_acs_nxtlvl_wr_cnt;
#[doc = "DBUS2_ACS_HIT_CNT (r) register accessor: L1-DCache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_hit_cnt`] module"]
pub type DBUS2_ACS_HIT_CNT = crate::Reg<dbus2_acs_hit_cnt::DBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L1-DCache bus2 Hit-Access Counter register"]
pub mod dbus2_acs_hit_cnt;
#[doc = "DBUS2_ACS_MISS_CNT (r) register accessor: L1-DCache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_miss_cnt`] module"]
pub type DBUS2_ACS_MISS_CNT = crate::Reg<dbus2_acs_miss_cnt::DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L1-DCache bus2 Miss-Access Counter register"]
pub mod dbus2_acs_miss_cnt;
#[doc = "DBUS2_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_conflict_cnt`] module"]
pub type DBUS2_ACS_CONFLICT_CNT = crate::Reg<dbus2_acs_conflict_cnt::DBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-DCache bus2 Conflict-Access Counter register"]
pub mod dbus2_acs_conflict_cnt;
#[doc = "DBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS2_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus2_acs_nxtlvl_rd_cnt::DBUS2_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-DCache bus2 Next-Level-Access Counter register"]
pub mod dbus2_acs_nxtlvl_rd_cnt;
#[doc = "DBUS2_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus2 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus2_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS2_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus2_acs_nxtlvl_wr_cnt::DBUS2_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L1-DCache bus2 WB-Access Counter register"]
pub mod dbus2_acs_nxtlvl_wr_cnt;
#[doc = "DBUS3_ACS_HIT_CNT (r) register accessor: L1-DCache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus3_acs_hit_cnt`] module"]
pub type DBUS3_ACS_HIT_CNT = crate::Reg<dbus3_acs_hit_cnt::DBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L1-DCache bus3 Hit-Access Counter register"]
pub mod dbus3_acs_hit_cnt;
#[doc = "DBUS3_ACS_MISS_CNT (r) register accessor: L1-DCache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus3_acs_miss_cnt`] module"]
pub type DBUS3_ACS_MISS_CNT = crate::Reg<dbus3_acs_miss_cnt::DBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L1-DCache bus3 Miss-Access Counter register"]
pub mod dbus3_acs_miss_cnt;
#[doc = "DBUS3_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus3 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus3_acs_conflict_cnt`] module"]
pub type DBUS3_ACS_CONFLICT_CNT = crate::Reg<dbus3_acs_conflict_cnt::DBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-DCache bus3 Conflict-Access Counter register"]
pub mod dbus3_acs_conflict_cnt;
#[doc = "DBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus3_acs_nxtlvl_rd_cnt`] module"]
pub type DBUS3_ACS_NXTLVL_RD_CNT =
    crate::Reg<dbus3_acs_nxtlvl_rd_cnt::DBUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L1-DCache bus3 Next-Level-Access Counter register"]
pub mod dbus3_acs_nxtlvl_rd_cnt;
#[doc = "DBUS3_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus3 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus3_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbus3_acs_nxtlvl_wr_cnt`] module"]
pub type DBUS3_ACS_NXTLVL_WR_CNT =
    crate::Reg<dbus3_acs_nxtlvl_wr_cnt::DBUS3_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L1-DCache bus3 WB-Access Counter register"]
pub mod dbus3_acs_nxtlvl_wr_cnt;
#[doc = "ICACHE0_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_acs_fail_id_attr`] module"]
pub type ICACHE0_ACS_FAIL_ID_ATTR =
    crate::Reg<icache0_acs_fail_id_attr::ICACHE0_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod icache0_acs_fail_id_attr;
#[doc = "ICACHE0_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache0_acs_fail_addr`] module"]
pub type ICACHE0_ACS_FAIL_ADDR = crate::Reg<icache0_acs_fail_addr::ICACHE0_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod icache0_acs_fail_addr;
#[doc = "ICACHE1_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_acs_fail_id_attr`] module"]
pub type ICACHE1_ACS_FAIL_ID_ATTR =
    crate::Reg<icache1_acs_fail_id_attr::ICACHE1_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod icache1_acs_fail_id_attr;
#[doc = "ICACHE1_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache1_acs_fail_addr`] module"]
pub type ICACHE1_ACS_FAIL_ADDR = crate::Reg<icache1_acs_fail_addr::ICACHE1_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod icache1_acs_fail_addr;
#[doc = "ICACHE2_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_acs_fail_id_attr`] module"]
pub type ICACHE2_ACS_FAIL_ID_ATTR =
    crate::Reg<icache2_acs_fail_id_attr::ICACHE2_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod icache2_acs_fail_id_attr;
#[doc = "ICACHE2_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache2_acs_fail_addr`] module"]
pub type ICACHE2_ACS_FAIL_ADDR = crate::Reg<icache2_acs_fail_addr::ICACHE2_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod icache2_acs_fail_addr;
#[doc = "ICACHE3_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_acs_fail_id_attr`] module"]
pub type ICACHE3_ACS_FAIL_ID_ATTR =
    crate::Reg<icache3_acs_fail_id_attr::ICACHE3_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod icache3_acs_fail_id_attr;
#[doc = "ICACHE3_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icache3_acs_fail_addr`] module"]
pub type ICACHE3_ACS_FAIL_ADDR = crate::Reg<icache3_acs_fail_addr::ICACHE3_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod icache3_acs_fail_addr;
#[doc = "DCACHE_ACS_FAIL_ID_ATTR (r) register accessor: L1-Cache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_acs_fail_id_attr`] module"]
pub type DCACHE_ACS_FAIL_ID_ATTR =
    crate::Reg<dcache_acs_fail_id_attr::DCACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-Cache Access Fail ID/attribution information register"]
pub mod dcache_acs_fail_id_attr;
#[doc = "DCACHE_ACS_FAIL_ADDR (r) register accessor: L1-Cache Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcache_acs_fail_addr`] module"]
pub type DCACHE_ACS_FAIL_ADDR = crate::Reg<dcache_acs_fail_addr::DCACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-Cache Access Fail Address information register"]
pub mod dcache_acs_fail_addr;
#[doc = "CACHE_SYNC_PRELOAD_INT_ENA (rw) register accessor: L1-Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_ena`] module"]
pub type CACHE_SYNC_PRELOAD_INT_ENA =
    crate::Reg<cache_sync_preload_int_ena::CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt enable register"]
pub mod cache_sync_preload_int_ena;
#[doc = "CACHE_SYNC_PRELOAD_INT_CLR (rw) register accessor: Sync Preload operation Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_clr`] module"]
pub type CACHE_SYNC_PRELOAD_INT_CLR =
    crate::Reg<cache_sync_preload_int_clr::CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Sync Preload operation Interrupt clear register"]
pub mod cache_sync_preload_int_clr;
#[doc = "CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: Sync Preload operation Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_preload_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_raw`] module"]
pub type CACHE_SYNC_PRELOAD_INT_RAW =
    crate::Reg<cache_sync_preload_int_raw::CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Sync Preload operation Interrupt raw register"]
pub mod cache_sync_preload_int_raw;
#[doc = "CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: L1-Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_preload_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sync_preload_int_st`] module"]
pub type CACHE_SYNC_PRELOAD_INT_ST =
    crate::Reg<cache_sync_preload_int_st::CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt status register"]
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
#[doc = "CACHE_LEVEL_SPLIT0 (r) register accessor: USED TO SPLIT L1 CACHE AND L2 CACHE\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_level_split0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_level_split0`] module"]
pub type CACHE_LEVEL_SPLIT0 = crate::Reg<cache_level_split0::CACHE_LEVEL_SPLIT0_SPEC>;
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE"]
pub mod cache_level_split0;
#[doc = "CACHE_L2_CACHE_CTRL (rw) register accessor: L2 Cache(L2-Cache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_ctrl`] module"]
pub type CACHE_L2_CACHE_CTRL = crate::Reg<cache_l2_cache_ctrl::CACHE_L2_CACHE_CTRL_SPEC>;
#[doc = "L2 Cache(L2-Cache) control register"]
pub mod cache_l2_cache_ctrl;
#[doc = "CACHE_L2_BYPASS_CACHE_CONF (r) register accessor: Bypass Cache configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_bypass_cache_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_bypass_cache_conf`] module"]
pub type CACHE_L2_BYPASS_CACHE_CONF =
    crate::Reg<cache_l2_bypass_cache_conf::CACHE_L2_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Bypass Cache configure register"]
pub mod cache_l2_bypass_cache_conf;
#[doc = "CACHE_L2_CACHE_CACHESIZE_CONF (r) register accessor: L2 Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_cachesize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_cachesize_conf`] module"]
pub type CACHE_L2_CACHE_CACHESIZE_CONF =
    crate::Reg<cache_l2_cache_cachesize_conf::CACHE_L2_CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L2 Cache CacheSize mode configure register"]
pub mod cache_l2_cache_cachesize_conf;
#[doc = "CACHE_L2_CACHE_BLOCKSIZE_CONF (r) register accessor: L2 Cache BlockSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_blocksize_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_blocksize_conf`] module"]
pub type CACHE_L2_CACHE_BLOCKSIZE_CONF =
    crate::Reg<cache_l2_cache_blocksize_conf::CACHE_L2_CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L2 Cache BlockSize mode configure register"]
pub mod cache_l2_cache_blocksize_conf;
#[doc = "CACHE_L2_CACHE_WRAP_AROUND_CTRL (r) register accessor: Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_wrap_around_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_wrap_around_ctrl`] module"]
pub type CACHE_L2_CACHE_WRAP_AROUND_CTRL =
    crate::Reg<cache_l2_cache_wrap_around_ctrl::CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod cache_l2_cache_wrap_around_ctrl;
#[doc = "CACHE_L2_CACHE_MISS_ACCESS_CTRL (r) register accessor: Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_miss_access_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_miss_access_ctrl`] module"]
pub type CACHE_L2_CACHE_MISS_ACCESS_CTRL =
    crate::Reg<cache_l2_cache_miss_access_ctrl::CACHE_L2_CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod cache_l2_cache_miss_access_ctrl;
#[doc = "CACHE_L2_CACHE_FREEZE_CTRL (r) register accessor: Cache Freeze control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_freeze_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_freeze_ctrl`] module"]
pub type CACHE_L2_CACHE_FREEZE_CTRL =
    crate::Reg<cache_l2_cache_freeze_ctrl::CACHE_L2_CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Cache Freeze control register"]
pub mod cache_l2_cache_freeze_ctrl;
#[doc = "CACHE_L2_CACHE_DATA_MEM_ACS_CONF (r) register accessor: Cache data memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_data_mem_acs_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_data_mem_acs_conf`] module"]
pub type CACHE_L2_CACHE_DATA_MEM_ACS_CONF =
    crate::Reg<cache_l2_cache_data_mem_acs_conf::CACHE_L2_CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Cache data memory access configure register"]
pub mod cache_l2_cache_data_mem_acs_conf;
#[doc = "CACHE_L2_CACHE_TAG_MEM_ACS_CONF (r) register accessor: Cache tag memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_tag_mem_acs_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_tag_mem_acs_conf`] module"]
pub type CACHE_L2_CACHE_TAG_MEM_ACS_CONF =
    crate::Reg<cache_l2_cache_tag_mem_acs_conf::CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Cache tag memory access configure register"]
pub mod cache_l2_cache_tag_mem_acs_conf;
#[doc = "CACHE_L2_CACHE_PRELOCK_CONF (r) register accessor: L2 Cache prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_prelock_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_prelock_conf`] module"]
pub type CACHE_L2_CACHE_PRELOCK_CONF =
    crate::Reg<cache_l2_cache_prelock_conf::CACHE_L2_CACHE_PRELOCK_CONF_SPEC>;
#[doc = "L2 Cache prelock configure register"]
pub mod cache_l2_cache_prelock_conf;
#[doc = "CACHE_L2_CACHE_PRELOCK_SCT0_ADDR (r) register accessor: L2 Cache prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_prelock_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_prelock_sct0_addr`] module"]
pub type CACHE_L2_CACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<cache_l2_cache_prelock_sct0_addr::CACHE_L2_CACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L2 Cache prelock section0 address configure register"]
pub mod cache_l2_cache_prelock_sct0_addr;
#[doc = "CACHE_L2_CACHE_PRELOCK_SCT1_ADDR (r) register accessor: L2 Cache prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_prelock_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_prelock_sct1_addr`] module"]
pub type CACHE_L2_CACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<cache_l2_cache_prelock_sct1_addr::CACHE_L2_CACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L2 Cache prelock section1 address configure register"]
pub mod cache_l2_cache_prelock_sct1_addr;
#[doc = "CACHE_L2_CACHE_PRELOCK_SCT_SIZE (r) register accessor: L2 Cache prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_prelock_sct_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_prelock_sct_size`] module"]
pub type CACHE_L2_CACHE_PRELOCK_SCT_SIZE =
    crate::Reg<cache_l2_cache_prelock_sct_size::CACHE_L2_CACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L2 Cache prelock section size configure register"]
pub mod cache_l2_cache_prelock_sct_size;
#[doc = "CACHE_L2_CACHE_PRELOAD_CTRL (rw) register accessor: L2 Cache preload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_preload_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_preload_ctrl`] module"]
pub type CACHE_L2_CACHE_PRELOAD_CTRL =
    crate::Reg<cache_l2_cache_preload_ctrl::CACHE_L2_CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "L2 Cache preload-operation control register"]
pub mod cache_l2_cache_preload_ctrl;
#[doc = "CACHE_L2_CACHE_PRELOAD_ADDR (r) register accessor: L2 Cache preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_preload_addr`] module"]
pub type CACHE_L2_CACHE_PRELOAD_ADDR =
    crate::Reg<cache_l2_cache_preload_addr::CACHE_L2_CACHE_PRELOAD_ADDR_SPEC>;
#[doc = "L2 Cache preload address configure register"]
pub mod cache_l2_cache_preload_addr;
#[doc = "CACHE_L2_CACHE_PRELOAD_SIZE (r) register accessor: L2 Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_preload_size`] module"]
pub type CACHE_L2_CACHE_PRELOAD_SIZE =
    crate::Reg<cache_l2_cache_preload_size::CACHE_L2_CACHE_PRELOAD_SIZE_SPEC>;
#[doc = "L2 Cache preload size configure register"]
pub mod cache_l2_cache_preload_size;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_CTRL (r) register accessor: L2 Cache autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_ctrl`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_CTRL =
    crate::Reg<cache_l2_cache_autoload_ctrl::CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "L2 Cache autoload-operation control register"]
pub mod cache_l2_cache_autoload_ctrl;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT0_ADDR (r) register accessor: L2 Cache autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct0_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct0_addr`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<cache_l2_cache_autoload_sct0_addr::CACHE_L2_CACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 0 address configure register"]
pub mod cache_l2_cache_autoload_sct0_addr;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT0_SIZE (r) register accessor: L2 Cache autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct0_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct0_size`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<cache_l2_cache_autoload_sct0_size::CACHE_L2_CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 0 size configure register"]
pub mod cache_l2_cache_autoload_sct0_size;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT1_ADDR (r) register accessor: L2 Cache autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct1_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct1_addr`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<cache_l2_cache_autoload_sct1_addr::CACHE_L2_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 1 address configure register"]
pub mod cache_l2_cache_autoload_sct1_addr;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT1_SIZE (r) register accessor: L2 Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct1_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct1_size`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<cache_l2_cache_autoload_sct1_size::CACHE_L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 1 size configure register"]
pub mod cache_l2_cache_autoload_sct1_size;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT2_ADDR (r) register accessor: L2 Cache autoload section 2 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct2_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct2_addr`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT2_ADDR =
    crate::Reg<cache_l2_cache_autoload_sct2_addr::CACHE_L2_CACHE_AUTOLOAD_SCT2_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 2 address configure register"]
pub mod cache_l2_cache_autoload_sct2_addr;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE (r) register accessor: L2 Cache autoload section 2 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct2_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct2_size`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE =
    crate::Reg<cache_l2_cache_autoload_sct2_size::CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 2 size configure register"]
pub mod cache_l2_cache_autoload_sct2_size;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR (r) register accessor: L2 Cache autoload section 3 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct3_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct3_addr`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR =
    crate::Reg<cache_l2_cache_autoload_sct3_addr::CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 3 address configure register"]
pub mod cache_l2_cache_autoload_sct3_addr;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_SCT3_SIZE (r) register accessor: L2 Cache autoload section 3 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct3_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_sct3_size`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT3_SIZE =
    crate::Reg<cache_l2_cache_autoload_sct3_size::CACHE_L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 3 size configure register"]
pub mod cache_l2_cache_autoload_sct3_size;
#[doc = "CACHE_L2_CACHE_ACS_CNT_INT_ENA (r) register accessor: Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_int_ena::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_cnt_int_ena`] module"]
pub type CACHE_L2_CACHE_ACS_CNT_INT_ENA =
    crate::Reg<cache_l2_cache_acs_cnt_int_ena::CACHE_L2_CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Cache Access Counter Interrupt enable register"]
pub mod cache_l2_cache_acs_cnt_int_ena;
#[doc = "CACHE_L2_CACHE_ACS_CNT_INT_CLR (r) register accessor: Cache Access Counter Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_int_clr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_cnt_int_clr`] module"]
pub type CACHE_L2_CACHE_ACS_CNT_INT_CLR =
    crate::Reg<cache_l2_cache_acs_cnt_int_clr::CACHE_L2_CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Cache Access Counter Interrupt clear register"]
pub mod cache_l2_cache_acs_cnt_int_clr;
#[doc = "CACHE_L2_CACHE_ACS_CNT_INT_RAW (rw) register accessor: Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_acs_cnt_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_cnt_int_raw`] module"]
pub type CACHE_L2_CACHE_ACS_CNT_INT_RAW =
    crate::Reg<cache_l2_cache_acs_cnt_int_raw::CACHE_L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Cache Access Counter Interrupt raw register"]
pub mod cache_l2_cache_acs_cnt_int_raw;
#[doc = "CACHE_L2_CACHE_ACS_CNT_INT_ST (r) register accessor: Cache Access Counter Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_cnt_int_st`] module"]
pub type CACHE_L2_CACHE_ACS_CNT_INT_ST =
    crate::Reg<cache_l2_cache_acs_cnt_int_st::CACHE_L2_CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Cache Access Counter Interrupt status register"]
pub mod cache_l2_cache_acs_cnt_int_st;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_CTRL (r) register accessor: Cache Access Fail Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_ctrl`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_CTRL =
    crate::Reg<cache_l2_cache_acs_fail_ctrl::CACHE_L2_CACHE_ACS_FAIL_CTRL_SPEC>;
#[doc = "Cache Access Fail Configuration register"]
pub mod cache_l2_cache_acs_fail_ctrl;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_INT_ENA (r) register accessor: Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_int_ena::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_int_ena`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_INT_ENA =
    crate::Reg<cache_l2_cache_acs_fail_int_ena::CACHE_L2_CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Cache Access Fail Interrupt enable register"]
pub mod cache_l2_cache_acs_fail_int_ena;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_INT_CLR (r) register accessor: L1-Cache Access Fail Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_int_clr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_int_clr`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_INT_CLR =
    crate::Reg<cache_l2_cache_acs_fail_int_clr::CACHE_L2_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt clear register"]
pub mod cache_l2_cache_acs_fail_int_clr;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_INT_RAW (rw) register accessor: Cache Access Fail Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_acs_fail_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_int_raw`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_INT_RAW =
    crate::Reg<cache_l2_cache_acs_fail_int_raw::CACHE_L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Cache Access Fail Interrupt raw register"]
pub mod cache_l2_cache_acs_fail_int_raw;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_INT_ST (r) register accessor: Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_int_st`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_INT_ST =
    crate::Reg<cache_l2_cache_acs_fail_int_st::CACHE_L2_CACHE_ACS_FAIL_INT_ST_SPEC>;
#[doc = "Cache Access Fail Interrupt status register"]
pub mod cache_l2_cache_acs_fail_int_st;
#[doc = "CACHE_L2_CACHE_ACS_CNT_CTRL (r) register accessor: Cache Access Counter enable and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_cnt_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_cnt_ctrl`] module"]
pub type CACHE_L2_CACHE_ACS_CNT_CTRL =
    crate::Reg<cache_l2_cache_acs_cnt_ctrl::CACHE_L2_CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Cache Access Counter enable and clear register"]
pub mod cache_l2_cache_acs_cnt_ctrl;
#[doc = "CACHE_L2_IBUS0_ACS_HIT_CNT (r) register accessor: L2-Cache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus0_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus0_acs_hit_cnt`] module"]
pub type CACHE_L2_IBUS0_ACS_HIT_CNT =
    crate::Reg<cache_l2_ibus0_acs_hit_cnt::CACHE_L2_IBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Hit-Access Counter register"]
pub mod cache_l2_ibus0_acs_hit_cnt;
#[doc = "CACHE_L2_IBUS0_ACS_MISS_CNT (r) register accessor: L2-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus0_acs_miss_cnt`] module"]
pub type CACHE_L2_IBUS0_ACS_MISS_CNT =
    crate::Reg<cache_l2_ibus0_acs_miss_cnt::CACHE_L2_IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus0 Miss-Access Counter register"]
pub mod cache_l2_ibus0_acs_miss_cnt;
#[doc = "CACHE_L2_IBUS0_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus0 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus0_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus0_acs_conflict_cnt`] module"]
pub type CACHE_L2_IBUS0_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_ibus0_acs_conflict_cnt::CACHE_L2_IBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Conflict-Access Counter register"]
pub mod cache_l2_ibus0_acs_conflict_cnt;
#[doc = "CACHE_L2_IBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus0_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus0_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_IBUS0_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_ibus0_acs_nxtlvl_rd_cnt::CACHE_L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus0 Next-Level-Access Counter register"]
pub mod cache_l2_ibus0_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_IBUS1_ACS_HIT_CNT (r) register accessor: L2-Cache bus1 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus1_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus1_acs_hit_cnt`] module"]
pub type CACHE_L2_IBUS1_ACS_HIT_CNT =
    crate::Reg<cache_l2_ibus1_acs_hit_cnt::CACHE_L2_IBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Hit-Access Counter register"]
pub mod cache_l2_ibus1_acs_hit_cnt;
#[doc = "CACHE_L2_IBUS1_ACS_MISS_CNT (r) register accessor: L2-Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus1_acs_miss_cnt`] module"]
pub type CACHE_L2_IBUS1_ACS_MISS_CNT =
    crate::Reg<cache_l2_ibus1_acs_miss_cnt::CACHE_L2_IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus1 Miss-Access Counter register"]
pub mod cache_l2_ibus1_acs_miss_cnt;
#[doc = "CACHE_L2_IBUS1_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus1_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus1_acs_conflict_cnt`] module"]
pub type CACHE_L2_IBUS1_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_ibus1_acs_conflict_cnt::CACHE_L2_IBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Conflict-Access Counter register"]
pub mod cache_l2_ibus1_acs_conflict_cnt;
#[doc = "CACHE_L2_IBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus1_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus1_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_IBUS1_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_ibus1_acs_nxtlvl_rd_cnt::CACHE_L2_IBUS1_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus1 Next-Level-Access Counter register"]
pub mod cache_l2_ibus1_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_IBUS2_ACS_HIT_CNT (r) register accessor: L2-Cache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus2_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus2_acs_hit_cnt`] module"]
pub type CACHE_L2_IBUS2_ACS_HIT_CNT =
    crate::Reg<cache_l2_ibus2_acs_hit_cnt::CACHE_L2_IBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Hit-Access Counter register"]
pub mod cache_l2_ibus2_acs_hit_cnt;
#[doc = "CACHE_L2_IBUS2_ACS_MISS_CNT (r) register accessor: L2-Cache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus2_acs_miss_cnt`] module"]
pub type CACHE_L2_IBUS2_ACS_MISS_CNT =
    crate::Reg<cache_l2_ibus2_acs_miss_cnt::CACHE_L2_IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus2 Miss-Access Counter register"]
pub mod cache_l2_ibus2_acs_miss_cnt;
#[doc = "CACHE_L2_IBUS2_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus2_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus2_acs_conflict_cnt`] module"]
pub type CACHE_L2_IBUS2_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_ibus2_acs_conflict_cnt::CACHE_L2_IBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Conflict-Access Counter register"]
pub mod cache_l2_ibus2_acs_conflict_cnt;
#[doc = "CACHE_L2_IBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus2_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus2_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_IBUS2_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_ibus2_acs_nxtlvl_rd_cnt::CACHE_L2_IBUS2_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus2 Next-Level-Access Counter register"]
pub mod cache_l2_ibus2_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_IBUS3_ACS_HIT_CNT (r) register accessor: L2-Cache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus3_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus3_acs_hit_cnt`] module"]
pub type CACHE_L2_IBUS3_ACS_HIT_CNT =
    crate::Reg<cache_l2_ibus3_acs_hit_cnt::CACHE_L2_IBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Hit-Access Counter register"]
pub mod cache_l2_ibus3_acs_hit_cnt;
#[doc = "CACHE_L2_IBUS3_ACS_MISS_CNT (r) register accessor: L2-Cache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus3_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus3_acs_miss_cnt`] module"]
pub type CACHE_L2_IBUS3_ACS_MISS_CNT =
    crate::Reg<cache_l2_ibus3_acs_miss_cnt::CACHE_L2_IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus3 Miss-Access Counter register"]
pub mod cache_l2_ibus3_acs_miss_cnt;
#[doc = "CACHE_L2_IBUS3_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus3 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus3_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus3_acs_conflict_cnt`] module"]
pub type CACHE_L2_IBUS3_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_ibus3_acs_conflict_cnt::CACHE_L2_IBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Conflict-Access Counter register"]
pub mod cache_l2_ibus3_acs_conflict_cnt;
#[doc = "CACHE_L2_IBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_ibus3_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_ibus3_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_IBUS3_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_ibus3_acs_nxtlvl_rd_cnt::CACHE_L2_IBUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus3 Next-Level-Access Counter register"]
pub mod cache_l2_ibus3_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_DBUS0_ACS_HIT_CNT (r) register accessor: L2-Cache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus0_acs_hit_cnt`] module"]
pub type CACHE_L2_DBUS0_ACS_HIT_CNT =
    crate::Reg<cache_l2_dbus0_acs_hit_cnt::CACHE_L2_DBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Hit-Access Counter register"]
pub mod cache_l2_dbus0_acs_hit_cnt;
#[doc = "CACHE_L2_DBUS0_ACS_MISS_CNT (r) register accessor: L2-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus0_acs_miss_cnt`] module"]
pub type CACHE_L2_DBUS0_ACS_MISS_CNT =
    crate::Reg<cache_l2_dbus0_acs_miss_cnt::CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus0 Miss-Access Counter register"]
pub mod cache_l2_dbus0_acs_miss_cnt;
#[doc = "CACHE_L2_DBUS0_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus0 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus0_acs_conflict_cnt`] module"]
pub type CACHE_L2_DBUS0_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_dbus0_acs_conflict_cnt::CACHE_L2_DBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Conflict-Access Counter register"]
pub mod cache_l2_dbus0_acs_conflict_cnt;
#[doc = "CACHE_L2_DBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus0_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_DBUS0_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_dbus0_acs_nxtlvl_rd_cnt::CACHE_L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus0 Next-Level-Access Counter register"]
pub mod cache_l2_dbus0_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_DBUS0_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus0 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus0_acs_nxtlvl_wr_cnt`] module"]
pub type CACHE_L2_DBUS0_ACS_NXTLVL_WR_CNT =
    crate::Reg<cache_l2_dbus0_acs_nxtlvl_wr_cnt::CACHE_L2_DBUS0_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L2-Cache bus0 WB-Access Counter register"]
pub mod cache_l2_dbus0_acs_nxtlvl_wr_cnt;
#[doc = "CACHE_L2_DBUS1_ACS_HIT_CNT (r) register accessor: L2-Cache bus1 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus1_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus1_acs_hit_cnt`] module"]
pub type CACHE_L2_DBUS1_ACS_HIT_CNT =
    crate::Reg<cache_l2_dbus1_acs_hit_cnt::CACHE_L2_DBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Hit-Access Counter register"]
pub mod cache_l2_dbus1_acs_hit_cnt;
#[doc = "CACHE_L2_DBUS1_ACS_MISS_CNT (r) register accessor: L2-Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus1_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus1_acs_miss_cnt`] module"]
pub type CACHE_L2_DBUS1_ACS_MISS_CNT =
    crate::Reg<cache_l2_dbus1_acs_miss_cnt::CACHE_L2_DBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus1 Miss-Access Counter register"]
pub mod cache_l2_dbus1_acs_miss_cnt;
#[doc = "CACHE_L2_DBUS1_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus1_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus1_acs_conflict_cnt`] module"]
pub type CACHE_L2_DBUS1_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_dbus1_acs_conflict_cnt::CACHE_L2_DBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Conflict-Access Counter register"]
pub mod cache_l2_dbus1_acs_conflict_cnt;
#[doc = "CACHE_L2_DBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus1_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus1_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_DBUS1_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_dbus1_acs_nxtlvl_rd_cnt::CACHE_L2_DBUS1_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus1 Next-Level-Access Counter register"]
pub mod cache_l2_dbus1_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_DBUS1_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus1 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus1_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus1_acs_nxtlvl_wr_cnt`] module"]
pub type CACHE_L2_DBUS1_ACS_NXTLVL_WR_CNT =
    crate::Reg<cache_l2_dbus1_acs_nxtlvl_wr_cnt::CACHE_L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L2-Cache bus1 WB-Access Counter register"]
pub mod cache_l2_dbus1_acs_nxtlvl_wr_cnt;
#[doc = "CACHE_L2_DBUS2_ACS_HIT_CNT (r) register accessor: L2-Cache bus2 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus2_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus2_acs_hit_cnt`] module"]
pub type CACHE_L2_DBUS2_ACS_HIT_CNT =
    crate::Reg<cache_l2_dbus2_acs_hit_cnt::CACHE_L2_DBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Hit-Access Counter register"]
pub mod cache_l2_dbus2_acs_hit_cnt;
#[doc = "CACHE_L2_DBUS2_ACS_MISS_CNT (r) register accessor: L2-Cache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus2_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus2_acs_miss_cnt`] module"]
pub type CACHE_L2_DBUS2_ACS_MISS_CNT =
    crate::Reg<cache_l2_dbus2_acs_miss_cnt::CACHE_L2_DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus2 Miss-Access Counter register"]
pub mod cache_l2_dbus2_acs_miss_cnt;
#[doc = "CACHE_L2_DBUS2_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus2_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus2_acs_conflict_cnt`] module"]
pub type CACHE_L2_DBUS2_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_dbus2_acs_conflict_cnt::CACHE_L2_DBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Conflict-Access Counter register"]
pub mod cache_l2_dbus2_acs_conflict_cnt;
#[doc = "CACHE_L2_DBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus2_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus2_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_DBUS2_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_dbus2_acs_nxtlvl_rd_cnt::CACHE_L2_DBUS2_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus2 Next-Level-Access Counter register"]
pub mod cache_l2_dbus2_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_DBUS2_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus2 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus2_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus2_acs_nxtlvl_wr_cnt`] module"]
pub type CACHE_L2_DBUS2_ACS_NXTLVL_WR_CNT =
    crate::Reg<cache_l2_dbus2_acs_nxtlvl_wr_cnt::CACHE_L2_DBUS2_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L2-Cache bus2 WB-Access Counter register"]
pub mod cache_l2_dbus2_acs_nxtlvl_wr_cnt;
#[doc = "CACHE_L2_DBUS3_ACS_HIT_CNT (r) register accessor: L2-Cache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus3_acs_hit_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus3_acs_hit_cnt`] module"]
pub type CACHE_L2_DBUS3_ACS_HIT_CNT =
    crate::Reg<cache_l2_dbus3_acs_hit_cnt::CACHE_L2_DBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Hit-Access Counter register"]
pub mod cache_l2_dbus3_acs_hit_cnt;
#[doc = "CACHE_L2_DBUS3_ACS_MISS_CNT (r) register accessor: L2-Cache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus3_acs_miss_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus3_acs_miss_cnt`] module"]
pub type CACHE_L2_DBUS3_ACS_MISS_CNT =
    crate::Reg<cache_l2_dbus3_acs_miss_cnt::CACHE_L2_DBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus3 Miss-Access Counter register"]
pub mod cache_l2_dbus3_acs_miss_cnt;
#[doc = "CACHE_L2_DBUS3_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus3 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus3_acs_conflict_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus3_acs_conflict_cnt`] module"]
pub type CACHE_L2_DBUS3_ACS_CONFLICT_CNT =
    crate::Reg<cache_l2_dbus3_acs_conflict_cnt::CACHE_L2_DBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Conflict-Access Counter register"]
pub mod cache_l2_dbus3_acs_conflict_cnt;
#[doc = "CACHE_L2_DBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus3_acs_nxtlvl_rd_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus3_acs_nxtlvl_rd_cnt`] module"]
pub type CACHE_L2_DBUS3_ACS_NXTLVL_RD_CNT =
    crate::Reg<cache_l2_dbus3_acs_nxtlvl_rd_cnt::CACHE_L2_DBUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "L2-Cache bus3 Next-Level-Access Counter register"]
pub mod cache_l2_dbus3_acs_nxtlvl_rd_cnt;
#[doc = "CACHE_L2_DBUS3_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus3 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus3_acs_nxtlvl_wr_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_dbus3_acs_nxtlvl_wr_cnt`] module"]
pub type CACHE_L2_DBUS3_ACS_NXTLVL_WR_CNT =
    crate::Reg<cache_l2_dbus3_acs_nxtlvl_wr_cnt::CACHE_L2_DBUS3_ACS_NXTLVL_WR_CNT_SPEC>;
#[doc = "L2-Cache bus3 WB-Access Counter register"]
pub mod cache_l2_dbus3_acs_nxtlvl_wr_cnt;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_ID_ATTR (r) register accessor: L2-Cache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_id_attr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_id_attr`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_ID_ATTR =
    crate::Reg<cache_l2_cache_acs_fail_id_attr::CACHE_L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L2-Cache Access Fail ID/attribution information register"]
pub mod cache_l2_cache_acs_fail_id_attr;
#[doc = "CACHE_L2_CACHE_ACS_FAIL_ADDR (r) register accessor: L2-Cache Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_acs_fail_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_acs_fail_addr`] module"]
pub type CACHE_L2_CACHE_ACS_FAIL_ADDR =
    crate::Reg<cache_l2_cache_acs_fail_addr::CACHE_L2_CACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "L2-Cache Access Fail Address information register"]
pub mod cache_l2_cache_acs_fail_addr;
#[doc = "CACHE_L2_CACHE_SYNC_PRELOAD_INT_ENA (r) register accessor: L1-Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_int_ena::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_preload_int_ena`] module"]
pub type CACHE_L2_CACHE_SYNC_PRELOAD_INT_ENA =
    crate::Reg<cache_l2_cache_sync_preload_int_ena::CACHE_L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt enable register"]
pub mod cache_l2_cache_sync_preload_int_ena;
#[doc = "CACHE_L2_CACHE_SYNC_PRELOAD_INT_CLR (r) register accessor: Sync Preload operation Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_int_clr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_preload_int_clr`] module"]
pub type CACHE_L2_CACHE_SYNC_PRELOAD_INT_CLR =
    crate::Reg<cache_l2_cache_sync_preload_int_clr::CACHE_L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Sync Preload operation Interrupt clear register"]
pub mod cache_l2_cache_sync_preload_int_clr;
#[doc = "CACHE_L2_CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: Sync Preload operation Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_sync_preload_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_preload_int_raw`] module"]
pub type CACHE_L2_CACHE_SYNC_PRELOAD_INT_RAW =
    crate::Reg<cache_l2_cache_sync_preload_int_raw::CACHE_L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Sync Preload operation Interrupt raw register"]
pub mod cache_l2_cache_sync_preload_int_raw;
#[doc = "CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: L1-Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_preload_int_st`] module"]
pub type CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST =
    crate::Reg<cache_l2_cache_sync_preload_int_st::CACHE_L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt status register"]
pub mod cache_l2_cache_sync_preload_int_st;
#[doc = "CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION (r) register accessor: Cache Sync/Preload Operation exception register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_preload_exception::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_preload_exception`] module"]
pub type CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION =
    crate::Reg<cache_l2_cache_sync_preload_exception::CACHE_L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Cache Sync/Preload Operation exception register"]
pub mod cache_l2_cache_sync_preload_exception;
#[doc = "CACHE_L2_CACHE_SYNC_RST_CTRL (r) register accessor: Cache Sync Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_sync_rst_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_sync_rst_ctrl`] module"]
pub type CACHE_L2_CACHE_SYNC_RST_CTRL =
    crate::Reg<cache_l2_cache_sync_rst_ctrl::CACHE_L2_CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Cache Sync Reset control register"]
pub mod cache_l2_cache_sync_rst_ctrl;
#[doc = "CACHE_L2_CACHE_PRELOAD_RST_CTRL (r) register accessor: Cache Preload Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_rst_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_preload_rst_ctrl`] module"]
pub type CACHE_L2_CACHE_PRELOAD_RST_CTRL =
    crate::Reg<cache_l2_cache_preload_rst_ctrl::CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Cache Preload Reset control register"]
pub mod cache_l2_cache_preload_rst_ctrl;
#[doc = "CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL (r) register accessor: Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_buf_clr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_autoload_buf_clr_ctrl`] module"]
pub type CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL =
    crate::Reg<cache_l2_cache_autoload_buf_clr_ctrl::CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Cache Autoload buffer clear control register"]
pub mod cache_l2_cache_autoload_buf_clr_ctrl;
#[doc = "CACHE_L2_UNALLOCATE_BUFFER_CLEAR (r) register accessor: Unallocate request buffer clear registers\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_unallocate_buffer_clear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_unallocate_buffer_clear`] module"]
pub type CACHE_L2_UNALLOCATE_BUFFER_CLEAR =
    crate::Reg<cache_l2_unallocate_buffer_clear::CACHE_L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Unallocate request buffer clear registers"]
pub mod cache_l2_unallocate_buffer_clear;
#[doc = "CACHE_L2_CACHE_ACCESS_ATTR_CTRL (r) register accessor: L2 cache access attribute control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_access_attr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_access_attr_ctrl`] module"]
pub type CACHE_L2_CACHE_ACCESS_ATTR_CTRL =
    crate::Reg<cache_l2_cache_access_attr_ctrl::CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
#[doc = "L2 cache access attribute control register"]
pub mod cache_l2_cache_access_attr_ctrl;
#[doc = "CACHE_L2_CACHE_OBJECT_CTRL (r) register accessor: Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_object_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_object_ctrl`] module"]
pub type CACHE_L2_CACHE_OBJECT_CTRL =
    crate::Reg<cache_l2_cache_object_ctrl::CACHE_L2_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Cache Tag and Data memory Object control register"]
pub mod cache_l2_cache_object_ctrl;
#[doc = "CACHE_L2_CACHE_WAY_OBJECT (r) register accessor: Cache Tag and Data memory way register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_way_object::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_way_object`] module"]
pub type CACHE_L2_CACHE_WAY_OBJECT =
    crate::Reg<cache_l2_cache_way_object::CACHE_L2_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Cache Tag and Data memory way register"]
pub mod cache_l2_cache_way_object;
#[doc = "CACHE_L2_CACHE_ADDR (r) register accessor: Cache address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_addr`] module"]
pub type CACHE_L2_CACHE_ADDR = crate::Reg<cache_l2_cache_addr::CACHE_L2_CACHE_ADDR_SPEC>;
#[doc = "Cache address register"]
pub mod cache_l2_cache_addr;
#[doc = "CACHE_L2_CACHE_DEBUG_BUS (rw) register accessor: Cache Tag/data memory content register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_debug_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_debug_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_l2_cache_debug_bus`] module"]
pub type CACHE_L2_CACHE_DEBUG_BUS =
    crate::Reg<cache_l2_cache_debug_bus::CACHE_L2_CACHE_DEBUG_BUS_SPEC>;
#[doc = "Cache Tag/data memory content register"]
pub mod cache_l2_cache_debug_bus;
#[doc = "CACHE_LEVEL_SPLIT1 (r) register accessor: USED TO SPLIT L1 CACHE AND L2 CACHE\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_level_split1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_level_split1`] module"]
pub type CACHE_LEVEL_SPLIT1 = crate::Reg<cache_level_split1::CACHE_LEVEL_SPLIT1_SPEC>;
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE"]
pub mod cache_level_split1;
#[doc = "CACHE_CLOCK_GATE (rw) register accessor: Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_clock_gate`] module"]
pub type CACHE_CLOCK_GATE = crate::Reg<cache_clock_gate::CACHE_CLOCK_GATE_SPEC>;
#[doc = "Clock gate control register"]
pub mod cache_clock_gate;
#[doc = "CACHE_TRACE_ENA (rw) register accessor: Clock gate control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_trace_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_trace_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_trace_ena`] module"]
pub type CACHE_TRACE_ENA = crate::Reg<cache_trace_ena::CACHE_TRACE_ENA_SPEC>;
#[doc = "Clock gate control register"]
pub mod cache_trace_ena;
#[doc = "CACHE_REDUNDANCY_SIG0 (rw) register accessor: Cache redundancy signal 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_redundancy_sig0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_redundancy_sig0`] module"]
pub type CACHE_REDUNDANCY_SIG0 = crate::Reg<cache_redundancy_sig0::CACHE_REDUNDANCY_SIG0_SPEC>;
#[doc = "Cache redundancy signal 0 register"]
pub mod cache_redundancy_sig0;
#[doc = "CACHE_REDUNDANCY_SIG1 (rw) register accessor: Cache redundancy signal 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_redundancy_sig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_redundancy_sig1`] module"]
pub type CACHE_REDUNDANCY_SIG1 = crate::Reg<cache_redundancy_sig1::CACHE_REDUNDANCY_SIG1_SPEC>;
#[doc = "Cache redundancy signal 1 register"]
pub mod cache_redundancy_sig1;
#[doc = "CACHE_REDUNDANCY_SIG2 (rw) register accessor: Cache redundancy signal 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_redundancy_sig2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_redundancy_sig2`] module"]
pub type CACHE_REDUNDANCY_SIG2 = crate::Reg<cache_redundancy_sig2::CACHE_REDUNDANCY_SIG2_SPEC>;
#[doc = "Cache redundancy signal 2 register"]
pub mod cache_redundancy_sig2;
#[doc = "CACHE_REDUNDANCY_SIG3 (rw) register accessor: Cache redundancy signal 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_redundancy_sig3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_redundancy_sig3`] module"]
pub type CACHE_REDUNDANCY_SIG3 = crate::Reg<cache_redundancy_sig3::CACHE_REDUNDANCY_SIG3_SPEC>;
#[doc = "Cache redundancy signal 3 register"]
pub mod cache_redundancy_sig3;
#[doc = "CACHE_REDUNDANCY_SIG4 (r) register accessor: Cache redundancy signal 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_redundancy_sig4`] module"]
pub type CACHE_REDUNDANCY_SIG4 = crate::Reg<cache_redundancy_sig4::CACHE_REDUNDANCY_SIG4_SPEC>;
#[doc = "Cache redundancy signal 0 register"]
pub mod cache_redundancy_sig4;
#[doc = "CACHE_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_date`] module"]
pub type CACHE_DATE = crate::Reg<cache_date::CACHE_DATE_SPEC>;
#[doc = "Version control register"]
pub mod cache_date;
