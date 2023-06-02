#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - register description"]
    pub mac_intr_map: MAC_INTR_MAP,
    #[doc = "0x04 - register description"]
    pub wifi_mac_nmi_map: WIFI_MAC_NMI_MAP,
    #[doc = "0x08 - register description"]
    pub wifi_pwr_int_map: WIFI_PWR_INT_MAP,
    #[doc = "0x0c - register description"]
    pub wifi_bb_int_map: WIFI_BB_INT_MAP,
    #[doc = "0x10 - register description"]
    pub bt_mac_int_map: BT_MAC_INT_MAP,
    #[doc = "0x14 - register description"]
    pub bt_bb_int_map: BT_BB_INT_MAP,
    #[doc = "0x18 - register description"]
    pub bt_bb_nmi_map: BT_BB_NMI_MAP,
    #[doc = "0x1c - register description"]
    pub lp_timer_int_map: LP_TIMER_INT_MAP,
    #[doc = "0x20 - register description"]
    pub coex_int_map: COEX_INT_MAP,
    #[doc = "0x24 - register description"]
    pub ble_timer_int_map: BLE_TIMER_INT_MAP,
    #[doc = "0x28 - register description"]
    pub ble_sec_int_map: BLE_SEC_INT_MAP,
    #[doc = "0x2c - register description"]
    pub i2c_mst_int_map: I2C_MST_INT_MAP,
    #[doc = "0x30 - register description"]
    pub apb_ctrl_intr_map: APB_CTRL_INTR_MAP,
    #[doc = "0x34 - register description"]
    pub gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x38 - register description"]
    pub gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x3c - register description"]
    pub spi_intr_1_map: SPI_INTR_1_MAP,
    #[doc = "0x40 - register description"]
    pub spi_intr_2_map: SPI_INTR_2_MAP,
    #[doc = "0x44 - register description"]
    pub uart_intr_map: UART_INTR_MAP,
    #[doc = "0x48 - register description"]
    pub uart1_intr_map: UART1_INTR_MAP,
    #[doc = "0x4c - register description"]
    pub ledc_int_map: LEDC_INT_MAP,
    #[doc = "0x50 - register description"]
    pub efuse_int_map: EFUSE_INT_MAP,
    #[doc = "0x54 - register description"]
    pub rtc_core_intr_map: RTC_CORE_INTR_MAP,
    #[doc = "0x58 - register description"]
    pub i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    #[doc = "0x5c - register description"]
    pub tg_t0_int_map: TG_T0_INT_MAP,
    #[doc = "0x60 - register description"]
    pub tg_wdt_int_map: TG_WDT_INT_MAP,
    #[doc = "0x64 - register description"]
    pub cache_ia_int_map: CACHE_IA_INT_MAP,
    #[doc = "0x68 - register description"]
    pub systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0x6c - register description"]
    pub systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0x70 - register description"]
    pub systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0x74 - register description"]
    pub spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0x78 - register description"]
    pub icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    #[doc = "0x7c - register description"]
    pub icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    #[doc = "0x80 - register description"]
    pub apb_adc_int_map: APB_ADC_INT_MAP,
    #[doc = "0x84 - register description"]
    pub dma_ch0_int_map: DMA_CH0_INT_MAP,
    #[doc = "0x88 - register description"]
    pub sha_int_map: SHA_INT_MAP,
    #[doc = "0x8c - register description"]
    pub ecc_int_map: ECC_INT_MAP,
    #[doc = "0x90 - register description"]
    pub cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x94 - register description"]
    pub cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x98 - register description"]
    pub cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x9c - register description"]
    pub cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0xa0 - register description"]
    pub assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    #[doc = "0xa4 - register description"]
    pub core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0xa8 - register description"]
    pub cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    #[doc = "0xac - register description"]
    pub intr_status_reg_0: INTR_STATUS_REG_0,
    #[doc = "0xb0 - register description"]
    pub intr_status_reg_1: INTR_STATUS_REG_1,
    #[doc = "0xb4 - register description"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0xb8 - register description"]
    pub cpu_int_enable: CPU_INT_ENABLE,
    #[doc = "0xbc - register description"]
    pub cpu_int_type: CPU_INT_TYPE,
    #[doc = "0xc0 - register description"]
    pub cpu_int_clear: CPU_INT_CLEAR,
    #[doc = "0xc4 - register description"]
    pub cpu_int_eip_status: CPU_INT_EIP_STATUS,
    #[doc = "0xc8 - register description"]
    pub cpu_int_pri_0: CPU_INT_PRI_0,
    #[doc = "0xcc - register description"]
    pub cpu_int_pri_1: CPU_INT_PRI_1,
    #[doc = "0xd0 - register description"]
    pub cpu_int_pri_2: CPU_INT_PRI_2,
    #[doc = "0xd4 - register description"]
    pub cpu_int_pri_3: CPU_INT_PRI_3,
    #[doc = "0xd8 - register description"]
    pub cpu_int_pri_4: CPU_INT_PRI_4,
    #[doc = "0xdc - register description"]
    pub cpu_int_pri_5: CPU_INT_PRI_5,
    #[doc = "0xe0 - register description"]
    pub cpu_int_pri_6: CPU_INT_PRI_6,
    #[doc = "0xe4 - register description"]
    pub cpu_int_pri_7: CPU_INT_PRI_7,
    #[doc = "0xe8 - register description"]
    pub cpu_int_pri_8: CPU_INT_PRI_8,
    #[doc = "0xec - register description"]
    pub cpu_int_pri_9: CPU_INT_PRI_9,
    #[doc = "0xf0 - register description"]
    pub cpu_int_pri_10: CPU_INT_PRI_10,
    #[doc = "0xf4 - register description"]
    pub cpu_int_pri_11: CPU_INT_PRI_11,
    #[doc = "0xf8 - register description"]
    pub cpu_int_pri_12: CPU_INT_PRI_12,
    #[doc = "0xfc - register description"]
    pub cpu_int_pri_13: CPU_INT_PRI_13,
    #[doc = "0x100 - register description"]
    pub cpu_int_pri_14: CPU_INT_PRI_14,
    #[doc = "0x104 - register description"]
    pub cpu_int_pri_15: CPU_INT_PRI_15,
    #[doc = "0x108 - register description"]
    pub cpu_int_pri_16: CPU_INT_PRI_16,
    #[doc = "0x10c - register description"]
    pub cpu_int_pri_17: CPU_INT_PRI_17,
    #[doc = "0x110 - register description"]
    pub cpu_int_pri_18: CPU_INT_PRI_18,
    #[doc = "0x114 - register description"]
    pub cpu_int_pri_19: CPU_INT_PRI_19,
    #[doc = "0x118 - register description"]
    pub cpu_int_pri_20: CPU_INT_PRI_20,
    #[doc = "0x11c - register description"]
    pub cpu_int_pri_21: CPU_INT_PRI_21,
    #[doc = "0x120 - register description"]
    pub cpu_int_pri_22: CPU_INT_PRI_22,
    #[doc = "0x124 - register description"]
    pub cpu_int_pri_23: CPU_INT_PRI_23,
    #[doc = "0x128 - register description"]
    pub cpu_int_pri_24: CPU_INT_PRI_24,
    #[doc = "0x12c - register description"]
    pub cpu_int_pri_25: CPU_INT_PRI_25,
    #[doc = "0x130 - register description"]
    pub cpu_int_pri_26: CPU_INT_PRI_26,
    #[doc = "0x134 - register description"]
    pub cpu_int_pri_27: CPU_INT_PRI_27,
    #[doc = "0x138 - register description"]
    pub cpu_int_pri_28: CPU_INT_PRI_28,
    #[doc = "0x13c - register description"]
    pub cpu_int_pri_29: CPU_INT_PRI_29,
    #[doc = "0x140 - register description"]
    pub cpu_int_pri_30: CPU_INT_PRI_30,
    #[doc = "0x144 - register description"]
    pub cpu_int_pri_31: CPU_INT_PRI_31,
    #[doc = "0x148 - register description"]
    pub cpu_int_thresh: CPU_INT_THRESH,
    _reserved83: [u8; 0x06b0],
    #[doc = "0x7fc - register description"]
    pub interrupt_reg_date: INTERRUPT_REG_DATE,
}
#[doc = "MAC_INTR_MAP (rw) register accessor: an alias for `Reg<MAC_INTR_MAP_SPEC>`"]
pub type MAC_INTR_MAP = crate::Reg<mac_intr_map::MAC_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod mac_intr_map;
#[doc = "WIFI_MAC_NMI_MAP (rw) register accessor: an alias for `Reg<WIFI_MAC_NMI_MAP_SPEC>`"]
pub type WIFI_MAC_NMI_MAP = crate::Reg<wifi_mac_nmi_map::WIFI_MAC_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_mac_nmi_map;
#[doc = "WIFI_PWR_INT_MAP (rw) register accessor: an alias for `Reg<WIFI_PWR_INT_MAP_SPEC>`"]
pub type WIFI_PWR_INT_MAP = crate::Reg<wifi_pwr_int_map::WIFI_PWR_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_pwr_int_map;
#[doc = "WIFI_BB_INT_MAP (rw) register accessor: an alias for `Reg<WIFI_BB_INT_MAP_SPEC>`"]
pub type WIFI_BB_INT_MAP = crate::Reg<wifi_bb_int_map::WIFI_BB_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod wifi_bb_int_map;
#[doc = "BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<BT_MAC_INT_MAP_SPEC>`"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<BT_BB_INT_MAP_SPEC>`"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<BT_BB_NMI_MAP_SPEC>`"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod bt_bb_nmi_map;
#[doc = "LP_TIMER_INT_MAP (rw) register accessor: an alias for `Reg<LP_TIMER_INT_MAP_SPEC>`"]
pub type LP_TIMER_INT_MAP = crate::Reg<lp_timer_int_map::LP_TIMER_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod lp_timer_int_map;
#[doc = "COEX_INT_MAP (rw) register accessor: an alias for `Reg<COEX_INT_MAP_SPEC>`"]
pub type COEX_INT_MAP = crate::Reg<coex_int_map::COEX_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod coex_int_map;
#[doc = "BLE_TIMER_INT_MAP (rw) register accessor: an alias for `Reg<BLE_TIMER_INT_MAP_SPEC>`"]
pub type BLE_TIMER_INT_MAP = crate::Reg<ble_timer_int_map::BLE_TIMER_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_timer_int_map;
#[doc = "BLE_SEC_INT_MAP (rw) register accessor: an alias for `Reg<BLE_SEC_INT_MAP_SPEC>`"]
pub type BLE_SEC_INT_MAP = crate::Reg<ble_sec_int_map::BLE_SEC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod ble_sec_int_map;
#[doc = "I2C_MST_INT_MAP (rw) register accessor: an alias for `Reg<I2C_MST_INT_MAP_SPEC>`"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_mst_int_map;
#[doc = "APB_CTRL_INTR_MAP (rw) register accessor: an alias for `Reg<APB_CTRL_INTR_MAP_SPEC>`"]
pub type APB_CTRL_INTR_MAP = crate::Reg<apb_ctrl_intr_map::APB_CTRL_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod apb_ctrl_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "register description"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_1_MAP_SPEC>`"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "register description"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_2_MAP_SPEC>`"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "register description"]
pub mod spi_intr_2_map;
#[doc = "UART_INTR_MAP (rw) register accessor: an alias for `Reg<UART_INTR_MAP_SPEC>`"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: an alias for `Reg<UART1_INTR_MAP_SPEC>`"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod uart1_intr_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: an alias for `Reg<LEDC_INT_MAP_SPEC>`"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<EFUSE_INT_MAP_SPEC>`"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod efuse_int_map;
#[doc = "RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<RTC_CORE_INTR_MAP_SPEC>`"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod rtc_core_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<I2C_EXT0_INTR_MAP_SPEC>`"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod i2c_ext0_intr_map;
#[doc = "TG_T0_INT_MAP (rw) register accessor: an alias for `Reg<TG_T0_INT_MAP_SPEC>`"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod tg_t0_int_map;
#[doc = "TG_WDT_INT_MAP (rw) register accessor: an alias for `Reg<TG_WDT_INT_MAP_SPEC>`"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod tg_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_IA_INT_MAP_SPEC>`"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP (rw) register accessor: an alias for `Reg<SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod spi_mem_reject_intr_map;
#[doc = "ICACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod icache_preload_int_map;
#[doc = "ICACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP (rw) register accessor: an alias for `Reg<APB_ADC_INT_MAP_SPEC>`"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod apb_adc_int_map;
#[doc = "DMA_CH0_INT_MAP (rw) register accessor: an alias for `Reg<DMA_CH0_INT_MAP_SPEC>`"]
pub type DMA_CH0_INT_MAP = crate::Reg<dma_ch0_int_map::DMA_CH0_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod dma_ch0_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: an alias for `Reg<SHA_INT_MAP_SPEC>`"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod sha_int_map;
#[doc = "ECC_INT_MAP (rw) register accessor: an alias for `Reg<ECC_INT_MAP_SPEC>`"]
pub type ECC_INT_MAP = crate::Reg<ecc_int_map::ECC_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod ecc_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: an alias for `Reg<ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "register description"]
pub mod assist_debug_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "register description"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_CORE0_ACS_INT_MAP_SPEC>`"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "register description"]
pub mod cache_core0_acs_int_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: an alias for `Reg<INTR_STATUS_REG_0_SPEC>`"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: an alias for `Reg<INTR_STATUS_REG_1_SPEC>`"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "register description"]
pub mod intr_status_reg_1;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "CPU_INT_ENABLE (rw) register accessor: an alias for `Reg<CPU_INT_ENABLE_SPEC>`"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: an alias for `Reg<CPU_INT_TYPE_SPEC>`"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_type;
#[doc = "CPU_INT_CLEAR (rw) register accessor: an alias for `Reg<CPU_INT_CLEAR_SPEC>`"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "register description"]
pub mod cpu_int_clear;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: an alias for `Reg<CPU_INT_EIP_STATUS_SPEC>`"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "register description"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_0_SPEC>`"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_1_SPEC>`"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_2_SPEC>`"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_3_SPEC>`"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_4_SPEC>`"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_5_SPEC>`"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_6_SPEC>`"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_7_SPEC>`"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_8_SPEC>`"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_9_SPEC>`"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_10_SPEC>`"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_11_SPEC>`"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_12_SPEC>`"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_13_SPEC>`"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_14_SPEC>`"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_15_SPEC>`"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_16_SPEC>`"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_17_SPEC>`"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_18_SPEC>`"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_19_SPEC>`"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_20_SPEC>`"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_21_SPEC>`"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_22_SPEC>`"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_23_SPEC>`"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_24_SPEC>`"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_25_SPEC>`"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_26_SPEC>`"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_27_SPEC>`"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_28_SPEC>`"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_29_SPEC>`"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_30_SPEC>`"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_31_SPEC>`"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH (rw) register accessor: an alias for `Reg<CPU_INT_THRESH_SPEC>`"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "register description"]
pub mod cpu_int_thresh;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: an alias for `Reg<INTERRUPT_REG_DATE_SPEC>`"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "register description"]
pub mod interrupt_reg_date;
