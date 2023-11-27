#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    lp_rtc_int_map: LP_RTC_INT_MAP,
    lp_wdt_int_map: LP_WDT_INT_MAP,
    lp_timer_reg_0_int_map: LP_TIMER_REG_0_INT_MAP,
    lp_timer_reg_1_int_map: LP_TIMER_REG_1_INT_MAP,
    mb_hp_int_map: MB_HP_INT_MAP,
    mb_lp_int_map: MB_LP_INT_MAP,
    pmu_reg_0_int_map: PMU_REG_0_INT_MAP,
    pmu_reg_1_int_map: PMU_REG_1_INT_MAP,
    lp_anaperi_int_map: LP_ANAPERI_INT_MAP,
    lp_adc_int_map: LP_ADC_INT_MAP,
    lp_gpio_int_map: LP_GPIO_INT_MAP,
    lp_i2c_int_map: LP_I2C_INT_MAP,
    lp_i2s_int_map: LP_I2S_INT_MAP,
    lp_spi_int_map: LP_SPI_INT_MAP,
    lp_touch_int_map: LP_TOUCH_INT_MAP,
    lp_tsens_int_map: LP_TSENS_INT_MAP,
    lp_uart_int_map: LP_UART_INT_MAP,
    lp_efuse_int_map: LP_EFUSE_INT_MAP,
    lp_sw_int_map: LP_SW_INT_MAP,
    lp_sysreg_int_map: LP_SYSREG_INT_MAP,
    lp_huk_int_map: LP_HUK_INT_MAP,
    sys_icm_int_map: SYS_ICM_INT_MAP,
    usb_device_int_map: USB_DEVICE_INT_MAP,
    sdio_host_int_map: SDIO_HOST_INT_MAP,
    gdma_int_map: GDMA_INT_MAP,
    spi2_int_map: SPI2_INT_MAP,
    spi3_int_map: SPI3_INT_MAP,
    i2s0_int_map: I2S0_INT_MAP,
    i2s1_int_map: I2S1_INT_MAP,
    i2s2_int_map: I2S2_INT_MAP,
    uhci0_int_map: UHCI0_INT_MAP,
    uart0_int_map: UART0_INT_MAP,
    uart1_int_map: UART1_INT_MAP,
    uart2_int_map: UART2_INT_MAP,
    uart3_int_map: UART3_INT_MAP,
    uart4_int_map: UART4_INT_MAP,
    lcd_cam_int_map: LCD_CAM_INT_MAP,
    adc_int_map: ADC_INT_MAP,
    pwm0_int_map: PWM0_INT_MAP,
    pwm1_int_map: PWM1_INT_MAP,
    can0_int_map: CAN0_INT_MAP,
    can1_int_map: CAN1_INT_MAP,
    can2_int_map: CAN2_INT_MAP,
    rmt_int_map: RMT_INT_MAP,
    i2c0_int_map: I2C0_INT_MAP,
    i2c1_int_map: I2C1_INT_MAP,
    timergrp0_t0_int_map: TIMERGRP0_T0_INT_MAP,
    timergrp0_t1_int_map: TIMERGRP0_T1_INT_MAP,
    timergrp0_wdt_int_map: TIMERGRP0_WDT_INT_MAP,
    timergrp1_t0_int_map: TIMERGRP1_T0_INT_MAP,
    timergrp1_t1_int_map: TIMERGRP1_T1_INT_MAP,
    timergrp1_wdt_int_map: TIMERGRP1_WDT_INT_MAP,
    ledc_int_map: LEDC_INT_MAP,
    systimer_target0_int_map: SYSTIMER_TARGET0_INT_MAP,
    systimer_target1_int_map: SYSTIMER_TARGET1_INT_MAP,
    systimer_target2_int_map: SYSTIMER_TARGET2_INT_MAP,
    ahb_pdma_in_ch0_int_map: AHB_PDMA_IN_CH0_INT_MAP,
    ahb_pdma_in_ch1_int_map: AHB_PDMA_IN_CH1_INT_MAP,
    ahb_pdma_in_ch2_int_map: AHB_PDMA_IN_CH2_INT_MAP,
    ahb_pdma_out_ch0_int_map: AHB_PDMA_OUT_CH0_INT_MAP,
    ahb_pdma_out_ch1_int_map: AHB_PDMA_OUT_CH1_INT_MAP,
    ahb_pdma_out_ch2_int_map: AHB_PDMA_OUT_CH2_INT_MAP,
    axi_pdma_in_ch0_int_map: AXI_PDMA_IN_CH0_INT_MAP,
    axi_pdma_in_ch1_int_map: AXI_PDMA_IN_CH1_INT_MAP,
    axi_pdma_in_ch2_int_map: AXI_PDMA_IN_CH2_INT_MAP,
    axi_pdma_out_ch0_int_map: AXI_PDMA_OUT_CH0_INT_MAP,
    axi_pdma_out_ch1_int_map: AXI_PDMA_OUT_CH1_INT_MAP,
    axi_pdma_out_ch2_int_map: AXI_PDMA_OUT_CH2_INT_MAP,
    rsa_int_map: RSA_INT_MAP,
    aes_int_map: AES_INT_MAP,
    sha_int_map: SHA_INT_MAP,
    ecc_int_map: ECC_INT_MAP,
    ecdsa_int_map: ECDSA_INT_MAP,
    km_int_map: KM_INT_MAP,
    gpio_int0_map: GPIO_INT0_MAP,
    gpio_int1_map: GPIO_INT1_MAP,
    gpio_int2_map: GPIO_INT2_MAP,
    gpio_int3_map: GPIO_INT3_MAP,
    gpio_pad_comp_int_map: GPIO_PAD_COMP_INT_MAP,
    cpu_int_from_cpu_0_map: CPU_INT_FROM_CPU_0_MAP,
    cpu_int_from_cpu_1_map: CPU_INT_FROM_CPU_1_MAP,
    cpu_int_from_cpu_2_map: CPU_INT_FROM_CPU_2_MAP,
    cpu_int_from_cpu_3_map: CPU_INT_FROM_CPU_3_MAP,
    cache_int_map: CACHE_INT_MAP,
    flash_mspi_int_map: FLASH_MSPI_INT_MAP,
    csi_bridge_int_map: CSI_BRIDGE_INT_MAP,
    dsi_bridge_int_map: DSI_BRIDGE_INT_MAP,
    csi_int_map: CSI_INT_MAP,
    dsi_int_map: DSI_INT_MAP,
    gmii_phy_int_map: GMII_PHY_INT_MAP,
    lpi_int_map: LPI_INT_MAP,
    pmt_int_map: PMT_INT_MAP,
    sbd_int_map: SBD_INT_MAP,
    usb_otg_int_map: USB_OTG_INT_MAP,
    usb_otg_endp_multi_proc_int_map: USB_OTG_ENDP_MULTI_PROC_INT_MAP,
    jpeg_int_map: JPEG_INT_MAP,
    ppa_int_map: PPA_INT_MAP,
    core0_trace_int_map: CORE0_TRACE_INT_MAP,
    core1_trace_int_map: CORE1_TRACE_INT_MAP,
    hp_core_ctrl_int_map: HP_CORE_CTRL_INT_MAP,
    isp_int_map: ISP_INT_MAP,
    i3c_mst_int_map: I3C_MST_INT_MAP,
    i3c_slv_int_map: I3C_SLV_INT_MAP,
    usb_otg11_int_map: USB_OTG11_INT_MAP,
    dma2d_in_ch0_int_map: DMA2D_IN_CH0_INT_MAP,
    dma2d_in_ch1_int_map: DMA2D_IN_CH1_INT_MAP,
    dma2d_out_ch0_int_map: DMA2D_OUT_CH0_INT_MAP,
    dma2d_out_ch1_int_map: DMA2D_OUT_CH1_INT_MAP,
    dma2d_out_ch2_int_map: DMA2D_OUT_CH2_INT_MAP,
    psram_mspi_int_map: PSRAM_MSPI_INT_MAP,
    hp_sysreg_int_map: HP_SYSREG_INT_MAP,
    pcnt_int_map: PCNT_INT_MAP,
    hp_pau_int_map: HP_PAU_INT_MAP,
    hp_parlio_rx_int_map: HP_PARLIO_RX_INT_MAP,
    hp_parlio_tx_int_map: HP_PARLIO_TX_INT_MAP,
    h264_dma2d_out_ch0_int_map: H264_DMA2D_OUT_CH0_INT_MAP,
    h264_dma2d_out_ch1_int_map: H264_DMA2D_OUT_CH1_INT_MAP,
    h264_dma2d_out_ch2_int_map: H264_DMA2D_OUT_CH2_INT_MAP,
    h264_dma2d_out_ch3_int_map: H264_DMA2D_OUT_CH3_INT_MAP,
    h264_dma2d_out_ch4_int_map: H264_DMA2D_OUT_CH4_INT_MAP,
    h264_dma2d_in_ch0_int_map: H264_DMA2D_IN_CH0_INT_MAP,
    h264_dma2d_in_ch1_int_map: H264_DMA2D_IN_CH1_INT_MAP,
    h264_dma2d_in_ch2_int_map: H264_DMA2D_IN_CH2_INT_MAP,
    h264_dma2d_in_ch3_int_map: H264_DMA2D_IN_CH3_INT_MAP,
    h264_dma2d_in_ch4_int_map: H264_DMA2D_IN_CH4_INT_MAP,
    h264_dma2d_in_ch5_int_map: H264_DMA2D_IN_CH5_INT_MAP,
    h264_reg_int_map: H264_REG_INT_MAP,
    assist_debug_int_map: ASSIST_DEBUG_INT_MAP,
    intr_status_reg_0: INTR_STATUS_REG_0,
    intr_status_reg_1: INTR_STATUS_REG_1,
    intr_status_reg_2: INTR_STATUS_REG_2,
    intr_status_reg_3: INTR_STATUS_REG_3,
    clock_gate: CLOCK_GATE,
    _reserved133: [u8; 0x01e8],
    interrupt_reg_date: INTERRUPT_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn lp_rtc_int_map(&self) -> &LP_RTC_INT_MAP {
        &self.lp_rtc_int_map
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn lp_wdt_int_map(&self) -> &LP_WDT_INT_MAP {
        &self.lp_wdt_int_map
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn lp_timer_reg_0_int_map(&self) -> &LP_TIMER_REG_0_INT_MAP {
        &self.lp_timer_reg_0_int_map
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn lp_timer_reg_1_int_map(&self) -> &LP_TIMER_REG_1_INT_MAP {
        &self.lp_timer_reg_1_int_map
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn mb_hp_int_map(&self) -> &MB_HP_INT_MAP {
        &self.mb_hp_int_map
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn mb_lp_int_map(&self) -> &MB_LP_INT_MAP {
        &self.mb_lp_int_map
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn pmu_reg_0_int_map(&self) -> &PMU_REG_0_INT_MAP {
        &self.pmu_reg_0_int_map
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn pmu_reg_1_int_map(&self) -> &PMU_REG_1_INT_MAP {
        &self.pmu_reg_1_int_map
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn lp_anaperi_int_map(&self) -> &LP_ANAPERI_INT_MAP {
        &self.lp_anaperi_int_map
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn lp_adc_int_map(&self) -> &LP_ADC_INT_MAP {
        &self.lp_adc_int_map
    }
    #[doc = "0x28 - NA"]
    #[inline(always)]
    pub const fn lp_gpio_int_map(&self) -> &LP_GPIO_INT_MAP {
        &self.lp_gpio_int_map
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn lp_i2c_int_map(&self) -> &LP_I2C_INT_MAP {
        &self.lp_i2c_int_map
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn lp_i2s_int_map(&self) -> &LP_I2S_INT_MAP {
        &self.lp_i2s_int_map
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn lp_spi_int_map(&self) -> &LP_SPI_INT_MAP {
        &self.lp_spi_int_map
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn lp_touch_int_map(&self) -> &LP_TOUCH_INT_MAP {
        &self.lp_touch_int_map
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn lp_tsens_int_map(&self) -> &LP_TSENS_INT_MAP {
        &self.lp_tsens_int_map
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn lp_uart_int_map(&self) -> &LP_UART_INT_MAP {
        &self.lp_uart_int_map
    }
    #[doc = "0x44 - NA"]
    #[inline(always)]
    pub const fn lp_efuse_int_map(&self) -> &LP_EFUSE_INT_MAP {
        &self.lp_efuse_int_map
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn lp_sw_int_map(&self) -> &LP_SW_INT_MAP {
        &self.lp_sw_int_map
    }
    #[doc = "0x4c - NA"]
    #[inline(always)]
    pub const fn lp_sysreg_int_map(&self) -> &LP_SYSREG_INT_MAP {
        &self.lp_sysreg_int_map
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn lp_huk_int_map(&self) -> &LP_HUK_INT_MAP {
        &self.lp_huk_int_map
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn sys_icm_int_map(&self) -> &SYS_ICM_INT_MAP {
        &self.sys_icm_int_map
    }
    #[doc = "0x58 - NA"]
    #[inline(always)]
    pub const fn usb_device_int_map(&self) -> &USB_DEVICE_INT_MAP {
        &self.usb_device_int_map
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn sdio_host_int_map(&self) -> &SDIO_HOST_INT_MAP {
        &self.sdio_host_int_map
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn gdma_int_map(&self) -> &GDMA_INT_MAP {
        &self.gdma_int_map
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn spi2_int_map(&self) -> &SPI2_INT_MAP {
        &self.spi2_int_map
    }
    #[doc = "0x68 - NA"]
    #[inline(always)]
    pub const fn spi3_int_map(&self) -> &SPI3_INT_MAP {
        &self.spi3_int_map
    }
    #[doc = "0x6c - NA"]
    #[inline(always)]
    pub const fn i2s0_int_map(&self) -> &I2S0_INT_MAP {
        &self.i2s0_int_map
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn i2s1_int_map(&self) -> &I2S1_INT_MAP {
        &self.i2s1_int_map
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn i2s2_int_map(&self) -> &I2S2_INT_MAP {
        &self.i2s2_int_map
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn uhci0_int_map(&self) -> &UHCI0_INT_MAP {
        &self.uhci0_int_map
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn uart0_int_map(&self) -> &UART0_INT_MAP {
        &self.uart0_int_map
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn uart1_int_map(&self) -> &UART1_INT_MAP {
        &self.uart1_int_map
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn uart2_int_map(&self) -> &UART2_INT_MAP {
        &self.uart2_int_map
    }
    #[doc = "0x88 - NA"]
    #[inline(always)]
    pub const fn uart3_int_map(&self) -> &UART3_INT_MAP {
        &self.uart3_int_map
    }
    #[doc = "0x8c - NA"]
    #[inline(always)]
    pub const fn uart4_int_map(&self) -> &UART4_INT_MAP {
        &self.uart4_int_map
    }
    #[doc = "0x90 - NA"]
    #[inline(always)]
    pub const fn lcd_cam_int_map(&self) -> &LCD_CAM_INT_MAP {
        &self.lcd_cam_int_map
    }
    #[doc = "0x94 - NA"]
    #[inline(always)]
    pub const fn adc_int_map(&self) -> &ADC_INT_MAP {
        &self.adc_int_map
    }
    #[doc = "0x98 - NA"]
    #[inline(always)]
    pub const fn pwm0_int_map(&self) -> &PWM0_INT_MAP {
        &self.pwm0_int_map
    }
    #[doc = "0x9c - NA"]
    #[inline(always)]
    pub const fn pwm1_int_map(&self) -> &PWM1_INT_MAP {
        &self.pwm1_int_map
    }
    #[doc = "0xa0 - NA"]
    #[inline(always)]
    pub const fn can0_int_map(&self) -> &CAN0_INT_MAP {
        &self.can0_int_map
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn can1_int_map(&self) -> &CAN1_INT_MAP {
        &self.can1_int_map
    }
    #[doc = "0xa8 - NA"]
    #[inline(always)]
    pub const fn can2_int_map(&self) -> &CAN2_INT_MAP {
        &self.can2_int_map
    }
    #[doc = "0xac - NA"]
    #[inline(always)]
    pub const fn rmt_int_map(&self) -> &RMT_INT_MAP {
        &self.rmt_int_map
    }
    #[doc = "0xb0 - NA"]
    #[inline(always)]
    pub const fn i2c0_int_map(&self) -> &I2C0_INT_MAP {
        &self.i2c0_int_map
    }
    #[doc = "0xb4 - NA"]
    #[inline(always)]
    pub const fn i2c1_int_map(&self) -> &I2C1_INT_MAP {
        &self.i2c1_int_map
    }
    #[doc = "0xb8 - NA"]
    #[inline(always)]
    pub const fn timergrp0_t0_int_map(&self) -> &TIMERGRP0_T0_INT_MAP {
        &self.timergrp0_t0_int_map
    }
    #[doc = "0xbc - NA"]
    #[inline(always)]
    pub const fn timergrp0_t1_int_map(&self) -> &TIMERGRP0_T1_INT_MAP {
        &self.timergrp0_t1_int_map
    }
    #[doc = "0xc0 - NA"]
    #[inline(always)]
    pub const fn timergrp0_wdt_int_map(&self) -> &TIMERGRP0_WDT_INT_MAP {
        &self.timergrp0_wdt_int_map
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn timergrp1_t0_int_map(&self) -> &TIMERGRP1_T0_INT_MAP {
        &self.timergrp1_t0_int_map
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn timergrp1_t1_int_map(&self) -> &TIMERGRP1_T1_INT_MAP {
        &self.timergrp1_t1_int_map
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn timergrp1_wdt_int_map(&self) -> &TIMERGRP1_WDT_INT_MAP {
        &self.timergrp1_wdt_int_map
    }
    #[doc = "0xd0 - NA"]
    #[inline(always)]
    pub const fn ledc_int_map(&self) -> &LEDC_INT_MAP {
        &self.ledc_int_map
    }
    #[doc = "0xd4 - NA"]
    #[inline(always)]
    pub const fn systimer_target0_int_map(&self) -> &SYSTIMER_TARGET0_INT_MAP {
        &self.systimer_target0_int_map
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn systimer_target1_int_map(&self) -> &SYSTIMER_TARGET1_INT_MAP {
        &self.systimer_target1_int_map
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn systimer_target2_int_map(&self) -> &SYSTIMER_TARGET2_INT_MAP {
        &self.systimer_target2_int_map
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch0_int_map(&self) -> &AHB_PDMA_IN_CH0_INT_MAP {
        &self.ahb_pdma_in_ch0_int_map
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch1_int_map(&self) -> &AHB_PDMA_IN_CH1_INT_MAP {
        &self.ahb_pdma_in_ch1_int_map
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_in_ch2_int_map(&self) -> &AHB_PDMA_IN_CH2_INT_MAP {
        &self.ahb_pdma_in_ch2_int_map
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch0_int_map(&self) -> &AHB_PDMA_OUT_CH0_INT_MAP {
        &self.ahb_pdma_out_ch0_int_map
    }
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch1_int_map(&self) -> &AHB_PDMA_OUT_CH1_INT_MAP {
        &self.ahb_pdma_out_ch1_int_map
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn ahb_pdma_out_ch2_int_map(&self) -> &AHB_PDMA_OUT_CH2_INT_MAP {
        &self.ahb_pdma_out_ch2_int_map
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn axi_pdma_in_ch0_int_map(&self) -> &AXI_PDMA_IN_CH0_INT_MAP {
        &self.axi_pdma_in_ch0_int_map
    }
    #[doc = "0xfc - NA"]
    #[inline(always)]
    pub const fn axi_pdma_in_ch1_int_map(&self) -> &AXI_PDMA_IN_CH1_INT_MAP {
        &self.axi_pdma_in_ch1_int_map
    }
    #[doc = "0x100 - NA"]
    #[inline(always)]
    pub const fn axi_pdma_in_ch2_int_map(&self) -> &AXI_PDMA_IN_CH2_INT_MAP {
        &self.axi_pdma_in_ch2_int_map
    }
    #[doc = "0x104 - NA"]
    #[inline(always)]
    pub const fn axi_pdma_out_ch0_int_map(&self) -> &AXI_PDMA_OUT_CH0_INT_MAP {
        &self.axi_pdma_out_ch0_int_map
    }
    #[doc = "0x108 - NA"]
    #[inline(always)]
    pub const fn axi_pdma_out_ch1_int_map(&self) -> &AXI_PDMA_OUT_CH1_INT_MAP {
        &self.axi_pdma_out_ch1_int_map
    }
    #[doc = "0x10c - NA"]
    #[inline(always)]
    pub const fn axi_pdma_out_ch2_int_map(&self) -> &AXI_PDMA_OUT_CH2_INT_MAP {
        &self.axi_pdma_out_ch2_int_map
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn rsa_int_map(&self) -> &RSA_INT_MAP {
        &self.rsa_int_map
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn aes_int_map(&self) -> &AES_INT_MAP {
        &self.aes_int_map
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn sha_int_map(&self) -> &SHA_INT_MAP {
        &self.sha_int_map
    }
    #[doc = "0x11c - NA"]
    #[inline(always)]
    pub const fn ecc_int_map(&self) -> &ECC_INT_MAP {
        &self.ecc_int_map
    }
    #[doc = "0x120 - NA"]
    #[inline(always)]
    pub const fn ecdsa_int_map(&self) -> &ECDSA_INT_MAP {
        &self.ecdsa_int_map
    }
    #[doc = "0x124 - NA"]
    #[inline(always)]
    pub const fn km_int_map(&self) -> &KM_INT_MAP {
        &self.km_int_map
    }
    #[doc = "0x128 - NA"]
    #[inline(always)]
    pub const fn gpio_int0_map(&self) -> &GPIO_INT0_MAP {
        &self.gpio_int0_map
    }
    #[doc = "0x12c - NA"]
    #[inline(always)]
    pub const fn gpio_int1_map(&self) -> &GPIO_INT1_MAP {
        &self.gpio_int1_map
    }
    #[doc = "0x130 - NA"]
    #[inline(always)]
    pub const fn gpio_int2_map(&self) -> &GPIO_INT2_MAP {
        &self.gpio_int2_map
    }
    #[doc = "0x134 - NA"]
    #[inline(always)]
    pub const fn gpio_int3_map(&self) -> &GPIO_INT3_MAP {
        &self.gpio_int3_map
    }
    #[doc = "0x138 - NA"]
    #[inline(always)]
    pub const fn gpio_pad_comp_int_map(&self) -> &GPIO_PAD_COMP_INT_MAP {
        &self.gpio_pad_comp_int_map
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn cpu_int_from_cpu_0_map(&self) -> &CPU_INT_FROM_CPU_0_MAP {
        &self.cpu_int_from_cpu_0_map
    }
    #[doc = "0x140 - NA"]
    #[inline(always)]
    pub const fn cpu_int_from_cpu_1_map(&self) -> &CPU_INT_FROM_CPU_1_MAP {
        &self.cpu_int_from_cpu_1_map
    }
    #[doc = "0x144 - NA"]
    #[inline(always)]
    pub const fn cpu_int_from_cpu_2_map(&self) -> &CPU_INT_FROM_CPU_2_MAP {
        &self.cpu_int_from_cpu_2_map
    }
    #[doc = "0x148 - NA"]
    #[inline(always)]
    pub const fn cpu_int_from_cpu_3_map(&self) -> &CPU_INT_FROM_CPU_3_MAP {
        &self.cpu_int_from_cpu_3_map
    }
    #[doc = "0x14c - NA"]
    #[inline(always)]
    pub const fn cache_int_map(&self) -> &CACHE_INT_MAP {
        &self.cache_int_map
    }
    #[doc = "0x150 - NA"]
    #[inline(always)]
    pub const fn flash_mspi_int_map(&self) -> &FLASH_MSPI_INT_MAP {
        &self.flash_mspi_int_map
    }
    #[doc = "0x154 - NA"]
    #[inline(always)]
    pub const fn csi_bridge_int_map(&self) -> &CSI_BRIDGE_INT_MAP {
        &self.csi_bridge_int_map
    }
    #[doc = "0x158 - NA"]
    #[inline(always)]
    pub const fn dsi_bridge_int_map(&self) -> &DSI_BRIDGE_INT_MAP {
        &self.dsi_bridge_int_map
    }
    #[doc = "0x15c - NA"]
    #[inline(always)]
    pub const fn csi_int_map(&self) -> &CSI_INT_MAP {
        &self.csi_int_map
    }
    #[doc = "0x160 - NA"]
    #[inline(always)]
    pub const fn dsi_int_map(&self) -> &DSI_INT_MAP {
        &self.dsi_int_map
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn gmii_phy_int_map(&self) -> &GMII_PHY_INT_MAP {
        &self.gmii_phy_int_map
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn lpi_int_map(&self) -> &LPI_INT_MAP {
        &self.lpi_int_map
    }
    #[doc = "0x16c - NA"]
    #[inline(always)]
    pub const fn pmt_int_map(&self) -> &PMT_INT_MAP {
        &self.pmt_int_map
    }
    #[doc = "0x170 - NA"]
    #[inline(always)]
    pub const fn sbd_int_map(&self) -> &SBD_INT_MAP {
        &self.sbd_int_map
    }
    #[doc = "0x174 - NA"]
    #[inline(always)]
    pub const fn usb_otg_int_map(&self) -> &USB_OTG_INT_MAP {
        &self.usb_otg_int_map
    }
    #[doc = "0x178 - NA"]
    #[inline(always)]
    pub const fn usb_otg_endp_multi_proc_int_map(&self) -> &USB_OTG_ENDP_MULTI_PROC_INT_MAP {
        &self.usb_otg_endp_multi_proc_int_map
    }
    #[doc = "0x17c - NA"]
    #[inline(always)]
    pub const fn jpeg_int_map(&self) -> &JPEG_INT_MAP {
        &self.jpeg_int_map
    }
    #[doc = "0x180 - NA"]
    #[inline(always)]
    pub const fn ppa_int_map(&self) -> &PPA_INT_MAP {
        &self.ppa_int_map
    }
    #[doc = "0x184 - NA"]
    #[inline(always)]
    pub const fn core0_trace_int_map(&self) -> &CORE0_TRACE_INT_MAP {
        &self.core0_trace_int_map
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn core1_trace_int_map(&self) -> &CORE1_TRACE_INT_MAP {
        &self.core1_trace_int_map
    }
    #[doc = "0x18c - NA"]
    #[inline(always)]
    pub const fn hp_core_ctrl_int_map(&self) -> &HP_CORE_CTRL_INT_MAP {
        &self.hp_core_ctrl_int_map
    }
    #[doc = "0x190 - NA"]
    #[inline(always)]
    pub const fn isp_int_map(&self) -> &ISP_INT_MAP {
        &self.isp_int_map
    }
    #[doc = "0x194 - NA"]
    #[inline(always)]
    pub const fn i3c_mst_int_map(&self) -> &I3C_MST_INT_MAP {
        &self.i3c_mst_int_map
    }
    #[doc = "0x198 - NA"]
    #[inline(always)]
    pub const fn i3c_slv_int_map(&self) -> &I3C_SLV_INT_MAP {
        &self.i3c_slv_int_map
    }
    #[doc = "0x19c - NA"]
    #[inline(always)]
    pub const fn usb_otg11_int_map(&self) -> &USB_OTG11_INT_MAP {
        &self.usb_otg11_int_map
    }
    #[doc = "0x1a0 - NA"]
    #[inline(always)]
    pub const fn dma2d_in_ch0_int_map(&self) -> &DMA2D_IN_CH0_INT_MAP {
        &self.dma2d_in_ch0_int_map
    }
    #[doc = "0x1a4 - NA"]
    #[inline(always)]
    pub const fn dma2d_in_ch1_int_map(&self) -> &DMA2D_IN_CH1_INT_MAP {
        &self.dma2d_in_ch1_int_map
    }
    #[doc = "0x1a8 - NA"]
    #[inline(always)]
    pub const fn dma2d_out_ch0_int_map(&self) -> &DMA2D_OUT_CH0_INT_MAP {
        &self.dma2d_out_ch0_int_map
    }
    #[doc = "0x1ac - NA"]
    #[inline(always)]
    pub const fn dma2d_out_ch1_int_map(&self) -> &DMA2D_OUT_CH1_INT_MAP {
        &self.dma2d_out_ch1_int_map
    }
    #[doc = "0x1b0 - NA"]
    #[inline(always)]
    pub const fn dma2d_out_ch2_int_map(&self) -> &DMA2D_OUT_CH2_INT_MAP {
        &self.dma2d_out_ch2_int_map
    }
    #[doc = "0x1b4 - NA"]
    #[inline(always)]
    pub const fn psram_mspi_int_map(&self) -> &PSRAM_MSPI_INT_MAP {
        &self.psram_mspi_int_map
    }
    #[doc = "0x1b8 - NA"]
    #[inline(always)]
    pub const fn hp_sysreg_int_map(&self) -> &HP_SYSREG_INT_MAP {
        &self.hp_sysreg_int_map
    }
    #[doc = "0x1bc - NA"]
    #[inline(always)]
    pub const fn pcnt_int_map(&self) -> &PCNT_INT_MAP {
        &self.pcnt_int_map
    }
    #[doc = "0x1c0 - NA"]
    #[inline(always)]
    pub const fn hp_pau_int_map(&self) -> &HP_PAU_INT_MAP {
        &self.hp_pau_int_map
    }
    #[doc = "0x1c4 - NA"]
    #[inline(always)]
    pub const fn hp_parlio_rx_int_map(&self) -> &HP_PARLIO_RX_INT_MAP {
        &self.hp_parlio_rx_int_map
    }
    #[doc = "0x1c8 - NA"]
    #[inline(always)]
    pub const fn hp_parlio_tx_int_map(&self) -> &HP_PARLIO_TX_INT_MAP {
        &self.hp_parlio_tx_int_map
    }
    #[doc = "0x1cc - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_out_ch0_int_map(&self) -> &H264_DMA2D_OUT_CH0_INT_MAP {
        &self.h264_dma2d_out_ch0_int_map
    }
    #[doc = "0x1d0 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_out_ch1_int_map(&self) -> &H264_DMA2D_OUT_CH1_INT_MAP {
        &self.h264_dma2d_out_ch1_int_map
    }
    #[doc = "0x1d4 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_out_ch2_int_map(&self) -> &H264_DMA2D_OUT_CH2_INT_MAP {
        &self.h264_dma2d_out_ch2_int_map
    }
    #[doc = "0x1d8 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_out_ch3_int_map(&self) -> &H264_DMA2D_OUT_CH3_INT_MAP {
        &self.h264_dma2d_out_ch3_int_map
    }
    #[doc = "0x1dc - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_out_ch4_int_map(&self) -> &H264_DMA2D_OUT_CH4_INT_MAP {
        &self.h264_dma2d_out_ch4_int_map
    }
    #[doc = "0x1e0 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch0_int_map(&self) -> &H264_DMA2D_IN_CH0_INT_MAP {
        &self.h264_dma2d_in_ch0_int_map
    }
    #[doc = "0x1e4 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch1_int_map(&self) -> &H264_DMA2D_IN_CH1_INT_MAP {
        &self.h264_dma2d_in_ch1_int_map
    }
    #[doc = "0x1e8 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch2_int_map(&self) -> &H264_DMA2D_IN_CH2_INT_MAP {
        &self.h264_dma2d_in_ch2_int_map
    }
    #[doc = "0x1ec - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch3_int_map(&self) -> &H264_DMA2D_IN_CH3_INT_MAP {
        &self.h264_dma2d_in_ch3_int_map
    }
    #[doc = "0x1f0 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch4_int_map(&self) -> &H264_DMA2D_IN_CH4_INT_MAP {
        &self.h264_dma2d_in_ch4_int_map
    }
    #[doc = "0x1f4 - NA"]
    #[inline(always)]
    pub const fn h264_dma2d_in_ch5_int_map(&self) -> &H264_DMA2D_IN_CH5_INT_MAP {
        &self.h264_dma2d_in_ch5_int_map
    }
    #[doc = "0x1f8 - NA"]
    #[inline(always)]
    pub const fn h264_reg_int_map(&self) -> &H264_REG_INT_MAP {
        &self.h264_reg_int_map
    }
    #[doc = "0x1fc - NA"]
    #[inline(always)]
    pub const fn assist_debug_int_map(&self) -> &ASSIST_DEBUG_INT_MAP {
        &self.assist_debug_int_map
    }
    #[doc = "0x200 - NA"]
    #[inline(always)]
    pub const fn intr_status_reg_0(&self) -> &INTR_STATUS_REG_0 {
        &self.intr_status_reg_0
    }
    #[doc = "0x204 - NA"]
    #[inline(always)]
    pub const fn intr_status_reg_1(&self) -> &INTR_STATUS_REG_1 {
        &self.intr_status_reg_1
    }
    #[doc = "0x208 - NA"]
    #[inline(always)]
    pub const fn intr_status_reg_2(&self) -> &INTR_STATUS_REG_2 {
        &self.intr_status_reg_2
    }
    #[doc = "0x20c - NA"]
    #[inline(always)]
    pub const fn intr_status_reg_3(&self) -> &INTR_STATUS_REG_3 {
        &self.intr_status_reg_3
    }
    #[doc = "0x210 - NA"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - NA"]
    #[inline(always)]
    pub const fn interrupt_reg_date(&self) -> &INTERRUPT_REG_DATE {
        &self.interrupt_reg_date
    }
}
#[doc = "LP_RTC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rtc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rtc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rtc_int_map`] module"]
pub type LP_RTC_INT_MAP = crate::Reg<lp_rtc_int_map::LP_RTC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_rtc_int_map;
#[doc = "LP_WDT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_wdt_int_map`] module"]
pub type LP_WDT_INT_MAP = crate::Reg<lp_wdt_int_map::LP_WDT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_wdt_int_map;
#[doc = "LP_TIMER_REG_0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timer_reg_0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timer_reg_0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_reg_0_int_map`] module"]
pub type LP_TIMER_REG_0_INT_MAP = crate::Reg<lp_timer_reg_0_int_map::LP_TIMER_REG_0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_timer_reg_0_int_map;
#[doc = "LP_TIMER_REG_1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timer_reg_1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timer_reg_1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timer_reg_1_int_map`] module"]
pub type LP_TIMER_REG_1_INT_MAP = crate::Reg<lp_timer_reg_1_int_map::LP_TIMER_REG_1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_timer_reg_1_int_map;
#[doc = "MB_HP_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mb_hp_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mb_hp_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_hp_int_map`] module"]
pub type MB_HP_INT_MAP = crate::Reg<mb_hp_int_map::MB_HP_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod mb_hp_int_map;
#[doc = "MB_LP_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mb_lp_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mb_lp_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_lp_int_map`] module"]
pub type MB_LP_INT_MAP = crate::Reg<mb_lp_int_map::MB_LP_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod mb_lp_int_map;
#[doc = "PMU_REG_0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_reg_0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_reg_0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_reg_0_int_map`] module"]
pub type PMU_REG_0_INT_MAP = crate::Reg<pmu_reg_0_int_map::PMU_REG_0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pmu_reg_0_int_map;
#[doc = "PMU_REG_1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_reg_1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_reg_1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_reg_1_int_map`] module"]
pub type PMU_REG_1_INT_MAP = crate::Reg<pmu_reg_1_int_map::PMU_REG_1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pmu_reg_1_int_map;
#[doc = "LP_ANAPERI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_anaperi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_anaperi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_anaperi_int_map`] module"]
pub type LP_ANAPERI_INT_MAP = crate::Reg<lp_anaperi_int_map::LP_ANAPERI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_anaperi_int_map;
#[doc = "LP_ADC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_adc_int_map`] module"]
pub type LP_ADC_INT_MAP = crate::Reg<lp_adc_int_map::LP_ADC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_adc_int_map;
#[doc = "LP_GPIO_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_gpio_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_gpio_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_gpio_int_map`] module"]
pub type LP_GPIO_INT_MAP = crate::Reg<lp_gpio_int_map::LP_GPIO_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_gpio_int_map;
#[doc = "LP_I2C_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2c_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2c_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2c_int_map`] module"]
pub type LP_I2C_INT_MAP = crate::Reg<lp_i2c_int_map::LP_I2C_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_i2c_int_map;
#[doc = "LP_I2S_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_i2s_int_map`] module"]
pub type LP_I2S_INT_MAP = crate::Reg<lp_i2s_int_map::LP_I2S_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_i2s_int_map;
#[doc = "LP_SPI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_spi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_spi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_spi_int_map`] module"]
pub type LP_SPI_INT_MAP = crate::Reg<lp_spi_int_map::LP_SPI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_spi_int_map;
#[doc = "LP_TOUCH_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_touch_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_touch_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_touch_int_map`] module"]
pub type LP_TOUCH_INT_MAP = crate::Reg<lp_touch_int_map::LP_TOUCH_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_touch_int_map;
#[doc = "LP_TSENS_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tsens_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tsens_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tsens_int_map`] module"]
pub type LP_TSENS_INT_MAP = crate::Reg<lp_tsens_int_map::LP_TSENS_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_tsens_int_map;
#[doc = "LP_UART_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_uart_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_uart_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_uart_int_map`] module"]
pub type LP_UART_INT_MAP = crate::Reg<lp_uart_int_map::LP_UART_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_uart_int_map;
#[doc = "LP_EFUSE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_efuse_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_efuse_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_efuse_int_map`] module"]
pub type LP_EFUSE_INT_MAP = crate::Reg<lp_efuse_int_map::LP_EFUSE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_efuse_int_map;
#[doc = "LP_SW_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sw_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sw_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sw_int_map`] module"]
pub type LP_SW_INT_MAP = crate::Reg<lp_sw_int_map::LP_SW_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_sw_int_map;
#[doc = "LP_SYSREG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sysreg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sysreg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sysreg_int_map`] module"]
pub type LP_SYSREG_INT_MAP = crate::Reg<lp_sysreg_int_map::LP_SYSREG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_sysreg_int_map;
#[doc = "LP_HUK_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_huk_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_huk_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_huk_int_map`] module"]
pub type LP_HUK_INT_MAP = crate::Reg<lp_huk_int_map::LP_HUK_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lp_huk_int_map;
#[doc = "SYS_ICM_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_icm_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_icm_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_icm_int_map`] module"]
pub type SYS_ICM_INT_MAP = crate::Reg<sys_icm_int_map::SYS_ICM_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod sys_icm_int_map;
#[doc = "USB_DEVICE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_device_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_device_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_int_map`] module"]
pub type USB_DEVICE_INT_MAP = crate::Reg<usb_device_int_map::USB_DEVICE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod usb_device_int_map;
#[doc = "SDIO_HOST_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_host_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_host_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_host_int_map`] module"]
pub type SDIO_HOST_INT_MAP = crate::Reg<sdio_host_int_map::SDIO_HOST_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod sdio_host_int_map;
#[doc = "GDMA_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdma_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdma_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_int_map`] module"]
pub type GDMA_INT_MAP = crate::Reg<gdma_int_map::GDMA_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod gdma_int_map;
#[doc = "SPI2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_int_map`] module"]
pub type SPI2_INT_MAP = crate::Reg<spi2_int_map::SPI2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod spi2_int_map;
#[doc = "SPI3_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi3_int_map`] module"]
pub type SPI3_INT_MAP = crate::Reg<spi3_int_map::SPI3_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod spi3_int_map;
#[doc = "I2S0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_int_map`] module"]
pub type I2S0_INT_MAP = crate::Reg<i2s0_int_map::I2S0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i2s0_int_map;
#[doc = "I2S1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_int_map`] module"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i2s1_int_map;
#[doc = "I2S2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s2_int_map`] module"]
pub type I2S2_INT_MAP = crate::Reg<i2s2_int_map::I2S2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i2s2_int_map;
#[doc = "UHCI0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci0_int_map`] module"]
pub type UHCI0_INT_MAP = crate::Reg<uhci0_int_map::UHCI0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uhci0_int_map;
#[doc = "UART0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_int_map`] module"]
pub type UART0_INT_MAP = crate::Reg<uart0_int_map::UART0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uart0_int_map;
#[doc = "UART1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_int_map`] module"]
pub type UART1_INT_MAP = crate::Reg<uart1_int_map::UART1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uart1_int_map;
#[doc = "UART2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_int_map`] module"]
pub type UART2_INT_MAP = crate::Reg<uart2_int_map::UART2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uart2_int_map;
#[doc = "UART3_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_int_map`] module"]
pub type UART3_INT_MAP = crate::Reg<uart3_int_map::UART3_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uart3_int_map;
#[doc = "UART4_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart4_int_map`] module"]
pub type UART4_INT_MAP = crate::Reg<uart4_int_map::UART4_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod uart4_int_map;
#[doc = "LCD_CAM_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cam_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cam_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cam_int_map`] module"]
pub type LCD_CAM_INT_MAP = crate::Reg<lcd_cam_int_map::LCD_CAM_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lcd_cam_int_map;
#[doc = "ADC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_int_map`] module"]
pub type ADC_INT_MAP = crate::Reg<adc_int_map::ADC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod adc_int_map;
#[doc = "PWM0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_int_map`] module"]
pub type PWM0_INT_MAP = crate::Reg<pwm0_int_map::PWM0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pwm0_int_map;
#[doc = "PWM1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_int_map`] module"]
pub type PWM1_INT_MAP = crate::Reg<pwm1_int_map::PWM1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pwm1_int_map;
#[doc = "CAN0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_int_map`] module"]
pub type CAN0_INT_MAP = crate::Reg<can0_int_map::CAN0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod can0_int_map;
#[doc = "CAN1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1_int_map`] module"]
pub type CAN1_INT_MAP = crate::Reg<can1_int_map::CAN1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod can1_int_map;
#[doc = "CAN2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can2_int_map`] module"]
pub type CAN2_INT_MAP = crate::Reg<can2_int_map::CAN2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod can2_int_map;
#[doc = "RMT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_int_map`] module"]
pub type RMT_INT_MAP = crate::Reg<rmt_int_map::RMT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod rmt_int_map;
#[doc = "I2C0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_int_map`] module"]
pub type I2C0_INT_MAP = crate::Reg<i2c0_int_map::I2C0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i2c0_int_map;
#[doc = "I2C1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_int_map`] module"]
pub type I2C1_INT_MAP = crate::Reg<i2c1_int_map::I2C1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i2c1_int_map;
#[doc = "TIMERGRP0_T0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp0_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp0_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_t0_int_map`] module"]
pub type TIMERGRP0_T0_INT_MAP = crate::Reg<timergrp0_t0_int_map::TIMERGRP0_T0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp0_t0_int_map;
#[doc = "TIMERGRP0_T1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp0_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp0_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_t1_int_map`] module"]
pub type TIMERGRP0_T1_INT_MAP = crate::Reg<timergrp0_t1_int_map::TIMERGRP0_T1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp0_t1_int_map;
#[doc = "TIMERGRP0_WDT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp0_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp0_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_wdt_int_map`] module"]
pub type TIMERGRP0_WDT_INT_MAP = crate::Reg<timergrp0_wdt_int_map::TIMERGRP0_WDT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp0_wdt_int_map;
#[doc = "TIMERGRP1_T0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp1_t0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp1_t0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_t0_int_map`] module"]
pub type TIMERGRP1_T0_INT_MAP = crate::Reg<timergrp1_t0_int_map::TIMERGRP1_T0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp1_t0_int_map;
#[doc = "TIMERGRP1_T1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp1_t1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp1_t1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_t1_int_map`] module"]
pub type TIMERGRP1_T1_INT_MAP = crate::Reg<timergrp1_t1_int_map::TIMERGRP1_T1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp1_t1_int_map;
#[doc = "TIMERGRP1_WDT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timergrp1_wdt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergrp1_wdt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_wdt_int_map`] module"]
pub type TIMERGRP1_WDT_INT_MAP = crate::Reg<timergrp1_wdt_int_map::TIMERGRP1_WDT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod timergrp1_wdt_int_map;
#[doc = "LEDC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_int_map`] module"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ledc_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target0_int_map`] module"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target1_int_map`] module"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimer_target2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_target2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_target2_int_map`] module"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod systimer_target2_int_map;
#[doc = "AHB_PDMA_IN_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch0_int_map`] module"]
pub type AHB_PDMA_IN_CH0_INT_MAP =
    crate::Reg<ahb_pdma_in_ch0_int_map::AHB_PDMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_in_ch0_int_map;
#[doc = "AHB_PDMA_IN_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch1_int_map`] module"]
pub type AHB_PDMA_IN_CH1_INT_MAP =
    crate::Reg<ahb_pdma_in_ch1_int_map::AHB_PDMA_IN_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_in_ch1_int_map;
#[doc = "AHB_PDMA_IN_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_in_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_in_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_in_ch2_int_map`] module"]
pub type AHB_PDMA_IN_CH2_INT_MAP =
    crate::Reg<ahb_pdma_in_ch2_int_map::AHB_PDMA_IN_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_in_ch2_int_map;
#[doc = "AHB_PDMA_OUT_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch0_int_map`] module"]
pub type AHB_PDMA_OUT_CH0_INT_MAP =
    crate::Reg<ahb_pdma_out_ch0_int_map::AHB_PDMA_OUT_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_out_ch0_int_map;
#[doc = "AHB_PDMA_OUT_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch1_int_map`] module"]
pub type AHB_PDMA_OUT_CH1_INT_MAP =
    crate::Reg<ahb_pdma_out_ch1_int_map::AHB_PDMA_OUT_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_out_ch1_int_map;
#[doc = "AHB_PDMA_OUT_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_pdma_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_pdma_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_out_ch2_int_map`] module"]
pub type AHB_PDMA_OUT_CH2_INT_MAP =
    crate::Reg<ahb_pdma_out_ch2_int_map::AHB_PDMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ahb_pdma_out_ch2_int_map;
#[doc = "AXI_PDMA_IN_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch0_int_map`] module"]
pub type AXI_PDMA_IN_CH0_INT_MAP =
    crate::Reg<axi_pdma_in_ch0_int_map::AXI_PDMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_in_ch0_int_map;
#[doc = "AXI_PDMA_IN_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch1_int_map`] module"]
pub type AXI_PDMA_IN_CH1_INT_MAP =
    crate::Reg<axi_pdma_in_ch1_int_map::AXI_PDMA_IN_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_in_ch1_int_map;
#[doc = "AXI_PDMA_IN_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_in_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_in_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_in_ch2_int_map`] module"]
pub type AXI_PDMA_IN_CH2_INT_MAP =
    crate::Reg<axi_pdma_in_ch2_int_map::AXI_PDMA_IN_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_in_ch2_int_map;
#[doc = "AXI_PDMA_OUT_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch0_int_map`] module"]
pub type AXI_PDMA_OUT_CH0_INT_MAP =
    crate::Reg<axi_pdma_out_ch0_int_map::AXI_PDMA_OUT_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_out_ch0_int_map;
#[doc = "AXI_PDMA_OUT_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch1_int_map`] module"]
pub type AXI_PDMA_OUT_CH1_INT_MAP =
    crate::Reg<axi_pdma_out_ch1_int_map::AXI_PDMA_OUT_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_out_ch1_int_map;
#[doc = "AXI_PDMA_OUT_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_pdma_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_pdma_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_out_ch2_int_map`] module"]
pub type AXI_PDMA_OUT_CH2_INT_MAP =
    crate::Reg<axi_pdma_out_ch2_int_map::AXI_PDMA_OUT_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod axi_pdma_out_ch2_int_map;
#[doc = "RSA_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_int_map`] module"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_int_map`] module"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_int_map`] module"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod sha_int_map;
#[doc = "ECC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_int_map`] module"]
pub type ECC_INT_MAP = crate::Reg<ecc_int_map::ECC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ecc_int_map;
#[doc = "ECDSA_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecdsa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecdsa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa_int_map`] module"]
pub type ECDSA_INT_MAP = crate::Reg<ecdsa_int_map::ECDSA_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ecdsa_int_map;
#[doc = "KM_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`km_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`km_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_int_map`] module"]
pub type KM_INT_MAP = crate::Reg<km_int_map::KM_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod km_int_map;
#[doc = "GPIO_INT0_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int0_map`] module"]
pub type GPIO_INT0_MAP = crate::Reg<gpio_int0_map::GPIO_INT0_MAP_SPEC>;
#[doc = "NA"]
pub mod gpio_int0_map;
#[doc = "GPIO_INT1_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int1_map`] module"]
pub type GPIO_INT1_MAP = crate::Reg<gpio_int1_map::GPIO_INT1_MAP_SPEC>;
#[doc = "NA"]
pub mod gpio_int1_map;
#[doc = "GPIO_INT2_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int2_map`] module"]
pub type GPIO_INT2_MAP = crate::Reg<gpio_int2_map::GPIO_INT2_MAP_SPEC>;
#[doc = "NA"]
pub mod gpio_int2_map;
#[doc = "GPIO_INT3_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int3_map`] module"]
pub type GPIO_INT3_MAP = crate::Reg<gpio_int3_map::GPIO_INT3_MAP_SPEC>;
#[doc = "NA"]
pub mod gpio_int3_map;
#[doc = "GPIO_PAD_COMP_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pad_comp_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pad_comp_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pad_comp_int_map`] module"]
pub type GPIO_PAD_COMP_INT_MAP = crate::Reg<gpio_pad_comp_int_map::GPIO_PAD_COMP_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod gpio_pad_comp_int_map;
#[doc = "CPU_INT_FROM_CPU_0_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_from_cpu_0_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_from_cpu_0_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_from_cpu_0_map`] module"]
pub type CPU_INT_FROM_CPU_0_MAP = crate::Reg<cpu_int_from_cpu_0_map::CPU_INT_FROM_CPU_0_MAP_SPEC>;
#[doc = "NA"]
pub mod cpu_int_from_cpu_0_map;
#[doc = "CPU_INT_FROM_CPU_1_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_from_cpu_1_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_from_cpu_1_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_from_cpu_1_map`] module"]
pub type CPU_INT_FROM_CPU_1_MAP = crate::Reg<cpu_int_from_cpu_1_map::CPU_INT_FROM_CPU_1_MAP_SPEC>;
#[doc = "NA"]
pub mod cpu_int_from_cpu_1_map;
#[doc = "CPU_INT_FROM_CPU_2_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_from_cpu_2_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_from_cpu_2_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_from_cpu_2_map`] module"]
pub type CPU_INT_FROM_CPU_2_MAP = crate::Reg<cpu_int_from_cpu_2_map::CPU_INT_FROM_CPU_2_MAP_SPEC>;
#[doc = "NA"]
pub mod cpu_int_from_cpu_2_map;
#[doc = "CPU_INT_FROM_CPU_3_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_from_cpu_3_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_from_cpu_3_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_int_from_cpu_3_map`] module"]
pub type CPU_INT_FROM_CPU_3_MAP = crate::Reg<cpu_int_from_cpu_3_map::CPU_INT_FROM_CPU_3_MAP_SPEC>;
#[doc = "NA"]
pub mod cpu_int_from_cpu_3_map;
#[doc = "CACHE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_int_map`] module"]
pub type CACHE_INT_MAP = crate::Reg<cache_int_map::CACHE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod cache_int_map;
#[doc = "FLASH_MSPI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_mspi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_mspi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_mspi_int_map`] module"]
pub type FLASH_MSPI_INT_MAP = crate::Reg<flash_mspi_int_map::FLASH_MSPI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod flash_mspi_int_map;
#[doc = "CSI_BRIDGE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_bridge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_bridge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_bridge_int_map`] module"]
pub type CSI_BRIDGE_INT_MAP = crate::Reg<csi_bridge_int_map::CSI_BRIDGE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod csi_bridge_int_map;
#[doc = "DSI_BRIDGE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_bridge_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_bridge_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_bridge_int_map`] module"]
pub type DSI_BRIDGE_INT_MAP = crate::Reg<dsi_bridge_int_map::DSI_BRIDGE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dsi_bridge_int_map;
#[doc = "CSI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_int_map`] module"]
pub type CSI_INT_MAP = crate::Reg<csi_int_map::CSI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod csi_int_map;
#[doc = "DSI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_int_map`] module"]
pub type DSI_INT_MAP = crate::Reg<dsi_int_map::DSI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dsi_int_map;
#[doc = "GMII_PHY_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_phy_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_phy_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_phy_int_map`] module"]
pub type GMII_PHY_INT_MAP = crate::Reg<gmii_phy_int_map::GMII_PHY_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod gmii_phy_int_map;
#[doc = "LPI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpi_int_map`] module"]
pub type LPI_INT_MAP = crate::Reg<lpi_int_map::LPI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod lpi_int_map;
#[doc = "PMT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_int_map`] module"]
pub type PMT_INT_MAP = crate::Reg<pmt_int_map::PMT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pmt_int_map;
#[doc = "SBD_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbd_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbd_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbd_int_map`] module"]
pub type SBD_INT_MAP = crate::Reg<sbd_int_map::SBD_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod sbd_int_map;
#[doc = "USB_OTG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_int_map`] module"]
pub type USB_OTG_INT_MAP = crate::Reg<usb_otg_int_map::USB_OTG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod usb_otg_int_map;
#[doc = "USB_OTG_ENDP_MULTI_PROC_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_endp_multi_proc_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_endp_multi_proc_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_endp_multi_proc_int_map`] module"]
pub type USB_OTG_ENDP_MULTI_PROC_INT_MAP =
    crate::Reg<usb_otg_endp_multi_proc_int_map::USB_OTG_ENDP_MULTI_PROC_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod usb_otg_endp_multi_proc_int_map;
#[doc = "JPEG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jpeg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jpeg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_int_map`] module"]
pub type JPEG_INT_MAP = crate::Reg<jpeg_int_map::JPEG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod jpeg_int_map;
#[doc = "PPA_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppa_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppa_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppa_int_map`] module"]
pub type PPA_INT_MAP = crate::Reg<ppa_int_map::PPA_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod ppa_int_map;
#[doc = "CORE0_TRACE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_trace_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_trace_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core0_trace_int_map`] module"]
pub type CORE0_TRACE_INT_MAP = crate::Reg<core0_trace_int_map::CORE0_TRACE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod core0_trace_int_map;
#[doc = "CORE1_TRACE_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_trace_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core1_trace_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_trace_int_map`] module"]
pub type CORE1_TRACE_INT_MAP = crate::Reg<core1_trace_int_map::CORE1_TRACE_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod core1_trace_int_map;
#[doc = "HP_CORE_CTRL_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_ctrl_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_ctrl_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_ctrl_int_map`] module"]
pub type HP_CORE_CTRL_INT_MAP = crate::Reg<hp_core_ctrl_int_map::HP_CORE_CTRL_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod hp_core_ctrl_int_map;
#[doc = "ISP_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_int_map`] module"]
pub type ISP_INT_MAP = crate::Reg<isp_int_map::ISP_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod isp_int_map;
#[doc = "I3C_MST_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_mst_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_mst_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c_mst_int_map`] module"]
pub type I3C_MST_INT_MAP = crate::Reg<i3c_mst_int_map::I3C_MST_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i3c_mst_int_map;
#[doc = "I3C_SLV_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_slv_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_slv_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3c_slv_int_map`] module"]
pub type I3C_SLV_INT_MAP = crate::Reg<i3c_slv_int_map::I3C_SLV_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod i3c_slv_int_map;
#[doc = "USB_OTG11_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg11_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg11_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg11_int_map`] module"]
pub type USB_OTG11_INT_MAP = crate::Reg<usb_otg11_int_map::USB_OTG11_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod usb_otg11_int_map;
#[doc = "DMA2D_IN_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2d_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2d_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_in_ch0_int_map`] module"]
pub type DMA2D_IN_CH0_INT_MAP = crate::Reg<dma2d_in_ch0_int_map::DMA2D_IN_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dma2d_in_ch0_int_map;
#[doc = "DMA2D_IN_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2d_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2d_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_in_ch1_int_map`] module"]
pub type DMA2D_IN_CH1_INT_MAP = crate::Reg<dma2d_in_ch1_int_map::DMA2D_IN_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dma2d_in_ch1_int_map;
#[doc = "DMA2D_OUT_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2d_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2d_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch0_int_map`] module"]
pub type DMA2D_OUT_CH0_INT_MAP = crate::Reg<dma2d_out_ch0_int_map::DMA2D_OUT_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dma2d_out_ch0_int_map;
#[doc = "DMA2D_OUT_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2d_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2d_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch1_int_map`] module"]
pub type DMA2D_OUT_CH1_INT_MAP = crate::Reg<dma2d_out_ch1_int_map::DMA2D_OUT_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dma2d_out_ch1_int_map;
#[doc = "DMA2D_OUT_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2d_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2d_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_out_ch2_int_map`] module"]
pub type DMA2D_OUT_CH2_INT_MAP = crate::Reg<dma2d_out_ch2_int_map::DMA2D_OUT_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod dma2d_out_ch2_int_map;
#[doc = "PSRAM_MSPI_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psram_mspi_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psram_mspi_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mspi_int_map`] module"]
pub type PSRAM_MSPI_INT_MAP = crate::Reg<psram_mspi_int_map::PSRAM_MSPI_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod psram_mspi_int_map;
#[doc = "HP_SYSREG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_sysreg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sysreg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sysreg_int_map`] module"]
pub type HP_SYSREG_INT_MAP = crate::Reg<hp_sysreg_int_map::HP_SYSREG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod hp_sysreg_int_map;
#[doc = "PCNT_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcnt_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt_int_map`] module"]
pub type PCNT_INT_MAP = crate::Reg<pcnt_int_map::PCNT_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod pcnt_int_map;
#[doc = "HP_PAU_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_pau_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_pau_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_pau_int_map`] module"]
pub type HP_PAU_INT_MAP = crate::Reg<hp_pau_int_map::HP_PAU_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod hp_pau_int_map;
#[doc = "HP_PARLIO_RX_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_parlio_rx_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_parlio_rx_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_parlio_rx_int_map`] module"]
pub type HP_PARLIO_RX_INT_MAP = crate::Reg<hp_parlio_rx_int_map::HP_PARLIO_RX_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod hp_parlio_rx_int_map;
#[doc = "HP_PARLIO_TX_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_parlio_tx_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_parlio_tx_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_parlio_tx_int_map`] module"]
pub type HP_PARLIO_TX_INT_MAP = crate::Reg<hp_parlio_tx_int_map::HP_PARLIO_TX_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod hp_parlio_tx_int_map;
#[doc = "H264_DMA2D_OUT_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_out_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_out_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_out_ch0_int_map`] module"]
pub type H264_DMA2D_OUT_CH0_INT_MAP =
    crate::Reg<h264_dma2d_out_ch0_int_map::H264_DMA2D_OUT_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_out_ch0_int_map;
#[doc = "H264_DMA2D_OUT_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_out_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_out_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_out_ch1_int_map`] module"]
pub type H264_DMA2D_OUT_CH1_INT_MAP =
    crate::Reg<h264_dma2d_out_ch1_int_map::H264_DMA2D_OUT_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_out_ch1_int_map;
#[doc = "H264_DMA2D_OUT_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_out_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_out_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_out_ch2_int_map`] module"]
pub type H264_DMA2D_OUT_CH2_INT_MAP =
    crate::Reg<h264_dma2d_out_ch2_int_map::H264_DMA2D_OUT_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_out_ch2_int_map;
#[doc = "H264_DMA2D_OUT_CH3_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_out_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_out_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_out_ch3_int_map`] module"]
pub type H264_DMA2D_OUT_CH3_INT_MAP =
    crate::Reg<h264_dma2d_out_ch3_int_map::H264_DMA2D_OUT_CH3_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_out_ch3_int_map;
#[doc = "H264_DMA2D_OUT_CH4_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_out_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_out_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_out_ch4_int_map`] module"]
pub type H264_DMA2D_OUT_CH4_INT_MAP =
    crate::Reg<h264_dma2d_out_ch4_int_map::H264_DMA2D_OUT_CH4_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_out_ch4_int_map;
#[doc = "H264_DMA2D_IN_CH0_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch0_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch0_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch0_int_map`] module"]
pub type H264_DMA2D_IN_CH0_INT_MAP =
    crate::Reg<h264_dma2d_in_ch0_int_map::H264_DMA2D_IN_CH0_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch0_int_map;
#[doc = "H264_DMA2D_IN_CH1_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch1_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch1_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch1_int_map`] module"]
pub type H264_DMA2D_IN_CH1_INT_MAP =
    crate::Reg<h264_dma2d_in_ch1_int_map::H264_DMA2D_IN_CH1_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch1_int_map;
#[doc = "H264_DMA2D_IN_CH2_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch2_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch2_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch2_int_map`] module"]
pub type H264_DMA2D_IN_CH2_INT_MAP =
    crate::Reg<h264_dma2d_in_ch2_int_map::H264_DMA2D_IN_CH2_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch2_int_map;
#[doc = "H264_DMA2D_IN_CH3_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch3_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch3_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch3_int_map`] module"]
pub type H264_DMA2D_IN_CH3_INT_MAP =
    crate::Reg<h264_dma2d_in_ch3_int_map::H264_DMA2D_IN_CH3_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch3_int_map;
#[doc = "H264_DMA2D_IN_CH4_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch4_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch4_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch4_int_map`] module"]
pub type H264_DMA2D_IN_CH4_INT_MAP =
    crate::Reg<h264_dma2d_in_ch4_int_map::H264_DMA2D_IN_CH4_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch4_int_map;
#[doc = "H264_DMA2D_IN_CH5_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_dma2d_in_ch5_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_dma2d_in_ch5_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_dma2d_in_ch5_int_map`] module"]
pub type H264_DMA2D_IN_CH5_INT_MAP =
    crate::Reg<h264_dma2d_in_ch5_int_map::H264_DMA2D_IN_CH5_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_dma2d_in_ch5_int_map;
#[doc = "H264_REG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h264_reg_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h264_reg_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h264_reg_int_map`] module"]
pub type H264_REG_INT_MAP = crate::Reg<h264_reg_int_map::H264_REG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod h264_reg_int_map;
#[doc = "ASSIST_DEBUG_INT_MAP (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`assist_debug_int_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_debug_int_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_debug_int_map`] module"]
pub type ASSIST_DEBUG_INT_MAP = crate::Reg<assist_debug_int_map::ASSIST_DEBUG_INT_MAP_SPEC>;
#[doc = "NA"]
pub mod assist_debug_int_map;
#[doc = "INTR_STATUS_REG_0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_0`] module"]
pub type INTR_STATUS_REG_0 = crate::Reg<intr_status_reg_0::INTR_STATUS_REG_0_SPEC>;
#[doc = "NA"]
pub mod intr_status_reg_0;
#[doc = "INTR_STATUS_REG_1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_1`] module"]
pub type INTR_STATUS_REG_1 = crate::Reg<intr_status_reg_1::INTR_STATUS_REG_1_SPEC>;
#[doc = "NA"]
pub mod intr_status_reg_1;
#[doc = "INTR_STATUS_REG_2 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_2`] module"]
pub type INTR_STATUS_REG_2 = crate::Reg<intr_status_reg_2::INTR_STATUS_REG_2_SPEC>;
#[doc = "NA"]
pub mod intr_status_reg_2;
#[doc = "INTR_STATUS_REG_3 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_status_reg_3`] module"]
pub type INTR_STATUS_REG_3 = crate::Reg<intr_status_reg_3::INTR_STATUS_REG_3_SPEC>;
#[doc = "NA"]
pub mod intr_status_reg_3;
#[doc = "CLOCK_GATE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "NA"]
pub mod clock_gate;
#[doc = "INTERRUPT_REG_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_reg_date`] module"]
pub type INTERRUPT_REG_DATE = crate::Reg<interrupt_reg_date::INTERRUPT_REG_DATE_SPEC>;
#[doc = "NA"]
pub mod interrupt_reg_date;
