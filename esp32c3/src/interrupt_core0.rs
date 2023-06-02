#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - mac intr map register"]
    pub mac_intr_map: MAC_INTR_MAP,
    #[doc = "0x04 - mac nmi_intr map register"]
    pub mac_nmi_map: MAC_NMI_MAP,
    #[doc = "0x08 - pwr intr map register"]
    pub pwr_intr_map: PWR_INTR_MAP,
    #[doc = "0x0c - bb intr map register"]
    pub bb_int_map: BB_INT_MAP,
    #[doc = "0x10 - bt intr map register"]
    pub bt_mac_int_map: BT_MAC_INT_MAP,
    #[doc = "0x14 - bb_bt intr map register"]
    pub bt_bb_int_map: BT_BB_INT_MAP,
    #[doc = "0x18 - bb_bt_nmi intr map register"]
    pub bt_bb_nmi_map: BT_BB_NMI_MAP,
    #[doc = "0x1c - rwbt intr map register"]
    pub rwbt_irq_map: RWBT_IRQ_MAP,
    #[doc = "0x20 - rwble intr map register"]
    pub rwble_irq_map: RWBLE_IRQ_MAP,
    #[doc = "0x24 - rwbt_nmi intr map register"]
    pub rwbt_nmi_map: RWBT_NMI_MAP,
    #[doc = "0x28 - rwble_nmi intr map register"]
    pub rwble_nmi_map: RWBLE_NMI_MAP,
    #[doc = "0x2c - i2c intr map register"]
    pub i2c_mst_int_map: I2C_MST_INT_MAP,
    #[doc = "0x30 - slc0 intr map register"]
    pub slc0_intr_map: SLC0_INTR_MAP,
    #[doc = "0x34 - slc1 intr map register"]
    pub slc1_intr_map: SLC1_INTR_MAP,
    #[doc = "0x38 - apb_ctrl intr map register"]
    pub apb_ctrl_intr_map: APB_CTRL_INTR_MAP,
    #[doc = "0x3c - uchi0 intr map register"]
    pub uhci0_intr_map: UHCI0_INTR_MAP,
    #[doc = "0x40 - gpio intr map register"]
    pub gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x44 - gpio_pro intr map register"]
    pub gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x48 - gpio_pro_nmi intr map register"]
    pub spi_intr_1_map: SPI_INTR_1_MAP,
    #[doc = "0x4c - spi1 intr map register"]
    pub spi_intr_2_map: SPI_INTR_2_MAP,
    #[doc = "0x50 - spi2 intr map register"]
    pub i2s1_int_map: I2S1_INT_MAP,
    #[doc = "0x54 - i2s1 intr map register"]
    pub uart_intr_map: UART_INTR_MAP,
    #[doc = "0x58 - uart1 intr map register"]
    pub uart1_intr_map: UART1_INTR_MAP,
    #[doc = "0x5c - ledc intr map register"]
    pub ledc_int_map: LEDC_INT_MAP,
    #[doc = "0x60 - efuse intr map register"]
    pub efuse_int_map: EFUSE_INT_MAP,
    #[doc = "0x64 - can intr map register"]
    pub can_int_map: CAN_INT_MAP,
    #[doc = "0x68 - usb intr map register"]
    pub usb_intr_map: USB_INTR_MAP,
    #[doc = "0x6c - rtc intr map register"]
    pub rtc_core_intr_map: RTC_CORE_INTR_MAP,
    #[doc = "0x70 - rmt intr map register"]
    pub rmt_intr_map: RMT_INTR_MAP,
    #[doc = "0x74 - i2c intr map register"]
    pub i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    #[doc = "0x78 - timer1 intr map register"]
    pub timer_int1_map: TIMER_INT1_MAP,
    #[doc = "0x7c - timer2 intr map register"]
    pub timer_int2_map: TIMER_INT2_MAP,
    #[doc = "0x80 - tg to intr map register"]
    pub tg_t0_int_map: TG_T0_INT_MAP,
    #[doc = "0x84 - tg wdt intr map register"]
    pub tg_wdt_int_map: TG_WDT_INT_MAP,
    #[doc = "0x88 - tg1 to intr map register"]
    pub tg1_t0_int_map: TG1_T0_INT_MAP,
    #[doc = "0x8c - tg1 wdt intr map register"]
    pub tg1_wdt_int_map: TG1_WDT_INT_MAP,
    #[doc = "0x90 - cache ia intr map register"]
    pub cache_ia_int_map: CACHE_IA_INT_MAP,
    #[doc = "0x94 - systimer intr map register"]
    pub systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0x98 - systimer target1 intr map register"]
    pub systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0x9c - systimer target2 intr map register"]
    pub systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0xa0 - spi mem reject intr map register"]
    pub spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0xa4 - icache perload intr map register"]
    pub icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    #[doc = "0xa8 - icache sync intr map register"]
    pub icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    #[doc = "0xac - adc intr map register"]
    pub apb_adc_int_map: APB_ADC_INT_MAP,
    #[doc = "0xb0 - dma ch0 intr map register"]
    pub dma_ch0_int_map: DMA_CH0_INT_MAP,
    #[doc = "0xb4 - dma ch1 intr map register"]
    pub dma_ch1_int_map: DMA_CH1_INT_MAP,
    #[doc = "0xb8 - dma ch2 intr map register"]
    pub dma_ch2_int_map: DMA_CH2_INT_MAP,
    #[doc = "0xbc - rsa intr map register"]
    pub rsa_int_map: RSA_INT_MAP,
    #[doc = "0xc0 - aes intr map register"]
    pub aes_int_map: AES_INT_MAP,
    #[doc = "0xc4 - sha intr map register"]
    pub sha_int_map: SHA_INT_MAP,
    #[doc = "0xc8 - cpu from cpu 0 intr map register"]
    pub cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0xcc - cpu from cpu 0 intr map register"]
    pub cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0xd0 - cpu from cpu 1 intr map register"]
    pub cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0xd4 - cpu from cpu 3 intr map register"]
    pub cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0xd8 - assist debug intr map register"]
    pub assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    #[doc = "0xdc - dma pms violatile intr map register"]
    pub dma_apbperi_pms_monitor_violate_intr_map: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe0 - iram0 pms violatile intr map register"]
    pub core_0_iram0_pms_monitor_violate_intr_map: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe4 - mac intr map register"]
    pub core_0_dram0_pms_monitor_violate_intr_map: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xe8 - mac intr map register"]
    pub core_0_pif_pms_monitor_violate_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0xec - mac intr map register"]
    pub core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0xf0 - mac intr map register"]
    pub backup_pms_violate_intr_map: BACKUP_PMS_VIOLATE_INTR_MAP,
    #[doc = "0xf4 - mac intr map register"]
    pub cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    #[doc = "0xf8 - mac intr map register"]
    pub intr_status_reg_0: INTR_STATUS_REG_0,
    #[doc = "0xfc - mac intr map register"]
    pub intr_status_reg_1: INTR_STATUS_REG_1,
    #[doc = "0x100 - mac intr map register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x104 - mac intr map register"]
    pub cpu_int_enable: CPU_INT_ENABLE,
    #[doc = "0x108 - mac intr map register"]
    pub cpu_int_type: CPU_INT_TYPE,
    #[doc = "0x10c - mac intr map register"]
    pub cpu_int_clear: CPU_INT_CLEAR,
    #[doc = "0x110 - mac intr map register"]
    pub cpu_int_eip_status: CPU_INT_EIP_STATUS,
    #[doc = "0x114 - mac intr map register"]
    pub cpu_int_pri_0: CPU_INT_PRI_0,
    #[doc = "0x118 - mac intr map register"]
    pub cpu_int_pri_1: CPU_INT_PRI_1,
    #[doc = "0x11c - mac intr map register"]
    pub cpu_int_pri_2: CPU_INT_PRI_2,
    #[doc = "0x120 - mac intr map register"]
    pub cpu_int_pri_3: CPU_INT_PRI_3,
    #[doc = "0x124 - mac intr map register"]
    pub cpu_int_pri_4: CPU_INT_PRI_4,
    #[doc = "0x128 - mac intr map register"]
    pub cpu_int_pri_5: CPU_INT_PRI_5,
    #[doc = "0x12c - mac intr map register"]
    pub cpu_int_pri_6: CPU_INT_PRI_6,
    #[doc = "0x130 - mac intr map register"]
    pub cpu_int_pri_7: CPU_INT_PRI_7,
    #[doc = "0x134 - mac intr map register"]
    pub cpu_int_pri_8: CPU_INT_PRI_8,
    #[doc = "0x138 - mac intr map register"]
    pub cpu_int_pri_9: CPU_INT_PRI_9,
    #[doc = "0x13c - mac intr map register"]
    pub cpu_int_pri_10: CPU_INT_PRI_10,
    #[doc = "0x140 - mac intr map register"]
    pub cpu_int_pri_11: CPU_INT_PRI_11,
    #[doc = "0x144 - mac intr map register"]
    pub cpu_int_pri_12: CPU_INT_PRI_12,
    #[doc = "0x148 - mac intr map register"]
    pub cpu_int_pri_13: CPU_INT_PRI_13,
    #[doc = "0x14c - mac intr map register"]
    pub cpu_int_pri_14: CPU_INT_PRI_14,
    #[doc = "0x150 - mac intr map register"]
    pub cpu_int_pri_15: CPU_INT_PRI_15,
    #[doc = "0x154 - mac intr map register"]
    pub cpu_int_pri_16: CPU_INT_PRI_16,
    #[doc = "0x158 - mac intr map register"]
    pub cpu_int_pri_17: CPU_INT_PRI_17,
    #[doc = "0x15c - mac intr map register"]
    pub cpu_int_pri_18: CPU_INT_PRI_18,
    #[doc = "0x160 - mac intr map register"]
    pub cpu_int_pri_19: CPU_INT_PRI_19,
    #[doc = "0x164 - mac intr map register"]
    pub cpu_int_pri_20: CPU_INT_PRI_20,
    #[doc = "0x168 - mac intr map register"]
    pub cpu_int_pri_21: CPU_INT_PRI_21,
    #[doc = "0x16c - mac intr map register"]
    pub cpu_int_pri_22: CPU_INT_PRI_22,
    #[doc = "0x170 - mac intr map register"]
    pub cpu_int_pri_23: CPU_INT_PRI_23,
    #[doc = "0x174 - mac intr map register"]
    pub cpu_int_pri_24: CPU_INT_PRI_24,
    #[doc = "0x178 - mac intr map register"]
    pub cpu_int_pri_25: CPU_INT_PRI_25,
    #[doc = "0x17c - mac intr map register"]
    pub cpu_int_pri_26: CPU_INT_PRI_26,
    #[doc = "0x180 - mac intr map register"]
    pub cpu_int_pri_27: CPU_INT_PRI_27,
    #[doc = "0x184 - mac intr map register"]
    pub cpu_int_pri_28: CPU_INT_PRI_28,
    #[doc = "0x188 - mac intr map register"]
    pub cpu_int_pri_29: CPU_INT_PRI_29,
    #[doc = "0x18c - mac intr map register"]
    pub cpu_int_pri_30: CPU_INT_PRI_30,
    #[doc = "0x190 - mac intr map register"]
    pub cpu_int_pri_31: CPU_INT_PRI_31,
    #[doc = "0x194 - mac intr map register"]
    pub cpu_int_thresh: CPU_INT_THRESH,
    _reserved102: [u8; 0x0664],
    #[doc = "0x7fc - mac intr map register"]
    pub interrupt_reg_date: INTERRUPT_REG_DATE,
}
#[doc = "MAC_INTR_MAP (rw) register accessor: an alias for `Reg<MAC_INTR_MAP_SPEC>`"]
pub type MAC_INTR_MAP = crate::Reg<mac_intr_map::MAC_INTR_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod mac_intr_map;
#[doc = "MAC_NMI_MAP (rw) register accessor: an alias for `Reg<MAC_NMI_MAP_SPEC>`"]
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
#[doc = "mac nmi_intr map register"]
pub mod mac_nmi_map;
#[doc = "PWR_INTR_MAP (rw) register accessor: an alias for `Reg<PWR_INTR_MAP_SPEC>`"]
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
#[doc = "pwr intr map register"]
pub mod pwr_intr_map;
#[doc = "BB_INT_MAP (rw) register accessor: an alias for `Reg<BB_INT_MAP_SPEC>`"]
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
#[doc = "bb intr map register"]
pub mod bb_int_map;
#[doc = "BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<BT_MAC_INT_MAP_SPEC>`"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "bt intr map register"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<BT_BB_INT_MAP_SPEC>`"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "bb_bt intr map register"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<BT_BB_NMI_MAP_SPEC>`"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "bb_bt_nmi intr map register"]
pub mod bt_bb_nmi_map;
#[doc = "RWBT_IRQ_MAP (rw) register accessor: an alias for `Reg<RWBT_IRQ_MAP_SPEC>`"]
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
#[doc = "rwbt intr map register"]
pub mod rwbt_irq_map;
#[doc = "RWBLE_IRQ_MAP (rw) register accessor: an alias for `Reg<RWBLE_IRQ_MAP_SPEC>`"]
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
#[doc = "rwble intr map register"]
pub mod rwble_irq_map;
#[doc = "RWBT_NMI_MAP (rw) register accessor: an alias for `Reg<RWBT_NMI_MAP_SPEC>`"]
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
#[doc = "rwbt_nmi intr map register"]
pub mod rwbt_nmi_map;
#[doc = "RWBLE_NMI_MAP (rw) register accessor: an alias for `Reg<RWBLE_NMI_MAP_SPEC>`"]
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
#[doc = "rwble_nmi intr map register"]
pub mod rwble_nmi_map;
#[doc = "I2C_MST_INT_MAP (rw) register accessor: an alias for `Reg<I2C_MST_INT_MAP_SPEC>`"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "i2c intr map register"]
pub mod i2c_mst_int_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: an alias for `Reg<SLC0_INTR_MAP_SPEC>`"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "slc0 intr map register"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: an alias for `Reg<SLC1_INTR_MAP_SPEC>`"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "slc1 intr map register"]
pub mod slc1_intr_map;
#[doc = "APB_CTRL_INTR_MAP (rw) register accessor: an alias for `Reg<APB_CTRL_INTR_MAP_SPEC>`"]
pub type APB_CTRL_INTR_MAP = crate::Reg<apb_ctrl_intr_map::APB_CTRL_INTR_MAP_SPEC>;
#[doc = "apb_ctrl intr map register"]
pub mod apb_ctrl_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: an alias for `Reg<UHCI0_INTR_MAP_SPEC>`"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "uchi0 intr map register"]
pub mod uhci0_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "gpio intr map register"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "gpio_pro intr map register"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_1_MAP_SPEC>`"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "gpio_pro_nmi intr map register"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_2_MAP_SPEC>`"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "spi1 intr map register"]
pub mod spi_intr_2_map;
#[doc = "I2S1_INT_MAP (rw) register accessor: an alias for `Reg<I2S1_INT_MAP_SPEC>`"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "spi2 intr map register"]
pub mod i2s1_int_map;
#[doc = "UART_INTR_MAP (rw) register accessor: an alias for `Reg<UART_INTR_MAP_SPEC>`"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "i2s1 intr map register"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: an alias for `Reg<UART1_INTR_MAP_SPEC>`"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "uart1 intr map register"]
pub mod uart1_intr_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: an alias for `Reg<LEDC_INT_MAP_SPEC>`"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "ledc intr map register"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<EFUSE_INT_MAP_SPEC>`"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "efuse intr map register"]
pub mod efuse_int_map;
#[doc = "CAN_INT_MAP (rw) register accessor: an alias for `Reg<CAN_INT_MAP_SPEC>`"]
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
#[doc = "can intr map register"]
pub mod can_int_map;
#[doc = "USB_INTR_MAP (rw) register accessor: an alias for `Reg<USB_INTR_MAP_SPEC>`"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "usb intr map register"]
pub mod usb_intr_map;
#[doc = "RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<RTC_CORE_INTR_MAP_SPEC>`"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "rtc intr map register"]
pub mod rtc_core_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: an alias for `Reg<RMT_INTR_MAP_SPEC>`"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "rmt intr map register"]
pub mod rmt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<I2C_EXT0_INTR_MAP_SPEC>`"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "i2c intr map register"]
pub mod i2c_ext0_intr_map;
#[doc = "TIMER_INT1_MAP (rw) register accessor: an alias for `Reg<TIMER_INT1_MAP_SPEC>`"]
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
#[doc = "timer1 intr map register"]
pub mod timer_int1_map;
#[doc = "TIMER_INT2_MAP (rw) register accessor: an alias for `Reg<TIMER_INT2_MAP_SPEC>`"]
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
#[doc = "timer2 intr map register"]
pub mod timer_int2_map;
#[doc = "TG_T0_INT_MAP (rw) register accessor: an alias for `Reg<TG_T0_INT_MAP_SPEC>`"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "tg to intr map register"]
pub mod tg_t0_int_map;
#[doc = "TG_WDT_INT_MAP (rw) register accessor: an alias for `Reg<TG_WDT_INT_MAP_SPEC>`"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "tg wdt intr map register"]
pub mod tg_wdt_int_map;
#[doc = "TG1_T0_INT_MAP (rw) register accessor: an alias for `Reg<TG1_T0_INT_MAP_SPEC>`"]
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
#[doc = "tg1 to intr map register"]
pub mod tg1_t0_int_map;
#[doc = "TG1_WDT_INT_MAP (rw) register accessor: an alias for `Reg<TG1_WDT_INT_MAP_SPEC>`"]
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
#[doc = "tg1 wdt intr map register"]
pub mod tg1_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_IA_INT_MAP_SPEC>`"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "cache ia intr map register"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "systimer intr map register"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "systimer target1 intr map register"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "systimer target2 intr map register"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP (rw) register accessor: an alias for `Reg<SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "spi mem reject intr map register"]
pub mod spi_mem_reject_intr_map;
#[doc = "ICACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "icache perload intr map register"]
pub mod icache_preload_int_map;
#[doc = "ICACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "icache sync intr map register"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP (rw) register accessor: an alias for `Reg<APB_ADC_INT_MAP_SPEC>`"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "adc intr map register"]
pub mod apb_adc_int_map;
#[doc = "DMA_CH0_INT_MAP (rw) register accessor: an alias for `Reg<DMA_CH0_INT_MAP_SPEC>`"]
pub type DMA_CH0_INT_MAP = crate::Reg<dma_ch0_int_map::DMA_CH0_INT_MAP_SPEC>;
#[doc = "dma ch0 intr map register"]
pub mod dma_ch0_int_map;
#[doc = "DMA_CH1_INT_MAP (rw) register accessor: an alias for `Reg<DMA_CH1_INT_MAP_SPEC>`"]
pub type DMA_CH1_INT_MAP = crate::Reg<dma_ch1_int_map::DMA_CH1_INT_MAP_SPEC>;
#[doc = "dma ch1 intr map register"]
pub mod dma_ch1_int_map;
#[doc = "DMA_CH2_INT_MAP (rw) register accessor: an alias for `Reg<DMA_CH2_INT_MAP_SPEC>`"]
pub type DMA_CH2_INT_MAP = crate::Reg<dma_ch2_int_map::DMA_CH2_INT_MAP_SPEC>;
#[doc = "dma ch2 intr map register"]
pub mod dma_ch2_int_map;
#[doc = "RSA_INT_MAP (rw) register accessor: an alias for `Reg<RSA_INT_MAP_SPEC>`"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "rsa intr map register"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP (rw) register accessor: an alias for `Reg<AES_INT_MAP_SPEC>`"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "aes intr map register"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: an alias for `Reg<SHA_INT_MAP_SPEC>`"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "sha intr map register"]
pub mod sha_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "cpu from cpu 0 intr map register"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "cpu from cpu 0 intr map register"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "cpu from cpu 1 intr map register"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "cpu from cpu 3 intr map register"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: an alias for `Reg<ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "assist debug intr map register"]
pub mod assist_debug_intr_map;
#[doc = "DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "dma pms violatile intr map register"]
pub mod dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "iram0 pms violatile intr map register"]
pub mod core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "mac intr map register"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "BACKUP_PMS_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>`"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod backup_pms_violate_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_CORE0_ACS_INT_MAP_SPEC>`"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "mac intr map register"]
pub mod cache_core0_acs_int_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: an alias for `Reg<INTR_STATUS_REG_0_SPEC>`"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "mac intr map register"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: an alias for `Reg<INTR_STATUS_REG_1_SPEC>`"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "mac intr map register"]
pub mod intr_status_reg_1;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "mac intr map register"]
pub mod clock_gate;
#[doc = "CPU_INT_ENABLE (rw) register accessor: an alias for `Reg<CPU_INT_ENABLE_SPEC>`"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: an alias for `Reg<CPU_INT_TYPE_SPEC>`"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_type;
#[doc = "CPU_INT_CLEAR (rw) register accessor: an alias for `Reg<CPU_INT_CLEAR_SPEC>`"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_clear;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: an alias for `Reg<CPU_INT_EIP_STATUS_SPEC>`"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_0_SPEC>`"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_1_SPEC>`"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_2_SPEC>`"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_3_SPEC>`"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_4_SPEC>`"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_5_SPEC>`"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_6_SPEC>`"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_7_SPEC>`"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_8_SPEC>`"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_9_SPEC>`"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_10_SPEC>`"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_11_SPEC>`"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_12_SPEC>`"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_13_SPEC>`"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_14_SPEC>`"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_15_SPEC>`"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_16_SPEC>`"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_17_SPEC>`"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_18_SPEC>`"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_19_SPEC>`"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_20_SPEC>`"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_21_SPEC>`"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_22_SPEC>`"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_23_SPEC>`"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_24_SPEC>`"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_25_SPEC>`"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_26_SPEC>`"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_27_SPEC>`"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_28_SPEC>`"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_29_SPEC>`"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_30_SPEC>`"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_31_SPEC>`"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH (rw) register accessor: an alias for `Reg<CPU_INT_THRESH_SPEC>`"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "mac intr map register"]
pub mod cpu_int_thresh;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: an alias for `Reg<INTERRUPT_REG_DATE_SPEC>`"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "mac intr map register"]
pub mod interrupt_reg_date;
