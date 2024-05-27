#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pro_mac_intr_map: PRO_MAC_INTR_MAP,
    mac_nmi_map: MAC_NMI_MAP,
    pwr_intr_map: PWR_INTR_MAP,
    bb_int_map: BB_INT_MAP,
    bt_mac_int_map: BT_MAC_INT_MAP,
    bt_bb_int_map: BT_BB_INT_MAP,
    bt_bb_nmi_map: BT_BB_NMI_MAP,
    rwbt_irq_map: RWBT_IRQ_MAP,
    rwble_irq_map: RWBLE_IRQ_MAP,
    rwbt_nmi_map: RWBT_NMI_MAP,
    rwble_nmi_map: RWBLE_NMI_MAP,
    i2c_mst_int_map: I2C_MST_INT_MAP,
    slc0_intr_map: SLC0_INTR_MAP,
    slc1_intr_map: SLC1_INTR_MAP,
    uhci0_intr_map: UHCI0_INTR_MAP,
    uhci1_intr_map: UHCI1_INTR_MAP,
    gpio_interrupt_pro_map: GPIO_INTERRUPT_PRO_MAP,
    gpio_interrupt_pro_nmi_map: GPIO_INTERRUPT_PRO_NMI_MAP,
    gpio_interrupt_app_map: GPIO_INTERRUPT_APP_MAP,
    gpio_interrupt_app_nmi_map: GPIO_INTERRUPT_APP_NMI_MAP,
    spi_intr_1_map: SPI_INTR_1_MAP,
    spi_intr_2_map: SPI_INTR_2_MAP,
    spi_intr_3_map: SPI_INTR_3_MAP,
    spi_intr_4_map: SPI_INTR_4_MAP,
    lcd_cam_int_map: LCD_CAM_INT_MAP,
    i2s0_int_map: I2S0_INT_MAP,
    i2s1_int_map: I2S1_INT_MAP,
    uart_intr_map: UART_INTR_MAP,
    uart1_intr_map: UART1_INTR_MAP,
    uart2_intr_map: UART2_INTR_MAP,
    sdio_host_interrupt_map: SDIO_HOST_INTERRUPT_MAP,
    pwm0_intr_map: PWM0_INTR_MAP,
    pwm1_intr_map: PWM1_INTR_MAP,
    pwm2_intr_map: PWM2_INTR_MAP,
    pwm3_intr_map: PWM3_INTR_MAP,
    ledc_int_map: LEDC_INT_MAP,
    efuse_int_map: EFUSE_INT_MAP,
    can_int_map: CAN_INT_MAP,
    usb_intr_map: USB_INTR_MAP,
    rtc_core_intr_map: RTC_CORE_INTR_MAP,
    rmt_intr_map: RMT_INTR_MAP,
    pcnt_intr_map: PCNT_INTR_MAP,
    i2c_ext0_intr_map: I2C_EXT0_INTR_MAP,
    i2c_ext1_intr_map: I2C_EXT1_INTR_MAP,
    spi2_dma_int_map: SPI2_DMA_INT_MAP,
    spi3_dma_int_map: SPI3_DMA_INT_MAP,
    spi4_dma_int_map: SPI4_DMA_INT_MAP,
    wdg_int_map: WDG_INT_MAP,
    timer_int1_map: TIMER_INT1_MAP,
    timer_int2_map: TIMER_INT2_MAP,
    tg_t0_int_map: TG_T0_INT_MAP,
    tg_t1_int_map: TG_T1_INT_MAP,
    tg_wdt_int_map: TG_WDT_INT_MAP,
    tg1_t0_int_map: TG1_T0_INT_MAP,
    tg1_t1_int_map: TG1_T1_INT_MAP,
    tg1_wdt_int_map: TG1_WDT_INT_MAP,
    cache_ia_int_map: CACHE_IA_INT_MAP,
    systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    spi_mem_reject_intr_map: SPI_MEM_REJECT_INTR_MAP,
    dcache_preload_int_map: DCACHE_PRELOAD_INT_MAP,
    icache_preload_int_map: ICACHE_PRELOAD_INT_MAP,
    dcache_sync_int_map: DCACHE_SYNC_INT_MAP,
    icache_sync_int_map: ICACHE_SYNC_INT_MAP,
    apb_adc_int_map: APB_ADC_INT_MAP,
    dma_in_ch0_int_map: DMA_IN_CH0_INT_MAP,
    dma_in_ch1_int_map: DMA_IN_CH1_INT_MAP,
    dma_in_ch2_int_map: DMA_IN_CH2_INT_MAP,
    dma_in_ch3_int_map: DMA_IN_CH3_INT_MAP,
    dma_in_ch4_int_map: DMA_IN_CH4_INT_MAP,
    dma_out_ch0_int_map: DMA_OUT_CH0_INT_MAP,
    dma_out_ch1_int_map: DMA_OUT_CH1_INT_MAP,
    dma_out_ch2_int_map: DMA_OUT_CH2_INT_MAP,
    dma_out_ch3_int_map: DMA_OUT_CH3_INT_MAP,
    dma_out_ch4_int_map: DMA_OUT_CH4_INT_MAP,
    rsa_int_map: RSA_INT_MAP,
    aes_int_map: AES_INT_MAP,
    sha_int_map: SHA_INT_MAP,
    cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    assist_debug_intr_map: ASSIST_DEBUG_INTR_MAP,
    dma_apbperi_pms_monitor_violate_intr_map: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_iram0_pms_monitor_violate_intr_map: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_dram0_pms_monitor_violate_intr_map: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_pif_pms_monitor_violate_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_0_pif_pms_monitor_violate_size_intr_map: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    core_1_iram0_pms_monitor_violate_intr_map: CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_1_dram0_pms_monitor_violate_intr_map: CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_1_pif_pms_monitor_violate_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP,
    core_1_pif_pms_monitor_violate_size_intr_map: CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP,
    backup_pms_violate_intr_map: BACKUP_PMS_VIOLATE_INTR_MAP,
    cache_core0_acs_int_map: CACHE_CORE0_ACS_INT_MAP,
    cache_core1_acs_int_map: CACHE_CORE1_ACS_INT_MAP,
    usb_device_int_map: USB_DEVICE_INT_MAP,
    peri_backup_int_map: PERI_BACKUP_INT_MAP,
    dma_extmem_reject_int_map: DMA_EXTMEM_REJECT_INT_MAP,
    pro_intr_status_0: PRO_INTR_STATUS_0,
    pro_intr_status_1: PRO_INTR_STATUS_1,
    pro_intr_status_2: PRO_INTR_STATUS_2,
    pro_intr_status_3: PRO_INTR_STATUS_3,
    clock_gate: CLOCK_GATE,
    _reserved104: [u8; 0x065c],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - mac interrupt configuration register
    #[inline(always)]
    pub const fn pro_mac_intr_map(&self) -> &PRO_MAC_INTR_MAP {
        &self.pro_mac_intr_map
    }
    ///0x04 - mac_nmi interrupt configuration register
    #[inline(always)]
    pub const fn mac_nmi_map(&self) -> &MAC_NMI_MAP {
        &self.mac_nmi_map
    }
    ///0x08 - pwr interrupt configuration register
    #[inline(always)]
    pub const fn pwr_intr_map(&self) -> &PWR_INTR_MAP {
        &self.pwr_intr_map
    }
    ///0x0c - bb interrupt configuration register
    #[inline(always)]
    pub const fn bb_int_map(&self) -> &BB_INT_MAP {
        &self.bb_int_map
    }
    ///0x10 - bb_mac interrupt configuration register
    #[inline(always)]
    pub const fn bt_mac_int_map(&self) -> &BT_MAC_INT_MAP {
        &self.bt_mac_int_map
    }
    ///0x14 - bt_bb interrupt configuration register
    #[inline(always)]
    pub const fn bt_bb_int_map(&self) -> &BT_BB_INT_MAP {
        &self.bt_bb_int_map
    }
    ///0x18 - bt_bb_nmi interrupt configuration register
    #[inline(always)]
    pub const fn bt_bb_nmi_map(&self) -> &BT_BB_NMI_MAP {
        &self.bt_bb_nmi_map
    }
    ///0x1c - rwbt_irq interrupt configuration register
    #[inline(always)]
    pub const fn rwbt_irq_map(&self) -> &RWBT_IRQ_MAP {
        &self.rwbt_irq_map
    }
    ///0x20 - rwble_irq interrupt configuration register
    #[inline(always)]
    pub const fn rwble_irq_map(&self) -> &RWBLE_IRQ_MAP {
        &self.rwble_irq_map
    }
    ///0x24 - rwbt_nmi interrupt configuration register
    #[inline(always)]
    pub const fn rwbt_nmi_map(&self) -> &RWBT_NMI_MAP {
        &self.rwbt_nmi_map
    }
    ///0x28 - rwble_nmi interrupt configuration register
    #[inline(always)]
    pub const fn rwble_nmi_map(&self) -> &RWBLE_NMI_MAP {
        &self.rwble_nmi_map
    }
    ///0x2c - i2c_mst interrupt configuration register
    #[inline(always)]
    pub const fn i2c_mst_int_map(&self) -> &I2C_MST_INT_MAP {
        &self.i2c_mst_int_map
    }
    ///0x30 - slc0 interrupt configuration register
    #[inline(always)]
    pub const fn slc0_intr_map(&self) -> &SLC0_INTR_MAP {
        &self.slc0_intr_map
    }
    ///0x34 - slc1 interrupt configuration register
    #[inline(always)]
    pub const fn slc1_intr_map(&self) -> &SLC1_INTR_MAP {
        &self.slc1_intr_map
    }
    ///0x38 - uhci0 interrupt configuration register
    #[inline(always)]
    pub const fn uhci0_intr_map(&self) -> &UHCI0_INTR_MAP {
        &self.uhci0_intr_map
    }
    ///0x3c - uhci1 interrupt configuration register
    #[inline(always)]
    pub const fn uhci1_intr_map(&self) -> &UHCI1_INTR_MAP {
        &self.uhci1_intr_map
    }
    ///0x40 - gpio_interrupt_pro interrupt configuration register
    #[inline(always)]
    pub const fn gpio_interrupt_pro_map(&self) -> &GPIO_INTERRUPT_PRO_MAP {
        &self.gpio_interrupt_pro_map
    }
    ///0x44 - gpio_interrupt_pro_nmi interrupt configuration register
    #[inline(always)]
    pub const fn gpio_interrupt_pro_nmi_map(&self) -> &GPIO_INTERRUPT_PRO_NMI_MAP {
        &self.gpio_interrupt_pro_nmi_map
    }
    ///0x48 - gpio_interrupt_app interrupt configuration register
    #[inline(always)]
    pub const fn gpio_interrupt_app_map(&self) -> &GPIO_INTERRUPT_APP_MAP {
        &self.gpio_interrupt_app_map
    }
    ///0x4c - gpio_interrupt_app_nmi interrupt configuration register
    #[inline(always)]
    pub const fn gpio_interrupt_app_nmi_map(&self) -> &GPIO_INTERRUPT_APP_NMI_MAP {
        &self.gpio_interrupt_app_nmi_map
    }
    ///0x50 - spi_intr_1 interrupt configuration register
    #[inline(always)]
    pub const fn spi_intr_1_map(&self) -> &SPI_INTR_1_MAP {
        &self.spi_intr_1_map
    }
    ///0x54 - spi_intr_2 interrupt configuration register
    #[inline(always)]
    pub const fn spi_intr_2_map(&self) -> &SPI_INTR_2_MAP {
        &self.spi_intr_2_map
    }
    ///0x58 - spi_intr_3 interrupt configuration register
    #[inline(always)]
    pub const fn spi_intr_3_map(&self) -> &SPI_INTR_3_MAP {
        &self.spi_intr_3_map
    }
    ///0x5c - spi_intr_4 interrupt configuration register
    #[inline(always)]
    pub const fn spi_intr_4_map(&self) -> &SPI_INTR_4_MAP {
        &self.spi_intr_4_map
    }
    ///0x60 - lcd_cam interrupt configuration register
    #[inline(always)]
    pub const fn lcd_cam_int_map(&self) -> &LCD_CAM_INT_MAP {
        &self.lcd_cam_int_map
    }
    ///0x64 - i2s0 interrupt configuration register
    #[inline(always)]
    pub const fn i2s0_int_map(&self) -> &I2S0_INT_MAP {
        &self.i2s0_int_map
    }
    ///0x68 - i2s1 interrupt configuration register
    #[inline(always)]
    pub const fn i2s1_int_map(&self) -> &I2S1_INT_MAP {
        &self.i2s1_int_map
    }
    ///0x6c - uart interrupt configuration register
    #[inline(always)]
    pub const fn uart_intr_map(&self) -> &UART_INTR_MAP {
        &self.uart_intr_map
    }
    ///0x70 - uart1 interrupt configuration register
    #[inline(always)]
    pub const fn uart1_intr_map(&self) -> &UART1_INTR_MAP {
        &self.uart1_intr_map
    }
    ///0x74 - uart2 interrupt configuration register
    #[inline(always)]
    pub const fn uart2_intr_map(&self) -> &UART2_INTR_MAP {
        &self.uart2_intr_map
    }
    ///0x78 - sdio_host interrupt configuration register
    #[inline(always)]
    pub const fn sdio_host_interrupt_map(&self) -> &SDIO_HOST_INTERRUPT_MAP {
        &self.sdio_host_interrupt_map
    }
    ///0x7c - pwm0 interrupt configuration register
    #[inline(always)]
    pub const fn pwm0_intr_map(&self) -> &PWM0_INTR_MAP {
        &self.pwm0_intr_map
    }
    ///0x80 - pwm1 interrupt configuration register
    #[inline(always)]
    pub const fn pwm1_intr_map(&self) -> &PWM1_INTR_MAP {
        &self.pwm1_intr_map
    }
    ///0x84 - pwm2 interrupt configuration register
    #[inline(always)]
    pub const fn pwm2_intr_map(&self) -> &PWM2_INTR_MAP {
        &self.pwm2_intr_map
    }
    ///0x88 - pwm3 interrupt configuration register
    #[inline(always)]
    pub const fn pwm3_intr_map(&self) -> &PWM3_INTR_MAP {
        &self.pwm3_intr_map
    }
    ///0x8c - ledc interrupt configuration register
    #[inline(always)]
    pub const fn ledc_int_map(&self) -> &LEDC_INT_MAP {
        &self.ledc_int_map
    }
    ///0x90 - efuse interrupt configuration register
    #[inline(always)]
    pub const fn efuse_int_map(&self) -> &EFUSE_INT_MAP {
        &self.efuse_int_map
    }
    ///0x94 - can interrupt configuration register
    #[inline(always)]
    pub const fn can_int_map(&self) -> &CAN_INT_MAP {
        &self.can_int_map
    }
    ///0x98 - usb interrupt configuration register
    #[inline(always)]
    pub const fn usb_intr_map(&self) -> &USB_INTR_MAP {
        &self.usb_intr_map
    }
    ///0x9c - rtc_core interrupt configuration register
    #[inline(always)]
    pub const fn rtc_core_intr_map(&self) -> &RTC_CORE_INTR_MAP {
        &self.rtc_core_intr_map
    }
    ///0xa0 - rmt interrupt configuration register
    #[inline(always)]
    pub const fn rmt_intr_map(&self) -> &RMT_INTR_MAP {
        &self.rmt_intr_map
    }
    ///0xa4 - pcnt interrupt configuration register
    #[inline(always)]
    pub const fn pcnt_intr_map(&self) -> &PCNT_INTR_MAP {
        &self.pcnt_intr_map
    }
    ///0xa8 - i2c_ext0 interrupt configuration register
    #[inline(always)]
    pub const fn i2c_ext0_intr_map(&self) -> &I2C_EXT0_INTR_MAP {
        &self.i2c_ext0_intr_map
    }
    ///0xac - i2c_ext1 interrupt configuration register
    #[inline(always)]
    pub const fn i2c_ext1_intr_map(&self) -> &I2C_EXT1_INTR_MAP {
        &self.i2c_ext1_intr_map
    }
    ///0xb0 - spi2_dma interrupt configuration register
    #[inline(always)]
    pub const fn spi2_dma_int_map(&self) -> &SPI2_DMA_INT_MAP {
        &self.spi2_dma_int_map
    }
    ///0xb4 - spi3_dma interrupt configuration register
    #[inline(always)]
    pub const fn spi3_dma_int_map(&self) -> &SPI3_DMA_INT_MAP {
        &self.spi3_dma_int_map
    }
    ///0xb8 - spi4_dma interrupt configuration register
    #[inline(always)]
    pub const fn spi4_dma_int_map(&self) -> &SPI4_DMA_INT_MAP {
        &self.spi4_dma_int_map
    }
    ///0xbc - wdg interrupt configuration register
    #[inline(always)]
    pub const fn wdg_int_map(&self) -> &WDG_INT_MAP {
        &self.wdg_int_map
    }
    ///0xc0 - timer_int1 interrupt configuration register
    #[inline(always)]
    pub const fn timer_int1_map(&self) -> &TIMER_INT1_MAP {
        &self.timer_int1_map
    }
    ///0xc4 - timer_int2 interrupt configuration register
    #[inline(always)]
    pub const fn timer_int2_map(&self) -> &TIMER_INT2_MAP {
        &self.timer_int2_map
    }
    ///0xc8 - tg_t0 interrupt configuration register
    #[inline(always)]
    pub const fn tg_t0_int_map(&self) -> &TG_T0_INT_MAP {
        &self.tg_t0_int_map
    }
    ///0xcc - tg_t1 interrupt configuration register
    #[inline(always)]
    pub const fn tg_t1_int_map(&self) -> &TG_T1_INT_MAP {
        &self.tg_t1_int_map
    }
    ///0xd0 - tg_wdt interrupt configuration register
    #[inline(always)]
    pub const fn tg_wdt_int_map(&self) -> &TG_WDT_INT_MAP {
        &self.tg_wdt_int_map
    }
    ///0xd4 - tg1_t0 interrupt configuration register
    #[inline(always)]
    pub const fn tg1_t0_int_map(&self) -> &TG1_T0_INT_MAP {
        &self.tg1_t0_int_map
    }
    ///0xd8 - tg1_t1 interrupt configuration register
    #[inline(always)]
    pub const fn tg1_t1_int_map(&self) -> &TG1_T1_INT_MAP {
        &self.tg1_t1_int_map
    }
    ///0xdc - tg1_wdt interrupt configuration register
    #[inline(always)]
    pub const fn tg1_wdt_int_map(&self) -> &TG1_WDT_INT_MAP {
        &self.tg1_wdt_int_map
    }
    ///0xe0 - cache_ia interrupt configuration register
    #[inline(always)]
    pub const fn cache_ia_int_map(&self) -> &CACHE_IA_INT_MAP {
        &self.cache_ia_int_map
    }
    ///0xe4 - systimer_target0 interrupt configuration register
    #[inline(always)]
    pub const fn systimer_target0_int_map(&self) -> &SYSTIMER_TARGET0_INT_MAP {
        &self.systimer_target0_int_map
    }
    ///0xe8 - systimer_target1 interrupt configuration register
    #[inline(always)]
    pub const fn systimer_target1_int_map(&self) -> &SYSTIMER_TARGET1_INT_MAP {
        &self.systimer_target1_int_map
    }
    ///0xec - systimer_target2 interrupt configuration register
    #[inline(always)]
    pub const fn systimer_target2_int_map(&self) -> &SYSTIMER_TARGET2_INT_MAP {
        &self.systimer_target2_int_map
    }
    ///0xf0 - spi_mem_reject interrupt configuration register
    #[inline(always)]
    pub const fn spi_mem_reject_intr_map(&self) -> &SPI_MEM_REJECT_INTR_MAP {
        &self.spi_mem_reject_intr_map
    }
    ///0xf4 - dcache_prelaod interrupt configuration register
    #[inline(always)]
    pub const fn dcache_preload_int_map(&self) -> &DCACHE_PRELOAD_INT_MAP {
        &self.dcache_preload_int_map
    }
    ///0xf8 - icache_preload interrupt configuration register
    #[inline(always)]
    pub const fn icache_preload_int_map(&self) -> &ICACHE_PRELOAD_INT_MAP {
        &self.icache_preload_int_map
    }
    ///0xfc - dcache_sync interrupt configuration register
    #[inline(always)]
    pub const fn dcache_sync_int_map(&self) -> &DCACHE_SYNC_INT_MAP {
        &self.dcache_sync_int_map
    }
    ///0x100 - icache_sync interrupt configuration register
    #[inline(always)]
    pub const fn icache_sync_int_map(&self) -> &ICACHE_SYNC_INT_MAP {
        &self.icache_sync_int_map
    }
    ///0x104 - apb_adc interrupt configuration register
    #[inline(always)]
    pub const fn apb_adc_int_map(&self) -> &APB_ADC_INT_MAP {
        &self.apb_adc_int_map
    }
    ///0x108 - dma_in_ch0 interrupt configuration register
    #[inline(always)]
    pub const fn dma_in_ch0_int_map(&self) -> &DMA_IN_CH0_INT_MAP {
        &self.dma_in_ch0_int_map
    }
    ///0x10c - dma_in_ch1 interrupt configuration register
    #[inline(always)]
    pub const fn dma_in_ch1_int_map(&self) -> &DMA_IN_CH1_INT_MAP {
        &self.dma_in_ch1_int_map
    }
    ///0x110 - dma_in_ch2 interrupt configuration register
    #[inline(always)]
    pub const fn dma_in_ch2_int_map(&self) -> &DMA_IN_CH2_INT_MAP {
        &self.dma_in_ch2_int_map
    }
    ///0x114 - dma_in_ch3 interrupt configuration register
    #[inline(always)]
    pub const fn dma_in_ch3_int_map(&self) -> &DMA_IN_CH3_INT_MAP {
        &self.dma_in_ch3_int_map
    }
    ///0x118 - dma_in_ch4 interrupt configuration register
    #[inline(always)]
    pub const fn dma_in_ch4_int_map(&self) -> &DMA_IN_CH4_INT_MAP {
        &self.dma_in_ch4_int_map
    }
    ///0x11c - dma_out_ch0 interrupt configuration register
    #[inline(always)]
    pub const fn dma_out_ch0_int_map(&self) -> &DMA_OUT_CH0_INT_MAP {
        &self.dma_out_ch0_int_map
    }
    ///0x120 - dma_out_ch1 interrupt configuration register
    #[inline(always)]
    pub const fn dma_out_ch1_int_map(&self) -> &DMA_OUT_CH1_INT_MAP {
        &self.dma_out_ch1_int_map
    }
    ///0x124 - dma_out_ch2 interrupt configuration register
    #[inline(always)]
    pub const fn dma_out_ch2_int_map(&self) -> &DMA_OUT_CH2_INT_MAP {
        &self.dma_out_ch2_int_map
    }
    ///0x128 - dma_out_ch3 interrupt configuration register
    #[inline(always)]
    pub const fn dma_out_ch3_int_map(&self) -> &DMA_OUT_CH3_INT_MAP {
        &self.dma_out_ch3_int_map
    }
    ///0x12c - dma_out_ch4 interrupt configuration register
    #[inline(always)]
    pub const fn dma_out_ch4_int_map(&self) -> &DMA_OUT_CH4_INT_MAP {
        &self.dma_out_ch4_int_map
    }
    ///0x130 - rsa interrupt configuration register
    #[inline(always)]
    pub const fn rsa_int_map(&self) -> &RSA_INT_MAP {
        &self.rsa_int_map
    }
    ///0x134 - aes interrupt configuration register
    #[inline(always)]
    pub const fn aes_int_map(&self) -> &AES_INT_MAP {
        &self.aes_int_map
    }
    ///0x138 - sha interrupt configuration register
    #[inline(always)]
    pub const fn sha_int_map(&self) -> &SHA_INT_MAP {
        &self.sha_int_map
    }
    ///0x13c - cpu_intr_from_cpu_0 interrupt configuration register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0_map(&self) -> &CPU_INTR_FROM_CPU_0_MAP {
        &self.cpu_intr_from_cpu_0_map
    }
    ///0x140 - cpu_intr_from_cpu_1 interrupt configuration register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1_map(&self) -> &CPU_INTR_FROM_CPU_1_MAP {
        &self.cpu_intr_from_cpu_1_map
    }
    ///0x144 - cpu_intr_from_cpu_2 interrupt configuration register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2_map(&self) -> &CPU_INTR_FROM_CPU_2_MAP {
        &self.cpu_intr_from_cpu_2_map
    }
    ///0x148 - cpu_intr_from_cpu_3 interrupt configuration register
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3_map(&self) -> &CPU_INTR_FROM_CPU_3_MAP {
        &self.cpu_intr_from_cpu_3_map
    }
    ///0x14c - assist_debug interrupt configuration register
    #[inline(always)]
    pub const fn assist_debug_intr_map(&self) -> &ASSIST_DEBUG_INTR_MAP {
        &self.assist_debug_intr_map
    }
    ///0x150 - dma_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn dma_apbperi_pms_monitor_violate_intr_map(
        &self,
    ) -> &DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.dma_apbperi_pms_monitor_violate_intr_map
    }
    ///0x154 - core0_IRam0_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_0_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_iram0_pms_monitor_violate_intr_map
    }
    ///0x158 - core0_DRam0_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_0_dram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_dram0_pms_monitor_violate_intr_map
    }
    ///0x15c - core0_PIF_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_0_pif_pms_monitor_violate_intr_map
    }
    ///0x160 - core0_PIF_pms_monitor_violatile_size interrupt configuration register
    #[inline(always)]
    pub const fn core_0_pif_pms_monitor_violate_size_intr_map(
        &self,
    ) -> &CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {
        &self.core_0_pif_pms_monitor_violate_size_intr_map
    }
    ///0x164 - core1_IRam0_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_1_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_1_iram0_pms_monitor_violate_intr_map
    }
    ///0x168 - core1_DRam0_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_1_dram0_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_1_dram0_pms_monitor_violate_intr_map
    }
    ///0x16c - core1_PIF_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_violate_intr_map(
        &self,
    ) -> &CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {
        &self.core_1_pif_pms_monitor_violate_intr_map
    }
    ///0x170 - core1_PIF_pms_monitor_violatile_size interrupt configuration register
    #[inline(always)]
    pub const fn core_1_pif_pms_monitor_violate_size_intr_map(
        &self,
    ) -> &CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {
        &self.core_1_pif_pms_monitor_violate_size_intr_map
    }
    ///0x174 - backup_pms_monitor_violatile interrupt configuration register
    #[inline(always)]
    pub const fn backup_pms_violate_intr_map(&self) -> &BACKUP_PMS_VIOLATE_INTR_MAP {
        &self.backup_pms_violate_intr_map
    }
    ///0x178 - cache_core0_acs interrupt configuration register
    #[inline(always)]
    pub const fn cache_core0_acs_int_map(&self) -> &CACHE_CORE0_ACS_INT_MAP {
        &self.cache_core0_acs_int_map
    }
    ///0x17c - cache_core1_acs interrupt configuration register
    #[inline(always)]
    pub const fn cache_core1_acs_int_map(&self) -> &CACHE_CORE1_ACS_INT_MAP {
        &self.cache_core1_acs_int_map
    }
    ///0x180 - usb_device interrupt configuration register
    #[inline(always)]
    pub const fn usb_device_int_map(&self) -> &USB_DEVICE_INT_MAP {
        &self.usb_device_int_map
    }
    ///0x184 - peri_backup interrupt configuration register
    #[inline(always)]
    pub const fn peri_backup_int_map(&self) -> &PERI_BACKUP_INT_MAP {
        &self.peri_backup_int_map
    }
    ///0x188 - dma_extmem_reject interrupt configuration register
    #[inline(always)]
    pub const fn dma_extmem_reject_int_map(&self) -> &DMA_EXTMEM_REJECT_INT_MAP {
        &self.dma_extmem_reject_int_map
    }
    ///0x18c - interrupt status register
    #[inline(always)]
    pub const fn pro_intr_status_0(&self) -> &PRO_INTR_STATUS_0 {
        &self.pro_intr_status_0
    }
    ///0x190 - interrupt status register
    #[inline(always)]
    pub const fn pro_intr_status_1(&self) -> &PRO_INTR_STATUS_1 {
        &self.pro_intr_status_1
    }
    ///0x194 - interrupt status register
    #[inline(always)]
    pub const fn pro_intr_status_2(&self) -> &PRO_INTR_STATUS_2 {
        &self.pro_intr_status_2
    }
    ///0x198 - interrupt status register
    #[inline(always)]
    pub const fn pro_intr_status_3(&self) -> &PRO_INTR_STATUS_3 {
        &self.pro_intr_status_3
    }
    ///0x19c - clock gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x7fc - version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PRO_MAC_INTR_MAP (rw) register accessor: mac interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_mac_intr_map`] module*/
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
///mac interrupt configuration register
pub mod pro_mac_intr_map;
/**MAC_NMI_MAP (rw) register accessor: mac_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mac_nmi_map`] module*/
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
///mac_nmi interrupt configuration register
pub mod mac_nmi_map;
/**PWR_INTR_MAP (rw) register accessor: pwr interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwr_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwr_intr_map`] module*/
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
///pwr interrupt configuration register
pub mod pwr_intr_map;
/**BB_INT_MAP (rw) register accessor: bb interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bb_int_map`] module*/
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
///bb interrupt configuration register
pub mod bb_int_map;
/**BT_MAC_INT_MAP (rw) register accessor: bb_mac interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_mac_int_map`] module*/
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
///bb_mac interrupt configuration register
pub mod bt_mac_int_map;
/**BT_BB_INT_MAP (rw) register accessor: bt_bb interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_bb_int_map`] module*/
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
///bt_bb interrupt configuration register
pub mod bt_bb_int_map;
/**BT_BB_NMI_MAP (rw) register accessor: bt_bb_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_bb_nmi_map`] module*/
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
///bt_bb_nmi interrupt configuration register
pub mod bt_bb_nmi_map;
/**RWBT_IRQ_MAP (rw) register accessor: rwbt_irq interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwbt_irq_map`] module*/
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
///rwbt_irq interrupt configuration register
pub mod rwbt_irq_map;
/**RWBLE_IRQ_MAP (rw) register accessor: rwble_irq interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwble_irq_map`] module*/
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
///rwble_irq interrupt configuration register
pub mod rwble_irq_map;
/**RWBT_NMI_MAP (rw) register accessor: rwbt_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwbt_nmi_map`] module*/
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
///rwbt_nmi interrupt configuration register
pub mod rwbt_nmi_map;
/**RWBLE_NMI_MAP (rw) register accessor: rwble_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwble_nmi_map`] module*/
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
///rwble_nmi interrupt configuration register
pub mod rwble_nmi_map;
/**I2C_MST_INT_MAP (rw) register accessor: i2c_mst interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c_mst_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_mst_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c_mst_int_map`] module*/
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
///i2c_mst interrupt configuration register
pub mod i2c_mst_int_map;
/**SLC0_INTR_MAP (rw) register accessor: slc0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slc0_intr_map`] module*/
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
///slc0 interrupt configuration register
pub mod slc0_intr_map;
/**SLC1_INTR_MAP (rw) register accessor: slc1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slc1_intr_map`] module*/
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
///slc1 interrupt configuration register
pub mod slc1_intr_map;
/**UHCI0_INTR_MAP (rw) register accessor: uhci0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uhci0_intr_map`] module*/
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
///uhci0 interrupt configuration register
pub mod uhci0_intr_map;
/**UHCI1_INTR_MAP (rw) register accessor: uhci1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uhci1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uhci1_intr_map`] module*/
pub type UHCI1_INTR_MAP = crate::Reg<uhci1_intr_map::UHCI1_INTR_MAP_SPEC>;
///uhci1 interrupt configuration register
pub mod uhci1_intr_map;
/**GPIO_INTERRUPT_PRO_MAP (rw) register accessor: gpio_interrupt_pro interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio_interrupt_pro_map`] module*/
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
///gpio_interrupt_pro interrupt configuration register
pub mod gpio_interrupt_pro_map;
/**GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: gpio_interrupt_pro_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio_interrupt_pro_nmi_map`] module*/
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
///gpio_interrupt_pro_nmi interrupt configuration register
pub mod gpio_interrupt_pro_nmi_map;
/**GPIO_INTERRUPT_APP_MAP (rw) register accessor: gpio_interrupt_app interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_app_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_app_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio_interrupt_app_map`] module*/
pub type GPIO_INTERRUPT_APP_MAP = crate::Reg<gpio_interrupt_app_map::GPIO_INTERRUPT_APP_MAP_SPEC>;
///gpio_interrupt_app interrupt configuration register
pub mod gpio_interrupt_app_map;
/**GPIO_INTERRUPT_APP_NMI_MAP (rw) register accessor: gpio_interrupt_app_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_app_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_app_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio_interrupt_app_nmi_map`] module*/
pub type GPIO_INTERRUPT_APP_NMI_MAP =
    crate::Reg<gpio_interrupt_app_nmi_map::GPIO_INTERRUPT_APP_NMI_MAP_SPEC>;
///gpio_interrupt_app_nmi interrupt configuration register
pub mod gpio_interrupt_app_nmi_map;
/**SPI_INTR_1_MAP (rw) register accessor: spi_intr_1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_intr_1_map`] module*/
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
///spi_intr_1 interrupt configuration register
pub mod spi_intr_1_map;
/**SPI_INTR_2_MAP (rw) register accessor: spi_intr_2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_intr_2_map`] module*/
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
///spi_intr_2 interrupt configuration register
pub mod spi_intr_2_map;
/**SPI_INTR_3_MAP (rw) register accessor: spi_intr_3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_intr_3_map`] module*/
pub type SPI_INTR_3_MAP = crate::Reg<spi_intr_3_map::SPI_INTR_3_MAP_SPEC>;
///spi_intr_3 interrupt configuration register
pub mod spi_intr_3_map;
/**SPI_INTR_4_MAP (rw) register accessor: spi_intr_4 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_intr_4_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_intr_4_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_intr_4_map`] module*/
pub type SPI_INTR_4_MAP = crate::Reg<spi_intr_4_map::SPI_INTR_4_MAP_SPEC>;
///spi_intr_4 interrupt configuration register
pub mod spi_intr_4_map;
/**LCD_CAM_INT_MAP (rw) register accessor: lcd_cam interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_cam_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cam_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_cam_int_map`] module*/
pub type LCD_CAM_INT_MAP = crate::Reg<lcd_cam_int_map::LCD_CAM_INT_MAP_SPEC>;
///lcd_cam interrupt configuration register
pub mod lcd_cam_int_map;
/**I2S0_INT_MAP (rw) register accessor: i2s0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s0_int_map`] module*/
pub type I2S0_INT_MAP = crate::Reg<i2s0_int_map::I2S0_INT_MAP_SPEC>;
///i2s0 interrupt configuration register
pub mod i2s0_int_map;
/**I2S1_INT_MAP (rw) register accessor: i2s1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s1_int_map`] module*/
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
///i2s1 interrupt configuration register
pub mod i2s1_int_map;
/**UART_INTR_MAP (rw) register accessor: uart interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart_intr_map`] module*/
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
///uart interrupt configuration register
pub mod uart_intr_map;
/**UART1_INTR_MAP (rw) register accessor: uart1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart1_intr_map`] module*/
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
///uart1 interrupt configuration register
pub mod uart1_intr_map;
/**UART2_INTR_MAP (rw) register accessor: uart2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart2_intr_map`] module*/
pub type UART2_INTR_MAP = crate::Reg<uart2_intr_map::UART2_INTR_MAP_SPEC>;
///uart2 interrupt configuration register
pub mod uart2_intr_map;
/**SDIO_HOST_INTERRUPT_MAP (rw) register accessor: sdio_host interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_host_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_host_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_host_interrupt_map`] module*/
pub type SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<sdio_host_interrupt_map::SDIO_HOST_INTERRUPT_MAP_SPEC>;
///sdio_host interrupt configuration register
pub mod sdio_host_interrupt_map;
/**PWM0_INTR_MAP (rw) register accessor: pwm0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm0_intr_map`] module*/
pub type PWM0_INTR_MAP = crate::Reg<pwm0_intr_map::PWM0_INTR_MAP_SPEC>;
///pwm0 interrupt configuration register
pub mod pwm0_intr_map;
/**PWM1_INTR_MAP (rw) register accessor: pwm1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm1_intr_map`] module*/
pub type PWM1_INTR_MAP = crate::Reg<pwm1_intr_map::PWM1_INTR_MAP_SPEC>;
///pwm1 interrupt configuration register
pub mod pwm1_intr_map;
/**PWM2_INTR_MAP (rw) register accessor: pwm2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm2_intr_map`] module*/
pub type PWM2_INTR_MAP = crate::Reg<pwm2_intr_map::PWM2_INTR_MAP_SPEC>;
///pwm2 interrupt configuration register
pub mod pwm2_intr_map;
/**PWM3_INTR_MAP (rw) register accessor: pwm3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm3_intr_map`] module*/
pub type PWM3_INTR_MAP = crate::Reg<pwm3_intr_map::PWM3_INTR_MAP_SPEC>;
///pwm3 interrupt configuration register
pub mod pwm3_intr_map;
/**LEDC_INT_MAP (rw) register accessor: ledc interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ledc_int_map`] module*/
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
///ledc interrupt configuration register
pub mod ledc_int_map;
/**EFUSE_INT_MAP (rw) register accessor: efuse interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@efuse_int_map`] module*/
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
///efuse interrupt configuration register
pub mod efuse_int_map;
/**CAN_INT_MAP (rw) register accessor: can interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@can_int_map`] module*/
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
///can interrupt configuration register
pub mod can_int_map;
/**USB_INTR_MAP (rw) register accessor: usb interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb_intr_map`] module*/
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
///usb interrupt configuration register
pub mod usb_intr_map;
/**RTC_CORE_INTR_MAP (rw) register accessor: rtc_core interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_core_intr_map`] module*/
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
///rtc_core interrupt configuration register
pub mod rtc_core_intr_map;
/**RMT_INTR_MAP (rw) register accessor: rmt interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmt_intr_map`] module*/
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
///rmt interrupt configuration register
pub mod rmt_intr_map;
/**PCNT_INTR_MAP (rw) register accessor: pcnt interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcnt_intr_map`] module*/
pub type PCNT_INTR_MAP = crate::Reg<pcnt_intr_map::PCNT_INTR_MAP_SPEC>;
///pcnt interrupt configuration register
pub mod pcnt_intr_map;
/**I2C_EXT0_INTR_MAP (rw) register accessor: i2c_ext0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c_ext0_intr_map`] module*/
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
///i2c_ext0 interrupt configuration register
pub mod i2c_ext0_intr_map;
/**I2C_EXT1_INTR_MAP (rw) register accessor: i2c_ext1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c_ext1_intr_map`] module*/
pub type I2C_EXT1_INTR_MAP = crate::Reg<i2c_ext1_intr_map::I2C_EXT1_INTR_MAP_SPEC>;
///i2c_ext1 interrupt configuration register
pub mod i2c_ext1_intr_map;
/**SPI2_DMA_INT_MAP (rw) register accessor: spi2_dma interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi2_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi2_dma_int_map`] module*/
pub type SPI2_DMA_INT_MAP = crate::Reg<spi2_dma_int_map::SPI2_DMA_INT_MAP_SPEC>;
///spi2_dma interrupt configuration register
pub mod spi2_dma_int_map;
/**SPI3_DMA_INT_MAP (rw) register accessor: spi3_dma interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi3_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi3_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi3_dma_int_map`] module*/
pub type SPI3_DMA_INT_MAP = crate::Reg<spi3_dma_int_map::SPI3_DMA_INT_MAP_SPEC>;
///spi3_dma interrupt configuration register
pub mod spi3_dma_int_map;
/**SPI4_DMA_INT_MAP (rw) register accessor: spi4_dma interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi4_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi4_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi4_dma_int_map`] module*/
pub type SPI4_DMA_INT_MAP = crate::Reg<spi4_dma_int_map::SPI4_DMA_INT_MAP_SPEC>;
///spi4_dma interrupt configuration register
pub mod spi4_dma_int_map;
/**WDG_INT_MAP (rw) register accessor: wdg interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`wdg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdg_int_map`] module*/
pub type WDG_INT_MAP = crate::Reg<wdg_int_map::WDG_INT_MAP_SPEC>;
///wdg interrupt configuration register
pub mod wdg_int_map;
/**TIMER_INT1_MAP (rw) register accessor: timer_int1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_int1_map`] module*/
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
///timer_int1 interrupt configuration register
pub mod timer_int1_map;
/**TIMER_INT2_MAP (rw) register accessor: timer_int2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer_int2_map`] module*/
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
///timer_int2 interrupt configuration register
pub mod timer_int2_map;
/**TG_T0_INT_MAP (rw) register accessor: tg_t0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg_t0_int_map`] module*/
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
///tg_t0 interrupt configuration register
pub mod tg_t0_int_map;
/**TG_T1_INT_MAP (rw) register accessor: tg_t1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg_t1_int_map`] module*/
pub type TG_T1_INT_MAP = crate::Reg<tg_t1_int_map::TG_T1_INT_MAP_SPEC>;
///tg_t1 interrupt configuration register
pub mod tg_t1_int_map;
/**TG_WDT_INT_MAP (rw) register accessor: tg_wdt interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg_wdt_int_map`] module*/
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
///tg_wdt interrupt configuration register
pub mod tg_wdt_int_map;
/**TG1_T0_INT_MAP (rw) register accessor: tg1_t0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg1_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg1_t0_int_map`] module*/
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
///tg1_t0 interrupt configuration register
pub mod tg1_t0_int_map;
/**TG1_T1_INT_MAP (rw) register accessor: tg1_t1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg1_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg1_t1_int_map`] module*/
pub type TG1_T1_INT_MAP = crate::Reg<tg1_t1_int_map::TG1_T1_INT_MAP_SPEC>;
///tg1_t1 interrupt configuration register
pub mod tg1_t1_int_map;
/**TG1_WDT_INT_MAP (rw) register accessor: tg1_wdt interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tg1_wdt_int_map`] module*/
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
///tg1_wdt interrupt configuration register
pub mod tg1_wdt_int_map;
/**CACHE_IA_INT_MAP (rw) register accessor: cache_ia interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_ia_int_map`] module*/
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
///cache_ia interrupt configuration register
pub mod cache_ia_int_map;
/**SYSTIMER_TARGET0_INT_MAP (rw) register accessor: systimer_target0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@systimer_target0_int_map`] module*/
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
///systimer_target0 interrupt configuration register
pub mod systimer_target0_int_map;
/**SYSTIMER_TARGET1_INT_MAP (rw) register accessor: systimer_target1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@systimer_target1_int_map`] module*/
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
///systimer_target1 interrupt configuration register
pub mod systimer_target1_int_map;
/**SYSTIMER_TARGET2_INT_MAP (rw) register accessor: systimer_target2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@systimer_target2_int_map`] module*/
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
///systimer_target2 interrupt configuration register
pub mod systimer_target2_int_map;
/**SPI_MEM_REJECT_INTR_MAP (rw) register accessor: spi_mem_reject interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_reject_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_mem_reject_intr_map`] module*/
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
///spi_mem_reject interrupt configuration register
pub mod spi_mem_reject_intr_map;
/**DCACHE_PRELOAD_INT_MAP (rw) register accessor: dcache_prelaod interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dcache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcache_preload_int_map`] module*/
pub type DCACHE_PRELOAD_INT_MAP = crate::Reg<dcache_preload_int_map::DCACHE_PRELOAD_INT_MAP_SPEC>;
///dcache_prelaod interrupt configuration register
pub mod dcache_preload_int_map;
/**ICACHE_PRELOAD_INT_MAP (rw) register accessor: icache_preload interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`icache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icache_preload_int_map`] module*/
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
///icache_preload interrupt configuration register
pub mod icache_preload_int_map;
/**DCACHE_SYNC_INT_MAP (rw) register accessor: dcache_sync interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcache_sync_int_map`] module*/
pub type DCACHE_SYNC_INT_MAP = crate::Reg<dcache_sync_int_map::DCACHE_SYNC_INT_MAP_SPEC>;
///dcache_sync interrupt configuration register
pub mod dcache_sync_int_map;
/**ICACHE_SYNC_INT_MAP (rw) register accessor: icache_sync interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`icache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icache_sync_int_map`] module*/
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
///icache_sync interrupt configuration register
pub mod icache_sync_int_map;
/**APB_ADC_INT_MAP (rw) register accessor: apb_adc interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@apb_adc_int_map`] module*/
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
///apb_adc interrupt configuration register
pub mod apb_adc_int_map;
/**DMA_IN_CH0_INT_MAP (rw) register accessor: dma_in_ch0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_ch0_int_map`] module*/
pub type DMA_IN_CH0_INT_MAP = crate::Reg<dma_in_ch0_int_map::DMA_IN_CH0_INT_MAP_SPEC>;
///dma_in_ch0 interrupt configuration register
pub mod dma_in_ch0_int_map;
/**DMA_IN_CH1_INT_MAP (rw) register accessor: dma_in_ch1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_ch1_int_map`] module*/
pub type DMA_IN_CH1_INT_MAP = crate::Reg<dma_in_ch1_int_map::DMA_IN_CH1_INT_MAP_SPEC>;
///dma_in_ch1 interrupt configuration register
pub mod dma_in_ch1_int_map;
/**DMA_IN_CH2_INT_MAP (rw) register accessor: dma_in_ch2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_ch2_int_map`] module*/
pub type DMA_IN_CH2_INT_MAP = crate::Reg<dma_in_ch2_int_map::DMA_IN_CH2_INT_MAP_SPEC>;
///dma_in_ch2 interrupt configuration register
pub mod dma_in_ch2_int_map;
/**DMA_IN_CH3_INT_MAP (rw) register accessor: dma_in_ch3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_ch3_int_map`] module*/
pub type DMA_IN_CH3_INT_MAP = crate::Reg<dma_in_ch3_int_map::DMA_IN_CH3_INT_MAP_SPEC>;
///dma_in_ch3 interrupt configuration register
pub mod dma_in_ch3_int_map;
/**DMA_IN_CH4_INT_MAP (rw) register accessor: dma_in_ch4 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_ch4_int_map`] module*/
pub type DMA_IN_CH4_INT_MAP = crate::Reg<dma_in_ch4_int_map::DMA_IN_CH4_INT_MAP_SPEC>;
///dma_in_ch4 interrupt configuration register
pub mod dma_in_ch4_int_map;
/**DMA_OUT_CH0_INT_MAP (rw) register accessor: dma_out_ch0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_ch0_int_map`] module*/
pub type DMA_OUT_CH0_INT_MAP = crate::Reg<dma_out_ch0_int_map::DMA_OUT_CH0_INT_MAP_SPEC>;
///dma_out_ch0 interrupt configuration register
pub mod dma_out_ch0_int_map;
/**DMA_OUT_CH1_INT_MAP (rw) register accessor: dma_out_ch1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_ch1_int_map`] module*/
pub type DMA_OUT_CH1_INT_MAP = crate::Reg<dma_out_ch1_int_map::DMA_OUT_CH1_INT_MAP_SPEC>;
///dma_out_ch1 interrupt configuration register
pub mod dma_out_ch1_int_map;
/**DMA_OUT_CH2_INT_MAP (rw) register accessor: dma_out_ch2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_ch2_int_map`] module*/
pub type DMA_OUT_CH2_INT_MAP = crate::Reg<dma_out_ch2_int_map::DMA_OUT_CH2_INT_MAP_SPEC>;
///dma_out_ch2 interrupt configuration register
pub mod dma_out_ch2_int_map;
/**DMA_OUT_CH3_INT_MAP (rw) register accessor: dma_out_ch3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_ch3_int_map`] module*/
pub type DMA_OUT_CH3_INT_MAP = crate::Reg<dma_out_ch3_int_map::DMA_OUT_CH3_INT_MAP_SPEC>;
///dma_out_ch3 interrupt configuration register
pub mod dma_out_ch3_int_map;
/**DMA_OUT_CH4_INT_MAP (rw) register accessor: dma_out_ch4 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_ch4_int_map`] module*/
pub type DMA_OUT_CH4_INT_MAP = crate::Reg<dma_out_ch4_int_map::DMA_OUT_CH4_INT_MAP_SPEC>;
///dma_out_ch4 interrupt configuration register
pub mod dma_out_ch4_int_map;
/**RSA_INT_MAP (rw) register accessor: rsa interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsa_int_map`] module*/
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
///rsa interrupt configuration register
pub mod rsa_int_map;
/**AES_INT_MAP (rw) register accessor: aes interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`aes_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@aes_int_map`] module*/
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
///aes interrupt configuration register
pub mod aes_int_map;
/**SHA_INT_MAP (rw) register accessor: sha interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sha_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_int_map`] module*/
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
///sha interrupt configuration register
pub mod sha_int_map;
/**CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: cpu_intr_from_cpu_0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_0_map`] module*/
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
///cpu_intr_from_cpu_0 interrupt configuration register
pub mod cpu_intr_from_cpu_0_map;
/**CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: cpu_intr_from_cpu_1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_1_map`] module*/
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
///cpu_intr_from_cpu_1 interrupt configuration register
pub mod cpu_intr_from_cpu_1_map;
/**CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: cpu_intr_from_cpu_2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_2_map`] module*/
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
///cpu_intr_from_cpu_2 interrupt configuration register
pub mod cpu_intr_from_cpu_2_map;
/**CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: cpu_intr_from_cpu_3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_3_map`] module*/
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
///cpu_intr_from_cpu_3 interrupt configuration register
pub mod cpu_intr_from_cpu_3_map;
/**ASSIST_DEBUG_INTR_MAP (rw) register accessor: assist_debug interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@assist_debug_intr_map`] module*/
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
///assist_debug interrupt configuration register
pub mod assist_debug_intr_map;
/**DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: dma_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_apbperi_pms_monitor_violate_intr_map`] module*/
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///dma_pms_monitor_violatile interrupt configuration register
pub mod dma_apbperi_pms_monitor_violate_intr_map;
/**CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_IRam0_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_iram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_iram0_pms_monitor_violate_intr_map`] module*/
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core0_IRam0_pms_monitor_violatile interrupt configuration register
pub mod core_0_iram0_pms_monitor_violate_intr_map;
/**CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_DRam0_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_dram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_dram0_pms_monitor_violate_intr_map`] module*/
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core0_DRam0_pms_monitor_violatile interrupt configuration register
pub mod core_0_dram0_pms_monitor_violate_intr_map;
/**CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core0_PIF_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_pif_pms_monitor_violate_intr_map`] module*/
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core0_PIF_pms_monitor_violatile interrupt configuration register
pub mod core_0_pif_pms_monitor_violate_intr_map;
/**CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: core0_PIF_pms_monitor_violatile_size interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_monitor_violate_size_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_monitor_violate_size_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_pif_pms_monitor_violate_size_intr_map`] module*/
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
///core0_PIF_pms_monitor_violatile_size interrupt configuration register
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
/**CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_IRam0_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_1_iram0_pms_monitor_violate_intr_map`] module*/
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_iram0_pms_monitor_violate_intr_map::CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core1_IRam0_pms_monitor_violatile interrupt configuration register
pub mod core_1_iram0_pms_monitor_violate_intr_map;
/**CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_DRam0_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_dram0_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_1_dram0_pms_monitor_violate_intr_map`] module*/
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_dram0_pms_monitor_violate_intr_map::CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core1_DRam0_pms_monitor_violatile interrupt configuration register
pub mod core_1_dram0_pms_monitor_violate_intr_map;
/**CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP (rw) register accessor: core1_PIF_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_1_pif_pms_monitor_violate_intr_map`] module*/
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
///core1_PIF_pms_monitor_violatile interrupt configuration register
pub mod core_1_pif_pms_monitor_violate_intr_map;
/**CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP (rw) register accessor: core1_PIF_pms_monitor_violatile_size interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_violate_size_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_violate_size_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_1_pif_pms_monitor_violate_size_intr_map`] module*/
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_1_pif_pms_monitor_violate_size_intr_map::CORE_1_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
///core1_PIF_pms_monitor_violatile_size interrupt configuration register
pub mod core_1_pif_pms_monitor_violate_size_intr_map;
/**BACKUP_PMS_VIOLATE_INTR_MAP (rw) register accessor: backup_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`backup_pms_violate_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_pms_violate_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@backup_pms_violate_intr_map`] module*/
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
///backup_pms_monitor_violatile interrupt configuration register
pub mod backup_pms_violate_intr_map;
/**CACHE_CORE0_ACS_INT_MAP (rw) register accessor: cache_core0_acs interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_core0_acs_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_core0_acs_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_core0_acs_int_map`] module*/
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
///cache_core0_acs interrupt configuration register
pub mod cache_core0_acs_int_map;
/**CACHE_CORE1_ACS_INT_MAP (rw) register accessor: cache_core1_acs interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_core1_acs_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_core1_acs_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_core1_acs_int_map`] module*/
pub type CACHE_CORE1_ACS_INT_MAP =
    crate::Reg<cache_core1_acs_int_map::CACHE_CORE1_ACS_INT_MAP_SPEC>;
///cache_core1_acs interrupt configuration register
pub mod cache_core1_acs_int_map;
/**USB_DEVICE_INT_MAP (rw) register accessor: usb_device interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_device_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_device_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb_device_int_map`] module*/
pub type USB_DEVICE_INT_MAP = crate::Reg<usb_device_int_map::USB_DEVICE_INT_MAP_SPEC>;
///usb_device interrupt configuration register
pub mod usb_device_int_map;
/**PERI_BACKUP_INT_MAP (rw) register accessor: peri_backup interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@peri_backup_int_map`] module*/
pub type PERI_BACKUP_INT_MAP = crate::Reg<peri_backup_int_map::PERI_BACKUP_INT_MAP_SPEC>;
///peri_backup interrupt configuration register
pub mod peri_backup_int_map;
/**DMA_EXTMEM_REJECT_INT_MAP (rw) register accessor: dma_extmem_reject interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_extmem_reject_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_extmem_reject_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_extmem_reject_int_map`] module*/
pub type DMA_EXTMEM_REJECT_INT_MAP =
    crate::Reg<dma_extmem_reject_int_map::DMA_EXTMEM_REJECT_INT_MAP_SPEC>;
///dma_extmem_reject interrupt configuration register
pub mod dma_extmem_reject_int_map;
/**PRO_INTR_STATUS_0 (r) register accessor: interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_0`] module*/
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
///interrupt status register
pub mod pro_intr_status_0;
/**PRO_INTR_STATUS_1 (r) register accessor: interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_1`] module*/
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
///interrupt status register
pub mod pro_intr_status_1;
/**PRO_INTR_STATUS_2 (r) register accessor: interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_2`] module*/
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
///interrupt status register
pub mod pro_intr_status_2;
/**PRO_INTR_STATUS_3 (r) register accessor: interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_3`] module*/
pub type PRO_INTR_STATUS_3 = crate::Reg<pro_intr_status_3::PRO_INTR_STATUS_3_SPEC>;
///interrupt status register
pub mod pro_intr_status_3;
/**CLOCK_GATE (rw) register accessor: clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///clock gate register
pub mod clock_gate;
/**DATE (rw) register accessor: version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///version register
pub mod date;
