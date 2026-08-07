#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    uart0_conf: UART0_CONF,
    uart0_sclk_conf: UART0_SCLK_CONF,
    uart0_mem_lp_ctrl: UART0_MEM_LP_CTRL,
    uart1_conf: UART1_CONF,
    uart1_sclk_conf: UART1_SCLK_CONF,
    uart1_mem_lp_ctrl: UART1_MEM_LP_CTRL,
    mspi_conf: MSPI_CONF,
    mspi_clk_conf: MSPI_CLK_CONF,
    mspi_mem_lp_ctrl: MSPI_MEM_LP_CTRL,
    i2c0_conf: I2C0_CONF,
    i2c0_sclk_conf: I2C0_SCLK_CONF,
    twai0_conf: TWAI0_CONF,
    twai0_func_clk_conf: TWAI0_FUNC_CLK_CONF,
    uhci_conf: UHCI_CONF,
    rmt_conf: RMT_CONF,
    rmt_sclk_conf: RMT_SCLK_CONF,
    rmt_mem_lp_ctrl: RMT_MEM_LP_CTRL,
    ledc_conf: LEDC_CONF,
    ledc_sclk_conf: LEDC_SCLK_CONF,
    _reserved19: [u8; 0x04],
    timergroup0_conf: TIMERGROUP0_CONF,
    timergroup0_timer_clk_conf: TIMERGROUP0_TIMER_CLK_CONF,
    timergroup0_wdt_clk_conf: TIMERGROUP0_WDT_CLK_CONF,
    timergroup1_conf: TIMERGROUP1_CONF,
    timergroup1_timer_clk_conf: TIMERGROUP1_TIMER_CLK_CONF,
    timergroup1_wdt_clk_conf: TIMERGROUP1_WDT_CLK_CONF,
    systimer_conf: SYSTIMER_CONF,
    systimer_func_clk_conf: SYSTIMER_FUNC_CLK_CONF,
    i2s_conf: I2S_CONF,
    i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    saradc_conf: SARADC_CONF,
    saradc_clkm_conf: SARADC_CLKM_CONF,
    tsens_clk_conf: TSENS_CLK_CONF,
    usb_device_conf: USB_DEVICE_CONF,
    usb_device_mem_lp_ctrl: USB_DEVICE_MEM_LP_CTRL,
    intmtx_conf: INTMTX_CONF,
    pcnt_conf: PCNT_CONF,
    etm_conf: ETM_CONF,
    pwm0_conf: PWM0_CONF,
    pwm0_clk_conf: PWM0_CLK_CONF,
    parl_io_conf: PARL_IO_CONF,
    parl_clk_rx_conf: PARL_CLK_RX_CONF,
    parl_clk_tx_conf: PARL_CLK_TX_CONF,
    pvt_monitor_conf: PVT_MONITOR_CONF,
    pvt_monitor_func_clk_conf: PVT_MONITOR_FUNC_CLK_CONF,
    gdma_conf: GDMA_CONF,
    spi2_conf: SPI2_CONF,
    spi2_clkm_conf: SPI2_CLKM_CONF,
    aes_conf: AES_CONF,
    sha_conf: SHA_CONF,
    rsa_conf: RSA_CONF,
    _reserved53: [u8; 0x04],
    ecc_conf: ECC_CONF,
    ecc_mem_lp_ctrl: ECC_MEM_LP_CTRL,
    ds_conf: DS_CONF,
    hmac_conf: HMAC_CONF,
    ecdsa_conf: ECDSA_CONF,
    iomux_conf: IOMUX_CONF,
    iomux_clk_conf: IOMUX_CLK_CONF,
    regdma_conf: REGDMA_CONF,
    trace_conf: TRACE_CONF,
    assist_conf: ASSIST_CONF,
    cache_conf: CACHE_CONF,
    cache_mem_lp_ctrl: CACHE_MEM_LP_CTRL,
    modem_conf: MODEM_CONF,
    timeout_conf: TIMEOUT_CONF,
    sysclk_conf: SYSCLK_CONF,
    cpu_waiti_conf: CPU_WAITI_CONF,
    cpu_freq_conf: CPU_FREQ_CONF,
    ahb_freq_conf: AHB_FREQ_CONF,
    apb_freq_conf: APB_FREQ_CONF,
    sysclk_freq_query_0: SYSCLK_FREQ_QUERY_0,
    pll_div_clk_en: PLL_DIV_CLK_EN,
    ctrl_clk_out_en: CTRL_CLK_OUT_EN,
    timg_cali_clk_conf: TIMG_CALI_CLK_CONF,
    rom_mem_lp_ctrl: ROM_MEM_LP_CTRL,
    sram_power_conf_1: SRAM_POWER_CONF_1,
    sec_conf: SEC_CONF,
    adc_inv_phase_conf: ADC_INV_PHASE_CONF,
    bus_clk_update: BUS_CLK_UPDATE,
    sar_clk_div: SAR_CLK_DIV,
    pwdet_sar_clk_conf: PWDET_SAR_CLK_CONF,
    timergroup_wdt_conf: TIMERGROUP_WDT_CONF,
    _reserved84: [u8; 0x08],
    timergroup_xtal_conf: TIMERGROUP_XTAL_CONF,
    km_conf: KM_CONF,
    km_mem_lp_ctrl: KM_MEM_LP_CTRL,
    tcm_mem_monitor_conf: TCM_MEM_MONITOR_CONF,
    psram_mem_monitor_conf: PSRAM_MEM_MONITOR_CONF,
    reset_event_bypass: RESET_EVENT_BYPASS,
    hpcore_mem_lp_ctrl: HPCORE_MEM_LP_CTRL,
    usb_otg11_conf: USB_OTG11_CONF,
    usb_otg11_mem_lp_ctrl: USB_OTG11_MEM_LP_CTRL,
    sdm_inv_phase_conf: SDM_INV_PHASE_CONF,
    core1_conf: CORE1_CONF,
    i2c1_conf: I2C1_CONF,
    i2c1_sclk_conf: I2C1_SCLK_CONF,
    pwm1_conf: PWM1_CONF,
    pwm1_clk_conf: PWM1_CLK_CONF,
    spi3_conf: SPI3_CONF,
    spi3_clkm_conf: SPI3_CLKM_CONF,
    _reserved101: [u8; 0x04],
    asrc_func_clk_conf: ASRC_FUNC_CLK_CONF,
    bus_clock_gate_bypass: BUS_CLOCK_GATE_BYPASS,
    zero_det_conf: ZERO_DET_CONF,
    zero_det_clk_conf: ZERO_DET_CLK_CONF,
    rst_en: RST_EN,
    fpga_debug: FPGA_DEBUG,
    _reserved107: [u8; 0x0e38],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - UART0 configuration register"]
    #[inline(always)]
    pub const fn uart0_conf(&self) -> &UART0_CONF {
        &self.uart0_conf
    }
    #[doc = "0x04 - UART0_SCLK configuration register"]
    #[inline(always)]
    pub const fn uart0_sclk_conf(&self) -> &UART0_SCLK_CONF {
        &self.uart0_sclk_conf
    }
    #[doc = "0x08 - UART0 memory power control register"]
    #[inline(always)]
    pub const fn uart0_mem_lp_ctrl(&self) -> &UART0_MEM_LP_CTRL {
        &self.uart0_mem_lp_ctrl
    }
    #[doc = "0x0c - UART1 configuration register"]
    #[inline(always)]
    pub const fn uart1_conf(&self) -> &UART1_CONF {
        &self.uart1_conf
    }
    #[doc = "0x10 - UART1_SCLK configuration register"]
    #[inline(always)]
    pub const fn uart1_sclk_conf(&self) -> &UART1_SCLK_CONF {
        &self.uart1_sclk_conf
    }
    #[doc = "0x14 - UART1 memory power control register"]
    #[inline(always)]
    pub const fn uart1_mem_lp_ctrl(&self) -> &UART1_MEM_LP_CTRL {
        &self.uart1_mem_lp_ctrl
    }
    #[doc = "0x18 - MSPI configuration register"]
    #[inline(always)]
    pub const fn mspi_conf(&self) -> &MSPI_CONF {
        &self.mspi_conf
    }
    #[doc = "0x1c - MSPI_CLK configuration register"]
    #[inline(always)]
    pub const fn mspi_clk_conf(&self) -> &MSPI_CLK_CONF {
        &self.mspi_clk_conf
    }
    #[doc = "0x20 - MSPI memory power control register"]
    #[inline(always)]
    pub const fn mspi_mem_lp_ctrl(&self) -> &MSPI_MEM_LP_CTRL {
        &self.mspi_mem_lp_ctrl
    }
    #[doc = "0x24 - I2C configuration register"]
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> &I2C0_CONF {
        &self.i2c0_conf
    }
    #[doc = "0x28 - I2C_SCLK configuration register"]
    #[inline(always)]
    pub const fn i2c0_sclk_conf(&self) -> &I2C0_SCLK_CONF {
        &self.i2c0_sclk_conf
    }
    #[doc = "0x2c - TWAI0 configuration register"]
    #[inline(always)]
    pub const fn twai0_conf(&self) -> &TWAI0_CONF {
        &self.twai0_conf
    }
    #[doc = "0x30 - TWAI0_FUNC_CLK configuration register"]
    #[inline(always)]
    pub const fn twai0_func_clk_conf(&self) -> &TWAI0_FUNC_CLK_CONF {
        &self.twai0_func_clk_conf
    }
    #[doc = "0x34 - UHCI configuration register"]
    #[inline(always)]
    pub const fn uhci_conf(&self) -> &UHCI_CONF {
        &self.uhci_conf
    }
    #[doc = "0x38 - RMT configuration register"]
    #[inline(always)]
    pub const fn rmt_conf(&self) -> &RMT_CONF {
        &self.rmt_conf
    }
    #[doc = "0x3c - RMT_SCLK configuration register"]
    #[inline(always)]
    pub const fn rmt_sclk_conf(&self) -> &RMT_SCLK_CONF {
        &self.rmt_sclk_conf
    }
    #[doc = "0x40 - RMT memory power control register"]
    #[inline(always)]
    pub const fn rmt_mem_lp_ctrl(&self) -> &RMT_MEM_LP_CTRL {
        &self.rmt_mem_lp_ctrl
    }
    #[doc = "0x44 - LEDC configuration register"]
    #[inline(always)]
    pub const fn ledc_conf(&self) -> &LEDC_CONF {
        &self.ledc_conf
    }
    #[doc = "0x48 - LEDC_SCLK configuration register"]
    #[inline(always)]
    pub const fn ledc_sclk_conf(&self) -> &LEDC_SCLK_CONF {
        &self.ledc_sclk_conf
    }
    #[doc = "0x50 - TIMERGROUP0 configuration register"]
    #[inline(always)]
    pub const fn timergroup0_conf(&self) -> &TIMERGROUP0_CONF {
        &self.timergroup0_conf
    }
    #[doc = "0x54 - TIMERGROUP0_TIMER_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup0_timer_clk_conf(&self) -> &TIMERGROUP0_TIMER_CLK_CONF {
        &self.timergroup0_timer_clk_conf
    }
    #[doc = "0x58 - TIMERGROUP0_WDT_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup0_wdt_clk_conf(&self) -> &TIMERGROUP0_WDT_CLK_CONF {
        &self.timergroup0_wdt_clk_conf
    }
    #[doc = "0x5c - TIMERGROUP1 configuration register"]
    #[inline(always)]
    pub const fn timergroup1_conf(&self) -> &TIMERGROUP1_CONF {
        &self.timergroup1_conf
    }
    #[doc = "0x60 - TIMERGROUP1_TIMER_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup1_timer_clk_conf(&self) -> &TIMERGROUP1_TIMER_CLK_CONF {
        &self.timergroup1_timer_clk_conf
    }
    #[doc = "0x64 - TIMERGROUP1_WDT_CLK configuration register"]
    #[inline(always)]
    pub const fn timergroup1_wdt_clk_conf(&self) -> &TIMERGROUP1_WDT_CLK_CONF {
        &self.timergroup1_wdt_clk_conf
    }
    #[doc = "0x68 - SYSTIMER configuration register"]
    #[inline(always)]
    pub const fn systimer_conf(&self) -> &SYSTIMER_CONF {
        &self.systimer_conf
    }
    #[doc = "0x6c - SYSTIMER_FUNC_CLK configuration register"]
    #[inline(always)]
    pub const fn systimer_func_clk_conf(&self) -> &SYSTIMER_FUNC_CLK_CONF {
        &self.systimer_func_clk_conf
    }
    #[doc = "0x70 - I2S configuration register"]
    #[inline(always)]
    pub const fn i2s_conf(&self) -> &I2S_CONF {
        &self.i2s_conf
    }
    #[doc = "0x74 - I2S_TX_CLKM configuration register"]
    #[inline(always)]
    pub const fn i2s_tx_clkm_conf(&self) -> &I2S_TX_CLKM_CONF {
        &self.i2s_tx_clkm_conf
    }
    #[doc = "0x78 - I2S_TX_CLKM_DIV configuration register"]
    #[inline(always)]
    pub const fn i2s_tx_clkm_div_conf(&self) -> &I2S_TX_CLKM_DIV_CONF {
        &self.i2s_tx_clkm_div_conf
    }
    #[doc = "0x7c - I2S_RX_CLKM configuration register"]
    #[inline(always)]
    pub const fn i2s_rx_clkm_conf(&self) -> &I2S_RX_CLKM_CONF {
        &self.i2s_rx_clkm_conf
    }
    #[doc = "0x80 - I2S_RX_CLKM_DIV configuration register"]
    #[inline(always)]
    pub const fn i2s_rx_clkm_div_conf(&self) -> &I2S_RX_CLKM_DIV_CONF {
        &self.i2s_rx_clkm_div_conf
    }
    #[doc = "0x84 - SARADC configuration register"]
    #[inline(always)]
    pub const fn saradc_conf(&self) -> &SARADC_CONF {
        &self.saradc_conf
    }
    #[doc = "0x88 - SARADC_CLKM configuration register"]
    #[inline(always)]
    pub const fn saradc_clkm_conf(&self) -> &SARADC_CLKM_CONF {
        &self.saradc_clkm_conf
    }
    #[doc = "0x8c - TSENS_CLK configuration register"]
    #[inline(always)]
    pub const fn tsens_clk_conf(&self) -> &TSENS_CLK_CONF {
        &self.tsens_clk_conf
    }
    #[doc = "0x90 - USB_DEVICE configuration register"]
    #[inline(always)]
    pub const fn usb_device_conf(&self) -> &USB_DEVICE_CONF {
        &self.usb_device_conf
    }
    #[doc = "0x94 - USB_DEVICE memory power control register"]
    #[inline(always)]
    pub const fn usb_device_mem_lp_ctrl(&self) -> &USB_DEVICE_MEM_LP_CTRL {
        &self.usb_device_mem_lp_ctrl
    }
    #[doc = "0x98 - INTMTX configuration register"]
    #[inline(always)]
    pub const fn intmtx_conf(&self) -> &INTMTX_CONF {
        &self.intmtx_conf
    }
    #[doc = "0x9c - PCNT configuration register"]
    #[inline(always)]
    pub const fn pcnt_conf(&self) -> &PCNT_CONF {
        &self.pcnt_conf
    }
    #[doc = "0xa0 - ETM configuration register"]
    #[inline(always)]
    pub const fn etm_conf(&self) -> &ETM_CONF {
        &self.etm_conf
    }
    #[doc = "0xa4 - PWM0 configuration register"]
    #[inline(always)]
    pub const fn pwm0_conf(&self) -> &PWM0_CONF {
        &self.pwm0_conf
    }
    #[doc = "0xa8 - PWM0_CLK configuration register"]
    #[inline(always)]
    pub const fn pwm0_clk_conf(&self) -> &PWM0_CLK_CONF {
        &self.pwm0_clk_conf
    }
    #[doc = "0xac - PARL_IO configuration register"]
    #[inline(always)]
    pub const fn parl_io_conf(&self) -> &PARL_IO_CONF {
        &self.parl_io_conf
    }
    #[doc = "0xb0 - PARL_CLK_RX configuration register"]
    #[inline(always)]
    pub const fn parl_clk_rx_conf(&self) -> &PARL_CLK_RX_CONF {
        &self.parl_clk_rx_conf
    }
    #[doc = "0xb4 - PARL_CLK_TX configuration register"]
    #[inline(always)]
    pub const fn parl_clk_tx_conf(&self) -> &PARL_CLK_TX_CONF {
        &self.parl_clk_tx_conf
    }
    #[doc = "0xb8 - PVT_MONITOR configuration register"]
    #[inline(always)]
    pub const fn pvt_monitor_conf(&self) -> &PVT_MONITOR_CONF {
        &self.pvt_monitor_conf
    }
    #[doc = "0xbc - PVT_MONITOR function clock configuration register"]
    #[inline(always)]
    pub const fn pvt_monitor_func_clk_conf(&self) -> &PVT_MONITOR_FUNC_CLK_CONF {
        &self.pvt_monitor_func_clk_conf
    }
    #[doc = "0xc0 - GDMA configuration register"]
    #[inline(always)]
    pub const fn gdma_conf(&self) -> &GDMA_CONF {
        &self.gdma_conf
    }
    #[doc = "0xc4 - SPI2 configuration register"]
    #[inline(always)]
    pub const fn spi2_conf(&self) -> &SPI2_CONF {
        &self.spi2_conf
    }
    #[doc = "0xc8 - SPI2_CLKM configuration register"]
    #[inline(always)]
    pub const fn spi2_clkm_conf(&self) -> &SPI2_CLKM_CONF {
        &self.spi2_clkm_conf
    }
    #[doc = "0xcc - AES configuration register"]
    #[inline(always)]
    pub const fn aes_conf(&self) -> &AES_CONF {
        &self.aes_conf
    }
    #[doc = "0xd0 - SHA configuration register"]
    #[inline(always)]
    pub const fn sha_conf(&self) -> &SHA_CONF {
        &self.sha_conf
    }
    #[doc = "0xd4 - RSA configuration register"]
    #[inline(always)]
    pub const fn rsa_conf(&self) -> &RSA_CONF {
        &self.rsa_conf
    }
    #[doc = "0xdc - ECC configuration register"]
    #[inline(always)]
    pub const fn ecc_conf(&self) -> &ECC_CONF {
        &self.ecc_conf
    }
    #[doc = "0xe0 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn ecc_mem_lp_ctrl(&self) -> &ECC_MEM_LP_CTRL {
        &self.ecc_mem_lp_ctrl
    }
    #[doc = "0xe4 - DS configuration register"]
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DS_CONF {
        &self.ds_conf
    }
    #[doc = "0xe8 - HMAC configuration register"]
    #[inline(always)]
    pub const fn hmac_conf(&self) -> &HMAC_CONF {
        &self.hmac_conf
    }
    #[doc = "0xec - ECDSA configuration register"]
    #[inline(always)]
    pub const fn ecdsa_conf(&self) -> &ECDSA_CONF {
        &self.ecdsa_conf
    }
    #[doc = "0xf0 - IOMUX configuration register"]
    #[inline(always)]
    pub const fn iomux_conf(&self) -> &IOMUX_CONF {
        &self.iomux_conf
    }
    #[doc = "0xf4 - IOMUX_CLK configuration register"]
    #[inline(always)]
    pub const fn iomux_clk_conf(&self) -> &IOMUX_CLK_CONF {
        &self.iomux_clk_conf
    }
    #[doc = "0xf8 - REGDMA configuration register"]
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &REGDMA_CONF {
        &self.regdma_conf
    }
    #[doc = "0xfc - TRACE configuration register"]
    #[inline(always)]
    pub const fn trace_conf(&self) -> &TRACE_CONF {
        &self.trace_conf
    }
    #[doc = "0x100 - ASSIST configuration register"]
    #[inline(always)]
    pub const fn assist_conf(&self) -> &ASSIST_CONF {
        &self.assist_conf
    }
    #[doc = "0x104 - CACHE configuration register"]
    #[inline(always)]
    pub const fn cache_conf(&self) -> &CACHE_CONF {
        &self.cache_conf
    }
    #[doc = "0x108 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn cache_mem_lp_ctrl(&self) -> &CACHE_MEM_LP_CTRL {
        &self.cache_mem_lp_ctrl
    }
    #[doc = "0x10c - MODEM_APB configuration register"]
    #[inline(always)]
    pub const fn modem_conf(&self) -> &MODEM_CONF {
        &self.modem_conf
    }
    #[doc = "0x110 - TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn timeout_conf(&self) -> &TIMEOUT_CONF {
        &self.timeout_conf
    }
    #[doc = "0x114 - SYSCLK configuration register"]
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    #[doc = "0x118 - CPU_WAITI configuration register"]
    #[inline(always)]
    pub const fn cpu_waiti_conf(&self) -> &CPU_WAITI_CONF {
        &self.cpu_waiti_conf
    }
    #[doc = "0x11c - CPU_FREQ configuration register"]
    #[inline(always)]
    pub const fn cpu_freq_conf(&self) -> &CPU_FREQ_CONF {
        &self.cpu_freq_conf
    }
    #[doc = "0x120 - AHB_FREQ configuration register"]
    #[inline(always)]
    pub const fn ahb_freq_conf(&self) -> &AHB_FREQ_CONF {
        &self.ahb_freq_conf
    }
    #[doc = "0x124 - APB_FREQ configuration register"]
    #[inline(always)]
    pub const fn apb_freq_conf(&self) -> &APB_FREQ_CONF {
        &self.apb_freq_conf
    }
    #[doc = "0x128 - SYSCLK frequency query 0 register"]
    #[inline(always)]
    pub const fn sysclk_freq_query_0(&self) -> &SYSCLK_FREQ_QUERY_0 {
        &self.sysclk_freq_query_0
    }
    #[doc = "0x12c - SPLL DIV clock-gating configuration register"]
    #[inline(always)]
    pub const fn pll_div_clk_en(&self) -> &PLL_DIV_CLK_EN {
        &self.pll_div_clk_en
    }
    #[doc = "0x130 - CLK_OUT_EN configuration register"]
    #[inline(always)]
    pub const fn ctrl_clk_out_en(&self) -> &CTRL_CLK_OUT_EN {
        &self.ctrl_clk_out_en
    }
    #[doc = "0x134 - timergrout calibrate clock configuration register"]
    #[inline(always)]
    pub const fn timg_cali_clk_conf(&self) -> &TIMG_CALI_CLK_CONF {
        &self.timg_cali_clk_conf
    }
    #[doc = "0x138 - rom power control register"]
    #[inline(always)]
    pub const fn rom_mem_lp_ctrl(&self) -> &ROM_MEM_LP_CTRL {
        &self.rom_mem_lp_ctrl
    }
    #[doc = "0x13c - HP SRAM/ROM configuration register"]
    #[inline(always)]
    pub const fn sram_power_conf_1(&self) -> &SRAM_POWER_CONF_1 {
        &self.sram_power_conf_1
    }
    #[doc = "0x140 - Clock source configuration register for External Memory Encryption and Decryption"]
    #[inline(always)]
    pub const fn sec_conf(&self) -> &SEC_CONF {
        &self.sec_conf
    }
    #[doc = "0x144 - xxxx"]
    #[inline(always)]
    pub const fn adc_inv_phase_conf(&self) -> &ADC_INV_PHASE_CONF {
        &self.adc_inv_phase_conf
    }
    #[doc = "0x148 - Configuration register for applying updated high-performance system clock sources"]
    #[inline(always)]
    pub const fn bus_clk_update(&self) -> &BUS_CLK_UPDATE {
        &self.bus_clk_update
    }
    #[doc = "0x14c - SAR ADC clock divisor configuration register"]
    #[inline(always)]
    pub const fn sar_clk_div(&self) -> &SAR_CLK_DIV {
        &self.sar_clk_div
    }
    #[doc = "0x150 - xxxx"]
    #[inline(always)]
    pub const fn pwdet_sar_clk_conf(&self) -> &PWDET_SAR_CLK_CONF {
        &self.pwdet_sar_clk_conf
    }
    #[doc = "0x154 - TIMERGROUP_WDT configuration register"]
    #[inline(always)]
    pub const fn timergroup_wdt_conf(&self) -> &TIMERGROUP_WDT_CONF {
        &self.timergroup_wdt_conf
    }
    #[doc = "0x160 - TIMERGROUP1 configuration register"]
    #[inline(always)]
    pub const fn timergroup_xtal_conf(&self) -> &TIMERGROUP_XTAL_CONF {
        &self.timergroup_xtal_conf
    }
    #[doc = "0x164 - Key Manager configuration register"]
    #[inline(always)]
    pub const fn km_conf(&self) -> &KM_CONF {
        &self.km_conf
    }
    #[doc = "0x168 - KM memory power control register"]
    #[inline(always)]
    pub const fn km_mem_lp_ctrl(&self) -> &KM_MEM_LP_CTRL {
        &self.km_mem_lp_ctrl
    }
    #[doc = "0x16c - TCM_MEM_MONITOR configuration register"]
    #[inline(always)]
    pub const fn tcm_mem_monitor_conf(&self) -> &TCM_MEM_MONITOR_CONF {
        &self.tcm_mem_monitor_conf
    }
    #[doc = "0x170 - PSRAM_MEM_MONITOR configuration register"]
    #[inline(always)]
    pub const fn psram_mem_monitor_conf(&self) -> &PSRAM_MEM_MONITOR_CONF {
        &self.psram_mem_monitor_conf
    }
    #[doc = "0x174 - reset event bypass backdoor configuration register"]
    #[inline(always)]
    pub const fn reset_event_bypass(&self) -> &RESET_EVENT_BYPASS {
        &self.reset_event_bypass
    }
    #[doc = "0x178 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn hpcore_mem_lp_ctrl(&self) -> &HPCORE_MEM_LP_CTRL {
        &self.hpcore_mem_lp_ctrl
    }
    #[doc = "0x17c - USB_OTG configuration register"]
    #[inline(always)]
    pub const fn usb_otg11_conf(&self) -> &USB_OTG11_CONF {
        &self.usb_otg11_conf
    }
    #[doc = "0x180 - USB_OTG11 memory power control register"]
    #[inline(always)]
    pub const fn usb_otg11_mem_lp_ctrl(&self) -> &USB_OTG11_MEM_LP_CTRL {
        &self.usb_otg11_mem_lp_ctrl
    }
    #[doc = "0x184 - xxxx"]
    #[inline(always)]
    pub const fn sdm_inv_phase_conf(&self) -> &SDM_INV_PHASE_CONF {
        &self.sdm_inv_phase_conf
    }
    #[doc = "0x188 - USB_OTG configuration register"]
    #[inline(always)]
    pub const fn core1_conf(&self) -> &CORE1_CONF {
        &self.core1_conf
    }
    #[doc = "0x18c - I2C configuration register"]
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> &I2C1_CONF {
        &self.i2c1_conf
    }
    #[doc = "0x190 - I2C_SCLK configuration register"]
    #[inline(always)]
    pub const fn i2c1_sclk_conf(&self) -> &I2C1_SCLK_CONF {
        &self.i2c1_sclk_conf
    }
    #[doc = "0x194 - PWM1 configuration register"]
    #[inline(always)]
    pub const fn pwm1_conf(&self) -> &PWM1_CONF {
        &self.pwm1_conf
    }
    #[doc = "0x198 - PWM1_CLK configuration register"]
    #[inline(always)]
    pub const fn pwm1_clk_conf(&self) -> &PWM1_CLK_CONF {
        &self.pwm1_clk_conf
    }
    #[doc = "0x19c - SPI3 configuration register"]
    #[inline(always)]
    pub const fn spi3_conf(&self) -> &SPI3_CONF {
        &self.spi3_conf
    }
    #[doc = "0x1a0 - SPI3_CLKM configuration register"]
    #[inline(always)]
    pub const fn spi3_clkm_conf(&self) -> &SPI3_CLKM_CONF {
        &self.spi3_clkm_conf
    }
    #[doc = "0x1a8 - AUDIO_SAMPLE_RATE_FUNC_CLK configuration register"]
    #[inline(always)]
    pub const fn asrc_func_clk_conf(&self) -> &ASRC_FUNC_CLK_CONF {
        &self.asrc_func_clk_conf
    }
    #[doc = "0x1ac - bus clock gating bypass configuration register"]
    #[inline(always)]
    pub const fn bus_clock_gate_bypass(&self) -> &BUS_CLOCK_GATE_BYPASS {
        &self.bus_clock_gate_bypass
    }
    #[doc = "0x1b0 - ZERO_DET configuration register"]
    #[inline(always)]
    pub const fn zero_det_conf(&self) -> &ZERO_DET_CONF {
        &self.zero_det_conf
    }
    #[doc = "0x1b4 - ZERO_DET_CLK configuration register"]
    #[inline(always)]
    pub const fn zero_det_clk_conf(&self) -> &ZERO_DET_CLK_CONF {
        &self.zero_det_clk_conf
    }
    #[doc = "0x1b8 - PCR clock gating configure register"]
    #[inline(always)]
    pub const fn rst_en(&self) -> &RST_EN {
        &self.rst_en
    }
    #[doc = "0x1bc - fpga debug register"]
    #[inline(always)]
    pub const fn fpga_debug(&self) -> &FPGA_DEBUG {
        &self.fpga_debug
    }
    #[doc = "0xff8 - PCR clock gating configure register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xffc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "UART0_CONF (rw) register accessor: UART0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_conf`] module"]
pub type UART0_CONF = crate::Reg<uart0_conf::UART0_CONF_SPEC>;
#[doc = "UART0 configuration register"]
pub mod uart0_conf;
#[doc = "UART0_SCLK_CONF (rw) register accessor: UART0_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_sclk_conf`] module"]
pub type UART0_SCLK_CONF = crate::Reg<uart0_sclk_conf::UART0_SCLK_CONF_SPEC>;
#[doc = "UART0_SCLK configuration register"]
pub mod uart0_sclk_conf;
#[doc = "UART0_MEM_LP_CTRL (rw) register accessor: UART0 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_mem_lp_ctrl`] module"]
pub type UART0_MEM_LP_CTRL = crate::Reg<uart0_mem_lp_ctrl::UART0_MEM_LP_CTRL_SPEC>;
#[doc = "UART0 memory power control register"]
pub mod uart0_mem_lp_ctrl;
#[doc = "UART1_CONF (rw) register accessor: UART1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_conf`] module"]
pub type UART1_CONF = crate::Reg<uart1_conf::UART1_CONF_SPEC>;
#[doc = "UART1 configuration register"]
pub mod uart1_conf;
#[doc = "UART1_SCLK_CONF (rw) register accessor: UART1_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_sclk_conf`] module"]
pub type UART1_SCLK_CONF = crate::Reg<uart1_sclk_conf::UART1_SCLK_CONF_SPEC>;
#[doc = "UART1_SCLK configuration register"]
pub mod uart1_sclk_conf;
#[doc = "UART1_MEM_LP_CTRL (rw) register accessor: UART1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_mem_lp_ctrl`] module"]
pub type UART1_MEM_LP_CTRL = crate::Reg<uart1_mem_lp_ctrl::UART1_MEM_LP_CTRL_SPEC>;
#[doc = "UART1 memory power control register"]
pub mod uart1_mem_lp_ctrl;
#[doc = "MSPI_CONF (rw) register accessor: MSPI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_conf`] module"]
pub type MSPI_CONF = crate::Reg<mspi_conf::MSPI_CONF_SPEC>;
#[doc = "MSPI configuration register"]
pub mod mspi_conf;
#[doc = "MSPI_CLK_CONF (rw) register accessor: MSPI_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_clk_conf`] module"]
pub type MSPI_CLK_CONF = crate::Reg<mspi_clk_conf::MSPI_CLK_CONF_SPEC>;
#[doc = "MSPI_CLK configuration register"]
pub mod mspi_clk_conf;
#[doc = "MSPI_MEM_LP_CTRL (rw) register accessor: MSPI memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_mem_lp_ctrl`] module"]
pub type MSPI_MEM_LP_CTRL = crate::Reg<mspi_mem_lp_ctrl::MSPI_MEM_LP_CTRL_SPEC>;
#[doc = "MSPI memory power control register"]
pub mod mspi_mem_lp_ctrl;
#[doc = "I2C0_CONF (rw) register accessor: I2C configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_conf`] module"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "I2C configuration register"]
pub mod i2c0_conf;
#[doc = "I2C0_SCLK_CONF (rw) register accessor: I2C_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_sclk_conf`] module"]
pub type I2C0_SCLK_CONF = crate::Reg<i2c0_sclk_conf::I2C0_SCLK_CONF_SPEC>;
#[doc = "I2C_SCLK configuration register"]
pub mod i2c0_sclk_conf;
#[doc = "TWAI0_CONF (rw) register accessor: TWAI0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`twai0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twai0_conf`] module"]
pub type TWAI0_CONF = crate::Reg<twai0_conf::TWAI0_CONF_SPEC>;
#[doc = "TWAI0 configuration register"]
pub mod twai0_conf;
#[doc = "TWAI0_FUNC_CLK_CONF (rw) register accessor: TWAI0_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`twai0_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai0_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twai0_func_clk_conf`] module"]
pub type TWAI0_FUNC_CLK_CONF = crate::Reg<twai0_func_clk_conf::TWAI0_FUNC_CLK_CONF_SPEC>;
#[doc = "TWAI0_FUNC_CLK configuration register"]
pub mod twai0_func_clk_conf;
#[doc = "UHCI_CONF (rw) register accessor: UHCI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci_conf`] module"]
pub type UHCI_CONF = crate::Reg<uhci_conf::UHCI_CONF_SPEC>;
#[doc = "UHCI configuration register"]
pub mod uhci_conf;
#[doc = "RMT_CONF (rw) register accessor: RMT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_conf`] module"]
pub type RMT_CONF = crate::Reg<rmt_conf::RMT_CONF_SPEC>;
#[doc = "RMT configuration register"]
pub mod rmt_conf;
#[doc = "RMT_SCLK_CONF (rw) register accessor: RMT_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_sclk_conf`] module"]
pub type RMT_SCLK_CONF = crate::Reg<rmt_sclk_conf::RMT_SCLK_CONF_SPEC>;
#[doc = "RMT_SCLK configuration register"]
pub mod rmt_sclk_conf;
#[doc = "RMT_MEM_LP_CTRL (rw) register accessor: RMT memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_mem_lp_ctrl`] module"]
pub type RMT_MEM_LP_CTRL = crate::Reg<rmt_mem_lp_ctrl::RMT_MEM_LP_CTRL_SPEC>;
#[doc = "RMT memory power control register"]
pub mod rmt_mem_lp_ctrl;
#[doc = "LEDC_CONF (rw) register accessor: LEDC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_conf`] module"]
pub type LEDC_CONF = crate::Reg<ledc_conf::LEDC_CONF_SPEC>;
#[doc = "LEDC configuration register"]
pub mod ledc_conf;
#[doc = "LEDC_SCLK_CONF (rw) register accessor: LEDC_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_sclk_conf`] module"]
pub type LEDC_SCLK_CONF = crate::Reg<ledc_sclk_conf::LEDC_SCLK_CONF_SPEC>;
#[doc = "LEDC_SCLK configuration register"]
pub mod ledc_sclk_conf;
#[doc = "TIMERGROUP0_CONF (rw) register accessor: TIMERGROUP0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_conf`] module"]
pub type TIMERGROUP0_CONF = crate::Reg<timergroup0_conf::TIMERGROUP0_CONF_SPEC>;
#[doc = "TIMERGROUP0 configuration register"]
pub mod timergroup0_conf;
#[doc = "TIMERGROUP0_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP0_TIMER_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_timer_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_timer_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_timer_clk_conf`] module"]
pub type TIMERGROUP0_TIMER_CLK_CONF =
    crate::Reg<timergroup0_timer_clk_conf::TIMERGROUP0_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_TIMER_CLK configuration register"]
pub mod timergroup0_timer_clk_conf;
#[doc = "TIMERGROUP0_WDT_CLK_CONF (rw) register accessor: TIMERGROUP0_WDT_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_wdt_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_wdt_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup0_wdt_clk_conf`] module"]
pub type TIMERGROUP0_WDT_CLK_CONF =
    crate::Reg<timergroup0_wdt_clk_conf::TIMERGROUP0_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_WDT_CLK configuration register"]
pub mod timergroup0_wdt_clk_conf;
#[doc = "TIMERGROUP1_CONF (rw) register accessor: TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_conf`] module"]
pub type TIMERGROUP1_CONF = crate::Reg<timergroup1_conf::TIMERGROUP1_CONF_SPEC>;
#[doc = "TIMERGROUP1 configuration register"]
pub mod timergroup1_conf;
#[doc = "TIMERGROUP1_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP1_TIMER_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_timer_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_timer_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_timer_clk_conf`] module"]
pub type TIMERGROUP1_TIMER_CLK_CONF =
    crate::Reg<timergroup1_timer_clk_conf::TIMERGROUP1_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_TIMER_CLK configuration register"]
pub mod timergroup1_timer_clk_conf;
#[doc = "TIMERGROUP1_WDT_CLK_CONF (rw) register accessor: TIMERGROUP1_WDT_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_wdt_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_wdt_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup1_wdt_clk_conf`] module"]
pub type TIMERGROUP1_WDT_CLK_CONF =
    crate::Reg<timergroup1_wdt_clk_conf::TIMERGROUP1_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_WDT_CLK configuration register"]
pub mod timergroup1_wdt_clk_conf;
#[doc = "SYSTIMER_CONF (rw) register accessor: SYSTIMER configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_conf`] module"]
pub type SYSTIMER_CONF = crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>;
#[doc = "SYSTIMER configuration register"]
pub mod systimer_conf;
#[doc = "SYSTIMER_FUNC_CLK_CONF (rw) register accessor: SYSTIMER_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_func_clk_conf`] module"]
pub type SYSTIMER_FUNC_CLK_CONF = crate::Reg<systimer_func_clk_conf::SYSTIMER_FUNC_CLK_CONF_SPEC>;
#[doc = "SYSTIMER_FUNC_CLK configuration register"]
pub mod systimer_func_clk_conf;
#[doc = "I2S_CONF (rw) register accessor: I2S configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_conf`] module"]
pub type I2S_CONF = crate::Reg<i2s_conf::I2S_CONF_SPEC>;
#[doc = "I2S configuration register"]
pub mod i2s_conf;
#[doc = "I2S_TX_CLKM_CONF (rw) register accessor: I2S_TX_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_tx_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_tx_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_tx_clkm_conf`] module"]
pub type I2S_TX_CLKM_CONF = crate::Reg<i2s_tx_clkm_conf::I2S_TX_CLKM_CONF_SPEC>;
#[doc = "I2S_TX_CLKM configuration register"]
pub mod i2s_tx_clkm_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF (rw) register accessor: I2S_TX_CLKM_DIV configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_tx_clkm_div_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_tx_clkm_div_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_tx_clkm_div_conf`] module"]
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<i2s_tx_clkm_div_conf::I2S_TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_TX_CLKM_DIV configuration register"]
pub mod i2s_tx_clkm_div_conf;
#[doc = "I2S_RX_CLKM_CONF (rw) register accessor: I2S_RX_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_rx_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_rx_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rx_clkm_conf`] module"]
pub type I2S_RX_CLKM_CONF = crate::Reg<i2s_rx_clkm_conf::I2S_RX_CLKM_CONF_SPEC>;
#[doc = "I2S_RX_CLKM configuration register"]
pub mod i2s_rx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF (rw) register accessor: I2S_RX_CLKM_DIV configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_rx_clkm_div_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_rx_clkm_div_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s_rx_clkm_div_conf`] module"]
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<i2s_rx_clkm_div_conf::I2S_RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_RX_CLKM_DIV configuration register"]
pub mod i2s_rx_clkm_div_conf;
#[doc = "SARADC_CONF (rw) register accessor: SARADC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_conf`] module"]
pub type SARADC_CONF = crate::Reg<saradc_conf::SARADC_CONF_SPEC>;
#[doc = "SARADC configuration register"]
pub mod saradc_conf;
#[doc = "SARADC_CLKM_CONF (rw) register accessor: SARADC_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`saradc_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saradc_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_clkm_conf`] module"]
pub type SARADC_CLKM_CONF = crate::Reg<saradc_clkm_conf::SARADC_CLKM_CONF_SPEC>;
#[doc = "SARADC_CLKM configuration register"]
pub mod saradc_clkm_conf;
#[doc = "TSENS_CLK_CONF (rw) register accessor: TSENS_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_clk_conf`] module"]
pub type TSENS_CLK_CONF = crate::Reg<tsens_clk_conf::TSENS_CLK_CONF_SPEC>;
#[doc = "TSENS_CLK configuration register"]
pub mod tsens_clk_conf;
#[doc = "USB_DEVICE_CONF (rw) register accessor: USB_DEVICE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_conf`] module"]
pub type USB_DEVICE_CONF = crate::Reg<usb_device_conf::USB_DEVICE_CONF_SPEC>;
#[doc = "USB_DEVICE configuration register"]
pub mod usb_device_conf;
#[doc = "USB_DEVICE_MEM_LP_CTRL (rw) register accessor: USB_DEVICE memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_mem_lp_ctrl`] module"]
pub type USB_DEVICE_MEM_LP_CTRL = crate::Reg<usb_device_mem_lp_ctrl::USB_DEVICE_MEM_LP_CTRL_SPEC>;
#[doc = "USB_DEVICE memory power control register"]
pub mod usb_device_mem_lp_ctrl;
#[doc = "INTMTX_CONF (rw) register accessor: INTMTX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmtx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmtx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmtx_conf`] module"]
pub type INTMTX_CONF = crate::Reg<intmtx_conf::INTMTX_CONF_SPEC>;
#[doc = "INTMTX configuration register"]
pub mod intmtx_conf;
#[doc = "PCNT_CONF (rw) register accessor: PCNT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt_conf`] module"]
pub type PCNT_CONF = crate::Reg<pcnt_conf::PCNT_CONF_SPEC>;
#[doc = "PCNT configuration register"]
pub mod pcnt_conf;
#[doc = "ETM_CONF (rw) register accessor: ETM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_conf`] module"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "ETM configuration register"]
pub mod etm_conf;
#[doc = "PWM0_CONF (rw) register accessor: PWM0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_conf`] module"]
pub type PWM0_CONF = crate::Reg<pwm0_conf::PWM0_CONF_SPEC>;
#[doc = "PWM0 configuration register"]
pub mod pwm0_conf;
#[doc = "PWM0_CLK_CONF (rw) register accessor: PWM0_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_clk_conf`] module"]
pub type PWM0_CLK_CONF = crate::Reg<pwm0_clk_conf::PWM0_CLK_CONF_SPEC>;
#[doc = "PWM0_CLK configuration register"]
pub mod pwm0_clk_conf;
#[doc = "PARL_IO_CONF (rw) register accessor: PARL_IO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_io_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_io_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parl_io_conf`] module"]
pub type PARL_IO_CONF = crate::Reg<parl_io_conf::PARL_IO_CONF_SPEC>;
#[doc = "PARL_IO configuration register"]
pub mod parl_io_conf;
#[doc = "PARL_CLK_RX_CONF (rw) register accessor: PARL_CLK_RX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_clk_rx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_clk_rx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parl_clk_rx_conf`] module"]
pub type PARL_CLK_RX_CONF = crate::Reg<parl_clk_rx_conf::PARL_CLK_RX_CONF_SPEC>;
#[doc = "PARL_CLK_RX configuration register"]
pub mod parl_clk_rx_conf;
#[doc = "PARL_CLK_TX_CONF (rw) register accessor: PARL_CLK_TX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_clk_tx_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_clk_tx_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parl_clk_tx_conf`] module"]
pub type PARL_CLK_TX_CONF = crate::Reg<parl_clk_tx_conf::PARL_CLK_TX_CONF_SPEC>;
#[doc = "PARL_CLK_TX configuration register"]
pub mod parl_clk_tx_conf;
#[doc = "PVT_MONITOR_CONF (rw) register accessor: PVT_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_conf`] module"]
pub type PVT_MONITOR_CONF = crate::Reg<pvt_monitor_conf::PVT_MONITOR_CONF_SPEC>;
#[doc = "PVT_MONITOR configuration register"]
pub mod pvt_monitor_conf;
#[doc = "PVT_MONITOR_FUNC_CLK_CONF (rw) register accessor: PVT_MONITOR function clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt_monitor_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt_monitor_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt_monitor_func_clk_conf`] module"]
pub type PVT_MONITOR_FUNC_CLK_CONF =
    crate::Reg<pvt_monitor_func_clk_conf::PVT_MONITOR_FUNC_CLK_CONF_SPEC>;
#[doc = "PVT_MONITOR function clock configuration register"]
pub mod pvt_monitor_func_clk_conf;
#[doc = "GDMA_CONF (rw) register accessor: GDMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_conf`] module"]
pub type GDMA_CONF = crate::Reg<gdma_conf::GDMA_CONF_SPEC>;
#[doc = "GDMA configuration register"]
pub mod gdma_conf;
#[doc = "SPI2_CONF (rw) register accessor: SPI2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_conf`] module"]
pub type SPI2_CONF = crate::Reg<spi2_conf::SPI2_CONF_SPEC>;
#[doc = "SPI2 configuration register"]
pub mod spi2_conf;
#[doc = "SPI2_CLKM_CONF (rw) register accessor: SPI2_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi2_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi2_clkm_conf`] module"]
pub type SPI2_CLKM_CONF = crate::Reg<spi2_clkm_conf::SPI2_CLKM_CONF_SPEC>;
#[doc = "SPI2_CLKM configuration register"]
pub mod spi2_clkm_conf;
#[doc = "AES_CONF (rw) register accessor: AES configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_conf`] module"]
pub type AES_CONF = crate::Reg<aes_conf::AES_CONF_SPEC>;
#[doc = "AES configuration register"]
pub mod aes_conf;
#[doc = "SHA_CONF (rw) register accessor: SHA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_conf`] module"]
pub type SHA_CONF = crate::Reg<sha_conf::SHA_CONF_SPEC>;
#[doc = "SHA configuration register"]
pub mod sha_conf;
#[doc = "RSA_CONF (r) register accessor: RSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_conf`] module"]
pub type RSA_CONF = crate::Reg<rsa_conf::RSA_CONF_SPEC>;
#[doc = "RSA configuration register"]
pub mod rsa_conf;
#[doc = "ECC_CONF (rw) register accessor: ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_conf`] module"]
pub type ECC_CONF = crate::Reg<ecc_conf::ECC_CONF_SPEC>;
#[doc = "ECC configuration register"]
pub mod ecc_conf;
#[doc = "ECC_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_mem_lp_ctrl`] module"]
pub type ECC_MEM_LP_CTRL = crate::Reg<ecc_mem_lp_ctrl::ECC_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod ecc_mem_lp_ctrl;
#[doc = "DS_CONF (r) register accessor: DS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_conf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_conf`] module"]
pub type DS_CONF = crate::Reg<ds_conf::DS_CONF_SPEC>;
#[doc = "DS configuration register"]
pub mod ds_conf;
#[doc = "HMAC_CONF (rw) register accessor: HMAC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hmac_conf`] module"]
pub type HMAC_CONF = crate::Reg<hmac_conf::HMAC_CONF_SPEC>;
#[doc = "HMAC configuration register"]
pub mod hmac_conf;
#[doc = "ECDSA_CONF (rw) register accessor: ECDSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecdsa_conf`] module"]
pub type ECDSA_CONF = crate::Reg<ecdsa_conf::ECDSA_CONF_SPEC>;
#[doc = "ECDSA configuration register"]
pub mod ecdsa_conf;
#[doc = "IOMUX_CONF (rw) register accessor: IOMUX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_conf`] module"]
pub type IOMUX_CONF = crate::Reg<iomux_conf::IOMUX_CONF_SPEC>;
#[doc = "IOMUX configuration register"]
pub mod iomux_conf;
#[doc = "IOMUX_CLK_CONF (rw) register accessor: IOMUX_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_clk_conf`] module"]
pub type IOMUX_CLK_CONF = crate::Reg<iomux_clk_conf::IOMUX_CLK_CONF_SPEC>;
#[doc = "IOMUX_CLK configuration register"]
pub mod iomux_clk_conf;
#[doc = "REGDMA_CONF (rw) register accessor: REGDMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_conf`] module"]
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
#[doc = "REGDMA configuration register"]
pub mod regdma_conf;
#[doc = "TRACE_CONF (rw) register accessor: TRACE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_conf`] module"]
pub type TRACE_CONF = crate::Reg<trace_conf::TRACE_CONF_SPEC>;
#[doc = "TRACE configuration register"]
pub mod trace_conf;
#[doc = "ASSIST_CONF (rw) register accessor: ASSIST configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@assist_conf`] module"]
pub type ASSIST_CONF = crate::Reg<assist_conf::ASSIST_CONF_SPEC>;
#[doc = "ASSIST configuration register"]
pub mod assist_conf;
#[doc = "CACHE_CONF (rw) register accessor: CACHE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_conf`] module"]
pub type CACHE_CONF = crate::Reg<cache_conf::CACHE_CONF_SPEC>;
#[doc = "CACHE configuration register"]
pub mod cache_conf;
#[doc = "CACHE_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_mem_lp_ctrl`] module"]
pub type CACHE_MEM_LP_CTRL = crate::Reg<cache_mem_lp_ctrl::CACHE_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod cache_mem_lp_ctrl;
#[doc = "MODEM_CONF (rw) register accessor: MODEM_APB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_conf`] module"]
pub type MODEM_CONF = crate::Reg<modem_conf::MODEM_CONF_SPEC>;
#[doc = "MODEM_APB configuration register"]
pub mod modem_conf;
#[doc = "TIMEOUT_CONF (rw) register accessor: TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_conf`] module"]
pub type TIMEOUT_CONF = crate::Reg<timeout_conf::TIMEOUT_CONF_SPEC>;
#[doc = "TIMEOUT configuration register"]
pub mod timeout_conf;
#[doc = "SYSCLK_CONF (rw) register accessor: SYSCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SYSCLK configuration register"]
pub mod sysclk_conf;
#[doc = "CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_waiti_conf`] module"]
pub type CPU_WAITI_CONF = crate::Reg<cpu_waiti_conf::CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod cpu_waiti_conf;
#[doc = "CPU_FREQ_CONF (rw) register accessor: CPU_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_freq_conf`] module"]
pub type CPU_FREQ_CONF = crate::Reg<cpu_freq_conf::CPU_FREQ_CONF_SPEC>;
#[doc = "CPU_FREQ configuration register"]
pub mod cpu_freq_conf;
#[doc = "AHB_FREQ_CONF (rw) register accessor: AHB_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_freq_conf`] module"]
pub type AHB_FREQ_CONF = crate::Reg<ahb_freq_conf::AHB_FREQ_CONF_SPEC>;
#[doc = "AHB_FREQ configuration register"]
pub mod ahb_freq_conf;
#[doc = "APB_FREQ_CONF (rw) register accessor: APB_FREQ configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_freq_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_freq_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_freq_conf`] module"]
pub type APB_FREQ_CONF = crate::Reg<apb_freq_conf::APB_FREQ_CONF_SPEC>;
#[doc = "APB_FREQ configuration register"]
pub mod apb_freq_conf;
#[doc = "SYSCLK_FREQ_QUERY_0 (r) register accessor: SYSCLK frequency query 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_freq_query_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_freq_query_0`] module"]
pub type SYSCLK_FREQ_QUERY_0 = crate::Reg<sysclk_freq_query_0::SYSCLK_FREQ_QUERY_0_SPEC>;
#[doc = "SYSCLK frequency query 0 register"]
pub mod sysclk_freq_query_0;
#[doc = "PLL_DIV_CLK_EN (rw) register accessor: SPLL DIV clock-gating configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_div_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_div_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_div_clk_en`] module"]
pub type PLL_DIV_CLK_EN = crate::Reg<pll_div_clk_en::PLL_DIV_CLK_EN_SPEC>;
#[doc = "SPLL DIV clock-gating configuration register"]
pub mod pll_div_clk_en;
#[doc = "CTRL_CLK_OUT_EN (rw) register accessor: CLK_OUT_EN configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_clk_out_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_clk_out_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_clk_out_en`] module"]
pub type CTRL_CLK_OUT_EN = crate::Reg<ctrl_clk_out_en::CTRL_CLK_OUT_EN_SPEC>;
#[doc = "CLK_OUT_EN configuration register"]
pub mod ctrl_clk_out_en;
#[doc = "TIMG_CALI_CLK_CONF (rw) register accessor: timergrout calibrate clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg_cali_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg_cali_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timg_cali_clk_conf`] module"]
pub type TIMG_CALI_CLK_CONF = crate::Reg<timg_cali_clk_conf::TIMG_CALI_CLK_CONF_SPEC>;
#[doc = "timergrout calibrate clock configuration register"]
pub mod timg_cali_clk_conf;
#[doc = "ROM_MEM_LP_CTRL (rw) register accessor: rom power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mem_lp_ctrl`] module"]
pub type ROM_MEM_LP_CTRL = crate::Reg<rom_mem_lp_ctrl::ROM_MEM_LP_CTRL_SPEC>;
#[doc = "rom power control register"]
pub mod rom_mem_lp_ctrl;
#[doc = "SRAM_POWER_CONF_1 (rw) register accessor: HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_power_conf_1`] module"]
pub type SRAM_POWER_CONF_1 = crate::Reg<sram_power_conf_1::SRAM_POWER_CONF_1_SPEC>;
#[doc = "HP SRAM/ROM configuration register"]
pub mod sram_power_conf_1;
#[doc = "SEC_CONF (rw) register accessor: Clock source configuration register for External Memory Encryption and Decryption\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_conf`] module"]
pub type SEC_CONF = crate::Reg<sec_conf::SEC_CONF_SPEC>;
#[doc = "Clock source configuration register for External Memory Encryption and Decryption"]
pub mod sec_conf;
#[doc = "ADC_INV_PHASE_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_inv_phase_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_inv_phase_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_inv_phase_conf`] module"]
pub type ADC_INV_PHASE_CONF = crate::Reg<adc_inv_phase_conf::ADC_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod adc_inv_phase_conf;
#[doc = "BUS_CLK_UPDATE (rw) register accessor: Configuration register for applying updated high-performance system clock sources\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clk_update`] module"]
pub type BUS_CLK_UPDATE = crate::Reg<bus_clk_update::BUS_CLK_UPDATE_SPEC>;
#[doc = "Configuration register for applying updated high-performance system clock sources"]
pub mod bus_clk_update;
#[doc = "SAR_CLK_DIV (rw) register accessor: SAR ADC clock divisor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar_clk_div`] module"]
pub type SAR_CLK_DIV = crate::Reg<sar_clk_div::SAR_CLK_DIV_SPEC>;
#[doc = "SAR ADC clock divisor configuration register"]
pub mod sar_clk_div;
#[doc = "PWDET_SAR_CLK_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdet_sar_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdet_sar_clk_conf`] module"]
pub type PWDET_SAR_CLK_CONF = crate::Reg<pwdet_sar_clk_conf::PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "xxxx"]
pub mod pwdet_sar_clk_conf;
#[doc = "TIMERGROUP_WDT_CONF (rw) register accessor: TIMERGROUP_WDT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup_wdt_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup_wdt_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup_wdt_conf`] module"]
pub type TIMERGROUP_WDT_CONF = crate::Reg<timergroup_wdt_conf::TIMERGROUP_WDT_CONF_SPEC>;
#[doc = "TIMERGROUP_WDT configuration register"]
pub mod timergroup_wdt_conf;
#[doc = "TIMERGROUP_XTAL_CONF (rw) register accessor: TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup_xtal_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup_xtal_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergroup_xtal_conf`] module"]
pub type TIMERGROUP_XTAL_CONF = crate::Reg<timergroup_xtal_conf::TIMERGROUP_XTAL_CONF_SPEC>;
#[doc = "TIMERGROUP1 configuration register"]
pub mod timergroup_xtal_conf;
#[doc = "KM_CONF (rw) register accessor: Key Manager configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_conf`] module"]
pub type KM_CONF = crate::Reg<km_conf::KM_CONF_SPEC>;
#[doc = "Key Manager configuration register"]
pub mod km_conf;
#[doc = "KM_MEM_LP_CTRL (rw) register accessor: KM memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_mem_lp_ctrl`] module"]
pub type KM_MEM_LP_CTRL = crate::Reg<km_mem_lp_ctrl::KM_MEM_LP_CTRL_SPEC>;
#[doc = "KM memory power control register"]
pub mod km_mem_lp_ctrl;
#[doc = "TCM_MEM_MONITOR_CONF (rw) register accessor: TCM_MEM_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_mem_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_mem_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_mem_monitor_conf`] module"]
pub type TCM_MEM_MONITOR_CONF = crate::Reg<tcm_mem_monitor_conf::TCM_MEM_MONITOR_CONF_SPEC>;
#[doc = "TCM_MEM_MONITOR configuration register"]
pub mod tcm_mem_monitor_conf;
#[doc = "PSRAM_MEM_MONITOR_CONF (rw) register accessor: PSRAM_MEM_MONITOR configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_mem_monitor_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_mem_monitor_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_mem_monitor_conf`] module"]
pub type PSRAM_MEM_MONITOR_CONF = crate::Reg<psram_mem_monitor_conf::PSRAM_MEM_MONITOR_CONF_SPEC>;
#[doc = "PSRAM_MEM_MONITOR configuration register"]
pub mod psram_mem_monitor_conf;
#[doc = "RESET_EVENT_BYPASS (rw) register accessor: reset event bypass backdoor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_event_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_event_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_event_bypass`] module"]
pub type RESET_EVENT_BYPASS = crate::Reg<reset_event_bypass::RESET_EVENT_BYPASS_SPEC>;
#[doc = "reset event bypass backdoor configuration register"]
pub mod reset_event_bypass;
#[doc = "HPCORE_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore_mem_lp_ctrl`] module"]
pub type HPCORE_MEM_LP_CTRL = crate::Reg<hpcore_mem_lp_ctrl::HPCORE_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod hpcore_mem_lp_ctrl;
#[doc = "USB_OTG11_CONF (rw) register accessor: USB_OTG configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otg11_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otg11_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg11_conf`] module"]
pub type USB_OTG11_CONF = crate::Reg<usb_otg11_conf::USB_OTG11_CONF_SPEC>;
#[doc = "USB_OTG configuration register"]
pub mod usb_otg11_conf;
#[doc = "USB_OTG11_MEM_LP_CTRL (rw) register accessor: USB_OTG11 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otg11_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otg11_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg11_mem_lp_ctrl`] module"]
pub type USB_OTG11_MEM_LP_CTRL = crate::Reg<usb_otg11_mem_lp_ctrl::USB_OTG11_MEM_LP_CTRL_SPEC>;
#[doc = "USB_OTG11 memory power control register"]
pub mod usb_otg11_mem_lp_ctrl;
#[doc = "SDM_INV_PHASE_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_inv_phase_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_inv_phase_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdm_inv_phase_conf`] module"]
pub type SDM_INV_PHASE_CONF = crate::Reg<sdm_inv_phase_conf::SDM_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod sdm_inv_phase_conf;
#[doc = "CORE1_CONF (rw) register accessor: USB_OTG configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_conf`] module"]
pub type CORE1_CONF = crate::Reg<core1_conf::CORE1_CONF_SPEC>;
#[doc = "USB_OTG configuration register"]
pub mod core1_conf;
#[doc = "I2C1_CONF (rw) register accessor: I2C configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_conf`] module"]
pub type I2C1_CONF = crate::Reg<i2c1_conf::I2C1_CONF_SPEC>;
#[doc = "I2C configuration register"]
pub mod i2c1_conf;
#[doc = "I2C1_SCLK_CONF (rw) register accessor: I2C_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sclk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sclk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_sclk_conf`] module"]
pub type I2C1_SCLK_CONF = crate::Reg<i2c1_sclk_conf::I2C1_SCLK_CONF_SPEC>;
#[doc = "I2C_SCLK configuration register"]
pub mod i2c1_sclk_conf;
#[doc = "PWM1_CONF (rw) register accessor: PWM1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_conf`] module"]
pub type PWM1_CONF = crate::Reg<pwm1_conf::PWM1_CONF_SPEC>;
#[doc = "PWM1 configuration register"]
pub mod pwm1_conf;
#[doc = "PWM1_CLK_CONF (rw) register accessor: PWM1_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_clk_conf`] module"]
pub type PWM1_CLK_CONF = crate::Reg<pwm1_clk_conf::PWM1_CLK_CONF_SPEC>;
#[doc = "PWM1_CLK configuration register"]
pub mod pwm1_clk_conf;
#[doc = "SPI3_CONF (rw) register accessor: SPI3 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi3_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi3_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi3_conf`] module"]
pub type SPI3_CONF = crate::Reg<spi3_conf::SPI3_CONF_SPEC>;
#[doc = "SPI3 configuration register"]
pub mod spi3_conf;
#[doc = "SPI3_CLKM_CONF (rw) register accessor: SPI3_CLKM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi3_clkm_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi3_clkm_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi3_clkm_conf`] module"]
pub type SPI3_CLKM_CONF = crate::Reg<spi3_clkm_conf::SPI3_CLKM_CONF_SPEC>;
#[doc = "SPI3_CLKM configuration register"]
pub mod spi3_clkm_conf;
#[doc = "ASRC_FUNC_CLK_CONF (rw) register accessor: AUDIO_SAMPLE_RATE_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_func_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_func_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asrc_func_clk_conf`] module"]
pub type ASRC_FUNC_CLK_CONF = crate::Reg<asrc_func_clk_conf::ASRC_FUNC_CLK_CONF_SPEC>;
#[doc = "AUDIO_SAMPLE_RATE_FUNC_CLK configuration register"]
pub mod asrc_func_clk_conf;
#[doc = "BUS_CLOCK_GATE_BYPASS (rw) register accessor: bus clock gating bypass configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clock_gate_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clock_gate_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clock_gate_bypass`] module"]
pub type BUS_CLOCK_GATE_BYPASS = crate::Reg<bus_clock_gate_bypass::BUS_CLOCK_GATE_BYPASS_SPEC>;
#[doc = "bus clock gating bypass configuration register"]
pub mod bus_clock_gate_bypass;
#[doc = "ZERO_DET_CONF (rw) register accessor: ZERO_DET configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det_conf`] module"]
pub type ZERO_DET_CONF = crate::Reg<zero_det_conf::ZERO_DET_CONF_SPEC>;
#[doc = "ZERO_DET configuration register"]
pub mod zero_det_conf;
#[doc = "ZERO_DET_CLK_CONF (rw) register accessor: ZERO_DET_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det_clk_conf`] module"]
pub type ZERO_DET_CLK_CONF = crate::Reg<zero_det_clk_conf::ZERO_DET_CLK_CONF_SPEC>;
#[doc = "ZERO_DET_CLK configuration register"]
pub mod zero_det_clk_conf;
#[doc = "RST_EN (rw) register accessor: PCR clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_en`] module"]
pub type RST_EN = crate::Reg<rst_en::RST_EN_SPEC>;
#[doc = "PCR clock gating configure register"]
pub mod rst_en;
#[doc = "FPGA_DEBUG (rw) register accessor: fpga debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpga_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpga_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpga_debug`] module"]
pub type FPGA_DEBUG = crate::Reg<fpga_debug::FPGA_DEBUG_SPEC>;
#[doc = "fpga debug register"]
pub mod fpga_debug;
#[doc = "CLOCK_GATE (rw) register accessor: PCR clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "PCR clock gating configure register"]
pub mod clock_gate;
pub use crate::dma::{date, DATE};
