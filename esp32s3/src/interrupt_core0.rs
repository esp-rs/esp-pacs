#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - mac interrupt configuration register"]
    pub pro_mac_intr_map: PRO_MAC_INTR_MAP,
    #[doc = "0x04 - mac_nmi interrupt configuration register"]
    pub mac_nmi_map: MAC_NMI_MAP,
    #[doc = "0x08 - pwr interrupt configuration register"]
    pub pwr_intr_map: PWR_INTR_MAP,
    #[doc = "0x0c - bb interrupt configuration register"]
    pub bb_int_map: BB_INT_MAP,
    #[doc = "0x10 - bb_mac interrupt configuration register"]
    pub bt_mac_int_map: BT_MAC_INT_MAP,
    #[doc = "0x14 - bt_bb interrupt configuration register"]
    pub bt_bb_int_map: BT_BB_INT_MAP,
    #[doc = "0x18 - bt_bb_nmi interrupt configuration register"]
    pub bt_bb_nmi_map: BT_BB_NMI_MAP,
    #[doc = "0x1c - rwbt_irq interrupt configuration register"]
    pub rwbt_irq_map: RWBT_IRQ_MAP,
    #[doc = "0x20 - rwble_irq interrupt configuration register"]
    pub rwble_irq_map: RWBLE_IRQ_MAP,
    #[doc = "0x24 - rwbt_nmi interrupt configuration register"]
    pub rwbt_nmi_map: RWBT_NMI_MAP,
    #[doc = "0x28 - rwble_nmi interrupt configuration register"]
    pub rwble_nmi_map: RWBLE_NMI_MAP,
    #[doc = "0x2c - i2c_mst interrupt configuration register"]
    pub i2c_mst_int_map: I2C_MST_INT_MAP,
    #[doc = "0x30 - slc0 interrupt configuration register"]
    pub slc0_intr_map: SLC0_INTR_MAP,
    #[doc = "0x34 - slc1 interrupt configuration register"]
    pub slc1_intr_map: SLC1_INTR_MAP,
    #[doc = "0x38 - uhci0 interrupt configuration register"]
    pub uhci0_intr_map: UHCI0_INTR_MAP,
    #[doc = "0x3c - uhci1 interrupt configuration register"]
    pub uhci1_intr_map: UHCI1_INTR_MAP,
    #[doc = "0x40 - gpio_interrupt_pro interrupt configuration register"]
    pub gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x44 - gpio_interrupt_pro_nmi interrupt configuration register"]
    pub gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x48 - gpio_interrupt_app interrupt configuration register"]
    pub gpio_interrupt_app_map: GPIO_INTERRUPT_APP_MAP,
    #[doc = "0x4c - gpio_interrupt_app_nmi interrupt configuration register"]
    pub gpio_interrupt_app_nmi_map: GPIO_INTERRUPT_APP_NMI_MAP,
    #[doc = "0x50 - spi_intr_1 interrupt configuration register"]
    pub spi_intr_1_map: SPI_INTR_1_MAP,
    #[doc = "0x54 - spi_intr_2 interrupt configuration register"]
    pub spi_intr_2_map: SPI_INTR_2_MAP,
    #[doc = "0x58 - spi_intr_3 interrupt configuration register"]
    pub spi_intr_3_map: SPI_INTR_3_MAP,
    #[doc = "0x5c - spi_intr_4 interrupt configuration register"]
    pub spi_intr_4_map: SPI_INTR_4_MAP,
    #[doc = "0x60 - lcd_cam interrupt configuration register"]
    pub lcd_cam_int_map: LCD_CAM_INT_MAP,
    #[doc = "0x64 - i2s0 interrupt configuration register"]
    pub i2s0_int_map: I2S0_INT_MAP,
    #[doc = "0x68 - i2s1 interrupt configuration register"]
    pub i2s1_int_map: I2S1_INT_MAP,
    #[doc = "0x6c - uart interrupt configuration register"]
    pub uart_intr_map: UART_INTR_MAP,
    #[doc = "0x70 - uart1 interrupt configuration register"]
    pub uart1_intr_map: UART1_INTR_MAP,
    #[doc = "0x74 - uart2 interrupt configuration register"]
    pub uart2_intr_map: UART2_INTR_MAP,
    #[doc = "0x78 - sdio_host interrupt configuration register"]
    pub sdio_host_interrupt_map: SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x7c - pwm0 interrupt configuration register"]
    pub pwm0_intr_map: PWM0_INTR_MAP,
    #[doc = "0x80 - pwm1 interrupt configuration register"]
    pub pwm1_intr_map: PWM1_INTR_MAP,
    #[doc = "0x84 - pwm2 interrupt configuration register"]
    pub pwm2_intr_map: PWM2_INTR_MAP,
    #[doc = "0x88 - pwm3 interrupt configuration register"]
    pub pwm3_intr_map: PWM3_INTR_MAP,
    #[doc = "0x8c - ledc interrupt configuration register"]
    pub ledc_int_map: LEDC_INT_MAP,
    #[doc = "0x90 - efuse interrupt configuration register"]
    pub efuse_int_map: EFUSE_INT_MAP,
    #[doc = "0x94 - can interrupt configuration register"]
    pub can_int_map: CAN_INT_MAP,
    #[doc = "0x98 - usb interrupt configuration register"]
    pub usb_intr_map: USB_INTR_MAP,
    #[doc = "0x9c - rtc_core interrupt configuration register"]
    pub rtc_core_intr_map: RTC_CORE_INTR_MAP,
    #[doc = "0xa0 - rmt interrupt configuration register"]
    pub rmt_intr_map: RMT_INTR_MAP,
    #[doc = "0xa4 - pcnt interrupt configuration register"]
    pub pcnt_intr_map: PCNT_INTR_MAP,
    #[doc = "0xa8 - i2c_ext0 interrupt configuration register"]
    pub i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    #[doc = "0xac - i2c_ext1 interrupt configuration register"]
    pub i2c_ext1_intr_map: I2C_EXT1_INTR_MAP,
    #[doc = "0xb0 - spi2_dma interrupt configuration register"]
    pub spi2_dma_int_map: SPI2_DMA_INT_MAP,
    #[doc = "0xb4 - spi3_dma interrupt configuration register"]
    pub spi3_dma_int_map: SPI3_DMA_INT_MAP,
    #[doc = "0xb8 - spi4_dma interrupt configuration register"]
    pub spi4_dma_int_map: SPI4_DMA_INT_MAP,
    #[doc = "0xbc - wdg interrupt configuration register"]
    pub wdg_int_map: WDG_INT_MAP,
    #[doc = "0xc0 - timer_int1 interrupt configuration register"]
    pub timer_int1_map: TIMER_INT1_MAP,
    #[doc = "0xc4 - timer_int2 interrupt configuration register"]
    pub timer_int2_map: TIMER_INT2_MAP,
    #[doc = "0xc8 - tg_t0 interrupt configuration register"]
    pub tg_t0_int_map: TG_T0_INT_MAP,
    #[doc = "0xcc - tg_t1 interrupt configuration register"]
    pub tg_t1_int_map: TG_T1_INT_MAP,
    #[doc = "0xd0 - tg_wdt interrupt configuration register"]
    pub tg_wdt_int_map: TG_WDT_INT_MAP,
    #[doc = "0xd4 - tg1_t0 interrupt configuration register"]
    pub tg1_t0_int_map: TG1_T0_INT_MAP,
    #[doc = "0xd8 - tg1_t1 interrupt configuration register"]
    pub tg1_t1_int_map: TG1_T1_INT_MAP,
    #[doc = "0xdc - tg1_wdt interrupt configuration register"]
    pub tg1_wdt_int_map: TG1_WDT_INT_MAP,
    #[doc = "0xe0 - cache_ia interrupt configuration register"]
    pub cache_ia_int_map: CACHE_IA_INT_MAP,
    #[doc = "0xe4 - systimer_target0 interrupt configuration register"]
    pub systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0xe8 - systimer_target1 interrupt configuration register"]
    pub systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0xec - systimer_target2 interrupt configuration register"]
    pub systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0xf0 - spi_mem_reject interrupt configuration register"]
    pub spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0xf4 - dcache_prelaod interrupt configuration register"]
    pub dcache_preload_int_map: DCACHE_PRELOAD_INT_MAP,
    #[doc = "0xf8 - icache_preload interrupt configuration register"]
    pub icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    #[doc = "0xfc - dcache_sync interrupt configuration register"]
    pub dcache_sync_int_map: DCACHE_SYNC_INT_MAP,
    #[doc = "0x100 - icache_sync interrupt configuration register"]
    pub icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    #[doc = "0x104 - apb_adc interrupt configuration register"]
    pub apb_adc_int_map: APB_ADC_INT_MAP,
    #[doc = "0x108 - dma_in_ch0 interrupt configuration register"]
    pub dma_in_ch0_int_map: DMA_IN_CH0_INT_MAP,
    #[doc = "0x10c - dma_in_ch1 interrupt configuration register"]
    pub dma_in_ch1_int_map: DMA_IN_CH1_INT_MAP,
    #[doc = "0x110 - dma_in_ch2 interrupt configuration register"]
    pub dma_in_ch2_int_map: DMA_IN_CH2_INT_MAP,
    #[doc = "0x114 - dma_in_ch3 interrupt configuration register"]
    pub dma_in_ch3_int_map: DMA_IN_CH3_INT_MAP,
    #[doc = "0x118 - dma_in_ch4 interrupt configuration register"]
    pub dma_in_ch4_int_map: DMA_IN_CH4_INT_MAP,
    #[doc = "0x11c - dma_out_ch0 interrupt configuration register"]
    pub dma_out_ch0_int_map: DMA_OUT_CH0_INT_MAP,
    #[doc = "0x120 - dma_out_ch1 interrupt configuration register"]
    pub dma_out_ch1_int_map: DMA_OUT_CH1_INT_MAP,
    #[doc = "0x124 - dma_out_ch2 interrupt configuration register"]
    pub dma_out_ch2_int_map: DMA_OUT_CH2_INT_MAP,
    #[doc = "0x128 - dma_out_ch3 interrupt configuration register"]
    pub dma_out_ch3_int_map: DMA_OUT_CH3_INT_MAP,
    #[doc = "0x12c - dma_out_ch4 interrupt configuration register"]
    pub dma_out_ch4_int_map: DMA_OUT_CH4_INT_MAP,
    #[doc = "0x130 - rsa interrupt configuration register"]
    pub rsa_int_map: RSA_INT_MAP,
    #[doc = "0x134 - aes interrupt configuration register"]
    pub aes_int_map: AES_INT_MAP,
    #[doc = "0x138 - sha interrupt configuration register"]
    pub sha_int_map: SHA_INT_MAP,
    #[doc = "0x13c - cpu_intr_from_cpu_0 interrupt configuration register"]
    pub cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x140 - cpu_intr_from_cpu_1 interrupt configuration register"]
    pub cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x144 - cpu_intr_from_cpu_2 interrupt configuration register"]
    pub cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x148 - cpu_intr_from_cpu_3 interrupt configuration register"]
    pub cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x14c - assist_debug interrupt configuration register"]
    pub assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    #[doc = "0x150 - dma_pms_monitor_violatile interrupt configuration register"]
    pub dma_apbperi_pms_monitor_violate_intr_map: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x154 - core0_IRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_0_iram0_pms_monitor_violate_intr_map: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x158 - core0_DRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_0_dram0_pms_monitor_violate_intr_map: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x15c - core0_PIF_pms_monitor_violatile interrupt configuration register"]
    pub core_0_pif_pms_monitor_violate_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x160 - core0_PIF_pms_monitor_violatile_size interrupt configuration register"]
    pub core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0x164 - core1_IRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_1_iram0_pms_monitor_violate_intr_map: CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x168 - core1_DRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_1_dram0_pms_monitor_violate_intr_map: CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x16c - core1_PIF_pms_monitor_violatile interrupt configuration register"]
    pub core_1_pif_pms_monitor_violate_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x170 - core1_PIF_pms_monitor_violatile_size interrupt configuration register"]
    pub core_1_pif_pms_monitor_violate_size_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0x174 - backup_pms_monitor_violatile interrupt configuration register"]
    pub backup_pms_violate_intr_map: BACKUP_PMS_VIOLATE_INTR_MAP,
    #[doc = "0x178 - cache_core0_acs interrupt configuration register"]
    pub cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    #[doc = "0x17c - cache_core1_acs interrupt configuration register"]
    pub cache_core1_acs_int_map: CACHE_CORE1_ACS_INT_MAP,
    #[doc = "0x180 - usb_device interrupt configuration register"]
    pub usb_device_int_map: USB_DEVICE_INT_MAP,
    #[doc = "0x184 - peri_backup interrupt configuration register"]
    pub peri_backup_int_map: PERI_BACKUP_INT_MAP,
    #[doc = "0x188 - dma_extmem_reject interrupt configuration register"]
    pub dma_extmem_reject_int_map: DMA_EXTMEM_REJECT_INT_MAP,
    #[doc = "0x18c - interrupt status register"]
    pub pro_intr_status_0: PRO_INTR_STATUS_0,
    #[doc = "0x190 - interrupt status register"]
    pub pro_intr_status_1: PRO_INTR_STATUS_1,
    #[doc = "0x194 - interrupt status register"]
    pub pro_intr_status_2: PRO_INTR_STATUS_2,
    #[doc = "0x198 - interrupt status register"]
    pub pro_intr_status_3: PRO_INTR_STATUS_3,
    #[doc = "0x19c - clock gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved104: [u8; 0x065c],
    #[doc = "0x7fc - version register"]
    pub date: DATE,
}
#[doc = "PRO_MAC_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_MAC_INTR_MAP_SPEC>`"]
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
#[doc = "mac interrupt configuration register"]
pub mod pro_mac_intr_map;
#[doc = "MAC_NMI_MAP (rw) register accessor: an alias for `Reg<MAC_NMI_MAP_SPEC>`"]
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
#[doc = "mac_nmi interrupt configuration register"]
pub mod mac_nmi_map;
#[doc = "PWR_INTR_MAP (rw) register accessor: an alias for `Reg<PWR_INTR_MAP_SPEC>`"]
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
#[doc = "pwr interrupt configuration register"]
pub mod pwr_intr_map;
#[doc = "BB_INT_MAP (rw) register accessor: an alias for `Reg<BB_INT_MAP_SPEC>`"]
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
#[doc = "bb interrupt configuration register"]
pub mod bb_int_map;
#[doc = "BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<BT_MAC_INT_MAP_SPEC>`"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "bb_mac interrupt configuration register"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<BT_BB_INT_MAP_SPEC>`"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "bt_bb interrupt configuration register"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<BT_BB_NMI_MAP_SPEC>`"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "bt_bb_nmi interrupt configuration register"]
pub mod bt_bb_nmi_map;
#[doc = "RWBT_IRQ_MAP (rw) register accessor: an alias for `Reg<RWBT_IRQ_MAP_SPEC>`"]
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
#[doc = "rwbt_irq interrupt configuration register"]
pub mod rwbt_irq_map;
#[doc = "RWBLE_IRQ_MAP (rw) register accessor: an alias for `Reg<RWBLE_IRQ_MAP_SPEC>`"]
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
#[doc = "rwble_irq interrupt configuration register"]
pub mod rwble_irq_map;
#[doc = "RWBT_NMI_MAP (rw) register accessor: an alias for `Reg<RWBT_NMI_MAP_SPEC>`"]
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
#[doc = "rwbt_nmi interrupt configuration register"]
pub mod rwbt_nmi_map;
#[doc = "RWBLE_NMI_MAP (rw) register accessor: an alias for `Reg<RWBLE_NMI_MAP_SPEC>`"]
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
#[doc = "rwble_nmi interrupt configuration register"]
pub mod rwble_nmi_map;
#[doc = "I2C_MST_INT_MAP (rw) register accessor: an alias for `Reg<I2C_MST_INT_MAP_SPEC>`"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "i2c_mst interrupt configuration register"]
pub mod i2c_mst_int_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: an alias for `Reg<SLC0_INTR_MAP_SPEC>`"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "slc0 interrupt configuration register"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: an alias for `Reg<SLC1_INTR_MAP_SPEC>`"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "slc1 interrupt configuration register"]
pub mod slc1_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: an alias for `Reg<UHCI0_INTR_MAP_SPEC>`"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "uhci0 interrupt configuration register"]
pub mod uhci0_intr_map;
#[doc = "UHCI1_INTR_MAP (rw) register accessor: an alias for `Reg<UHCI1_INTR_MAP_SPEC>`"]
pub type UHCI1_INTR_MAP = crate::Reg<uhci1_intr_map::UHCI1_INTR_MAP_SPEC>;
#[doc = "uhci1 interrupt configuration register"]
pub mod uhci1_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "gpio_interrupt_pro interrupt configuration register"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "gpio_interrupt_pro_nmi interrupt configuration register"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "GPIO_INTERRUPT_APP_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_APP_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_APP_MAP = crate::Reg<gpio_interrupt_app_map::GPIO_INTERRUPT_APP_MAP_SPEC>;
#[doc = "gpio_interrupt_app interrupt configuration register"]
pub mod gpio_interrupt_app_map;
#[doc = "GPIO_INTERRUPT_APP_NMI_MAP (rw) register accessor: an alias for `Reg<GPIO_INTERRUPT_APP_NMI_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_APP_NMI_MAP =
    crate::Reg<gpio_interrupt_app_nmi_map::GPIO_INTERRUPT_APP_NMI_MAP_SPEC>;
#[doc = "gpio_interrupt_app_nmi interrupt configuration register"]
pub mod gpio_interrupt_app_nmi_map;
#[doc = "SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_1_MAP_SPEC>`"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "spi_intr_1 interrupt configuration register"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_2_MAP_SPEC>`"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "spi_intr_2 interrupt configuration register"]
pub mod spi_intr_2_map;
#[doc = "SPI_INTR_3_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_3_MAP_SPEC>`"]
pub type SPI_INTR_3_MAP = crate::Reg<spi_intr_3_map::SPI_INTR_3_MAP_SPEC>;
#[doc = "spi_intr_3 interrupt configuration register"]
pub mod spi_intr_3_map;
#[doc = "SPI_INTR_4_MAP (rw) register accessor: an alias for `Reg<SPI_INTR_4_MAP_SPEC>`"]
pub type SPI_INTR_4_MAP = crate::Reg<spi_intr_4_map::SPI_INTR_4_MAP_SPEC>;
#[doc = "spi_intr_4 interrupt configuration register"]
pub mod spi_intr_4_map;
#[doc = "LCD_CAM_INT_MAP (rw) register accessor: an alias for `Reg<LCD_CAM_INT_MAP_SPEC>`"]
pub type LCD_CAM_INT_MAP = crate::Reg<lcd_cam_int_map::LCD_CAM_INT_MAP_SPEC>;
#[doc = "lcd_cam interrupt configuration register"]
pub mod lcd_cam_int_map;
#[doc = "I2S0_INT_MAP (rw) register accessor: an alias for `Reg<I2S0_INT_MAP_SPEC>`"]
pub type I2S0_INT_MAP = crate::Reg<i2s0_int_map::I2S0_INT_MAP_SPEC>;
#[doc = "i2s0 interrupt configuration register"]
pub mod i2s0_int_map;
#[doc = "I2S1_INT_MAP (rw) register accessor: an alias for `Reg<I2S1_INT_MAP_SPEC>`"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "i2s1 interrupt configuration register"]
pub mod i2s1_int_map;
#[doc = "UART_INTR_MAP (rw) register accessor: an alias for `Reg<UART_INTR_MAP_SPEC>`"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "uart interrupt configuration register"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: an alias for `Reg<UART1_INTR_MAP_SPEC>`"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "uart1 interrupt configuration register"]
pub mod uart1_intr_map;
#[doc = "UART2_INTR_MAP (rw) register accessor: an alias for `Reg<UART2_INTR_MAP_SPEC>`"]
pub type UART2_INTR_MAP = crate::Reg<uart2_intr_map::UART2_INTR_MAP_SPEC>;
#[doc = "uart2 interrupt configuration register"]
pub mod uart2_intr_map;
#[doc = "SDIO_HOST_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<SDIO_HOST_INTERRUPT_MAP_SPEC>`"]
pub type SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<sdio_host_interrupt_map::SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "sdio_host interrupt configuration register"]
pub mod sdio_host_interrupt_map;
#[doc = "PWM0_INTR_MAP (rw) register accessor: an alias for `Reg<PWM0_INTR_MAP_SPEC>`"]
pub type PWM0_INTR_MAP = crate::Reg<pwm0_intr_map::PWM0_INTR_MAP_SPEC>;
#[doc = "pwm0 interrupt configuration register"]
pub mod pwm0_intr_map;
#[doc = "PWM1_INTR_MAP (rw) register accessor: an alias for `Reg<PWM1_INTR_MAP_SPEC>`"]
pub type PWM1_INTR_MAP = crate::Reg<pwm1_intr_map::PWM1_INTR_MAP_SPEC>;
#[doc = "pwm1 interrupt configuration register"]
pub mod pwm1_intr_map;
#[doc = "PWM2_INTR_MAP (rw) register accessor: an alias for `Reg<PWM2_INTR_MAP_SPEC>`"]
pub type PWM2_INTR_MAP = crate::Reg<pwm2_intr_map::PWM2_INTR_MAP_SPEC>;
#[doc = "pwm2 interrupt configuration register"]
pub mod pwm2_intr_map;
#[doc = "PWM3_INTR_MAP (rw) register accessor: an alias for `Reg<PWM3_INTR_MAP_SPEC>`"]
pub type PWM3_INTR_MAP = crate::Reg<pwm3_intr_map::PWM3_INTR_MAP_SPEC>;
#[doc = "pwm3 interrupt configuration register"]
pub mod pwm3_intr_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: an alias for `Reg<LEDC_INT_MAP_SPEC>`"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "ledc interrupt configuration register"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<EFUSE_INT_MAP_SPEC>`"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "efuse interrupt configuration register"]
pub mod efuse_int_map;
#[doc = "CAN_INT_MAP (rw) register accessor: an alias for `Reg<CAN_INT_MAP_SPEC>`"]
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
#[doc = "can interrupt configuration register"]
pub mod can_int_map;
#[doc = "USB_INTR_MAP (rw) register accessor: an alias for `Reg<USB_INTR_MAP_SPEC>`"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "usb interrupt configuration register"]
pub mod usb_intr_map;
#[doc = "RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<RTC_CORE_INTR_MAP_SPEC>`"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "rtc_core interrupt configuration register"]
pub mod rtc_core_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: an alias for `Reg<RMT_INTR_MAP_SPEC>`"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "rmt interrupt configuration register"]
pub mod rmt_intr_map;
#[doc = "PCNT_INTR_MAP (rw) register accessor: an alias for `Reg<PCNT_INTR_MAP_SPEC>`"]
pub type PCNT_INTR_MAP = crate::Reg<pcnt_intr_map::PCNT_INTR_MAP_SPEC>;
#[doc = "pcnt interrupt configuration register"]
pub mod pcnt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<I2C_EXT0_INTR_MAP_SPEC>`"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "i2c_ext0 interrupt configuration register"]
pub mod i2c_ext0_intr_map;
#[doc = "I2C_EXT1_INTR_MAP (rw) register accessor: an alias for `Reg<I2C_EXT1_INTR_MAP_SPEC>`"]
pub type I2C_EXT1_INTR_MAP = crate::Reg<i2c_ext1_intr_map::I2C_EXT1_INTR_MAP_SPEC>;
#[doc = "i2c_ext1 interrupt configuration register"]
pub mod i2c_ext1_intr_map;
#[doc = "SPI2_DMA_INT_MAP (rw) register accessor: an alias for `Reg<SPI2_DMA_INT_MAP_SPEC>`"]
pub type SPI2_DMA_INT_MAP = crate::Reg<spi2_dma_int_map::SPI2_DMA_INT_MAP_SPEC>;
#[doc = "spi2_dma interrupt configuration register"]
pub mod spi2_dma_int_map;
#[doc = "SPI3_DMA_INT_MAP (rw) register accessor: an alias for `Reg<SPI3_DMA_INT_MAP_SPEC>`"]
pub type SPI3_DMA_INT_MAP = crate::Reg<spi3_dma_int_map::SPI3_DMA_INT_MAP_SPEC>;
#[doc = "spi3_dma interrupt configuration register"]
pub mod spi3_dma_int_map;
#[doc = "SPI4_DMA_INT_MAP (rw) register accessor: an alias for `Reg<SPI4_DMA_INT_MAP_SPEC>`"]
pub type SPI4_DMA_INT_MAP = crate::Reg<spi4_dma_int_map::SPI4_DMA_INT_MAP_SPEC>;
#[doc = "spi4_dma interrupt configuration register"]
pub mod spi4_dma_int_map;
#[doc = "WDG_INT_MAP (rw) register accessor: an alias for `Reg<WDG_INT_MAP_SPEC>`"]
pub type WDG_INT_MAP = crate::Reg<wdg_int_map::WDG_INT_MAP_SPEC>;
#[doc = "wdg interrupt configuration register"]
pub mod wdg_int_map;
#[doc = "TIMER_INT1_MAP (rw) register accessor: an alias for `Reg<TIMER_INT1_MAP_SPEC>`"]
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
#[doc = "timer_int1 interrupt configuration register"]
pub mod timer_int1_map;
#[doc = "TIMER_INT2_MAP (rw) register accessor: an alias for `Reg<TIMER_INT2_MAP_SPEC>`"]
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
#[doc = "timer_int2 interrupt configuration register"]
pub mod timer_int2_map;
#[doc = "TG_T0_INT_MAP (rw) register accessor: an alias for `Reg<TG_T0_INT_MAP_SPEC>`"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "tg_t0 interrupt configuration register"]
pub mod tg_t0_int_map;
#[doc = "TG_T1_INT_MAP (rw) register accessor: an alias for `Reg<TG_T1_INT_MAP_SPEC>`"]
pub type TG_T1_INT_MAP = crate::Reg<tg_t1_int_map::TG_T1_INT_MAP_SPEC>;
#[doc = "tg_t1 interrupt configuration register"]
pub mod tg_t1_int_map;
#[doc = "TG_WDT_INT_MAP (rw) register accessor: an alias for `Reg<TG_WDT_INT_MAP_SPEC>`"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "tg_wdt interrupt configuration register"]
pub mod tg_wdt_int_map;
#[doc = "TG1_T0_INT_MAP (rw) register accessor: an alias for `Reg<TG1_T0_INT_MAP_SPEC>`"]
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
#[doc = "tg1_t0 interrupt configuration register"]
pub mod tg1_t0_int_map;
#[doc = "TG1_T1_INT_MAP (rw) register accessor: an alias for `Reg<TG1_T1_INT_MAP_SPEC>`"]
pub type TG1_T1_INT_MAP = crate::Reg<tg1_t1_int_map::TG1_T1_INT_MAP_SPEC>;
#[doc = "tg1_t1 interrupt configuration register"]
pub mod tg1_t1_int_map;
#[doc = "TG1_WDT_INT_MAP (rw) register accessor: an alias for `Reg<TG1_WDT_INT_MAP_SPEC>`"]
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
#[doc = "tg1_wdt interrupt configuration register"]
pub mod tg1_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_IA_INT_MAP_SPEC>`"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "cache_ia interrupt configuration register"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "systimer_target0 interrupt configuration register"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "systimer_target1 interrupt configuration register"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: an alias for `Reg<SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "systimer_target2 interrupt configuration register"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP (rw) register accessor: an alias for `Reg<SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "spi_mem_reject interrupt configuration register"]
pub mod spi_mem_reject_intr_map;
#[doc = "DCACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<DCACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type DCACHE_PRELOAD_INT_MAP = crate::Reg<dcache_preload_int_map::DCACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "dcache_prelaod interrupt configuration register"]
pub mod dcache_preload_int_map;
#[doc = "ICACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "icache_preload interrupt configuration register"]
pub mod icache_preload_int_map;
#[doc = "DCACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<DCACHE_SYNC_INT_MAP_SPEC>`"]
pub type DCACHE_SYNC_INT_MAP = crate::Reg<dcache_sync_int_map::DCACHE_SYNC_INT_MAP_SPEC>;
#[doc = "dcache_sync interrupt configuration register"]
pub mod dcache_sync_int_map;
#[doc = "ICACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "icache_sync interrupt configuration register"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP (rw) register accessor: an alias for `Reg<APB_ADC_INT_MAP_SPEC>`"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "apb_adc interrupt configuration register"]
pub mod apb_adc_int_map;
#[doc = "DMA_IN_CH0_INT_MAP (rw) register accessor: an alias for `Reg<DMA_IN_CH0_INT_MAP_SPEC>`"]
pub type DMA_IN_CH0_INT_MAP = crate::Reg<dma_in_ch0_int_map::DMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "dma_in_ch0 interrupt configuration register"]
pub mod dma_in_ch0_int_map;
#[doc = "DMA_IN_CH1_INT_MAP (rw) register accessor: an alias for `Reg<DMA_IN_CH1_INT_MAP_SPEC>`"]
pub type DMA_IN_CH1_INT_MAP = crate::Reg<dma_in_ch1_int_map::DMA_IN_CH1_INT_MAP_SPEC>;
#[doc = "dma_in_ch1 interrupt configuration register"]
pub mod dma_in_ch1_int_map;
#[doc = "DMA_IN_CH2_INT_MAP (rw) register accessor: an alias for `Reg<DMA_IN_CH2_INT_MAP_SPEC>`"]
pub type DMA_IN_CH2_INT_MAP = crate::Reg<dma_in_ch2_int_map::DMA_IN_CH2_INT_MAP_SPEC>;
#[doc = "dma_in_ch2 interrupt configuration register"]
pub mod dma_in_ch2_int_map;
#[doc = "DMA_IN_CH3_INT_MAP (rw) register accessor: an alias for `Reg<DMA_IN_CH3_INT_MAP_SPEC>`"]
pub type DMA_IN_CH3_INT_MAP = crate::Reg<dma_in_ch3_int_map::DMA_IN_CH3_INT_MAP_SPEC>;
#[doc = "dma_in_ch3 interrupt configuration register"]
pub mod dma_in_ch3_int_map;
#[doc = "DMA_IN_CH4_INT_MAP (rw) register accessor: an alias for `Reg<DMA_IN_CH4_INT_MAP_SPEC>`"]
pub type DMA_IN_CH4_INT_MAP = crate::Reg<dma_in_ch4_int_map::DMA_IN_CH4_INT_MAP_SPEC>;
#[doc = "dma_in_ch4 interrupt configuration register"]
pub mod dma_in_ch4_int_map;
#[doc = "DMA_OUT_CH0_INT_MAP (rw) register accessor: an alias for `Reg<DMA_OUT_CH0_INT_MAP_SPEC>`"]
pub type DMA_OUT_CH0_INT_MAP = crate::Reg<dma_out_ch0_int_map::DMA_OUT_CH0_INT_MAP_SPEC>;
#[doc = "dma_out_ch0 interrupt configuration register"]
pub mod dma_out_ch0_int_map;
#[doc = "DMA_OUT_CH1_INT_MAP (rw) register accessor: an alias for `Reg<DMA_OUT_CH1_INT_MAP_SPEC>`"]
pub type DMA_OUT_CH1_INT_MAP = crate::Reg<dma_out_ch1_int_map::DMA_OUT_CH1_INT_MAP_SPEC>;
#[doc = "dma_out_ch1 interrupt configuration register"]
pub mod dma_out_ch1_int_map;
#[doc = "DMA_OUT_CH2_INT_MAP (rw) register accessor: an alias for `Reg<DMA_OUT_CH2_INT_MAP_SPEC>`"]
pub type DMA_OUT_CH2_INT_MAP = crate::Reg<dma_out_ch2_int_map::DMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "dma_out_ch2 interrupt configuration register"]
pub mod dma_out_ch2_int_map;
#[doc = "DMA_OUT_CH3_INT_MAP (rw) register accessor: an alias for `Reg<DMA_OUT_CH3_INT_MAP_SPEC>`"]
pub type DMA_OUT_CH3_INT_MAP = crate::Reg<dma_out_ch3_int_map::DMA_OUT_CH3_INT_MAP_SPEC>;
#[doc = "dma_out_ch3 interrupt configuration register"]
pub mod dma_out_ch3_int_map;
#[doc = "DMA_OUT_CH4_INT_MAP (rw) register accessor: an alias for `Reg<DMA_OUT_CH4_INT_MAP_SPEC>`"]
pub type DMA_OUT_CH4_INT_MAP = crate::Reg<dma_out_ch4_int_map::DMA_OUT_CH4_INT_MAP_SPEC>;
#[doc = "dma_out_ch4 interrupt configuration register"]
pub mod dma_out_ch4_int_map;
#[doc = "RSA_INT_MAP (rw) register accessor: an alias for `Reg<RSA_INT_MAP_SPEC>`"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "rsa interrupt configuration register"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP (rw) register accessor: an alias for `Reg<AES_INT_MAP_SPEC>`"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "aes interrupt configuration register"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: an alias for `Reg<SHA_INT_MAP_SPEC>`"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "sha interrupt configuration register"]
pub mod sha_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_0 interrupt configuration register"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_1 interrupt configuration register"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_2 interrupt configuration register"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_3 interrupt configuration register"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: an alias for `Reg<ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "assist_debug interrupt configuration register"]
pub mod assist_debug_intr_map;
#[doc = "DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "dma_pms_monitor_violatile interrupt configuration register"]
pub mod dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_IRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_DRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_PIF_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "core0_PIF_pms_monitor_violatile_size interrupt configuration register"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_iram0_pms_monitor_violate_intr_map::CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_IRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_dram0_pms_monitor_violate_intr_map::CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_DRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_PIF_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: an alias for `Reg<CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_size_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "core1_PIF_pms_monitor_violatile_size interrupt configuration register"]
pub mod core_1_pif_pms_monitor_violate_size_intr_map;
#[doc = "BACKUP_PMS_VIOLATE_INTR_MAP (rw) register accessor: an alias for `Reg<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>`"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "backup_pms_monitor_violatile interrupt configuration register"]
pub mod backup_pms_violate_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_CORE0_ACS_INT_MAP_SPEC>`"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "cache_core0_acs interrupt configuration register"]
pub mod cache_core0_acs_int_map;
#[doc = "CACHE_CORE1_ACS_INT_MAP (rw) register accessor: an alias for `Reg<CACHE_CORE1_ACS_INT_MAP_SPEC>`"]
pub type CACHE_CORE1_ACS_INT_MAP =
    crate::Reg<cache_core1_acs_int_map::CACHE_CORE1_ACS_INT_MAP_SPEC>;
#[doc = "cache_core1_acs interrupt configuration register"]
pub mod cache_core1_acs_int_map;
#[doc = "USB_DEVICE_INT_MAP (rw) register accessor: an alias for `Reg<USB_DEVICE_INT_MAP_SPEC>`"]
pub type USB_DEVICE_INT_MAP = crate::Reg<usb_device_int_map::USB_DEVICE_INT_MAP_SPEC>;
#[doc = "usb_device interrupt configuration register"]
pub mod usb_device_int_map;
#[doc = "PERI_BACKUP_INT_MAP (rw) register accessor: an alias for `Reg<PERI_BACKUP_INT_MAP_SPEC>`"]
pub type PERI_BACKUP_INT_MAP = crate::Reg<peri_backup_int_map::PERI_BACKUP_INT_MAP_SPEC>;
#[doc = "peri_backup interrupt configuration register"]
pub mod peri_backup_int_map;
#[doc = "DMA_EXTMEM_REJECT_INT_MAP (rw) register accessor: an alias for `Reg<DMA_EXTMEM_REJECT_INT_MAP_SPEC>`"]
pub type DMA_EXTMEM_REJECT_INT_MAP =
    crate::Reg<dma_extmem_reject_int_map::DMA_EXTMEM_REJECT_INT_MAP_SPEC>;
#[doc = "dma_extmem_reject interrupt configuration register"]
pub mod dma_extmem_reject_int_map;
#[doc = "PRO_INTR_STATUS_0 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_0_SPEC>`"]
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
#[doc = "interrupt status register"]
pub mod pro_intr_status_0;
#[doc = "PRO_INTR_STATUS_1 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_1_SPEC>`"]
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
#[doc = "interrupt status register"]
pub mod pro_intr_status_1;
#[doc = "PRO_INTR_STATUS_2 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_2_SPEC>`"]
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
#[doc = "interrupt status register"]
pub mod pro_intr_status_2;
#[doc = "PRO_INTR_STATUS_3 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_3_SPEC>`"]
pub type PRO_INTR_STATUS_3 = crate::Reg<pro_intr_status_3::PRO_INTR_STATUS_3_SPEC>;
#[doc = "interrupt status register"]
pub mod pro_intr_status_3;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
