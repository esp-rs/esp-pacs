#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pro_mac_intr_map: PRO_MAC_INTR_MAP,
    pro_mac_nmi_map: PRO_MAC_NMI_MAP,
    pro_pwr_intr_map: PRO_PWR_INTR_MAP,
    pro_bb_int_map: PRO_BB_INT_MAP,
    pro_bt_mac_int_map: PRO_BT_MAC_INT_MAP,
    pro_bt_bb_int_map: PRO_BT_BB_INT_MAP,
    pro_bt_bb_nmi_map: PRO_BT_BB_NMI_MAP,
    pro_rwbt_irq_map: PRO_RWBT_IRQ_MAP,
    pro_rwble_irq_map: PRO_RWBLE_IRQ_MAP,
    pro_rwbt_nmi_map: PRO_RWBT_NMI_MAP,
    pro_rwble_nmi_map: PRO_RWBLE_NMI_MAP,
    pro_slc0_intr_map: PRO_SLC0_INTR_MAP,
    pro_slc1_intr_map: PRO_SLC1_INTR_MAP,
    pro_uhci0_intr_map: PRO_UHCI0_INTR_MAP,
    pro_uhci1_intr_map: PRO_UHCI1_INTR_MAP,
    pro_tg_t0_level_int_map: PRO_TG_T0_LEVEL_INT_MAP,
    pro_tg_t1_level_int_map: PRO_TG_T1_LEVEL_INT_MAP,
    pro_tg_wdt_level_int_map: PRO_TG_WDT_LEVEL_INT_MAP,
    pro_tg_lact_level_int_map: PRO_TG_LACT_LEVEL_INT_MAP,
    pro_tg1_t0_level_int_map: PRO_TG1_T0_LEVEL_INT_MAP,
    pro_tg1_t1_level_int_map: PRO_TG1_T1_LEVEL_INT_MAP,
    pro_tg1_wdt_level_int_map: PRO_TG1_WDT_LEVEL_INT_MAP,
    pro_tg1_lact_level_int_map: PRO_TG1_LACT_LEVEL_INT_MAP,
    pro_gpio_interrupt_pro_map: PRO_GPIO_INTERRUPT_PRO_MAP,
    pro_gpio_interrupt_pro_nmi_map: PRO_GPIO_INTERRUPT_PRO_NMI_MAP,
    pro_gpio_interrupt_app_map: PRO_GPIO_INTERRUPT_APP_MAP,
    pro_gpio_interrupt_app_nmi_map: PRO_GPIO_INTERRUPT_APP_NMI_MAP,
    pro_dedicated_gpio_in_intr_map: PRO_DEDICATED_GPIO_IN_INTR_MAP,
    pro_cpu_intr_from_cpu_0_map: PRO_CPU_INTR_FROM_CPU_0_MAP,
    pro_cpu_intr_from_cpu_1_map: PRO_CPU_INTR_FROM_CPU_1_MAP,
    pro_cpu_intr_from_cpu_2_map: PRO_CPU_INTR_FROM_CPU_2_MAP,
    pro_cpu_intr_from_cpu_3_map: PRO_CPU_INTR_FROM_CPU_3_MAP,
    pro_spi_intr_1_map: PRO_SPI_INTR_1_MAP,
    pro_spi_intr_2_map: PRO_SPI_INTR_2_MAP,
    pro_spi_intr_3_map: PRO_SPI_INTR_3_MAP,
    pro_i2s0_int_map: PRO_I2S0_INT_MAP,
    pro_i2s1_int_map: PRO_I2S1_INT_MAP,
    pro_uart_intr_map: PRO_UART_INTR_MAP,
    pro_uart1_intr_map: PRO_UART1_INTR_MAP,
    pro_uart2_intr_map: PRO_UART2_INTR_MAP,
    pro_sdio_host_interrupt_map: PRO_SDIO_HOST_INTERRUPT_MAP,
    pro_pwm0_intr_map: PRO_PWM0_INTR_MAP,
    pro_pwm1_intr_map: PRO_PWM1_INTR_MAP,
    pro_pwm2_intr_map: PRO_PWM2_INTR_MAP,
    pro_pwm3_intr_map: PRO_PWM3_INTR_MAP,
    pro_ledc_int_map: PRO_LEDC_INT_MAP,
    pro_efuse_int_map: PRO_EFUSE_INT_MAP,
    pro_can_int_map: PRO_CAN_INT_MAP,
    pro_usb_intr_map: PRO_USB_INTR_MAP,
    pro_rtc_core_intr_map: PRO_RTC_CORE_INTR_MAP,
    pro_rmt_intr_map: PRO_RMT_INTR_MAP,
    pro_pcnt_intr_map: PRO_PCNT_INTR_MAP,
    pro_i2c_ext0_intr_map: PRO_I2C_EXT0_INTR_MAP,
    pro_i2c_ext1_intr_map: PRO_I2C_EXT1_INTR_MAP,
    pro_rsa_intr_map: PRO_RSA_INTR_MAP,
    pro_sha_intr_map: PRO_SHA_INTR_MAP,
    pro_aes_intr_map: PRO_AES_INTR_MAP,
    pro_spi2_dma_int_map: PRO_SPI2_DMA_INT_MAP,
    pro_spi3_dma_int_map: PRO_SPI3_DMA_INT_MAP,
    pro_wdg_int_map: PRO_WDG_INT_MAP,
    pro_timer_int1_map: PRO_TIMER_INT1_MAP,
    pro_timer_int2_map: PRO_TIMER_INT2_MAP,
    pro_tg_t0_edge_int_map: PRO_TG_T0_EDGE_INT_MAP,
    pro_tg_t1_edge_int_map: PRO_TG_T1_EDGE_INT_MAP,
    pro_tg_wdt_edge_int_map: PRO_TG_WDT_EDGE_INT_MAP,
    pro_tg_lact_edge_int_map: PRO_TG_LACT_EDGE_INT_MAP,
    pro_tg1_t0_edge_int_map: PRO_TG1_T0_EDGE_INT_MAP,
    pro_tg1_t1_edge_int_map: PRO_TG1_T1_EDGE_INT_MAP,
    pro_tg1_wdt_edge_int_map: PRO_TG1_WDT_EDGE_INT_MAP,
    pro_tg1_lact_edge_int_map: PRO_TG1_LACT_EDGE_INT_MAP,
    pro_cache_ia_int_map: PRO_CACHE_IA_INT_MAP,
    pro_systimer_target0_int_map: PRO_SYSTIMER_TARGET0_INT_MAP,
    pro_systimer_target1_int_map: PRO_SYSTIMER_TARGET1_INT_MAP,
    pro_systimer_target2_int_map: PRO_SYSTIMER_TARGET2_INT_MAP,
    pro_assist_debug_intr_map: PRO_ASSIST_DEBUG_INTR_MAP,
    pro_pms_pro_iram0_ilg_intr_map: PRO_PMS_PRO_IRAM0_ILG_INTR_MAP,
    pro_pms_pro_dram0_ilg_intr_map: PRO_PMS_PRO_DRAM0_ILG_INTR_MAP,
    pro_pms_pro_dport_ilg_intr_map: PRO_PMS_PRO_DPORT_ILG_INTR_MAP,
    pro_pms_pro_ahb_ilg_intr_map: PRO_PMS_PRO_AHB_ILG_INTR_MAP,
    pro_pms_pro_cache_ilg_intr_map: PRO_PMS_PRO_CACHE_ILG_INTR_MAP,
    pro_pms_dma_apb_i_ilg_intr_map: PRO_PMS_DMA_APB_I_ILG_INTR_MAP,
    pro_pms_dma_rx_i_ilg_intr_map: PRO_PMS_DMA_RX_I_ILG_INTR_MAP,
    pro_pms_dma_tx_i_ilg_intr_map: PRO_PMS_DMA_TX_I_ILG_INTR_MAP,
    pro_spi_mem_reject_intr_map: PRO_SPI_MEM_REJECT_INTR_MAP,
    pro_dma_copy_intr_map: PRO_DMA_COPY_INTR_MAP,
    pro_spi4_dma_int_map: PRO_SPI4_DMA_INT_MAP,
    pro_spi_intr_4_map: PRO_SPI_INTR_4_MAP,
    pro_dcache_preload_int_map: PRO_DCACHE_PRELOAD_INT_MAP,
    pro_icache_preload_int_map: PRO_ICACHE_PRELOAD_INT_MAP,
    pro_apb_adc_int_map: PRO_APB_ADC_INT_MAP,
    pro_crypto_dma_int_map: PRO_CRYPTO_DMA_INT_MAP,
    pro_cpu_peri_error_int_map: PRO_CPU_PERI_ERROR_INT_MAP,
    pro_apb_peri_error_int_map: PRO_APB_PERI_ERROR_INT_MAP,
    pro_dcache_sync_int_map: PRO_DCACHE_SYNC_INT_MAP,
    pro_icache_sync_int_map: PRO_ICACHE_SYNC_INT_MAP,
    pro_intr_status_0: PRO_INTR_STATUS_0,
    pro_intr_status_1: PRO_INTR_STATUS_1,
    pro_intr_status_2: PRO_INTR_STATUS_2,
    clock_gate: CLOCK_GATE,
    _reserved99: [u8; 0x0e70],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    ///0x00 - MAC_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_mac_intr_map(&self) -> &PRO_MAC_INTR_MAP {
        &self.pro_mac_intr_map
    }
    ///0x04 - MAC_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_mac_nmi_map(&self) -> &PRO_MAC_NMI_MAP {
        &self.pro_mac_nmi_map
    }
    ///0x08 - PWR_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pwr_intr_map(&self) -> &PRO_PWR_INTR_MAP {
        &self.pro_pwr_intr_map
    }
    ///0x0c - BB_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_bb_int_map(&self) -> &PRO_BB_INT_MAP {
        &self.pro_bb_int_map
    }
    ///0x10 - BT_MAC_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_bt_mac_int_map(&self) -> &PRO_BT_MAC_INT_MAP {
        &self.pro_bt_mac_int_map
    }
    ///0x14 - BT_BB_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_bt_bb_int_map(&self) -> &PRO_BT_BB_INT_MAP {
        &self.pro_bt_bb_int_map
    }
    ///0x18 - BT_BB_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_bt_bb_nmi_map(&self) -> &PRO_BT_BB_NMI_MAP {
        &self.pro_bt_bb_nmi_map
    }
    ///0x1c - RWBT_IRQ interrupt configuration register
    #[inline(always)]
    pub const fn pro_rwbt_irq_map(&self) -> &PRO_RWBT_IRQ_MAP {
        &self.pro_rwbt_irq_map
    }
    ///0x20 - RWBLE_IRQ interrupt configuration register
    #[inline(always)]
    pub const fn pro_rwble_irq_map(&self) -> &PRO_RWBLE_IRQ_MAP {
        &self.pro_rwble_irq_map
    }
    ///0x24 - RWBT_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_rwbt_nmi_map(&self) -> &PRO_RWBT_NMI_MAP {
        &self.pro_rwbt_nmi_map
    }
    ///0x28 - RWBLE_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_rwble_nmi_map(&self) -> &PRO_RWBLE_NMI_MAP {
        &self.pro_rwble_nmi_map
    }
    ///0x2c - SLC0_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_slc0_intr_map(&self) -> &PRO_SLC0_INTR_MAP {
        &self.pro_slc0_intr_map
    }
    ///0x30 - SLC1_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_slc1_intr_map(&self) -> &PRO_SLC1_INTR_MAP {
        &self.pro_slc1_intr_map
    }
    ///0x34 - UHCI0_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_uhci0_intr_map(&self) -> &PRO_UHCI0_INTR_MAP {
        &self.pro_uhci0_intr_map
    }
    ///0x38 - UHCI1_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_uhci1_intr_map(&self) -> &PRO_UHCI1_INTR_MAP {
        &self.pro_uhci1_intr_map
    }
    ///0x3c - TG_T0_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_t0_level_int_map(&self) -> &PRO_TG_T0_LEVEL_INT_MAP {
        &self.pro_tg_t0_level_int_map
    }
    ///0x40 - TG_T1_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_t1_level_int_map(&self) -> &PRO_TG_T1_LEVEL_INT_MAP {
        &self.pro_tg_t1_level_int_map
    }
    ///0x44 - TG_WDT_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_wdt_level_int_map(&self) -> &PRO_TG_WDT_LEVEL_INT_MAP {
        &self.pro_tg_wdt_level_int_map
    }
    ///0x48 - TG_LACT_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_lact_level_int_map(&self) -> &PRO_TG_LACT_LEVEL_INT_MAP {
        &self.pro_tg_lact_level_int_map
    }
    ///0x4c - TG1_T0_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_t0_level_int_map(&self) -> &PRO_TG1_T0_LEVEL_INT_MAP {
        &self.pro_tg1_t0_level_int_map
    }
    ///0x50 - TG1_T1_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_t1_level_int_map(&self) -> &PRO_TG1_T1_LEVEL_INT_MAP {
        &self.pro_tg1_t1_level_int_map
    }
    ///0x54 - TG1_WDT_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_wdt_level_int_map(&self) -> &PRO_TG1_WDT_LEVEL_INT_MAP {
        &self.pro_tg1_wdt_level_int_map
    }
    ///0x58 - TG1_LACT_LEVEL_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_lact_level_int_map(&self) -> &PRO_TG1_LACT_LEVEL_INT_MAP {
        &self.pro_tg1_lact_level_int_map
    }
    ///0x5c - GPIO_INTERRUPT_PRO interrupt configuration register
    #[inline(always)]
    pub const fn pro_gpio_interrupt_pro_map(&self) -> &PRO_GPIO_INTERRUPT_PRO_MAP {
        &self.pro_gpio_interrupt_pro_map
    }
    ///0x60 - GPIO_INTERRUPT_PRO_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_gpio_interrupt_pro_nmi_map(&self) -> &PRO_GPIO_INTERRUPT_PRO_NMI_MAP {
        &self.pro_gpio_interrupt_pro_nmi_map
    }
    ///0x64 - GPIO_INTERRUPT_APP interrupt configuration register
    #[inline(always)]
    pub const fn pro_gpio_interrupt_app_map(&self) -> &PRO_GPIO_INTERRUPT_APP_MAP {
        &self.pro_gpio_interrupt_app_map
    }
    ///0x68 - GPIO_INTERRUPT_APP_NMI interrupt configuration register
    #[inline(always)]
    pub const fn pro_gpio_interrupt_app_nmi_map(&self) -> &PRO_GPIO_INTERRUPT_APP_NMI_MAP {
        &self.pro_gpio_interrupt_app_nmi_map
    }
    ///0x6c - DEDICATED_GPIO_IN_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_dedicated_gpio_in_intr_map(&self) -> &PRO_DEDICATED_GPIO_IN_INTR_MAP {
        &self.pro_dedicated_gpio_in_intr_map
    }
    ///0x70 - CPU_INTR_FROM_CPU_0 interrupt configuration register
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_0_map(&self) -> &PRO_CPU_INTR_FROM_CPU_0_MAP {
        &self.pro_cpu_intr_from_cpu_0_map
    }
    ///0x74 - CPU_INTR_FROM_CPU_1 interrupt configuration register
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_1_map(&self) -> &PRO_CPU_INTR_FROM_CPU_1_MAP {
        &self.pro_cpu_intr_from_cpu_1_map
    }
    ///0x78 - CPU_INTR_FROM_CPU_2 interrupt configuration register
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_2_map(&self) -> &PRO_CPU_INTR_FROM_CPU_2_MAP {
        &self.pro_cpu_intr_from_cpu_2_map
    }
    ///0x7c - CPU_INTR_FROM_CPU_3 interrupt configuration register
    #[inline(always)]
    pub const fn pro_cpu_intr_from_cpu_3_map(&self) -> &PRO_CPU_INTR_FROM_CPU_3_MAP {
        &self.pro_cpu_intr_from_cpu_3_map
    }
    ///0x80 - SPI_INTR_1 interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi_intr_1_map(&self) -> &PRO_SPI_INTR_1_MAP {
        &self.pro_spi_intr_1_map
    }
    ///0x84 - SPI_INTR_2 interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi_intr_2_map(&self) -> &PRO_SPI_INTR_2_MAP {
        &self.pro_spi_intr_2_map
    }
    ///0x88 - SPI_INTR_3 interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi_intr_3_map(&self) -> &PRO_SPI_INTR_3_MAP {
        &self.pro_spi_intr_3_map
    }
    ///0x8c - I2S0_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_i2s0_int_map(&self) -> &PRO_I2S0_INT_MAP {
        &self.pro_i2s0_int_map
    }
    ///0x90 - I2S1_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_i2s1_int_map(&self) -> &PRO_I2S1_INT_MAP {
        &self.pro_i2s1_int_map
    }
    ///0x94 - UART_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_uart_intr_map(&self) -> &PRO_UART_INTR_MAP {
        &self.pro_uart_intr_map
    }
    ///0x98 - UART1_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_uart1_intr_map(&self) -> &PRO_UART1_INTR_MAP {
        &self.pro_uart1_intr_map
    }
    ///0x9c - UART2_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_uart2_intr_map(&self) -> &PRO_UART2_INTR_MAP {
        &self.pro_uart2_intr_map
    }
    ///0xa0 - SDIO_HOST_INTERRUPT configuration register
    #[inline(always)]
    pub const fn pro_sdio_host_interrupt_map(&self) -> &PRO_SDIO_HOST_INTERRUPT_MAP {
        &self.pro_sdio_host_interrupt_map
    }
    ///0xa4 - PWM0_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pwm0_intr_map(&self) -> &PRO_PWM0_INTR_MAP {
        &self.pro_pwm0_intr_map
    }
    ///0xa8 - PWM1_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pwm1_intr_map(&self) -> &PRO_PWM1_INTR_MAP {
        &self.pro_pwm1_intr_map
    }
    ///0xac - PWM2_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pwm2_intr_map(&self) -> &PRO_PWM2_INTR_MAP {
        &self.pro_pwm2_intr_map
    }
    ///0xb0 - PWM3_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pwm3_intr_map(&self) -> &PRO_PWM3_INTR_MAP {
        &self.pro_pwm3_intr_map
    }
    ///0xb4 - LEDC_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_ledc_int_map(&self) -> &PRO_LEDC_INT_MAP {
        &self.pro_ledc_int_map
    }
    ///0xb8 - EFUSE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_efuse_int_map(&self) -> &PRO_EFUSE_INT_MAP {
        &self.pro_efuse_int_map
    }
    ///0xbc - CAN_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_can_int_map(&self) -> &PRO_CAN_INT_MAP {
        &self.pro_can_int_map
    }
    ///0xc0 - USB_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_usb_intr_map(&self) -> &PRO_USB_INTR_MAP {
        &self.pro_usb_intr_map
    }
    ///0xc4 - RTC_CORE_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_rtc_core_intr_map(&self) -> &PRO_RTC_CORE_INTR_MAP {
        &self.pro_rtc_core_intr_map
    }
    ///0xc8 - RMT_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_rmt_intr_map(&self) -> &PRO_RMT_INTR_MAP {
        &self.pro_rmt_intr_map
    }
    ///0xcc - PCNT_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_pcnt_intr_map(&self) -> &PRO_PCNT_INTR_MAP {
        &self.pro_pcnt_intr_map
    }
    ///0xd0 - I2C_EXT0_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_i2c_ext0_intr_map(&self) -> &PRO_I2C_EXT0_INTR_MAP {
        &self.pro_i2c_ext0_intr_map
    }
    ///0xd4 - I2C_EXT1_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_i2c_ext1_intr_map(&self) -> &PRO_I2C_EXT1_INTR_MAP {
        &self.pro_i2c_ext1_intr_map
    }
    ///0xd8 - RSA_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_rsa_intr_map(&self) -> &PRO_RSA_INTR_MAP {
        &self.pro_rsa_intr_map
    }
    ///0xdc - SHA_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_sha_intr_map(&self) -> &PRO_SHA_INTR_MAP {
        &self.pro_sha_intr_map
    }
    ///0xe0 - AES_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_aes_intr_map(&self) -> &PRO_AES_INTR_MAP {
        &self.pro_aes_intr_map
    }
    ///0xe4 - SPI2_DMA_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi2_dma_int_map(&self) -> &PRO_SPI2_DMA_INT_MAP {
        &self.pro_spi2_dma_int_map
    }
    ///0xe8 - SPI3_DMA_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi3_dma_int_map(&self) -> &PRO_SPI3_DMA_INT_MAP {
        &self.pro_spi3_dma_int_map
    }
    ///0xec - WDG_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_wdg_int_map(&self) -> &PRO_WDG_INT_MAP {
        &self.pro_wdg_int_map
    }
    ///0xf0 - TIMER_INT1 interrupt configuration register
    #[inline(always)]
    pub const fn pro_timer_int1_map(&self) -> &PRO_TIMER_INT1_MAP {
        &self.pro_timer_int1_map
    }
    ///0xf4 - TIMER_INT2 interrupt configuration register
    #[inline(always)]
    pub const fn pro_timer_int2_map(&self) -> &PRO_TIMER_INT2_MAP {
        &self.pro_timer_int2_map
    }
    ///0xf8 - TG_T0_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_t0_edge_int_map(&self) -> &PRO_TG_T0_EDGE_INT_MAP {
        &self.pro_tg_t0_edge_int_map
    }
    ///0xfc - TG_T1_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_t1_edge_int_map(&self) -> &PRO_TG_T1_EDGE_INT_MAP {
        &self.pro_tg_t1_edge_int_map
    }
    ///0x100 - TG_WDT_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_wdt_edge_int_map(&self) -> &PRO_TG_WDT_EDGE_INT_MAP {
        &self.pro_tg_wdt_edge_int_map
    }
    ///0x104 - TG_LACT_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg_lact_edge_int_map(&self) -> &PRO_TG_LACT_EDGE_INT_MAP {
        &self.pro_tg_lact_edge_int_map
    }
    ///0x108 - TG1_T0_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_t0_edge_int_map(&self) -> &PRO_TG1_T0_EDGE_INT_MAP {
        &self.pro_tg1_t0_edge_int_map
    }
    ///0x10c - TG1_T1_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_t1_edge_int_map(&self) -> &PRO_TG1_T1_EDGE_INT_MAP {
        &self.pro_tg1_t1_edge_int_map
    }
    ///0x110 - TG1_WDT_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_wdt_edge_int_map(&self) -> &PRO_TG1_WDT_EDGE_INT_MAP {
        &self.pro_tg1_wdt_edge_int_map
    }
    ///0x114 - TG1_LACT_EDGE_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_tg1_lact_edge_int_map(&self) -> &PRO_TG1_LACT_EDGE_INT_MAP {
        &self.pro_tg1_lact_edge_int_map
    }
    ///0x118 - CACHE_IA_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_cache_ia_int_map(&self) -> &PRO_CACHE_IA_INT_MAP {
        &self.pro_cache_ia_int_map
    }
    ///0x11c - SYSTIMER_TARGET0_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_systimer_target0_int_map(&self) -> &PRO_SYSTIMER_TARGET0_INT_MAP {
        &self.pro_systimer_target0_int_map
    }
    ///0x120 - SYSTIMER_TARGET1_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_systimer_target1_int_map(&self) -> &PRO_SYSTIMER_TARGET1_INT_MAP {
        &self.pro_systimer_target1_int_map
    }
    ///0x124 - SYSTIMER_TARGET2_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_systimer_target2_int_map(&self) -> &PRO_SYSTIMER_TARGET2_INT_MAP {
        &self.pro_systimer_target2_int_map
    }
    ///0x128 - ASSIST_DEBUG_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_assist_debug_intr_map(&self) -> &PRO_ASSIST_DEBUG_INTR_MAP {
        &self.pro_assist_debug_intr_map
    }
    ///0x12c - PMS_PRO_IRAM0_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_pro_iram0_ilg_intr_map(&self) -> &PRO_PMS_PRO_IRAM0_ILG_INTR_MAP {
        &self.pro_pms_pro_iram0_ilg_intr_map
    }
    ///0x130 - PMS_PRO_DRAM0_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_pro_dram0_ilg_intr_map(&self) -> &PRO_PMS_PRO_DRAM0_ILG_INTR_MAP {
        &self.pro_pms_pro_dram0_ilg_intr_map
    }
    ///0x134 - PMS_PRO_DPORT_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_pro_dport_ilg_intr_map(&self) -> &PRO_PMS_PRO_DPORT_ILG_INTR_MAP {
        &self.pro_pms_pro_dport_ilg_intr_map
    }
    ///0x138 - PMS_PRO_AHB_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_pro_ahb_ilg_intr_map(&self) -> &PRO_PMS_PRO_AHB_ILG_INTR_MAP {
        &self.pro_pms_pro_ahb_ilg_intr_map
    }
    ///0x13c - PMS_PRO_CACHE_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_pro_cache_ilg_intr_map(&self) -> &PRO_PMS_PRO_CACHE_ILG_INTR_MAP {
        &self.pro_pms_pro_cache_ilg_intr_map
    }
    ///0x140 - PMS_DMA_APB_I_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_dma_apb_i_ilg_intr_map(&self) -> &PRO_PMS_DMA_APB_I_ILG_INTR_MAP {
        &self.pro_pms_dma_apb_i_ilg_intr_map
    }
    ///0x144 - PMS_DMA_RX_I_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_dma_rx_i_ilg_intr_map(&self) -> &PRO_PMS_DMA_RX_I_ILG_INTR_MAP {
        &self.pro_pms_dma_rx_i_ilg_intr_map
    }
    ///0x148 - PMS_DMA_TX_I_ILG interrupt configuration register
    #[inline(always)]
    pub const fn pro_pms_dma_tx_i_ilg_intr_map(&self) -> &PRO_PMS_DMA_TX_I_ILG_INTR_MAP {
        &self.pro_pms_dma_tx_i_ilg_intr_map
    }
    ///0x14c - SPI_MEM_REJECT_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi_mem_reject_intr_map(&self) -> &PRO_SPI_MEM_REJECT_INTR_MAP {
        &self.pro_spi_mem_reject_intr_map
    }
    ///0x150 - DMA_COPY_INTR interrupt configuration register
    #[inline(always)]
    pub const fn pro_dma_copy_intr_map(&self) -> &PRO_DMA_COPY_INTR_MAP {
        &self.pro_dma_copy_intr_map
    }
    ///0x154 - SPI4_DMA_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi4_dma_int_map(&self) -> &PRO_SPI4_DMA_INT_MAP {
        &self.pro_spi4_dma_int_map
    }
    ///0x158 - SPI_INTR_4 interrupt configuration register
    #[inline(always)]
    pub const fn pro_spi_intr_4_map(&self) -> &PRO_SPI_INTR_4_MAP {
        &self.pro_spi_intr_4_map
    }
    ///0x15c - DCACHE_PRELOAD_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_dcache_preload_int_map(&self) -> &PRO_DCACHE_PRELOAD_INT_MAP {
        &self.pro_dcache_preload_int_map
    }
    ///0x160 - ICACHE_PRELOAD_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_icache_preload_int_map(&self) -> &PRO_ICACHE_PRELOAD_INT_MAP {
        &self.pro_icache_preload_int_map
    }
    ///0x164 - APB_ADC_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_apb_adc_int_map(&self) -> &PRO_APB_ADC_INT_MAP {
        &self.pro_apb_adc_int_map
    }
    ///0x168 - CRYPTO_DMA_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_crypto_dma_int_map(&self) -> &PRO_CRYPTO_DMA_INT_MAP {
        &self.pro_crypto_dma_int_map
    }
    ///0x16c - CPU_PERI_ERROR_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_cpu_peri_error_int_map(&self) -> &PRO_CPU_PERI_ERROR_INT_MAP {
        &self.pro_cpu_peri_error_int_map
    }
    ///0x170 - APB_PERI_ERROR_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_apb_peri_error_int_map(&self) -> &PRO_APB_PERI_ERROR_INT_MAP {
        &self.pro_apb_peri_error_int_map
    }
    ///0x174 - DCACHE_SYNC_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_dcache_sync_int_map(&self) -> &PRO_DCACHE_SYNC_INT_MAP {
        &self.pro_dcache_sync_int_map
    }
    ///0x178 - ICACHE_SYNC_INT interrupt configuration register
    #[inline(always)]
    pub const fn pro_icache_sync_int_map(&self) -> &PRO_ICACHE_SYNC_INT_MAP {
        &self.pro_icache_sync_int_map
    }
    ///0x17c - Interrupt status register 0
    #[inline(always)]
    pub const fn pro_intr_status_0(&self) -> &PRO_INTR_STATUS_0 {
        &self.pro_intr_status_0
    }
    ///0x180 - Interrupt status register 1
    #[inline(always)]
    pub const fn pro_intr_status_1(&self) -> &PRO_INTR_STATUS_1 {
        &self.pro_intr_status_1
    }
    ///0x184 - Interrupt status register 2
    #[inline(always)]
    pub const fn pro_intr_status_2(&self) -> &PRO_INTR_STATUS_2 {
        &self.pro_intr_status_2
    }
    ///0x188 - NMI interrupt signals mask register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0xffc - Version control register
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
/**PRO_MAC_INTR_MAP (rw) register accessor: MAC_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_mac_intr_map`] module*/
pub type PRO_MAC_INTR_MAP = crate::Reg<pro_mac_intr_map::PRO_MAC_INTR_MAP_SPEC>;
///MAC_INTR interrupt configuration register
pub mod pro_mac_intr_map;
/**PRO_MAC_NMI_MAP (rw) register accessor: MAC_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_mac_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_mac_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_mac_nmi_map`] module*/
pub type PRO_MAC_NMI_MAP = crate::Reg<pro_mac_nmi_map::PRO_MAC_NMI_MAP_SPEC>;
///MAC_NMI interrupt configuration register
pub mod pro_mac_nmi_map;
/**PRO_PWR_INTR_MAP (rw) register accessor: PWR_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pwr_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwr_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pwr_intr_map`] module*/
pub type PRO_PWR_INTR_MAP = crate::Reg<pro_pwr_intr_map::PRO_PWR_INTR_MAP_SPEC>;
///PWR_INTR interrupt configuration register
pub mod pro_pwr_intr_map;
/**PRO_BB_INT_MAP (rw) register accessor: BB_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_bb_int_map`] module*/
pub type PRO_BB_INT_MAP = crate::Reg<pro_bb_int_map::PRO_BB_INT_MAP_SPEC>;
///BB_INT interrupt configuration register
pub mod pro_bb_int_map;
/**PRO_BT_MAC_INT_MAP (rw) register accessor: BT_MAC_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_mac_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_mac_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_bt_mac_int_map`] module*/
pub type PRO_BT_MAC_INT_MAP = crate::Reg<pro_bt_mac_int_map::PRO_BT_MAC_INT_MAP_SPEC>;
///BT_MAC_INT interrupt configuration register
pub mod pro_bt_mac_int_map;
/**PRO_BT_BB_INT_MAP (rw) register accessor: BT_BB_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_bb_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_bb_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_bt_bb_int_map`] module*/
pub type PRO_BT_BB_INT_MAP = crate::Reg<pro_bt_bb_int_map::PRO_BT_BB_INT_MAP_SPEC>;
///BT_BB_INT interrupt configuration register
pub mod pro_bt_bb_int_map;
/**PRO_BT_BB_NMI_MAP (rw) register accessor: BT_BB_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_bt_bb_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_bt_bb_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_bt_bb_nmi_map`] module*/
pub type PRO_BT_BB_NMI_MAP = crate::Reg<pro_bt_bb_nmi_map::PRO_BT_BB_NMI_MAP_SPEC>;
///BT_BB_NMI interrupt configuration register
pub mod pro_bt_bb_nmi_map;
/**PRO_RWBT_IRQ_MAP (rw) register accessor: RWBT_IRQ interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rwbt_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwbt_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rwbt_irq_map`] module*/
pub type PRO_RWBT_IRQ_MAP = crate::Reg<pro_rwbt_irq_map::PRO_RWBT_IRQ_MAP_SPEC>;
///RWBT_IRQ interrupt configuration register
pub mod pro_rwbt_irq_map;
/**PRO_RWBLE_IRQ_MAP (rw) register accessor: RWBLE_IRQ interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rwble_irq_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwble_irq_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rwble_irq_map`] module*/
pub type PRO_RWBLE_IRQ_MAP = crate::Reg<pro_rwble_irq_map::PRO_RWBLE_IRQ_MAP_SPEC>;
///RWBLE_IRQ interrupt configuration register
pub mod pro_rwble_irq_map;
/**PRO_RWBT_NMI_MAP (rw) register accessor: RWBT_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rwbt_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwbt_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rwbt_nmi_map`] module*/
pub type PRO_RWBT_NMI_MAP = crate::Reg<pro_rwbt_nmi_map::PRO_RWBT_NMI_MAP_SPEC>;
///RWBT_NMI interrupt configuration register
pub mod pro_rwbt_nmi_map;
/**PRO_RWBLE_NMI_MAP (rw) register accessor: RWBLE_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rwble_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwble_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rwble_nmi_map`] module*/
pub type PRO_RWBLE_NMI_MAP = crate::Reg<pro_rwble_nmi_map::PRO_RWBLE_NMI_MAP_SPEC>;
///RWBLE_NMI interrupt configuration register
pub mod pro_rwble_nmi_map;
/**PRO_SLC0_INTR_MAP (rw) register accessor: SLC0_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_slc0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_slc0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_slc0_intr_map`] module*/
pub type PRO_SLC0_INTR_MAP = crate::Reg<pro_slc0_intr_map::PRO_SLC0_INTR_MAP_SPEC>;
///SLC0_INTR interrupt configuration register
pub mod pro_slc0_intr_map;
/**PRO_SLC1_INTR_MAP (rw) register accessor: SLC1_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_slc1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_slc1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_slc1_intr_map`] module*/
pub type PRO_SLC1_INTR_MAP = crate::Reg<pro_slc1_intr_map::PRO_SLC1_INTR_MAP_SPEC>;
///SLC1_INTR interrupt configuration register
pub mod pro_slc1_intr_map;
/**PRO_UHCI0_INTR_MAP (rw) register accessor: UHCI0_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uhci0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uhci0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_uhci0_intr_map`] module*/
pub type PRO_UHCI0_INTR_MAP = crate::Reg<pro_uhci0_intr_map::PRO_UHCI0_INTR_MAP_SPEC>;
///UHCI0_INTR interrupt configuration register
pub mod pro_uhci0_intr_map;
/**PRO_UHCI1_INTR_MAP (rw) register accessor: UHCI1_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uhci1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uhci1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_uhci1_intr_map`] module*/
pub type PRO_UHCI1_INTR_MAP = crate::Reg<pro_uhci1_intr_map::PRO_UHCI1_INTR_MAP_SPEC>;
///UHCI1_INTR interrupt configuration register
pub mod pro_uhci1_intr_map;
/**PRO_TG_T0_LEVEL_INT_MAP (rw) register accessor: TG_T0_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_t0_level_int_map`] module*/
pub type PRO_TG_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t0_level_int_map::PRO_TG_T0_LEVEL_INT_MAP_SPEC>;
///TG_T0_LEVEL_INT interrupt configuration register
pub mod pro_tg_t0_level_int_map;
/**PRO_TG_T1_LEVEL_INT_MAP (rw) register accessor: TG_T1_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_t1_level_int_map`] module*/
pub type PRO_TG_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg_t1_level_int_map::PRO_TG_T1_LEVEL_INT_MAP_SPEC>;
///TG_T1_LEVEL_INT interrupt configuration register
pub mod pro_tg_t1_level_int_map;
/**PRO_TG_WDT_LEVEL_INT_MAP (rw) register accessor: TG_WDT_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_wdt_level_int_map`] module*/
pub type PRO_TG_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_wdt_level_int_map::PRO_TG_WDT_LEVEL_INT_MAP_SPEC>;
///TG_WDT_LEVEL_INT interrupt configuration register
pub mod pro_tg_wdt_level_int_map;
/**PRO_TG_LACT_LEVEL_INT_MAP (rw) register accessor: TG_LACT_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_lact_level_int_map`] module*/
pub type PRO_TG_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg_lact_level_int_map::PRO_TG_LACT_LEVEL_INT_MAP_SPEC>;
///TG_LACT_LEVEL_INT interrupt configuration register
pub mod pro_tg_lact_level_int_map;
/**PRO_TG1_T0_LEVEL_INT_MAP (rw) register accessor: TG1_T0_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t0_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t0_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_t0_level_int_map`] module*/
pub type PRO_TG1_T0_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t0_level_int_map::PRO_TG1_T0_LEVEL_INT_MAP_SPEC>;
///TG1_T0_LEVEL_INT interrupt configuration register
pub mod pro_tg1_t0_level_int_map;
/**PRO_TG1_T1_LEVEL_INT_MAP (rw) register accessor: TG1_T1_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t1_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t1_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_t1_level_int_map`] module*/
pub type PRO_TG1_T1_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_t1_level_int_map::PRO_TG1_T1_LEVEL_INT_MAP_SPEC>;
///TG1_T1_LEVEL_INT interrupt configuration register
pub mod pro_tg1_t1_level_int_map;
/**PRO_TG1_WDT_LEVEL_INT_MAP (rw) register accessor: TG1_WDT_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_wdt_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_wdt_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_wdt_level_int_map`] module*/
pub type PRO_TG1_WDT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_wdt_level_int_map::PRO_TG1_WDT_LEVEL_INT_MAP_SPEC>;
///TG1_WDT_LEVEL_INT interrupt configuration register
pub mod pro_tg1_wdt_level_int_map;
/**PRO_TG1_LACT_LEVEL_INT_MAP (rw) register accessor: TG1_LACT_LEVEL_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_lact_level_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_lact_level_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_lact_level_int_map`] module*/
pub type PRO_TG1_LACT_LEVEL_INT_MAP =
    crate::Reg<pro_tg1_lact_level_int_map::PRO_TG1_LACT_LEVEL_INT_MAP_SPEC>;
///TG1_LACT_LEVEL_INT interrupt configuration register
pub mod pro_tg1_lact_level_int_map;
/**PRO_GPIO_INTERRUPT_PRO_MAP (rw) register accessor: GPIO_INTERRUPT_PRO interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_pro_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_pro_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_gpio_interrupt_pro_map`] module*/
pub type PRO_GPIO_INTERRUPT_PRO_MAP =
    crate::Reg<pro_gpio_interrupt_pro_map::PRO_GPIO_INTERRUPT_PRO_MAP_SPEC>;
///GPIO_INTERRUPT_PRO interrupt configuration register
pub mod pro_gpio_interrupt_pro_map;
/**PRO_GPIO_INTERRUPT_PRO_NMI_MAP (rw) register accessor: GPIO_INTERRUPT_PRO_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_pro_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_pro_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_gpio_interrupt_pro_nmi_map`] module*/
pub type PRO_GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_pro_nmi_map::PRO_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
///GPIO_INTERRUPT_PRO_NMI interrupt configuration register
pub mod pro_gpio_interrupt_pro_nmi_map;
/**PRO_GPIO_INTERRUPT_APP_MAP (rw) register accessor: GPIO_INTERRUPT_APP interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_app_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_app_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_gpio_interrupt_app_map`] module*/
pub type PRO_GPIO_INTERRUPT_APP_MAP =
    crate::Reg<pro_gpio_interrupt_app_map::PRO_GPIO_INTERRUPT_APP_MAP_SPEC>;
///GPIO_INTERRUPT_APP interrupt configuration register
pub mod pro_gpio_interrupt_app_map;
/**PRO_GPIO_INTERRUPT_APP_NMI_MAP (rw) register accessor: GPIO_INTERRUPT_APP_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_gpio_interrupt_app_nmi_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_gpio_interrupt_app_nmi_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_gpio_interrupt_app_nmi_map`] module*/
pub type PRO_GPIO_INTERRUPT_APP_NMI_MAP =
    crate::Reg<pro_gpio_interrupt_app_nmi_map::PRO_GPIO_INTERRUPT_APP_NMI_MAP_SPEC>;
///GPIO_INTERRUPT_APP_NMI interrupt configuration register
pub mod pro_gpio_interrupt_app_nmi_map;
/**PRO_DEDICATED_GPIO_IN_INTR_MAP (rw) register accessor: DEDICATED_GPIO_IN_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dedicated_gpio_in_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dedicated_gpio_in_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_dedicated_gpio_in_intr_map`] module*/
pub type PRO_DEDICATED_GPIO_IN_INTR_MAP =
    crate::Reg<pro_dedicated_gpio_in_intr_map::PRO_DEDICATED_GPIO_IN_INTR_MAP_SPEC>;
///DEDICATED_GPIO_IN_INTR interrupt configuration register
pub mod pro_dedicated_gpio_in_intr_map;
/**PRO_CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: CPU_INTR_FROM_CPU_0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cpu_intr_from_cpu_0_map`] module*/
pub type PRO_CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_0_map::PRO_CPU_INTR_FROM_CPU_0_MAP_SPEC>;
///CPU_INTR_FROM_CPU_0 interrupt configuration register
pub mod pro_cpu_intr_from_cpu_0_map;
/**PRO_CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: CPU_INTR_FROM_CPU_1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cpu_intr_from_cpu_1_map`] module*/
pub type PRO_CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_1_map::PRO_CPU_INTR_FROM_CPU_1_MAP_SPEC>;
///CPU_INTR_FROM_CPU_1 interrupt configuration register
pub mod pro_cpu_intr_from_cpu_1_map;
/**PRO_CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: CPU_INTR_FROM_CPU_2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cpu_intr_from_cpu_2_map`] module*/
pub type PRO_CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_2_map::PRO_CPU_INTR_FROM_CPU_2_MAP_SPEC>;
///CPU_INTR_FROM_CPU_2 interrupt configuration register
pub mod pro_cpu_intr_from_cpu_2_map;
/**PRO_CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: CPU_INTR_FROM_CPU_3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_intr_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cpu_intr_from_cpu_3_map`] module*/
pub type PRO_CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<pro_cpu_intr_from_cpu_3_map::PRO_CPU_INTR_FROM_CPU_3_MAP_SPEC>;
///CPU_INTR_FROM_CPU_3 interrupt configuration register
pub mod pro_cpu_intr_from_cpu_3_map;
/**PRO_SPI_INTR_1_MAP (rw) register accessor: SPI_INTR_1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi_intr_1_map`] module*/
pub type PRO_SPI_INTR_1_MAP = crate::Reg<pro_spi_intr_1_map::PRO_SPI_INTR_1_MAP_SPEC>;
///SPI_INTR_1 interrupt configuration register
pub mod pro_spi_intr_1_map;
/**PRO_SPI_INTR_2_MAP (rw) register accessor: SPI_INTR_2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi_intr_2_map`] module*/
pub type PRO_SPI_INTR_2_MAP = crate::Reg<pro_spi_intr_2_map::PRO_SPI_INTR_2_MAP_SPEC>;
///SPI_INTR_2 interrupt configuration register
pub mod pro_spi_intr_2_map;
/**PRO_SPI_INTR_3_MAP (rw) register accessor: SPI_INTR_3 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi_intr_3_map`] module*/
pub type PRO_SPI_INTR_3_MAP = crate::Reg<pro_spi_intr_3_map::PRO_SPI_INTR_3_MAP_SPEC>;
///SPI_INTR_3 interrupt configuration register
pub mod pro_spi_intr_3_map;
/**PRO_I2S0_INT_MAP (rw) register accessor: I2S0_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_i2s0_int_map`] module*/
pub type PRO_I2S0_INT_MAP = crate::Reg<pro_i2s0_int_map::PRO_I2S0_INT_MAP_SPEC>;
///I2S0_INT interrupt configuration register
pub mod pro_i2s0_int_map;
/**PRO_I2S1_INT_MAP (rw) register accessor: I2S1_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_i2s1_int_map`] module*/
pub type PRO_I2S1_INT_MAP = crate::Reg<pro_i2s1_int_map::PRO_I2S1_INT_MAP_SPEC>;
///I2S1_INT interrupt configuration register
pub mod pro_i2s1_int_map;
/**PRO_UART_INTR_MAP (rw) register accessor: UART_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uart_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_uart_intr_map`] module*/
pub type PRO_UART_INTR_MAP = crate::Reg<pro_uart_intr_map::PRO_UART_INTR_MAP_SPEC>;
///UART_INT interrupt configuration register
pub mod pro_uart_intr_map;
/**PRO_UART1_INTR_MAP (rw) register accessor: UART1_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uart1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_uart1_intr_map`] module*/
pub type PRO_UART1_INTR_MAP = crate::Reg<pro_uart1_intr_map::PRO_UART1_INTR_MAP_SPEC>;
///UART1_INT interrupt configuration register
pub mod pro_uart1_intr_map;
/**PRO_UART2_INTR_MAP (rw) register accessor: UART2_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_uart2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_uart2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_uart2_intr_map`] module*/
pub type PRO_UART2_INTR_MAP = crate::Reg<pro_uart2_intr_map::PRO_UART2_INTR_MAP_SPEC>;
///UART2_INT interrupt configuration register
pub mod pro_uart2_intr_map;
/**PRO_SDIO_HOST_INTERRUPT_MAP (rw) register accessor: SDIO_HOST_INTERRUPT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_sdio_host_interrupt_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_sdio_host_interrupt_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_sdio_host_interrupt_map`] module*/
pub type PRO_SDIO_HOST_INTERRUPT_MAP =
    crate::Reg<pro_sdio_host_interrupt_map::PRO_SDIO_HOST_INTERRUPT_MAP_SPEC>;
///SDIO_HOST_INTERRUPT configuration register
pub mod pro_sdio_host_interrupt_map;
/**PRO_PWM0_INTR_MAP (rw) register accessor: PWM0_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pwm0_intr_map`] module*/
pub type PRO_PWM0_INTR_MAP = crate::Reg<pro_pwm0_intr_map::PRO_PWM0_INTR_MAP_SPEC>;
///PWM0_INTR interrupt configuration register
pub mod pro_pwm0_intr_map;
/**PRO_PWM1_INTR_MAP (rw) register accessor: PWM1_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pwm1_intr_map`] module*/
pub type PRO_PWM1_INTR_MAP = crate::Reg<pro_pwm1_intr_map::PRO_PWM1_INTR_MAP_SPEC>;
///PWM1_INTR interrupt configuration register
pub mod pro_pwm1_intr_map;
/**PRO_PWM2_INTR_MAP (rw) register accessor: PWM2_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm2_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm2_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pwm2_intr_map`] module*/
pub type PRO_PWM2_INTR_MAP = crate::Reg<pro_pwm2_intr_map::PRO_PWM2_INTR_MAP_SPEC>;
///PWM2_INTR interrupt configuration register
pub mod pro_pwm2_intr_map;
/**PRO_PWM3_INTR_MAP (rw) register accessor: PWM3_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pwm3_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pwm3_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pwm3_intr_map`] module*/
pub type PRO_PWM3_INTR_MAP = crate::Reg<pro_pwm3_intr_map::PRO_PWM3_INTR_MAP_SPEC>;
///PWM3_INTR interrupt configuration register
pub mod pro_pwm3_intr_map;
/**PRO_LEDC_INT_MAP (rw) register accessor: LEDC_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_ledc_int_map`] module*/
pub type PRO_LEDC_INT_MAP = crate::Reg<pro_ledc_int_map::PRO_LEDC_INT_MAP_SPEC>;
///LEDC_INTR interrupt configuration register
pub mod pro_ledc_int_map;
/**PRO_EFUSE_INT_MAP (rw) register accessor: EFUSE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_efuse_int_map`] module*/
pub type PRO_EFUSE_INT_MAP = crate::Reg<pro_efuse_int_map::PRO_EFUSE_INT_MAP_SPEC>;
///EFUSE_INT interrupt configuration register
pub mod pro_efuse_int_map;
/**PRO_CAN_INT_MAP (rw) register accessor: CAN_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_can_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_can_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_can_int_map`] module*/
pub type PRO_CAN_INT_MAP = crate::Reg<pro_can_int_map::PRO_CAN_INT_MAP_SPEC>;
///CAN_INT interrupt configuration register
pub mod pro_can_int_map;
/**PRO_USB_INTR_MAP (rw) register accessor: USB_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_usb_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_usb_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_usb_intr_map`] module*/
pub type PRO_USB_INTR_MAP = crate::Reg<pro_usb_intr_map::PRO_USB_INTR_MAP_SPEC>;
///USB_INT interrupt configuration register
pub mod pro_usb_intr_map;
/**PRO_RTC_CORE_INTR_MAP (rw) register accessor: RTC_CORE_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rtc_core_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rtc_core_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rtc_core_intr_map`] module*/
pub type PRO_RTC_CORE_INTR_MAP = crate::Reg<pro_rtc_core_intr_map::PRO_RTC_CORE_INTR_MAP_SPEC>;
///RTC_CORE_INTR interrupt configuration register
pub mod pro_rtc_core_intr_map;
/**PRO_RMT_INTR_MAP (rw) register accessor: RMT_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rmt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rmt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rmt_intr_map`] module*/
pub type PRO_RMT_INTR_MAP = crate::Reg<pro_rmt_intr_map::PRO_RMT_INTR_MAP_SPEC>;
///RMT_INTR interrupt configuration register
pub mod pro_rmt_intr_map;
/**PRO_PCNT_INTR_MAP (rw) register accessor: PCNT_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pcnt_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pcnt_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pcnt_intr_map`] module*/
pub type PRO_PCNT_INTR_MAP = crate::Reg<pro_pcnt_intr_map::PRO_PCNT_INTR_MAP_SPEC>;
///PCNT_INTR interrupt configuration register
pub mod pro_pcnt_intr_map;
/**PRO_I2C_EXT0_INTR_MAP (rw) register accessor: I2C_EXT0_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_i2c_ext0_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2c_ext0_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_i2c_ext0_intr_map`] module*/
pub type PRO_I2C_EXT0_INTR_MAP = crate::Reg<pro_i2c_ext0_intr_map::PRO_I2C_EXT0_INTR_MAP_SPEC>;
///I2C_EXT0_INTR interrupt configuration register
pub mod pro_i2c_ext0_intr_map;
/**PRO_I2C_EXT1_INTR_MAP (rw) register accessor: I2C_EXT1_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_i2c_ext1_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2c_ext1_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_i2c_ext1_intr_map`] module*/
pub type PRO_I2C_EXT1_INTR_MAP = crate::Reg<pro_i2c_ext1_intr_map::PRO_I2C_EXT1_INTR_MAP_SPEC>;
///I2C_EXT1_INTR interrupt configuration register
pub mod pro_i2c_ext1_intr_map;
/**PRO_RSA_INTR_MAP (rw) register accessor: RSA_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rsa_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rsa_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_rsa_intr_map`] module*/
pub type PRO_RSA_INTR_MAP = crate::Reg<pro_rsa_intr_map::PRO_RSA_INTR_MAP_SPEC>;
///RSA_INTR interrupt configuration register
pub mod pro_rsa_intr_map;
/**PRO_SHA_INTR_MAP (rw) register accessor: SHA_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_sha_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_sha_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_sha_intr_map`] module*/
pub type PRO_SHA_INTR_MAP = crate::Reg<pro_sha_intr_map::PRO_SHA_INTR_MAP_SPEC>;
///SHA_INTR interrupt configuration register
pub mod pro_sha_intr_map;
/**PRO_AES_INTR_MAP (rw) register accessor: AES_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_aes_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_aes_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_aes_intr_map`] module*/
pub type PRO_AES_INTR_MAP = crate::Reg<pro_aes_intr_map::PRO_AES_INTR_MAP_SPEC>;
///AES_INTR interrupt configuration register
pub mod pro_aes_intr_map;
/**PRO_SPI2_DMA_INT_MAP (rw) register accessor: SPI2_DMA_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi2_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi2_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi2_dma_int_map`] module*/
pub type PRO_SPI2_DMA_INT_MAP = crate::Reg<pro_spi2_dma_int_map::PRO_SPI2_DMA_INT_MAP_SPEC>;
///SPI2_DMA_INT interrupt configuration register
pub mod pro_spi2_dma_int_map;
/**PRO_SPI3_DMA_INT_MAP (rw) register accessor: SPI3_DMA_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi3_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi3_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi3_dma_int_map`] module*/
pub type PRO_SPI3_DMA_INT_MAP = crate::Reg<pro_spi3_dma_int_map::PRO_SPI3_DMA_INT_MAP_SPEC>;
///SPI3_DMA_INT interrupt configuration register
pub mod pro_spi3_dma_int_map;
/**PRO_WDG_INT_MAP (rw) register accessor: WDG_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_wdg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_wdg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_wdg_int_map`] module*/
pub type PRO_WDG_INT_MAP = crate::Reg<pro_wdg_int_map::PRO_WDG_INT_MAP_SPEC>;
///WDG_INT interrupt configuration register
pub mod pro_wdg_int_map;
/**PRO_TIMER_INT1_MAP (rw) register accessor: TIMER_INT1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_timer_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_timer_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_timer_int1_map`] module*/
pub type PRO_TIMER_INT1_MAP = crate::Reg<pro_timer_int1_map::PRO_TIMER_INT1_MAP_SPEC>;
///TIMER_INT1 interrupt configuration register
pub mod pro_timer_int1_map;
/**PRO_TIMER_INT2_MAP (rw) register accessor: TIMER_INT2 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_timer_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_timer_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_timer_int2_map`] module*/
pub type PRO_TIMER_INT2_MAP = crate::Reg<pro_timer_int2_map::PRO_TIMER_INT2_MAP_SPEC>;
///TIMER_INT2 interrupt configuration register
pub mod pro_timer_int2_map;
/**PRO_TG_T0_EDGE_INT_MAP (rw) register accessor: TG_T0_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_t0_edge_int_map`] module*/
pub type PRO_TG_T0_EDGE_INT_MAP = crate::Reg<pro_tg_t0_edge_int_map::PRO_TG_T0_EDGE_INT_MAP_SPEC>;
///TG_T0_EDGE_INT interrupt configuration register
pub mod pro_tg_t0_edge_int_map;
/**PRO_TG_T1_EDGE_INT_MAP (rw) register accessor: TG_T1_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_t1_edge_int_map`] module*/
pub type PRO_TG_T1_EDGE_INT_MAP = crate::Reg<pro_tg_t1_edge_int_map::PRO_TG_T1_EDGE_INT_MAP_SPEC>;
///TG_T1_EDGE_INT interrupt configuration register
pub mod pro_tg_t1_edge_int_map;
/**PRO_TG_WDT_EDGE_INT_MAP (rw) register accessor: TG_WDT_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_wdt_edge_int_map`] module*/
pub type PRO_TG_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg_wdt_edge_int_map::PRO_TG_WDT_EDGE_INT_MAP_SPEC>;
///TG_WDT_EDGE_INT interrupt configuration register
pub mod pro_tg_wdt_edge_int_map;
/**PRO_TG_LACT_EDGE_INT_MAP (rw) register accessor: TG_LACT_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg_lact_edge_int_map`] module*/
pub type PRO_TG_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg_lact_edge_int_map::PRO_TG_LACT_EDGE_INT_MAP_SPEC>;
///TG_LACT_EDGE_INT interrupt configuration register
pub mod pro_tg_lact_edge_int_map;
/**PRO_TG1_T0_EDGE_INT_MAP (rw) register accessor: TG1_T0_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t0_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t0_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_t0_edge_int_map`] module*/
pub type PRO_TG1_T0_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t0_edge_int_map::PRO_TG1_T0_EDGE_INT_MAP_SPEC>;
///TG1_T0_EDGE_INT interrupt configuration register
pub mod pro_tg1_t0_edge_int_map;
/**PRO_TG1_T1_EDGE_INT_MAP (rw) register accessor: TG1_T1_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_t1_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_t1_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_t1_edge_int_map`] module*/
pub type PRO_TG1_T1_EDGE_INT_MAP =
    crate::Reg<pro_tg1_t1_edge_int_map::PRO_TG1_T1_EDGE_INT_MAP_SPEC>;
///TG1_T1_EDGE_INT interrupt configuration register
pub mod pro_tg1_t1_edge_int_map;
/**PRO_TG1_WDT_EDGE_INT_MAP (rw) register accessor: TG1_WDT_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_wdt_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_wdt_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_wdt_edge_int_map`] module*/
pub type PRO_TG1_WDT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_wdt_edge_int_map::PRO_TG1_WDT_EDGE_INT_MAP_SPEC>;
///TG1_WDT_EDGE_INT interrupt configuration register
pub mod pro_tg1_wdt_edge_int_map;
/**PRO_TG1_LACT_EDGE_INT_MAP (rw) register accessor: TG1_LACT_EDGE_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_tg1_lact_edge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tg1_lact_edge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_tg1_lact_edge_int_map`] module*/
pub type PRO_TG1_LACT_EDGE_INT_MAP =
    crate::Reg<pro_tg1_lact_edge_int_map::PRO_TG1_LACT_EDGE_INT_MAP_SPEC>;
///TG1_LACT_EDGE_INT interrupt configuration register
pub mod pro_tg1_lact_edge_int_map;
/**PRO_CACHE_IA_INT_MAP (rw) register accessor: CACHE_IA_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ia_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ia_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cache_ia_int_map`] module*/
pub type PRO_CACHE_IA_INT_MAP = crate::Reg<pro_cache_ia_int_map::PRO_CACHE_IA_INT_MAP_SPEC>;
///CACHE_IA_INT interrupt configuration register
pub mod pro_cache_ia_int_map;
/**PRO_SYSTIMER_TARGET0_INT_MAP (rw) register accessor: SYSTIMER_TARGET0_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_systimer_target0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_systimer_target0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_systimer_target0_int_map`] module*/
pub type PRO_SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<pro_systimer_target0_int_map::PRO_SYSTIMER_TARGET0_INT_MAP_SPEC>;
///SYSTIMER_TARGET0_INT interrupt configuration register
pub mod pro_systimer_target0_int_map;
/**PRO_SYSTIMER_TARGET1_INT_MAP (rw) register accessor: SYSTIMER_TARGET1_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_systimer_target1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_systimer_target1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_systimer_target1_int_map`] module*/
pub type PRO_SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<pro_systimer_target1_int_map::PRO_SYSTIMER_TARGET1_INT_MAP_SPEC>;
///SYSTIMER_TARGET1_INT interrupt configuration register
pub mod pro_systimer_target1_int_map;
/**PRO_SYSTIMER_TARGET2_INT_MAP (rw) register accessor: SYSTIMER_TARGET2_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_systimer_target2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_systimer_target2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_systimer_target2_int_map`] module*/
pub type PRO_SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<pro_systimer_target2_int_map::PRO_SYSTIMER_TARGET2_INT_MAP_SPEC>;
///SYSTIMER_TARGET2_INT interrupt configuration register
pub mod pro_systimer_target2_int_map;
/**PRO_ASSIST_DEBUG_INTR_MAP (rw) register accessor: ASSIST_DEBUG_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_assist_debug_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_assist_debug_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_assist_debug_intr_map`] module*/
pub type PRO_ASSIST_DEBUG_INTR_MAP =
    crate::Reg<pro_assist_debug_intr_map::PRO_ASSIST_DEBUG_INTR_MAP_SPEC>;
///ASSIST_DEBUG_INTR interrupt configuration register
pub mod pro_assist_debug_intr_map;
/**PRO_PMS_PRO_IRAM0_ILG_INTR_MAP (rw) register accessor: PMS_PRO_IRAM0_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_pro_iram0_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_pro_iram0_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_pro_iram0_ilg_intr_map`] module*/
pub type PRO_PMS_PRO_IRAM0_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_iram0_ilg_intr_map::PRO_PMS_PRO_IRAM0_ILG_INTR_MAP_SPEC>;
///PMS_PRO_IRAM0_ILG interrupt configuration register
pub mod pro_pms_pro_iram0_ilg_intr_map;
/**PRO_PMS_PRO_DRAM0_ILG_INTR_MAP (rw) register accessor: PMS_PRO_DRAM0_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_pro_dram0_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_pro_dram0_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_pro_dram0_ilg_intr_map`] module*/
pub type PRO_PMS_PRO_DRAM0_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_dram0_ilg_intr_map::PRO_PMS_PRO_DRAM0_ILG_INTR_MAP_SPEC>;
///PMS_PRO_DRAM0_ILG interrupt configuration register
pub mod pro_pms_pro_dram0_ilg_intr_map;
/**PRO_PMS_PRO_DPORT_ILG_INTR_MAP (rw) register accessor: PMS_PRO_DPORT_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_pro_dport_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_pro_dport_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_pro_dport_ilg_intr_map`] module*/
pub type PRO_PMS_PRO_DPORT_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_dport_ilg_intr_map::PRO_PMS_PRO_DPORT_ILG_INTR_MAP_SPEC>;
///PMS_PRO_DPORT_ILG interrupt configuration register
pub mod pro_pms_pro_dport_ilg_intr_map;
/**PRO_PMS_PRO_AHB_ILG_INTR_MAP (rw) register accessor: PMS_PRO_AHB_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_pro_ahb_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_pro_ahb_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_pro_ahb_ilg_intr_map`] module*/
pub type PRO_PMS_PRO_AHB_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_ahb_ilg_intr_map::PRO_PMS_PRO_AHB_ILG_INTR_MAP_SPEC>;
///PMS_PRO_AHB_ILG interrupt configuration register
pub mod pro_pms_pro_ahb_ilg_intr_map;
/**PRO_PMS_PRO_CACHE_ILG_INTR_MAP (rw) register accessor: PMS_PRO_CACHE_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_pro_cache_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_pro_cache_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_pro_cache_ilg_intr_map`] module*/
pub type PRO_PMS_PRO_CACHE_ILG_INTR_MAP =
    crate::Reg<pro_pms_pro_cache_ilg_intr_map::PRO_PMS_PRO_CACHE_ILG_INTR_MAP_SPEC>;
///PMS_PRO_CACHE_ILG interrupt configuration register
pub mod pro_pms_pro_cache_ilg_intr_map;
/**PRO_PMS_DMA_APB_I_ILG_INTR_MAP (rw) register accessor: PMS_DMA_APB_I_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_dma_apb_i_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_dma_apb_i_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_dma_apb_i_ilg_intr_map`] module*/
pub type PRO_PMS_DMA_APB_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_apb_i_ilg_intr_map::PRO_PMS_DMA_APB_I_ILG_INTR_MAP_SPEC>;
///PMS_DMA_APB_I_ILG interrupt configuration register
pub mod pro_pms_dma_apb_i_ilg_intr_map;
/**PRO_PMS_DMA_RX_I_ILG_INTR_MAP (rw) register accessor: PMS_DMA_RX_I_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_dma_rx_i_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_dma_rx_i_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_dma_rx_i_ilg_intr_map`] module*/
pub type PRO_PMS_DMA_RX_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_rx_i_ilg_intr_map::PRO_PMS_DMA_RX_I_ILG_INTR_MAP_SPEC>;
///PMS_DMA_RX_I_ILG interrupt configuration register
pub mod pro_pms_dma_rx_i_ilg_intr_map;
/**PRO_PMS_DMA_TX_I_ILG_INTR_MAP (rw) register accessor: PMS_DMA_TX_I_ILG interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_pms_dma_tx_i_ilg_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_pms_dma_tx_i_ilg_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_pms_dma_tx_i_ilg_intr_map`] module*/
pub type PRO_PMS_DMA_TX_I_ILG_INTR_MAP =
    crate::Reg<pro_pms_dma_tx_i_ilg_intr_map::PRO_PMS_DMA_TX_I_ILG_INTR_MAP_SPEC>;
///PMS_DMA_TX_I_ILG interrupt configuration register
pub mod pro_pms_dma_tx_i_ilg_intr_map;
/**PRO_SPI_MEM_REJECT_INTR_MAP (rw) register accessor: SPI_MEM_REJECT_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_mem_reject_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_mem_reject_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi_mem_reject_intr_map`] module*/
pub type PRO_SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<pro_spi_mem_reject_intr_map::PRO_SPI_MEM_REJECT_INTR_MAP_SPEC>;
///SPI_MEM_REJECT_INTR interrupt configuration register
pub mod pro_spi_mem_reject_intr_map;
/**PRO_DMA_COPY_INTR_MAP (rw) register accessor: DMA_COPY_INTR interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dma_copy_intr_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dma_copy_intr_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_dma_copy_intr_map`] module*/
pub type PRO_DMA_COPY_INTR_MAP = crate::Reg<pro_dma_copy_intr_map::PRO_DMA_COPY_INTR_MAP_SPEC>;
///DMA_COPY_INTR interrupt configuration register
pub mod pro_dma_copy_intr_map;
/**PRO_SPI4_DMA_INT_MAP (rw) register accessor: SPI4_DMA_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi4_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi4_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi4_dma_int_map`] module*/
pub type PRO_SPI4_DMA_INT_MAP = crate::Reg<pro_spi4_dma_int_map::PRO_SPI4_DMA_INT_MAP_SPEC>;
///SPI4_DMA_INT interrupt configuration register
pub mod pro_spi4_dma_int_map;
/**PRO_SPI_INTR_4_MAP (rw) register accessor: SPI_INTR_4 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_spi_intr_4_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_spi_intr_4_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_spi_intr_4_map`] module*/
pub type PRO_SPI_INTR_4_MAP = crate::Reg<pro_spi_intr_4_map::PRO_SPI_INTR_4_MAP_SPEC>;
///SPI_INTR_4 interrupt configuration register
pub mod pro_spi_intr_4_map;
/**PRO_DCACHE_PRELOAD_INT_MAP (rw) register accessor: DCACHE_PRELOAD_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_dcache_preload_int_map`] module*/
pub type PRO_DCACHE_PRELOAD_INT_MAP =
    crate::Reg<pro_dcache_preload_int_map::PRO_DCACHE_PRELOAD_INT_MAP_SPEC>;
///DCACHE_PRELOAD_INT interrupt configuration register
pub mod pro_dcache_preload_int_map;
/**PRO_ICACHE_PRELOAD_INT_MAP (rw) register accessor: ICACHE_PRELOAD_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_preload_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_preload_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_icache_preload_int_map`] module*/
pub type PRO_ICACHE_PRELOAD_INT_MAP =
    crate::Reg<pro_icache_preload_int_map::PRO_ICACHE_PRELOAD_INT_MAP_SPEC>;
///ICACHE_PRELOAD_INT interrupt configuration register
pub mod pro_icache_preload_int_map;
/**PRO_APB_ADC_INT_MAP (rw) register accessor: APB_ADC_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_apb_adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_apb_adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_apb_adc_int_map`] module*/
pub type PRO_APB_ADC_INT_MAP = crate::Reg<pro_apb_adc_int_map::PRO_APB_ADC_INT_MAP_SPEC>;
///APB_ADC_INT interrupt configuration register
pub mod pro_apb_adc_int_map;
/**PRO_CRYPTO_DMA_INT_MAP (rw) register accessor: CRYPTO_DMA_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_crypto_dma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_crypto_dma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_crypto_dma_int_map`] module*/
pub type PRO_CRYPTO_DMA_INT_MAP = crate::Reg<pro_crypto_dma_int_map::PRO_CRYPTO_DMA_INT_MAP_SPEC>;
///CRYPTO_DMA_INT interrupt configuration register
pub mod pro_crypto_dma_int_map;
/**PRO_CPU_PERI_ERROR_INT_MAP (rw) register accessor: CPU_PERI_ERROR_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_peri_error_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_peri_error_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_cpu_peri_error_int_map`] module*/
pub type PRO_CPU_PERI_ERROR_INT_MAP =
    crate::Reg<pro_cpu_peri_error_int_map::PRO_CPU_PERI_ERROR_INT_MAP_SPEC>;
///CPU_PERI_ERROR_INT interrupt configuration register
pub mod pro_cpu_peri_error_int_map;
/**PRO_APB_PERI_ERROR_INT_MAP (rw) register accessor: APB_PERI_ERROR_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_apb_peri_error_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_apb_peri_error_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_apb_peri_error_int_map`] module*/
pub type PRO_APB_PERI_ERROR_INT_MAP =
    crate::Reg<pro_apb_peri_error_int_map::PRO_APB_PERI_ERROR_INT_MAP_SPEC>;
///APB_PERI_ERROR_INT interrupt configuration register
pub mod pro_apb_peri_error_int_map;
/**PRO_DCACHE_SYNC_INT_MAP (rw) register accessor: DCACHE_SYNC_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_dcache_sync_int_map`] module*/
pub type PRO_DCACHE_SYNC_INT_MAP =
    crate::Reg<pro_dcache_sync_int_map::PRO_DCACHE_SYNC_INT_MAP_SPEC>;
///DCACHE_SYNC_INT interrupt configuration register
pub mod pro_dcache_sync_int_map;
/**PRO_ICACHE_SYNC_INT_MAP (rw) register accessor: ICACHE_SYNC_INT interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_sync_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_sync_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_icache_sync_int_map`] module*/
pub type PRO_ICACHE_SYNC_INT_MAP =
    crate::Reg<pro_icache_sync_int_map::PRO_ICACHE_SYNC_INT_MAP_SPEC>;
///ICACHE_SYNC_INT interrupt configuration register
pub mod pro_icache_sync_int_map;
/**PRO_INTR_STATUS_0 (r) register accessor: Interrupt status register 0

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_0`] module*/
pub type PRO_INTR_STATUS_0 = crate::Reg<pro_intr_status_0::PRO_INTR_STATUS_0_SPEC>;
///Interrupt status register 0
pub mod pro_intr_status_0;
/**PRO_INTR_STATUS_1 (r) register accessor: Interrupt status register 1

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_1`] module*/
pub type PRO_INTR_STATUS_1 = crate::Reg<pro_intr_status_1::PRO_INTR_STATUS_1_SPEC>;
///Interrupt status register 1
pub mod pro_intr_status_1;
/**PRO_INTR_STATUS_2 (r) register accessor: Interrupt status register 2

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pro_intr_status_2`] module*/
pub type PRO_INTR_STATUS_2 = crate::Reg<pro_intr_status_2::PRO_INTR_STATUS_2_SPEC>;
///Interrupt status register 2
pub mod pro_intr_status_2;
/**CLOCK_GATE (rw) register accessor: NMI interrupt signals mask register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///NMI interrupt signals mask register
pub mod clock_gate;
/**REG_DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_date`] module*/
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
///Version control register
pub mod reg_date;
