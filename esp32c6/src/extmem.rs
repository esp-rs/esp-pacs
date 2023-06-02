#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - L1 instruction Cache(L1-ICache) control register"]
    pub l1_icache_ctrl: L1_ICACHE_CTRL,
    #[doc = "0x04 - L1 data Cache(L1-Cache) control register"]
    pub l1_cache_ctrl: L1_CACHE_CTRL,
    #[doc = "0x08 - Bypass Cache configure register"]
    pub l1_bypass_cache_conf: L1_BYPASS_CACHE_CONF,
    #[doc = "0x0c - L1 Cache atomic feature configure register"]
    pub l1_cache_atomic_conf: L1_CACHE_ATOMIC_CONF,
    #[doc = "0x10 - L1 instruction Cache CacheSize mode configure register"]
    pub l1_icache_cachesize_conf: L1_ICACHE_CACHESIZE_CONF,
    #[doc = "0x14 - L1 instruction Cache BlockSize mode configure register"]
    pub l1_icache_blocksize_conf: L1_ICACHE_BLOCKSIZE_CONF,
    #[doc = "0x18 - L1 data Cache CacheSize mode configure register"]
    pub l1_cache_cachesize_conf: L1_CACHE_CACHESIZE_CONF,
    #[doc = "0x1c - L1 data Cache BlockSize mode configure register"]
    pub l1_cache_blocksize_conf: L1_CACHE_BLOCKSIZE_CONF,
    #[doc = "0x20 - Cache wrap around control register"]
    pub l1_cache_wrap_around_ctrl: L1_CACHE_WRAP_AROUND_CTRL,
    #[doc = "0x24 - Cache tag memory power control register"]
    pub l1_cache_tag_mem_power_ctrl: L1_CACHE_TAG_MEM_POWER_CTRL,
    #[doc = "0x28 - Cache data memory power control register"]
    pub l1_cache_data_mem_power_ctrl: L1_CACHE_DATA_MEM_POWER_CTRL,
    #[doc = "0x2c - Cache Freeze control register"]
    pub l1_cache_freeze_ctrl: L1_CACHE_FREEZE_CTRL,
    #[doc = "0x30 - Cache data memory access configure register"]
    pub l1_cache_data_mem_acs_conf: L1_CACHE_DATA_MEM_ACS_CONF,
    #[doc = "0x34 - Cache tag memory access configure register"]
    pub l1_cache_tag_mem_acs_conf: L1_CACHE_TAG_MEM_ACS_CONF,
    #[doc = "0x38 - L1 instruction Cache 0 prelock configure register"]
    pub l1_icache0_prelock_conf: L1_ICACHE0_PRELOCK_CONF,
    #[doc = "0x3c - L1 instruction Cache 0 prelock section0 address configure register"]
    pub l1_icache0_prelock_sct0_addr: L1_ICACHE0_PRELOCK_SCT0_ADDR,
    #[doc = "0x40 - L1 instruction Cache 0 prelock section1 address configure register"]
    pub l1_icache0_prelock_sct1_addr: L1_ICACHE0_PRELOCK_SCT1_ADDR,
    #[doc = "0x44 - L1 instruction Cache 0 prelock section size configure register"]
    pub l1_icache0_prelock_sct_size: L1_ICACHE0_PRELOCK_SCT_SIZE,
    #[doc = "0x48 - L1 instruction Cache 1 prelock configure register"]
    pub l1_icache1_prelock_conf: L1_ICACHE1_PRELOCK_CONF,
    #[doc = "0x4c - L1 instruction Cache 1 prelock section0 address configure register"]
    pub l1_icache1_prelock_sct0_addr: L1_ICACHE1_PRELOCK_SCT0_ADDR,
    #[doc = "0x50 - L1 instruction Cache 1 prelock section1 address configure register"]
    pub l1_icache1_prelock_sct1_addr: L1_ICACHE1_PRELOCK_SCT1_ADDR,
    #[doc = "0x54 - L1 instruction Cache 1 prelock section size configure register"]
    pub l1_icache1_prelock_sct_size: L1_ICACHE1_PRELOCK_SCT_SIZE,
    #[doc = "0x58 - L1 instruction Cache 2 prelock configure register"]
    pub l1_icache2_prelock_conf: L1_ICACHE2_PRELOCK_CONF,
    #[doc = "0x5c - L1 instruction Cache 2 prelock section0 address configure register"]
    pub l1_icache2_prelock_sct0_addr: L1_ICACHE2_PRELOCK_SCT0_ADDR,
    #[doc = "0x60 - L1 instruction Cache 2 prelock section1 address configure register"]
    pub l1_icache2_prelock_sct1_addr: L1_ICACHE2_PRELOCK_SCT1_ADDR,
    #[doc = "0x64 - L1 instruction Cache 2 prelock section size configure register"]
    pub l1_icache2_prelock_sct_size: L1_ICACHE2_PRELOCK_SCT_SIZE,
    #[doc = "0x68 - L1 instruction Cache 3 prelock configure register"]
    pub l1_icache3_prelock_conf: L1_ICACHE3_PRELOCK_CONF,
    #[doc = "0x6c - L1 instruction Cache 3 prelock section0 address configure register"]
    pub l1_icache3_prelock_sct0_addr: L1_ICACHE3_PRELOCK_SCT0_ADDR,
    #[doc = "0x70 - L1 instruction Cache 3 prelock section1 address configure register"]
    pub l1_icache3_prelock_sct1_addr: L1_ICACHE3_PRELOCK_SCT1_ADDR,
    #[doc = "0x74 - L1 instruction Cache 3 prelock section size configure register"]
    pub l1_icache3_prelock_sct_size: L1_ICACHE3_PRELOCK_SCT_SIZE,
    #[doc = "0x78 - L1 Cache prelock configure register"]
    pub l1_cache_prelock_conf: L1_CACHE_PRELOCK_CONF,
    #[doc = "0x7c - L1 Cache prelock section0 address configure register"]
    pub l1_cache_prelock_sct0_addr: L1_CACHE_PRELOCK_SCT0_ADDR,
    #[doc = "0x80 - L1 Cache prelock section1 address configure register"]
    pub l1_dcache_prelock_sct1_addr: L1_DCACHE_PRELOCK_SCT1_ADDR,
    #[doc = "0x84 - L1 Cache prelock section size configure register"]
    pub l1_dcache_prelock_sct_size: L1_DCACHE_PRELOCK_SCT_SIZE,
    #[doc = "0x88 - Lock-class (manual lock) operation control register"]
    pub cache_lock_ctrl: CACHE_LOCK_CTRL,
    #[doc = "0x8c - Lock (manual lock) map configure register"]
    pub cache_lock_map: CACHE_LOCK_MAP,
    #[doc = "0x90 - Lock (manual lock) address configure register"]
    pub cache_lock_addr: CACHE_LOCK_ADDR,
    #[doc = "0x94 - Lock (manual lock) size configure register"]
    pub cache_lock_size: CACHE_LOCK_SIZE,
    #[doc = "0x98 - Sync-class operation control register"]
    pub cache_sync_ctrl: CACHE_SYNC_CTRL,
    #[doc = "0x9c - Sync map configure register"]
    pub cache_sync_map: CACHE_SYNC_MAP,
    #[doc = "0xa0 - Sync address configure register"]
    pub cache_sync_addr: CACHE_SYNC_ADDR,
    #[doc = "0xa4 - Sync size configure register"]
    pub cache_sync_size: CACHE_SYNC_SIZE,
    #[doc = "0xa8 - L1 instruction Cache 0 preload-operation control register"]
    pub l1_icache0_preload_ctrl: L1_ICACHE0_PRELOAD_CTRL,
    #[doc = "0xac - L1 instruction Cache 0 preload address configure register"]
    pub l1_icache0_preload_addr: L1_ICACHE0_PRELOAD_ADDR,
    #[doc = "0xb0 - L1 instruction Cache 0 preload size configure register"]
    pub l1_icache0_preload_size: L1_ICACHE0_PRELOAD_SIZE,
    #[doc = "0xb4 - L1 instruction Cache 1 preload-operation control register"]
    pub l1_icache1_preload_ctrl: L1_ICACHE1_PRELOAD_CTRL,
    #[doc = "0xb8 - L1 instruction Cache 1 preload address configure register"]
    pub l1_icache1_preload_addr: L1_ICACHE1_PRELOAD_ADDR,
    #[doc = "0xbc - L1 instruction Cache 1 preload size configure register"]
    pub l1_icache1_preload_size: L1_ICACHE1_PRELOAD_SIZE,
    #[doc = "0xc0 - L1 instruction Cache 2 preload-operation control register"]
    pub l1_icache2_preload_ctrl: L1_ICACHE2_PRELOAD_CTRL,
    #[doc = "0xc4 - L1 instruction Cache 2 preload address configure register"]
    pub l1_icache2_preload_addr: L1_ICACHE2_PRELOAD_ADDR,
    #[doc = "0xc8 - L1 instruction Cache 2 preload size configure register"]
    pub l1_icache2_preload_size: L1_ICACHE2_PRELOAD_SIZE,
    #[doc = "0xcc - L1 instruction Cache 3 preload-operation control register"]
    pub l1_icache3_preload_ctrl: L1_ICACHE3_PRELOAD_CTRL,
    #[doc = "0xd0 - L1 instruction Cache 3 preload address configure register"]
    pub l1_icache3_preload_addr: L1_ICACHE3_PRELOAD_ADDR,
    #[doc = "0xd4 - L1 instruction Cache 3 preload size configure register"]
    pub l1_icache3_preload_size: L1_ICACHE3_PRELOAD_SIZE,
    #[doc = "0xd8 - L1 Cache preload-operation control register"]
    pub l1_cache_preload_ctrl: L1_CACHE_PRELOAD_CTRL,
    #[doc = "0xdc - L1 Cache preload address configure register"]
    pub l1_dcache_preload_addr: L1_DCACHE_PRELOAD_ADDR,
    #[doc = "0xe0 - L1 Cache preload size configure register"]
    pub l1_dcache_preload_size: L1_DCACHE_PRELOAD_SIZE,
    #[doc = "0xe4 - L1 instruction Cache 0 autoload-operation control register"]
    pub l1_icache0_autoload_ctrl: L1_ICACHE0_AUTOLOAD_CTRL,
    #[doc = "0xe8 - L1 instruction Cache 0 autoload section 0 address configure register"]
    pub l1_icache0_autoload_sct0_addr: L1_ICACHE0_AUTOLOAD_SCT0_ADDR,
    #[doc = "0xec - L1 instruction Cache 0 autoload section 0 size configure register"]
    pub l1_icache0_autoload_sct0_size: L1_ICACHE0_AUTOLOAD_SCT0_SIZE,
    #[doc = "0xf0 - L1 instruction Cache 0 autoload section 1 address configure register"]
    pub l1_icache0_autoload_sct1_addr: L1_ICACHE0_AUTOLOAD_SCT1_ADDR,
    #[doc = "0xf4 - L1 instruction Cache 0 autoload section 1 size configure register"]
    pub l1_icache0_autoload_sct1_size: L1_ICACHE0_AUTOLOAD_SCT1_SIZE,
    #[doc = "0xf8 - L1 instruction Cache 1 autoload-operation control register"]
    pub l1_icache1_autoload_ctrl: L1_ICACHE1_AUTOLOAD_CTRL,
    #[doc = "0xfc - L1 instruction Cache 1 autoload section 0 address configure register"]
    pub l1_icache1_autoload_sct0_addr: L1_ICACHE1_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x100 - L1 instruction Cache 1 autoload section 0 size configure register"]
    pub l1_icache1_autoload_sct0_size: L1_ICACHE1_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x104 - L1 instruction Cache 1 autoload section 1 address configure register"]
    pub l1_icache1_autoload_sct1_addr: L1_ICACHE1_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x108 - L1 instruction Cache 1 autoload section 1 size configure register"]
    pub l1_icache1_autoload_sct1_size: L1_ICACHE1_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x10c - L1 instruction Cache 2 autoload-operation control register"]
    pub l1_icache2_autoload_ctrl: L1_ICACHE2_AUTOLOAD_CTRL,
    #[doc = "0x110 - L1 instruction Cache 2 autoload section 0 address configure register"]
    pub l1_icache2_autoload_sct0_addr: L1_ICACHE2_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x114 - L1 instruction Cache 2 autoload section 0 size configure register"]
    pub l1_icache2_autoload_sct0_size: L1_ICACHE2_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x118 - L1 instruction Cache 2 autoload section 1 address configure register"]
    pub l1_icache2_autoload_sct1_addr: L1_ICACHE2_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x11c - L1 instruction Cache 2 autoload section 1 size configure register"]
    pub l1_icache2_autoload_sct1_size: L1_ICACHE2_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x120 - L1 instruction Cache 3 autoload-operation control register"]
    pub l1_icache3_autoload_ctrl: L1_ICACHE3_AUTOLOAD_CTRL,
    #[doc = "0x124 - L1 instruction Cache 3 autoload section 0 address configure register"]
    pub l1_icache3_autoload_sct0_addr: L1_ICACHE3_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x128 - L1 instruction Cache 3 autoload section 0 size configure register"]
    pub l1_icache3_autoload_sct0_size: L1_ICACHE3_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x12c - L1 instruction Cache 3 autoload section 1 address configure register"]
    pub l1_icache3_autoload_sct1_addr: L1_ICACHE3_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x130 - L1 instruction Cache 3 autoload section 1 size configure register"]
    pub l1_icache3_autoload_sct1_size: L1_ICACHE3_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x134 - L1 Cache autoload-operation control register"]
    pub l1_cache_autoload_ctrl: L1_CACHE_AUTOLOAD_CTRL,
    #[doc = "0x138 - L1 Cache autoload section 0 address configure register"]
    pub l1_cache_autoload_sct0_addr: L1_CACHE_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x13c - L1 Cache autoload section 0 size configure register"]
    pub l1_cache_autoload_sct0_size: L1_CACHE_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x140 - L1 Cache autoload section 1 address configure register"]
    pub l1_cache_autoload_sct1_addr: L1_CACHE_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x144 - L1 Cache autoload section 1 size configure register"]
    pub l1_cache_autoload_sct1_size: L1_CACHE_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x148 - L1 Cache autoload section 2 address configure register"]
    pub l1_cache_autoload_sct2_addr: L1_CACHE_AUTOLOAD_SCT2_ADDR,
    #[doc = "0x14c - L1 Cache autoload section 2 size configure register"]
    pub l1_cache_autoload_sct2_size: L1_CACHE_AUTOLOAD_SCT2_SIZE,
    #[doc = "0x150 - L1 Cache autoload section 1 address configure register"]
    pub l1_cache_autoload_sct3_addr: L1_CACHE_AUTOLOAD_SCT3_ADDR,
    #[doc = "0x154 - L1 Cache autoload section 1 size configure register"]
    pub l1_cache_autoload_sct3_size: L1_CACHE_AUTOLOAD_SCT3_SIZE,
    #[doc = "0x158 - Cache Access Counter Interrupt enable register"]
    pub l1_cache_acs_cnt_int_ena: L1_CACHE_ACS_CNT_INT_ENA,
    #[doc = "0x15c - Cache Access Counter Interrupt clear register"]
    pub l1_cache_acs_cnt_int_clr: L1_CACHE_ACS_CNT_INT_CLR,
    #[doc = "0x160 - Cache Access Counter Interrupt raw register"]
    pub l1_cache_acs_cnt_int_raw: L1_CACHE_ACS_CNT_INT_RAW,
    #[doc = "0x164 - Cache Access Counter Interrupt status register"]
    pub l1_cache_acs_cnt_int_st: L1_CACHE_ACS_CNT_INT_ST,
    #[doc = "0x168 - Cache Access Fail Interrupt enable register"]
    pub l1_cache_acs_fail_int_ena: L1_CACHE_ACS_FAIL_INT_ENA,
    #[doc = "0x16c - L1-Cache Access Fail Interrupt clear register"]
    pub l1_cache_acs_fail_int_clr: L1_CACHE_ACS_FAIL_INT_CLR,
    #[doc = "0x170 - Cache Access Fail Interrupt raw register"]
    pub l1_cache_acs_fail_int_raw: L1_CACHE_ACS_FAIL_INT_RAW,
    #[doc = "0x174 - Cache Access Fail Interrupt status register"]
    pub l1_cache_acs_fail_int_st: L1_CACHE_ACS_FAIL_INT_ST,
    #[doc = "0x178 - Cache Access Counter enable and clear register"]
    pub l1_cache_acs_cnt_ctrl: L1_CACHE_ACS_CNT_CTRL,
    #[doc = "0x17c - L1-ICache bus0 Hit-Access Counter register"]
    pub l1_ibus0_acs_hit_cnt: L1_IBUS0_ACS_HIT_CNT,
    #[doc = "0x180 - L1-ICache bus0 Miss-Access Counter register"]
    pub l1_ibus0_acs_miss_cnt: L1_IBUS0_ACS_MISS_CNT,
    #[doc = "0x184 - L1-ICache bus0 Conflict-Access Counter register"]
    pub l1_ibus0_acs_conflict_cnt: L1_IBUS0_ACS_CONFLICT_CNT,
    #[doc = "0x188 - L1-ICache bus0 Next-Level-Access Counter register"]
    pub l1_ibus0_acs_nxtlvl_cnt: L1_IBUS0_ACS_NXTLVL_CNT,
    #[doc = "0x18c - L1-ICache bus1 Hit-Access Counter register"]
    pub l1_ibus1_acs_hit_cnt: L1_IBUS1_ACS_HIT_CNT,
    #[doc = "0x190 - L1-ICache bus1 Miss-Access Counter register"]
    pub l1_ibus1_acs_miss_cnt: L1_IBUS1_ACS_MISS_CNT,
    #[doc = "0x194 - L1-ICache bus1 Conflict-Access Counter register"]
    pub l1_ibus1_acs_conflict_cnt: L1_IBUS1_ACS_CONFLICT_CNT,
    #[doc = "0x198 - L1-ICache bus1 Next-Level-Access Counter register"]
    pub l1_ibus1_acs_nxtlvl_cnt: L1_IBUS1_ACS_NXTLVL_CNT,
    #[doc = "0x19c - L1-ICache bus2 Hit-Access Counter register"]
    pub l1_ibus2_acs_hit_cnt: L1_IBUS2_ACS_HIT_CNT,
    #[doc = "0x1a0 - L1-ICache bus2 Miss-Access Counter register"]
    pub l1_ibus2_acs_miss_cnt: L1_IBUS2_ACS_MISS_CNT,
    #[doc = "0x1a4 - L1-ICache bus2 Conflict-Access Counter register"]
    pub l1_ibus2_acs_conflict_cnt: L1_IBUS2_ACS_CONFLICT_CNT,
    #[doc = "0x1a8 - L1-ICache bus2 Next-Level-Access Counter register"]
    pub l1_ibus2_acs_nxtlvl_cnt: L1_IBUS2_ACS_NXTLVL_CNT,
    #[doc = "0x1ac - L1-ICache bus3 Hit-Access Counter register"]
    pub l1_ibus3_acs_hit_cnt: L1_IBUS3_ACS_HIT_CNT,
    #[doc = "0x1b0 - L1-ICache bus3 Miss-Access Counter register"]
    pub l1_ibus3_acs_miss_cnt: L1_IBUS3_ACS_MISS_CNT,
    #[doc = "0x1b4 - L1-ICache bus3 Conflict-Access Counter register"]
    pub l1_ibus3_acs_conflict_cnt: L1_IBUS3_ACS_CONFLICT_CNT,
    #[doc = "0x1b8 - L1-ICache bus3 Next-Level-Access Counter register"]
    pub l1_ibus3_acs_nxtlvl_cnt: L1_IBUS3_ACS_NXTLVL_CNT,
    #[doc = "0x1bc - L1-Cache bus0 Hit-Access Counter register"]
    pub l1_bus0_acs_hit_cnt: L1_BUS0_ACS_HIT_CNT,
    #[doc = "0x1c0 - L1-Cache bus0 Miss-Access Counter register"]
    pub l1_bus0_acs_miss_cnt: L1_BUS0_ACS_MISS_CNT,
    #[doc = "0x1c4 - L1-Cache bus0 Conflict-Access Counter register"]
    pub l1_bus0_acs_conflict_cnt: L1_BUS0_ACS_CONFLICT_CNT,
    #[doc = "0x1c8 - L1-Cache bus0 Next-Level-Access Counter register"]
    pub l1_bus0_acs_nxtlvl_cnt: L1_BUS0_ACS_NXTLVL_CNT,
    #[doc = "0x1cc - L1-Cache bus1 Hit-Access Counter register"]
    pub l1_bus1_acs_hit_cnt: L1_BUS1_ACS_HIT_CNT,
    #[doc = "0x1d0 - L1-Cache bus1 Miss-Access Counter register"]
    pub l1_bus1_acs_miss_cnt: L1_BUS1_ACS_MISS_CNT,
    #[doc = "0x1d4 - L1-Cache bus1 Conflict-Access Counter register"]
    pub l1_bus1_acs_conflict_cnt: L1_BUS1_ACS_CONFLICT_CNT,
    #[doc = "0x1d8 - L1-Cache bus1 Next-Level-Access Counter register"]
    pub l1_bus1_acs_nxtlvl_cnt: L1_BUS1_ACS_NXTLVL_CNT,
    #[doc = "0x1dc - L1-DCache bus2 Hit-Access Counter register"]
    pub l1_dbus2_acs_hit_cnt: L1_DBUS2_ACS_HIT_CNT,
    #[doc = "0x1e0 - L1-DCache bus2 Miss-Access Counter register"]
    pub l1_dbus2_acs_miss_cnt: L1_DBUS2_ACS_MISS_CNT,
    #[doc = "0x1e4 - L1-DCache bus2 Conflict-Access Counter register"]
    pub l1_dbus2_acs_conflict_cnt: L1_DBUS2_ACS_CONFLICT_CNT,
    #[doc = "0x1e8 - L1-DCache bus2 Next-Level-Access Counter register"]
    pub l1_dbus2_acs_nxtlvl_cnt: L1_DBUS2_ACS_NXTLVL_CNT,
    #[doc = "0x1ec - L1-DCache bus3 Hit-Access Counter register"]
    pub l1_dbus3_acs_hit_cnt: L1_DBUS3_ACS_HIT_CNT,
    #[doc = "0x1f0 - L1-DCache bus3 Miss-Access Counter register"]
    pub l1_dbus3_acs_miss_cnt: L1_DBUS3_ACS_MISS_CNT,
    #[doc = "0x1f4 - L1-DCache bus3 Conflict-Access Counter register"]
    pub l1_dbus3_acs_conflict_cnt: L1_DBUS3_ACS_CONFLICT_CNT,
    #[doc = "0x1f8 - L1-DCache bus3 Next-Level-Access Counter register"]
    pub l1_dbus3_acs_nxtlvl_cnt: L1_DBUS3_ACS_NXTLVL_CNT,
    #[doc = "0x1fc - L1-ICache0 Access Fail ID/attribution information register"]
    pub l1_icache0_acs_fail_id_attr: L1_ICACHE0_ACS_FAIL_ID_ATTR,
    #[doc = "0x200 - L1-ICache0 Access Fail Address information register"]
    pub l1_icache0_acs_fail_addr: L1_ICACHE0_ACS_FAIL_ADDR,
    #[doc = "0x204 - L1-ICache0 Access Fail ID/attribution information register"]
    pub l1_icache1_acs_fail_id_attr: L1_ICACHE1_ACS_FAIL_ID_ATTR,
    #[doc = "0x208 - L1-ICache0 Access Fail Address information register"]
    pub l1_icache1_acs_fail_addr: L1_ICACHE1_ACS_FAIL_ADDR,
    #[doc = "0x20c - L1-ICache0 Access Fail ID/attribution information register"]
    pub l1_icache2_acs_fail_id_attr: L1_ICACHE2_ACS_FAIL_ID_ATTR,
    #[doc = "0x210 - L1-ICache0 Access Fail Address information register"]
    pub l1_icache2_acs_fail_addr: L1_ICACHE2_ACS_FAIL_ADDR,
    #[doc = "0x214 - L1-ICache0 Access Fail ID/attribution information register"]
    pub l1_icache3_acs_fail_id_attr: L1_ICACHE3_ACS_FAIL_ID_ATTR,
    #[doc = "0x218 - L1-ICache0 Access Fail Address information register"]
    pub l1_icache3_acs_fail_addr: L1_ICACHE3_ACS_FAIL_ADDR,
    #[doc = "0x21c - L1-Cache Access Fail ID/attribution information register"]
    pub l1_cache_acs_fail_id_attr: L1_CACHE_ACS_FAIL_ID_ATTR,
    #[doc = "0x220 - L1-Cache Access Fail Address information register"]
    pub l1_dcache_acs_fail_addr: L1_DCACHE_ACS_FAIL_ADDR,
    #[doc = "0x224 - L1-Cache Access Fail Interrupt enable register"]
    pub l1_cache_sync_preload_int_ena: L1_CACHE_SYNC_PRELOAD_INT_ENA,
    #[doc = "0x228 - Sync Preload operation Interrupt clear register"]
    pub l1_cache_sync_preload_int_clr: L1_CACHE_SYNC_PRELOAD_INT_CLR,
    #[doc = "0x22c - Sync Preload operation Interrupt raw register"]
    pub l1_cache_sync_preload_int_raw: L1_CACHE_SYNC_PRELOAD_INT_RAW,
    #[doc = "0x230 - L1-Cache Access Fail Interrupt status register"]
    pub l1_cache_sync_preload_int_st: L1_CACHE_SYNC_PRELOAD_INT_ST,
    #[doc = "0x234 - Cache Sync/Preload Operation exception register"]
    pub l1_cache_sync_preload_exception: L1_CACHE_SYNC_PRELOAD_EXCEPTION,
    #[doc = "0x238 - Cache Sync Reset control register"]
    pub l1_cache_sync_rst_ctrl: L1_CACHE_SYNC_RST_CTRL,
    #[doc = "0x23c - Cache Preload Reset control register"]
    pub l1_cache_preload_rst_ctrl: L1_CACHE_PRELOAD_RST_CTRL,
    #[doc = "0x240 - Cache Autoload buffer clear control register"]
    pub l1_cache_autoload_buf_clr_ctrl: L1_CACHE_AUTOLOAD_BUF_CLR_CTRL,
    #[doc = "0x244 - Unallocate request buffer clear registers"]
    pub l1_unallocate_buffer_clear: L1_UNALLOCATE_BUFFER_CLEAR,
    #[doc = "0x248 - Cache Tag and Data memory Object control register"]
    pub l1_cache_object_ctrl: L1_CACHE_OBJECT_CTRL,
    #[doc = "0x24c - Cache Tag and Data memory way register"]
    pub l1_cache_way_object: L1_CACHE_WAY_OBJECT,
    #[doc = "0x250 - Cache Vaddr register"]
    pub l1_cache_vaddr: L1_CACHE_VADDR,
    #[doc = "0x254 - Cache Tag/data memory content register"]
    pub l1_cache_debug_bus: L1_CACHE_DEBUG_BUS,
    #[doc = "0x258 - USED TO SPLIT L1 CACHE AND L2 CACHE"]
    pub level_split0: LEVEL_SPLIT0,
    #[doc = "0x25c - L2 Cache(L2-Cache) control register"]
    pub l2_cache_ctrl: L2_CACHE_CTRL,
    #[doc = "0x260 - Bypass Cache configure register"]
    pub l2_bypass_cache_conf: L2_BYPASS_CACHE_CONF,
    #[doc = "0x264 - L2 Cache CacheSize mode configure register"]
    pub l2_cache_cachesize_conf: L2_CACHE_CACHESIZE_CONF,
    #[doc = "0x268 - L2 Cache BlockSize mode configure register"]
    pub l2_cache_blocksize_conf: L2_CACHE_BLOCKSIZE_CONF,
    #[doc = "0x26c - Cache wrap around control register"]
    pub l2_cache_wrap_around_ctrl: L2_CACHE_WRAP_AROUND_CTRL,
    #[doc = "0x270 - Cache tag memory power control register"]
    pub l2_cache_tag_mem_power_ctrl: L2_CACHE_TAG_MEM_POWER_CTRL,
    #[doc = "0x274 - Cache data memory power control register"]
    pub l2_cache_data_mem_power_ctrl: L2_CACHE_DATA_MEM_POWER_CTRL,
    #[doc = "0x278 - Cache Freeze control register"]
    pub l2_cache_freeze_ctrl: L2_CACHE_FREEZE_CTRL,
    #[doc = "0x27c - Cache data memory access configure register"]
    pub l2_cache_data_mem_acs_conf: L2_CACHE_DATA_MEM_ACS_CONF,
    #[doc = "0x280 - Cache tag memory access configure register"]
    pub l2_cache_tag_mem_acs_conf: L2_CACHE_TAG_MEM_ACS_CONF,
    #[doc = "0x284 - L2 Cache prelock configure register"]
    pub l2_cache_prelock_conf: L2_CACHE_PRELOCK_CONF,
    #[doc = "0x288 - L2 Cache prelock section0 address configure register"]
    pub l2_cache_prelock_sct0_addr: L2_CACHE_PRELOCK_SCT0_ADDR,
    #[doc = "0x28c - L2 Cache prelock section1 address configure register"]
    pub l2_cache_prelock_sct1_addr: L2_CACHE_PRELOCK_SCT1_ADDR,
    #[doc = "0x290 - L2 Cache prelock section size configure register"]
    pub l2_cache_prelock_sct_size: L2_CACHE_PRELOCK_SCT_SIZE,
    #[doc = "0x294 - L2 Cache preload-operation control register"]
    pub l2_cache_preload_ctrl: L2_CACHE_PRELOAD_CTRL,
    #[doc = "0x298 - L2 Cache preload address configure register"]
    pub l2_cache_preload_addr: L2_CACHE_PRELOAD_ADDR,
    #[doc = "0x29c - L2 Cache preload size configure register"]
    pub l2_cache_preload_size: L2_CACHE_PRELOAD_SIZE,
    #[doc = "0x2a0 - L2 Cache autoload-operation control register"]
    pub l2_cache_autoload_ctrl: L2_CACHE_AUTOLOAD_CTRL,
    #[doc = "0x2a4 - L2 Cache autoload section 0 address configure register"]
    pub l2_cache_autoload_sct0_addr: L2_CACHE_AUTOLOAD_SCT0_ADDR,
    #[doc = "0x2a8 - L2 Cache autoload section 0 size configure register"]
    pub l2_cache_autoload_sct0_size: L2_CACHE_AUTOLOAD_SCT0_SIZE,
    #[doc = "0x2ac - L2 Cache autoload section 1 address configure register"]
    pub l2_cache_autoload_sct1_addr: L2_CACHE_AUTOLOAD_SCT1_ADDR,
    #[doc = "0x2b0 - L2 Cache autoload section 1 size configure register"]
    pub l2_cache_autoload_sct1_size: L2_CACHE_AUTOLOAD_SCT1_SIZE,
    #[doc = "0x2b4 - L2 Cache autoload section 2 address configure register"]
    pub l2_cache_autoload_sct2_addr: L2_CACHE_AUTOLOAD_SCT2_ADDR,
    #[doc = "0x2b8 - L2 Cache autoload section 2 size configure register"]
    pub l2_cache_autoload_sct2_size: L2_CACHE_AUTOLOAD_SCT2_SIZE,
    #[doc = "0x2bc - L2 Cache autoload section 3 address configure register"]
    pub l2_cache_autoload_sct3_addr: L2_CACHE_AUTOLOAD_SCT3_ADDR,
    #[doc = "0x2c0 - L2 Cache autoload section 3 size configure register"]
    pub l2_cache_autoload_sct3_size: L2_CACHE_AUTOLOAD_SCT3_SIZE,
    #[doc = "0x2c4 - Cache Access Counter Interrupt enable register"]
    pub l2_cache_acs_cnt_int_ena: L2_CACHE_ACS_CNT_INT_ENA,
    #[doc = "0x2c8 - Cache Access Counter Interrupt clear register"]
    pub l2_cache_acs_cnt_int_clr: L2_CACHE_ACS_CNT_INT_CLR,
    #[doc = "0x2cc - Cache Access Counter Interrupt raw register"]
    pub l2_cache_acs_cnt_int_raw: L2_CACHE_ACS_CNT_INT_RAW,
    #[doc = "0x2d0 - Cache Access Counter Interrupt status register"]
    pub l2_cache_acs_cnt_int_st: L2_CACHE_ACS_CNT_INT_ST,
    #[doc = "0x2d4 - Cache Access Fail Interrupt enable register"]
    pub l2_cache_acs_fail_int_ena: L2_CACHE_ACS_FAIL_INT_ENA,
    #[doc = "0x2d8 - L1-Cache Access Fail Interrupt clear register"]
    pub l2_cache_acs_fail_int_clr: L2_CACHE_ACS_FAIL_INT_CLR,
    #[doc = "0x2dc - Cache Access Fail Interrupt raw register"]
    pub l2_cache_acs_fail_int_raw: L2_CACHE_ACS_FAIL_INT_RAW,
    #[doc = "0x2e0 - Cache Access Fail Interrupt status register"]
    pub l2_cache_acs_fail_int_st: L2_CACHE_ACS_FAIL_INT_ST,
    #[doc = "0x2e4 - Cache Access Counter enable and clear register"]
    pub l2_cache_acs_cnt_ctrl: L2_CACHE_ACS_CNT_CTRL,
    #[doc = "0x2e8 - L2-Cache bus0 Hit-Access Counter register"]
    pub l2_ibus0_acs_hit_cnt: L2_IBUS0_ACS_HIT_CNT,
    #[doc = "0x2ec - L2-Cache bus0 Miss-Access Counter register"]
    pub l2_ibus0_acs_miss_cnt: L2_IBUS0_ACS_MISS_CNT,
    #[doc = "0x2f0 - L2-Cache bus0 Conflict-Access Counter register"]
    pub l2_ibus0_acs_conflict_cnt: L2_IBUS0_ACS_CONFLICT_CNT,
    #[doc = "0x2f4 - L2-Cache bus0 Next-Level-Access Counter register"]
    pub l2_ibus0_acs_nxtlvl_cnt: L2_IBUS0_ACS_NXTLVL_CNT,
    #[doc = "0x2f8 - L2-Cache bus1 Hit-Access Counter register"]
    pub l2_ibus1_acs_hit_cnt: L2_IBUS1_ACS_HIT_CNT,
    #[doc = "0x2fc - L2-Cache bus1 Miss-Access Counter register"]
    pub l2_ibus1_acs_miss_cnt: L2_IBUS1_ACS_MISS_CNT,
    #[doc = "0x300 - L2-Cache bus1 Conflict-Access Counter register"]
    pub l2_ibus1_acs_conflict_cnt: L2_IBUS1_ACS_CONFLICT_CNT,
    #[doc = "0x304 - L2-Cache bus1 Next-Level-Access Counter register"]
    pub l2_ibus1_acs_nxtlvl_cnt: L2_IBUS1_ACS_NXTLVL_CNT,
    #[doc = "0x308 - L2-Cache bus2 Hit-Access Counter register"]
    pub l2_ibus2_acs_hit_cnt: L2_IBUS2_ACS_HIT_CNT,
    #[doc = "0x30c - L2-Cache bus2 Miss-Access Counter register"]
    pub l2_ibus2_acs_miss_cnt: L2_IBUS2_ACS_MISS_CNT,
    #[doc = "0x310 - L2-Cache bus2 Conflict-Access Counter register"]
    pub l2_ibus2_acs_conflict_cnt: L2_IBUS2_ACS_CONFLICT_CNT,
    #[doc = "0x314 - L2-Cache bus2 Next-Level-Access Counter register"]
    pub l2_ibus2_acs_nxtlvl_cnt: L2_IBUS2_ACS_NXTLVL_CNT,
    #[doc = "0x318 - L2-Cache bus3 Hit-Access Counter register"]
    pub l2_ibus3_acs_hit_cnt: L2_IBUS3_ACS_HIT_CNT,
    #[doc = "0x31c - L2-Cache bus3 Miss-Access Counter register"]
    pub l2_ibus3_acs_miss_cnt: L2_IBUS3_ACS_MISS_CNT,
    #[doc = "0x320 - L2-Cache bus3 Conflict-Access Counter register"]
    pub l2_ibus3_acs_conflict_cnt: L2_IBUS3_ACS_CONFLICT_CNT,
    #[doc = "0x324 - L2-Cache bus3 Next-Level-Access Counter register"]
    pub l2_ibus3_acs_nxtlvl_cnt: L2_IBUS3_ACS_NXTLVL_CNT,
    #[doc = "0x328 - L2-Cache bus0 Hit-Access Counter register"]
    pub l2_dbus0_acs_hit_cnt: L2_DBUS0_ACS_HIT_CNT,
    #[doc = "0x32c - L2-Cache bus0 Miss-Access Counter register"]
    pub l2_dbus0_acs_miss_cnt: L2_DBUS0_ACS_MISS_CNT,
    #[doc = "0x330 - L2-Cache bus0 Conflict-Access Counter register"]
    pub l2_dbus0_acs_conflict_cnt: L2_DBUS0_ACS_CONFLICT_CNT,
    #[doc = "0x334 - L2-Cache bus0 Next-Level-Access Counter register"]
    pub l2_dbus0_acs_nxtlvl_cnt: L2_DBUS0_ACS_NXTLVL_CNT,
    #[doc = "0x338 - L2-Cache bus1 Hit-Access Counter register"]
    pub l2_dbus1_acs_hit_cnt: L2_DBUS1_ACS_HIT_CNT,
    #[doc = "0x33c - L2-Cache bus1 Miss-Access Counter register"]
    pub l2_dbus1_acs_miss_cnt: L2_DBUS1_ACS_MISS_CNT,
    #[doc = "0x340 - L2-Cache bus1 Conflict-Access Counter register"]
    pub l2_dbus1_acs_conflict_cnt: L2_DBUS1_ACS_CONFLICT_CNT,
    #[doc = "0x344 - L2-Cache bus1 Next-Level-Access Counter register"]
    pub l2_dbus1_acs_nxtlvl_cnt: L2_DBUS1_ACS_NXTLVL_CNT,
    #[doc = "0x348 - L2-Cache bus2 Hit-Access Counter register"]
    pub l2_dbus2_acs_hit_cnt: L2_DBUS2_ACS_HIT_CNT,
    #[doc = "0x34c - L2-Cache bus2 Miss-Access Counter register"]
    pub l2_dbus2_acs_miss_cnt: L2_DBUS2_ACS_MISS_CNT,
    #[doc = "0x350 - L2-Cache bus2 Conflict-Access Counter register"]
    pub l2_dbus2_acs_conflict_cnt: L2_DBUS2_ACS_CONFLICT_CNT,
    #[doc = "0x354 - L2-Cache bus2 Next-Level-Access Counter register"]
    pub l2_dbus2_acs_nxtlvl_cnt: L2_DBUS2_ACS_NXTLVL_CNT,
    #[doc = "0x358 - L2-Cache bus3 Hit-Access Counter register"]
    pub l2_dbus3_acs_hit_cnt: L2_DBUS3_ACS_HIT_CNT,
    #[doc = "0x35c - L2-Cache bus3 Miss-Access Counter register"]
    pub l2_dbus3_acs_miss_cnt: L2_DBUS3_ACS_MISS_CNT,
    #[doc = "0x360 - L2-Cache bus3 Conflict-Access Counter register"]
    pub l2_dbus3_acs_conflict_cnt: L2_DBUS3_ACS_CONFLICT_CNT,
    #[doc = "0x364 - L2-Cache bus3 Next-Level-Access Counter register"]
    pub l2_dbus3_acs_nxtlvl_cnt: L2_DBUS3_ACS_NXTLVL_CNT,
    #[doc = "0x368 - L2-Cache Access Fail ID/attribution information register"]
    pub l2_cache_acs_fail_id_attr: L2_CACHE_ACS_FAIL_ID_ATTR,
    #[doc = "0x36c - L2-Cache Access Fail Address information register"]
    pub l2_cache_acs_fail_addr: L2_CACHE_ACS_FAIL_ADDR,
    #[doc = "0x370 - L1-Cache Access Fail Interrupt enable register"]
    pub l2_cache_sync_preload_int_ena: L2_CACHE_SYNC_PRELOAD_INT_ENA,
    #[doc = "0x374 - Sync Preload operation Interrupt clear register"]
    pub l2_cache_sync_preload_int_clr: L2_CACHE_SYNC_PRELOAD_INT_CLR,
    #[doc = "0x378 - Sync Preload operation Interrupt raw register"]
    pub l2_cache_sync_preload_int_raw: L2_CACHE_SYNC_PRELOAD_INT_RAW,
    #[doc = "0x37c - L1-Cache Access Fail Interrupt status register"]
    pub l2_cache_sync_preload_int_st: L2_CACHE_SYNC_PRELOAD_INT_ST,
    #[doc = "0x380 - Cache Sync/Preload Operation exception register"]
    pub l2_cache_sync_preload_exception: L2_CACHE_SYNC_PRELOAD_EXCEPTION,
    #[doc = "0x384 - Cache Sync Reset control register"]
    pub l2_cache_sync_rst_ctrl: L2_CACHE_SYNC_RST_CTRL,
    #[doc = "0x388 - Cache Preload Reset control register"]
    pub l2_cache_preload_rst_ctrl: L2_CACHE_PRELOAD_RST_CTRL,
    #[doc = "0x38c - Cache Autoload buffer clear control register"]
    pub l2_cache_autoload_buf_clr_ctrl: L2_CACHE_AUTOLOAD_BUF_CLR_CTRL,
    #[doc = "0x390 - Unallocate request buffer clear registers"]
    pub l2_unallocate_buffer_clear: L2_UNALLOCATE_BUFFER_CLEAR,
    #[doc = "0x394 - L1 Cache access Attribute propagation control register"]
    pub l2_cache_access_attr_ctrl: L2_CACHE_ACCESS_ATTR_CTRL,
    #[doc = "0x398 - Cache Tag and Data memory Object control register"]
    pub l2_cache_object_ctrl: L2_CACHE_OBJECT_CTRL,
    #[doc = "0x39c - Cache Tag and Data memory way register"]
    pub l2_cache_way_object: L2_CACHE_WAY_OBJECT,
    #[doc = "0x3a0 - Cache Vaddr register"]
    pub l2_cache_vaddr: L2_CACHE_VADDR,
    #[doc = "0x3a4 - Cache Tag/data memory content register"]
    pub l2_cache_debug_bus: L2_CACHE_DEBUG_BUS,
    #[doc = "0x3a8 - USED TO SPLIT L1 CACHE AND L2 CACHE"]
    pub level_split1: LEVEL_SPLIT1,
    #[doc = "0x3ac - Clock gate control register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x3b0 - Cache redundancy signal 0 register"]
    pub redundancy_sig0: REDUNDANCY_SIG0,
    #[doc = "0x3b4 - Cache redundancy signal 1 register"]
    pub redundancy_sig1: REDUNDANCY_SIG1,
    #[doc = "0x3b8 - Cache redundancy signal 2 register"]
    pub redundancy_sig2: REDUNDANCY_SIG2,
    #[doc = "0x3bc - Cache redundancy signal 3 register"]
    pub redundancy_sig3: REDUNDANCY_SIG3,
    #[doc = "0x3c0 - Cache redundancy signal 0 register"]
    pub redundancy_sig4: REDUNDANCY_SIG4,
    _reserved241: [u8; 0x38],
    #[doc = "0x3fc - Version control register"]
    pub date: DATE,
}
#[doc = "L1_ICACHE_CTRL (r) register accessor: an alias for `Reg<L1_ICACHE_CTRL_SPEC>`"]
pub type L1_ICACHE_CTRL = crate::Reg<l1_icache_ctrl::L1_ICACHE_CTRL_SPEC>;
#[doc = "L1 instruction Cache(L1-ICache) control register"]
pub mod l1_icache_ctrl;
#[doc = "L1_CACHE_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_CTRL_SPEC>`"]
pub type L1_CACHE_CTRL = crate::Reg<l1_cache_ctrl::L1_CACHE_CTRL_SPEC>;
#[doc = "L1 data Cache(L1-Cache) control register"]
pub mod l1_cache_ctrl;
#[doc = "L1_BYPASS_CACHE_CONF (r) register accessor: an alias for `Reg<L1_BYPASS_CACHE_CONF_SPEC>`"]
pub type L1_BYPASS_CACHE_CONF = crate::Reg<l1_bypass_cache_conf::L1_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Bypass Cache configure register"]
pub mod l1_bypass_cache_conf;
#[doc = "L1_CACHE_ATOMIC_CONF (r) register accessor: an alias for `Reg<L1_CACHE_ATOMIC_CONF_SPEC>`"]
pub type L1_CACHE_ATOMIC_CONF = crate::Reg<l1_cache_atomic_conf::L1_CACHE_ATOMIC_CONF_SPEC>;
#[doc = "L1 Cache atomic feature configure register"]
pub mod l1_cache_atomic_conf;
#[doc = "L1_ICACHE_CACHESIZE_CONF (r) register accessor: an alias for `Reg<L1_ICACHE_CACHESIZE_CONF_SPEC>`"]
pub type L1_ICACHE_CACHESIZE_CONF =
    crate::Reg<l1_icache_cachesize_conf::L1_ICACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L1 instruction Cache CacheSize mode configure register"]
pub mod l1_icache_cachesize_conf;
#[doc = "L1_ICACHE_BLOCKSIZE_CONF (r) register accessor: an alias for `Reg<L1_ICACHE_BLOCKSIZE_CONF_SPEC>`"]
pub type L1_ICACHE_BLOCKSIZE_CONF =
    crate::Reg<l1_icache_blocksize_conf::L1_ICACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L1 instruction Cache BlockSize mode configure register"]
pub mod l1_icache_blocksize_conf;
#[doc = "L1_CACHE_CACHESIZE_CONF (r) register accessor: an alias for `Reg<L1_CACHE_CACHESIZE_CONF_SPEC>`"]
pub type L1_CACHE_CACHESIZE_CONF =
    crate::Reg<l1_cache_cachesize_conf::L1_CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L1 data Cache CacheSize mode configure register"]
pub mod l1_cache_cachesize_conf;
#[doc = "L1_CACHE_BLOCKSIZE_CONF (r) register accessor: an alias for `Reg<L1_CACHE_BLOCKSIZE_CONF_SPEC>`"]
pub type L1_CACHE_BLOCKSIZE_CONF =
    crate::Reg<l1_cache_blocksize_conf::L1_CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L1 data Cache BlockSize mode configure register"]
pub mod l1_cache_blocksize_conf;
#[doc = "L1_CACHE_WRAP_AROUND_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_WRAP_AROUND_CTRL_SPEC>`"]
pub type L1_CACHE_WRAP_AROUND_CTRL =
    crate::Reg<l1_cache_wrap_around_ctrl::L1_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod l1_cache_wrap_around_ctrl;
#[doc = "L1_CACHE_TAG_MEM_POWER_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>`"]
pub type L1_CACHE_TAG_MEM_POWER_CTRL =
    crate::Reg<l1_cache_tag_mem_power_ctrl::L1_CACHE_TAG_MEM_POWER_CTRL_SPEC>;
#[doc = "Cache tag memory power control register"]
pub mod l1_cache_tag_mem_power_ctrl;
#[doc = "L1_CACHE_DATA_MEM_POWER_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC>`"]
pub type L1_CACHE_DATA_MEM_POWER_CTRL =
    crate::Reg<l1_cache_data_mem_power_ctrl::L1_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
#[doc = "Cache data memory power control register"]
pub mod l1_cache_data_mem_power_ctrl;
#[doc = "L1_CACHE_FREEZE_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_FREEZE_CTRL_SPEC>`"]
pub type L1_CACHE_FREEZE_CTRL = crate::Reg<l1_cache_freeze_ctrl::L1_CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Cache Freeze control register"]
pub mod l1_cache_freeze_ctrl;
#[doc = "L1_CACHE_DATA_MEM_ACS_CONF (rw) register accessor: an alias for `Reg<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>`"]
pub type L1_CACHE_DATA_MEM_ACS_CONF =
    crate::Reg<l1_cache_data_mem_acs_conf::L1_CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Cache data memory access configure register"]
pub mod l1_cache_data_mem_acs_conf;
#[doc = "L1_CACHE_TAG_MEM_ACS_CONF (rw) register accessor: an alias for `Reg<L1_CACHE_TAG_MEM_ACS_CONF_SPEC>`"]
pub type L1_CACHE_TAG_MEM_ACS_CONF =
    crate::Reg<l1_cache_tag_mem_acs_conf::L1_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Cache tag memory access configure register"]
pub mod l1_cache_tag_mem_acs_conf;
#[doc = "L1_ICACHE0_PRELOCK_CONF (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOCK_CONF_SPEC>`"]
pub type L1_ICACHE0_PRELOCK_CONF =
    crate::Reg<l1_icache0_prelock_conf::L1_ICACHE0_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 0 prelock configure register"]
pub mod l1_icache0_prelock_conf;
#[doc = "L1_ICACHE0_PRELOCK_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE0_PRELOCK_SCT0_ADDR =
    crate::Reg<l1_icache0_prelock_sct0_addr::L1_ICACHE0_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section0 address configure register"]
pub mod l1_icache0_prelock_sct0_addr;
#[doc = "L1_ICACHE0_PRELOCK_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE0_PRELOCK_SCT1_ADDR =
    crate::Reg<l1_icache0_prelock_sct1_addr::L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section1 address configure register"]
pub mod l1_icache0_prelock_sct1_addr;
#[doc = "L1_ICACHE0_PRELOCK_SCT_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L1_ICACHE0_PRELOCK_SCT_SIZE =
    crate::Reg<l1_icache0_prelock_sct_size::L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 prelock section size configure register"]
pub mod l1_icache0_prelock_sct_size;
#[doc = "L1_ICACHE1_PRELOCK_CONF (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOCK_CONF_SPEC>`"]
pub type L1_ICACHE1_PRELOCK_CONF =
    crate::Reg<l1_icache1_prelock_conf::L1_ICACHE1_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 1 prelock configure register"]
pub mod l1_icache1_prelock_conf;
#[doc = "L1_ICACHE1_PRELOCK_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE1_PRELOCK_SCT0_ADDR =
    crate::Reg<l1_icache1_prelock_sct0_addr::L1_ICACHE1_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section0 address configure register"]
pub mod l1_icache1_prelock_sct0_addr;
#[doc = "L1_ICACHE1_PRELOCK_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE1_PRELOCK_SCT1_ADDR =
    crate::Reg<l1_icache1_prelock_sct1_addr::L1_ICACHE1_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section1 address configure register"]
pub mod l1_icache1_prelock_sct1_addr;
#[doc = "L1_ICACHE1_PRELOCK_SCT_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L1_ICACHE1_PRELOCK_SCT_SIZE =
    crate::Reg<l1_icache1_prelock_sct_size::L1_ICACHE1_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 prelock section size configure register"]
pub mod l1_icache1_prelock_sct_size;
#[doc = "L1_ICACHE2_PRELOCK_CONF (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOCK_CONF_SPEC>`"]
pub type L1_ICACHE2_PRELOCK_CONF =
    crate::Reg<l1_icache2_prelock_conf::L1_ICACHE2_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 2 prelock configure register"]
pub mod l1_icache2_prelock_conf;
#[doc = "L1_ICACHE2_PRELOCK_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE2_PRELOCK_SCT0_ADDR =
    crate::Reg<l1_icache2_prelock_sct0_addr::L1_ICACHE2_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section0 address configure register"]
pub mod l1_icache2_prelock_sct0_addr;
#[doc = "L1_ICACHE2_PRELOCK_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE2_PRELOCK_SCT1_ADDR =
    crate::Reg<l1_icache2_prelock_sct1_addr::L1_ICACHE2_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section1 address configure register"]
pub mod l1_icache2_prelock_sct1_addr;
#[doc = "L1_ICACHE2_PRELOCK_SCT_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L1_ICACHE2_PRELOCK_SCT_SIZE =
    crate::Reg<l1_icache2_prelock_sct_size::L1_ICACHE2_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 prelock section size configure register"]
pub mod l1_icache2_prelock_sct_size;
#[doc = "L1_ICACHE3_PRELOCK_CONF (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOCK_CONF_SPEC>`"]
pub type L1_ICACHE3_PRELOCK_CONF =
    crate::Reg<l1_icache3_prelock_conf::L1_ICACHE3_PRELOCK_CONF_SPEC>;
#[doc = "L1 instruction Cache 3 prelock configure register"]
pub mod l1_icache3_prelock_conf;
#[doc = "L1_ICACHE3_PRELOCK_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE3_PRELOCK_SCT0_ADDR =
    crate::Reg<l1_icache3_prelock_sct0_addr::L1_ICACHE3_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section0 address configure register"]
pub mod l1_icache3_prelock_sct0_addr;
#[doc = "L1_ICACHE3_PRELOCK_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE3_PRELOCK_SCT1_ADDR =
    crate::Reg<l1_icache3_prelock_sct1_addr::L1_ICACHE3_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section1 address configure register"]
pub mod l1_icache3_prelock_sct1_addr;
#[doc = "L1_ICACHE3_PRELOCK_SCT_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L1_ICACHE3_PRELOCK_SCT_SIZE =
    crate::Reg<l1_icache3_prelock_sct_size::L1_ICACHE3_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 prelock section size configure register"]
pub mod l1_icache3_prelock_sct_size;
#[doc = "L1_CACHE_PRELOCK_CONF (rw) register accessor: an alias for `Reg<L1_CACHE_PRELOCK_CONF_SPEC>`"]
pub type L1_CACHE_PRELOCK_CONF = crate::Reg<l1_cache_prelock_conf::L1_CACHE_PRELOCK_CONF_SPEC>;
#[doc = "L1 Cache prelock configure register"]
pub mod l1_cache_prelock_conf;
#[doc = "L1_CACHE_PRELOCK_SCT0_ADDR (rw) register accessor: an alias for `Reg<L1_CACHE_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L1_CACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<l1_cache_prelock_sct0_addr::L1_CACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L1 Cache prelock section0 address configure register"]
pub mod l1_cache_prelock_sct0_addr;
#[doc = "L1_DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: an alias for `Reg<L1_DCACHE_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L1_DCACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<l1_dcache_prelock_sct1_addr::L1_DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L1 Cache prelock section1 address configure register"]
pub mod l1_dcache_prelock_sct1_addr;
#[doc = "L1_DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: an alias for `Reg<L1_DCACHE_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L1_DCACHE_PRELOCK_SCT_SIZE =
    crate::Reg<l1_dcache_prelock_sct_size::L1_DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L1 Cache prelock section size configure register"]
pub mod l1_dcache_prelock_sct_size;
#[doc = "CACHE_LOCK_CTRL (rw) register accessor: an alias for `Reg<CACHE_LOCK_CTRL_SPEC>`"]
pub type CACHE_LOCK_CTRL = crate::Reg<cache_lock_ctrl::CACHE_LOCK_CTRL_SPEC>;
#[doc = "Lock-class (manual lock) operation control register"]
pub mod cache_lock_ctrl;
#[doc = "CACHE_LOCK_MAP (rw) register accessor: an alias for `Reg<CACHE_LOCK_MAP_SPEC>`"]
pub type CACHE_LOCK_MAP = crate::Reg<cache_lock_map::CACHE_LOCK_MAP_SPEC>;
#[doc = "Lock (manual lock) map configure register"]
pub mod cache_lock_map;
#[doc = "CACHE_LOCK_ADDR (rw) register accessor: an alias for `Reg<CACHE_LOCK_ADDR_SPEC>`"]
pub type CACHE_LOCK_ADDR = crate::Reg<cache_lock_addr::CACHE_LOCK_ADDR_SPEC>;
#[doc = "Lock (manual lock) address configure register"]
pub mod cache_lock_addr;
#[doc = "CACHE_LOCK_SIZE (rw) register accessor: an alias for `Reg<CACHE_LOCK_SIZE_SPEC>`"]
pub type CACHE_LOCK_SIZE = crate::Reg<cache_lock_size::CACHE_LOCK_SIZE_SPEC>;
#[doc = "Lock (manual lock) size configure register"]
pub mod cache_lock_size;
#[doc = "CACHE_SYNC_CTRL (rw) register accessor: an alias for `Reg<CACHE_SYNC_CTRL_SPEC>`"]
pub type CACHE_SYNC_CTRL = crate::Reg<cache_sync_ctrl::CACHE_SYNC_CTRL_SPEC>;
#[doc = "Sync-class operation control register"]
pub mod cache_sync_ctrl;
#[doc = "CACHE_SYNC_MAP (rw) register accessor: an alias for `Reg<CACHE_SYNC_MAP_SPEC>`"]
pub type CACHE_SYNC_MAP = crate::Reg<cache_sync_map::CACHE_SYNC_MAP_SPEC>;
#[doc = "Sync map configure register"]
pub mod cache_sync_map;
#[doc = "CACHE_SYNC_ADDR (rw) register accessor: an alias for `Reg<CACHE_SYNC_ADDR_SPEC>`"]
pub type CACHE_SYNC_ADDR = crate::Reg<cache_sync_addr::CACHE_SYNC_ADDR_SPEC>;
#[doc = "Sync address configure register"]
pub mod cache_sync_addr;
#[doc = "CACHE_SYNC_SIZE (rw) register accessor: an alias for `Reg<CACHE_SYNC_SIZE_SPEC>`"]
pub type CACHE_SYNC_SIZE = crate::Reg<cache_sync_size::CACHE_SYNC_SIZE_SPEC>;
#[doc = "Sync size configure register"]
pub mod cache_sync_size;
#[doc = "L1_ICACHE0_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L1_ICACHE0_PRELOAD_CTRL_SPEC>`"]
pub type L1_ICACHE0_PRELOAD_CTRL =
    crate::Reg<l1_icache0_preload_ctrl::L1_ICACHE0_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 0 preload-operation control register"]
pub mod l1_icache0_preload_ctrl;
#[doc = "L1_ICACHE0_PRELOAD_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOAD_ADDR_SPEC>`"]
pub type L1_ICACHE0_PRELOAD_ADDR =
    crate::Reg<l1_icache0_preload_addr::L1_ICACHE0_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 preload address configure register"]
pub mod l1_icache0_preload_addr;
#[doc = "L1_ICACHE0_PRELOAD_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE0_PRELOAD_SIZE_SPEC>`"]
pub type L1_ICACHE0_PRELOAD_SIZE =
    crate::Reg<l1_icache0_preload_size::L1_ICACHE0_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 preload size configure register"]
pub mod l1_icache0_preload_size;
#[doc = "L1_ICACHE1_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L1_ICACHE1_PRELOAD_CTRL_SPEC>`"]
pub type L1_ICACHE1_PRELOAD_CTRL =
    crate::Reg<l1_icache1_preload_ctrl::L1_ICACHE1_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 1 preload-operation control register"]
pub mod l1_icache1_preload_ctrl;
#[doc = "L1_ICACHE1_PRELOAD_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOAD_ADDR_SPEC>`"]
pub type L1_ICACHE1_PRELOAD_ADDR =
    crate::Reg<l1_icache1_preload_addr::L1_ICACHE1_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 preload address configure register"]
pub mod l1_icache1_preload_addr;
#[doc = "L1_ICACHE1_PRELOAD_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE1_PRELOAD_SIZE_SPEC>`"]
pub type L1_ICACHE1_PRELOAD_SIZE =
    crate::Reg<l1_icache1_preload_size::L1_ICACHE1_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 preload size configure register"]
pub mod l1_icache1_preload_size;
#[doc = "L1_ICACHE2_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L1_ICACHE2_PRELOAD_CTRL_SPEC>`"]
pub type L1_ICACHE2_PRELOAD_CTRL =
    crate::Reg<l1_icache2_preload_ctrl::L1_ICACHE2_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 2 preload-operation control register"]
pub mod l1_icache2_preload_ctrl;
#[doc = "L1_ICACHE2_PRELOAD_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOAD_ADDR_SPEC>`"]
pub type L1_ICACHE2_PRELOAD_ADDR =
    crate::Reg<l1_icache2_preload_addr::L1_ICACHE2_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 preload address configure register"]
pub mod l1_icache2_preload_addr;
#[doc = "L1_ICACHE2_PRELOAD_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE2_PRELOAD_SIZE_SPEC>`"]
pub type L1_ICACHE2_PRELOAD_SIZE =
    crate::Reg<l1_icache2_preload_size::L1_ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 preload size configure register"]
pub mod l1_icache2_preload_size;
#[doc = "L1_ICACHE3_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L1_ICACHE3_PRELOAD_CTRL_SPEC>`"]
pub type L1_ICACHE3_PRELOAD_CTRL =
    crate::Reg<l1_icache3_preload_ctrl::L1_ICACHE3_PRELOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 3 preload-operation control register"]
pub mod l1_icache3_preload_ctrl;
#[doc = "L1_ICACHE3_PRELOAD_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOAD_ADDR_SPEC>`"]
pub type L1_ICACHE3_PRELOAD_ADDR =
    crate::Reg<l1_icache3_preload_addr::L1_ICACHE3_PRELOAD_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 preload address configure register"]
pub mod l1_icache3_preload_addr;
#[doc = "L1_ICACHE3_PRELOAD_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE3_PRELOAD_SIZE_SPEC>`"]
pub type L1_ICACHE3_PRELOAD_SIZE =
    crate::Reg<l1_icache3_preload_size::L1_ICACHE3_PRELOAD_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 preload size configure register"]
pub mod l1_icache3_preload_size;
#[doc = "L1_CACHE_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_PRELOAD_CTRL_SPEC>`"]
pub type L1_CACHE_PRELOAD_CTRL = crate::Reg<l1_cache_preload_ctrl::L1_CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "L1 Cache preload-operation control register"]
pub mod l1_cache_preload_ctrl;
#[doc = "L1_DCACHE_PRELOAD_ADDR (rw) register accessor: an alias for `Reg<L1_DCACHE_PRELOAD_ADDR_SPEC>`"]
pub type L1_DCACHE_PRELOAD_ADDR = crate::Reg<l1_dcache_preload_addr::L1_DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "L1 Cache preload address configure register"]
pub mod l1_dcache_preload_addr;
#[doc = "L1_DCACHE_PRELOAD_SIZE (rw) register accessor: an alias for `Reg<L1_DCACHE_PRELOAD_SIZE_SPEC>`"]
pub type L1_DCACHE_PRELOAD_SIZE = crate::Reg<l1_dcache_preload_size::L1_DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "L1 Cache preload size configure register"]
pub mod l1_dcache_preload_size;
#[doc = "L1_ICACHE0_AUTOLOAD_CTRL (r) register accessor: an alias for `Reg<L1_ICACHE0_AUTOLOAD_CTRL_SPEC>`"]
pub type L1_ICACHE0_AUTOLOAD_CTRL =
    crate::Reg<l1_icache0_autoload_ctrl::L1_ICACHE0_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 0 autoload-operation control register"]
pub mod l1_icache0_autoload_ctrl;
#[doc = "L1_ICACHE0_AUTOLOAD_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE0_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l1_icache0_autoload_sct0_addr::L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 0 address configure register"]
pub mod l1_icache0_autoload_sct0_addr;
#[doc = "L1_ICACHE0_AUTOLOAD_SCT0_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE0_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L1_ICACHE0_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l1_icache0_autoload_sct0_size::L1_ICACHE0_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 0 size configure register"]
pub mod l1_icache0_autoload_sct0_size;
#[doc = "L1_ICACHE0_AUTOLOAD_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE0_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l1_icache0_autoload_sct1_addr::L1_ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 1 address configure register"]
pub mod l1_icache0_autoload_sct1_addr;
#[doc = "L1_ICACHE0_AUTOLOAD_SCT1_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE0_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L1_ICACHE0_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l1_icache0_autoload_sct1_size::L1_ICACHE0_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 0 autoload section 1 size configure register"]
pub mod l1_icache0_autoload_sct1_size;
#[doc = "L1_ICACHE1_AUTOLOAD_CTRL (r) register accessor: an alias for `Reg<L1_ICACHE1_AUTOLOAD_CTRL_SPEC>`"]
pub type L1_ICACHE1_AUTOLOAD_CTRL =
    crate::Reg<l1_icache1_autoload_ctrl::L1_ICACHE1_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 1 autoload-operation control register"]
pub mod l1_icache1_autoload_ctrl;
#[doc = "L1_ICACHE1_AUTOLOAD_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE1_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l1_icache1_autoload_sct0_addr::L1_ICACHE1_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 0 address configure register"]
pub mod l1_icache1_autoload_sct0_addr;
#[doc = "L1_ICACHE1_AUTOLOAD_SCT0_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE1_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L1_ICACHE1_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l1_icache1_autoload_sct0_size::L1_ICACHE1_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 0 size configure register"]
pub mod l1_icache1_autoload_sct0_size;
#[doc = "L1_ICACHE1_AUTOLOAD_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE1_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l1_icache1_autoload_sct1_addr::L1_ICACHE1_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 1 address configure register"]
pub mod l1_icache1_autoload_sct1_addr;
#[doc = "L1_ICACHE1_AUTOLOAD_SCT1_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L1_ICACHE1_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l1_icache1_autoload_sct1_size::L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 1 autoload section 1 size configure register"]
pub mod l1_icache1_autoload_sct1_size;
#[doc = "L1_ICACHE2_AUTOLOAD_CTRL (r) register accessor: an alias for `Reg<L1_ICACHE2_AUTOLOAD_CTRL_SPEC>`"]
pub type L1_ICACHE2_AUTOLOAD_CTRL =
    crate::Reg<l1_icache2_autoload_ctrl::L1_ICACHE2_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 2 autoload-operation control register"]
pub mod l1_icache2_autoload_ctrl;
#[doc = "L1_ICACHE2_AUTOLOAD_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE2_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l1_icache2_autoload_sct0_addr::L1_ICACHE2_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 0 address configure register"]
pub mod l1_icache2_autoload_sct0_addr;
#[doc = "L1_ICACHE2_AUTOLOAD_SCT0_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L1_ICACHE2_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l1_icache2_autoload_sct0_size::L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 0 size configure register"]
pub mod l1_icache2_autoload_sct0_size;
#[doc = "L1_ICACHE2_AUTOLOAD_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE2_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l1_icache2_autoload_sct1_addr::L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 1 address configure register"]
pub mod l1_icache2_autoload_sct1_addr;
#[doc = "L1_ICACHE2_AUTOLOAD_SCT1_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L1_ICACHE2_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l1_icache2_autoload_sct1_size::L1_ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 2 autoload section 1 size configure register"]
pub mod l1_icache2_autoload_sct1_size;
#[doc = "L1_ICACHE3_AUTOLOAD_CTRL (r) register accessor: an alias for `Reg<L1_ICACHE3_AUTOLOAD_CTRL_SPEC>`"]
pub type L1_ICACHE3_AUTOLOAD_CTRL =
    crate::Reg<l1_icache3_autoload_ctrl::L1_ICACHE3_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 instruction Cache 3 autoload-operation control register"]
pub mod l1_icache3_autoload_ctrl;
#[doc = "L1_ICACHE3_AUTOLOAD_SCT0_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L1_ICACHE3_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l1_icache3_autoload_sct0_addr::L1_ICACHE3_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 0 address configure register"]
pub mod l1_icache3_autoload_sct0_addr;
#[doc = "L1_ICACHE3_AUTOLOAD_SCT0_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L1_ICACHE3_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l1_icache3_autoload_sct0_size::L1_ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 0 size configure register"]
pub mod l1_icache3_autoload_sct0_size;
#[doc = "L1_ICACHE3_AUTOLOAD_SCT1_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L1_ICACHE3_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l1_icache3_autoload_sct1_addr::L1_ICACHE3_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 1 address configure register"]
pub mod l1_icache3_autoload_sct1_addr;
#[doc = "L1_ICACHE3_AUTOLOAD_SCT1_SIZE (r) register accessor: an alias for `Reg<L1_ICACHE3_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L1_ICACHE3_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l1_icache3_autoload_sct1_size::L1_ICACHE3_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 instruction Cache 3 autoload section 1 size configure register"]
pub mod l1_icache3_autoload_sct1_size;
#[doc = "L1_CACHE_AUTOLOAD_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_CTRL_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_CTRL = crate::Reg<l1_cache_autoload_ctrl::L1_CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "L1 Cache autoload-operation control register"]
pub mod l1_cache_autoload_ctrl;
#[doc = "L1_CACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l1_cache_autoload_sct0_addr::L1_CACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 0 address configure register"]
pub mod l1_cache_autoload_sct0_addr;
#[doc = "L1_CACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l1_cache_autoload_sct0_size::L1_CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 0 size configure register"]
pub mod l1_cache_autoload_sct0_size;
#[doc = "L1_CACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l1_cache_autoload_sct1_addr::L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 1 address configure register"]
pub mod l1_cache_autoload_sct1_addr;
#[doc = "L1_CACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l1_cache_autoload_sct1_size::L1_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 1 size configure register"]
pub mod l1_cache_autoload_sct1_size;
#[doc = "L1_CACHE_AUTOLOAD_SCT2_ADDR (r) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT2_ADDR_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT2_ADDR =
    crate::Reg<l1_cache_autoload_sct2_addr::L1_CACHE_AUTOLOAD_SCT2_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 2 address configure register"]
pub mod l1_cache_autoload_sct2_addr;
#[doc = "L1_CACHE_AUTOLOAD_SCT2_SIZE (r) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT2_SIZE =
    crate::Reg<l1_cache_autoload_sct2_size::L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 2 size configure register"]
pub mod l1_cache_autoload_sct2_size;
#[doc = "L1_CACHE_AUTOLOAD_SCT3_ADDR (r) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT3_ADDR =
    crate::Reg<l1_cache_autoload_sct3_addr::L1_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "L1 Cache autoload section 1 address configure register"]
pub mod l1_cache_autoload_sct3_addr;
#[doc = "L1_CACHE_AUTOLOAD_SCT3_SIZE (r) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_SCT3_SIZE =
    crate::Reg<l1_cache_autoload_sct3_size::L1_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "L1 Cache autoload section 1 size configure register"]
pub mod l1_cache_autoload_sct3_size;
#[doc = "L1_CACHE_ACS_CNT_INT_ENA (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_CNT_INT_ENA_SPEC>`"]
pub type L1_CACHE_ACS_CNT_INT_ENA =
    crate::Reg<l1_cache_acs_cnt_int_ena::L1_CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Cache Access Counter Interrupt enable register"]
pub mod l1_cache_acs_cnt_int_ena;
#[doc = "L1_CACHE_ACS_CNT_INT_CLR (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_CNT_INT_CLR_SPEC>`"]
pub type L1_CACHE_ACS_CNT_INT_CLR =
    crate::Reg<l1_cache_acs_cnt_int_clr::L1_CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Cache Access Counter Interrupt clear register"]
pub mod l1_cache_acs_cnt_int_clr;
#[doc = "L1_CACHE_ACS_CNT_INT_RAW (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_CNT_INT_RAW_SPEC>`"]
pub type L1_CACHE_ACS_CNT_INT_RAW =
    crate::Reg<l1_cache_acs_cnt_int_raw::L1_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Cache Access Counter Interrupt raw register"]
pub mod l1_cache_acs_cnt_int_raw;
#[doc = "L1_CACHE_ACS_CNT_INT_ST (r) register accessor: an alias for `Reg<L1_CACHE_ACS_CNT_INT_ST_SPEC>`"]
pub type L1_CACHE_ACS_CNT_INT_ST =
    crate::Reg<l1_cache_acs_cnt_int_st::L1_CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Cache Access Counter Interrupt status register"]
pub mod l1_cache_acs_cnt_int_st;
#[doc = "L1_CACHE_ACS_FAIL_INT_ENA (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_FAIL_INT_ENA_SPEC>`"]
pub type L1_CACHE_ACS_FAIL_INT_ENA =
    crate::Reg<l1_cache_acs_fail_int_ena::L1_CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Cache Access Fail Interrupt enable register"]
pub mod l1_cache_acs_fail_int_ena;
#[doc = "L1_CACHE_ACS_FAIL_INT_CLR (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_FAIL_INT_CLR_SPEC>`"]
pub type L1_CACHE_ACS_FAIL_INT_CLR =
    crate::Reg<l1_cache_acs_fail_int_clr::L1_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt clear register"]
pub mod l1_cache_acs_fail_int_clr;
#[doc = "L1_CACHE_ACS_FAIL_INT_RAW (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_FAIL_INT_RAW_SPEC>`"]
pub type L1_CACHE_ACS_FAIL_INT_RAW =
    crate::Reg<l1_cache_acs_fail_int_raw::L1_CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Cache Access Fail Interrupt raw register"]
pub mod l1_cache_acs_fail_int_raw;
#[doc = "L1_CACHE_ACS_FAIL_INT_ST (r) register accessor: an alias for `Reg<L1_CACHE_ACS_FAIL_INT_ST_SPEC>`"]
pub type L1_CACHE_ACS_FAIL_INT_ST =
    crate::Reg<l1_cache_acs_fail_int_st::L1_CACHE_ACS_FAIL_INT_ST_SPEC>;
#[doc = "Cache Access Fail Interrupt status register"]
pub mod l1_cache_acs_fail_int_st;
#[doc = "L1_CACHE_ACS_CNT_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_ACS_CNT_CTRL_SPEC>`"]
pub type L1_CACHE_ACS_CNT_CTRL = crate::Reg<l1_cache_acs_cnt_ctrl::L1_CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Cache Access Counter enable and clear register"]
pub mod l1_cache_acs_cnt_ctrl;
#[doc = "L1_IBUS0_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_IBUS0_ACS_HIT_CNT_SPEC>`"]
pub type L1_IBUS0_ACS_HIT_CNT = crate::Reg<l1_ibus0_acs_hit_cnt::L1_IBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus0 Hit-Access Counter register"]
pub mod l1_ibus0_acs_hit_cnt;
#[doc = "L1_IBUS0_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_IBUS0_ACS_MISS_CNT_SPEC>`"]
pub type L1_IBUS0_ACS_MISS_CNT = crate::Reg<l1_ibus0_acs_miss_cnt::L1_IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus0 Miss-Access Counter register"]
pub mod l1_ibus0_acs_miss_cnt;
#[doc = "L1_IBUS0_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_IBUS0_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_IBUS0_ACS_CONFLICT_CNT =
    crate::Reg<l1_ibus0_acs_conflict_cnt::L1_IBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus0 Conflict-Access Counter register"]
pub mod l1_ibus0_acs_conflict_cnt;
#[doc = "L1_IBUS0_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_IBUS0_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_IBUS0_ACS_NXTLVL_CNT =
    crate::Reg<l1_ibus0_acs_nxtlvl_cnt::L1_IBUS0_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-ICache bus0 Next-Level-Access Counter register"]
pub mod l1_ibus0_acs_nxtlvl_cnt;
#[doc = "L1_IBUS1_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_IBUS1_ACS_HIT_CNT_SPEC>`"]
pub type L1_IBUS1_ACS_HIT_CNT = crate::Reg<l1_ibus1_acs_hit_cnt::L1_IBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus1 Hit-Access Counter register"]
pub mod l1_ibus1_acs_hit_cnt;
#[doc = "L1_IBUS1_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_IBUS1_ACS_MISS_CNT_SPEC>`"]
pub type L1_IBUS1_ACS_MISS_CNT = crate::Reg<l1_ibus1_acs_miss_cnt::L1_IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus1 Miss-Access Counter register"]
pub mod l1_ibus1_acs_miss_cnt;
#[doc = "L1_IBUS1_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_IBUS1_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_IBUS1_ACS_CONFLICT_CNT =
    crate::Reg<l1_ibus1_acs_conflict_cnt::L1_IBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus1 Conflict-Access Counter register"]
pub mod l1_ibus1_acs_conflict_cnt;
#[doc = "L1_IBUS1_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_IBUS1_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_IBUS1_ACS_NXTLVL_CNT =
    crate::Reg<l1_ibus1_acs_nxtlvl_cnt::L1_IBUS1_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-ICache bus1 Next-Level-Access Counter register"]
pub mod l1_ibus1_acs_nxtlvl_cnt;
#[doc = "L1_IBUS2_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_IBUS2_ACS_HIT_CNT_SPEC>`"]
pub type L1_IBUS2_ACS_HIT_CNT = crate::Reg<l1_ibus2_acs_hit_cnt::L1_IBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus2 Hit-Access Counter register"]
pub mod l1_ibus2_acs_hit_cnt;
#[doc = "L1_IBUS2_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_IBUS2_ACS_MISS_CNT_SPEC>`"]
pub type L1_IBUS2_ACS_MISS_CNT = crate::Reg<l1_ibus2_acs_miss_cnt::L1_IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus2 Miss-Access Counter register"]
pub mod l1_ibus2_acs_miss_cnt;
#[doc = "L1_IBUS2_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_IBUS2_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_IBUS2_ACS_CONFLICT_CNT =
    crate::Reg<l1_ibus2_acs_conflict_cnt::L1_IBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus2 Conflict-Access Counter register"]
pub mod l1_ibus2_acs_conflict_cnt;
#[doc = "L1_IBUS2_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_IBUS2_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_IBUS2_ACS_NXTLVL_CNT =
    crate::Reg<l1_ibus2_acs_nxtlvl_cnt::L1_IBUS2_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-ICache bus2 Next-Level-Access Counter register"]
pub mod l1_ibus2_acs_nxtlvl_cnt;
#[doc = "L1_IBUS3_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_IBUS3_ACS_HIT_CNT_SPEC>`"]
pub type L1_IBUS3_ACS_HIT_CNT = crate::Reg<l1_ibus3_acs_hit_cnt::L1_IBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L1-ICache bus3 Hit-Access Counter register"]
pub mod l1_ibus3_acs_hit_cnt;
#[doc = "L1_IBUS3_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_IBUS3_ACS_MISS_CNT_SPEC>`"]
pub type L1_IBUS3_ACS_MISS_CNT = crate::Reg<l1_ibus3_acs_miss_cnt::L1_IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L1-ICache bus3 Miss-Access Counter register"]
pub mod l1_ibus3_acs_miss_cnt;
#[doc = "L1_IBUS3_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_IBUS3_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_IBUS3_ACS_CONFLICT_CNT =
    crate::Reg<l1_ibus3_acs_conflict_cnt::L1_IBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-ICache bus3 Conflict-Access Counter register"]
pub mod l1_ibus3_acs_conflict_cnt;
#[doc = "L1_IBUS3_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_IBUS3_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_IBUS3_ACS_NXTLVL_CNT =
    crate::Reg<l1_ibus3_acs_nxtlvl_cnt::L1_IBUS3_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-ICache bus3 Next-Level-Access Counter register"]
pub mod l1_ibus3_acs_nxtlvl_cnt;
#[doc = "L1_BUS0_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_BUS0_ACS_HIT_CNT_SPEC>`"]
pub type L1_BUS0_ACS_HIT_CNT = crate::Reg<l1_bus0_acs_hit_cnt::L1_BUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L1-Cache bus0 Hit-Access Counter register"]
pub mod l1_bus0_acs_hit_cnt;
#[doc = "L1_BUS0_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_BUS0_ACS_MISS_CNT_SPEC>`"]
pub type L1_BUS0_ACS_MISS_CNT = crate::Reg<l1_bus0_acs_miss_cnt::L1_BUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L1-Cache bus0 Miss-Access Counter register"]
pub mod l1_bus0_acs_miss_cnt;
#[doc = "L1_BUS0_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_BUS0_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_BUS0_ACS_CONFLICT_CNT =
    crate::Reg<l1_bus0_acs_conflict_cnt::L1_BUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-Cache bus0 Conflict-Access Counter register"]
pub mod l1_bus0_acs_conflict_cnt;
#[doc = "L1_BUS0_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_BUS0_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_BUS0_ACS_NXTLVL_CNT = crate::Reg<l1_bus0_acs_nxtlvl_cnt::L1_BUS0_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-Cache bus0 Next-Level-Access Counter register"]
pub mod l1_bus0_acs_nxtlvl_cnt;
#[doc = "L1_BUS1_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_BUS1_ACS_HIT_CNT_SPEC>`"]
pub type L1_BUS1_ACS_HIT_CNT = crate::Reg<l1_bus1_acs_hit_cnt::L1_BUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L1-Cache bus1 Hit-Access Counter register"]
pub mod l1_bus1_acs_hit_cnt;
#[doc = "L1_BUS1_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_BUS1_ACS_MISS_CNT_SPEC>`"]
pub type L1_BUS1_ACS_MISS_CNT = crate::Reg<l1_bus1_acs_miss_cnt::L1_BUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L1-Cache bus1 Miss-Access Counter register"]
pub mod l1_bus1_acs_miss_cnt;
#[doc = "L1_BUS1_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_BUS1_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_BUS1_ACS_CONFLICT_CNT =
    crate::Reg<l1_bus1_acs_conflict_cnt::L1_BUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-Cache bus1 Conflict-Access Counter register"]
pub mod l1_bus1_acs_conflict_cnt;
#[doc = "L1_BUS1_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_BUS1_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_BUS1_ACS_NXTLVL_CNT = crate::Reg<l1_bus1_acs_nxtlvl_cnt::L1_BUS1_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-Cache bus1 Next-Level-Access Counter register"]
pub mod l1_bus1_acs_nxtlvl_cnt;
#[doc = "L1_DBUS2_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_DBUS2_ACS_HIT_CNT_SPEC>`"]
pub type L1_DBUS2_ACS_HIT_CNT = crate::Reg<l1_dbus2_acs_hit_cnt::L1_DBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L1-DCache bus2 Hit-Access Counter register"]
pub mod l1_dbus2_acs_hit_cnt;
#[doc = "L1_DBUS2_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_DBUS2_ACS_MISS_CNT_SPEC>`"]
pub type L1_DBUS2_ACS_MISS_CNT = crate::Reg<l1_dbus2_acs_miss_cnt::L1_DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L1-DCache bus2 Miss-Access Counter register"]
pub mod l1_dbus2_acs_miss_cnt;
#[doc = "L1_DBUS2_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_DBUS2_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_DBUS2_ACS_CONFLICT_CNT =
    crate::Reg<l1_dbus2_acs_conflict_cnt::L1_DBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-DCache bus2 Conflict-Access Counter register"]
pub mod l1_dbus2_acs_conflict_cnt;
#[doc = "L1_DBUS2_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_DBUS2_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_DBUS2_ACS_NXTLVL_CNT =
    crate::Reg<l1_dbus2_acs_nxtlvl_cnt::L1_DBUS2_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-DCache bus2 Next-Level-Access Counter register"]
pub mod l1_dbus2_acs_nxtlvl_cnt;
#[doc = "L1_DBUS3_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L1_DBUS3_ACS_HIT_CNT_SPEC>`"]
pub type L1_DBUS3_ACS_HIT_CNT = crate::Reg<l1_dbus3_acs_hit_cnt::L1_DBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L1-DCache bus3 Hit-Access Counter register"]
pub mod l1_dbus3_acs_hit_cnt;
#[doc = "L1_DBUS3_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L1_DBUS3_ACS_MISS_CNT_SPEC>`"]
pub type L1_DBUS3_ACS_MISS_CNT = crate::Reg<l1_dbus3_acs_miss_cnt::L1_DBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L1-DCache bus3 Miss-Access Counter register"]
pub mod l1_dbus3_acs_miss_cnt;
#[doc = "L1_DBUS3_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L1_DBUS3_ACS_CONFLICT_CNT_SPEC>`"]
pub type L1_DBUS3_ACS_CONFLICT_CNT =
    crate::Reg<l1_dbus3_acs_conflict_cnt::L1_DBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L1-DCache bus3 Conflict-Access Counter register"]
pub mod l1_dbus3_acs_conflict_cnt;
#[doc = "L1_DBUS3_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L1_DBUS3_ACS_NXTLVL_CNT_SPEC>`"]
pub type L1_DBUS3_ACS_NXTLVL_CNT =
    crate::Reg<l1_dbus3_acs_nxtlvl_cnt::L1_DBUS3_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L1-DCache bus3 Next-Level-Access Counter register"]
pub mod l1_dbus3_acs_nxtlvl_cnt;
#[doc = "L1_ICACHE0_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L1_ICACHE0_ACS_FAIL_ID_ATTR =
    crate::Reg<l1_icache0_acs_fail_id_attr::L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod l1_icache0_acs_fail_id_attr;
#[doc = "L1_ICACHE0_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE0_ACS_FAIL_ADDR_SPEC>`"]
pub type L1_ICACHE0_ACS_FAIL_ADDR =
    crate::Reg<l1_icache0_acs_fail_addr::L1_ICACHE0_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod l1_icache0_acs_fail_addr;
#[doc = "L1_ICACHE1_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L1_ICACHE1_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L1_ICACHE1_ACS_FAIL_ID_ATTR =
    crate::Reg<l1_icache1_acs_fail_id_attr::L1_ICACHE1_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod l1_icache1_acs_fail_id_attr;
#[doc = "L1_ICACHE1_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE1_ACS_FAIL_ADDR_SPEC>`"]
pub type L1_ICACHE1_ACS_FAIL_ADDR =
    crate::Reg<l1_icache1_acs_fail_addr::L1_ICACHE1_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod l1_icache1_acs_fail_addr;
#[doc = "L1_ICACHE2_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L1_ICACHE2_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L1_ICACHE2_ACS_FAIL_ID_ATTR =
    crate::Reg<l1_icache2_acs_fail_id_attr::L1_ICACHE2_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod l1_icache2_acs_fail_id_attr;
#[doc = "L1_ICACHE2_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE2_ACS_FAIL_ADDR_SPEC>`"]
pub type L1_ICACHE2_ACS_FAIL_ADDR =
    crate::Reg<l1_icache2_acs_fail_addr::L1_ICACHE2_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod l1_icache2_acs_fail_addr;
#[doc = "L1_ICACHE3_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L1_ICACHE3_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L1_ICACHE3_ACS_FAIL_ID_ATTR =
    crate::Reg<l1_icache3_acs_fail_id_attr::L1_ICACHE3_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-ICache0 Access Fail ID/attribution information register"]
pub mod l1_icache3_acs_fail_id_attr;
#[doc = "L1_ICACHE3_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L1_ICACHE3_ACS_FAIL_ADDR_SPEC>`"]
pub type L1_ICACHE3_ACS_FAIL_ADDR =
    crate::Reg<l1_icache3_acs_fail_addr::L1_ICACHE3_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-ICache0 Access Fail Address information register"]
pub mod l1_icache3_acs_fail_addr;
#[doc = "L1_CACHE_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L1_CACHE_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L1_CACHE_ACS_FAIL_ID_ATTR =
    crate::Reg<l1_cache_acs_fail_id_attr::L1_CACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L1-Cache Access Fail ID/attribution information register"]
pub mod l1_cache_acs_fail_id_attr;
#[doc = "L1_DCACHE_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L1_DCACHE_ACS_FAIL_ADDR_SPEC>`"]
pub type L1_DCACHE_ACS_FAIL_ADDR =
    crate::Reg<l1_dcache_acs_fail_addr::L1_DCACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "L1-Cache Access Fail Address information register"]
pub mod l1_dcache_acs_fail_addr;
#[doc = "L1_CACHE_SYNC_PRELOAD_INT_ENA (rw) register accessor: an alias for `Reg<L1_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>`"]
pub type L1_CACHE_SYNC_PRELOAD_INT_ENA =
    crate::Reg<l1_cache_sync_preload_int_ena::L1_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt enable register"]
pub mod l1_cache_sync_preload_int_ena;
#[doc = "L1_CACHE_SYNC_PRELOAD_INT_CLR (rw) register accessor: an alias for `Reg<L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>`"]
pub type L1_CACHE_SYNC_PRELOAD_INT_CLR =
    crate::Reg<l1_cache_sync_preload_int_clr::L1_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Sync Preload operation Interrupt clear register"]
pub mod l1_cache_sync_preload_int_clr;
#[doc = "L1_CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: an alias for `Reg<L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>`"]
pub type L1_CACHE_SYNC_PRELOAD_INT_RAW =
    crate::Reg<l1_cache_sync_preload_int_raw::L1_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Sync Preload operation Interrupt raw register"]
pub mod l1_cache_sync_preload_int_raw;
#[doc = "L1_CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: an alias for `Reg<L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>`"]
pub type L1_CACHE_SYNC_PRELOAD_INT_ST =
    crate::Reg<l1_cache_sync_preload_int_st::L1_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt status register"]
pub mod l1_cache_sync_preload_int_st;
#[doc = "L1_CACHE_SYNC_PRELOAD_EXCEPTION (r) register accessor: an alias for `Reg<L1_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>`"]
pub type L1_CACHE_SYNC_PRELOAD_EXCEPTION =
    crate::Reg<l1_cache_sync_preload_exception::L1_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Cache Sync/Preload Operation exception register"]
pub mod l1_cache_sync_preload_exception;
#[doc = "L1_CACHE_SYNC_RST_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_SYNC_RST_CTRL_SPEC>`"]
pub type L1_CACHE_SYNC_RST_CTRL = crate::Reg<l1_cache_sync_rst_ctrl::L1_CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Cache Sync Reset control register"]
pub mod l1_cache_sync_rst_ctrl;
#[doc = "L1_CACHE_PRELOAD_RST_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_PRELOAD_RST_CTRL_SPEC>`"]
pub type L1_CACHE_PRELOAD_RST_CTRL =
    crate::Reg<l1_cache_preload_rst_ctrl::L1_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Cache Preload Reset control register"]
pub mod l1_cache_preload_rst_ctrl;
#[doc = "L1_CACHE_AUTOLOAD_BUF_CLR_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>`"]
pub type L1_CACHE_AUTOLOAD_BUF_CLR_CTRL =
    crate::Reg<l1_cache_autoload_buf_clr_ctrl::L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Cache Autoload buffer clear control register"]
pub mod l1_cache_autoload_buf_clr_ctrl;
#[doc = "L1_UNALLOCATE_BUFFER_CLEAR (rw) register accessor: an alias for `Reg<L1_UNALLOCATE_BUFFER_CLEAR_SPEC>`"]
pub type L1_UNALLOCATE_BUFFER_CLEAR =
    crate::Reg<l1_unallocate_buffer_clear::L1_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Unallocate request buffer clear registers"]
pub mod l1_unallocate_buffer_clear;
#[doc = "L1_CACHE_OBJECT_CTRL (rw) register accessor: an alias for `Reg<L1_CACHE_OBJECT_CTRL_SPEC>`"]
pub type L1_CACHE_OBJECT_CTRL = crate::Reg<l1_cache_object_ctrl::L1_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Cache Tag and Data memory Object control register"]
pub mod l1_cache_object_ctrl;
#[doc = "L1_CACHE_WAY_OBJECT (rw) register accessor: an alias for `Reg<L1_CACHE_WAY_OBJECT_SPEC>`"]
pub type L1_CACHE_WAY_OBJECT = crate::Reg<l1_cache_way_object::L1_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Cache Tag and Data memory way register"]
pub mod l1_cache_way_object;
#[doc = "L1_CACHE_VADDR (rw) register accessor: an alias for `Reg<L1_CACHE_VADDR_SPEC>`"]
pub type L1_CACHE_VADDR = crate::Reg<l1_cache_vaddr::L1_CACHE_VADDR_SPEC>;
#[doc = "Cache Vaddr register"]
pub mod l1_cache_vaddr;
#[doc = "L1_CACHE_DEBUG_BUS (rw) register accessor: an alias for `Reg<L1_CACHE_DEBUG_BUS_SPEC>`"]
pub type L1_CACHE_DEBUG_BUS = crate::Reg<l1_cache_debug_bus::L1_CACHE_DEBUG_BUS_SPEC>;
#[doc = "Cache Tag/data memory content register"]
pub mod l1_cache_debug_bus;
#[doc = "LEVEL_SPLIT0 (r) register accessor: an alias for `Reg<LEVEL_SPLIT0_SPEC>`"]
pub type LEVEL_SPLIT0 = crate::Reg<level_split0::LEVEL_SPLIT0_SPEC>;
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE"]
pub mod level_split0;
#[doc = "L2_CACHE_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_CTRL_SPEC>`"]
pub type L2_CACHE_CTRL = crate::Reg<l2_cache_ctrl::L2_CACHE_CTRL_SPEC>;
#[doc = "L2 Cache(L2-Cache) control register"]
pub mod l2_cache_ctrl;
#[doc = "L2_BYPASS_CACHE_CONF (r) register accessor: an alias for `Reg<L2_BYPASS_CACHE_CONF_SPEC>`"]
pub type L2_BYPASS_CACHE_CONF = crate::Reg<l2_bypass_cache_conf::L2_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Bypass Cache configure register"]
pub mod l2_bypass_cache_conf;
#[doc = "L2_CACHE_CACHESIZE_CONF (r) register accessor: an alias for `Reg<L2_CACHE_CACHESIZE_CONF_SPEC>`"]
pub type L2_CACHE_CACHESIZE_CONF =
    crate::Reg<l2_cache_cachesize_conf::L2_CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "L2 Cache CacheSize mode configure register"]
pub mod l2_cache_cachesize_conf;
#[doc = "L2_CACHE_BLOCKSIZE_CONF (r) register accessor: an alias for `Reg<L2_CACHE_BLOCKSIZE_CONF_SPEC>`"]
pub type L2_CACHE_BLOCKSIZE_CONF =
    crate::Reg<l2_cache_blocksize_conf::L2_CACHE_BLOCKSIZE_CONF_SPEC>;
#[doc = "L2 Cache BlockSize mode configure register"]
pub mod l2_cache_blocksize_conf;
#[doc = "L2_CACHE_WRAP_AROUND_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_WRAP_AROUND_CTRL_SPEC>`"]
pub type L2_CACHE_WRAP_AROUND_CTRL =
    crate::Reg<l2_cache_wrap_around_ctrl::L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Cache wrap around control register"]
pub mod l2_cache_wrap_around_ctrl;
#[doc = "L2_CACHE_TAG_MEM_POWER_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>`"]
pub type L2_CACHE_TAG_MEM_POWER_CTRL =
    crate::Reg<l2_cache_tag_mem_power_ctrl::L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>;
#[doc = "Cache tag memory power control register"]
pub mod l2_cache_tag_mem_power_ctrl;
#[doc = "L2_CACHE_DATA_MEM_POWER_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC>`"]
pub type L2_CACHE_DATA_MEM_POWER_CTRL =
    crate::Reg<l2_cache_data_mem_power_ctrl::L2_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
#[doc = "Cache data memory power control register"]
pub mod l2_cache_data_mem_power_ctrl;
#[doc = "L2_CACHE_FREEZE_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_FREEZE_CTRL_SPEC>`"]
pub type L2_CACHE_FREEZE_CTRL = crate::Reg<l2_cache_freeze_ctrl::L2_CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Cache Freeze control register"]
pub mod l2_cache_freeze_ctrl;
#[doc = "L2_CACHE_DATA_MEM_ACS_CONF (r) register accessor: an alias for `Reg<L2_CACHE_DATA_MEM_ACS_CONF_SPEC>`"]
pub type L2_CACHE_DATA_MEM_ACS_CONF =
    crate::Reg<l2_cache_data_mem_acs_conf::L2_CACHE_DATA_MEM_ACS_CONF_SPEC>;
#[doc = "Cache data memory access configure register"]
pub mod l2_cache_data_mem_acs_conf;
#[doc = "L2_CACHE_TAG_MEM_ACS_CONF (r) register accessor: an alias for `Reg<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>`"]
pub type L2_CACHE_TAG_MEM_ACS_CONF =
    crate::Reg<l2_cache_tag_mem_acs_conf::L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Cache tag memory access configure register"]
pub mod l2_cache_tag_mem_acs_conf;
#[doc = "L2_CACHE_PRELOCK_CONF (r) register accessor: an alias for `Reg<L2_CACHE_PRELOCK_CONF_SPEC>`"]
pub type L2_CACHE_PRELOCK_CONF = crate::Reg<l2_cache_prelock_conf::L2_CACHE_PRELOCK_CONF_SPEC>;
#[doc = "L2 Cache prelock configure register"]
pub mod l2_cache_prelock_conf;
#[doc = "L2_CACHE_PRELOCK_SCT0_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type L2_CACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<l2_cache_prelock_sct0_addr::L2_CACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "L2 Cache prelock section0 address configure register"]
pub mod l2_cache_prelock_sct0_addr;
#[doc = "L2_CACHE_PRELOCK_SCT1_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type L2_CACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<l2_cache_prelock_sct1_addr::L2_CACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "L2 Cache prelock section1 address configure register"]
pub mod l2_cache_prelock_sct1_addr;
#[doc = "L2_CACHE_PRELOCK_SCT_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_PRELOCK_SCT_SIZE_SPEC>`"]
pub type L2_CACHE_PRELOCK_SCT_SIZE =
    crate::Reg<l2_cache_prelock_sct_size::L2_CACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "L2 Cache prelock section size configure register"]
pub mod l2_cache_prelock_sct_size;
#[doc = "L2_CACHE_PRELOAD_CTRL (rw) register accessor: an alias for `Reg<L2_CACHE_PRELOAD_CTRL_SPEC>`"]
pub type L2_CACHE_PRELOAD_CTRL = crate::Reg<l2_cache_preload_ctrl::L2_CACHE_PRELOAD_CTRL_SPEC>;
#[doc = "L2 Cache preload-operation control register"]
pub mod l2_cache_preload_ctrl;
#[doc = "L2_CACHE_PRELOAD_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_PRELOAD_ADDR_SPEC>`"]
pub type L2_CACHE_PRELOAD_ADDR = crate::Reg<l2_cache_preload_addr::L2_CACHE_PRELOAD_ADDR_SPEC>;
#[doc = "L2 Cache preload address configure register"]
pub mod l2_cache_preload_addr;
#[doc = "L2_CACHE_PRELOAD_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_PRELOAD_SIZE_SPEC>`"]
pub type L2_CACHE_PRELOAD_SIZE = crate::Reg<l2_cache_preload_size::L2_CACHE_PRELOAD_SIZE_SPEC>;
#[doc = "L2 Cache preload size configure register"]
pub mod l2_cache_preload_size;
#[doc = "L2_CACHE_AUTOLOAD_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_CTRL_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_CTRL = crate::Reg<l2_cache_autoload_ctrl::L2_CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "L2 Cache autoload-operation control register"]
pub mod l2_cache_autoload_ctrl;
#[doc = "L2_CACHE_AUTOLOAD_SCT0_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<l2_cache_autoload_sct0_addr::L2_CACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 0 address configure register"]
pub mod l2_cache_autoload_sct0_addr;
#[doc = "L2_CACHE_AUTOLOAD_SCT0_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<l2_cache_autoload_sct0_size::L2_CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 0 size configure register"]
pub mod l2_cache_autoload_sct0_size;
#[doc = "L2_CACHE_AUTOLOAD_SCT1_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<l2_cache_autoload_sct1_addr::L2_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 1 address configure register"]
pub mod l2_cache_autoload_sct1_addr;
#[doc = "L2_CACHE_AUTOLOAD_SCT1_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<l2_cache_autoload_sct1_size::L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 1 size configure register"]
pub mod l2_cache_autoload_sct1_size;
#[doc = "L2_CACHE_AUTOLOAD_SCT2_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT2_ADDR_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT2_ADDR =
    crate::Reg<l2_cache_autoload_sct2_addr::L2_CACHE_AUTOLOAD_SCT2_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 2 address configure register"]
pub mod l2_cache_autoload_sct2_addr;
#[doc = "L2_CACHE_AUTOLOAD_SCT2_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT2_SIZE =
    crate::Reg<l2_cache_autoload_sct2_size::L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 2 size configure register"]
pub mod l2_cache_autoload_sct2_size;
#[doc = "L2_CACHE_AUTOLOAD_SCT3_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT3_ADDR =
    crate::Reg<l2_cache_autoload_sct3_addr::L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "L2 Cache autoload section 3 address configure register"]
pub mod l2_cache_autoload_sct3_addr;
#[doc = "L2_CACHE_AUTOLOAD_SCT3_SIZE (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_SCT3_SIZE =
    crate::Reg<l2_cache_autoload_sct3_size::L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "L2 Cache autoload section 3 size configure register"]
pub mod l2_cache_autoload_sct3_size;
#[doc = "L2_CACHE_ACS_CNT_INT_ENA (r) register accessor: an alias for `Reg<L2_CACHE_ACS_CNT_INT_ENA_SPEC>`"]
pub type L2_CACHE_ACS_CNT_INT_ENA =
    crate::Reg<l2_cache_acs_cnt_int_ena::L2_CACHE_ACS_CNT_INT_ENA_SPEC>;
#[doc = "Cache Access Counter Interrupt enable register"]
pub mod l2_cache_acs_cnt_int_ena;
#[doc = "L2_CACHE_ACS_CNT_INT_CLR (r) register accessor: an alias for `Reg<L2_CACHE_ACS_CNT_INT_CLR_SPEC>`"]
pub type L2_CACHE_ACS_CNT_INT_CLR =
    crate::Reg<l2_cache_acs_cnt_int_clr::L2_CACHE_ACS_CNT_INT_CLR_SPEC>;
#[doc = "Cache Access Counter Interrupt clear register"]
pub mod l2_cache_acs_cnt_int_clr;
#[doc = "L2_CACHE_ACS_CNT_INT_RAW (rw) register accessor: an alias for `Reg<L2_CACHE_ACS_CNT_INT_RAW_SPEC>`"]
pub type L2_CACHE_ACS_CNT_INT_RAW =
    crate::Reg<l2_cache_acs_cnt_int_raw::L2_CACHE_ACS_CNT_INT_RAW_SPEC>;
#[doc = "Cache Access Counter Interrupt raw register"]
pub mod l2_cache_acs_cnt_int_raw;
#[doc = "L2_CACHE_ACS_CNT_INT_ST (r) register accessor: an alias for `Reg<L2_CACHE_ACS_CNT_INT_ST_SPEC>`"]
pub type L2_CACHE_ACS_CNT_INT_ST =
    crate::Reg<l2_cache_acs_cnt_int_st::L2_CACHE_ACS_CNT_INT_ST_SPEC>;
#[doc = "Cache Access Counter Interrupt status register"]
pub mod l2_cache_acs_cnt_int_st;
#[doc = "L2_CACHE_ACS_FAIL_INT_ENA (r) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_INT_ENA_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_INT_ENA =
    crate::Reg<l2_cache_acs_fail_int_ena::L2_CACHE_ACS_FAIL_INT_ENA_SPEC>;
#[doc = "Cache Access Fail Interrupt enable register"]
pub mod l2_cache_acs_fail_int_ena;
#[doc = "L2_CACHE_ACS_FAIL_INT_CLR (r) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_INT_CLR_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_INT_CLR =
    crate::Reg<l2_cache_acs_fail_int_clr::L2_CACHE_ACS_FAIL_INT_CLR_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt clear register"]
pub mod l2_cache_acs_fail_int_clr;
#[doc = "L2_CACHE_ACS_FAIL_INT_RAW (rw) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_INT_RAW =
    crate::Reg<l2_cache_acs_fail_int_raw::L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
#[doc = "Cache Access Fail Interrupt raw register"]
pub mod l2_cache_acs_fail_int_raw;
#[doc = "L2_CACHE_ACS_FAIL_INT_ST (r) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_INT_ST_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_INT_ST =
    crate::Reg<l2_cache_acs_fail_int_st::L2_CACHE_ACS_FAIL_INT_ST_SPEC>;
#[doc = "Cache Access Fail Interrupt status register"]
pub mod l2_cache_acs_fail_int_st;
#[doc = "L2_CACHE_ACS_CNT_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_ACS_CNT_CTRL_SPEC>`"]
pub type L2_CACHE_ACS_CNT_CTRL = crate::Reg<l2_cache_acs_cnt_ctrl::L2_CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Cache Access Counter enable and clear register"]
pub mod l2_cache_acs_cnt_ctrl;
#[doc = "L2_IBUS0_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_IBUS0_ACS_HIT_CNT_SPEC>`"]
pub type L2_IBUS0_ACS_HIT_CNT = crate::Reg<l2_ibus0_acs_hit_cnt::L2_IBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Hit-Access Counter register"]
pub mod l2_ibus0_acs_hit_cnt;
#[doc = "L2_IBUS0_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_IBUS0_ACS_MISS_CNT_SPEC>`"]
pub type L2_IBUS0_ACS_MISS_CNT = crate::Reg<l2_ibus0_acs_miss_cnt::L2_IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus0 Miss-Access Counter register"]
pub mod l2_ibus0_acs_miss_cnt;
#[doc = "L2_IBUS0_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_IBUS0_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_IBUS0_ACS_CONFLICT_CNT =
    crate::Reg<l2_ibus0_acs_conflict_cnt::L2_IBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Conflict-Access Counter register"]
pub mod l2_ibus0_acs_conflict_cnt;
#[doc = "L2_IBUS0_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_IBUS0_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_IBUS0_ACS_NXTLVL_CNT =
    crate::Reg<l2_ibus0_acs_nxtlvl_cnt::L2_IBUS0_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus0 Next-Level-Access Counter register"]
pub mod l2_ibus0_acs_nxtlvl_cnt;
#[doc = "L2_IBUS1_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_IBUS1_ACS_HIT_CNT_SPEC>`"]
pub type L2_IBUS1_ACS_HIT_CNT = crate::Reg<l2_ibus1_acs_hit_cnt::L2_IBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Hit-Access Counter register"]
pub mod l2_ibus1_acs_hit_cnt;
#[doc = "L2_IBUS1_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_IBUS1_ACS_MISS_CNT_SPEC>`"]
pub type L2_IBUS1_ACS_MISS_CNT = crate::Reg<l2_ibus1_acs_miss_cnt::L2_IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus1 Miss-Access Counter register"]
pub mod l2_ibus1_acs_miss_cnt;
#[doc = "L2_IBUS1_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_IBUS1_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_IBUS1_ACS_CONFLICT_CNT =
    crate::Reg<l2_ibus1_acs_conflict_cnt::L2_IBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Conflict-Access Counter register"]
pub mod l2_ibus1_acs_conflict_cnt;
#[doc = "L2_IBUS1_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_IBUS1_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_IBUS1_ACS_NXTLVL_CNT =
    crate::Reg<l2_ibus1_acs_nxtlvl_cnt::L2_IBUS1_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus1 Next-Level-Access Counter register"]
pub mod l2_ibus1_acs_nxtlvl_cnt;
#[doc = "L2_IBUS2_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_IBUS2_ACS_HIT_CNT_SPEC>`"]
pub type L2_IBUS2_ACS_HIT_CNT = crate::Reg<l2_ibus2_acs_hit_cnt::L2_IBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Hit-Access Counter register"]
pub mod l2_ibus2_acs_hit_cnt;
#[doc = "L2_IBUS2_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_IBUS2_ACS_MISS_CNT_SPEC>`"]
pub type L2_IBUS2_ACS_MISS_CNT = crate::Reg<l2_ibus2_acs_miss_cnt::L2_IBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus2 Miss-Access Counter register"]
pub mod l2_ibus2_acs_miss_cnt;
#[doc = "L2_IBUS2_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_IBUS2_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_IBUS2_ACS_CONFLICT_CNT =
    crate::Reg<l2_ibus2_acs_conflict_cnt::L2_IBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Conflict-Access Counter register"]
pub mod l2_ibus2_acs_conflict_cnt;
#[doc = "L2_IBUS2_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_IBUS2_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_IBUS2_ACS_NXTLVL_CNT =
    crate::Reg<l2_ibus2_acs_nxtlvl_cnt::L2_IBUS2_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus2 Next-Level-Access Counter register"]
pub mod l2_ibus2_acs_nxtlvl_cnt;
#[doc = "L2_IBUS3_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_IBUS3_ACS_HIT_CNT_SPEC>`"]
pub type L2_IBUS3_ACS_HIT_CNT = crate::Reg<l2_ibus3_acs_hit_cnt::L2_IBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Hit-Access Counter register"]
pub mod l2_ibus3_acs_hit_cnt;
#[doc = "L2_IBUS3_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_IBUS3_ACS_MISS_CNT_SPEC>`"]
pub type L2_IBUS3_ACS_MISS_CNT = crate::Reg<l2_ibus3_acs_miss_cnt::L2_IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus3 Miss-Access Counter register"]
pub mod l2_ibus3_acs_miss_cnt;
#[doc = "L2_IBUS3_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_IBUS3_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_IBUS3_ACS_CONFLICT_CNT =
    crate::Reg<l2_ibus3_acs_conflict_cnt::L2_IBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Conflict-Access Counter register"]
pub mod l2_ibus3_acs_conflict_cnt;
#[doc = "L2_IBUS3_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_IBUS3_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_IBUS3_ACS_NXTLVL_CNT =
    crate::Reg<l2_ibus3_acs_nxtlvl_cnt::L2_IBUS3_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus3 Next-Level-Access Counter register"]
pub mod l2_ibus3_acs_nxtlvl_cnt;
#[doc = "L2_DBUS0_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_DBUS0_ACS_HIT_CNT_SPEC>`"]
pub type L2_DBUS0_ACS_HIT_CNT = crate::Reg<l2_dbus0_acs_hit_cnt::L2_DBUS0_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Hit-Access Counter register"]
pub mod l2_dbus0_acs_hit_cnt;
#[doc = "L2_DBUS0_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_DBUS0_ACS_MISS_CNT_SPEC>`"]
pub type L2_DBUS0_ACS_MISS_CNT = crate::Reg<l2_dbus0_acs_miss_cnt::L2_DBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus0 Miss-Access Counter register"]
pub mod l2_dbus0_acs_miss_cnt;
#[doc = "L2_DBUS0_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_DBUS0_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_DBUS0_ACS_CONFLICT_CNT =
    crate::Reg<l2_dbus0_acs_conflict_cnt::L2_DBUS0_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus0 Conflict-Access Counter register"]
pub mod l2_dbus0_acs_conflict_cnt;
#[doc = "L2_DBUS0_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_DBUS0_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_DBUS0_ACS_NXTLVL_CNT =
    crate::Reg<l2_dbus0_acs_nxtlvl_cnt::L2_DBUS0_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus0 Next-Level-Access Counter register"]
pub mod l2_dbus0_acs_nxtlvl_cnt;
#[doc = "L2_DBUS1_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_DBUS1_ACS_HIT_CNT_SPEC>`"]
pub type L2_DBUS1_ACS_HIT_CNT = crate::Reg<l2_dbus1_acs_hit_cnt::L2_DBUS1_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Hit-Access Counter register"]
pub mod l2_dbus1_acs_hit_cnt;
#[doc = "L2_DBUS1_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_DBUS1_ACS_MISS_CNT_SPEC>`"]
pub type L2_DBUS1_ACS_MISS_CNT = crate::Reg<l2_dbus1_acs_miss_cnt::L2_DBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus1 Miss-Access Counter register"]
pub mod l2_dbus1_acs_miss_cnt;
#[doc = "L2_DBUS1_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_DBUS1_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_DBUS1_ACS_CONFLICT_CNT =
    crate::Reg<l2_dbus1_acs_conflict_cnt::L2_DBUS1_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus1 Conflict-Access Counter register"]
pub mod l2_dbus1_acs_conflict_cnt;
#[doc = "L2_DBUS1_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_DBUS1_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_DBUS1_ACS_NXTLVL_CNT =
    crate::Reg<l2_dbus1_acs_nxtlvl_cnt::L2_DBUS1_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus1 Next-Level-Access Counter register"]
pub mod l2_dbus1_acs_nxtlvl_cnt;
#[doc = "L2_DBUS2_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_DBUS2_ACS_HIT_CNT_SPEC>`"]
pub type L2_DBUS2_ACS_HIT_CNT = crate::Reg<l2_dbus2_acs_hit_cnt::L2_DBUS2_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Hit-Access Counter register"]
pub mod l2_dbus2_acs_hit_cnt;
#[doc = "L2_DBUS2_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_DBUS2_ACS_MISS_CNT_SPEC>`"]
pub type L2_DBUS2_ACS_MISS_CNT = crate::Reg<l2_dbus2_acs_miss_cnt::L2_DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus2 Miss-Access Counter register"]
pub mod l2_dbus2_acs_miss_cnt;
#[doc = "L2_DBUS2_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_DBUS2_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_DBUS2_ACS_CONFLICT_CNT =
    crate::Reg<l2_dbus2_acs_conflict_cnt::L2_DBUS2_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus2 Conflict-Access Counter register"]
pub mod l2_dbus2_acs_conflict_cnt;
#[doc = "L2_DBUS2_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_DBUS2_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_DBUS2_ACS_NXTLVL_CNT =
    crate::Reg<l2_dbus2_acs_nxtlvl_cnt::L2_DBUS2_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus2 Next-Level-Access Counter register"]
pub mod l2_dbus2_acs_nxtlvl_cnt;
#[doc = "L2_DBUS3_ACS_HIT_CNT (r) register accessor: an alias for `Reg<L2_DBUS3_ACS_HIT_CNT_SPEC>`"]
pub type L2_DBUS3_ACS_HIT_CNT = crate::Reg<l2_dbus3_acs_hit_cnt::L2_DBUS3_ACS_HIT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Hit-Access Counter register"]
pub mod l2_dbus3_acs_hit_cnt;
#[doc = "L2_DBUS3_ACS_MISS_CNT (r) register accessor: an alias for `Reg<L2_DBUS3_ACS_MISS_CNT_SPEC>`"]
pub type L2_DBUS3_ACS_MISS_CNT = crate::Reg<l2_dbus3_acs_miss_cnt::L2_DBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "L2-Cache bus3 Miss-Access Counter register"]
pub mod l2_dbus3_acs_miss_cnt;
#[doc = "L2_DBUS3_ACS_CONFLICT_CNT (r) register accessor: an alias for `Reg<L2_DBUS3_ACS_CONFLICT_CNT_SPEC>`"]
pub type L2_DBUS3_ACS_CONFLICT_CNT =
    crate::Reg<l2_dbus3_acs_conflict_cnt::L2_DBUS3_ACS_CONFLICT_CNT_SPEC>;
#[doc = "L2-Cache bus3 Conflict-Access Counter register"]
pub mod l2_dbus3_acs_conflict_cnt;
#[doc = "L2_DBUS3_ACS_NXTLVL_CNT (r) register accessor: an alias for `Reg<L2_DBUS3_ACS_NXTLVL_CNT_SPEC>`"]
pub type L2_DBUS3_ACS_NXTLVL_CNT =
    crate::Reg<l2_dbus3_acs_nxtlvl_cnt::L2_DBUS3_ACS_NXTLVL_CNT_SPEC>;
#[doc = "L2-Cache bus3 Next-Level-Access Counter register"]
pub mod l2_dbus3_acs_nxtlvl_cnt;
#[doc = "L2_CACHE_ACS_FAIL_ID_ATTR (r) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_ID_ATTR =
    crate::Reg<l2_cache_acs_fail_id_attr::L2_CACHE_ACS_FAIL_ID_ATTR_SPEC>;
#[doc = "L2-Cache Access Fail ID/attribution information register"]
pub mod l2_cache_acs_fail_id_attr;
#[doc = "L2_CACHE_ACS_FAIL_ADDR (r) register accessor: an alias for `Reg<L2_CACHE_ACS_FAIL_ADDR_SPEC>`"]
pub type L2_CACHE_ACS_FAIL_ADDR = crate::Reg<l2_cache_acs_fail_addr::L2_CACHE_ACS_FAIL_ADDR_SPEC>;
#[doc = "L2-Cache Access Fail Address information register"]
pub mod l2_cache_acs_fail_addr;
#[doc = "L2_CACHE_SYNC_PRELOAD_INT_ENA (r) register accessor: an alias for `Reg<L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>`"]
pub type L2_CACHE_SYNC_PRELOAD_INT_ENA =
    crate::Reg<l2_cache_sync_preload_int_ena::L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt enable register"]
pub mod l2_cache_sync_preload_int_ena;
#[doc = "L2_CACHE_SYNC_PRELOAD_INT_CLR (r) register accessor: an alias for `Reg<L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>`"]
pub type L2_CACHE_SYNC_PRELOAD_INT_CLR =
    crate::Reg<l2_cache_sync_preload_int_clr::L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC>;
#[doc = "Sync Preload operation Interrupt clear register"]
pub mod l2_cache_sync_preload_int_clr;
#[doc = "L2_CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: an alias for `Reg<L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>`"]
pub type L2_CACHE_SYNC_PRELOAD_INT_RAW =
    crate::Reg<l2_cache_sync_preload_int_raw::L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC>;
#[doc = "Sync Preload operation Interrupt raw register"]
pub mod l2_cache_sync_preload_int_raw;
#[doc = "L2_CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: an alias for `Reg<L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>`"]
pub type L2_CACHE_SYNC_PRELOAD_INT_ST =
    crate::Reg<l2_cache_sync_preload_int_st::L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC>;
#[doc = "L1-Cache Access Fail Interrupt status register"]
pub mod l2_cache_sync_preload_int_st;
#[doc = "L2_CACHE_SYNC_PRELOAD_EXCEPTION (r) register accessor: an alias for `Reg<L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>`"]
pub type L2_CACHE_SYNC_PRELOAD_EXCEPTION =
    crate::Reg<l2_cache_sync_preload_exception::L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC>;
#[doc = "Cache Sync/Preload Operation exception register"]
pub mod l2_cache_sync_preload_exception;
#[doc = "L2_CACHE_SYNC_RST_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_SYNC_RST_CTRL_SPEC>`"]
pub type L2_CACHE_SYNC_RST_CTRL = crate::Reg<l2_cache_sync_rst_ctrl::L2_CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Cache Sync Reset control register"]
pub mod l2_cache_sync_rst_ctrl;
#[doc = "L2_CACHE_PRELOAD_RST_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_PRELOAD_RST_CTRL_SPEC>`"]
pub type L2_CACHE_PRELOAD_RST_CTRL =
    crate::Reg<l2_cache_preload_rst_ctrl::L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Cache Preload Reset control register"]
pub mod l2_cache_preload_rst_ctrl;
#[doc = "L2_CACHE_AUTOLOAD_BUF_CLR_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>`"]
pub type L2_CACHE_AUTOLOAD_BUF_CLR_CTRL =
    crate::Reg<l2_cache_autoload_buf_clr_ctrl::L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Cache Autoload buffer clear control register"]
pub mod l2_cache_autoload_buf_clr_ctrl;
#[doc = "L2_UNALLOCATE_BUFFER_CLEAR (r) register accessor: an alias for `Reg<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>`"]
pub type L2_UNALLOCATE_BUFFER_CLEAR =
    crate::Reg<l2_unallocate_buffer_clear::L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Unallocate request buffer clear registers"]
pub mod l2_unallocate_buffer_clear;
#[doc = "L2_CACHE_ACCESS_ATTR_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>`"]
pub type L2_CACHE_ACCESS_ATTR_CTRL =
    crate::Reg<l2_cache_access_attr_ctrl::L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
#[doc = "L1 Cache access Attribute propagation control register"]
pub mod l2_cache_access_attr_ctrl;
#[doc = "L2_CACHE_OBJECT_CTRL (r) register accessor: an alias for `Reg<L2_CACHE_OBJECT_CTRL_SPEC>`"]
pub type L2_CACHE_OBJECT_CTRL = crate::Reg<l2_cache_object_ctrl::L2_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Cache Tag and Data memory Object control register"]
pub mod l2_cache_object_ctrl;
#[doc = "L2_CACHE_WAY_OBJECT (r) register accessor: an alias for `Reg<L2_CACHE_WAY_OBJECT_SPEC>`"]
pub type L2_CACHE_WAY_OBJECT = crate::Reg<l2_cache_way_object::L2_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Cache Tag and Data memory way register"]
pub mod l2_cache_way_object;
#[doc = "L2_CACHE_VADDR (r) register accessor: an alias for `Reg<L2_CACHE_VADDR_SPEC>`"]
pub type L2_CACHE_VADDR = crate::Reg<l2_cache_vaddr::L2_CACHE_VADDR_SPEC>;
#[doc = "Cache Vaddr register"]
pub mod l2_cache_vaddr;
#[doc = "L2_CACHE_DEBUG_BUS (r) register accessor: an alias for `Reg<L2_CACHE_DEBUG_BUS_SPEC>`"]
pub type L2_CACHE_DEBUG_BUS = crate::Reg<l2_cache_debug_bus::L2_CACHE_DEBUG_BUS_SPEC>;
#[doc = "Cache Tag/data memory content register"]
pub mod l2_cache_debug_bus;
#[doc = "LEVEL_SPLIT1 (r) register accessor: an alias for `Reg<LEVEL_SPLIT1_SPEC>`"]
pub type LEVEL_SPLIT1 = crate::Reg<level_split1::LEVEL_SPLIT1_SPEC>;
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE"]
pub mod level_split1;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gate control register"]
pub mod clock_gate;
#[doc = "REDUNDANCY_SIG0 (rw) register accessor: an alias for `Reg<REDUNDANCY_SIG0_SPEC>`"]
pub type REDUNDANCY_SIG0 = crate::Reg<redundancy_sig0::REDUNDANCY_SIG0_SPEC>;
#[doc = "Cache redundancy signal 0 register"]
pub mod redundancy_sig0;
#[doc = "REDUNDANCY_SIG1 (rw) register accessor: an alias for `Reg<REDUNDANCY_SIG1_SPEC>`"]
pub type REDUNDANCY_SIG1 = crate::Reg<redundancy_sig1::REDUNDANCY_SIG1_SPEC>;
#[doc = "Cache redundancy signal 1 register"]
pub mod redundancy_sig1;
#[doc = "REDUNDANCY_SIG2 (rw) register accessor: an alias for `Reg<REDUNDANCY_SIG2_SPEC>`"]
pub type REDUNDANCY_SIG2 = crate::Reg<redundancy_sig2::REDUNDANCY_SIG2_SPEC>;
#[doc = "Cache redundancy signal 2 register"]
pub mod redundancy_sig2;
#[doc = "REDUNDANCY_SIG3 (rw) register accessor: an alias for `Reg<REDUNDANCY_SIG3_SPEC>`"]
pub type REDUNDANCY_SIG3 = crate::Reg<redundancy_sig3::REDUNDANCY_SIG3_SPEC>;
#[doc = "Cache redundancy signal 3 register"]
pub mod redundancy_sig3;
#[doc = "REDUNDANCY_SIG4 (r) register accessor: an alias for `Reg<REDUNDANCY_SIG4_SPEC>`"]
pub type REDUNDANCY_SIG4 = crate::Reg<redundancy_sig4::REDUNDANCY_SIG4_SPEC>;
#[doc = "Cache redundancy signal 0 register"]
pub mod redundancy_sig4;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
