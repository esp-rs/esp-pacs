#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - UART0 configuration register"]
    pub uart0_conf: UART0_CONF,
    #[doc = "0x04 - UART0_SCLK configuration register"]
    pub uart0_sclk_conf: UART0_SCLK_CONF,
    #[doc = "0x08 - UART0 power control register"]
    pub uart0_pd_ctrl: UART0_PD_CTRL,
    #[doc = "0x0c - UART1 configuration register"]
    pub uart1_conf: UART1_CONF,
    #[doc = "0x10 - UART1_SCLK configuration register"]
    pub uart1_sclk_conf: UART1_SCLK_CONF,
    #[doc = "0x14 - UART1 power control register"]
    pub uart1_pd_ctrl: UART1_PD_CTRL,
    #[doc = "0x18 - MSPI configuration register"]
    pub mspi_conf: MSPI_CONF,
    #[doc = "0x1c - MSPI_CLK configuration register"]
    pub mspi_clk_conf: MSPI_CLK_CONF,
    #[doc = "0x20 - I2C configuration register"]
    pub i2c0_conf: I2C0_CONF,
    #[doc = "0x24 - I2C_SCLK configuration register"]
    pub i2c0_sclk_conf: I2C0_SCLK_CONF,
    #[doc = "0x28 - I2C configuration register"]
    pub i2c1_conf: I2C1_CONF,
    #[doc = "0x2c - I2C_SCLK configuration register"]
    pub i2c1_sclk_conf: I2C1_SCLK_CONF,
    #[doc = "0x30 - UHCI configuration register"]
    pub uhci_conf: UHCI_CONF,
    #[doc = "0x34 - RMT configuration register"]
    pub rmt_conf: RMT_CONF,
    #[doc = "0x38 - RMT_SCLK configuration register"]
    pub rmt_sclk_conf: RMT_SCLK_CONF,
    #[doc = "0x3c - LEDC configuration register"]
    pub ledc_conf: LEDC_CONF,
    #[doc = "0x40 - LEDC_SCLK configuration register"]
    pub ledc_sclk_conf: LEDC_SCLK_CONF,
    #[doc = "0x44 - TIMERGROUP0 configuration register"]
    pub timergroup0_conf: TIMERGROUP0_CONF,
    #[doc = "0x48 - TIMERGROUP0_TIMER_CLK configuration register"]
    pub timergroup0_timer_clk_conf: TIMERGROUP0_TIMER_CLK_CONF,
    #[doc = "0x4c - TIMERGROUP0_WDT_CLK configuration register"]
    pub timergroup0_wdt_clk_conf: TIMERGROUP0_WDT_CLK_CONF,
    #[doc = "0x50 - TIMERGROUP1 configuration register"]
    pub timergroup1_conf: TIMERGROUP1_CONF,
    #[doc = "0x54 - TIMERGROUP1_TIMER_CLK configuration register"]
    pub timergroup1_timer_clk_conf: TIMERGROUP1_TIMER_CLK_CONF,
    #[doc = "0x58 - TIMERGROUP1_WDT_CLK configuration register"]
    pub timergroup1_wdt_clk_conf: TIMERGROUP1_WDT_CLK_CONF,
    #[doc = "0x5c - SYSTIMER configuration register"]
    pub systimer_conf: SYSTIMER_CONF,
    #[doc = "0x60 - SYSTIMER_FUNC_CLK configuration register"]
    pub systimer_func_clk_conf: SYSTIMER_FUNC_CLK_CONF,
    #[doc = "0x64 - TWAI0 configuration register"]
    pub twai0_conf: TWAI0_CONF,
    #[doc = "0x68 - TWAI0_FUNC_CLK configuration register"]
    pub twai0_func_clk_conf: TWAI0_FUNC_CLK_CONF,
    #[doc = "0x6c - I2S configuration register"]
    pub i2s_conf: I2S_CONF,
    #[doc = "0x70 - I2S_TX_CLKM configuration register"]
    pub i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    #[doc = "0x74 - I2S_TX_CLKM_DIV configuration register"]
    pub i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    #[doc = "0x78 - I2S_RX_CLKM configuration register"]
    pub i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    #[doc = "0x7c - I2S_RX_CLKM_DIV configuration register"]
    pub i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    #[doc = "0x80 - SARADC configuration register"]
    pub saradc_conf: SARADC_CONF,
    #[doc = "0x84 - SARADC_CLKM configuration register"]
    pub saradc_clkm_conf: SARADC_CLKM_CONF,
    #[doc = "0x88 - TSENS_CLK configuration register"]
    pub tsens_clk_conf: TSENS_CLK_CONF,
    #[doc = "0x8c - USB_DEVICE configuration register"]
    pub usb_device_conf: USB_DEVICE_CONF,
    #[doc = "0x90 - INTMTX configuration register"]
    pub intmtx_conf: INTMTX_CONF,
    #[doc = "0x94 - PCNT configuration register"]
    pub pcnt_conf: PCNT_CONF,
    #[doc = "0x98 - ETM configuration register"]
    pub etm_conf: ETM_CONF,
    #[doc = "0x9c - PWM configuration register"]
    pub pwm_conf: PWM_CONF,
    #[doc = "0xa0 - PWM_CLK configuration register"]
    pub pwm_clk_conf: PWM_CLK_CONF,
    #[doc = "0xa4 - PARL_IO configuration register"]
    pub parl_io_conf: PARL_IO_CONF,
    #[doc = "0xa8 - PARL_CLK_RX configuration register"]
    pub parl_clk_rx_conf: PARL_CLK_RX_CONF,
    #[doc = "0xac - PARL_CLK_TX configuration register"]
    pub parl_clk_tx_conf: PARL_CLK_TX_CONF,
    #[doc = "0xb0 - PVT_MONITOR configuration register"]
    pub pvt_monitor_conf: PVT_MONITOR_CONF,
    #[doc = "0xb4 - PVT_MONITOR function clock configuration register"]
    pub pvt_monitor_func_clk_conf: PVT_MONITOR_FUNC_CLK_CONF,
    #[doc = "0xb8 - GDMA configuration register"]
    pub gdma_conf: GDMA_CONF,
    #[doc = "0xbc - SPI2 configuration register"]
    pub spi2_conf: SPI2_CONF,
    #[doc = "0xc0 - SPI2_CLKM configuration register"]
    pub spi2_clkm_conf: SPI2_CLKM_CONF,
    #[doc = "0xc4 - AES configuration register"]
    pub aes_conf: AES_CONF,
    #[doc = "0xc8 - SHA configuration register"]
    pub sha_conf: SHA_CONF,
    #[doc = "0xcc - RSA configuration register"]
    pub rsa_conf: RSA_CONF,
    #[doc = "0xd0 - RSA power control register"]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0xd4 - ECC configuration register"]
    pub ecc_conf: ECC_CONF,
    #[doc = "0xd8 - ECC power control register"]
    pub ecc_pd_ctrl: ECC_PD_CTRL,
    #[doc = "0xdc - DS configuration register"]
    pub ds_conf: DS_CONF,
    #[doc = "0xe0 - HMAC configuration register"]
    pub hmac_conf: HMAC_CONF,
    #[doc = "0xe4 - ECDSA configuration register"]
    pub ecdsa_conf: ECDSA_CONF,
    #[doc = "0xe8 - IOMUX configuration register"]
    pub iomux_conf: IOMUX_CONF,
    #[doc = "0xec - IOMUX_CLK configuration register"]
    pub iomux_clk_conf: IOMUX_CLK_CONF,
    #[doc = "0xf0 - MEM_MONITOR configuration register"]
    pub mem_monitor_conf: MEM_MONITOR_CONF,
    #[doc = "0xf4 - REGDMA configuration register"]
    pub regdma_conf: REGDMA_CONF,
    #[doc = "0xf8 - TRACE configuration register"]
    pub trace_conf: TRACE_CONF,
    #[doc = "0xfc - ASSIST configuration register"]
    pub assist_conf: ASSIST_CONF,
    #[doc = "0x100 - CACHE configuration register"]
    pub cache_conf: CACHE_CONF,
    #[doc = "0x104 - MODEM_APB configuration register"]
    pub modem_conf: MODEM_CONF,
    #[doc = "0x108 - TIMEOUT configuration register"]
    pub timeout_conf: TIMEOUT_CONF,
    #[doc = "0x10c - SYSCLK configuration register"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x110 - CPU_WAITI configuration register"]
    pub cpu_waiti_conf: CPU_WAITI_CONF,
    #[doc = "0x114 - CPU_FREQ configuration register"]
    pub cpu_freq_conf: CPU_FREQ_CONF,
    #[doc = "0x118 - AHB_FREQ configuration register"]
    pub ahb_freq_conf: AHB_FREQ_CONF,
    #[doc = "0x11c - APB_FREQ configuration register"]
    pub apb_freq_conf: APB_FREQ_CONF,
    #[doc = "0x120 - SYSCLK frequency query 0 register"]
    pub sysclk_freq_query_0: SYSCLK_FREQ_QUERY_0,
    #[doc = "0x124 - SPLL DIV clock-gating configuration register"]
    pub pll_div_clk_en: PLL_DIV_CLK_EN,
    #[doc = "0x128 - CLK_OUT_EN configuration register"]
    pub ctrl_clk_out_en: CTRL_CLK_OUT_EN,
    #[doc = "0x12c - TICK configuration register"]
    pub ctrl_tick_conf: CTRL_TICK_CONF,
    #[doc = "0x130 - 32KHz clock configuration register"]
    pub ctrl_32k_conf: CTRL_32K_CONF,
    #[doc = "0x134 - HP SRAM/ROM configuration register"]
    pub sram_power_conf_0: SRAM_POWER_CONF_0,
    #[doc = "0x138 - HP SRAM/ROM configuration register"]
    pub sram_power_conf_1: SRAM_POWER_CONF_1,
    #[doc = "0x13c - xxxx"]
    pub sec_conf: SEC_CONF,
    #[doc = "0x140 - xxxx"]
    pub adc_inv_phase_conf: ADC_INV_PHASE_CONF,
    #[doc = "0x144 - xxxx"]
    pub sdm_inv_phase_conf: SDM_INV_PHASE_CONF,
    #[doc = "0x148 - xxxx"]
    pub bus_clk_update: BUS_CLK_UPDATE,
    #[doc = "0x14c - xxxx"]
    pub sar_clk_div: SAR_CLK_DIV,
    #[doc = "0x150 - xxxx"]
    pub pwdet_sar_clk_conf: PWDET_SAR_CLK_CONF,
    _reserved85: [u8; 0x0e9c],
    #[doc = "0xff0 - reset event bypass backdoor configuration register"]
    pub reset_event_bypass: RESET_EVENT_BYPASS,
    #[doc = "0xff4 - fpga debug register"]
    pub fpga_debug: FPGA_DEBUG,
    #[doc = "0xff8 - PCR clock gating configure register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0xffc - Date register."]
    pub date: DATE,
}
#[doc = "UART0_CONF (rw) register accessor: an alias for `Reg<UART0_CONF_SPEC>`"]
pub type UART0_CONF = crate::Reg<uart0_conf::UART0_CONF_SPEC>;
#[doc = "UART0 configuration register"]
pub mod uart0_conf;
#[doc = "UART0_SCLK_CONF (rw) register accessor: an alias for `Reg<UART0_SCLK_CONF_SPEC>`"]
pub type UART0_SCLK_CONF = crate::Reg<uart0_sclk_conf::UART0_SCLK_CONF_SPEC>;
#[doc = "UART0_SCLK configuration register"]
pub mod uart0_sclk_conf;
#[doc = "UART0_PD_CTRL (rw) register accessor: an alias for `Reg<UART0_PD_CTRL_SPEC>`"]
pub type UART0_PD_CTRL = crate::Reg<uart0_pd_ctrl::UART0_PD_CTRL_SPEC>;
#[doc = "UART0 power control register"]
pub mod uart0_pd_ctrl;
#[doc = "UART1_CONF (rw) register accessor: an alias for `Reg<UART1_CONF_SPEC>`"]
pub type UART1_CONF = crate::Reg<uart1_conf::UART1_CONF_SPEC>;
#[doc = "UART1 configuration register"]
pub mod uart1_conf;
#[doc = "UART1_SCLK_CONF (rw) register accessor: an alias for `Reg<UART1_SCLK_CONF_SPEC>`"]
pub type UART1_SCLK_CONF = crate::Reg<uart1_sclk_conf::UART1_SCLK_CONF_SPEC>;
#[doc = "UART1_SCLK configuration register"]
pub mod uart1_sclk_conf;
#[doc = "UART1_PD_CTRL (rw) register accessor: an alias for `Reg<UART1_PD_CTRL_SPEC>`"]
pub type UART1_PD_CTRL = crate::Reg<uart1_pd_ctrl::UART1_PD_CTRL_SPEC>;
#[doc = "UART1 power control register"]
pub mod uart1_pd_ctrl;
#[doc = "MSPI_CONF (rw) register accessor: an alias for `Reg<MSPI_CONF_SPEC>`"]
pub type MSPI_CONF = crate::Reg<mspi_conf::MSPI_CONF_SPEC>;
#[doc = "MSPI configuration register"]
pub mod mspi_conf;
#[doc = "MSPI_CLK_CONF (rw) register accessor: an alias for `Reg<MSPI_CLK_CONF_SPEC>`"]
pub type MSPI_CLK_CONF = crate::Reg<mspi_clk_conf::MSPI_CLK_CONF_SPEC>;
#[doc = "MSPI_CLK configuration register"]
pub mod mspi_clk_conf;
#[doc = "I2C0_CONF (rw) register accessor: an alias for `Reg<I2C0_CONF_SPEC>`"]
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
#[doc = "I2C configuration register"]
pub mod i2c0_conf;
#[doc = "I2C0_SCLK_CONF (rw) register accessor: an alias for `Reg<I2C0_SCLK_CONF_SPEC>`"]
pub type I2C0_SCLK_CONF = crate::Reg<i2c0_sclk_conf::I2C0_SCLK_CONF_SPEC>;
#[doc = "I2C_SCLK configuration register"]
pub mod i2c0_sclk_conf;
#[doc = "I2C1_CONF (rw) register accessor: an alias for `Reg<I2C1_CONF_SPEC>`"]
pub type I2C1_CONF = crate::Reg<i2c1_conf::I2C1_CONF_SPEC>;
#[doc = "I2C configuration register"]
pub mod i2c1_conf;
#[doc = "I2C1_SCLK_CONF (rw) register accessor: an alias for `Reg<I2C1_SCLK_CONF_SPEC>`"]
pub type I2C1_SCLK_CONF = crate::Reg<i2c1_sclk_conf::I2C1_SCLK_CONF_SPEC>;
#[doc = "I2C_SCLK configuration register"]
pub mod i2c1_sclk_conf;
#[doc = "UHCI_CONF (rw) register accessor: an alias for `Reg<UHCI_CONF_SPEC>`"]
pub type UHCI_CONF = crate::Reg<uhci_conf::UHCI_CONF_SPEC>;
#[doc = "UHCI configuration register"]
pub mod uhci_conf;
#[doc = "RMT_CONF (rw) register accessor: an alias for `Reg<RMT_CONF_SPEC>`"]
pub type RMT_CONF = crate::Reg<rmt_conf::RMT_CONF_SPEC>;
#[doc = "RMT configuration register"]
pub mod rmt_conf;
#[doc = "RMT_SCLK_CONF (rw) register accessor: an alias for `Reg<RMT_SCLK_CONF_SPEC>`"]
pub type RMT_SCLK_CONF = crate::Reg<rmt_sclk_conf::RMT_SCLK_CONF_SPEC>;
#[doc = "RMT_SCLK configuration register"]
pub mod rmt_sclk_conf;
#[doc = "LEDC_CONF (rw) register accessor: an alias for `Reg<LEDC_CONF_SPEC>`"]
pub type LEDC_CONF = crate::Reg<ledc_conf::LEDC_CONF_SPEC>;
#[doc = "LEDC configuration register"]
pub mod ledc_conf;
#[doc = "LEDC_SCLK_CONF (rw) register accessor: an alias for `Reg<LEDC_SCLK_CONF_SPEC>`"]
pub type LEDC_SCLK_CONF = crate::Reg<ledc_sclk_conf::LEDC_SCLK_CONF_SPEC>;
#[doc = "LEDC_SCLK configuration register"]
pub mod ledc_sclk_conf;
#[doc = "TIMERGROUP0_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP0_CONF_SPEC>`"]
pub type TIMERGROUP0_CONF = crate::Reg<timergroup0_conf::TIMERGROUP0_CONF_SPEC>;
#[doc = "TIMERGROUP0 configuration register"]
pub mod timergroup0_conf;
#[doc = "TIMERGROUP0_TIMER_CLK_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP0_TIMER_CLK_CONF_SPEC>`"]
pub type TIMERGROUP0_TIMER_CLK_CONF =
    crate::Reg<timergroup0_timer_clk_conf::TIMERGROUP0_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_TIMER_CLK configuration register"]
pub mod timergroup0_timer_clk_conf;
#[doc = "TIMERGROUP0_WDT_CLK_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP0_WDT_CLK_CONF_SPEC>`"]
pub type TIMERGROUP0_WDT_CLK_CONF =
    crate::Reg<timergroup0_wdt_clk_conf::TIMERGROUP0_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP0_WDT_CLK configuration register"]
pub mod timergroup0_wdt_clk_conf;
#[doc = "TIMERGROUP1_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP1_CONF_SPEC>`"]
pub type TIMERGROUP1_CONF = crate::Reg<timergroup1_conf::TIMERGROUP1_CONF_SPEC>;
#[doc = "TIMERGROUP1 configuration register"]
pub mod timergroup1_conf;
#[doc = "TIMERGROUP1_TIMER_CLK_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP1_TIMER_CLK_CONF_SPEC>`"]
pub type TIMERGROUP1_TIMER_CLK_CONF =
    crate::Reg<timergroup1_timer_clk_conf::TIMERGROUP1_TIMER_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_TIMER_CLK configuration register"]
pub mod timergroup1_timer_clk_conf;
#[doc = "TIMERGROUP1_WDT_CLK_CONF (rw) register accessor: an alias for `Reg<TIMERGROUP1_WDT_CLK_CONF_SPEC>`"]
pub type TIMERGROUP1_WDT_CLK_CONF =
    crate::Reg<timergroup1_wdt_clk_conf::TIMERGROUP1_WDT_CLK_CONF_SPEC>;
#[doc = "TIMERGROUP1_WDT_CLK configuration register"]
pub mod timergroup1_wdt_clk_conf;
#[doc = "SYSTIMER_CONF (rw) register accessor: an alias for `Reg<SYSTIMER_CONF_SPEC>`"]
pub type SYSTIMER_CONF = crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>;
#[doc = "SYSTIMER configuration register"]
pub mod systimer_conf;
#[doc = "SYSTIMER_FUNC_CLK_CONF (rw) register accessor: an alias for `Reg<SYSTIMER_FUNC_CLK_CONF_SPEC>`"]
pub type SYSTIMER_FUNC_CLK_CONF = crate::Reg<systimer_func_clk_conf::SYSTIMER_FUNC_CLK_CONF_SPEC>;
#[doc = "SYSTIMER_FUNC_CLK configuration register"]
pub mod systimer_func_clk_conf;
#[doc = "TWAI0_CONF (rw) register accessor: an alias for `Reg<TWAI0_CONF_SPEC>`"]
pub type TWAI0_CONF = crate::Reg<twai0_conf::TWAI0_CONF_SPEC>;
#[doc = "TWAI0 configuration register"]
pub mod twai0_conf;
#[doc = "TWAI0_FUNC_CLK_CONF (rw) register accessor: an alias for `Reg<TWAI0_FUNC_CLK_CONF_SPEC>`"]
pub type TWAI0_FUNC_CLK_CONF = crate::Reg<twai0_func_clk_conf::TWAI0_FUNC_CLK_CONF_SPEC>;
#[doc = "TWAI0_FUNC_CLK configuration register"]
pub mod twai0_func_clk_conf;
#[doc = "I2S_CONF (rw) register accessor: an alias for `Reg<I2S_CONF_SPEC>`"]
pub type I2S_CONF = crate::Reg<i2s_conf::I2S_CONF_SPEC>;
#[doc = "I2S configuration register"]
pub mod i2s_conf;
#[doc = "I2S_TX_CLKM_CONF (rw) register accessor: an alias for `Reg<I2S_TX_CLKM_CONF_SPEC>`"]
pub type I2S_TX_CLKM_CONF = crate::Reg<i2s_tx_clkm_conf::I2S_TX_CLKM_CONF_SPEC>;
#[doc = "I2S_TX_CLKM configuration register"]
pub mod i2s_tx_clkm_conf;
#[doc = "I2S_TX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<I2S_TX_CLKM_DIV_CONF_SPEC>`"]
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<i2s_tx_clkm_div_conf::I2S_TX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_TX_CLKM_DIV configuration register"]
pub mod i2s_tx_clkm_div_conf;
#[doc = "I2S_RX_CLKM_CONF (rw) register accessor: an alias for `Reg<I2S_RX_CLKM_CONF_SPEC>`"]
pub type I2S_RX_CLKM_CONF = crate::Reg<i2s_rx_clkm_conf::I2S_RX_CLKM_CONF_SPEC>;
#[doc = "I2S_RX_CLKM configuration register"]
pub mod i2s_rx_clkm_conf;
#[doc = "I2S_RX_CLKM_DIV_CONF (rw) register accessor: an alias for `Reg<I2S_RX_CLKM_DIV_CONF_SPEC>`"]
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<i2s_rx_clkm_div_conf::I2S_RX_CLKM_DIV_CONF_SPEC>;
#[doc = "I2S_RX_CLKM_DIV configuration register"]
pub mod i2s_rx_clkm_div_conf;
#[doc = "SARADC_CONF (rw) register accessor: an alias for `Reg<SARADC_CONF_SPEC>`"]
pub type SARADC_CONF = crate::Reg<saradc_conf::SARADC_CONF_SPEC>;
#[doc = "SARADC configuration register"]
pub mod saradc_conf;
#[doc = "SARADC_CLKM_CONF (rw) register accessor: an alias for `Reg<SARADC_CLKM_CONF_SPEC>`"]
pub type SARADC_CLKM_CONF = crate::Reg<saradc_clkm_conf::SARADC_CLKM_CONF_SPEC>;
#[doc = "SARADC_CLKM configuration register"]
pub mod saradc_clkm_conf;
#[doc = "TSENS_CLK_CONF (rw) register accessor: an alias for `Reg<TSENS_CLK_CONF_SPEC>`"]
pub type TSENS_CLK_CONF = crate::Reg<tsens_clk_conf::TSENS_CLK_CONF_SPEC>;
#[doc = "TSENS_CLK configuration register"]
pub mod tsens_clk_conf;
#[doc = "USB_DEVICE_CONF (rw) register accessor: an alias for `Reg<USB_DEVICE_CONF_SPEC>`"]
pub type USB_DEVICE_CONF = crate::Reg<usb_device_conf::USB_DEVICE_CONF_SPEC>;
#[doc = "USB_DEVICE configuration register"]
pub mod usb_device_conf;
#[doc = "INTMTX_CONF (rw) register accessor: an alias for `Reg<INTMTX_CONF_SPEC>`"]
pub type INTMTX_CONF = crate::Reg<intmtx_conf::INTMTX_CONF_SPEC>;
#[doc = "INTMTX configuration register"]
pub mod intmtx_conf;
#[doc = "PCNT_CONF (rw) register accessor: an alias for `Reg<PCNT_CONF_SPEC>`"]
pub type PCNT_CONF = crate::Reg<pcnt_conf::PCNT_CONF_SPEC>;
#[doc = "PCNT configuration register"]
pub mod pcnt_conf;
#[doc = "ETM_CONF (rw) register accessor: an alias for `Reg<ETM_CONF_SPEC>`"]
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
#[doc = "ETM configuration register"]
pub mod etm_conf;
#[doc = "PWM_CONF (rw) register accessor: an alias for `Reg<PWM_CONF_SPEC>`"]
pub type PWM_CONF = crate::Reg<pwm_conf::PWM_CONF_SPEC>;
#[doc = "PWM configuration register"]
pub mod pwm_conf;
#[doc = "PWM_CLK_CONF (rw) register accessor: an alias for `Reg<PWM_CLK_CONF_SPEC>`"]
pub type PWM_CLK_CONF = crate::Reg<pwm_clk_conf::PWM_CLK_CONF_SPEC>;
#[doc = "PWM_CLK configuration register"]
pub mod pwm_clk_conf;
#[doc = "PARL_IO_CONF (rw) register accessor: an alias for `Reg<PARL_IO_CONF_SPEC>`"]
pub type PARL_IO_CONF = crate::Reg<parl_io_conf::PARL_IO_CONF_SPEC>;
#[doc = "PARL_IO configuration register"]
pub mod parl_io_conf;
#[doc = "PARL_CLK_RX_CONF (rw) register accessor: an alias for `Reg<PARL_CLK_RX_CONF_SPEC>`"]
pub type PARL_CLK_RX_CONF = crate::Reg<parl_clk_rx_conf::PARL_CLK_RX_CONF_SPEC>;
#[doc = "PARL_CLK_RX configuration register"]
pub mod parl_clk_rx_conf;
#[doc = "PARL_CLK_TX_CONF (rw) register accessor: an alias for `Reg<PARL_CLK_TX_CONF_SPEC>`"]
pub type PARL_CLK_TX_CONF = crate::Reg<parl_clk_tx_conf::PARL_CLK_TX_CONF_SPEC>;
#[doc = "PARL_CLK_TX configuration register"]
pub mod parl_clk_tx_conf;
#[doc = "PVT_MONITOR_CONF (rw) register accessor: an alias for `Reg<PVT_MONITOR_CONF_SPEC>`"]
pub type PVT_MONITOR_CONF = crate::Reg<pvt_monitor_conf::PVT_MONITOR_CONF_SPEC>;
#[doc = "PVT_MONITOR configuration register"]
pub mod pvt_monitor_conf;
#[doc = "PVT_MONITOR_FUNC_CLK_CONF (rw) register accessor: an alias for `Reg<PVT_MONITOR_FUNC_CLK_CONF_SPEC>`"]
pub type PVT_MONITOR_FUNC_CLK_CONF =
    crate::Reg<pvt_monitor_func_clk_conf::PVT_MONITOR_FUNC_CLK_CONF_SPEC>;
#[doc = "PVT_MONITOR function clock configuration register"]
pub mod pvt_monitor_func_clk_conf;
#[doc = "GDMA_CONF (rw) register accessor: an alias for `Reg<GDMA_CONF_SPEC>`"]
pub type GDMA_CONF = crate::Reg<gdma_conf::GDMA_CONF_SPEC>;
#[doc = "GDMA configuration register"]
pub mod gdma_conf;
#[doc = "SPI2_CONF (rw) register accessor: an alias for `Reg<SPI2_CONF_SPEC>`"]
pub type SPI2_CONF = crate::Reg<spi2_conf::SPI2_CONF_SPEC>;
#[doc = "SPI2 configuration register"]
pub mod spi2_conf;
#[doc = "SPI2_CLKM_CONF (rw) register accessor: an alias for `Reg<SPI2_CLKM_CONF_SPEC>`"]
pub type SPI2_CLKM_CONF = crate::Reg<spi2_clkm_conf::SPI2_CLKM_CONF_SPEC>;
#[doc = "SPI2_CLKM configuration register"]
pub mod spi2_clkm_conf;
#[doc = "AES_CONF (rw) register accessor: an alias for `Reg<AES_CONF_SPEC>`"]
pub type AES_CONF = crate::Reg<aes_conf::AES_CONF_SPEC>;
#[doc = "AES configuration register"]
pub mod aes_conf;
#[doc = "SHA_CONF (rw) register accessor: an alias for `Reg<SHA_CONF_SPEC>`"]
pub type SHA_CONF = crate::Reg<sha_conf::SHA_CONF_SPEC>;
#[doc = "SHA configuration register"]
pub mod sha_conf;
#[doc = "RSA_CONF (rw) register accessor: an alias for `Reg<RSA_CONF_SPEC>`"]
pub type RSA_CONF = crate::Reg<rsa_conf::RSA_CONF_SPEC>;
#[doc = "RSA configuration register"]
pub mod rsa_conf;
#[doc = "RSA_PD_CTRL (rw) register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "RSA power control register"]
pub mod rsa_pd_ctrl;
#[doc = "ECC_CONF (rw) register accessor: an alias for `Reg<ECC_CONF_SPEC>`"]
pub type ECC_CONF = crate::Reg<ecc_conf::ECC_CONF_SPEC>;
#[doc = "ECC configuration register"]
pub mod ecc_conf;
#[doc = "ECC_PD_CTRL (rw) register accessor: an alias for `Reg<ECC_PD_CTRL_SPEC>`"]
pub type ECC_PD_CTRL = crate::Reg<ecc_pd_ctrl::ECC_PD_CTRL_SPEC>;
#[doc = "ECC power control register"]
pub mod ecc_pd_ctrl;
#[doc = "DS_CONF (rw) register accessor: an alias for `Reg<DS_CONF_SPEC>`"]
pub type DS_CONF = crate::Reg<ds_conf::DS_CONF_SPEC>;
#[doc = "DS configuration register"]
pub mod ds_conf;
#[doc = "HMAC_CONF (rw) register accessor: an alias for `Reg<HMAC_CONF_SPEC>`"]
pub type HMAC_CONF = crate::Reg<hmac_conf::HMAC_CONF_SPEC>;
#[doc = "HMAC configuration register"]
pub mod hmac_conf;
#[doc = "ECDSA_CONF (rw) register accessor: an alias for `Reg<ECDSA_CONF_SPEC>`"]
pub type ECDSA_CONF = crate::Reg<ecdsa_conf::ECDSA_CONF_SPEC>;
#[doc = "ECDSA configuration register"]
pub mod ecdsa_conf;
#[doc = "IOMUX_CONF (rw) register accessor: an alias for `Reg<IOMUX_CONF_SPEC>`"]
pub type IOMUX_CONF = crate::Reg<iomux_conf::IOMUX_CONF_SPEC>;
#[doc = "IOMUX configuration register"]
pub mod iomux_conf;
#[doc = "IOMUX_CLK_CONF (rw) register accessor: an alias for `Reg<IOMUX_CLK_CONF_SPEC>`"]
pub type IOMUX_CLK_CONF = crate::Reg<iomux_clk_conf::IOMUX_CLK_CONF_SPEC>;
#[doc = "IOMUX_CLK configuration register"]
pub mod iomux_clk_conf;
#[doc = "MEM_MONITOR_CONF (rw) register accessor: an alias for `Reg<MEM_MONITOR_CONF_SPEC>`"]
pub type MEM_MONITOR_CONF = crate::Reg<mem_monitor_conf::MEM_MONITOR_CONF_SPEC>;
#[doc = "MEM_MONITOR configuration register"]
pub mod mem_monitor_conf;
#[doc = "REGDMA_CONF (rw) register accessor: an alias for `Reg<REGDMA_CONF_SPEC>`"]
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
#[doc = "REGDMA configuration register"]
pub mod regdma_conf;
#[doc = "TRACE_CONF (rw) register accessor: an alias for `Reg<TRACE_CONF_SPEC>`"]
pub type TRACE_CONF = crate::Reg<trace_conf::TRACE_CONF_SPEC>;
#[doc = "TRACE configuration register"]
pub mod trace_conf;
#[doc = "ASSIST_CONF (rw) register accessor: an alias for `Reg<ASSIST_CONF_SPEC>`"]
pub type ASSIST_CONF = crate::Reg<assist_conf::ASSIST_CONF_SPEC>;
#[doc = "ASSIST configuration register"]
pub mod assist_conf;
#[doc = "CACHE_CONF (rw) register accessor: an alias for `Reg<CACHE_CONF_SPEC>`"]
pub type CACHE_CONF = crate::Reg<cache_conf::CACHE_CONF_SPEC>;
#[doc = "CACHE configuration register"]
pub mod cache_conf;
#[doc = "MODEM_CONF (rw) register accessor: an alias for `Reg<MODEM_CONF_SPEC>`"]
pub type MODEM_CONF = crate::Reg<modem_conf::MODEM_CONF_SPEC>;
#[doc = "MODEM_APB configuration register"]
pub mod modem_conf;
#[doc = "TIMEOUT_CONF (rw) register accessor: an alias for `Reg<TIMEOUT_CONF_SPEC>`"]
pub type TIMEOUT_CONF = crate::Reg<timeout_conf::TIMEOUT_CONF_SPEC>;
#[doc = "TIMEOUT configuration register"]
pub mod timeout_conf;
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SYSCLK configuration register"]
pub mod sysclk_conf;
#[doc = "CPU_WAITI_CONF (rw) register accessor: an alias for `Reg<CPU_WAITI_CONF_SPEC>`"]
pub type CPU_WAITI_CONF = crate::Reg<cpu_waiti_conf::CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod cpu_waiti_conf;
#[doc = "CPU_FREQ_CONF (rw) register accessor: an alias for `Reg<CPU_FREQ_CONF_SPEC>`"]
pub type CPU_FREQ_CONF = crate::Reg<cpu_freq_conf::CPU_FREQ_CONF_SPEC>;
#[doc = "CPU_FREQ configuration register"]
pub mod cpu_freq_conf;
#[doc = "AHB_FREQ_CONF (rw) register accessor: an alias for `Reg<AHB_FREQ_CONF_SPEC>`"]
pub type AHB_FREQ_CONF = crate::Reg<ahb_freq_conf::AHB_FREQ_CONF_SPEC>;
#[doc = "AHB_FREQ configuration register"]
pub mod ahb_freq_conf;
#[doc = "APB_FREQ_CONF (rw) register accessor: an alias for `Reg<APB_FREQ_CONF_SPEC>`"]
pub type APB_FREQ_CONF = crate::Reg<apb_freq_conf::APB_FREQ_CONF_SPEC>;
#[doc = "APB_FREQ configuration register"]
pub mod apb_freq_conf;
#[doc = "SYSCLK_FREQ_QUERY_0 (r) register accessor: an alias for `Reg<SYSCLK_FREQ_QUERY_0_SPEC>`"]
pub type SYSCLK_FREQ_QUERY_0 = crate::Reg<sysclk_freq_query_0::SYSCLK_FREQ_QUERY_0_SPEC>;
#[doc = "SYSCLK frequency query 0 register"]
pub mod sysclk_freq_query_0;
#[doc = "PLL_DIV_CLK_EN (rw) register accessor: an alias for `Reg<PLL_DIV_CLK_EN_SPEC>`"]
pub type PLL_DIV_CLK_EN = crate::Reg<pll_div_clk_en::PLL_DIV_CLK_EN_SPEC>;
#[doc = "SPLL DIV clock-gating configuration register"]
pub mod pll_div_clk_en;
#[doc = "CTRL_CLK_OUT_EN (rw) register accessor: an alias for `Reg<CTRL_CLK_OUT_EN_SPEC>`"]
pub type CTRL_CLK_OUT_EN = crate::Reg<ctrl_clk_out_en::CTRL_CLK_OUT_EN_SPEC>;
#[doc = "CLK_OUT_EN configuration register"]
pub mod ctrl_clk_out_en;
#[doc = "CTRL_TICK_CONF (rw) register accessor: an alias for `Reg<CTRL_TICK_CONF_SPEC>`"]
pub type CTRL_TICK_CONF = crate::Reg<ctrl_tick_conf::CTRL_TICK_CONF_SPEC>;
#[doc = "TICK configuration register"]
pub mod ctrl_tick_conf;
#[doc = "CTRL_32K_CONF (rw) register accessor: an alias for `Reg<CTRL_32K_CONF_SPEC>`"]
pub type CTRL_32K_CONF = crate::Reg<ctrl_32k_conf::CTRL_32K_CONF_SPEC>;
#[doc = "32KHz clock configuration register"]
pub mod ctrl_32k_conf;
#[doc = "SRAM_POWER_CONF_0 (rw) register accessor: an alias for `Reg<SRAM_POWER_CONF_0_SPEC>`"]
pub type SRAM_POWER_CONF_0 = crate::Reg<sram_power_conf_0::SRAM_POWER_CONF_0_SPEC>;
#[doc = "HP SRAM/ROM configuration register"]
pub mod sram_power_conf_0;
#[doc = "SRAM_POWER_CONF_1 (rw) register accessor: an alias for `Reg<SRAM_POWER_CONF_1_SPEC>`"]
pub type SRAM_POWER_CONF_1 = crate::Reg<sram_power_conf_1::SRAM_POWER_CONF_1_SPEC>;
#[doc = "HP SRAM/ROM configuration register"]
pub mod sram_power_conf_1;
#[doc = "SEC_CONF (rw) register accessor: an alias for `Reg<SEC_CONF_SPEC>`"]
pub type SEC_CONF = crate::Reg<sec_conf::SEC_CONF_SPEC>;
#[doc = "xxxx"]
pub mod sec_conf;
#[doc = "ADC_INV_PHASE_CONF (rw) register accessor: an alias for `Reg<ADC_INV_PHASE_CONF_SPEC>`"]
pub type ADC_INV_PHASE_CONF = crate::Reg<adc_inv_phase_conf::ADC_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod adc_inv_phase_conf;
#[doc = "SDM_INV_PHASE_CONF (rw) register accessor: an alias for `Reg<SDM_INV_PHASE_CONF_SPEC>`"]
pub type SDM_INV_PHASE_CONF = crate::Reg<sdm_inv_phase_conf::SDM_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod sdm_inv_phase_conf;
#[doc = "BUS_CLK_UPDATE (rw) register accessor: an alias for `Reg<BUS_CLK_UPDATE_SPEC>`"]
pub type BUS_CLK_UPDATE = crate::Reg<bus_clk_update::BUS_CLK_UPDATE_SPEC>;
#[doc = "xxxx"]
pub mod bus_clk_update;
#[doc = "SAR_CLK_DIV (rw) register accessor: an alias for `Reg<SAR_CLK_DIV_SPEC>`"]
pub type SAR_CLK_DIV = crate::Reg<sar_clk_div::SAR_CLK_DIV_SPEC>;
#[doc = "xxxx"]
pub mod sar_clk_div;
#[doc = "PWDET_SAR_CLK_CONF (rw) register accessor: an alias for `Reg<PWDET_SAR_CLK_CONF_SPEC>`"]
pub type PWDET_SAR_CLK_CONF = crate::Reg<pwdet_sar_clk_conf::PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "xxxx"]
pub mod pwdet_sar_clk_conf;
#[doc = "RESET_EVENT_BYPASS (rw) register accessor: an alias for `Reg<RESET_EVENT_BYPASS_SPEC>`"]
pub type RESET_EVENT_BYPASS = crate::Reg<reset_event_bypass::RESET_EVENT_BYPASS_SPEC>;
#[doc = "reset event bypass backdoor configuration register"]
pub mod reset_event_bypass;
#[doc = "FPGA_DEBUG (rw) register accessor: an alias for `Reg<FPGA_DEBUG_SPEC>`"]
pub type FPGA_DEBUG = crate::Reg<fpga_debug::FPGA_DEBUG_SPEC>;
#[doc = "fpga debug register"]
pub mod fpga_debug;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "PCR clock gating configure register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
