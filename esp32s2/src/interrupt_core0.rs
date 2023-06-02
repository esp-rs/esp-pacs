#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC_INTR interrupt configuration register"]
    pub pro_mac_intr_map: PRO_MAC_INTR_MAP,
    #[doc = "0x04 - MAC_NMI interrupt configuration register"]
    pub pro_mac_nmi_map: PRO_MAC_NMI_MAP,
    #[doc = "0x08 - PWR_INTR interrupt configuration register"]
    pub pro_pwr_intr_map: PRO_PWR_INTR_MAP,
    #[doc = "0x0c - BB_INT interrupt configuration register"]
    pub pro_bb_int_map: PRO_BB_INT_MAP,
    #[doc = "0x10 - BT_MAC_INT interrupt configuration register"]
    pub pro_bt_mac_int_map: PRO_BT_MAC_INT_MAP,
    #[doc = "0x14 - BT_BB_INT interrupt configuration register"]
    pub pro_bt_bb_int_map: PRO_BT_BB_INT_MAP,
    #[doc = "0x18 - BT_BB_NMI interrupt configuration register"]
    pub pro_bt_bb_nmi_map: PRO_BT_BB_NMI_MAP,
    #[doc = "0x1c - RWBT_IRQ interrupt configuration register"]
    pub pro_rwbt_irq_map: PRO_RWBT_IRQ_MAP,
    #[doc = "0x20 - RWBLE_IRQ interrupt configuration register"]
    pub pro_rwble_irq_map: PRO_RWBLE_IRQ_MAP,
    #[doc = "0x24 - RWBT_NMI interrupt configuration register"]
    pub pro_rwbt_nmi_map: PRO_RWBT_NMI_MAP,
    #[doc = "0x28 - RWBLE_NMI interrupt configuration register"]
    pub pro_rwble_nmi_map: PRO_RWBLE_NMI_MAP,
    #[doc = "0x2c - SLC0_INTR interrupt configuration register"]
    pub pro_slc0_intr_map: PRO_SLC0_INTR_MAP,
    #[doc = "0x30 - SLC1_INTR interrupt configuration register"]
    pub pro_slc1_intr_map: PRO_SLC1_INTR_MAP,
    #[doc = "0x34 - UHCI0_INTR interrupt configuration register"]
    pub pro_uhci0_intr_map: PRO_UHCI0_INTR_MAP,
    #[doc = "0x38 - UHCI1_INTR interrupt configuration register"]
    pub pro_uhci1_intr_map: PRO_UHCI1_INTR_MAP,
    #[doc = "0x3c - TG_T0_LEVEL_INT interrupt configuration register"]
    pub pro_tg_t0_level_int_map: PRO_TG_T0_LEVEL_INT_MAP,
    #[doc = "0x40 - TG_T1_LEVEL_INT interrupt configuration register"]
    pub pro_tg_t1_level_int_map: PRO_TG_T1_LEVEL_INT_MAP,
    #[doc = "0x44 - TG_WDT_LEVEL_INT interrupt configuration register"]
    pub pro_tg_wdt_level_int_map: PRO_TG_WDT_LEVEL_INT_MAP,
    #[doc = "0x48 - TG_LACT_LEVEL_INT interrupt configuration register"]
    pub pro_tg_lact_level_int_map: PRO_TG_LACT_LEVEL_INT_MAP,
    #[doc = "0x4c - TG1_T0_LEVEL_INT interrupt configuration register"]
    pub pro_tg1_t0_level_int_map: PRO_TG1_T0_LEVEL_INT_MAP,
    #[doc = "0x50 - TG1_T1_LEVEL_INT interrupt configuration register"]
    pub pro_tg1_t1_level_int_map: PRO_TG1_T1_LEVEL_INT_MAP,
    #[doc = "0x54 - TG1_WDT_LEVEL_INT interrupt configuration register"]
    pub pro_tg1_wdt_level_int_map: PRO_TG1_WDT_LEVEL_INT_MAP,
    #[doc = "0x58 - TG1_LACT_LEVEL_INT interrupt configuration register"]
    pub pro_tg1_lact_level_int_map: PRO_TG1_LACT_LEVEL_INT_MAP,
    #[doc = "0x5c - GPIO_INTERRUPT_PRO interrupt configuration register"]
    pub pro_gpio_interrupt_pro_map: PRO_GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x60 - GPIO_INTERRUPT_PRO_NMI interrupt configuration register"]
    pub pro_gpio_interrupt_pro_nmi_map: PRO_GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x64 - GPIO_INTERRUPT_APP interrupt configuration register"]
    pub pro_gpio_interrupt_app_map: PRO_GPIO_INTERRUPT_APP_MAP,
    #[doc = "0x68 - GPIO_INTERRUPT_APP_NMI interrupt configuration register"]
    pub pro_gpio_interrupt_app_nmi_map: PRO_GPIO_INTERRUPT_APP_NMI_MAP,
    #[doc = "0x6c - DEDICATED_GPIO_IN_INTR interrupt configuration register"]
    pub pro_dedicated_gpio_in_intr_map: PRO_DEDICATED_GPIO_IN_INTR_MAP,
    #[doc = "0x70 - CPU_INTR_FROM_CPU_0 interrupt configuration register"]
    pub pro_cpu_intr_from_cpu_0_map: PRO_CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x74 - CPU_INTR_FROM_CPU_1 interrupt configuration register"]
    pub pro_cpu_intr_from_cpu_1_map: PRO_CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x78 - CPU_INTR_FROM_CPU_2 interrupt configuration register"]
    pub pro_cpu_intr_from_cpu_2_map: PRO_CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x7c - CPU_INTR_FROM_CPU_3 interrupt configuration register"]
    pub pro_cpu_intr_from_cpu_3_map: PRO_CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x80 - SPI_INTR_1 interrupt configuration register"]
    pub pro_spi_intr_1_map: PRO_SPI_INTR_1_MAP,
    #[doc = "0x84 - SPI_INTR_2 interrupt configuration register"]
    pub pro_spi_intr_2_map: PRO_SPI_INTR_2_MAP,
    #[doc = "0x88 - SPI_INTR_3 interrupt configuration register"]
    pub pro_spi_intr_3_map: PRO_SPI_INTR_3_MAP,
    #[doc = "0x8c - I2S0_INT interrupt configuration register"]
    pub pro_i2s0_int_map: PRO_I2S0_INT_MAP,
    #[doc = "0x90 - I2S1_INT interrupt configuration register"]
    pub pro_i2s1_int_map: PRO_I2S1_INT_MAP,
    #[doc = "0x94 - UART_INT interrupt configuration register"]
    pub pro_uart_intr_map: PRO_UART_INTR_MAP,
    #[doc = "0x98 - UART1_INT interrupt configuration register"]
    pub pro_uart1_intr_map: PRO_UART1_INTR_MAP,
    #[doc = "0x9c - UART2_INT interrupt configuration register"]
    pub pro_uart2_intr_map: PRO_UART2_INTR_MAP,
    #[doc = "0xa0 - SDIO_HOST_INTERRUPT configuration register"]
    pub pro_sdio_host_interrupt_map: PRO_SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0xa4 - PWM0_INTR interrupt configuration register"]
    pub pro_pwm0_intr_map: PRO_PWM0_INTR_MAP,
    #[doc = "0xa8 - PWM1_INTR interrupt configuration register"]
    pub pro_pwm1_intr_map: PRO_PWM1_INTR_MAP,
    #[doc = "0xac - PWM2_INTR interrupt configuration register"]
    pub pro_pwm2_intr_map: PRO_PWM2_INTR_MAP,
    #[doc = "0xb0 - PWM3_INTR interrupt configuration register"]
    pub pro_pwm3_intr_map: PRO_PWM3_INTR_MAP,
    #[doc = "0xb4 - LEDC_INTR interrupt configuration register"]
    pub pro_ledc_int_map: PRO_LEDC_INT_MAP,
    #[doc = "0xb8 - EFUSE_INT interrupt configuration register"]
    pub pro_efuse_int_map: PRO_EFUSE_INT_MAP,
    #[doc = "0xbc - CAN_INT interrupt configuration register"]
    pub pro_can_int_map: PRO_CAN_INT_MAP,
    #[doc = "0xc0 - USB_INT interrupt configuration register"]
    pub pro_usb_intr_map: PRO_USB_INTR_MAP,
    #[doc = "0xc4 - RTC_CORE_INTR interrupt configuration register"]
    pub pro_rtc_core_intr_map: PRO_RTC_CORE_INTR_MAP,
    #[doc = "0xc8 - RMT_INTR interrupt configuration register"]
    pub pro_rmt_intr_map: PRO_RMT_INTR_MAP,
    #[doc = "0xcc - PCNT_INTR interrupt configuration register"]
    pub pro_pcnt_intr_map: PRO_PCNT_INTR_MAP,
    #[doc = "0xd0 - I2C_EXT0_INTR interrupt configuration register"]
    pub pro_i2c_ext0_intr_map: PRO_I2C_EXT0_INTR_MAP,
    #[doc = "0xd4 - I2C_EXT1_INTR interrupt configuration register"]
    pub pro_i2c_ext1_intr_map: PRO_I2C_EXT1_INTR_MAP,
    #[doc = "0xd8 - RSA_INTR interrupt configuration register"]
    pub pro_rsa_intr_map: PRO_RSA_INTR_MAP,
    #[doc = "0xdc - SHA_INTR interrupt configuration register"]
    pub pro_sha_intr_map: PRO_SHA_INTR_MAP,
    #[doc = "0xe0 - AES_INTR interrupt configuration register"]
    pub pro_aes_intr_map: PRO_AES_INTR_MAP,
    #[doc = "0xe4 - SPI2_DMA_INT interrupt configuration register"]
    pub pro_spi2_dma_int_map: PRO_SPI2_DMA_INT_MAP,
    #[doc = "0xe8 - SPI3_DMA_INT interrupt configuration register"]
    pub pro_spi3_dma_int_map: PRO_SPI3_DMA_INT_MAP,
    #[doc = "0xec - WDG_INT interrupt configuration register"]
    pub pro_wdg_int_map: PRO_WDG_INT_MAP,
    #[doc = "0xf0 - TIMER_INT1 interrupt configuration register"]
    pub pro_timer_int1_map: PRO_TIMER_INT1_MAP,
    #[doc = "0xf4 - TIMER_INT2 interrupt configuration register"]
    pub pro_timer_int2_map: PRO_TIMER_INT2_MAP,
    #[doc = "0xf8 - TG_T0_EDGE_INT interrupt configuration register"]
    pub pro_tg_t0_edge_int_map: PRO_TG_T0_EDGE_INT_MAP,
    #[doc = "0xfc - TG_T1_EDGE_INT interrupt configuration register"]
    pub pro_tg_t1_edge_int_map: PRO_TG_T1_EDGE_INT_MAP,
    #[doc = "0x100 - TG_WDT_EDGE_INT interrupt configuration register"]
    pub pro_tg_wdt_edge_int_map: PRO_TG_WDT_EDGE_INT_MAP,
    #[doc = "0x104 - TG_LACT_EDGE_INT interrupt configuration register"]
    pub pro_tg_lact_edge_int_map: PRO_TG_LACT_EDGE_INT_MAP,
    #[doc = "0x108 - TG1_T0_EDGE_INT interrupt configuration register"]
    pub pro_tg1_t0_edge_int_map: PRO_TG1_T0_EDGE_INT_MAP,
    #[doc = "0x10c - TG1_T1_EDGE_INT interrupt configuration register"]
    pub pro_tg1_t1_edge_int_map: PRO_TG1_T1_EDGE_INT_MAP,
    #[doc = "0x110 - TG1_WDT_EDGE_INT interrupt configuration register"]
    pub pro_tg1_wdt_edge_int_map: PRO_TG1_WDT_EDGE_INT_MAP,
    #[doc = "0x114 - TG1_LACT_EDGE_INT interrupt configuration register"]
    pub pro_tg1_lact_edge_int_map: PRO_TG1_LACT_EDGE_INT_MAP,
    #[doc = "0x118 - CACHE_IA_INT interrupt configuration register"]
    pub pro_cache_ia_int_map: PRO_CACHE_IA_INT_MAP,
    #[doc = "0x11c - SYSTIMER_TARGET0_INT interrupt configuration register"]
    pub pro_systimer_target0_int_map: PRO_SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0x120 - SYSTIMER_TARGET1_INT interrupt configuration register"]
    pub pro_systimer_target1_int_map: PRO_SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0x124 - SYSTIMER_TARGET2_INT interrupt configuration register"]
    pub pro_systimer_target2_int_map: PRO_SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0x128 - ASSIST_DEBUG_INTR interrupt configuration register"]
    pub pro_assist_debug_intr_map: PRO_ASSIST_DEBUG_INTR_MAP,
    #[doc = "0x12c - PMS_PRO_IRAM0_ILG interrupt configuration register"]
    pub pro_pms_pro_iram0_ilg_intr_map: PRO_PMS_PRO_IRAM0_ILG_INTR_MAP,
    #[doc = "0x130 - PMS_PRO_DRAM0_ILG interrupt configuration register"]
    pub pro_pms_pro_dram0_ilg_intr_map: PRO_PMS_PRO_DRAM0_ILG_INTR_MAP,
    #[doc = "0x134 - PMS_PRO_DPORT_ILG interrupt configuration register"]
    pub pro_pms_pro_dport_ilg_intr_map: PRO_PMS_PRO_DPORT_ILG_INTR_MAP,
    #[doc = "0x138 - PMS_PRO_AHB_ILG interrupt configuration register"]
    pub pro_pms_pro_ahb_ilg_intr_map: PRO_PMS_PRO_AHB_ILG_INTR_MAP,
    #[doc = "0x13c - PMS_PRO_CACHE_ILG interrupt configuration register"]
    pub pro_pms_pro_cache_ilg_intr_map: PRO_PMS_PRO_CACHE_ILG_INTR_MAP,
    #[doc = "0x140 - PMS_DMA_APB_I_ILG interrupt configuration register"]
    pub pro_pms_dma_apb_i_ilg_intr_map: PRO_PMS_DMA_APB_I_ILG_INTR_MAP,
    #[doc = "0x144 - PMS_DMA_RX_I_ILG interrupt configuration register"]
    pub pro_pms_dma_rx_i_ilg_intr_map: PRO_PMS_DMA_RX_I_ILG_INTR_MAP,
    #[doc = "0x148 - PMS_DMA_TX_I_ILG interrupt configuration register"]
    pub pro_pms_dma_tx_i_ilg_intr_map: PRO_PMS_DMA_TX_I_ILG_INTR_MAP,
    #[doc = "0x14c - SPI_MEM_REJECT_INTR interrupt configuration register"]
    pub pro_spi_mem_reject_intr_map: PRO_SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0x150 - DMA_COPY_INTR interrupt configuration register"]
    pub pro_dma_copy_intr_map: PRO_DMA_COPY_INTR_MAP,
    #[doc = "0x154 - SPI4_DMA_INT interrupt configuration register"]
    pub pro_spi4_dma_int_map: PRO_SPI4_DMA_INT_MAP,
    #[doc = "0x158 - SPI_INTR_4 interrupt configuration register"]
    pub pro_spi_intr_4_map: PRO_SPI_INTR_4_MAP,
    #[doc = "0x15c - DCACHE_PRELOAD_INT interrupt configuration register"]
    pub pro_dcache_preload_int_map: PRO_DCACHE_PRELOAD_INT_MAP,
    #[doc = "0x160 - ICACHE_PRELOAD_INT interrupt configuration register"]
    pub pro_icache_preload_int_map: PRO_ICACHE_PRELOAD_INT_MAP,
    #[doc = "0x164 - APB_ADC_INT interrupt configuration register"]
    pub pro_apb_adc_int_map: PRO_APB_ADC_INT_MAP,
    #[doc = "0x168 - CRYPTO_DMA_INT interrupt configuration register"]
    pub pro_crypto_dma_int_map: PRO_CRYPTO_DMA_INT_MAP,
    #[doc = "0x16c - CPU_PERI_ERROR_INT interrupt configuration register"]
    pub pro_cpu_peri_error_int_map: PRO_CPU_PERI_ERROR_INT_MAP,
    #[doc = "0x170 - APB_PERI_ERROR_INT interrupt configuration register"]
    pub pro_apb_peri_error_int_map: PRO_APB_PERI_ERROR_INT_MAP,
    #[doc = "0x174 - DCACHE_SYNC_INT interrupt configuration register"]
    pub pro_dcache_sync_int_map: PRO_DCACHE_SYNC_INT_MAP,
    #[doc = "0x178 - ICACHE_SYNC_INT interrupt configuration register"]
    pub pro_icache_sync_int_map: PRO_ICACHE_SYNC_INT_MAP,
    #[doc = "0x17c - Interrupt status register 0"]
    pub pro_intr_status_0: PRO_INTR_STATUS_0,
    #[doc = "0x180 - Interrupt status register 1"]
    pub pro_intr_status_1: PRO_INTR_STATUS_1,
    #[doc = "0x184 - Interrupt status register 2"]
    pub pro_intr_status_2: PRO_INTR_STATUS_2,
    #[doc = "0x188 - NMI interrupt signals mask register"]
    pub clock_gate: CLOCK_GATE,
    _reserved99: [u8; 0x0e70],
    #[doc = "0xffc - Version control register"]
    pub reg_date: REG_DATE,
}
#[doc = "PRO_MAC_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_MAC_INTR_MAP_SPEC>`"]
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
#[doc = "MAC_INTR interrupt configuration register"]
pub mod pro_mac_intr_map;
#[doc = "PRO_MAC_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_MAC_NMI_MAP_SPEC>`"]
pub type PRO_MAC_NMI_MAP = crate::Reg<pro_mac_nmi_map::PRO_MAC_NMI_MAP_SPEC>;
#[doc = "MAC_NMI interrupt configuration register"]
pub mod pro_mac_nmi_map;
#[doc = "PRO_PWR_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWR_INTR_MAP_SPEC>`"]
pub type PRO_PWR_INTR_MAP = crate::Reg<pro_pwr_intr_map::PRO_PWR_INTR_MAP_SPEC>;
#[doc = "PWR_INTR interrupt configuration register"]
pub mod pro_pwr_intr_map;
#[doc = "PRO_BB_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BB_INT_MAP_SPEC>`"]
pub type PRO_BB_INT_MAP = crate::Reg<pro_bb_int_map::PRO_BB_INT_MAP_SPEC>;
#[doc = "BB_INT interrupt configuration register"]
pub mod pro_bb_int_map;
#[doc = "PRO_BT_MAC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BT_MAC_INT_MAP_SPEC>`"]
pub type PRO_BT_MAC_INT_MAP = crate::Reg<pro_bt_mac_int_map::PRO_BT_MAC_INT_MAP_SPEC>;
#[doc = "BT_MAC_INT interrupt configuration register"]
pub mod pro_bt_mac_int_map;
#[doc = "PRO_BT_BB_INT_MAP (rw) register accessor: an alias for `Reg<PRO_BT_BB_INT_MAP_SPEC>`"]
pub type PRO_BT_BB_INT_MAP = crate::Reg<pro_bt_bb_int_map::PRO_BT_BB_INT_MAP_SPEC>;
#[doc = "BT_BB_INT interrupt configuration register"]
pub mod pro_bt_bb_int_map;
#[doc = "PRO_BT_BB_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_BT_BB_NMI_MAP_SPEC>`"]
pub type PRO_BT_BB_NMI_MAP = crate::Reg<pro_bt_bb_nmi_map::PRO_BT_BB_NMI_MAP_SPEC>;
#[doc = "BT_BB_NMI interrupt configuration register"]
pub mod pro_bt_bb_nmi_map;
#[doc = "PRO_RWBT_IRQ_MAP (rw) register accessor: an alias for `Reg<PRO_RWBT_IRQ_MAP_SPEC>`"]
pub type PRO_RWBT_IRQ_MAP = crate::Reg<pro_rwbt_irq_map::PRO_RWBT_IRQ_MAP_SPEC>;
#[doc = "RWBT_IRQ interrupt configuration register"]
pub mod pro_rwbt_irq_map;
#[doc = "PRO_RWBLE_IRQ_MAP (rw) register accessor: an alias for `Reg<PRO_RWBLE_IRQ_MAP_SPEC>`"]
pub type PRO_RWBLE_IRQ_MAP = crate::Reg<pro_rwble_irq_map::PRO_RWBLE_IRQ_MAP_SPEC>;
#[doc = "RWBLE_IRQ interrupt configuration register"]
pub mod pro_rwble_irq_map;
#[doc = "PRO_RWBT_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_RWBT_NMI_MAP_SPEC>`"]
pub type PRO_RWBT_NMI_MAP = crate::Reg<pro_rwbt_nmi_map::PRO_RWBT_NMI_MAP_SPEC>;
#[doc = "RWBT_NMI interrupt configuration register"]
pub mod pro_rwbt_nmi_map;
#[doc = "PRO_RWBLE_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_RWBLE_NMI_MAP_SPEC>`"]
pub type PRO_RWBLE_NMI_MAP = crate::Reg<pro_rwble_nmi_map::PRO_RWBLE_NMI_MAP_SPEC>;
#[doc = "RWBLE_NMI interrupt configuration register"]
pub mod pro_rwble_nmi_map;
#[doc = "PRO_SLC0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SLC0_INTR_MAP_SPEC>`"]
pub type PRO_SLC0_INTR_MAP = crate::Reg<pro_slc0_intr_map::PRO_SLC0_INTR_MAP_SPEC>;
#[doc = "SLC0_INTR interrupt configuration register"]
pub mod pro_slc0_intr_map;
#[doc = "PRO_SLC1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SLC1_INTR_MAP_SPEC>`"]
pub type PRO_SLC1_INTR_MAP = crate::Reg<pro_slc1_intr_map::PRO_SLC1_INTR_MAP_SPEC>;
#[doc = "SLC1_INTR interrupt configuration register"]
pub mod pro_slc1_intr_map;
#[doc = "PRO_UHCI0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UHCI0_INTR_MAP_SPEC>`"]
pub type PRO_UHCI0_INTR_MAP = crate::Reg<pro_uhci0_intr_map::PRO_UHCI0_INTR_MAP_SPEC>;
#[doc = "UHCI0_INTR interrupt configuration register"]
pub mod pro_uhci0_intr_map;
#[doc = "PRO_UHCI1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UHCI1_INTR_MAP_SPEC>`"]
pub type PRO_UHCI1_INTR_MAP = crate::Reg<pro_uhci1_intr_map::PRO_UHCI1_INTR_MAP_SPEC>;
#[doc = "UHCI1_INTR interrupt configuration register"]
pub mod pro_uhci1_intr_map;
#[doc = "PRO_TG_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T0_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t0_level_int_map::PRO_TG_T0_LEVEL_INT_MAP_SPEC>;
#[doc = "TG_T0_LEVEL_INT interrupt configuration register"]
pub mod pro_tg_t0_level_int_map;
#[doc = "PRO_TG_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T1_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t1_level_int_map::PRO_TG_T1_LEVEL_INT_MAP_SPEC>;
#[doc = "TG_T1_LEVEL_INT interrupt configuration register"]
pub mod pro_tg_t1_level_int_map;
#[doc = "PRO_TG_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_wdt_level_int_map::PRO_TG_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = "TG_WDT_LEVEL_INT interrupt configuration register"]
pub mod pro_tg_wdt_level_int_map;
#[doc = "PRO_TG_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_lact_level_int_map::PRO_TG_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = "TG_LACT_LEVEL_INT interrupt configuration register"]
pub mod pro_tg_lact_level_int_map;
#[doc = "PRO_TG1_T0_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T0_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t0_level_int_map::PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
#[doc = "TG1_T0_LEVEL_INT interrupt configuration register"]
pub mod pro_tg1_t0_level_int_map;
#[doc = "PRO_TG1_T1_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T1_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t1_level_int_map::PRO_TG1_T1_LEVEL_INT_MAP_SPEC>;
#[doc = "TG1_T1_LEVEL_INT interrupt configuration register"]
pub mod pro_tg1_t1_level_int_map;
#[doc = "PRO_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_wdt_level_int_map::PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
#[doc = "TG1_WDT_LEVEL_INT interrupt configuration register"]
pub mod pro_tg1_wdt_level_int_map;
#[doc = "PRO_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>`"]
pub type PRO_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_lact_level_int_map::PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>;
#[doc = "TG1_LACT_LEVEL_INT interrupt configuration register"]
pub mod pro_tg1_lact_level_int_map;
#[doc = "PRO_GPIO_INTERRUPT_PRO_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_PRO_MAP =
    crate::Reg<pro_gpio_interrupt_pro_map::PRO_GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_PRO interrupt configuration register"]
pub mod pro_gpio_interrupt_pro_map;
#[doc = "PRO_GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_pro_nmi_map::PRO_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_PRO_NMI interrupt configuration register"]
pub mod pro_gpio_interrupt_pro_nmi_map;
#[doc = "PRO_GPIO_INTERRUPT_APP_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_APP_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_APP_MAP =
    crate::Reg<pro_gpio_interrupt_app_map::PRO_GPIO_INTERRUPT_APP_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_APP interrupt configuration register"]
pub mod pro_gpio_interrupt_app_map;
#[doc = "PRO_GPIO_INTERRUPT_APP_NMI_MAP (rw) register accessor: an alias for `Reg<PRO_GPIO_INTERRUPT_APP_NMI_MAP_SPEC>`"]
pub type PRO_GPIO_INTERRUPT_APP_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_app_nmi_map::PRO_GPIO_INTERRUPT_APP_NMI_MAP_SPEC>;
#[doc = "GPIO_INTERRUPT_APP_NMI interrupt configuration register"]
pub mod pro_gpio_interrupt_app_nmi_map;
#[doc = "PRO_DEDICATED_GPIO_IN_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_DEDICATED_GPIO_IN_INTR_MAP_SPEC>`"]
pub type PRO_DEDICATED_GPIO_IN_INTR_MAP =
    crate::Reg<pro_dedicated_gpio_in_intr_map::PRO_DEDICATED_GPIO_IN_INTR_MAP_SPEC>;
#[doc = "DEDICATED_GPIO_IN_INTR interrupt configuration register"]
pub mod pro_dedicated_gpio_in_intr_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_0_map::PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_0 interrupt configuration register"]
pub mod pro_cpu_intr_from_cpu_0_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_1_map::PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_1 interrupt configuration register"]
pub mod pro_cpu_intr_from_cpu_1_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_2_map::PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_2 interrupt configuration register"]
pub mod pro_cpu_intr_from_cpu_2_map;
#[doc = "PRO_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type PRO_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_3_map::PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "CPU_INTR_FROM_CPU_3 interrupt configuration register"]
pub mod pro_cpu_intr_from_cpu_3_map;
#[doc = "PRO_SPI_INTR_1_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_1_MAP_SPEC>`"]
pub type PRO_SPI_INTR_1_MAP = crate::Reg<pro_spi_intr_1_map::PRO_SPI_INTR_1_MAP_SPEC>;
#[doc = "SPI_INTR_1 interrupt configuration register"]
pub mod pro_spi_intr_1_map;
#[doc = "PRO_SPI_INTR_2_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_2_MAP_SPEC>`"]
pub type PRO_SPI_INTR_2_MAP = crate::Reg<pro_spi_intr_2_map::PRO_SPI_INTR_2_MAP_SPEC>;
#[doc = "SPI_INTR_2 interrupt configuration register"]
pub mod pro_spi_intr_2_map;
#[doc = "PRO_SPI_INTR_3_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_3_MAP_SPEC>`"]
pub type PRO_SPI_INTR_3_MAP = crate::Reg<pro_spi_intr_3_map::PRO_SPI_INTR_3_MAP_SPEC>;
#[doc = "SPI_INTR_3 interrupt configuration register"]
pub mod pro_spi_intr_3_map;
#[doc = "PRO_I2S0_INT_MAP (rw) register accessor: an alias for `Reg<PRO_I2S0_INT_MAP_SPEC>`"]
pub type PRO_I2S0_INT_MAP = crate::Reg<pro_i2s0_int_map::PRO_I2S0_INT_MAP_SPEC>;
#[doc = "I2S0_INT interrupt configuration register"]
pub mod pro_i2s0_int_map;
#[doc = "PRO_I2S1_INT_MAP (rw) register accessor: an alias for `Reg<PRO_I2S1_INT_MAP_SPEC>`"]
pub type PRO_I2S1_INT_MAP = crate::Reg<pro_i2s1_int_map::PRO_I2S1_INT_MAP_SPEC>;
#[doc = "I2S1_INT interrupt configuration register"]
pub mod pro_i2s1_int_map;
#[doc = "PRO_UART_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART_INTR_MAP_SPEC>`"]
pub type PRO_UART_INTR_MAP = crate::Reg<pro_uart_intr_map::PRO_UART_INTR_MAP_SPEC>;
#[doc = "UART_INT interrupt configuration register"]
pub mod pro_uart_intr_map;
#[doc = "PRO_UART1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART1_INTR_MAP_SPEC>`"]
pub type PRO_UART1_INTR_MAP = crate::Reg<pro_uart1_intr_map::PRO_UART1_INTR_MAP_SPEC>;
#[doc = "UART1_INT interrupt configuration register"]
pub mod pro_uart1_intr_map;
#[doc = "PRO_UART2_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_UART2_INTR_MAP_SPEC>`"]
pub type PRO_UART2_INTR_MAP = crate::Reg<pro_uart2_intr_map::PRO_UART2_INTR_MAP_SPEC>;
#[doc = "UART2_INT interrupt configuration register"]
pub mod pro_uart2_intr_map;
#[doc = "PRO_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: an alias for `Reg<PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>`"]
pub type PRO_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<pro_sdio_host_interrupt_map::PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "SDIO_HOST_INTERRUPT configuration register"]
pub mod pro_sdio_host_interrupt_map;
#[doc = "PRO_PWM0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM0_INTR_MAP_SPEC>`"]
pub type PRO_PWM0_INTR_MAP = crate::Reg<pro_pwm0_intr_map::PRO_PWM0_INTR_MAP_SPEC>;
#[doc = "PWM0_INTR interrupt configuration register"]
pub mod pro_pwm0_intr_map;
#[doc = "PRO_PWM1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM1_INTR_MAP_SPEC>`"]
pub type PRO_PWM1_INTR_MAP = crate::Reg<pro_pwm1_intr_map::PRO_PWM1_INTR_MAP_SPEC>;
#[doc = "PWM1_INTR interrupt configuration register"]
pub mod pro_pwm1_intr_map;
#[doc = "PRO_PWM2_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM2_INTR_MAP_SPEC>`"]
pub type PRO_PWM2_INTR_MAP = crate::Reg<pro_pwm2_intr_map::PRO_PWM2_INTR_MAP_SPEC>;
#[doc = "PWM2_INTR interrupt configuration register"]
pub mod pro_pwm2_intr_map;
#[doc = "PRO_PWM3_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PWM3_INTR_MAP_SPEC>`"]
pub type PRO_PWM3_INTR_MAP = crate::Reg<pro_pwm3_intr_map::PRO_PWM3_INTR_MAP_SPEC>;
#[doc = "PWM3_INTR interrupt configuration register"]
pub mod pro_pwm3_intr_map;
#[doc = "PRO_LEDC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_LEDC_INT_MAP_SPEC>`"]
pub type PRO_LEDC_INT_MAP = crate::Reg<pro_ledc_int_map::PRO_LEDC_INT_MAP_SPEC>;
#[doc = "LEDC_INTR interrupt configuration register"]
pub mod pro_ledc_int_map;
#[doc = "PRO_EFUSE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_EFUSE_INT_MAP_SPEC>`"]
pub type PRO_EFUSE_INT_MAP = crate::Reg<pro_efuse_int_map::PRO_EFUSE_INT_MAP_SPEC>;
#[doc = "EFUSE_INT interrupt configuration register"]
pub mod pro_efuse_int_map;
#[doc = "PRO_CAN_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CAN_INT_MAP_SPEC>`"]
pub type PRO_CAN_INT_MAP = crate::Reg<pro_can_int_map::PRO_CAN_INT_MAP_SPEC>;
#[doc = "CAN_INT interrupt configuration register"]
pub mod pro_can_int_map;
#[doc = "PRO_USB_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_USB_INTR_MAP_SPEC>`"]
pub type PRO_USB_INTR_MAP = crate::Reg<pro_usb_intr_map::PRO_USB_INTR_MAP_SPEC>;
#[doc = "USB_INT interrupt configuration register"]
pub mod pro_usb_intr_map;
#[doc = "PRO_RTC_CORE_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RTC_CORE_INTR_MAP_SPEC>`"]
pub type PRO_RTC_CORE_INTR_MAP = crate::Reg<pro_rtc_core_intr_map::PRO_RTC_CORE_INTR_MAP_SPEC>;
#[doc = "RTC_CORE_INTR interrupt configuration register"]
pub mod pro_rtc_core_intr_map;
#[doc = "PRO_RMT_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RMT_INTR_MAP_SPEC>`"]
pub type PRO_RMT_INTR_MAP = crate::Reg<pro_rmt_intr_map::PRO_RMT_INTR_MAP_SPEC>;
#[doc = "RMT_INTR interrupt configuration register"]
pub mod pro_rmt_intr_map;
#[doc = "PRO_PCNT_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PCNT_INTR_MAP_SPEC>`"]
pub type PRO_PCNT_INTR_MAP = crate::Reg<pro_pcnt_intr_map::PRO_PCNT_INTR_MAP_SPEC>;
#[doc = "PCNT_INTR interrupt configuration register"]
pub mod pro_pcnt_intr_map;
#[doc = "PRO_I2C_EXT0_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_I2C_EXT0_INTR_MAP_SPEC>`"]
pub type PRO_I2C_EXT0_INTR_MAP = crate::Reg<pro_i2c_ext0_intr_map::PRO_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "I2C_EXT0_INTR interrupt configuration register"]
pub mod pro_i2c_ext0_intr_map;
#[doc = "PRO_I2C_EXT1_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_I2C_EXT1_INTR_MAP_SPEC>`"]
pub type PRO_I2C_EXT1_INTR_MAP = crate::Reg<pro_i2c_ext1_intr_map::PRO_I2C_EXT1_INTR_MAP_SPEC>;
#[doc = "I2C_EXT1_INTR interrupt configuration register"]
pub mod pro_i2c_ext1_intr_map;
#[doc = "PRO_RSA_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_RSA_INTR_MAP_SPEC>`"]
pub type PRO_RSA_INTR_MAP = crate::Reg<pro_rsa_intr_map::PRO_RSA_INTR_MAP_SPEC>;
#[doc = "RSA_INTR interrupt configuration register"]
pub mod pro_rsa_intr_map;
#[doc = "PRO_SHA_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SHA_INTR_MAP_SPEC>`"]
pub type PRO_SHA_INTR_MAP = crate::Reg<pro_sha_intr_map::PRO_SHA_INTR_MAP_SPEC>;
#[doc = "SHA_INTR interrupt configuration register"]
pub mod pro_sha_intr_map;
#[doc = "PRO_AES_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_AES_INTR_MAP_SPEC>`"]
pub type PRO_AES_INTR_MAP = crate::Reg<pro_aes_intr_map::PRO_AES_INTR_MAP_SPEC>;
#[doc = "AES_INTR interrupt configuration register"]
pub mod pro_aes_intr_map;
#[doc = "PRO_SPI2_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI2_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI2_DMA_INT_MAP = crate::Reg<pro_spi2_dma_int_map::PRO_SPI2_DMA_INT_MAP_SPEC>;
#[doc = "SPI2_DMA_INT interrupt configuration register"]
pub mod pro_spi2_dma_int_map;
#[doc = "PRO_SPI3_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI3_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI3_DMA_INT_MAP = crate::Reg<pro_spi3_dma_int_map::PRO_SPI3_DMA_INT_MAP_SPEC>;
#[doc = "SPI3_DMA_INT interrupt configuration register"]
pub mod pro_spi3_dma_int_map;
#[doc = "PRO_WDG_INT_MAP (rw) register accessor: an alias for `Reg<PRO_WDG_INT_MAP_SPEC>`"]
pub type PRO_WDG_INT_MAP = crate::Reg<pro_wdg_int_map::PRO_WDG_INT_MAP_SPEC>;
#[doc = "WDG_INT interrupt configuration register"]
pub mod pro_wdg_int_map;
#[doc = "PRO_TIMER_INT1_MAP (rw) register accessor: an alias for `Reg<PRO_TIMER_INT1_MAP_SPEC>`"]
pub type PRO_TIMER_INT1_MAP = crate::Reg<pro_timer_int1_map::PRO_TIMER_INT1_MAP_SPEC>;
#[doc = "TIMER_INT1 interrupt configuration register"]
pub mod pro_timer_int1_map;
#[doc = "PRO_TIMER_INT2_MAP (rw) register accessor: an alias for `Reg<PRO_TIMER_INT2_MAP_SPEC>`"]
pub type PRO_TIMER_INT2_MAP = crate::Reg<pro_timer_int2_map::PRO_TIMER_INT2_MAP_SPEC>;
#[doc = "TIMER_INT2 interrupt configuration register"]
pub mod pro_timer_int2_map;
#[doc = "PRO_TG_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T0_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_T0_EDGE_INT_MAP = crate::Reg<pro_tg_t0_edge_int_map::PRO_TG_T0_EDGE_INT_MAP_SPEC>;
#[doc = "TG_T0_EDGE_INT interrupt configuration register"]
pub mod pro_tg_t0_edge_int_map;
#[doc = "PRO_TG_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_T1_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_T1_EDGE_INT_MAP = crate::Reg<pro_tg_t1_edge_int_map::PRO_TG_T1_EDGE_INT_MAP_SPEC>;
#[doc = "TG_T1_EDGE_INT interrupt configuration register"]
pub mod pro_tg_t1_edge_int_map;
#[doc = "PRO_TG_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_WDT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg_wdt_edge_int_map::PRO_TG_WDT_EDGE_INT_MAP_SPEC>;
#[doc = "TG_WDT_EDGE_INT interrupt configuration register"]
pub mod pro_tg_wdt_edge_int_map;
#[doc = "PRO_TG_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG_LACT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg_lact_edge_int_map::PRO_TG_LACT_EDGE_INT_MAP_SPEC>;
#[doc = "TG_LACT_EDGE_INT interrupt configuration register"]
pub mod pro_tg_lact_edge_int_map;
#[doc = "PRO_TG1_T0_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T0_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_T0_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t0_edge_int_map::PRO_TG1_T0_EDGE_INT_MAP_SPEC>;
#[doc = "TG1_T0_EDGE_INT interrupt configuration register"]
pub mod pro_tg1_t0_edge_int_map;
#[doc = "PRO_TG1_T1_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_T1_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_T1_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t1_edge_int_map::PRO_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = "TG1_T1_EDGE_INT interrupt configuration register"]
pub mod pro_tg1_t1_edge_int_map;
#[doc = "PRO_TG1_WDT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_WDT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_wdt_edge_int_map::PRO_TG1_WDT_EDGE_INT_MAP_SPEC>;
#[doc = "TG1_WDT_EDGE_INT interrupt configuration register"]
pub mod pro_tg1_wdt_edge_int_map;
#[doc = "PRO_TG1_LACT_EDGE_INT_MAP (rw) register accessor: an alias for `Reg<PRO_TG1_LACT_EDGE_INT_MAP_SPEC>`"]
pub type PRO_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_lact_edge_int_map::PRO_TG1_LACT_EDGE_INT_MAP_SPEC>;
#[doc = "TG1_LACT_EDGE_INT interrupt configuration register"]
pub mod pro_tg1_lact_edge_int_map;
#[doc = "PRO_CACHE_IA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CACHE_IA_INT_MAP_SPEC>`"]
pub type PRO_CACHE_IA_INT_MAP = crate::Reg<pro_cache_ia_int_map::PRO_CACHE_IA_INT_MAP_SPEC>;
#[doc = "CACHE_IA_INT interrupt configuration register"]
pub mod pro_cache_ia_int_map;
#[doc = "PRO_SYSTIMER_TARGET0_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type PRO_SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<pro_systimer_target0_int_map::PRO_SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET0_INT interrupt configuration register"]
pub mod pro_systimer_target0_int_map;
#[doc = "PRO_SYSTIMER_TARGET1_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type PRO_SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<pro_systimer_target1_int_map::PRO_SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET1_INT interrupt configuration register"]
pub mod pro_systimer_target1_int_map;
#[doc = "PRO_SYSTIMER_TARGET2_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type PRO_SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<pro_systimer_target2_int_map::PRO_SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "SYSTIMER_TARGET2_INT interrupt configuration register"]
pub mod pro_systimer_target2_int_map;
#[doc = "PRO_ASSIST_DEBUG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type PRO_ASSIST_DEBUG_INTR_MAP =
    crate::Reg<pro_assist_debug_intr_map::PRO_ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "ASSIST_DEBUG_INTR interrupt configuration register"]
pub mod pro_assist_debug_intr_map;
#[doc = "PRO_PMS_PRO_IRAM0_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_PRO_IRAM0_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_PRO_IRAM0_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_iram0_ilg_intr_map::PRO_PMS_PRO_IRAM0_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_PRO_IRAM0_ILG interrupt configuration register"]
pub mod pro_pms_pro_iram0_ilg_intr_map;
#[doc = "PRO_PMS_PRO_DRAM0_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_PRO_DRAM0_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_PRO_DRAM0_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_dram0_ilg_intr_map::PRO_PMS_PRO_DRAM0_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_PRO_DRAM0_ILG interrupt configuration register"]
pub mod pro_pms_pro_dram0_ilg_intr_map;
#[doc = "PRO_PMS_PRO_DPORT_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_PRO_DPORT_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_PRO_DPORT_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_dport_ilg_intr_map::PRO_PMS_PRO_DPORT_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_PRO_DPORT_ILG interrupt configuration register"]
pub mod pro_pms_pro_dport_ilg_intr_map;
#[doc = "PRO_PMS_PRO_AHB_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_PRO_AHB_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_PRO_AHB_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_ahb_ilg_intr_map::PRO_PMS_PRO_AHB_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_PRO_AHB_ILG interrupt configuration register"]
pub mod pro_pms_pro_ahb_ilg_intr_map;
#[doc = "PRO_PMS_PRO_CACHE_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_PRO_CACHE_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_PRO_CACHE_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_cache_ilg_intr_map::PRO_PMS_PRO_CACHE_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_PRO_CACHE_ILG interrupt configuration register"]
pub mod pro_pms_pro_cache_ilg_intr_map;
#[doc = "PRO_PMS_DMA_APB_I_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_DMA_APB_I_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_DMA_APB_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_apb_i_ilg_intr_map::PRO_PMS_DMA_APB_I_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_DMA_APB_I_ILG interrupt configuration register"]
pub mod pro_pms_dma_apb_i_ilg_intr_map;
#[doc = "PRO_PMS_DMA_RX_I_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_DMA_RX_I_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_DMA_RX_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_rx_i_ilg_intr_map::PRO_PMS_DMA_RX_I_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_DMA_RX_I_ILG interrupt configuration register"]
pub mod pro_pms_dma_rx_i_ilg_intr_map;
#[doc = "PRO_PMS_DMA_TX_I_ILG_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC>`"]
pub type PRO_PMS_DMA_TX_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_tx_i_ilg_intr_map::PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC>;
#[doc = "PMS_DMA_TX_I_ILG interrupt configuration register"]
pub mod pro_pms_dma_tx_i_ilg_intr_map;
#[doc = "PRO_SPI_MEM_REJECT_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type PRO_SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<pro_spi_mem_reject_intr_map::PRO_SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "SPI_MEM_REJECT_INTR interrupt configuration register"]
pub mod pro_spi_mem_reject_intr_map;
#[doc = "PRO_DMA_COPY_INTR_MAP (rw) register accessor: an alias for `Reg<PRO_DMA_COPY_INTR_MAP_SPEC>`"]
pub type PRO_DMA_COPY_INTR_MAP = crate::Reg<pro_dma_copy_intr_map::PRO_DMA_COPY_INTR_MAP_SPEC>;
#[doc = "DMA_COPY_INTR interrupt configuration register"]
pub mod pro_dma_copy_intr_map;
#[doc = "PRO_SPI4_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_SPI4_DMA_INT_MAP_SPEC>`"]
pub type PRO_SPI4_DMA_INT_MAP = crate::Reg<pro_spi4_dma_int_map::PRO_SPI4_DMA_INT_MAP_SPEC>;
#[doc = "SPI4_DMA_INT interrupt configuration register"]
pub mod pro_spi4_dma_int_map;
#[doc = "PRO_SPI_INTR_4_MAP (rw) register accessor: an alias for `Reg<PRO_SPI_INTR_4_MAP_SPEC>`"]
pub type PRO_SPI_INTR_4_MAP = crate::Reg<pro_spi_intr_4_map::PRO_SPI_INTR_4_MAP_SPEC>;
#[doc = "SPI_INTR_4 interrupt configuration register"]
pub mod pro_spi_intr_4_map;
#[doc = "PRO_DCACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<PRO_DCACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type PRO_DCACHE_PRELOAD_INT_MAP =
    crate::Reg<pro_dcache_preload_int_map::PRO_DCACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "DCACHE_PRELOAD_INT interrupt configuration register"]
pub mod pro_dcache_preload_int_map;
#[doc = "PRO_ICACHE_PRELOAD_INT_MAP (rw) register accessor: an alias for `Reg<PRO_ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type PRO_ICACHE_PRELOAD_INT_MAP =
    crate::Reg<pro_icache_preload_int_map::PRO_ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "ICACHE_PRELOAD_INT interrupt configuration register"]
pub mod pro_icache_preload_int_map;
#[doc = "PRO_APB_ADC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_APB_ADC_INT_MAP_SPEC>`"]
pub type PRO_APB_ADC_INT_MAP = crate::Reg<pro_apb_adc_int_map::PRO_APB_ADC_INT_MAP_SPEC>;
#[doc = "APB_ADC_INT interrupt configuration register"]
pub mod pro_apb_adc_int_map;
#[doc = "PRO_CRYPTO_DMA_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CRYPTO_DMA_INT_MAP_SPEC>`"]
pub type PRO_CRYPTO_DMA_INT_MAP = crate::Reg<pro_crypto_dma_int_map::PRO_CRYPTO_DMA_INT_MAP_SPEC>;
#[doc = "CRYPTO_DMA_INT interrupt configuration register"]
pub mod pro_crypto_dma_int_map;
#[doc = "PRO_CPU_PERI_ERROR_INT_MAP (rw) register accessor: an alias for `Reg<PRO_CPU_PERI_ERROR_INT_MAP_SPEC>`"]
pub type PRO_CPU_PERI_ERROR_INT_MAP =
    crate::Reg<pro_cpu_peri_error_int_map::PRO_CPU_PERI_ERROR_INT_MAP_SPEC>;
#[doc = "CPU_PERI_ERROR_INT interrupt configuration register"]
pub mod pro_cpu_peri_error_int_map;
#[doc = "PRO_APB_PERI_ERROR_INT_MAP (rw) register accessor: an alias for `Reg<PRO_APB_PERI_ERROR_INT_MAP_SPEC>`"]
pub type PRO_APB_PERI_ERROR_INT_MAP =
    crate::Reg<pro_apb_peri_error_int_map::PRO_APB_PERI_ERROR_INT_MAP_SPEC>;
#[doc = "APB_PERI_ERROR_INT interrupt configuration register"]
pub mod pro_apb_peri_error_int_map;
#[doc = "PRO_DCACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_DCACHE_SYNC_INT_MAP_SPEC>`"]
pub type PRO_DCACHE_SYNC_INT_MAP =
    crate::Reg<pro_dcache_sync_int_map::PRO_DCACHE_SYNC_INT_MAP_SPEC>;
#[doc = "DCACHE_SYNC_INT interrupt configuration register"]
pub mod pro_dcache_sync_int_map;
#[doc = "PRO_ICACHE_SYNC_INT_MAP (rw) register accessor: an alias for `Reg<PRO_ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type PRO_ICACHE_SYNC_INT_MAP =
    crate::Reg<pro_icache_sync_int_map::PRO_ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "ICACHE_SYNC_INT interrupt configuration register"]
pub mod pro_icache_sync_int_map;
#[doc = "PRO_INTR_STATUS_0 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_0_SPEC>`"]
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
#[doc = "Interrupt status register 0"]
pub mod pro_intr_status_0;
#[doc = "PRO_INTR_STATUS_1 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_1_SPEC>`"]
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
#[doc = "Interrupt status register 1"]
pub mod pro_intr_status_1;
#[doc = "PRO_INTR_STATUS_2 (r) register accessor: an alias for `Reg<PRO_INTR_STATUS_2_SPEC>`"]
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
#[doc = "Interrupt status register 2"]
pub mod pro_intr_status_2;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "NMI interrupt signals mask register"]
pub mod clock_gate;
#[doc = "REG_DATE (rw) register accessor: an alias for `Reg<REG_DATE_SPEC>`"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "Version control register"]
pub mod reg_date;
