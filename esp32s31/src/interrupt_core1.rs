#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_icm_intr_map: SYS_ICM_INTR_MAP,
    axi_perf_mon_intr_map: AXI_PERF_MON_INTR_MAP,
    usb_device_intr_map: USB_DEVICE_INTR_MAP,
    sdio_host_intr_map: SDIO_HOST_INTR_MAP,
    spi2_intr_map: SPI2_INTR_MAP,
    spi3_intr_map: SPI3_INTR_MAP,
    i2s0_intr_map: I2S0_INTR_MAP,
    i2s1_intr_map: I2S1_INTR_MAP,
    uhci0_intr_map: UHCI0_INTR_MAP,
    uart0_intr_map: UART0_INTR_MAP,
    uart1_intr_map: UART1_INTR_MAP,
    uart2_intr_map: UART2_INTR_MAP,
    uart3_intr_map: UART3_INTR_MAP,
    lcd_cam_intr_map: LCD_CAM_INTR_MAP,
    pwm0_intr_map: PWM0_INTR_MAP,
    pwm1_intr_map: PWM1_INTR_MAP,
    pwm2_intr_map: PWM2_INTR_MAP,
    pwm3_intr_map: PWM3_INTR_MAP,
    can0_intr_map: CAN0_INTR_MAP,
    can0_timer_intr_map: CAN0_TIMER_INTR_MAP,
    can1_intr_map: CAN1_INTR_MAP,
    can1_timer_intr_map: CAN1_TIMER_INTR_MAP,
    rmt_intr_map: RMT_INTR_MAP,
    i2c0_intr_map: I2C0_INTR_MAP,
    i2c1_intr_map: I2C1_INTR_MAP,
    timergrp0_t0_intr_map: TIMERGRP0_T0_INTR_MAP,
    timergrp0_t1_intr_map: TIMERGRP0_T1_INTR_MAP,
    timergrp0_wdt_intr_map: TIMERGRP0_WDT_INTR_MAP,
    timergrp1_t0_intr_map: TIMERGRP1_T0_INTR_MAP,
    timergrp1_t1_intr_map: TIMERGRP1_T1_INTR_MAP,
    timergrp1_wdt_intr_map: TIMERGRP1_WDT_INTR_MAP,
    ledc0_intr_map: LEDC0_INTR_MAP,
    ledc1_intr_map: LEDC1_INTR_MAP,
    systimer_target0_intr_map: SYSTIMER_TARGET0_INTR_MAP,
    systimer_target1_intr_map: SYSTIMER_TARGET1_INTR_MAP,
    systimer_target2_intr_map: SYSTIMER_TARGET2_INTR_MAP,
    ahb_pdma_in_ch0_intr_map: AHB_PDMA_IN_CH0_INTR_MAP,
    ahb_pdma_in_ch1_intr_map: AHB_PDMA_IN_CH1_INTR_MAP,
    ahb_pdma_in_ch2_intr_map: AHB_PDMA_IN_CH2_INTR_MAP,
    ahb_pdma_in_ch3_intr_map: AHB_PDMA_IN_CH3_INTR_MAP,
    ahb_pdma_in_ch4_intr_map: AHB_PDMA_IN_CH4_INTR_MAP,
    ahb_pdma_out_ch0_intr_map: AHB_PDMA_OUT_CH0_INTR_MAP,
    ahb_pdma_out_ch1_intr_map: AHB_PDMA_OUT_CH1_INTR_MAP,
    ahb_pdma_out_ch2_intr_map: AHB_PDMA_OUT_CH2_INTR_MAP,
    ahb_pdma_out_ch3_intr_map: AHB_PDMA_OUT_CH3_INTR_MAP,
    ahb_pdma_out_ch4_intr_map: AHB_PDMA_OUT_CH4_INTR_MAP,
    asrc_chnl0_intr_map: ASRC_CHNL0_INTR_MAP,
    asrc_chnl1_intr_map: ASRC_CHNL1_INTR_MAP,
    axi_pdma_in_ch0_intr_map: AXI_PDMA_IN_CH0_INTR_MAP,
    axi_pdma_in_ch1_intr_map: AXI_PDMA_IN_CH1_INTR_MAP,
    axi_pdma_in_ch2_intr_map: AXI_PDMA_IN_CH2_INTR_MAP,
    axi_pdma_out_ch0_intr_map: AXI_PDMA_OUT_CH0_INTR_MAP,
    axi_pdma_out_ch1_intr_map: AXI_PDMA_OUT_CH1_INTR_MAP,
    axi_pdma_out_ch2_intr_map: AXI_PDMA_OUT_CH2_INTR_MAP,
    rsa_intr_map: RSA_INTR_MAP,
    aes_intr_map: AES_INTR_MAP,
    sha_intr_map: SHA_INTR_MAP,
    ecc_intr_map: ECC_INTR_MAP,
    ecdsa_intr_map: ECDSA_INTR_MAP,
    km_intr_map: KM_INTR_MAP,
    rma_intr_map: RMA_INTR_MAP,
    gpio_intr0_map: GPIO_INTR0_MAP,
    gpio_intr1_map: GPIO_INTR1_MAP,
    gpio_intr2_map: GPIO_INTR2_MAP,
    gpio_intr3_map: GPIO_INTR3_MAP,
    cpu_intr_from_cpu_0_map: CPU_INTR_FROM_CPU_0_MAP,
    cpu_intr_from_cpu_1_map: CPU_INTR_FROM_CPU_1_MAP,
    cpu_intr_from_cpu_2_map: CPU_INTR_FROM_CPU_2_MAP,
    cpu_intr_from_cpu_3_map: CPU_INTR_FROM_CPU_3_MAP,
    cache_intr_map: CACHE_INTR_MAP,
    cpu_apm_m0_intr_map: CPU_APM_M0_INTR_MAP,
    cpu_apm_m1_intr_map: CPU_APM_M1_INTR_MAP,
    cpu_apm_m2_intr_map: CPU_APM_M2_INTR_MAP,
    cpu_apm_m3_intr_map: CPU_APM_M3_INTR_MAP,
    hp_mem_apm_m0_intr_map: HP_MEM_APM_M0_INTR_MAP,
    hp_mem_apm_m1_intr_map: HP_MEM_APM_M1_INTR_MAP,
    hp_mem_apm_m2_intr_map: HP_MEM_APM_M2_INTR_MAP,
    hp_mem_apm_m3_intr_map: HP_MEM_APM_M3_INTR_MAP,
    hp_mem_apm_m4_intr_map: HP_MEM_APM_M4_INTR_MAP,
    hp_mem_apm_m5_intr_map: HP_MEM_APM_M5_INTR_MAP,
    cpu_peri0_timeout_intr_map: CPU_PERI0_TIMEOUT_INTR_MAP,
    cpu_peri1_timeout_intr_map: CPU_PERI1_TIMEOUT_INTR_MAP,
    hp_peri0_timeout_intr_map: HP_PERI0_TIMEOUT_INTR_MAP,
    hp_peri1_timeout_intr_map: HP_PERI1_TIMEOUT_INTR_MAP,
    hp_apm_m0_intr_map: HP_APM_M0_INTR_MAP,
    hp_apm_m1_intr_map: HP_APM_M1_INTR_MAP,
    hp_apm_m2_intr_map: HP_APM_M2_INTR_MAP,
    hp_apm_m3_intr_map: HP_APM_M3_INTR_MAP,
    hp_apm_m4_intr_map: HP_APM_M4_INTR_MAP,
    hp_apm_m5_intr_map: HP_APM_M5_INTR_MAP,
    hp_apm_m6_intr_map: HP_APM_M6_INTR_MAP,
    hp_peri0_pms_intr_map: HP_PERI0_PMS_INTR_MAP,
    hp_peri1_pms_intr_map: HP_PERI1_PMS_INTR_MAP,
    cpu0_peri_pms_intr_map: CPU0_PERI_PMS_INTR_MAP,
    cpu1_peri_pms_intr_map: CPU1_PERI_PMS_INTR_MAP,
    flash_mspi_intr_map: FLASH_MSPI_INTR_MAP,
    lpi_intr_map: LPI_INTR_MAP,
    pmt_intr_map: PMT_INTR_MAP,
    sbd_intr_map: SBD_INTR_MAP,
    usb_otghs_intr_map: USB_OTGHS_INTR_MAP,
    usb_otghs_endp_multi_proc_intr_map: USB_OTGHS_ENDP_MULTI_PROC_INTR_MAP,
    jpeg_intr_map: JPEG_INTR_MAP,
    ppa_intr_map: PPA_INTR_MAP,
    core0_trace_intr_map: CORE0_TRACE_INTR_MAP,
    core1_trace_intr_map: CORE1_TRACE_INTR_MAP,
    dma2d_in_ch0_intr_map: DMA2D_IN_CH0_INTR_MAP,
    dma2d_in_ch1_intr_map: DMA2D_IN_CH1_INTR_MAP,
    dma2d_in_ch2_intr_map: DMA2D_IN_CH2_INTR_MAP,
    dma2d_out_ch0_intr_map: DMA2D_OUT_CH0_INTR_MAP,
    dma2d_out_ch1_intr_map: DMA2D_OUT_CH1_INTR_MAP,
    dma2d_out_ch2_intr_map: DMA2D_OUT_CH2_INTR_MAP,
    dma2d_out_ch3_intr_map: DMA2D_OUT_CH3_INTR_MAP,
    psram_mspi_intr_map: PSRAM_MSPI_INTR_MAP,
    hp_sysreg_intr_map: HP_SYSREG_INTR_MAP,
    pcnt0_intr_map: PCNT0_INTR_MAP,
    pcnt1_intr_map: PCNT1_INTR_MAP,
    hp_pau_intr_map: HP_PAU_INTR_MAP,
    hp_parlio_rx_intr_map: HP_PARLIO_RX_INTR_MAP,
    hp_parlio_tx_intr_map: HP_PARLIO_TX_INTR_MAP,
    bus_monitor_intr_map: BUS_MONITOR_INTR_MAP,
    modem_wifi_mac_intr_map: MODEM_WIFI_MAC_INTR_MAP,
    modem_wifi_mac_nmi_intr_map: MODEM_WIFI_MAC_NMI_INTR_MAP,
    modem_wifi_pwr_intr_map: MODEM_WIFI_PWR_INTR_MAP,
    modem_wifi_bb_intr_map: MODEM_WIFI_BB_INTR_MAP,
    modem_bt_mac_intr_map: MODEM_BT_MAC_INTR_MAP,
    modem_bt_bb_intr_map: MODEM_BT_BB_INTR_MAP,
    modem_bt_bb_nmi_intr_map: MODEM_BT_BB_NMI_INTR_MAP,
    modem_lp_timer_intr_map: MODEM_LP_TIMER_INTR_MAP,
    modem_coex_intr_map: MODEM_COEX_INTR_MAP,
    modem_ble_timer_intr_map: MODEM_BLE_TIMER_INTR_MAP,
    modem_ble_sec_intr_map: MODEM_BLE_SEC_INTR_MAP,
    modem_i2c_mst_intr_map: MODEM_I2C_MST_INTR_MAP,
    modem_zb_mac_intr_map: MODEM_ZB_MAC_INTR_MAP,
    modem_bt_mac_int1_intr_map: MODEM_BT_MAC_INT1_INTR_MAP,
    cordic_intr_map: CORDIC_INTR_MAP,
    zero_det_intr_map: ZERO_DET_INTR_MAP,
    lp_wdt_intr_map: LP_WDT_INTR_MAP,
    lp_timer_reg_0_intr_map: LP_TIMER_REG_0_INTR_MAP,
    lp_timer_reg_1_intr_map: LP_TIMER_REG_1_INTR_MAP,
    mb_hp_intr_map: MB_HP_INTR_MAP,
    mb_lp_intr_map: MB_LP_INTR_MAP,
    pmu_reg_0_intr_map: PMU_REG_0_INTR_MAP,
    pmu_reg_1_intr_map: PMU_REG_1_INTR_MAP,
    lp_anaperi_intr_map: LP_ANAPERI_INTR_MAP,
    lp_adc_intr_map: LP_ADC_INTR_MAP,
    lp_dac_intr_map: LP_DAC_INTR_MAP,
    lp_gpio_intr_map: LP_GPIO_INTR_MAP,
    lp_i2c_intr_map: LP_I2C_INTR_MAP,
    lp_spi_intr_map: LP_SPI_INTR_MAP,
    lp_touch_intr_map: LP_TOUCH_INTR_MAP,
    lp_tsens_intr_map: LP_TSENS_INTR_MAP,
    lp_uart_intr_map: LP_UART_INTR_MAP,
    lp_efuse_intr_map: LP_EFUSE_INTR_MAP,
    lp_sw_intr_map: LP_SW_INTR_MAP,
    lp_trng_intr_map: LP_TRNG_INTR_MAP,
    lp_sysreg_intr_map: LP_SYSREG_INTR_MAP,
    lp_apm_m0_intr_map: LP_APM_M0_INTR_MAP,
    lp_apm_m1_intr_map: LP_APM_M1_INTR_MAP,
    lp_apm_m2_intr_map: LP_APM_M2_INTR_MAP,
    lp_apm_m3_intr_map: LP_APM_M3_INTR_MAP,
    lp_peri0_pms_intr_map: LP_PERI0_PMS_INTR_MAP,
    lp_peri1_pms_intr_map: LP_PERI1_PMS_INTR_MAP,
    lp_huk_intr_map: LP_HUK_INTR_MAP,
    lp_peri_timeout_intr_map: LP_PERI_TIMEOUT_INTR_MAP,
    lp_ahb_pdma_in_ch0_intr_map: LP_AHB_PDMA_IN_CH0_INTR_MAP,
    lp_ahb_pdma_in_ch1_intr_map: LP_AHB_PDMA_IN_CH1_INTR_MAP,
    lp_ahb_pdma_out_ch0_intr_map: LP_AHB_PDMA_OUT_CH0_INTR_MAP,
    lp_ahb_pdma_out_ch1_intr_map: LP_AHB_PDMA_OUT_CH1_INTR_MAP,
    lp_sw_invalid_sleep_intr_map: LP_SW_INVALID_SLEEP_INTR_MAP,
    sig_idx_assert_in_sec: SIG_IDX_ASSERT_IN_SEC,
    core_1_intr_status: [CORE_1_INTR_STATUS; 5],
    core_1_intr_status5: CORE_1_INTR_STATUS5,
    src_pass_in_s_status: [SRC_PASS_IN_S_STATUS; 5],
    src_pass_in_s_status_5: SRC_PASS_IN_S_STATUS_5,
    src_pass_in_m_status: [SRC_PASS_IN_M_STATUS; 5],
    src_pass_in_m_status_5: SRC_PASS_IN_M_STATUS_5,
    config_state: CONFIG_STATE,
    s_status: S_STATUS,
    m_status: M_STATUS,
    clock_gate: CLOCK_GATE,
    _reserved180: [u8; 0x04fc],
    interrupt_date: INTERRUPT_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sys_icm_intr_map(&self) -> &SYS_ICM_INTR_MAP {
        &self.sys_icm_intr_map
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn axi_perf_mon_intr_map(&self) -> &AXI_PERF_MON_INTR_MAP {
        &self.axi_perf_mon_intr_map
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn usb_device_intr_map(&self) -> &USB_DEVICE_INTR_MAP {
        &self.usb_device_intr_map
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn sdio_host_intr_map(&self) -> &SDIO_HOST_INTR_MAP {
        &self.sdio_host_intr_map
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn spi2_intr_map(&self) -> &SPI2_INTR_MAP {
        &self.spi2_intr_map
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn spi3_intr_map(&self) -> &SPI3_INTR_MAP {
        &self.spi3_intr_map
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn i2s0_intr_map(&self) -> &I2S0_INTR_MAP {
        &self.i2s0_intr_map
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn i2s1_intr_map(&self) -> &I2S1_INTR_MAP {
        &self.i2s1_intr_map
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn uhci0_intr_map(&self) -> &UHCI0_INTR_MAP {
        &self.uhci0_intr_map
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn uart0_intr_map(&self) -> &UART0_INTR_MAP {
        &self.uart0_intr_map
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn uart1_intr_map(&self) -> &UART1_INTR_MAP {
        &self.uart1_intr_map
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn uart2_intr_map(&self) -> &UART2_INTR_MAP {
        &self.uart2_intr_map
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn uart3_intr_map(&self) -> &UART3_INTR_MAP {
        &self.uart3_intr_map
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn lcd_cam_intr_map(&self) -> &LCD_CAM_INTR_MAP {
        &self.lcd_cam_intr_map
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn pwm0_intr_map(&self) -> &PWM0_INTR_MAP {
        &self.pwm0_intr_map
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pwm1_intr_map(&self) -> &PWM1_INTR_MAP {
        &self.pwm1_intr_map
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn pwm2_intr_map(&self) -> &PWM2_INTR_MAP {
        &self.pwm2_intr_map
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn pwm3_intr_map(&self) -> &PWM3_INTR_MAP {
        &self.pwm3_intr_map
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn can0_intr_map(&self) -> &CAN0_INTR_MAP {
        &self.can0_intr_map
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn can0_timer_intr_map(&self) -> &CAN0_TIMER_INTR_MAP {
        &self.can0_timer_intr_map
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn can1_intr_map(&self) -> &CAN1_INTR_MAP {
        &self.can1_intr_map
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn can1_timer_intr_map(&self) -> &CAN1_TIMER_INTR_MAP {
        &self.can1_timer_intr_map
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn rmt_intr_map(&self) -> &RMT_INTR_MAP {
        &self.rmt_intr_map
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn i2c0_intr_map(&self) -> &I2C0_INTR_MAP {
        &self.i2c0_intr_map
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn i2c1_intr_map(&self) -> &I2C1_INTR_MAP {
        &self.i2c1_intr_map
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn timergrp0_t0_intr_map(&self) -> &TIMERGRP0_T0_INTR_MAP {
        &self.timergrp0_t0_intr_map
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn timergrp0_t1_intr_map(&self) -> &TIMERGRP0_T1_INTR_MAP {
        &self.timergrp0_t1_intr_map
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn timergrp0_wdt_intr_map(&self) -> &TIMERGRP0_WDT_INTR_MAP {
        &self.timergrp0_wdt_intr_map
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn timergrp1_t0_intr_map(&self) -> &TIMERGRP1_T0_INTR_MAP {
        &self.timergrp1_t0_intr_map
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn timergrp1_t1_intr_map(&self) -> &TIMERGRP1_T1_INTR_MAP {
        &self.timergrp1_t1_intr_map
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn timergrp1_wdt_intr_map(&self) -> &TIMERGRP1_WDT_INTR_MAP {
        &self.timergrp1_wdt_intr_map
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn ledc0_intr_map(&self) -> &LEDC0_INTR_MAP {
        &self.ledc0_intr_map
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn ledc1_intr_map(&self) -> &LEDC1_INTR_MAP {
        &self.ledc1_intr_map
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn systimer_target0_intr_map(&self) -> &SYSTIMER_TARGET0_INTR_MAP {
        &self.systimer_target0_intr_map
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn systimer_target1_intr_map(&self) -> &SYSTIMER_TARGET1_INTR_MAP {
        &self.systimer_target1_intr_map
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn systimer_target2_intr_map(&self) -> &SYSTIMER_TARGET2_INTR_MAP {
        &self.systimer_target2_intr_map
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch0_intr_map(&self) -> &AHB_PDMA_IN_CH0_INTR_MAP {
        &self.ahb_pdma_in_ch0_intr_map
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch1_intr_map(&self) -> &AHB_PDMA_IN_CH1_INTR_MAP {
        &self.ahb_pdma_in_ch1_intr_map
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch2_intr_map(&self) -> &AHB_PDMA_IN_CH2_INTR_MAP {
        &self.ahb_pdma_in_ch2_intr_map
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch3_intr_map(&self) -> &AHB_PDMA_IN_CH3_INTR_MAP {
        &self.ahb_pdma_in_ch3_intr_map
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch4_intr_map(&self) -> &AHB_PDMA_IN_CH4_INTR_MAP {
        &self.ahb_pdma_in_ch4_intr_map
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch0_intr_map(&self) -> &AHB_PDMA_OUT_CH0_INTR_MAP {
        &self.ahb_pdma_out_ch0_intr_map
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch1_intr_map(&self) -> &AHB_PDMA_OUT_CH1_INTR_MAP {
        &self.ahb_pdma_out_ch1_intr_map
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch2_intr_map(&self) -> &AHB_PDMA_OUT_CH2_INTR_MAP {
        &self.ahb_pdma_out_ch2_intr_map
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch3_intr_map(&self) -> &AHB_PDMA_OUT_CH3_INTR_MAP {
        &self.ahb_pdma_out_ch3_intr_map
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch4_intr_map(&self) -> &AHB_PDMA_OUT_CH4_INTR_MAP {
        &self.ahb_pdma_out_ch4_intr_map
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn asrc_chnl0_intr_map(&self) -> &ASRC_CHNL0_INTR_MAP {
        &self.asrc_chnl0_intr_map
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn asrc_chnl1_intr_map(&self) -> &ASRC_CHNL1_INTR_MAP {
        &self.asrc_chnl1_intr_map
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn axi_pdma_in_ch0_intr_map(&self) -> &AXI_PDMA_IN_CH0_INTR_MAP {
        &self.axi_pdma_in_ch0_intr_map
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn axi_pdma_in_ch1_intr_map(&self) -> &AXI_PDMA_IN_CH1_INTR_MAP {
        &self.axi_pdma_in_ch1_intr_map
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn axi_pdma_in_ch2_intr_map(&self) -> &AXI_PDMA_IN_CH2_INTR_MAP {
        &self.axi_pdma_in_ch2_intr_map
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn axi_pdma_out_ch0_intr_map(&self) -> &AXI_PDMA_OUT_CH0_INTR_MAP {
        &self.axi_pdma_out_ch0_intr_map
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn axi_pdma_out_ch1_intr_map(&self) -> &AXI_PDMA_OUT_CH1_INTR_MAP {
        &self.axi_pdma_out_ch1_intr_map
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn axi_pdma_out_ch2_intr_map(&self) -> &AXI_PDMA_OUT_CH2_INTR_MAP {
        &self.axi_pdma_out_ch2_intr_map
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn rsa_intr_map(&self) -> &RSA_INTR_MAP {
        &self.rsa_intr_map
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn aes_intr_map(&self) -> &AES_INTR_MAP {
        &self.aes_intr_map
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn sha_intr_map(&self) -> &SHA_INTR_MAP {
        &self.sha_intr_map
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn ecc_intr_map(&self) -> &ECC_INTR_MAP {
        &self.ecc_intr_map
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn ecdsa_intr_map(&self) -> &ECDSA_INTR_MAP {
        &self.ecdsa_intr_map
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn km_intr_map(&self) -> &KM_INTR_MAP {
        &self.km_intr_map
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn rma_intr_map(&self) -> &RMA_INTR_MAP {
        &self.rma_intr_map
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn gpio_intr0_map(&self) -> &GPIO_INTR0_MAP {
        &self.gpio_intr0_map
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn gpio_intr1_map(&self) -> &GPIO_INTR1_MAP {
        &self.gpio_intr1_map
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn gpio_intr2_map(&self) -> &GPIO_INTR2_MAP {
        &self.gpio_intr2_map
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn gpio_intr3_map(&self) -> &GPIO_INTR3_MAP {
        &self.gpio_intr3_map
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0_map(&self) -> &CPU_INTR_FROM_CPU_0_MAP {
        &self.cpu_intr_from_cpu_0_map
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1_map(&self) -> &CPU_INTR_FROM_CPU_1_MAP {
        &self.cpu_intr_from_cpu_1_map
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2_map(&self) -> &CPU_INTR_FROM_CPU_2_MAP {
        &self.cpu_intr_from_cpu_2_map
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3_map(&self) -> &CPU_INTR_FROM_CPU_3_MAP {
        &self.cpu_intr_from_cpu_3_map
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn cache_intr_map(&self) -> &CACHE_INTR_MAP {
        &self.cache_intr_map
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn cpu_apm_m0_intr_map(&self) -> &CPU_APM_M0_INTR_MAP {
        &self.cpu_apm_m0_intr_map
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn cpu_apm_m1_intr_map(&self) -> &CPU_APM_M1_INTR_MAP {
        &self.cpu_apm_m1_intr_map
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn cpu_apm_m2_intr_map(&self) -> &CPU_APM_M2_INTR_MAP {
        &self.cpu_apm_m2_intr_map
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn cpu_apm_m3_intr_map(&self) -> &CPU_APM_M3_INTR_MAP {
        &self.cpu_apm_m3_intr_map
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m0_intr_map(&self) -> &HP_MEM_APM_M0_INTR_MAP {
        &self.hp_mem_apm_m0_intr_map
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m1_intr_map(&self) -> &HP_MEM_APM_M1_INTR_MAP {
        &self.hp_mem_apm_m1_intr_map
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m2_intr_map(&self) -> &HP_MEM_APM_M2_INTR_MAP {
        &self.hp_mem_apm_m2_intr_map
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m3_intr_map(&self) -> &HP_MEM_APM_M3_INTR_MAP {
        &self.hp_mem_apm_m3_intr_map
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m4_intr_map(&self) -> &HP_MEM_APM_M4_INTR_MAP {
        &self.hp_mem_apm_m4_intr_map
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn hp_mem_apm_m5_intr_map(&self) -> &HP_MEM_APM_M5_INTR_MAP {
        &self.hp_mem_apm_m5_intr_map
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn cpu_peri0_timeout_intr_map(&self) -> &CPU_PERI0_TIMEOUT_INTR_MAP {
        &self.cpu_peri0_timeout_intr_map
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn cpu_peri1_timeout_intr_map(&self) -> &CPU_PERI1_TIMEOUT_INTR_MAP {
        &self.cpu_peri1_timeout_intr_map
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn hp_peri0_timeout_intr_map(&self) -> &HP_PERI0_TIMEOUT_INTR_MAP {
        &self.hp_peri0_timeout_intr_map
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn hp_peri1_timeout_intr_map(&self) -> &HP_PERI1_TIMEOUT_INTR_MAP {
        &self.hp_peri1_timeout_intr_map
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn hp_apm_m0_intr_map(&self) -> &HP_APM_M0_INTR_MAP {
        &self.hp_apm_m0_intr_map
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn hp_apm_m1_intr_map(&self) -> &HP_APM_M1_INTR_MAP {
        &self.hp_apm_m1_intr_map
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn hp_apm_m2_intr_map(&self) -> &HP_APM_M2_INTR_MAP {
        &self.hp_apm_m2_intr_map
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn hp_apm_m3_intr_map(&self) -> &HP_APM_M3_INTR_MAP {
        &self.hp_apm_m3_intr_map
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn hp_apm_m4_intr_map(&self) -> &HP_APM_M4_INTR_MAP {
        &self.hp_apm_m4_intr_map
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn hp_apm_m5_intr_map(&self) -> &HP_APM_M5_INTR_MAP {
        &self.hp_apm_m5_intr_map
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn hp_apm_m6_intr_map(&self) -> &HP_APM_M6_INTR_MAP {
        &self.hp_apm_m6_intr_map
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn hp_peri0_pms_intr_map(&self) -> &HP_PERI0_PMS_INTR_MAP {
        &self.hp_peri0_pms_intr_map
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn hp_peri1_pms_intr_map(&self) -> &HP_PERI1_PMS_INTR_MAP {
        &self.hp_peri1_pms_intr_map
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn cpu0_peri_pms_intr_map(&self) -> &CPU0_PERI_PMS_INTR_MAP {
        &self.cpu0_peri_pms_intr_map
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn cpu1_peri_pms_intr_map(&self) -> &CPU1_PERI_PMS_INTR_MAP {
        &self.cpu1_peri_pms_intr_map
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn flash_mspi_intr_map(&self) -> &FLASH_MSPI_INTR_MAP {
        &self.flash_mspi_intr_map
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn lpi_intr_map(&self) -> &LPI_INTR_MAP {
        &self.lpi_intr_map
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn pmt_intr_map(&self) -> &PMT_INTR_MAP {
        &self.pmt_intr_map
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn sbd_intr_map(&self) -> &SBD_INTR_MAP {
        &self.sbd_intr_map
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn usb_otghs_intr_map(&self) -> &USB_OTGHS_INTR_MAP {
        &self.usb_otghs_intr_map
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn usb_otghs_endp_multi_proc_intr_map(&self) -> &USB_OTGHS_ENDP_MULTI_PROC_INTR_MAP {
        &self.usb_otghs_endp_multi_proc_intr_map
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn jpeg_intr_map(&self) -> &JPEG_INTR_MAP {
        &self.jpeg_intr_map
    }
    #[doc = "0x198 - "]
    #[inline(always)]
    pub const fn ppa_intr_map(&self) -> &PPA_INTR_MAP {
        &self.ppa_intr_map
    }
    #[doc = "0x19c - "]
    #[inline(always)]
    pub const fn core0_trace_intr_map(&self) -> &CORE0_TRACE_INTR_MAP {
        &self.core0_trace_intr_map
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn core1_trace_intr_map(&self) -> &CORE1_TRACE_INTR_MAP {
        &self.core1_trace_intr_map
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn dma2d_in_ch0_intr_map(&self) -> &DMA2D_IN_CH0_INTR_MAP {
        &self.dma2d_in_ch0_intr_map
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn dma2d_in_ch1_intr_map(&self) -> &DMA2D_IN_CH1_INTR_MAP {
        &self.dma2d_in_ch1_intr_map
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn dma2d_in_ch2_intr_map(&self) -> &DMA2D_IN_CH2_INTR_MAP {
        &self.dma2d_in_ch2_intr_map
    }
    #[doc = "0x1b0 - "]
    #[inline(always)]
    pub const fn dma2d_out_ch0_intr_map(&self) -> &DMA2D_OUT_CH0_INTR_MAP {
        &self.dma2d_out_ch0_intr_map
    }
    #[doc = "0x1b4 - "]
    #[inline(always)]
    pub const fn dma2d_out_ch1_intr_map(&self) -> &DMA2D_OUT_CH1_INTR_MAP {
        &self.dma2d_out_ch1_intr_map
    }
    #[doc = "0x1b8 - "]
    #[inline(always)]
    pub const fn dma2d_out_ch2_intr_map(&self) -> &DMA2D_OUT_CH2_INTR_MAP {
        &self.dma2d_out_ch2_intr_map
    }
    #[doc = "0x1bc - "]
    #[inline(always)]
    pub const fn dma2d_out_ch3_intr_map(&self) -> &DMA2D_OUT_CH3_INTR_MAP {
        &self.dma2d_out_ch3_intr_map
    }
    #[doc = "0x1c0 - "]
    #[inline(always)]
    pub const fn psram_mspi_intr_map(&self) -> &PSRAM_MSPI_INTR_MAP {
        &self.psram_mspi_intr_map
    }
    #[doc = "0x1c4 - "]
    #[inline(always)]
    pub const fn hp_sysreg_intr_map(&self) -> &HP_SYSREG_INTR_MAP {
        &self.hp_sysreg_intr_map
    }
    #[doc = "0x1c8 - "]
    #[inline(always)]
    pub const fn pcnt0_intr_map(&self) -> &PCNT0_INTR_MAP {
        &self.pcnt0_intr_map
    }
    #[doc = "0x1cc - "]
    #[inline(always)]
    pub const fn pcnt1_intr_map(&self) -> &PCNT1_INTR_MAP {
        &self.pcnt1_intr_map
    }
    #[doc = "0x1d0 - "]
    #[inline(always)]
    pub const fn hp_pau_intr_map(&self) -> &HP_PAU_INTR_MAP {
        &self.hp_pau_intr_map
    }
    #[doc = "0x1d4 - "]
    #[inline(always)]
    pub const fn hp_parlio_rx_intr_map(&self) -> &HP_PARLIO_RX_INTR_MAP {
        &self.hp_parlio_rx_intr_map
    }
    #[doc = "0x1d8 - "]
    #[inline(always)]
    pub const fn hp_parlio_tx_intr_map(&self) -> &HP_PARLIO_TX_INTR_MAP {
        &self.hp_parlio_tx_intr_map
    }
    #[doc = "0x1dc - "]
    #[inline(always)]
    pub const fn bus_monitor_intr_map(&self) -> &BUS_MONITOR_INTR_MAP {
        &self.bus_monitor_intr_map
    }
    #[doc = "0x1e0 - "]
    #[inline(always)]
    pub const fn modem_wifi_mac_intr_map(&self) -> &MODEM_WIFI_MAC_INTR_MAP {
        &self.modem_wifi_mac_intr_map
    }
    #[doc = "0x1e4 - "]
    #[inline(always)]
    pub const fn modem_wifi_mac_nmi_intr_map(&self) -> &MODEM_WIFI_MAC_NMI_INTR_MAP {
        &self.modem_wifi_mac_nmi_intr_map
    }
    #[doc = "0x1e8 - "]
    #[inline(always)]
    pub const fn modem_wifi_pwr_intr_map(&self) -> &MODEM_WIFI_PWR_INTR_MAP {
        &self.modem_wifi_pwr_intr_map
    }
    #[doc = "0x1ec - "]
    #[inline(always)]
    pub const fn modem_wifi_bb_intr_map(&self) -> &MODEM_WIFI_BB_INTR_MAP {
        &self.modem_wifi_bb_intr_map
    }
    #[doc = "0x1f0 - "]
    #[inline(always)]
    pub const fn modem_bt_mac_intr_map(&self) -> &MODEM_BT_MAC_INTR_MAP {
        &self.modem_bt_mac_intr_map
    }
    #[doc = "0x1f4 - "]
    #[inline(always)]
    pub const fn modem_bt_bb_intr_map(&self) -> &MODEM_BT_BB_INTR_MAP {
        &self.modem_bt_bb_intr_map
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn modem_bt_bb_nmi_intr_map(&self) -> &MODEM_BT_BB_NMI_INTR_MAP {
        &self.modem_bt_bb_nmi_intr_map
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn modem_lp_timer_intr_map(&self) -> &MODEM_LP_TIMER_INTR_MAP {
        &self.modem_lp_timer_intr_map
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn modem_coex_intr_map(&self) -> &MODEM_COEX_INTR_MAP {
        &self.modem_coex_intr_map
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn modem_ble_timer_intr_map(&self) -> &MODEM_BLE_TIMER_INTR_MAP {
        &self.modem_ble_timer_intr_map
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn modem_ble_sec_intr_map(&self) -> &MODEM_BLE_SEC_INTR_MAP {
        &self.modem_ble_sec_intr_map
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn modem_i2c_mst_intr_map(&self) -> &MODEM_I2C_MST_INTR_MAP {
        &self.modem_i2c_mst_intr_map
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn modem_zb_mac_intr_map(&self) -> &MODEM_ZB_MAC_INTR_MAP {
        &self.modem_zb_mac_intr_map
    }
    #[doc = "0x214 - "]
    #[inline(always)]
    pub const fn modem_bt_mac_int1_intr_map(&self) -> &MODEM_BT_MAC_INT1_INTR_MAP {
        &self.modem_bt_mac_int1_intr_map
    }
    #[doc = "0x218 - "]
    #[inline(always)]
    pub const fn cordic_intr_map(&self) -> &CORDIC_INTR_MAP {
        &self.cordic_intr_map
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn zero_det_intr_map(&self) -> &ZERO_DET_INTR_MAP {
        &self.zero_det_intr_map
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn lp_wdt_intr_map(&self) -> &LP_WDT_INTR_MAP {
        &self.lp_wdt_intr_map
    }
    #[doc = "0x224 - "]
    #[inline(always)]
    pub const fn lp_timer_reg_0_intr_map(&self) -> &LP_TIMER_REG_0_INTR_MAP {
        &self.lp_timer_reg_0_intr_map
    }
    #[doc = "0x228 - "]
    #[inline(always)]
    pub const fn lp_timer_reg_1_intr_map(&self) -> &LP_TIMER_REG_1_INTR_MAP {
        &self.lp_timer_reg_1_intr_map
    }
    #[doc = "0x22c - "]
    #[inline(always)]
    pub const fn mb_hp_intr_map(&self) -> &MB_HP_INTR_MAP {
        &self.mb_hp_intr_map
    }
    #[doc = "0x230 - "]
    #[inline(always)]
    pub const fn mb_lp_intr_map(&self) -> &MB_LP_INTR_MAP {
        &self.mb_lp_intr_map
    }
    #[doc = "0x234 - "]
    #[inline(always)]
    pub const fn pmu_reg_0_intr_map(&self) -> &PMU_REG_0_INTR_MAP {
        &self.pmu_reg_0_intr_map
    }
    #[doc = "0x238 - "]
    #[inline(always)]
    pub const fn pmu_reg_1_intr_map(&self) -> &PMU_REG_1_INTR_MAP {
        &self.pmu_reg_1_intr_map
    }
    #[doc = "0x23c - "]
    #[inline(always)]
    pub const fn lp_anaperi_intr_map(&self) -> &LP_ANAPERI_INTR_MAP {
        &self.lp_anaperi_intr_map
    }
    #[doc = "0x240 - "]
    #[inline(always)]
    pub const fn lp_adc_intr_map(&self) -> &LP_ADC_INTR_MAP {
        &self.lp_adc_intr_map
    }
    #[doc = "0x244 - "]
    #[inline(always)]
    pub const fn lp_dac_intr_map(&self) -> &LP_DAC_INTR_MAP {
        &self.lp_dac_intr_map
    }
    #[doc = "0x248 - "]
    #[inline(always)]
    pub const fn lp_gpio_intr_map(&self) -> &LP_GPIO_INTR_MAP {
        &self.lp_gpio_intr_map
    }
    #[doc = "0x24c - "]
    #[inline(always)]
    pub const fn lp_i2c_intr_map(&self) -> &LP_I2C_INTR_MAP {
        &self.lp_i2c_intr_map
    }
    #[doc = "0x250 - "]
    #[inline(always)]
    pub const fn lp_spi_intr_map(&self) -> &LP_SPI_INTR_MAP {
        &self.lp_spi_intr_map
    }
    #[doc = "0x254 - "]
    #[inline(always)]
    pub const fn lp_touch_intr_map(&self) -> &LP_TOUCH_INTR_MAP {
        &self.lp_touch_intr_map
    }
    #[doc = "0x258 - "]
    #[inline(always)]
    pub const fn lp_tsens_intr_map(&self) -> &LP_TSENS_INTR_MAP {
        &self.lp_tsens_intr_map
    }
    #[doc = "0x25c - "]
    #[inline(always)]
    pub const fn lp_uart_intr_map(&self) -> &LP_UART_INTR_MAP {
        &self.lp_uart_intr_map
    }
    #[doc = "0x260 - "]
    #[inline(always)]
    pub const fn lp_efuse_intr_map(&self) -> &LP_EFUSE_INTR_MAP {
        &self.lp_efuse_intr_map
    }
    #[doc = "0x264 - "]
    #[inline(always)]
    pub const fn lp_sw_intr_map(&self) -> &LP_SW_INTR_MAP {
        &self.lp_sw_intr_map
    }
    #[doc = "0x268 - "]
    #[inline(always)]
    pub const fn lp_trng_intr_map(&self) -> &LP_TRNG_INTR_MAP {
        &self.lp_trng_intr_map
    }
    #[doc = "0x26c - "]
    #[inline(always)]
    pub const fn lp_sysreg_intr_map(&self) -> &LP_SYSREG_INTR_MAP {
        &self.lp_sysreg_intr_map
    }
    #[doc = "0x270 - "]
    #[inline(always)]
    pub const fn lp_apm_m0_intr_map(&self) -> &LP_APM_M0_INTR_MAP {
        &self.lp_apm_m0_intr_map
    }
    #[doc = "0x274 - "]
    #[inline(always)]
    pub const fn lp_apm_m1_intr_map(&self) -> &LP_APM_M1_INTR_MAP {
        &self.lp_apm_m1_intr_map
    }
    #[doc = "0x278 - "]
    #[inline(always)]
    pub const fn lp_apm_m2_intr_map(&self) -> &LP_APM_M2_INTR_MAP {
        &self.lp_apm_m2_intr_map
    }
    #[doc = "0x27c - "]
    #[inline(always)]
    pub const fn lp_apm_m3_intr_map(&self) -> &LP_APM_M3_INTR_MAP {
        &self.lp_apm_m3_intr_map
    }
    #[doc = "0x280 - "]
    #[inline(always)]
    pub const fn lp_peri0_pms_intr_map(&self) -> &LP_PERI0_PMS_INTR_MAP {
        &self.lp_peri0_pms_intr_map
    }
    #[doc = "0x284 - "]
    #[inline(always)]
    pub const fn lp_peri1_pms_intr_map(&self) -> &LP_PERI1_PMS_INTR_MAP {
        &self.lp_peri1_pms_intr_map
    }
    #[doc = "0x288 - "]
    #[inline(always)]
    pub const fn lp_huk_intr_map(&self) -> &LP_HUK_INTR_MAP {
        &self.lp_huk_intr_map
    }
    #[doc = "0x28c - "]
    #[inline(always)]
    pub const fn lp_peri_timeout_intr_map(&self) -> &LP_PERI_TIMEOUT_INTR_MAP {
        &self.lp_peri_timeout_intr_map
    }
    #[doc = "0x290 - "]
    #[inline(always)]
    pub const fn lp_ahb_pdma_in_ch0_intr_map(&self) -> &LP_AHB_PDMA_IN_CH0_INTR_MAP {
        &self.lp_ahb_pdma_in_ch0_intr_map
    }
    #[doc = "0x294 - "]
    #[inline(always)]
    pub const fn lp_ahb_pdma_in_ch1_intr_map(&self) -> &LP_AHB_PDMA_IN_CH1_INTR_MAP {
        &self.lp_ahb_pdma_in_ch1_intr_map
    }
    #[doc = "0x298 - "]
    #[inline(always)]
    pub const fn lp_ahb_pdma_out_ch0_intr_map(&self) -> &LP_AHB_PDMA_OUT_CH0_INTR_MAP {
        &self.lp_ahb_pdma_out_ch0_intr_map
    }
    #[doc = "0x29c - "]
    #[inline(always)]
    pub const fn lp_ahb_pdma_out_ch1_intr_map(&self) -> &LP_AHB_PDMA_OUT_CH1_INTR_MAP {
        &self.lp_ahb_pdma_out_ch1_intr_map
    }
    #[doc = "0x2a0 - "]
    #[inline(always)]
    pub const fn lp_sw_invalid_sleep_intr_map(&self) -> &LP_SW_INVALID_SLEEP_INTR_MAP {
        &self.lp_sw_invalid_sleep_intr_map
    }
    #[doc = "0x2a4 - "]
    #[inline(always)]
    pub const fn sig_idx_assert_in_sec(&self) -> &SIG_IDX_ASSERT_IN_SEC {
        &self.sig_idx_assert_in_sec
    }
    #[doc = "0x2a8..0x2bc - "]
    #[inline(always)]
    pub const fn core_1_intr_status(&self, n: usize) -> &CORE_1_INTR_STATUS {
        &self.core_1_intr_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2a8..0x2bc - "]
    #[inline(always)]
    pub fn core_1_intr_status_iter(&self) -> impl Iterator<Item = &CORE_1_INTR_STATUS> {
        self.core_1_intr_status.iter()
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn core_1_intr_status5(&self) -> &CORE_1_INTR_STATUS5 {
        &self.core_1_intr_status5
    }
    #[doc = "0x2c0..0x2d4 - "]
    #[inline(always)]
    pub const fn src_pass_in_s_status(&self, n: usize) -> &SRC_PASS_IN_S_STATUS {
        &self.src_pass_in_s_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2d4 - "]
    #[inline(always)]
    pub fn src_pass_in_s_status_iter(&self) -> impl Iterator<Item = &SRC_PASS_IN_S_STATUS> {
        self.src_pass_in_s_status.iter()
    }
    #[doc = "0x2d4 - "]
    #[inline(always)]
    pub const fn src_pass_in_s_status_5(&self) -> &SRC_PASS_IN_S_STATUS_5 {
        &self.src_pass_in_s_status_5
    }
    #[doc = "0x2d8..0x2ec - "]
    #[inline(always)]
    pub const fn src_pass_in_m_status(&self, n: usize) -> &SRC_PASS_IN_M_STATUS {
        &self.src_pass_in_m_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d8..0x2ec - "]
    #[inline(always)]
    pub fn src_pass_in_m_status_iter(&self) -> impl Iterator<Item = &SRC_PASS_IN_M_STATUS> {
        self.src_pass_in_m_status.iter()
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn src_pass_in_m_status_5(&self) -> &SRC_PASS_IN_M_STATUS_5 {
        &self.src_pass_in_m_status_5
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn config_state(&self) -> &CONFIG_STATE {
        &self.config_state
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn s_status(&self) -> &S_STATUS {
        &self.s_status
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn m_status(&self) -> &M_STATUS {
        &self.m_status
    }
    #[doc = "0x2fc - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x7fc - "]
    #[inline(always)]
    pub const fn interrupt_date(&self) -> &INTERRUPT_DATE {
        &self.interrupt_date
    }
}
#[doc = "SYS_ICM_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_icm_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_icm_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_icm_intr_map`] module"]
pub type SYS_ICM_INTR_MAP = crate::Reg<sys_icm_intr_map::SYS_ICM_INTR_MAP_SPEC>;
#[doc = ""]
pub mod sys_icm_intr_map;
#[doc = "AXI_PERF_MON_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_perf_mon_intr_map`] module"]
pub type AXI_PERF_MON_INTR_MAP = crate::Reg<axi_perf_mon_intr_map::AXI_PERF_MON_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_perf_mon_intr_map;
#[doc = "USB_DEVICE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_intr_map`] module"]
pub type USB_DEVICE_INTR_MAP = crate::Reg<usb_device_intr_map::USB_DEVICE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod usb_device_intr_map;
#[doc = "SDIO_HOST_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_host_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_host_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_host_intr_map`] module"]
pub type SDIO_HOST_INTR_MAP = crate::Reg<sdio_host_intr_map::SDIO_HOST_INTR_MAP_SPEC>;
#[doc = ""]
pub mod sdio_host_intr_map;
#[doc = "SPI2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_intr_map`] module"]
pub type SPI2_INTR_MAP = crate::Reg<spi2_intr_map::SPI2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod spi2_intr_map;
#[doc = "SPI3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi3_intr_map`] module"]
pub type SPI3_INTR_MAP = crate::Reg<spi3_intr_map::SPI3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod spi3_intr_map;
#[doc = "I2S0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_intr_map`] module"]
pub type I2S0_INTR_MAP = crate::Reg<i2s0_intr_map::I2S0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod i2s0_intr_map;
#[doc = "I2S1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_intr_map`] module"]
pub type I2S1_INTR_MAP = crate::Reg<i2s1_intr_map::I2S1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod i2s1_intr_map;
#[doc = "UHCI0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uhci0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci0_intr_map`] module"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod uhci0_intr_map;
#[doc = "UART0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_intr_map`] module"]
pub type UART0_INTR_MAP = crate::Reg<uart0_intr_map::UART0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod uart0_intr_map;
#[doc = "UART1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_intr_map`] module"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod uart1_intr_map;
#[doc = "UART2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_intr_map`] module"]
pub type UART2_INTR_MAP = crate::Reg<uart2_intr_map::UART2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod uart2_intr_map;
#[doc = "UART3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_intr_map`] module"]
pub type UART3_INTR_MAP = crate::Reg<uart3_intr_map::UART3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod uart3_intr_map;
#[doc = "LCD_CAM_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_cam_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cam_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cam_intr_map`] module"]
pub type LCD_CAM_INTR_MAP = crate::Reg<lcd_cam_intr_map::LCD_CAM_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lcd_cam_intr_map;
#[doc = "PWM0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_intr_map`] module"]
pub type PWM0_INTR_MAP = crate::Reg<pwm0_intr_map::PWM0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pwm0_intr_map;
#[doc = "PWM1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_intr_map`] module"]
pub type PWM1_INTR_MAP = crate::Reg<pwm1_intr_map::PWM1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pwm1_intr_map;
#[doc = "PWM2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_intr_map`] module"]
pub type PWM2_INTR_MAP = crate::Reg<pwm2_intr_map::PWM2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pwm2_intr_map;
#[doc = "PWM3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_intr_map`] module"]
pub type PWM3_INTR_MAP = crate::Reg<pwm3_intr_map::PWM3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pwm3_intr_map;
#[doc = "CAN0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`can0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_intr_map`] module"]
pub type CAN0_INTR_MAP = crate::Reg<can0_intr_map::CAN0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod can0_intr_map;
#[doc = "CAN0_TIMER_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`can0_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can0_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_timer_intr_map`] module"]
pub type CAN0_TIMER_INTR_MAP = crate::Reg<can0_timer_intr_map::CAN0_TIMER_INTR_MAP_SPEC>;
#[doc = ""]
pub mod can0_timer_intr_map;
#[doc = "CAN1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`can1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1_intr_map`] module"]
pub type CAN1_INTR_MAP = crate::Reg<can1_intr_map::CAN1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod can1_intr_map;
#[doc = "CAN1_TIMER_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`can1_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1_timer_intr_map`] module"]
pub type CAN1_TIMER_INTR_MAP = crate::Reg<can1_timer_intr_map::CAN1_TIMER_INTR_MAP_SPEC>;
#[doc = ""]
pub mod can1_timer_intr_map;
#[doc = "RMT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_intr_map`] module"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod rmt_intr_map;
#[doc = "I2C0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_intr_map`] module"]
pub type I2C0_INTR_MAP = crate::Reg<i2c0_intr_map::I2C0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod i2c0_intr_map;
#[doc = "I2C1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_intr_map`] module"]
pub type I2C1_INTR_MAP = crate::Reg<i2c1_intr_map::I2C1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod i2c1_intr_map;
#[doc = "TIMERGRP0_T0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_t0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_t0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_t0_intr_map`] module"]
pub type TIMERGRP0_T0_INTR_MAP = crate::Reg<timergrp0_t0_intr_map::TIMERGRP0_T0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp0_t0_intr_map;
#[doc = "TIMERGRP0_T1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_t1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_t1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_t1_intr_map`] module"]
pub type TIMERGRP0_T1_INTR_MAP = crate::Reg<timergrp0_t1_intr_map::TIMERGRP0_T1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp0_t1_intr_map;
#[doc = "TIMERGRP0_WDT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_wdt_intr_map`] module"]
pub type TIMERGRP0_WDT_INTR_MAP = crate::Reg<timergrp0_wdt_intr_map::TIMERGRP0_WDT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp0_wdt_intr_map;
#[doc = "TIMERGRP1_T0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_t0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_t0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_t0_intr_map`] module"]
pub type TIMERGRP1_T0_INTR_MAP = crate::Reg<timergrp1_t0_intr_map::TIMERGRP1_T0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp1_t0_intr_map;
#[doc = "TIMERGRP1_T1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_t1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_t1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_t1_intr_map`] module"]
pub type TIMERGRP1_T1_INTR_MAP = crate::Reg<timergrp1_t1_intr_map::TIMERGRP1_T1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp1_t1_intr_map;
#[doc = "TIMERGRP1_WDT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_wdt_intr_map`] module"]
pub type TIMERGRP1_WDT_INTR_MAP = crate::Reg<timergrp1_wdt_intr_map::TIMERGRP1_WDT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod timergrp1_wdt_intr_map;
#[doc = "LEDC0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ledc0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc0_intr_map`] module"]
pub type LEDC0_INTR_MAP = crate::Reg<ledc0_intr_map::LEDC0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ledc0_intr_map;
#[doc = "LEDC1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ledc1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc1_intr_map`] module"]
pub type LEDC1_INTR_MAP = crate::Reg<ledc1_intr_map::LEDC1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ledc1_intr_map;
#[doc = "SYSTIMER_TARGET0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target0_intr_map`] module"]
pub type SYSTIMER_TARGET0_INTR_MAP =
    crate::Reg<systimer_target0_intr_map::SYSTIMER_TARGET0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod systimer_target0_intr_map;
#[doc = "SYSTIMER_TARGET1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target1_intr_map`] module"]
pub type SYSTIMER_TARGET1_INTR_MAP =
    crate::Reg<systimer_target1_intr_map::SYSTIMER_TARGET1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod systimer_target1_intr_map;
#[doc = "SYSTIMER_TARGET2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target2_intr_map`] module"]
pub type SYSTIMER_TARGET2_INTR_MAP =
    crate::Reg<systimer_target2_intr_map::SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod systimer_target2_intr_map;
#[doc = "AHB_PDMA_IN_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_in_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_in_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch0_intr_map`] module"]
pub type AHB_PDMA_IN_CH0_INTR_MAP =
    crate::Reg<ahb_pdma_in_ch0_intr_map::AHB_PDMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_in_ch0_intr_map;
#[doc = "AHB_PDMA_IN_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_in_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_in_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch1_intr_map`] module"]
pub type AHB_PDMA_IN_CH1_INTR_MAP =
    crate::Reg<ahb_pdma_in_ch1_intr_map::AHB_PDMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_in_ch1_intr_map;
#[doc = "AHB_PDMA_IN_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_in_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_in_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch2_intr_map`] module"]
pub type AHB_PDMA_IN_CH2_INTR_MAP =
    crate::Reg<ahb_pdma_in_ch2_intr_map::AHB_PDMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_in_ch2_intr_map;
#[doc = "AHB_PDMA_IN_CH3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_in_ch3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_in_ch3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch3_intr_map`] module"]
pub type AHB_PDMA_IN_CH3_INTR_MAP =
    crate::Reg<ahb_pdma_in_ch3_intr_map::AHB_PDMA_IN_CH3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_in_ch3_intr_map;
#[doc = "AHB_PDMA_IN_CH4_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_in_ch4_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_in_ch4_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch4_intr_map`] module"]
pub type AHB_PDMA_IN_CH4_INTR_MAP =
    crate::Reg<ahb_pdma_in_ch4_intr_map::AHB_PDMA_IN_CH4_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_in_ch4_intr_map;
#[doc = "AHB_PDMA_OUT_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_out_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_out_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch0_intr_map`] module"]
pub type AHB_PDMA_OUT_CH0_INTR_MAP =
    crate::Reg<ahb_pdma_out_ch0_intr_map::AHB_PDMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_out_ch0_intr_map;
#[doc = "AHB_PDMA_OUT_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_out_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_out_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch1_intr_map`] module"]
pub type AHB_PDMA_OUT_CH1_INTR_MAP =
    crate::Reg<ahb_pdma_out_ch1_intr_map::AHB_PDMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_out_ch1_intr_map;
#[doc = "AHB_PDMA_OUT_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_out_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_out_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch2_intr_map`] module"]
pub type AHB_PDMA_OUT_CH2_INTR_MAP =
    crate::Reg<ahb_pdma_out_ch2_intr_map::AHB_PDMA_OUT_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_out_ch2_intr_map;
#[doc = "AHB_PDMA_OUT_CH3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_out_ch3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_out_ch3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch3_intr_map`] module"]
pub type AHB_PDMA_OUT_CH3_INTR_MAP =
    crate::Reg<ahb_pdma_out_ch3_intr_map::AHB_PDMA_OUT_CH3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_out_ch3_intr_map;
#[doc = "AHB_PDMA_OUT_CH4_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_out_ch4_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_out_ch4_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch4_intr_map`] module"]
pub type AHB_PDMA_OUT_CH4_INTR_MAP =
    crate::Reg<ahb_pdma_out_ch4_intr_map::AHB_PDMA_OUT_CH4_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ahb_pdma_out_ch4_intr_map;
#[doc = "ASRC_CHNL0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_chnl0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_chnl0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrc_chnl0_intr_map`] module"]
pub type ASRC_CHNL0_INTR_MAP = crate::Reg<asrc_chnl0_intr_map::ASRC_CHNL0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod asrc_chnl0_intr_map;
#[doc = "ASRC_CHNL1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_chnl1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_chnl1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrc_chnl1_intr_map`] module"]
pub type ASRC_CHNL1_INTR_MAP = crate::Reg<asrc_chnl1_intr_map::ASRC_CHNL1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod asrc_chnl1_intr_map;
#[doc = "AXI_PDMA_IN_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_in_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_in_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch0_intr_map`] module"]
pub type AXI_PDMA_IN_CH0_INTR_MAP =
    crate::Reg<axi_pdma_in_ch0_intr_map::AXI_PDMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_in_ch0_intr_map;
#[doc = "AXI_PDMA_IN_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_in_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_in_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch1_intr_map`] module"]
pub type AXI_PDMA_IN_CH1_INTR_MAP =
    crate::Reg<axi_pdma_in_ch1_intr_map::AXI_PDMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_in_ch1_intr_map;
#[doc = "AXI_PDMA_IN_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_in_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_in_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch2_intr_map`] module"]
pub type AXI_PDMA_IN_CH2_INTR_MAP =
    crate::Reg<axi_pdma_in_ch2_intr_map::AXI_PDMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_in_ch2_intr_map;
#[doc = "AXI_PDMA_OUT_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_out_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_out_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch0_intr_map`] module"]
pub type AXI_PDMA_OUT_CH0_INTR_MAP =
    crate::Reg<axi_pdma_out_ch0_intr_map::AXI_PDMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_out_ch0_intr_map;
#[doc = "AXI_PDMA_OUT_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_out_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_out_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch1_intr_map`] module"]
pub type AXI_PDMA_OUT_CH1_INTR_MAP =
    crate::Reg<axi_pdma_out_ch1_intr_map::AXI_PDMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_out_ch1_intr_map;
#[doc = "AXI_PDMA_OUT_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_out_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_out_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch2_intr_map`] module"]
pub type AXI_PDMA_OUT_CH2_INTR_MAP =
    crate::Reg<axi_pdma_out_ch2_intr_map::AXI_PDMA_OUT_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod axi_pdma_out_ch2_intr_map;
#[doc = "RSA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_intr_map`] module"]
pub type RSA_INTR_MAP = crate::Reg<rsa_intr_map::RSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod rsa_intr_map;
#[doc = "AES_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aes_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_intr_map`] module"]
pub type AES_INTR_MAP = crate::Reg<aes_intr_map::AES_INTR_MAP_SPEC>;
#[doc = ""]
pub mod aes_intr_map;
#[doc = "SHA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sha_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_intr_map`] module"]
pub type SHA_INTR_MAP = crate::Reg<sha_intr_map::SHA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod sha_intr_map;
#[doc = "ECC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_intr_map`] module"]
pub type ECC_INTR_MAP = crate::Reg<ecc_intr_map::ECC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ecc_intr_map;
#[doc = "ECDSA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa_intr_map`] module"]
pub type ECDSA_INTR_MAP = crate::Reg<ecdsa_intr_map::ECDSA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ecdsa_intr_map;
#[doc = "KM_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`km_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_intr_map`] module"]
pub type KM_INTR_MAP = crate::Reg<km_intr_map::KM_INTR_MAP_SPEC>;
#[doc = ""]
pub mod km_intr_map;
#[doc = "RMA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rma_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rma_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rma_intr_map`] module"]
pub type RMA_INTR_MAP = crate::Reg<rma_intr_map::RMA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod rma_intr_map;
#[doc = "GPIO_INTR0_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr0_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr0_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intr0_map`] module"]
pub type GPIO_INTR0_MAP = crate::Reg<gpio_intr0_map::GPIO_INTR0_MAP_SPEC>;
#[doc = ""]
pub mod gpio_intr0_map;
#[doc = "GPIO_INTR1_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr1_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr1_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intr1_map`] module"]
pub type GPIO_INTR1_MAP = crate::Reg<gpio_intr1_map::GPIO_INTR1_MAP_SPEC>;
#[doc = ""]
pub mod gpio_intr1_map;
#[doc = "GPIO_INTR2_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr2_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr2_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intr2_map`] module"]
pub type GPIO_INTR2_MAP = crate::Reg<gpio_intr2_map::GPIO_INTR2_MAP_SPEC>;
#[doc = ""]
pub mod gpio_intr2_map;
#[doc = "GPIO_INTR3_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr3_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr3_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intr3_map`] module"]
pub type GPIO_INTR3_MAP = crate::Reg<gpio_intr3_map::GPIO_INTR3_MAP_SPEC>;
#[doc = ""]
pub mod gpio_intr3_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0_map`] module"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_1_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1_map`] module"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_2_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2_map`] module"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_3_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3_map`] module"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = ""]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "CACHE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cache_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_intr_map`] module"]
pub type CACHE_INTR_MAP = crate::Reg<cache_intr_map::CACHE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cache_intr_map;
#[doc = "CPU_APM_M0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m0_intr_map`] module"]
pub type CPU_APM_M0_INTR_MAP = crate::Reg<cpu_apm_m0_intr_map::CPU_APM_M0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_apm_m0_intr_map;
#[doc = "CPU_APM_M1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m1_intr_map`] module"]
pub type CPU_APM_M1_INTR_MAP = crate::Reg<cpu_apm_m1_intr_map::CPU_APM_M1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_apm_m1_intr_map;
#[doc = "CPU_APM_M2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m2_intr_map`] module"]
pub type CPU_APM_M2_INTR_MAP = crate::Reg<cpu_apm_m2_intr_map::CPU_APM_M2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_apm_m2_intr_map;
#[doc = "CPU_APM_M3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_apm_m3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_apm_m3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_apm_m3_intr_map`] module"]
pub type CPU_APM_M3_INTR_MAP = crate::Reg<cpu_apm_m3_intr_map::CPU_APM_M3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_apm_m3_intr_map;
#[doc = "HP_MEM_APM_M0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m0_intr_map`] module"]
pub type HP_MEM_APM_M0_INTR_MAP = crate::Reg<hp_mem_apm_m0_intr_map::HP_MEM_APM_M0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m0_intr_map;
#[doc = "HP_MEM_APM_M1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m1_intr_map`] module"]
pub type HP_MEM_APM_M1_INTR_MAP = crate::Reg<hp_mem_apm_m1_intr_map::HP_MEM_APM_M1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m1_intr_map;
#[doc = "HP_MEM_APM_M2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m2_intr_map`] module"]
pub type HP_MEM_APM_M2_INTR_MAP = crate::Reg<hp_mem_apm_m2_intr_map::HP_MEM_APM_M2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m2_intr_map;
#[doc = "HP_MEM_APM_M3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m3_intr_map`] module"]
pub type HP_MEM_APM_M3_INTR_MAP = crate::Reg<hp_mem_apm_m3_intr_map::HP_MEM_APM_M3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m3_intr_map;
#[doc = "HP_MEM_APM_M4_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m4_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m4_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m4_intr_map`] module"]
pub type HP_MEM_APM_M4_INTR_MAP = crate::Reg<hp_mem_apm_m4_intr_map::HP_MEM_APM_M4_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m4_intr_map;
#[doc = "HP_MEM_APM_M5_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_apm_m5_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_apm_m5_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_apm_m5_intr_map`] module"]
pub type HP_MEM_APM_M5_INTR_MAP = crate::Reg<hp_mem_apm_m5_intr_map::HP_MEM_APM_M5_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_mem_apm_m5_intr_map;
#[doc = "CPU_PERI0_TIMEOUT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri0_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri0_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri0_timeout_intr_map`] module"]
pub type CPU_PERI0_TIMEOUT_INTR_MAP =
    crate::Reg<cpu_peri0_timeout_intr_map::CPU_PERI0_TIMEOUT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_peri0_timeout_intr_map;
#[doc = "CPU_PERI1_TIMEOUT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri1_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri1_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri1_timeout_intr_map`] module"]
pub type CPU_PERI1_TIMEOUT_INTR_MAP =
    crate::Reg<cpu_peri1_timeout_intr_map::CPU_PERI1_TIMEOUT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu_peri1_timeout_intr_map;
#[doc = "HP_PERI0_TIMEOUT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri0_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_timeout_intr_map`] module"]
pub type HP_PERI0_TIMEOUT_INTR_MAP =
    crate::Reg<hp_peri0_timeout_intr_map::HP_PERI0_TIMEOUT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_peri0_timeout_intr_map;
#[doc = "HP_PERI1_TIMEOUT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri1_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_timeout_intr_map`] module"]
pub type HP_PERI1_TIMEOUT_INTR_MAP =
    crate::Reg<hp_peri1_timeout_intr_map::HP_PERI1_TIMEOUT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_peri1_timeout_intr_map;
#[doc = "HP_APM_M0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m0_intr_map`] module"]
pub type HP_APM_M0_INTR_MAP = crate::Reg<hp_apm_m0_intr_map::HP_APM_M0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m0_intr_map;
#[doc = "HP_APM_M1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m1_intr_map`] module"]
pub type HP_APM_M1_INTR_MAP = crate::Reg<hp_apm_m1_intr_map::HP_APM_M1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m1_intr_map;
#[doc = "HP_APM_M2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m2_intr_map`] module"]
pub type HP_APM_M2_INTR_MAP = crate::Reg<hp_apm_m2_intr_map::HP_APM_M2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m2_intr_map;
#[doc = "HP_APM_M3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m3_intr_map`] module"]
pub type HP_APM_M3_INTR_MAP = crate::Reg<hp_apm_m3_intr_map::HP_APM_M3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m3_intr_map;
#[doc = "HP_APM_M4_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m4_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m4_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m4_intr_map`] module"]
pub type HP_APM_M4_INTR_MAP = crate::Reg<hp_apm_m4_intr_map::HP_APM_M4_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m4_intr_map;
#[doc = "HP_APM_M5_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m5_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m5_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m5_intr_map`] module"]
pub type HP_APM_M5_INTR_MAP = crate::Reg<hp_apm_m5_intr_map::HP_APM_M5_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m5_intr_map;
#[doc = "HP_APM_M6_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_apm_m6_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_apm_m6_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_apm_m6_intr_map`] module"]
pub type HP_APM_M6_INTR_MAP = crate::Reg<hp_apm_m6_intr_map::HP_APM_M6_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_apm_m6_intr_map;
#[doc = "HP_PERI0_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri0_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_pms_intr_map`] module"]
pub type HP_PERI0_PMS_INTR_MAP = crate::Reg<hp_peri0_pms_intr_map::HP_PERI0_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_peri0_pms_intr_map;
#[doc = "HP_PERI1_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri1_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_pms_intr_map`] module"]
pub type HP_PERI1_PMS_INTR_MAP = crate::Reg<hp_peri1_pms_intr_map::HP_PERI1_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_peri1_pms_intr_map;
#[doc = "CPU0_PERI_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_peri_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_peri_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_peri_pms_intr_map`] module"]
pub type CPU0_PERI_PMS_INTR_MAP = crate::Reg<cpu0_peri_pms_intr_map::CPU0_PERI_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu0_peri_pms_intr_map;
#[doc = "CPU1_PERI_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_peri_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_peri_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_peri_pms_intr_map`] module"]
pub type CPU1_PERI_PMS_INTR_MAP = crate::Reg<cpu1_peri_pms_intr_map::CPU1_PERI_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cpu1_peri_pms_intr_map;
#[doc = "FLASH_MSPI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`flash_mspi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_mspi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_mspi_intr_map`] module"]
pub type FLASH_MSPI_INTR_MAP = crate::Reg<flash_mspi_intr_map::FLASH_MSPI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod flash_mspi_intr_map;
#[doc = "LPI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lpi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpi_intr_map`] module"]
pub type LPI_INTR_MAP = crate::Reg<lpi_intr_map::LPI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lpi_intr_map;
#[doc = "PMT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_intr_map`] module"]
pub type PMT_INTR_MAP = crate::Reg<pmt_intr_map::PMT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pmt_intr_map;
#[doc = "SBD_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sbd_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbd_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbd_intr_map`] module"]
pub type SBD_INTR_MAP = crate::Reg<sbd_intr_map::SBD_INTR_MAP_SPEC>;
#[doc = ""]
pub mod sbd_intr_map;
#[doc = "USB_OTGHS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otghs_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otghs_intr_map`] module"]
pub type USB_OTGHS_INTR_MAP = crate::Reg<usb_otghs_intr_map::USB_OTGHS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod usb_otghs_intr_map;
#[doc = "USB_OTGHS_ENDP_MULTI_PROC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_endp_multi_proc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otghs_endp_multi_proc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otghs_endp_multi_proc_intr_map`] module"]
pub type USB_OTGHS_ENDP_MULTI_PROC_INTR_MAP =
    crate::Reg<usb_otghs_endp_multi_proc_intr_map::USB_OTGHS_ENDP_MULTI_PROC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod usb_otghs_endp_multi_proc_intr_map;
#[doc = "JPEG_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_intr_map`] module"]
pub type JPEG_INTR_MAP = crate::Reg<jpeg_intr_map::JPEG_INTR_MAP_SPEC>;
#[doc = ""]
pub mod jpeg_intr_map;
#[doc = "PPA_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ppa_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppa_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppa_intr_map`] module"]
pub type PPA_INTR_MAP = crate::Reg<ppa_intr_map::PPA_INTR_MAP_SPEC>;
#[doc = ""]
pub mod ppa_intr_map;
#[doc = "CORE0_TRACE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core0_trace_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core0_trace_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_trace_intr_map`] module"]
pub type CORE0_TRACE_INTR_MAP = crate::Reg<core0_trace_intr_map::CORE0_TRACE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core0_trace_intr_map;
#[doc = "CORE1_TRACE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core1_trace_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_trace_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_trace_intr_map`] module"]
pub type CORE1_TRACE_INTR_MAP = crate::Reg<core1_trace_intr_map::CORE1_TRACE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod core1_trace_intr_map;
#[doc = "DMA2D_IN_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_in_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_in_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_in_ch0_intr_map`] module"]
pub type DMA2D_IN_CH0_INTR_MAP = crate::Reg<dma2d_in_ch0_intr_map::DMA2D_IN_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_in_ch0_intr_map;
#[doc = "DMA2D_IN_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_in_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_in_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_in_ch1_intr_map`] module"]
pub type DMA2D_IN_CH1_INTR_MAP = crate::Reg<dma2d_in_ch1_intr_map::DMA2D_IN_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_in_ch1_intr_map;
#[doc = "DMA2D_IN_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_in_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_in_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_in_ch2_intr_map`] module"]
pub type DMA2D_IN_CH2_INTR_MAP = crate::Reg<dma2d_in_ch2_intr_map::DMA2D_IN_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_in_ch2_intr_map;
#[doc = "DMA2D_OUT_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_out_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_out_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch0_intr_map`] module"]
pub type DMA2D_OUT_CH0_INTR_MAP = crate::Reg<dma2d_out_ch0_intr_map::DMA2D_OUT_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_out_ch0_intr_map;
#[doc = "DMA2D_OUT_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_out_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_out_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch1_intr_map`] module"]
pub type DMA2D_OUT_CH1_INTR_MAP = crate::Reg<dma2d_out_ch1_intr_map::DMA2D_OUT_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_out_ch1_intr_map;
#[doc = "DMA2D_OUT_CH2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_out_ch2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_out_ch2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch2_intr_map`] module"]
pub type DMA2D_OUT_CH2_INTR_MAP = crate::Reg<dma2d_out_ch2_intr_map::DMA2D_OUT_CH2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_out_ch2_intr_map;
#[doc = "DMA2D_OUT_CH3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_out_ch3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_out_ch3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch3_intr_map`] module"]
pub type DMA2D_OUT_CH3_INTR_MAP = crate::Reg<dma2d_out_ch3_intr_map::DMA2D_OUT_CH3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod dma2d_out_ch3_intr_map;
#[doc = "PSRAM_MSPI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mspi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mspi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mspi_intr_map`] module"]
pub type PSRAM_MSPI_INTR_MAP = crate::Reg<psram_mspi_intr_map::PSRAM_MSPI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod psram_mspi_intr_map;
#[doc = "HP_SYSREG_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sysreg_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sysreg_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sysreg_intr_map`] module"]
pub type HP_SYSREG_INTR_MAP = crate::Reg<hp_sysreg_intr_map::HP_SYSREG_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_sysreg_intr_map;
#[doc = "PCNT0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0_intr_map`] module"]
pub type PCNT0_INTR_MAP = crate::Reg<pcnt0_intr_map::PCNT0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pcnt0_intr_map;
#[doc = "PCNT1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt1_intr_map`] module"]
pub type PCNT1_INTR_MAP = crate::Reg<pcnt1_intr_map::PCNT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pcnt1_intr_map;
#[doc = "HP_PAU_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pau_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pau_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pau_intr_map`] module"]
pub type HP_PAU_INTR_MAP = crate::Reg<hp_pau_intr_map::HP_PAU_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_pau_intr_map;
#[doc = "HP_PARLIO_RX_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_parlio_rx_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_parlio_rx_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_parlio_rx_intr_map`] module"]
pub type HP_PARLIO_RX_INTR_MAP = crate::Reg<hp_parlio_rx_intr_map::HP_PARLIO_RX_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_parlio_rx_intr_map;
#[doc = "HP_PARLIO_TX_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_parlio_tx_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_parlio_tx_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_parlio_tx_intr_map`] module"]
pub type HP_PARLIO_TX_INTR_MAP = crate::Reg<hp_parlio_tx_intr_map::HP_PARLIO_TX_INTR_MAP_SPEC>;
#[doc = ""]
pub mod hp_parlio_tx_intr_map;
#[doc = "BUS_MONITOR_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`bus_monitor_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_monitor_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_monitor_intr_map`] module"]
pub type BUS_MONITOR_INTR_MAP = crate::Reg<bus_monitor_intr_map::BUS_MONITOR_INTR_MAP_SPEC>;
#[doc = ""]
pub mod bus_monitor_intr_map;
#[doc = "MODEM_WIFI_MAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_wifi_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_wifi_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_wifi_mac_intr_map`] module"]
pub type MODEM_WIFI_MAC_INTR_MAP =
    crate::Reg<modem_wifi_mac_intr_map::MODEM_WIFI_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_wifi_mac_intr_map;
#[doc = "MODEM_WIFI_MAC_NMI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_wifi_mac_nmi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_wifi_mac_nmi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_wifi_mac_nmi_intr_map`] module"]
pub type MODEM_WIFI_MAC_NMI_INTR_MAP =
    crate::Reg<modem_wifi_mac_nmi_intr_map::MODEM_WIFI_MAC_NMI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_wifi_mac_nmi_intr_map;
#[doc = "MODEM_WIFI_PWR_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_wifi_pwr_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_wifi_pwr_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_wifi_pwr_intr_map`] module"]
pub type MODEM_WIFI_PWR_INTR_MAP =
    crate::Reg<modem_wifi_pwr_intr_map::MODEM_WIFI_PWR_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_wifi_pwr_intr_map;
#[doc = "MODEM_WIFI_BB_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_wifi_bb_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_wifi_bb_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_wifi_bb_intr_map`] module"]
pub type MODEM_WIFI_BB_INTR_MAP = crate::Reg<modem_wifi_bb_intr_map::MODEM_WIFI_BB_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_wifi_bb_intr_map;
#[doc = "MODEM_BT_MAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bt_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bt_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_bt_mac_intr_map`] module"]
pub type MODEM_BT_MAC_INTR_MAP = crate::Reg<modem_bt_mac_intr_map::MODEM_BT_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_bt_mac_intr_map;
#[doc = "MODEM_BT_BB_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bt_bb_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bt_bb_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_bt_bb_intr_map`] module"]
pub type MODEM_BT_BB_INTR_MAP = crate::Reg<modem_bt_bb_intr_map::MODEM_BT_BB_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_bt_bb_intr_map;
#[doc = "MODEM_BT_BB_NMI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bt_bb_nmi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bt_bb_nmi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_bt_bb_nmi_intr_map`] module"]
pub type MODEM_BT_BB_NMI_INTR_MAP =
    crate::Reg<modem_bt_bb_nmi_intr_map::MODEM_BT_BB_NMI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_bt_bb_nmi_intr_map;
#[doc = "MODEM_LP_TIMER_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_lp_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_lp_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_lp_timer_intr_map`] module"]
pub type MODEM_LP_TIMER_INTR_MAP =
    crate::Reg<modem_lp_timer_intr_map::MODEM_LP_TIMER_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_lp_timer_intr_map;
#[doc = "MODEM_COEX_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_coex_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_coex_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_coex_intr_map`] module"]
pub type MODEM_COEX_INTR_MAP = crate::Reg<modem_coex_intr_map::MODEM_COEX_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_coex_intr_map;
#[doc = "MODEM_BLE_TIMER_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ble_timer_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ble_timer_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ble_timer_intr_map`] module"]
pub type MODEM_BLE_TIMER_INTR_MAP =
    crate::Reg<modem_ble_timer_intr_map::MODEM_BLE_TIMER_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_ble_timer_intr_map;
#[doc = "MODEM_BLE_SEC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ble_sec_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ble_sec_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ble_sec_intr_map`] module"]
pub type MODEM_BLE_SEC_INTR_MAP = crate::Reg<modem_ble_sec_intr_map::MODEM_BLE_SEC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_ble_sec_intr_map;
#[doc = "MODEM_I2C_MST_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_i2c_mst_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_i2c_mst_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_i2c_mst_intr_map`] module"]
pub type MODEM_I2C_MST_INTR_MAP = crate::Reg<modem_i2c_mst_intr_map::MODEM_I2C_MST_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_i2c_mst_intr_map;
#[doc = "MODEM_ZB_MAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_zb_mac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_zb_mac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_zb_mac_intr_map`] module"]
pub type MODEM_ZB_MAC_INTR_MAP = crate::Reg<modem_zb_mac_intr_map::MODEM_ZB_MAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_zb_mac_intr_map;
#[doc = "MODEM_BT_MAC_INT1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bt_mac_int1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bt_mac_int1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_bt_mac_int1_intr_map`] module"]
pub type MODEM_BT_MAC_INT1_INTR_MAP =
    crate::Reg<modem_bt_mac_int1_intr_map::MODEM_BT_MAC_INT1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod modem_bt_mac_int1_intr_map;
#[doc = "CORDIC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_intr_map`] module"]
pub type CORDIC_INTR_MAP = crate::Reg<cordic_intr_map::CORDIC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod cordic_intr_map;
#[doc = "ZERO_DET_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det_intr_map`] module"]
pub type ZERO_DET_INTR_MAP = crate::Reg<zero_det_intr_map::ZERO_DET_INTR_MAP_SPEC>;
#[doc = ""]
pub mod zero_det_intr_map;
#[doc = "LP_WDT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_wdt_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_wdt_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_intr_map`] module"]
pub type LP_WDT_INTR_MAP = crate::Reg<lp_wdt_intr_map::LP_WDT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_wdt_intr_map;
#[doc = "LP_TIMER_REG_0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_reg_0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_reg_0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_reg_0_intr_map`] module"]
pub type LP_TIMER_REG_0_INTR_MAP =
    crate::Reg<lp_timer_reg_0_intr_map::LP_TIMER_REG_0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_timer_reg_0_intr_map;
#[doc = "LP_TIMER_REG_1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_timer_reg_1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_timer_reg_1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_reg_1_intr_map`] module"]
pub type LP_TIMER_REG_1_INTR_MAP =
    crate::Reg<lp_timer_reg_1_intr_map::LP_TIMER_REG_1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_timer_reg_1_intr_map;
#[doc = "MB_HP_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mb_hp_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_hp_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_hp_intr_map`] module"]
pub type MB_HP_INTR_MAP = crate::Reg<mb_hp_intr_map::MB_HP_INTR_MAP_SPEC>;
#[doc = ""]
pub mod mb_hp_intr_map;
#[doc = "MB_LP_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mb_lp_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_lp_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_lp_intr_map`] module"]
pub type MB_LP_INTR_MAP = crate::Reg<mb_lp_intr_map::MB_LP_INTR_MAP_SPEC>;
#[doc = ""]
pub mod mb_lp_intr_map;
#[doc = "PMU_REG_0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_reg_0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_reg_0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_reg_0_intr_map`] module"]
pub type PMU_REG_0_INTR_MAP = crate::Reg<pmu_reg_0_intr_map::PMU_REG_0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pmu_reg_0_intr_map;
#[doc = "PMU_REG_1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_reg_1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_reg_1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_reg_1_intr_map`] module"]
pub type PMU_REG_1_INTR_MAP = crate::Reg<pmu_reg_1_intr_map::PMU_REG_1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod pmu_reg_1_intr_map;
#[doc = "LP_ANAPERI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_anaperi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_anaperi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_anaperi_intr_map`] module"]
pub type LP_ANAPERI_INTR_MAP = crate::Reg<lp_anaperi_intr_map::LP_ANAPERI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_anaperi_intr_map;
#[doc = "LP_ADC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_adc_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_adc_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_adc_intr_map`] module"]
pub type LP_ADC_INTR_MAP = crate::Reg<lp_adc_intr_map::LP_ADC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_adc_intr_map;
#[doc = "LP_DAC_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_dac_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_dac_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_dac_intr_map`] module"]
pub type LP_DAC_INTR_MAP = crate::Reg<lp_dac_intr_map::LP_DAC_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_dac_intr_map;
#[doc = "LP_GPIO_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_gpio_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_gpio_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_gpio_intr_map`] module"]
pub type LP_GPIO_INTR_MAP = crate::Reg<lp_gpio_intr_map::LP_GPIO_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_gpio_intr_map;
#[doc = "LP_I2C_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2c_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2c_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2c_intr_map`] module"]
pub type LP_I2C_INTR_MAP = crate::Reg<lp_i2c_intr_map::LP_I2C_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_i2c_intr_map;
#[doc = "LP_SPI_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_spi_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_spi_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_spi_intr_map`] module"]
pub type LP_SPI_INTR_MAP = crate::Reg<lp_spi_intr_map::LP_SPI_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_spi_intr_map;
#[doc = "LP_TOUCH_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_touch_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_touch_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_touch_intr_map`] module"]
pub type LP_TOUCH_INTR_MAP = crate::Reg<lp_touch_intr_map::LP_TOUCH_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_touch_intr_map;
#[doc = "LP_TSENS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tsens_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tsens_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tsens_intr_map`] module"]
pub type LP_TSENS_INTR_MAP = crate::Reg<lp_tsens_intr_map::LP_TSENS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_tsens_intr_map;
#[doc = "LP_UART_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_uart_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_uart_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_uart_intr_map`] module"]
pub type LP_UART_INTR_MAP = crate::Reg<lp_uart_intr_map::LP_UART_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_uart_intr_map;
#[doc = "LP_EFUSE_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_efuse_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_efuse_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_efuse_intr_map`] module"]
pub type LP_EFUSE_INTR_MAP = crate::Reg<lp_efuse_intr_map::LP_EFUSE_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_efuse_intr_map;
#[doc = "LP_SW_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sw_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sw_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sw_intr_map`] module"]
pub type LP_SW_INTR_MAP = crate::Reg<lp_sw_intr_map::LP_SW_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_sw_intr_map;
#[doc = "LP_TRNG_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_trng_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_trng_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_trng_intr_map`] module"]
pub type LP_TRNG_INTR_MAP = crate::Reg<lp_trng_intr_map::LP_TRNG_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_trng_intr_map;
#[doc = "LP_SYSREG_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sysreg_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sysreg_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sysreg_intr_map`] module"]
pub type LP_SYSREG_INTR_MAP = crate::Reg<lp_sysreg_intr_map::LP_SYSREG_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_sysreg_intr_map;
#[doc = "LP_APM_M0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m0_intr_map`] module"]
pub type LP_APM_M0_INTR_MAP = crate::Reg<lp_apm_m0_intr_map::LP_APM_M0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_apm_m0_intr_map;
#[doc = "LP_APM_M1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m1_intr_map`] module"]
pub type LP_APM_M1_INTR_MAP = crate::Reg<lp_apm_m1_intr_map::LP_APM_M1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_apm_m1_intr_map;
#[doc = "LP_APM_M2_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m2_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m2_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m2_intr_map`] module"]
pub type LP_APM_M2_INTR_MAP = crate::Reg<lp_apm_m2_intr_map::LP_APM_M2_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_apm_m2_intr_map;
#[doc = "LP_APM_M3_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_apm_m3_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_apm_m3_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_apm_m3_intr_map`] module"]
pub type LP_APM_M3_INTR_MAP = crate::Reg<lp_apm_m3_intr_map::LP_APM_M3_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_apm_m3_intr_map;
#[doc = "LP_PERI0_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri0_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri0_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri0_pms_intr_map`] module"]
pub type LP_PERI0_PMS_INTR_MAP = crate::Reg<lp_peri0_pms_intr_map::LP_PERI0_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_peri0_pms_intr_map;
#[doc = "LP_PERI1_PMS_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri1_pms_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri1_pms_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri1_pms_intr_map`] module"]
pub type LP_PERI1_PMS_INTR_MAP = crate::Reg<lp_peri1_pms_intr_map::LP_PERI1_PMS_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_peri1_pms_intr_map;
#[doc = "LP_HUK_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_huk_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_huk_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_huk_intr_map`] module"]
pub type LP_HUK_INTR_MAP = crate::Reg<lp_huk_intr_map::LP_HUK_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_huk_intr_map;
#[doc = "LP_PERI_TIMEOUT_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_timeout_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_intr_map`] module"]
pub type LP_PERI_TIMEOUT_INTR_MAP =
    crate::Reg<lp_peri_timeout_intr_map::LP_PERI_TIMEOUT_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_peri_timeout_intr_map;
#[doc = "LP_AHB_PDMA_IN_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ahb_pdma_in_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ahb_pdma_in_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ahb_pdma_in_ch0_intr_map`] module"]
pub type LP_AHB_PDMA_IN_CH0_INTR_MAP =
    crate::Reg<lp_ahb_pdma_in_ch0_intr_map::LP_AHB_PDMA_IN_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_ahb_pdma_in_ch0_intr_map;
#[doc = "LP_AHB_PDMA_IN_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ahb_pdma_in_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ahb_pdma_in_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ahb_pdma_in_ch1_intr_map`] module"]
pub type LP_AHB_PDMA_IN_CH1_INTR_MAP =
    crate::Reg<lp_ahb_pdma_in_ch1_intr_map::LP_AHB_PDMA_IN_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_ahb_pdma_in_ch1_intr_map;
#[doc = "LP_AHB_PDMA_OUT_CH0_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ahb_pdma_out_ch0_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ahb_pdma_out_ch0_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ahb_pdma_out_ch0_intr_map`] module"]
pub type LP_AHB_PDMA_OUT_CH0_INTR_MAP =
    crate::Reg<lp_ahb_pdma_out_ch0_intr_map::LP_AHB_PDMA_OUT_CH0_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_ahb_pdma_out_ch0_intr_map;
#[doc = "LP_AHB_PDMA_OUT_CH1_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ahb_pdma_out_ch1_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ahb_pdma_out_ch1_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ahb_pdma_out_ch1_intr_map`] module"]
pub type LP_AHB_PDMA_OUT_CH1_INTR_MAP =
    crate::Reg<lp_ahb_pdma_out_ch1_intr_map::LP_AHB_PDMA_OUT_CH1_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_ahb_pdma_out_ch1_intr_map;
#[doc = "LP_SW_INVALID_SLEEP_INTR_MAP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sw_invalid_sleep_intr_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sw_invalid_sleep_intr_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sw_invalid_sleep_intr_map`] module"]
pub type LP_SW_INVALID_SLEEP_INTR_MAP =
    crate::Reg<lp_sw_invalid_sleep_intr_map::LP_SW_INVALID_SLEEP_INTR_MAP_SPEC>;
#[doc = ""]
pub mod lp_sw_invalid_sleep_intr_map;
#[doc = "SIG_IDX_ASSERT_IN_SEC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sig_idx_assert_in_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sig_idx_assert_in_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_idx_assert_in_sec`] module"]
pub type SIG_IDX_ASSERT_IN_SEC = crate::Reg<sig_idx_assert_in_sec::SIG_IDX_ASSERT_IN_SEC_SPEC>;
#[doc = ""]
pub mod sig_idx_assert_in_sec;
#[doc = "CORE_1_INTR_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status`] module"]
pub type CORE_1_INTR_STATUS = crate::Reg<core_1_intr_status::CORE_1_INTR_STATUS_SPEC>;
#[doc = ""]
pub mod core_1_intr_status;
#[doc = "CORE_1_INTR_STATUS5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_status5`] module"]
pub type CORE_1_INTR_STATUS5 = crate::Reg<core_1_intr_status5::CORE_1_INTR_STATUS5_SPEC>;
#[doc = ""]
pub mod core_1_intr_status5;
#[doc = "SRC_PASS_IN_S_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_s_status`] module"]
pub type SRC_PASS_IN_S_STATUS = crate::Reg<src_pass_in_s_status::SRC_PASS_IN_S_STATUS_SPEC>;
#[doc = ""]
pub mod src_pass_in_s_status;
#[doc = "SRC_PASS_IN_S_STATUS_5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_s_status_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_s_status_5`] module"]
pub type SRC_PASS_IN_S_STATUS_5 = crate::Reg<src_pass_in_s_status_5::SRC_PASS_IN_S_STATUS_5_SPEC>;
#[doc = ""]
pub mod src_pass_in_s_status_5;
#[doc = "SRC_PASS_IN_M_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_m_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_m_status`] module"]
pub type SRC_PASS_IN_M_STATUS = crate::Reg<src_pass_in_m_status::SRC_PASS_IN_M_STATUS_SPEC>;
#[doc = ""]
pub mod src_pass_in_m_status;
#[doc = "SRC_PASS_IN_M_STATUS_5 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_m_status_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_pass_in_m_status_5`] module"]
pub type SRC_PASS_IN_M_STATUS_5 = crate::Reg<src_pass_in_m_status_5::SRC_PASS_IN_M_STATUS_5_SPEC>;
#[doc = ""]
pub mod src_pass_in_m_status_5;
#[doc = "CONFIG_STATE (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`config_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_state`] module"]
pub type CONFIG_STATE = crate::Reg<config_state::CONFIG_STATE_SPEC>;
#[doc = ""]
pub mod config_state;
#[doc = "S_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_status`] module"]
pub type S_STATUS = crate::Reg<s_status::S_STATUS_SPEC>;
#[doc = ""]
pub mod s_status;
#[doc = "M_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`m_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_status`] module"]
pub type M_STATUS = crate::Reg<m_status::M_STATUS_SPEC>;
#[doc = ""]
pub mod m_status;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
#[doc = "INTERRUPT_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_date`] module"]
pub type INTERRUPT_DATE = crate::Reg<interrupt_date::INTERRUPT_DATE_SPEC>;
#[doc = ""]
pub mod interrupt_date;
