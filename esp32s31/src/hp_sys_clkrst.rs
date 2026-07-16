#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    soc_clk_sel: SOC_CLK_SEL,
    cpu_freq_ctrl0: CPU_FREQ_CTRL0,
    mem_freq_ctrl0: MEM_FREQ_CTRL0,
    sys_freq_ctrl0: SYS_FREQ_CTRL0,
    apb_freq_ctrl0: APB_FREQ_CTRL0,
    root_clk_ctrl0: ROOT_CLK_CTRL0,
    cpu_wfi_delay_ctrl0: CPU_WFI_DELAY_CTRL0,
    hpcore0_ctrl0: HPCORE0_CTRL0,
    hpcore1_ctrl0: HPCORE1_CTRL0,
    trace_ctrl0: TRACE_CTRL0,
    tcm_ctrl0: TCM_CTRL0,
    tcmmon_ctrl0: TCMMON_CTRL0,
    psrammon_ctrl0: PSRAMMON_CTRL0,
    rom_ctrl0: ROM_CTRL0,
    cache_ctrl0: CACHE_CTRL0,
    icm_ctrl0: ICM_CTRL0,
    modem_ctrl0: MODEM_CTRL0,
    misc_ctrl0: MISC_CTRL0,
    busmon_ctrl0: BUSMON_CTRL0,
    remove_tmp0: REMOVE_TMP0,
    pvt0_ctrl0: PVT0_CTRL0,
    pvt0_peri_ctrl0: PVT0_PERI_CTRL0,
    crypto_ctrl0: CRYPTO_CTRL0,
    key_manager_ctrl0: KEY_MANAGER_CTRL0,
    dpa_ctrl0: DPA_CTRL0,
    flash_ctrl0: FLASH_CTRL0,
    psram_ctrl0: PSRAM_CTRL0,
    gpspi2_ctrl0: GPSPI2_CTRL0,
    gpspi3_ctrl0: GPSPI3_CTRL0,
    gdma_ctrl0: GDMA_CTRL0,
    axi_pdma_ctrl0: AXI_PDMA_CTRL0,
    ahb_pdma_ctrl0: AHB_PDMA_CTRL0,
    regdma_ctrl0: REGDMA_CTRL0,
    uhci_ctrl0: UHCI_CTRL0,
    uart0_ctrl0: UART0_CTRL0,
    uart1_ctrl0: UART1_CTRL0,
    uart2_ctrl0: UART2_CTRL0,
    uart3_ctrl0: UART3_CTRL0,
    parlio_ctrl0: PARLIO_CTRL0,
    parlio_rx_ctrl0: PARLIO_RX_CTRL0,
    parlio_tx_ctrl0: PARLIO_TX_CTRL0,
    bitsrambler_ctrl0: BITSRAMBLER_CTRL0,
    etm_ctrl0: ETM_CTRL0,
    usb_otghs_ctrl0: USB_OTGHS_CTRL0,
    dma2d_ctrl0: DMA2D_CTRL0,
    ppa_ctrl0: PPA_CTRL0,
    jpeg_ctrl0: JPEG_CTRL0,
    rmt_ctrl0: RMT_CTRL0,
    sdio_host_ctrl0: SDIO_HOST_CTRL0,
    sdio_host_func_ctrl0: SDIO_HOST_FUNC_CTRL0,
    emac_ctrl0: EMAC_CTRL0,
    dsi_ctrl0: DSI_CTRL0,
    dsi_phy_ctrl0: DSI_PHY_CTRL0,
    dsi_dpi_ctrl0: DSI_DPI_CTRL0,
    mspi_pad_ctrl0: MSPI_PAD_CTRL0,
    i2c0_ctrl0: I2C0_CTRL0,
    i2c1_ctrl0: I2C1_CTRL0,
    i2s0_ctrl0: I2S0_CTRL0,
    i2s0_rx_ctrl0: I2S0_RX_CTRL0,
    i2s0_rx_div_ctrl0: I2S0_RX_DIV_CTRL0,
    i2s0_tx_ctrl0: I2S0_TX_CTRL0,
    i2s0_tx_div_ctrl0: I2S0_TX_DIV_CTRL0,
    i2s1_ctrl0: I2S1_CTRL0,
    i2s1_rx_ctrl0: I2S1_RX_CTRL0,
    i2s1_rx_div_ctrl0: I2S1_RX_DIV_CTRL0,
    i2s1_tx_ctrl0: I2S1_TX_CTRL0,
    i2s1_tx_div_ctrl0: I2S1_TX_DIV_CTRL0,
    twai0_ctrl0: TWAI0_CTRL0,
    twai1_ctrl0: TWAI1_CTRL0,
    timergrp0_ctrl0: TIMERGRP0_CTRL0,
    timergrp0_tgrt_ctrl0: TIMERGRP0_TGRT_CTRL0,
    timergrp1_ctrl0: TIMERGRP1_CTRL0,
    systimer_ctrl0: SYSTIMER_CTRL0,
    mcpwm0_ctrl0: MCPWM0_CTRL0,
    mcpwm1_ctrl0: MCPWM1_CTRL0,
    mcpwm2_ctrl0: MCPWM2_CTRL0,
    mcpwm3_ctrl0: MCPWM3_CTRL0,
    intrmtx_ctrl0: INTRMTX_CTRL0,
    pcnt_ctrl0: PCNT_CTRL0,
    _reserved79: [u8; 0x08],
    usb_device_ctrl0: USB_DEVICE_CTRL0,
    ledc_ctrl0: LEDC_CTRL0,
    lcdcam_ctrl0: LCDCAM_CTRL0,
    lcdcam_lcd_ctrl0: LCDCAM_LCD_CTRL0,
    lcdcam_lcdcam_ctrl0: LCDCAM_LCDCAM_CTRL0,
    lcdcam_cam_ctrl0: LCDCAM_CAM_CTRL0,
    iomux_ctrl0: IOMUX_CTRL0,
    hpwdt_core0_rst_ctrl0: HPWDT_CORE0_RST_CTRL0,
    hpwdt_core1_rst_ctrl0: HPWDT_CORE1_RST_CTRL0,
    cpu_src_freq0: CPU_SRC_FREQ0,
    cpu_clk_status0: CPU_CLK_STATUS0,
    hpcore_wdt_reset_source0: HPCORE_WDT_RESET_SOURCE0,
    ana_pll_ctrl0: ANA_PLL_CTRL0,
    ref_500m_ctrl0: REF_500M_CTRL0,
    ref_240m_ctrl0: REF_240M_CTRL0,
    ref_160m_ctrl0: REF_160M_CTRL0,
    ref_120m_ctrl0: REF_120M_CTRL0,
    ref_80m_ctrl0: REF_80M_CTRL0,
    ref_60m_ctrl0: REF_60M_CTRL0,
    ref_20m_ctrl0: REF_20M_CTRL0,
    ref_50m_ctrl0: REF_50M_CTRL0,
    ref_25m_ctrl0: REF_25M_CTRL0,
    tm_clk_ctrl0: TM_CLK_CTRL0,
    dbg0_clk_ctrl0: DBG0_CLK_CTRL0,
    dbg1_clk_ctrl0: DBG1_CLK_CTRL0,
    dbg2_clk_ctrl0: DBG2_CLK_CTRL0,
    lp_clk_ctrl0: LP_CLK_CTRL0,
    ahb_asrc_ctrl0: AHB_ASRC_CTRL0,
    cordic_ctrl0: CORDIC_CTRL0,
    zero_det_ctrl0: ZERO_DET_CTRL0,
    cordic_ctrl1: CORDIC_CTRL1,
    _reserved110: [u8; 0x08],
    clk_pwr_decrease: CLK_PWR_DECREASE,
    cnnt_iomux_ctrl0: CNNT_IOMUX_CTRL0,
    hp_i2cmst_ctrl0: HP_I2CMST_CTRL0,
    axi_perf_mon_ctrl0: AXI_PERF_MON_CTRL0,
    cnnt_sysreg_ctrl0: CNNT_SYSREG_CTRL0,
    hp_alive_sysreg_ctrl0: HP_ALIVE_SYSREG_CTRL0,
    modem_conf: MODEM_CONF,
    adc_dac_inv_phase_conf: ADC_DAC_INV_PHASE_CONF,
    pwdet_sar_clk_conf: PWDET_SAR_CLK_CONF,
    pad_bist_sdio: PAD_BIST_SDIO,
    pad_bist_gmac0: PAD_BIST_GMAC0,
    pad_bist_gmac1: PAD_BIST_GMAC1,
    clk_en0: CLK_EN0,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn soc_clk_sel(&self) -> &SOC_CLK_SEL {
        &self.soc_clk_sel
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn cpu_freq_ctrl0(&self) -> &CPU_FREQ_CTRL0 {
        &self.cpu_freq_ctrl0
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn mem_freq_ctrl0(&self) -> &MEM_FREQ_CTRL0 {
        &self.mem_freq_ctrl0
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn sys_freq_ctrl0(&self) -> &SYS_FREQ_CTRL0 {
        &self.sys_freq_ctrl0
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn apb_freq_ctrl0(&self) -> &APB_FREQ_CTRL0 {
        &self.apb_freq_ctrl0
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn root_clk_ctrl0(&self) -> &ROOT_CLK_CTRL0 {
        &self.root_clk_ctrl0
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn cpu_wfi_delay_ctrl0(&self) -> &CPU_WFI_DELAY_CTRL0 {
        &self.cpu_wfi_delay_ctrl0
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn hpcore0_ctrl0(&self) -> &HPCORE0_CTRL0 {
        &self.hpcore0_ctrl0
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn hpcore1_ctrl0(&self) -> &HPCORE1_CTRL0 {
        &self.hpcore1_ctrl0
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn trace_ctrl0(&self) -> &TRACE_CTRL0 {
        &self.trace_ctrl0
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn tcm_ctrl0(&self) -> &TCM_CTRL0 {
        &self.tcm_ctrl0
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn tcmmon_ctrl0(&self) -> &TCMMON_CTRL0 {
        &self.tcmmon_ctrl0
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn psrammon_ctrl0(&self) -> &PSRAMMON_CTRL0 {
        &self.psrammon_ctrl0
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn rom_ctrl0(&self) -> &ROM_CTRL0 {
        &self.rom_ctrl0
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn cache_ctrl0(&self) -> &CACHE_CTRL0 {
        &self.cache_ctrl0
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn icm_ctrl0(&self) -> &ICM_CTRL0 {
        &self.icm_ctrl0
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn modem_ctrl0(&self) -> &MODEM_CTRL0 {
        &self.modem_ctrl0
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn misc_ctrl0(&self) -> &MISC_CTRL0 {
        &self.misc_ctrl0
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn busmon_ctrl0(&self) -> &BUSMON_CTRL0 {
        &self.busmon_ctrl0
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn remove_tmp0(&self) -> &REMOVE_TMP0 {
        &self.remove_tmp0
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn pvt0_ctrl0(&self) -> &PVT0_CTRL0 {
        &self.pvt0_ctrl0
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn pvt0_peri_ctrl0(&self) -> &PVT0_PERI_CTRL0 {
        &self.pvt0_peri_ctrl0
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn crypto_ctrl0(&self) -> &CRYPTO_CTRL0 {
        &self.crypto_ctrl0
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn key_manager_ctrl0(&self) -> &KEY_MANAGER_CTRL0 {
        &self.key_manager_ctrl0
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn dpa_ctrl0(&self) -> &DPA_CTRL0 {
        &self.dpa_ctrl0
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn flash_ctrl0(&self) -> &FLASH_CTRL0 {
        &self.flash_ctrl0
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn psram_ctrl0(&self) -> &PSRAM_CTRL0 {
        &self.psram_ctrl0
    }
    #[doc = "0x6c - need_des"]
    #[inline(always)]
    pub const fn gpspi2_ctrl0(&self) -> &GPSPI2_CTRL0 {
        &self.gpspi2_ctrl0
    }
    #[doc = "0x70 - need_des"]
    #[inline(always)]
    pub const fn gpspi3_ctrl0(&self) -> &GPSPI3_CTRL0 {
        &self.gpspi3_ctrl0
    }
    #[doc = "0x74 - need_des"]
    #[inline(always)]
    pub const fn gdma_ctrl0(&self) -> &GDMA_CTRL0 {
        &self.gdma_ctrl0
    }
    #[doc = "0x78 - need_des"]
    #[inline(always)]
    pub const fn axi_pdma_ctrl0(&self) -> &AXI_PDMA_CTRL0 {
        &self.axi_pdma_ctrl0
    }
    #[doc = "0x7c - need_des"]
    #[inline(always)]
    pub const fn ahb_pdma_ctrl0(&self) -> &AHB_PDMA_CTRL0 {
        &self.ahb_pdma_ctrl0
    }
    #[doc = "0x80 - need_des"]
    #[inline(always)]
    pub const fn regdma_ctrl0(&self) -> &REGDMA_CTRL0 {
        &self.regdma_ctrl0
    }
    #[doc = "0x84 - need_des"]
    #[inline(always)]
    pub const fn uhci_ctrl0(&self) -> &UHCI_CTRL0 {
        &self.uhci_ctrl0
    }
    #[doc = "0x88 - need_des"]
    #[inline(always)]
    pub const fn uart0_ctrl0(&self) -> &UART0_CTRL0 {
        &self.uart0_ctrl0
    }
    #[doc = "0x8c - need_des"]
    #[inline(always)]
    pub const fn uart1_ctrl0(&self) -> &UART1_CTRL0 {
        &self.uart1_ctrl0
    }
    #[doc = "0x90 - need_des"]
    #[inline(always)]
    pub const fn uart2_ctrl0(&self) -> &UART2_CTRL0 {
        &self.uart2_ctrl0
    }
    #[doc = "0x94 - need_des"]
    #[inline(always)]
    pub const fn uart3_ctrl0(&self) -> &UART3_CTRL0 {
        &self.uart3_ctrl0
    }
    #[doc = "0x98 - need_des"]
    #[inline(always)]
    pub const fn parlio_ctrl0(&self) -> &PARLIO_CTRL0 {
        &self.parlio_ctrl0
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn parlio_rx_ctrl0(&self) -> &PARLIO_RX_CTRL0 {
        &self.parlio_rx_ctrl0
    }
    #[doc = "0xa0 - need_des"]
    #[inline(always)]
    pub const fn parlio_tx_ctrl0(&self) -> &PARLIO_TX_CTRL0 {
        &self.parlio_tx_ctrl0
    }
    #[doc = "0xa4 - need_des"]
    #[inline(always)]
    pub const fn bitsrambler_ctrl0(&self) -> &BITSRAMBLER_CTRL0 {
        &self.bitsrambler_ctrl0
    }
    #[doc = "0xa8 - need_des"]
    #[inline(always)]
    pub const fn etm_ctrl0(&self) -> &ETM_CTRL0 {
        &self.etm_ctrl0
    }
    #[doc = "0xac - need_des"]
    #[inline(always)]
    pub const fn usb_otghs_ctrl0(&self) -> &USB_OTGHS_CTRL0 {
        &self.usb_otghs_ctrl0
    }
    #[doc = "0xb0 - need_des"]
    #[inline(always)]
    pub const fn dma2d_ctrl0(&self) -> &DMA2D_CTRL0 {
        &self.dma2d_ctrl0
    }
    #[doc = "0xb4 - need_des"]
    #[inline(always)]
    pub const fn ppa_ctrl0(&self) -> &PPA_CTRL0 {
        &self.ppa_ctrl0
    }
    #[doc = "0xb8 - need_des"]
    #[inline(always)]
    pub const fn jpeg_ctrl0(&self) -> &JPEG_CTRL0 {
        &self.jpeg_ctrl0
    }
    #[doc = "0xbc - need_des"]
    #[inline(always)]
    pub const fn rmt_ctrl0(&self) -> &RMT_CTRL0 {
        &self.rmt_ctrl0
    }
    #[doc = "0xc0 - need_des"]
    #[inline(always)]
    pub const fn sdio_host_ctrl0(&self) -> &SDIO_HOST_CTRL0 {
        &self.sdio_host_ctrl0
    }
    #[doc = "0xc4 - need_des"]
    #[inline(always)]
    pub const fn sdio_host_func_ctrl0(&self) -> &SDIO_HOST_FUNC_CTRL0 {
        &self.sdio_host_func_ctrl0
    }
    #[doc = "0xc8 - need_des"]
    #[inline(always)]
    pub const fn emac_ctrl0(&self) -> &EMAC_CTRL0 {
        &self.emac_ctrl0
    }
    #[doc = "0xcc - need_des"]
    #[inline(always)]
    pub const fn dsi_ctrl0(&self) -> &DSI_CTRL0 {
        &self.dsi_ctrl0
    }
    #[doc = "0xd0 - need_des"]
    #[inline(always)]
    pub const fn dsi_phy_ctrl0(&self) -> &DSI_PHY_CTRL0 {
        &self.dsi_phy_ctrl0
    }
    #[doc = "0xd4 - need_des"]
    #[inline(always)]
    pub const fn dsi_dpi_ctrl0(&self) -> &DSI_DPI_CTRL0 {
        &self.dsi_dpi_ctrl0
    }
    #[doc = "0xd8 - need_des"]
    #[inline(always)]
    pub const fn mspi_pad_ctrl0(&self) -> &MSPI_PAD_CTRL0 {
        &self.mspi_pad_ctrl0
    }
    #[doc = "0xdc - need_des"]
    #[inline(always)]
    pub const fn i2c0_ctrl0(&self) -> &I2C0_CTRL0 {
        &self.i2c0_ctrl0
    }
    #[doc = "0xe0 - need_des"]
    #[inline(always)]
    pub const fn i2c1_ctrl0(&self) -> &I2C1_CTRL0 {
        &self.i2c1_ctrl0
    }
    #[doc = "0xe4 - need_des"]
    #[inline(always)]
    pub const fn i2s0_ctrl0(&self) -> &I2S0_CTRL0 {
        &self.i2s0_ctrl0
    }
    #[doc = "0xe8 - need_des"]
    #[inline(always)]
    pub const fn i2s0_rx_ctrl0(&self) -> &I2S0_RX_CTRL0 {
        &self.i2s0_rx_ctrl0
    }
    #[doc = "0xec - need_des"]
    #[inline(always)]
    pub const fn i2s0_rx_div_ctrl0(&self) -> &I2S0_RX_DIV_CTRL0 {
        &self.i2s0_rx_div_ctrl0
    }
    #[doc = "0xf0 - need_des"]
    #[inline(always)]
    pub const fn i2s0_tx_ctrl0(&self) -> &I2S0_TX_CTRL0 {
        &self.i2s0_tx_ctrl0
    }
    #[doc = "0xf4 - need_des"]
    #[inline(always)]
    pub const fn i2s0_tx_div_ctrl0(&self) -> &I2S0_TX_DIV_CTRL0 {
        &self.i2s0_tx_div_ctrl0
    }
    #[doc = "0xf8 - need_des"]
    #[inline(always)]
    pub const fn i2s1_ctrl0(&self) -> &I2S1_CTRL0 {
        &self.i2s1_ctrl0
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn i2s1_rx_ctrl0(&self) -> &I2S1_RX_CTRL0 {
        &self.i2s1_rx_ctrl0
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn i2s1_rx_div_ctrl0(&self) -> &I2S1_RX_DIV_CTRL0 {
        &self.i2s1_rx_div_ctrl0
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn i2s1_tx_ctrl0(&self) -> &I2S1_TX_CTRL0 {
        &self.i2s1_tx_ctrl0
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn i2s1_tx_div_ctrl0(&self) -> &I2S1_TX_DIV_CTRL0 {
        &self.i2s1_tx_div_ctrl0
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn twai0_ctrl0(&self) -> &TWAI0_CTRL0 {
        &self.twai0_ctrl0
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn twai1_ctrl0(&self) -> &TWAI1_CTRL0 {
        &self.twai1_ctrl0
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn timergrp0_ctrl0(&self) -> &TIMERGRP0_CTRL0 {
        &self.timergrp0_ctrl0
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn timergrp0_tgrt_ctrl0(&self) -> &TIMERGRP0_TGRT_CTRL0 {
        &self.timergrp0_tgrt_ctrl0
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn timergrp1_ctrl0(&self) -> &TIMERGRP1_CTRL0 {
        &self.timergrp1_ctrl0
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn systimer_ctrl0(&self) -> &SYSTIMER_CTRL0 {
        &self.systimer_ctrl0
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn mcpwm0_ctrl0(&self) -> &MCPWM0_CTRL0 {
        &self.mcpwm0_ctrl0
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn mcpwm1_ctrl0(&self) -> &MCPWM1_CTRL0 {
        &self.mcpwm1_ctrl0
    }
    #[doc = "0x12c - need_des"]
    #[inline(always)]
    pub const fn mcpwm2_ctrl0(&self) -> &MCPWM2_CTRL0 {
        &self.mcpwm2_ctrl0
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn mcpwm3_ctrl0(&self) -> &MCPWM3_CTRL0 {
        &self.mcpwm3_ctrl0
    }
    #[doc = "0x134 - need_des"]
    #[inline(always)]
    pub const fn intrmtx_ctrl0(&self) -> &INTRMTX_CTRL0 {
        &self.intrmtx_ctrl0
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn pcnt_ctrl0(&self) -> &PCNT_CTRL0 {
        &self.pcnt_ctrl0
    }
    #[doc = "0x144 - need_des"]
    #[inline(always)]
    pub const fn usb_device_ctrl0(&self) -> &USB_DEVICE_CTRL0 {
        &self.usb_device_ctrl0
    }
    #[doc = "0x148 - need_des"]
    #[inline(always)]
    pub const fn ledc_ctrl0(&self) -> &LEDC_CTRL0 {
        &self.ledc_ctrl0
    }
    #[doc = "0x14c - need_des"]
    #[inline(always)]
    pub const fn lcdcam_ctrl0(&self) -> &LCDCAM_CTRL0 {
        &self.lcdcam_ctrl0
    }
    #[doc = "0x150 - need_des"]
    #[inline(always)]
    pub const fn lcdcam_lcd_ctrl0(&self) -> &LCDCAM_LCD_CTRL0 {
        &self.lcdcam_lcd_ctrl0
    }
    #[doc = "0x154 - need_des"]
    #[inline(always)]
    pub const fn lcdcam_lcdcam_ctrl0(&self) -> &LCDCAM_LCDCAM_CTRL0 {
        &self.lcdcam_lcdcam_ctrl0
    }
    #[doc = "0x158 - need_des"]
    #[inline(always)]
    pub const fn lcdcam_cam_ctrl0(&self) -> &LCDCAM_CAM_CTRL0 {
        &self.lcdcam_cam_ctrl0
    }
    #[doc = "0x15c - need_des"]
    #[inline(always)]
    pub const fn iomux_ctrl0(&self) -> &IOMUX_CTRL0 {
        &self.iomux_ctrl0
    }
    #[doc = "0x160 - need_des"]
    #[inline(always)]
    pub const fn hpwdt_core0_rst_ctrl0(&self) -> &HPWDT_CORE0_RST_CTRL0 {
        &self.hpwdt_core0_rst_ctrl0
    }
    #[doc = "0x164 - need_des"]
    #[inline(always)]
    pub const fn hpwdt_core1_rst_ctrl0(&self) -> &HPWDT_CORE1_RST_CTRL0 {
        &self.hpwdt_core1_rst_ctrl0
    }
    #[doc = "0x168 - CPU Source Frequency"]
    #[inline(always)]
    pub const fn cpu_src_freq0(&self) -> &CPU_SRC_FREQ0 {
        &self.cpu_src_freq0
    }
    #[doc = "0x16c - CPU Clock Status"]
    #[inline(always)]
    pub const fn cpu_clk_status0(&self) -> &CPU_CLK_STATUS0 {
        &self.cpu_clk_status0
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn hpcore_wdt_reset_source0(&self) -> &HPCORE_WDT_RESET_SOURCE0 {
        &self.hpcore_wdt_reset_source0
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn ana_pll_ctrl0(&self) -> &ANA_PLL_CTRL0 {
        &self.ana_pll_ctrl0
    }
    #[doc = "0x178 - need_des"]
    #[inline(always)]
    pub const fn ref_500m_ctrl0(&self) -> &REF_500M_CTRL0 {
        &self.ref_500m_ctrl0
    }
    #[doc = "0x17c - need_des"]
    #[inline(always)]
    pub const fn ref_240m_ctrl0(&self) -> &REF_240M_CTRL0 {
        &self.ref_240m_ctrl0
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn ref_160m_ctrl0(&self) -> &REF_160M_CTRL0 {
        &self.ref_160m_ctrl0
    }
    #[doc = "0x184 - need_des"]
    #[inline(always)]
    pub const fn ref_120m_ctrl0(&self) -> &REF_120M_CTRL0 {
        &self.ref_120m_ctrl0
    }
    #[doc = "0x188 - need_des"]
    #[inline(always)]
    pub const fn ref_80m_ctrl0(&self) -> &REF_80M_CTRL0 {
        &self.ref_80m_ctrl0
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn ref_60m_ctrl0(&self) -> &REF_60M_CTRL0 {
        &self.ref_60m_ctrl0
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn ref_20m_ctrl0(&self) -> &REF_20M_CTRL0 {
        &self.ref_20m_ctrl0
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn ref_50m_ctrl0(&self) -> &REF_50M_CTRL0 {
        &self.ref_50m_ctrl0
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn ref_25m_ctrl0(&self) -> &REF_25M_CTRL0 {
        &self.ref_25m_ctrl0
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn tm_clk_ctrl0(&self) -> &TM_CLK_CTRL0 {
        &self.tm_clk_ctrl0
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn dbg0_clk_ctrl0(&self) -> &DBG0_CLK_CTRL0 {
        &self.dbg0_clk_ctrl0
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn dbg1_clk_ctrl0(&self) -> &DBG1_CLK_CTRL0 {
        &self.dbg1_clk_ctrl0
    }
    #[doc = "0x1a8 - need_des"]
    #[inline(always)]
    pub const fn dbg2_clk_ctrl0(&self) -> &DBG2_CLK_CTRL0 {
        &self.dbg2_clk_ctrl0
    }
    #[doc = "0x1ac - need_des"]
    #[inline(always)]
    pub const fn lp_clk_ctrl0(&self) -> &LP_CLK_CTRL0 {
        &self.lp_clk_ctrl0
    }
    #[doc = "0x1b0 - need_des"]
    #[inline(always)]
    pub const fn ahb_asrc_ctrl0(&self) -> &AHB_ASRC_CTRL0 {
        &self.ahb_asrc_ctrl0
    }
    #[doc = "0x1b4 - need_des"]
    #[inline(always)]
    pub const fn cordic_ctrl0(&self) -> &CORDIC_CTRL0 {
        &self.cordic_ctrl0
    }
    #[doc = "0x1b8 - need_des"]
    #[inline(always)]
    pub const fn zero_det_ctrl0(&self) -> &ZERO_DET_CTRL0 {
        &self.zero_det_ctrl0
    }
    #[doc = "0x1bc - need_des"]
    #[inline(always)]
    pub const fn cordic_ctrl1(&self) -> &CORDIC_CTRL1 {
        &self.cordic_ctrl1
    }
    #[doc = "0x1c8 - need_des"]
    #[inline(always)]
    pub const fn clk_pwr_decrease(&self) -> &CLK_PWR_DECREASE {
        &self.clk_pwr_decrease
    }
    #[doc = "0x1cc - need_des"]
    #[inline(always)]
    pub const fn cnnt_iomux_ctrl0(&self) -> &CNNT_IOMUX_CTRL0 {
        &self.cnnt_iomux_ctrl0
    }
    #[doc = "0x1d0 - need_des"]
    #[inline(always)]
    pub const fn hp_i2cmst_ctrl0(&self) -> &HP_I2CMST_CTRL0 {
        &self.hp_i2cmst_ctrl0
    }
    #[doc = "0x1d4 - need_des"]
    #[inline(always)]
    pub const fn axi_perf_mon_ctrl0(&self) -> &AXI_PERF_MON_CTRL0 {
        &self.axi_perf_mon_ctrl0
    }
    #[doc = "0x1d8 - need_des"]
    #[inline(always)]
    pub const fn cnnt_sysreg_ctrl0(&self) -> &CNNT_SYSREG_CTRL0 {
        &self.cnnt_sysreg_ctrl0
    }
    #[doc = "0x1dc - need_des"]
    #[inline(always)]
    pub const fn hp_alive_sysreg_ctrl0(&self) -> &HP_ALIVE_SYSREG_CTRL0 {
        &self.hp_alive_sysreg_ctrl0
    }
    #[doc = "0x1e0 - MODEM_APB configuration register"]
    #[inline(always)]
    pub const fn modem_conf(&self) -> &MODEM_CONF {
        &self.modem_conf
    }
    #[doc = "0x1e4 - xxxx"]
    #[inline(always)]
    pub const fn adc_dac_inv_phase_conf(&self) -> &ADC_DAC_INV_PHASE_CONF {
        &self.adc_dac_inv_phase_conf
    }
    #[doc = "0x1e8 - xxxx"]
    #[inline(always)]
    pub const fn pwdet_sar_clk_conf(&self) -> &PWDET_SAR_CLK_CONF {
        &self.pwdet_sar_clk_conf
    }
    #[doc = "0x1ec - need_des"]
    #[inline(always)]
    pub const fn pad_bist_sdio(&self) -> &PAD_BIST_SDIO {
        &self.pad_bist_sdio
    }
    #[doc = "0x1f0 - need_des"]
    #[inline(always)]
    pub const fn pad_bist_gmac0(&self) -> &PAD_BIST_GMAC0 {
        &self.pad_bist_gmac0
    }
    #[doc = "0x1f4 - need_des"]
    #[inline(always)]
    pub const fn pad_bist_gmac1(&self) -> &PAD_BIST_GMAC1 {
        &self.pad_bist_gmac1
    }
    #[doc = "0x1f8 - need_des"]
    #[inline(always)]
    pub const fn clk_en0(&self) -> &CLK_EN0 {
        &self.clk_en0
    }
    #[doc = "0x1fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SOC_CLK_SEL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_clk_sel`] module"]
pub type SOC_CLK_SEL = crate::Reg<soc_clk_sel::SOC_CLK_SEL_SPEC>;
#[doc = "need_des"]
pub mod soc_clk_sel;
#[doc = "CPU_FREQ_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_freq_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_freq_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_freq_ctrl0`] module"]
pub type CPU_FREQ_CTRL0 = crate::Reg<cpu_freq_ctrl0::CPU_FREQ_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cpu_freq_ctrl0;
#[doc = "MEM_FREQ_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_freq_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_freq_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_freq_ctrl0`] module"]
pub type MEM_FREQ_CTRL0 = crate::Reg<mem_freq_ctrl0::MEM_FREQ_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mem_freq_ctrl0;
#[doc = "SYS_FREQ_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_freq_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_freq_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_freq_ctrl0`] module"]
pub type SYS_FREQ_CTRL0 = crate::Reg<sys_freq_ctrl0::SYS_FREQ_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod sys_freq_ctrl0;
#[doc = "APB_FREQ_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_freq_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_freq_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_freq_ctrl0`] module"]
pub type APB_FREQ_CTRL0 = crate::Reg<apb_freq_ctrl0::APB_FREQ_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod apb_freq_ctrl0;
#[doc = "ROOT_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_ctrl0`] module"]
pub type ROOT_CLK_CTRL0 = crate::Reg<root_clk_ctrl0::ROOT_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod root_clk_ctrl0;
#[doc = "CPU_WFI_DELAY_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wfi_delay_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wfi_delay_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_wfi_delay_ctrl0`] module"]
pub type CPU_WFI_DELAY_CTRL0 = crate::Reg<cpu_wfi_delay_ctrl0::CPU_WFI_DELAY_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cpu_wfi_delay_ctrl0;
#[doc = "HPCORE0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore0_ctrl0`] module"]
pub type HPCORE0_CTRL0 = crate::Reg<hpcore0_ctrl0::HPCORE0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hpcore0_ctrl0;
#[doc = "HPCORE1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore1_ctrl0`] module"]
pub type HPCORE1_CTRL0 = crate::Reg<hpcore1_ctrl0::HPCORE1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hpcore1_ctrl0;
#[doc = "TRACE_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_ctrl0`] module"]
pub type TRACE_CTRL0 = crate::Reg<trace_ctrl0::TRACE_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod trace_ctrl0;
#[doc = "TCM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_ctrl0`] module"]
pub type TCM_CTRL0 = crate::Reg<tcm_ctrl0::TCM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod tcm_ctrl0;
#[doc = "TCMMON_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmmon_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmmon_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmmon_ctrl0`] module"]
pub type TCMMON_CTRL0 = crate::Reg<tcmmon_ctrl0::TCMMON_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod tcmmon_ctrl0;
#[doc = "PSRAMMON_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psrammon_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psrammon_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psrammon_ctrl0`] module"]
pub type PSRAMMON_CTRL0 = crate::Reg<psrammon_ctrl0::PSRAMMON_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod psrammon_ctrl0;
#[doc = "ROM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_ctrl0`] module"]
pub type ROM_CTRL0 = crate::Reg<rom_ctrl0::ROM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod rom_ctrl0;
#[doc = "CACHE_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_ctrl0`] module"]
pub type CACHE_CTRL0 = crate::Reg<cache_ctrl0::CACHE_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cache_ctrl0;
#[doc = "ICM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_ctrl0`] module"]
pub type ICM_CTRL0 = crate::Reg<icm_ctrl0::ICM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod icm_ctrl0;
#[doc = "MODEM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_ctrl0`] module"]
pub type MODEM_CTRL0 = crate::Reg<modem_ctrl0::MODEM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod modem_ctrl0;
#[doc = "MISC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_ctrl0`] module"]
pub type MISC_CTRL0 = crate::Reg<misc_ctrl0::MISC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod misc_ctrl0;
#[doc = "BUSMON_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`busmon_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmon_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmon_ctrl0`] module"]
pub type BUSMON_CTRL0 = crate::Reg<busmon_ctrl0::BUSMON_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod busmon_ctrl0;
#[doc = "REMOVE_TMP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`remove_tmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remove_tmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remove_tmp0`] module"]
pub type REMOVE_TMP0 = crate::Reg<remove_tmp0::REMOVE_TMP0_SPEC>;
#[doc = "need_des"]
pub mod remove_tmp0;
#[doc = "PVT0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt0_ctrl0`] module"]
pub type PVT0_CTRL0 = crate::Reg<pvt0_ctrl0::PVT0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod pvt0_ctrl0;
#[doc = "PVT0_PERI_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pvt0_peri_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvt0_peri_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvt0_peri_ctrl0`] module"]
pub type PVT0_PERI_CTRL0 = crate::Reg<pvt0_peri_ctrl0::PVT0_PERI_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod pvt0_peri_ctrl0;
#[doc = "CRYPTO_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_ctrl0`] module"]
pub type CRYPTO_CTRL0 = crate::Reg<crypto_ctrl0::CRYPTO_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod crypto_ctrl0;
#[doc = "KEY_MANAGER_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`key_manager_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_manager_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_manager_ctrl0`] module"]
pub type KEY_MANAGER_CTRL0 = crate::Reg<key_manager_ctrl0::KEY_MANAGER_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod key_manager_ctrl0;
#[doc = "DPA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_ctrl0`] module"]
pub type DPA_CTRL0 = crate::Reg<dpa_ctrl0::DPA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dpa_ctrl0;
#[doc = "FLASH_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ctrl0`] module"]
pub type FLASH_CTRL0 = crate::Reg<flash_ctrl0::FLASH_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod flash_ctrl0;
#[doc = "PSRAM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_ctrl0`] module"]
pub type PSRAM_CTRL0 = crate::Reg<psram_ctrl0::PSRAM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod psram_ctrl0;
#[doc = "GPSPI2_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi2_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi2_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi2_ctrl0`] module"]
pub type GPSPI2_CTRL0 = crate::Reg<gpspi2_ctrl0::GPSPI2_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod gpspi2_ctrl0;
#[doc = "GPSPI3_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gpspi3_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpspi3_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpspi3_ctrl0`] module"]
pub type GPSPI3_CTRL0 = crate::Reg<gpspi3_ctrl0::GPSPI3_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod gpspi3_ctrl0;
#[doc = "GDMA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctrl0`] module"]
pub type GDMA_CTRL0 = crate::Reg<gdma_ctrl0::GDMA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod gdma_ctrl0;
#[doc = "AXI_PDMA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_ctrl0`] module"]
pub type AXI_PDMA_CTRL0 = crate::Reg<axi_pdma_ctrl0::AXI_PDMA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod axi_pdma_ctrl0;
#[doc = "AHB_PDMA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_pdma_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_pdma_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_pdma_ctrl0`] module"]
pub type AHB_PDMA_CTRL0 = crate::Reg<ahb_pdma_ctrl0::AHB_PDMA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ahb_pdma_ctrl0;
#[doc = "REGDMA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_ctrl0`] module"]
pub type REGDMA_CTRL0 = crate::Reg<regdma_ctrl0::REGDMA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod regdma_ctrl0;
#[doc = "UHCI_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci_ctrl0`] module"]
pub type UHCI_CTRL0 = crate::Reg<uhci_ctrl0::UHCI_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod uhci_ctrl0;
#[doc = "UART0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_ctrl0`] module"]
pub type UART0_CTRL0 = crate::Reg<uart0_ctrl0::UART0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod uart0_ctrl0;
#[doc = "UART1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_ctrl0`] module"]
pub type UART1_CTRL0 = crate::Reg<uart1_ctrl0::UART1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod uart1_ctrl0;
#[doc = "UART2_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_ctrl0`] module"]
pub type UART2_CTRL0 = crate::Reg<uart2_ctrl0::UART2_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod uart2_ctrl0;
#[doc = "UART3_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_ctrl0`] module"]
pub type UART3_CTRL0 = crate::Reg<uart3_ctrl0::UART3_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod uart3_ctrl0;
#[doc = "PARLIO_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`parlio_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parlio_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parlio_ctrl0`] module"]
pub type PARLIO_CTRL0 = crate::Reg<parlio_ctrl0::PARLIO_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod parlio_ctrl0;
#[doc = "PARLIO_RX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`parlio_rx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parlio_rx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parlio_rx_ctrl0`] module"]
pub type PARLIO_RX_CTRL0 = crate::Reg<parlio_rx_ctrl0::PARLIO_RX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod parlio_rx_ctrl0;
#[doc = "PARLIO_TX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`parlio_tx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parlio_tx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@parlio_tx_ctrl0`] module"]
pub type PARLIO_TX_CTRL0 = crate::Reg<parlio_tx_ctrl0::PARLIO_TX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod parlio_tx_ctrl0;
#[doc = "BITSRAMBLER_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`bitsrambler_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitsrambler_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitsrambler_ctrl0`] module"]
pub type BITSRAMBLER_CTRL0 = crate::Reg<bitsrambler_ctrl0::BITSRAMBLER_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod bitsrambler_ctrl0;
#[doc = "ETM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etm_ctrl0`] module"]
pub type ETM_CTRL0 = crate::Reg<etm_ctrl0::ETM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod etm_ctrl0;
#[doc = "USB_OTGHS_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_otghs_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otghs_ctrl0`] module"]
pub type USB_OTGHS_CTRL0 = crate::Reg<usb_otghs_ctrl0::USB_OTGHS_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod usb_otghs_ctrl0;
#[doc = "DMA2D_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_ctrl0`] module"]
pub type DMA2D_CTRL0 = crate::Reg<dma2d_ctrl0::DMA2D_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dma2d_ctrl0;
#[doc = "PPA_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ppa_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppa_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppa_ctrl0`] module"]
pub type PPA_CTRL0 = crate::Reg<ppa_ctrl0::PPA_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ppa_ctrl0;
#[doc = "JPEG_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_ctrl0`] module"]
pub type JPEG_CTRL0 = crate::Reg<jpeg_ctrl0::JPEG_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod jpeg_ctrl0;
#[doc = "RMT_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_ctrl0`] module"]
pub type RMT_CTRL0 = crate::Reg<rmt_ctrl0::RMT_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod rmt_ctrl0;
#[doc = "SDIO_HOST_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_host_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_host_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_host_ctrl0`] module"]
pub type SDIO_HOST_CTRL0 = crate::Reg<sdio_host_ctrl0::SDIO_HOST_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod sdio_host_ctrl0;
#[doc = "SDIO_HOST_FUNC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_host_func_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_host_func_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_host_func_ctrl0`] module"]
pub type SDIO_HOST_FUNC_CTRL0 = crate::Reg<sdio_host_func_ctrl0::SDIO_HOST_FUNC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod sdio_host_func_ctrl0;
#[doc = "EMAC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`emac_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emac_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emac_ctrl0`] module"]
pub type EMAC_CTRL0 = crate::Reg<emac_ctrl0::EMAC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod emac_ctrl0;
#[doc = "DSI_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ctrl0`] module"]
pub type DSI_CTRL0 = crate::Reg<dsi_ctrl0::DSI_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dsi_ctrl0;
#[doc = "DSI_PHY_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_phy_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_phy_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_phy_ctrl0`] module"]
pub type DSI_PHY_CTRL0 = crate::Reg<dsi_phy_ctrl0::DSI_PHY_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dsi_phy_ctrl0;
#[doc = "DSI_DPI_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_dpi_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dpi_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpi_ctrl0`] module"]
pub type DSI_DPI_CTRL0 = crate::Reg<dsi_dpi_ctrl0::DSI_DPI_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dsi_dpi_ctrl0;
#[doc = "MSPI_PAD_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_pad_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_pad_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_pad_ctrl0`] module"]
pub type MSPI_PAD_CTRL0 = crate::Reg<mspi_pad_ctrl0::MSPI_PAD_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mspi_pad_ctrl0;
#[doc = "I2C0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_ctrl0`] module"]
pub type I2C0_CTRL0 = crate::Reg<i2c0_ctrl0::I2C0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2c0_ctrl0;
#[doc = "I2C1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_ctrl0`] module"]
pub type I2C1_CTRL0 = crate::Reg<i2c1_ctrl0::I2C1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2c1_ctrl0;
#[doc = "I2S0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_ctrl0`] module"]
pub type I2S0_CTRL0 = crate::Reg<i2s0_ctrl0::I2S0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s0_ctrl0;
#[doc = "I2S0_RX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_rx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_rx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_rx_ctrl0`] module"]
pub type I2S0_RX_CTRL0 = crate::Reg<i2s0_rx_ctrl0::I2S0_RX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s0_rx_ctrl0;
#[doc = "I2S0_RX_DIV_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_rx_div_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_rx_div_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_rx_div_ctrl0`] module"]
pub type I2S0_RX_DIV_CTRL0 = crate::Reg<i2s0_rx_div_ctrl0::I2S0_RX_DIV_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s0_rx_div_ctrl0;
#[doc = "I2S0_TX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_tx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_tx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_tx_ctrl0`] module"]
pub type I2S0_TX_CTRL0 = crate::Reg<i2s0_tx_ctrl0::I2S0_TX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s0_tx_ctrl0;
#[doc = "I2S0_TX_DIV_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_tx_div_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_tx_div_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s0_tx_div_ctrl0`] module"]
pub type I2S0_TX_DIV_CTRL0 = crate::Reg<i2s0_tx_div_ctrl0::I2S0_TX_DIV_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s0_tx_div_ctrl0;
#[doc = "I2S1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_ctrl0`] module"]
pub type I2S1_CTRL0 = crate::Reg<i2s1_ctrl0::I2S1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s1_ctrl0;
#[doc = "I2S1_RX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_rx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_rx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_rx_ctrl0`] module"]
pub type I2S1_RX_CTRL0 = crate::Reg<i2s1_rx_ctrl0::I2S1_RX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s1_rx_ctrl0;
#[doc = "I2S1_RX_DIV_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_rx_div_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_rx_div_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_rx_div_ctrl0`] module"]
pub type I2S1_RX_DIV_CTRL0 = crate::Reg<i2s1_rx_div_ctrl0::I2S1_RX_DIV_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s1_rx_div_ctrl0;
#[doc = "I2S1_TX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_tx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_tx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_tx_ctrl0`] module"]
pub type I2S1_TX_CTRL0 = crate::Reg<i2s1_tx_ctrl0::I2S1_TX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s1_tx_ctrl0;
#[doc = "I2S1_TX_DIV_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_tx_div_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_tx_div_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2s1_tx_div_ctrl0`] module"]
pub type I2S1_TX_DIV_CTRL0 = crate::Reg<i2s1_tx_div_ctrl0::I2S1_TX_DIV_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod i2s1_tx_div_ctrl0;
#[doc = "TWAI0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`twai0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twai0_ctrl0`] module"]
pub type TWAI0_CTRL0 = crate::Reg<twai0_ctrl0::TWAI0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod twai0_ctrl0;
#[doc = "TWAI1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`twai1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twai1_ctrl0`] module"]
pub type TWAI1_CTRL0 = crate::Reg<twai1_ctrl0::TWAI1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod twai1_ctrl0;
#[doc = "TIMERGRP0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_ctrl0`] module"]
pub type TIMERGRP0_CTRL0 = crate::Reg<timergrp0_ctrl0::TIMERGRP0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod timergrp0_ctrl0;
#[doc = "TIMERGRP0_TGRT_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_tgrt_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_tgrt_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp0_tgrt_ctrl0`] module"]
pub type TIMERGRP0_TGRT_CTRL0 = crate::Reg<timergrp0_tgrt_ctrl0::TIMERGRP0_TGRT_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod timergrp0_tgrt_ctrl0;
#[doc = "TIMERGRP1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timergrp1_ctrl0`] module"]
pub type TIMERGRP1_CTRL0 = crate::Reg<timergrp1_ctrl0::TIMERGRP1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod timergrp1_ctrl0;
#[doc = "SYSTIMER_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimer_ctrl0`] module"]
pub type SYSTIMER_CTRL0 = crate::Reg<systimer_ctrl0::SYSTIMER_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod systimer_ctrl0;
#[doc = "MCPWM0_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm0_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm0_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm0_ctrl0`] module"]
pub type MCPWM0_CTRL0 = crate::Reg<mcpwm0_ctrl0::MCPWM0_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mcpwm0_ctrl0;
#[doc = "MCPWM1_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm1_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm1_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm1_ctrl0`] module"]
pub type MCPWM1_CTRL0 = crate::Reg<mcpwm1_ctrl0::MCPWM1_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mcpwm1_ctrl0;
#[doc = "MCPWM2_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm2_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm2_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm2_ctrl0`] module"]
pub type MCPWM2_CTRL0 = crate::Reg<mcpwm2_ctrl0::MCPWM2_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mcpwm2_ctrl0;
#[doc = "MCPWM3_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm3_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm3_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcpwm3_ctrl0`] module"]
pub type MCPWM3_CTRL0 = crate::Reg<mcpwm3_ctrl0::MCPWM3_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod mcpwm3_ctrl0;
#[doc = "INTRMTX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`intrmtx_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrmtx_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrmtx_ctrl0`] module"]
pub type INTRMTX_CTRL0 = crate::Reg<intrmtx_ctrl0::INTRMTX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod intrmtx_ctrl0;
#[doc = "PCNT_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt_ctrl0`] module"]
pub type PCNT_CTRL0 = crate::Reg<pcnt_ctrl0::PCNT_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod pcnt_ctrl0;
#[doc = "USB_DEVICE_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_device_ctrl0`] module"]
pub type USB_DEVICE_CTRL0 = crate::Reg<usb_device_ctrl0::USB_DEVICE_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod usb_device_ctrl0;
#[doc = "LEDC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc_ctrl0`] module"]
pub type LEDC_CTRL0 = crate::Reg<ledc_ctrl0::LEDC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ledc_ctrl0;
#[doc = "LCDCAM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcam_ctrl0`] module"]
pub type LCDCAM_CTRL0 = crate::Reg<lcdcam_ctrl0::LCDCAM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lcdcam_ctrl0;
#[doc = "LCDCAM_LCD_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_lcd_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_lcd_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcam_lcd_ctrl0`] module"]
pub type LCDCAM_LCD_CTRL0 = crate::Reg<lcdcam_lcd_ctrl0::LCDCAM_LCD_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lcdcam_lcd_ctrl0;
#[doc = "LCDCAM_LCDCAM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_lcdcam_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_lcdcam_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcam_lcdcam_ctrl0`] module"]
pub type LCDCAM_LCDCAM_CTRL0 = crate::Reg<lcdcam_lcdcam_ctrl0::LCDCAM_LCDCAM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lcdcam_lcdcam_ctrl0;
#[doc = "LCDCAM_CAM_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_cam_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_cam_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcam_cam_ctrl0`] module"]
pub type LCDCAM_CAM_CTRL0 = crate::Reg<lcdcam_cam_ctrl0::LCDCAM_CAM_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lcdcam_cam_ctrl0;
#[doc = "IOMUX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iomux_ctrl0`] module"]
pub type IOMUX_CTRL0 = crate::Reg<iomux_ctrl0::IOMUX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod iomux_ctrl0;
#[doc = "HPWDT_CORE0_RST_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpwdt_core0_rst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpwdt_core0_rst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpwdt_core0_rst_ctrl0`] module"]
pub type HPWDT_CORE0_RST_CTRL0 = crate::Reg<hpwdt_core0_rst_ctrl0::HPWDT_CORE0_RST_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hpwdt_core0_rst_ctrl0;
#[doc = "HPWDT_CORE1_RST_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpwdt_core1_rst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpwdt_core1_rst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpwdt_core1_rst_ctrl0`] module"]
pub type HPWDT_CORE1_RST_CTRL0 = crate::Reg<hpwdt_core1_rst_ctrl0::HPWDT_CORE1_RST_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hpwdt_core1_rst_ctrl0;
#[doc = "CPU_SRC_FREQ0 (r) register accessor: CPU Source Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_src_freq0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_src_freq0`] module"]
pub type CPU_SRC_FREQ0 = crate::Reg<cpu_src_freq0::CPU_SRC_FREQ0_SPEC>;
#[doc = "CPU Source Frequency"]
pub mod cpu_src_freq0;
#[doc = "CPU_CLK_STATUS0 (r) register accessor: CPU Clock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_clk_status0`] module"]
pub type CPU_CLK_STATUS0 = crate::Reg<cpu_clk_status0::CPU_CLK_STATUS0_SPEC>;
#[doc = "CPU Clock Status"]
pub mod cpu_clk_status0;
#[doc = "HPCORE_WDT_RESET_SOURCE0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_wdt_reset_source0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_wdt_reset_source0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore_wdt_reset_source0`] module"]
pub type HPCORE_WDT_RESET_SOURCE0 =
    crate::Reg<hpcore_wdt_reset_source0::HPCORE_WDT_RESET_SOURCE0_SPEC>;
#[doc = "need_des"]
pub mod hpcore_wdt_reset_source0;
#[doc = "ANA_PLL_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_pll_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pll_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_pll_ctrl0`] module"]
pub type ANA_PLL_CTRL0 = crate::Reg<ana_pll_ctrl0::ANA_PLL_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ana_pll_ctrl0;
#[doc = "REF_500M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_500m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_500m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_500m_ctrl0`] module"]
pub type REF_500M_CTRL0 = crate::Reg<ref_500m_ctrl0::REF_500M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_500m_ctrl0;
#[doc = "REF_240M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_240m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_240m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_240m_ctrl0`] module"]
pub type REF_240M_CTRL0 = crate::Reg<ref_240m_ctrl0::REF_240M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_240m_ctrl0;
#[doc = "REF_160M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_160m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_160m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_160m_ctrl0`] module"]
pub type REF_160M_CTRL0 = crate::Reg<ref_160m_ctrl0::REF_160M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_160m_ctrl0;
#[doc = "REF_120M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_120m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_120m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_120m_ctrl0`] module"]
pub type REF_120M_CTRL0 = crate::Reg<ref_120m_ctrl0::REF_120M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_120m_ctrl0;
#[doc = "REF_80M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_80m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_80m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_80m_ctrl0`] module"]
pub type REF_80M_CTRL0 = crate::Reg<ref_80m_ctrl0::REF_80M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_80m_ctrl0;
#[doc = "REF_60M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_60m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_60m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_60m_ctrl0`] module"]
pub type REF_60M_CTRL0 = crate::Reg<ref_60m_ctrl0::REF_60M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_60m_ctrl0;
#[doc = "REF_20M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_20m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_20m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_20m_ctrl0`] module"]
pub type REF_20M_CTRL0 = crate::Reg<ref_20m_ctrl0::REF_20M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_20m_ctrl0;
#[doc = "REF_50M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_50m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_50m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_50m_ctrl0`] module"]
pub type REF_50M_CTRL0 = crate::Reg<ref_50m_ctrl0::REF_50M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_50m_ctrl0;
#[doc = "REF_25M_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_25m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_25m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_25m_ctrl0`] module"]
pub type REF_25M_CTRL0 = crate::Reg<ref_25m_ctrl0::REF_25M_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ref_25m_ctrl0;
#[doc = "TM_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tm_clk_ctrl0`] module"]
pub type TM_CLK_CTRL0 = crate::Reg<tm_clk_ctrl0::TM_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod tm_clk_ctrl0;
#[doc = "DBG0_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg0_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg0_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg0_clk_ctrl0`] module"]
pub type DBG0_CLK_CTRL0 = crate::Reg<dbg0_clk_ctrl0::DBG0_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dbg0_clk_ctrl0;
#[doc = "DBG1_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg1_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg1_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg1_clk_ctrl0`] module"]
pub type DBG1_CLK_CTRL0 = crate::Reg<dbg1_clk_ctrl0::DBG1_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dbg1_clk_ctrl0;
#[doc = "DBG2_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg2_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg2_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg2_clk_ctrl0`] module"]
pub type DBG2_CLK_CTRL0 = crate::Reg<dbg2_clk_ctrl0::DBG2_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod dbg2_clk_ctrl0;
#[doc = "LP_CLK_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_ctrl0`] module"]
pub type LP_CLK_CTRL0 = crate::Reg<lp_clk_ctrl0::LP_CLK_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_ctrl0;
#[doc = "AHB_ASRC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_asrc_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_asrc_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_asrc_ctrl0`] module"]
pub type AHB_ASRC_CTRL0 = crate::Reg<ahb_asrc_ctrl0::AHB_ASRC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod ahb_asrc_ctrl0;
#[doc = "CORDIC_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_ctrl0`] module"]
pub type CORDIC_CTRL0 = crate::Reg<cordic_ctrl0::CORDIC_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cordic_ctrl0;
#[doc = "ZERO_DET_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_det_ctrl0`] module"]
pub type ZERO_DET_CTRL0 = crate::Reg<zero_det_ctrl0::ZERO_DET_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod zero_det_ctrl0;
#[doc = "CORDIC_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cordic_ctrl1`] module"]
pub type CORDIC_CTRL1 = crate::Reg<cordic_ctrl1::CORDIC_CTRL1_SPEC>;
#[doc = "need_des"]
pub mod cordic_ctrl1;
#[doc = "CLK_PWR_DECREASE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pwr_decrease::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pwr_decrease::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pwr_decrease`] module"]
pub type CLK_PWR_DECREASE = crate::Reg<clk_pwr_decrease::CLK_PWR_DECREASE_SPEC>;
#[doc = "need_des"]
pub mod clk_pwr_decrease;
#[doc = "CNNT_IOMUX_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_iomux_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_iomux_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnnt_iomux_ctrl0`] module"]
pub type CNNT_IOMUX_CTRL0 = crate::Reg<cnnt_iomux_ctrl0::CNNT_IOMUX_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cnnt_iomux_ctrl0;
#[doc = "HP_I2CMST_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2cmst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2cmst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_i2cmst_ctrl0`] module"]
pub type HP_I2CMST_CTRL0 = crate::Reg<hp_i2cmst_ctrl0::HP_I2CMST_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hp_i2cmst_ctrl0;
#[doc = "AXI_PERF_MON_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_perf_mon_ctrl0`] module"]
pub type AXI_PERF_MON_CTRL0 = crate::Reg<axi_perf_mon_ctrl0::AXI_PERF_MON_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod axi_perf_mon_ctrl0;
#[doc = "CNNT_SYSREG_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cnnt_sysreg_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnnt_sysreg_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnnt_sysreg_ctrl0`] module"]
pub type CNNT_SYSREG_CTRL0 = crate::Reg<cnnt_sysreg_ctrl0::CNNT_SYSREG_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod cnnt_sysreg_ctrl0;
#[doc = "HP_ALIVE_SYSREG_CTRL0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_alive_sysreg_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_alive_sysreg_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_alive_sysreg_ctrl0`] module"]
pub type HP_ALIVE_SYSREG_CTRL0 = crate::Reg<hp_alive_sysreg_ctrl0::HP_ALIVE_SYSREG_CTRL0_SPEC>;
#[doc = "need_des"]
pub mod hp_alive_sysreg_ctrl0;
#[doc = "MODEM_CONF (rw) register accessor: MODEM_APB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_conf`] module"]
pub type MODEM_CONF = crate::Reg<modem_conf::MODEM_CONF_SPEC>;
#[doc = "MODEM_APB configuration register"]
pub mod modem_conf;
#[doc = "ADC_DAC_INV_PHASE_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dac_inv_phase_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dac_inv_phase_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_dac_inv_phase_conf`] module"]
pub type ADC_DAC_INV_PHASE_CONF = crate::Reg<adc_dac_inv_phase_conf::ADC_DAC_INV_PHASE_CONF_SPEC>;
#[doc = "xxxx"]
pub mod adc_dac_inv_phase_conf;
#[doc = "PWDET_SAR_CLK_CONF (rw) register accessor: xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdet_sar_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwdet_sar_clk_conf`] module"]
pub type PWDET_SAR_CLK_CONF = crate::Reg<pwdet_sar_clk_conf::PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "xxxx"]
pub mod pwdet_sar_clk_conf;
#[doc = "PAD_BIST_SDIO (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_bist_sdio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_bist_sdio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_bist_sdio`] module"]
pub type PAD_BIST_SDIO = crate::Reg<pad_bist_sdio::PAD_BIST_SDIO_SPEC>;
#[doc = "need_des"]
pub mod pad_bist_sdio;
#[doc = "PAD_BIST_GMAC0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_bist_gmac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_bist_gmac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_bist_gmac0`] module"]
pub type PAD_BIST_GMAC0 = crate::Reg<pad_bist_gmac0::PAD_BIST_GMAC0_SPEC>;
#[doc = "need_des"]
pub mod pad_bist_gmac0;
#[doc = "PAD_BIST_GMAC1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_bist_gmac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_bist_gmac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_bist_gmac1`] module"]
pub type PAD_BIST_GMAC1 = crate::Reg<pad_bist_gmac1::PAD_BIST_GMAC1_SPEC>;
#[doc = "need_des"]
pub mod pad_bist_gmac1;
#[doc = "CLK_EN0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en0`] module"]
pub type CLK_EN0 = crate::Reg<clk_en0::CLK_EN0_SPEC>;
#[doc = "need_des"]
pub mod clk_en0;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
