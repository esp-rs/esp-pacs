#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - mac interrupt configuration register"]
    pub app_mac_intr_map: APP_MAC_INTR_MAP,
    #[doc = "0x804 - mac_nmi interrupt configuration register"]
    pub mac_nmi_map: MAC_NMI_MAP,
    #[doc = "0x808 - pwr interrupt configuration register"]
    pub pwr_intr_map: PWR_INTR_MAP,
    #[doc = "0x80c - bb interrupt configuration register"]
    pub bb_int_map: BB_INT_MAP,
    #[doc = "0x810 - bb_mac interrupt configuration register"]
    pub bt_mac_int_map: BT_MAC_INT_MAP,
    #[doc = "0x814 - bt_bb interrupt configuration register"]
    pub bt_bb_int_map: BT_BB_INT_MAP,
    #[doc = "0x818 - bt_bb_nmi interrupt configuration register"]
    pub bt_bb_nmi_map: BT_BB_NMI_MAP,
    #[doc = "0x81c - rwbt_irq interrupt configuration register"]
    pub rwbt_irq_map: RWBT_IRQ_MAP,
    #[doc = "0x820 - rwble_irq interrupt configuration register"]
    pub rwble_irq_map: RWBLE_IRQ_MAP,
    #[doc = "0x824 - rwbt_nmi interrupt configuration register"]
    pub rwbt_nmi_map: RWBT_NMI_MAP,
    #[doc = "0x828 - rwble_nmi interrupt configuration register"]
    pub rwble_nmi_map: RWBLE_NMI_MAP,
    #[doc = "0x82c - i2c_mst interrupt configuration register"]
    pub i2c_mst_int_map: I2C_MST_INT_MAP,
    #[doc = "0x830 - slc0 interrupt configuration register"]
    pub slc0_intr_map: SLC0_INTR_MAP,
    #[doc = "0x834 - slc1 interrupt configuration register"]
    pub slc1_intr_map: SLC1_INTR_MAP,
    #[doc = "0x838 - uhci0 interrupt configuration register"]
    pub uhci0_intr_map: UHCI0_INTR_MAP,
    #[doc = "0x83c - uhci1 interrupt configuration register"]
    pub uhci1_intr_map: UHCI1_INTR_MAP,
    #[doc = "0x840 - gpio_interrupt_pro interrupt configuration register"]
    pub gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    #[doc = "0x844 - gpio_interrupt_pro_nmi interrupt configuration register"]
    pub gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    #[doc = "0x848 - gpio_interrupt_app interrupt configuration register"]
    pub gpio_interrupt_app_map: GPIO_INTERRUPT_APP_MAP,
    #[doc = "0x84c - gpio_interrupt_app_nmi interrupt configuration register"]
    pub gpio_interrupt_app_nmi_map: GPIO_INTERRUPT_APP_NMI_MAP,
    #[doc = "0x850 - spi_intr_1 interrupt configuration register"]
    pub spi_intr_1_map: SPI_INTR_1_MAP,
    #[doc = "0x854 - spi_intr_2 interrupt configuration register"]
    pub spi_intr_2_map: SPI_INTR_2_MAP,
    #[doc = "0x858 - spi_intr_3 interrupt configuration register"]
    pub spi_intr_3_map: SPI_INTR_3_MAP,
    #[doc = "0x85c - spi_intr_4 interrupt configuration register"]
    pub spi_intr_4_map: SPI_INTR_4_MAP,
    #[doc = "0x860 - lcd_cam interrupt configuration register"]
    pub lcd_cam_int_map: LCD_CAM_INT_MAP,
    #[doc = "0x864 - i2s0 interrupt configuration register"]
    pub i2s0_int_map: I2S0_INT_MAP,
    #[doc = "0x868 - i2s1 interrupt configuration register"]
    pub i2s1_int_map: I2S1_INT_MAP,
    #[doc = "0x86c - uart interrupt configuration register"]
    pub uart_intr_map: UART_INTR_MAP,
    #[doc = "0x870 - uart1 interrupt configuration register"]
    pub uart1_intr_map: UART1_INTR_MAP,
    #[doc = "0x874 - uart2 interrupt configuration register"]
    pub uart2_intr_map: UART2_INTR_MAP,
    #[doc = "0x878 - sdio_host interrupt configuration register"]
    pub sdio_host_interrupt_map: SDIO_HOST_INTERRUPT_MAP,
    #[doc = "0x87c - pwm0 interrupt configuration register"]
    pub pwm0_intr_map: PWM0_INTR_MAP,
    #[doc = "0x880 - pwm1 interrupt configuration register"]
    pub pwm1_intr_map: PWM1_INTR_MAP,
    #[doc = "0x884 - pwm2 interrupt configuration register"]
    pub pwm2_intr_map: PWM2_INTR_MAP,
    #[doc = "0x888 - pwm3 interrupt configuration register"]
    pub pwm3_intr_map: PWM3_INTR_MAP,
    #[doc = "0x88c - ledc interrupt configuration register"]
    pub ledc_int_map: LEDC_INT_MAP,
    #[doc = "0x890 - efuse interrupt configuration register"]
    pub efuse_int_map: EFUSE_INT_MAP,
    #[doc = "0x894 - can interrupt configuration register"]
    pub can_int_map: CAN_INT_MAP,
    #[doc = "0x898 - usb interrupt configuration register"]
    pub usb_intr_map: USB_INTR_MAP,
    #[doc = "0x89c - rtc_core interrupt configuration register"]
    pub rtc_core_intr_map: RTC_CORE_INTR_MAP,
    #[doc = "0x8a0 - rmt interrupt configuration register"]
    pub rmt_intr_map: RMT_INTR_MAP,
    #[doc = "0x8a4 - pcnt interrupt configuration register"]
    pub pcnt_intr_map: PCNT_INTR_MAP,
    #[doc = "0x8a8 - i2c_ext0 interrupt configuration register"]
    pub i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    #[doc = "0x8ac - i2c_ext1 interrupt configuration register"]
    pub i2c_ext1_intr_map: I2C_EXT1_INTR_MAP,
    #[doc = "0x8b0 - spi2_dma interrupt configuration register"]
    pub spi2_dma_int_map: SPI2_DMA_INT_MAP,
    #[doc = "0x8b4 - spi3_dma interrupt configuration register"]
    pub spi3_dma_int_map: SPI3_DMA_INT_MAP,
    #[doc = "0x8b8 - spi4_dma interrupt configuration register"]
    pub spi4_dma_int_map: SPI4_DMA_INT_MAP,
    #[doc = "0x8bc - wdg interrupt configuration register"]
    pub wdg_int_map: WDG_INT_MAP,
    #[doc = "0x8c0 - timer_int1 interrupt configuration register"]
    pub timer_int1_map: TIMER_INT1_MAP,
    #[doc = "0x8c4 - timer_int2 interrupt configuration register"]
    pub timer_int2_map: TIMER_INT2_MAP,
    #[doc = "0x8c8 - tg_t0 interrupt configuration register"]
    pub tg_t0_int_map: TG_T0_INT_MAP,
    #[doc = "0x8cc - tg_t1 interrupt configuration register"]
    pub tg_t1_int_map: TG_T1_INT_MAP,
    #[doc = "0x8d0 - tg_wdt interrupt configuration register"]
    pub tg_wdt_int_map: TG_WDT_INT_MAP,
    #[doc = "0x8d4 - tg1_t0 interrupt configuration register"]
    pub tg1_t0_int_map: TG1_T0_INT_MAP,
    #[doc = "0x8d8 - tg1_t1 interrupt configuration register"]
    pub tg1_t1_int_map: TG1_T1_INT_MAP,
    #[doc = "0x8dc - tg1_wdt interrupt configuration register"]
    pub tg1_wdt_int_map: TG1_WDT_INT_MAP,
    #[doc = "0x8e0 - cache_ia interrupt configuration register"]
    pub cache_ia_int_map: CACHE_IA_INT_MAP,
    #[doc = "0x8e4 - systimer_target0 interrupt configuration register"]
    pub systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    #[doc = "0x8e8 - systimer_target1 interrupt configuration register"]
    pub systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    #[doc = "0x8ec - systimer_target2 interrupt configuration register"]
    pub systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    #[doc = "0x8f0 - spi_mem_reject interrupt configuration register"]
    pub spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    #[doc = "0x8f4 - dcache_prelaod interrupt configuration register"]
    pub dcache_preload_int_map: DCACHE_PRELOAD_INT_MAP,
    #[doc = "0x8f8 - icache_preload interrupt configuration register"]
    pub icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    #[doc = "0x8fc - dcache_sync interrupt configuration register"]
    pub dcache_sync_int_map: DCACHE_SYNC_INT_MAP,
    #[doc = "0x900 - icache_sync interrupt configuration register"]
    pub icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    #[doc = "0x904 - apb_adc interrupt configuration register"]
    pub apb_adc_int_map: APB_ADC_INT_MAP,
    #[doc = "0x908 - dma_in_ch0 interrupt configuration register"]
    pub dma_in_ch0_int_map: DMA_IN_CH0_INT_MAP,
    #[doc = "0x90c - dma_in_ch1 interrupt configuration register"]
    pub dma_in_ch1_int_map: DMA_IN_CH1_INT_MAP,
    #[doc = "0x910 - dma_in_ch2 interrupt configuration register"]
    pub dma_in_ch2_int_map: DMA_IN_CH2_INT_MAP,
    #[doc = "0x914 - dma_in_ch3 interrupt configuration register"]
    pub dma_in_ch3_int_map: DMA_IN_CH3_INT_MAP,
    #[doc = "0x918 - dma_in_ch4 interrupt configuration register"]
    pub dma_in_ch4_int_map: DMA_IN_CH4_INT_MAP,
    #[doc = "0x91c - dma_out_ch0 interrupt configuration register"]
    pub dma_out_ch0_int_map: DMA_OUT_CH0_INT_MAP,
    #[doc = "0x920 - dma_out_ch1 interrupt configuration register"]
    pub dma_out_ch1_int_map: DMA_OUT_CH1_INT_MAP,
    #[doc = "0x924 - dma_out_ch2 interrupt configuration register"]
    pub dma_out_ch2_int_map: DMA_OUT_CH2_INT_MAP,
    #[doc = "0x928 - dma_out_ch3 interrupt configuration register"]
    pub dma_out_ch3_int_map: DMA_OUT_CH3_INT_MAP,
    #[doc = "0x92c - dma_out_ch4 interrupt configuration register"]
    pub dma_out_ch4_int_map: DMA_OUT_CH4_INT_MAP,
    #[doc = "0x930 - rsa interrupt configuration register"]
    pub rsa_int_map: RSA_INT_MAP,
    #[doc = "0x934 - aes interrupt configuration register"]
    pub aes_int_map: AES_INT_MAP,
    #[doc = "0x938 - sha interrupt configuration register"]
    pub sha_int_map: SHA_INT_MAP,
    #[doc = "0x93c - cpu_intr_from_cpu_0 interrupt configuration register"]
    pub cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    #[doc = "0x940 - cpu_intr_from_cpu_1 interrupt configuration register"]
    pub cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    #[doc = "0x944 - cpu_intr_from_cpu_2 interrupt configuration register"]
    pub cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    #[doc = "0x948 - cpu_intr_from_cpu_3 interrupt configuration register"]
    pub cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    #[doc = "0x94c - assist_debug interrupt configuration register"]
    pub assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    #[doc = "0x950 - dma_pms_monitor_violatile interrupt configuration register"]
    pub dma_apbperi_pms_monitor_violate_intr_map: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x954 - core0_IRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_0_iram0_pms_monitor_violate_intr_map: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x958 - core0_DRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_0_dram0_pms_monitor_violate_intr_map: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x95c - core0_PIF_pms_monitor_violatile interrupt configuration register"]
    pub core_0_pif_pms_monitor_violate_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x960 - core0_PIF_pms_monitor_violatile_size interrupt configuration register"]
    pub core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0x964 - core1_IRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_1_iram0_pms_monitor_violate_intr_map: CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x968 - core1_DRam0_pms_monitor_violatile interrupt configuration register"]
    pub core_1_dram0_pms_monitor_violate_intr_map: CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x96c - core1_PIF_pms_monitor_violatile interrupt configuration register"]
    pub core_1_pif_pms_monitor_violate_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    #[doc = "0x970 - core1_PIF_pms_monitor_violatile_size interrupt configuration register"]
    pub core_1_pif_pms_monitor_violate_size_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    #[doc = "0x974 - backup_pms_monitor_violatile interrupt configuration register"]
    pub backup_pms_violate_intr_map: BACKUP_PMS_VIOLATE_INTR_MAP,
    #[doc = "0x978 - cache_core0_acs interrupt configuration register"]
    pub cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    #[doc = "0x97c - cache_core1_acs interrupt configuration register"]
    pub cache_core1_acs_int_map: CACHE_CORE1_ACS_INT_MAP,
    #[doc = "0x980 - usb_device interrupt configuration register"]
    pub usb_device_int_map: USB_DEVICE_INT_MAP,
    #[doc = "0x984 - peri_backup interrupt configuration register"]
    pub peri_backup_int_map: PERI_BACKUP_INT_MAP,
    #[doc = "0x988 - dma_extmem_reject interrupt configuration register"]
    pub dma_extmem_reject_int_map: DMA_EXTMEM_REJECT_INT_MAP,
    #[doc = "0x98c - interrupt status register"]
    pub app_intr_status_0: APP_INTR_STATUS_0,
    #[doc = "0x990 - interrupt status register"]
    pub app_intr_status_1: APP_INTR_STATUS_1,
    #[doc = "0x994 - interrupt status register"]
    pub app_intr_status_2: APP_INTR_STATUS_2,
    #[doc = "0x998 - interrupt status register"]
    pub app_intr_status_3: APP_INTR_STATUS_3,
    #[doc = "0x99c - clock gate register"]
    pub clock_gate: CLOCK_GATE,
    _reserved104: [u8; 0x065c],
    #[doc = "0xffc - version register"]
    pub date: DATE,
}
#[doc = "APP_MAC_INTR_MAP (rw) register accessor: mac interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`app_mac_intr_map`] module"]
pub type APP_MAC_INTR_MAP = crate::Reg<app_mac_intr_map::APP_MAC_INTR_MAP_SPEC>;
#[doc = "mac interrupt configuration register"]
pub mod app_mac_intr_map;
#[doc = "MAC_NMI_MAP (rw) register accessor: mac_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mac_nmi_map`] module"]
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
#[doc = "mac_nmi interrupt configuration register"]
pub mod mac_nmi_map;
#[doc = "PWR_INTR_MAP (rw) register accessor: pwr interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwr_intr_map`] module"]
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
#[doc = "pwr interrupt configuration register"]
pub mod pwr_intr_map;
#[doc = "BB_INT_MAP (rw) register accessor: bb interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bb_int_map`] module"]
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
#[doc = "bb interrupt configuration register"]
pub mod bb_int_map;
#[doc = "BT_MAC_INT_MAP (rw) register accessor: bb_mac interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_mac_int_map`] module"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "bb_mac interrupt configuration register"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP (rw) register accessor: bt_bb interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_bb_int_map`] module"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "bt_bb interrupt configuration register"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP (rw) register accessor: bt_bb_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_bb_nmi_map`] module"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "bt_bb_nmi interrupt configuration register"]
pub mod bt_bb_nmi_map;
#[doc = "RWBT_IRQ_MAP (rw) register accessor: rwbt_irq interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rwbt_irq_map`] module"]
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
#[doc = "rwbt_irq interrupt configuration register"]
pub mod rwbt_irq_map;
#[doc = "RWBLE_IRQ_MAP (rw) register accessor: rwble_irq interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rwble_irq_map`] module"]
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
#[doc = "rwble_irq interrupt configuration register"]
pub mod rwble_irq_map;
#[doc = "RWBT_NMI_MAP (rw) register accessor: rwbt_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rwbt_nmi_map`] module"]
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
#[doc = "rwbt_nmi interrupt configuration register"]
pub mod rwbt_nmi_map;
#[doc = "RWBLE_NMI_MAP (rw) register accessor: rwble_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rwble_nmi_map`] module"]
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
#[doc = "rwble_nmi interrupt configuration register"]
pub mod rwble_nmi_map;
#[doc = "I2C_MST_INT_MAP (rw) register accessor: i2c_mst interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_mst_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_mst_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_mst_int_map`] module"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "i2c_mst interrupt configuration register"]
pub mod i2c_mst_int_map;
#[doc = "SLC0_INTR_MAP (rw) register accessor: slc0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slc0_intr_map`] module"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "slc0 interrupt configuration register"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP (rw) register accessor: slc1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slc1_intr_map`] module"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "slc1 interrupt configuration register"]
pub mod slc1_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: uhci0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uhci0_intr_map`] module"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "uhci0 interrupt configuration register"]
pub mod uhci0_intr_map;
#[doc = "UHCI1_INTR_MAP (rw) register accessor: uhci1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uhci1_intr_map`] module"]
pub type UHCI1_INTR_MAP = crate::Reg<uhci1_intr_map::UHCI1_INTR_MAP_SPEC>;
#[doc = "uhci1 interrupt configuration register"]
pub mod uhci1_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP (rw) register accessor: gpio_interrupt_pro interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_pro_map`] module"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "gpio_interrupt_pro interrupt configuration register"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: gpio_interrupt_pro_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_pro_nmi_map`] module"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "gpio_interrupt_pro_nmi interrupt configuration register"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "GPIO_INTERRUPT_APP_MAP (rw) register accessor: gpio_interrupt_app interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_app_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_app_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_app_map`] module"]
pub type GPIO_INTERRUPT_APP_MAP = crate::Reg<gpio_interrupt_app_map::GPIO_INTERRUPT_APP_MAP_SPEC>;
#[doc = "gpio_interrupt_app interrupt configuration register"]
pub mod gpio_interrupt_app_map;
#[doc = "GPIO_INTERRUPT_APP_NMI_MAP (rw) register accessor: gpio_interrupt_app_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_app_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_app_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio_interrupt_app_nmi_map`] module"]
pub type GPIO_INTERRUPT_APP_NMI_MAP =
    crate::Reg<gpio_interrupt_app_nmi_map::GPIO_INTERRUPT_APP_NMI_MAP_SPEC>;
#[doc = "gpio_interrupt_app_nmi interrupt configuration register"]
pub mod gpio_interrupt_app_nmi_map;
#[doc = "SPI_INTR_1_MAP (rw) register accessor: spi_intr_1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_intr_1_map`] module"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "spi_intr_1 interrupt configuration register"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP (rw) register accessor: spi_intr_2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_intr_2_map`] module"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "spi_intr_2 interrupt configuration register"]
pub mod spi_intr_2_map;
#[doc = "SPI_INTR_3_MAP (rw) register accessor: spi_intr_3 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_intr_3_map`] module"]
pub type SPI_INTR_3_MAP = crate::Reg<spi_intr_3_map::SPI_INTR_3_MAP_SPEC>;
#[doc = "spi_intr_3 interrupt configuration register"]
pub mod spi_intr_3_map;
#[doc = "SPI_INTR_4_MAP (rw) register accessor: spi_intr_4 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_4_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_4_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_intr_4_map`] module"]
pub type SPI_INTR_4_MAP = crate::Reg<spi_intr_4_map::SPI_INTR_4_MAP_SPEC>;
#[doc = "spi_intr_4 interrupt configuration register"]
pub mod spi_intr_4_map;
#[doc = "LCD_CAM_INT_MAP (rw) register accessor: lcd_cam interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cam_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cam_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_cam_int_map`] module"]
pub type LCD_CAM_INT_MAP = crate::Reg<lcd_cam_int_map::LCD_CAM_INT_MAP_SPEC>;
#[doc = "lcd_cam interrupt configuration register"]
pub mod lcd_cam_int_map;
#[doc = "I2S0_INT_MAP (rw) register accessor: i2s0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2s0_int_map`] module"]
pub type I2S0_INT_MAP = crate::Reg<i2s0_int_map::I2S0_INT_MAP_SPEC>;
#[doc = "i2s0 interrupt configuration register"]
pub mod i2s0_int_map;
#[doc = "I2S1_INT_MAP (rw) register accessor: i2s1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2s1_int_map`] module"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "i2s1 interrupt configuration register"]
pub mod i2s1_int_map;
#[doc = "UART_INTR_MAP (rw) register accessor: uart interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_intr_map`] module"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "uart interrupt configuration register"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: uart1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "uart1 interrupt configuration register"]
pub mod uart1_intr_map;
#[doc = "UART2_INTR_MAP (rw) register accessor: uart2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart2_intr_map`] module"]
pub type UART2_INTR_MAP = crate::Reg<uart2_intr_map::UART2_INTR_MAP_SPEC>;
#[doc = "uart2 interrupt configuration register"]
pub mod uart2_intr_map;
#[doc = "SDIO_HOST_INTERRUPT_MAP (rw) register accessor: sdio_host interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_host_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_host_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_host_interrupt_map`] module"]
pub type SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<sdio_host_interrupt_map::SDIO_HOST_INTERRUPT_MAP_SPEC>;
#[doc = "sdio_host interrupt configuration register"]
pub mod sdio_host_interrupt_map;
#[doc = "PWM0_INTR_MAP (rw) register accessor: pwm0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm0_intr_map`] module"]
pub type PWM0_INTR_MAP = crate::Reg<pwm0_intr_map::PWM0_INTR_MAP_SPEC>;
#[doc = "pwm0 interrupt configuration register"]
pub mod pwm0_intr_map;
#[doc = "PWM1_INTR_MAP (rw) register accessor: pwm1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm1_intr_map`] module"]
pub type PWM1_INTR_MAP = crate::Reg<pwm1_intr_map::PWM1_INTR_MAP_SPEC>;
#[doc = "pwm1 interrupt configuration register"]
pub mod pwm1_intr_map;
#[doc = "PWM2_INTR_MAP (rw) register accessor: pwm2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm2_intr_map`] module"]
pub type PWM2_INTR_MAP = crate::Reg<pwm2_intr_map::PWM2_INTR_MAP_SPEC>;
#[doc = "pwm2 interrupt configuration register"]
pub mod pwm2_intr_map;
#[doc = "PWM3_INTR_MAP (rw) register accessor: pwm3 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwm3_intr_map`] module"]
pub type PWM3_INTR_MAP = crate::Reg<pwm3_intr_map::PWM3_INTR_MAP_SPEC>;
#[doc = "pwm3 interrupt configuration register"]
pub mod pwm3_intr_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: ledc interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ledc_int_map`] module"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "ledc interrupt configuration register"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP (rw) register accessor: efuse interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`efuse_int_map`] module"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "efuse interrupt configuration register"]
pub mod efuse_int_map;
#[doc = "CAN_INT_MAP (rw) register accessor: can interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`can_int_map`] module"]
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
#[doc = "can interrupt configuration register"]
pub mod can_int_map;
#[doc = "USB_INTR_MAP (rw) register accessor: usb interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb_intr_map`] module"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "usb interrupt configuration register"]
pub mod usb_intr_map;
#[doc = "RTC_CORE_INTR_MAP (rw) register accessor: rtc_core interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtc_core_intr_map`] module"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "rtc_core interrupt configuration register"]
pub mod rtc_core_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: rmt interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rmt_intr_map`] module"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "rmt interrupt configuration register"]
pub mod rmt_intr_map;
#[doc = "PCNT_INTR_MAP (rw) register accessor: pcnt interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcnt_intr_map`] module"]
pub type PCNT_INTR_MAP = crate::Reg<pcnt_intr_map::PCNT_INTR_MAP_SPEC>;
#[doc = "pcnt interrupt configuration register"]
pub mod pcnt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP (rw) register accessor: i2c_ext0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_ext0_intr_map`] module"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "i2c_ext0 interrupt configuration register"]
pub mod i2c_ext0_intr_map;
#[doc = "I2C_EXT1_INTR_MAP (rw) register accessor: i2c_ext1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`i2c_ext1_intr_map`] module"]
pub type I2C_EXT1_INTR_MAP = crate::Reg<i2c_ext1_intr_map::I2C_EXT1_INTR_MAP_SPEC>;
#[doc = "i2c_ext1 interrupt configuration register"]
pub mod i2c_ext1_intr_map;
#[doc = "SPI2_DMA_INT_MAP (rw) register accessor: spi2_dma interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi2_dma_int_map`] module"]
pub type SPI2_DMA_INT_MAP = crate::Reg<spi2_dma_int_map::SPI2_DMA_INT_MAP_SPEC>;
#[doc = "spi2_dma interrupt configuration register"]
pub mod spi2_dma_int_map;
#[doc = "SPI3_DMA_INT_MAP (rw) register accessor: spi3_dma interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi3_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi3_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi3_dma_int_map`] module"]
pub type SPI3_DMA_INT_MAP = crate::Reg<spi3_dma_int_map::SPI3_DMA_INT_MAP_SPEC>;
#[doc = "spi3_dma interrupt configuration register"]
pub mod spi3_dma_int_map;
#[doc = "SPI4_DMA_INT_MAP (rw) register accessor: spi4_dma interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi4_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi4_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi4_dma_int_map`] module"]
pub type SPI4_DMA_INT_MAP = crate::Reg<spi4_dma_int_map::SPI4_DMA_INT_MAP_SPEC>;
#[doc = "spi4_dma interrupt configuration register"]
pub mod spi4_dma_int_map;
#[doc = "WDG_INT_MAP (rw) register accessor: wdg interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wdg_int_map`] module"]
pub type WDG_INT_MAP = crate::Reg<wdg_int_map::WDG_INT_MAP_SPEC>;
#[doc = "wdg interrupt configuration register"]
pub mod wdg_int_map;
#[doc = "TIMER_INT1_MAP (rw) register accessor: timer_int1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_int1_map`] module"]
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
#[doc = "timer_int1 interrupt configuration register"]
pub mod timer_int1_map;
#[doc = "TIMER_INT2_MAP (rw) register accessor: timer_int2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timer_int2_map`] module"]
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
#[doc = "timer_int2 interrupt configuration register"]
pub mod timer_int2_map;
#[doc = "TG_T0_INT_MAP (rw) register accessor: tg_t0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg_t0_int_map`] module"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "tg_t0 interrupt configuration register"]
pub mod tg_t0_int_map;
#[doc = "TG_T1_INT_MAP (rw) register accessor: tg_t1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg_t1_int_map`] module"]
pub type TG_T1_INT_MAP = crate::Reg<tg_t1_int_map::TG_T1_INT_MAP_SPEC>;
#[doc = "tg_t1 interrupt configuration register"]
pub mod tg_t1_int_map;
#[doc = "TG_WDT_INT_MAP (rw) register accessor: tg_wdt interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg_wdt_int_map`] module"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "tg_wdt interrupt configuration register"]
pub mod tg_wdt_int_map;
#[doc = "TG1_T0_INT_MAP (rw) register accessor: tg1_t0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg1_t0_int_map`] module"]
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
#[doc = "tg1_t0 interrupt configuration register"]
pub mod tg1_t0_int_map;
#[doc = "TG1_T1_INT_MAP (rw) register accessor: tg1_t1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg1_t1_int_map`] module"]
pub type TG1_T1_INT_MAP = crate::Reg<tg1_t1_int_map::TG1_T1_INT_MAP_SPEC>;
#[doc = "tg1_t1 interrupt configuration register"]
pub mod tg1_t1_int_map;
#[doc = "TG1_WDT_INT_MAP (rw) register accessor: tg1_wdt interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tg1_wdt_int_map`] module"]
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
#[doc = "tg1_wdt interrupt configuration register"]
pub mod tg1_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP (rw) register accessor: cache_ia interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_ia_int_map`] module"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "cache_ia interrupt configuration register"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: systimer_target0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target0_int_map`] module"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "systimer_target0 interrupt configuration register"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: systimer_target1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target1_int_map`] module"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "systimer_target1 interrupt configuration register"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: systimer_target2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`systimer_target2_int_map`] module"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "systimer_target2 interrupt configuration register"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP (rw) register accessor: spi_mem_reject interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_reject_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_reject_intr_map`] module"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "spi_mem_reject interrupt configuration register"]
pub mod spi_mem_reject_intr_map;
#[doc = "DCACHE_PRELOAD_INT_MAP (rw) register accessor: dcache_prelaod interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_preload_int_map`] module"]
pub type DCACHE_PRELOAD_INT_MAP = crate::Reg<dcache_preload_int_map::DCACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "dcache_prelaod interrupt configuration register"]
pub mod dcache_preload_int_map;
#[doc = "ICACHE_PRELOAD_INT_MAP (rw) register accessor: icache_preload interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_preload_int_map`] module"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "icache_preload interrupt configuration register"]
pub mod icache_preload_int_map;
#[doc = "DCACHE_SYNC_INT_MAP (rw) register accessor: dcache_sync interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcache_sync_int_map`] module"]
pub type DCACHE_SYNC_INT_MAP = crate::Reg<dcache_sync_int_map::DCACHE_SYNC_INT_MAP_SPEC>;
#[doc = "dcache_sync interrupt configuration register"]
pub mod dcache_sync_int_map;
#[doc = "ICACHE_SYNC_INT_MAP (rw) register accessor: icache_sync interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`icache_sync_int_map`] module"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "icache_sync interrupt configuration register"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP (rw) register accessor: apb_adc interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb_adc_int_map`] module"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "apb_adc interrupt configuration register"]
pub mod apb_adc_int_map;
#[doc = "DMA_IN_CH0_INT_MAP (rw) register accessor: dma_in_ch0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch0_int_map`] module"]
pub type DMA_IN_CH0_INT_MAP = crate::Reg<dma_in_ch0_int_map::DMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "dma_in_ch0 interrupt configuration register"]
pub mod dma_in_ch0_int_map;
#[doc = "DMA_IN_CH1_INT_MAP (rw) register accessor: dma_in_ch1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch1_int_map`] module"]
pub type DMA_IN_CH1_INT_MAP = crate::Reg<dma_in_ch1_int_map::DMA_IN_CH1_INT_MAP_SPEC>;
#[doc = "dma_in_ch1 interrupt configuration register"]
pub mod dma_in_ch1_int_map;
#[doc = "DMA_IN_CH2_INT_MAP (rw) register accessor: dma_in_ch2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch2_int_map`] module"]
pub type DMA_IN_CH2_INT_MAP = crate::Reg<dma_in_ch2_int_map::DMA_IN_CH2_INT_MAP_SPEC>;
#[doc = "dma_in_ch2 interrupt configuration register"]
pub mod dma_in_ch2_int_map;
#[doc = "DMA_IN_CH3_INT_MAP (rw) register accessor: dma_in_ch3 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch3_int_map`] module"]
pub type DMA_IN_CH3_INT_MAP = crate::Reg<dma_in_ch3_int_map::DMA_IN_CH3_INT_MAP_SPEC>;
#[doc = "dma_in_ch3 interrupt configuration register"]
pub mod dma_in_ch3_int_map;
#[doc = "DMA_IN_CH4_INT_MAP (rw) register accessor: dma_in_ch4 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_in_ch4_int_map`] module"]
pub type DMA_IN_CH4_INT_MAP = crate::Reg<dma_in_ch4_int_map::DMA_IN_CH4_INT_MAP_SPEC>;
#[doc = "dma_in_ch4 interrupt configuration register"]
pub mod dma_in_ch4_int_map;
#[doc = "DMA_OUT_CH0_INT_MAP (rw) register accessor: dma_out_ch0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch0_int_map`] module"]
pub type DMA_OUT_CH0_INT_MAP = crate::Reg<dma_out_ch0_int_map::DMA_OUT_CH0_INT_MAP_SPEC>;
#[doc = "dma_out_ch0 interrupt configuration register"]
pub mod dma_out_ch0_int_map;
#[doc = "DMA_OUT_CH1_INT_MAP (rw) register accessor: dma_out_ch1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch1_int_map`] module"]
pub type DMA_OUT_CH1_INT_MAP = crate::Reg<dma_out_ch1_int_map::DMA_OUT_CH1_INT_MAP_SPEC>;
#[doc = "dma_out_ch1 interrupt configuration register"]
pub mod dma_out_ch1_int_map;
#[doc = "DMA_OUT_CH2_INT_MAP (rw) register accessor: dma_out_ch2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch2_int_map`] module"]
pub type DMA_OUT_CH2_INT_MAP = crate::Reg<dma_out_ch2_int_map::DMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "dma_out_ch2 interrupt configuration register"]
pub mod dma_out_ch2_int_map;
#[doc = "DMA_OUT_CH3_INT_MAP (rw) register accessor: dma_out_ch3 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch3_int_map`] module"]
pub type DMA_OUT_CH3_INT_MAP = crate::Reg<dma_out_ch3_int_map::DMA_OUT_CH3_INT_MAP_SPEC>;
#[doc = "dma_out_ch3 interrupt configuration register"]
pub mod dma_out_ch3_int_map;
#[doc = "DMA_OUT_CH4_INT_MAP (rw) register accessor: dma_out_ch4 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_out_ch4_int_map`] module"]
pub type DMA_OUT_CH4_INT_MAP = crate::Reg<dma_out_ch4_int_map::DMA_OUT_CH4_INT_MAP_SPEC>;
#[doc = "dma_out_ch4 interrupt configuration register"]
pub mod dma_out_ch4_int_map;
#[doc = "RSA_INT_MAP (rw) register accessor: rsa interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsa_int_map`] module"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "rsa interrupt configuration register"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP (rw) register accessor: aes interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aes_int_map`] module"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "aes interrupt configuration register"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: sha interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sha_int_map`] module"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "sha interrupt configuration register"]
pub mod sha_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: cpu_intr_from_cpu_0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_0 interrupt configuration register"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: cpu_intr_from_cpu_1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_1 interrupt configuration register"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: cpu_intr_from_cpu_2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_2 interrupt configuration register"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: cpu_intr_from_cpu_3 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "cpu_intr_from_cpu_3 interrupt configuration register"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP (rw) register accessor: assist_debug interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`assist_debug_intr_map`] module"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "assist_debug interrupt configuration register"]
pub mod assist_debug_intr_map;
#[doc = "DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: dma_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_apbperi_pms_monitor_violate_intr_map`] module"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "dma_pms_monitor_violatile interrupt configuration register"]
pub mod dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_IRam0_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_0_iram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_IRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_DRam0_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_dram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_0_dram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_DRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_PIF_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_0_pif_pms_monitor_violate_intr_map`] module"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core0_PIF_pms_monitor_violatile interrupt configuration register"]
pub mod core_0_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: core0_PIF_pms_monitor_violatile_size interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_size_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_size_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_0_pif_pms_monitor_violate_size_intr_map`] module"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "core0_PIF_pms_monitor_violatile_size interrupt configuration register"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_IRam0_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_1_iram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_iram0_pms_monitor_violate_intr_map::CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_IRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_DRam0_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_dram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_1_dram0_pms_monitor_violate_intr_map`] module"]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_dram0_pms_monitor_violate_intr_map::CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_DRam0_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_PIF_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_1_pif_pms_monitor_violate_intr_map`] module"]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "core1_PIF_pms_monitor_violatile interrupt configuration register"]
pub mod core_1_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: core1_PIF_pms_monitor_violatile_size interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_violate_size_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_violate_size_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`core_1_pif_pms_monitor_violate_size_intr_map`] module"]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_size_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "core1_PIF_pms_monitor_violatile_size interrupt configuration register"]
pub mod core_1_pif_pms_monitor_violate_size_intr_map;
#[doc = "BACKUP_PMS_VIOLATE_INTR_MAP (rw) register accessor: backup_pms_monitor_violatile interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_pms_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_pms_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`backup_pms_violate_intr_map`] module"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "backup_pms_monitor_violatile interrupt configuration register"]
pub mod backup_pms_violate_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP (rw) register accessor: cache_core0_acs interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_core0_acs_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_core0_acs_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_core0_acs_int_map`] module"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "cache_core0_acs interrupt configuration register"]
pub mod cache_core0_acs_int_map;
#[doc = "CACHE_CORE1_ACS_INT_MAP (rw) register accessor: cache_core1_acs interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_core1_acs_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_core1_acs_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cache_core1_acs_int_map`] module"]
pub type CACHE_CORE1_ACS_INT_MAP =
    crate::Reg<cache_core1_acs_int_map::CACHE_CORE1_ACS_INT_MAP_SPEC>;
#[doc = "cache_core1_acs interrupt configuration register"]
pub mod cache_core1_acs_int_map;
#[doc = "USB_DEVICE_INT_MAP (rw) register accessor: usb_device interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_device_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_device_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usb_device_int_map`] module"]
pub type USB_DEVICE_INT_MAP = crate::Reg<usb_device_int_map::USB_DEVICE_INT_MAP_SPEC>;
#[doc = "usb_device interrupt configuration register"]
pub mod usb_device_int_map;
#[doc = "PERI_BACKUP_INT_MAP (rw) register accessor: peri_backup interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_int_map`] module"]
pub type PERI_BACKUP_INT_MAP = crate::Reg<peri_backup_int_map::PERI_BACKUP_INT_MAP_SPEC>;
#[doc = "peri_backup interrupt configuration register"]
pub mod peri_backup_int_map;
#[doc = "DMA_EXTMEM_REJECT_INT_MAP (rw) register accessor: dma_extmem_reject interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_extmem_reject_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_extmem_reject_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_extmem_reject_int_map`] module"]
pub type DMA_EXTMEM_REJECT_INT_MAP =
    crate::Reg<dma_extmem_reject_int_map::DMA_EXTMEM_REJECT_INT_MAP_SPEC>;
#[doc = "dma_extmem_reject interrupt configuration register"]
pub mod dma_extmem_reject_int_map;
#[doc = "APP_INTR_STATUS_0 (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`app_intr_status_0`] module"]
pub type APP_INTR_STATUS_0 = crate::Reg<app_intr_status_0::APP_INTR_STATUS_0_SPEC>;
#[doc = "interrupt status register"]
pub mod app_intr_status_0;
#[doc = "APP_INTR_STATUS_1 (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`app_intr_status_1`] module"]
pub type APP_INTR_STATUS_1 = crate::Reg<app_intr_status_1::APP_INTR_STATUS_1_SPEC>;
#[doc = "interrupt status register"]
pub mod app_intr_status_1;
#[doc = "APP_INTR_STATUS_2 (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`app_intr_status_2`] module"]
pub type APP_INTR_STATUS_2 = crate::Reg<app_intr_status_2::APP_INTR_STATUS_2_SPEC>;
#[doc = "interrupt status register"]
pub mod app_intr_status_2;
#[doc = "APP_INTR_STATUS_3 (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`app_intr_status_3`] module"]
pub type APP_INTR_STATUS_3 = crate::Reg<app_intr_status_3::APP_INTR_STATUS_3_SPEC>;
#[doc = "interrupt status register"]
pub mod app_intr_status_3;
#[doc = "CLOCK_GATE (rw) register accessor: clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
