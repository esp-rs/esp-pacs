#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver_date: VER_DATE,
    hp_clk_en: HP_CLK_EN,
    _reserved2: [u8; 0x08],
    hp_cpu_int_from_cpu_0: HP_CPU_INT_FROM_CPU_0,
    hp_cpu_int_from_cpu_1: HP_CPU_INT_FROM_CPU_1,
    hp_cpu_int_from_cpu_2: HP_CPU_INT_FROM_CPU_2,
    hp_cpu_int_from_cpu_3: HP_CPU_INT_FROM_CPU_3,
    _reserved6: [u8; 0x08],
    hp_modem_diag_en: HP_MODEM_DIAG_EN,
    _reserved7: [u8; 0x10],
    hp_tcm_ram_pwr_ctrl0: HP_TCM_RAM_PWR_CTRL0,
    hp_rom_pwr_ctrl0: HP_ROM_PWR_CTRL0,
    _reserved9: [u8; 0x0c],
    hp_probea_ctrl: HP_PROBEA_CTRL,
    hp_probeb_ctrl: HP_PROBEB_CTRL,
    _reserved11: [u8; 0x04],
    hp_probe_out: HP_PROBE_OUT,
    _reserved12: [u8; 0x04],
    hp_cpu_corestalled_st: HP_CPU_CORESTALLED_ST,
    _reserved13: [u8; 0x08],
    hp_crypto_ctrl: HP_CRYPTO_CTRL,
    _reserved14: [u8; 0x08],
    hp_iomux_fpga_debug: HP_IOMUX_FPGA_DEBUG,
    rdn_eco_cs: RDN_ECO_CS,
    _reserved16: [u8; 0x60],
    hp_tcm_rdn_eco_cs: HP_TCM_RDN_ECO_CS,
    hp_tcm_rdn_eco_low: HP_TCM_RDN_ECO_LOW,
    hp_tcm_rdn_eco_high: HP_TCM_RDN_ECO_HIGH,
    _reserved19: [u8; 0x20],
    hp_cache_pwr_ctrl: HP_CACHE_PWR_CTRL,
    hp_tcm_data_dump_ctrl: HP_TCM_DATA_DUMP_CTRL,
    hp_cpu_waiti_conf: HP_CPU_WAITI_CONF,
    core_debug_runstall_conf: CORE_DEBUG_RUNSTALL_CONF,
    _reserved23: [u8; 0x04],
    hp_core_ibus_timeout_conf: HP_CORE_IBUS_TIMEOUT_CONF,
    hp_core_dbus_timeout_conf: HP_CORE_DBUS_TIMEOUT_CONF,
    _reserved25: [u8; 0x0c],
    hp_icm_h2x_cfg: HP_ICM_H2X_CFG,
    _reserved26: [u8; 0x04],
    hp_bitscrambler_peri_sel: HP_BITSCRAMBLER_PERI_SEL,
    _reserved27: [u8; 0x04],
    gdma_ctrl: GDMA_CTRL,
    _reserved28: [u8; 0x0c],
    vpu_ctrl: VPU_CTRL,
    _reserved29: [u8; 0x14],
    hp_design_for_verification0: HP_DESIGN_FOR_VERIFICATION0,
    hp_design_for_verification1: HP_DESIGN_FOR_VERIFICATION1,
    _reserved31: [u8; 0x10],
    hp_ahb2axi_bresp_err_int_raw: HP_AHB2AXI_BRESP_ERR_INT_RAW,
    hp_ahb2axi_bresp_err_int_st: HP_AHB2AXI_BRESP_ERR_INT_ST,
    hp_ahb2axi_bresp_err_int_ena: HP_AHB2AXI_BRESP_ERR_INT_ENA,
    hp_ahb2axi_bresp_err_int_clr: HP_AHB2AXI_BRESP_ERR_INT_CLR,
    _reserved35: [u8; 0x08],
    hp_core_dmactive_lpcore: HP_CORE_DMACTIVE_LPCORE,
    hp_core_err_resp_dis: HP_CORE_ERR_RESP_DIS,
    hp_core_timeout_int_raw: HP_CORE_TIMEOUT_INT_RAW,
    hp_core_timeout_int_st: HP_CORE_TIMEOUT_INT_ST,
    hp_core_timeout_int_ena: HP_CORE_TIMEOUT_INT_ENA,
    hp_core_timeout_int_clr: HP_CORE_TIMEOUT_INT_CLR,
    hp_core_pad_wakeup_event: HP_CORE_PAD_WAKEUP_EVENT,
    _reserved42: [u8; 0x04],
    hp_twai0_timestamp_l: HP_TWAI0_TIMESTAMP_L,
    hp_twai0_timestamp_h: HP_TWAI0_TIMESTAMP_H,
    hp_twai1_timestamp_l: HP_TWAI1_TIMESTAMP_L,
    hp_twai1_timestamp_h: HP_TWAI1_TIMESTAMP_H,
    _reserved46: [u8; 0x08],
    hp_rng_cfg: HP_RNG_CFG,
    _reserved47: [u8; 0x04],
    hp_peri_mem_clk_force_on: HP_PERI_MEM_CLK_FORCE_ON,
    _reserved48: [u8; 0x04],
    prdyn_st: PRDYN_ST,
    icm_cfg: ICM_CFG,
    uart3_mem_lp_ctrl: UART3_MEM_LP_CTRL,
    _reserved51: [u8; 0x10],
    rmt_mem_lp_ctrl: RMT_MEM_LP_CTRL,
    ledc0_mem_lp_ctrl: LEDC0_MEM_LP_CTRL,
    km_mem_lp_ctrl: KM_MEM_LP_CTRL,
    dma2d_mem_lp_ctrl: DMA2D_MEM_LP_CTRL,
    axi_pdma_mem_lp_ctrl: AXI_PDMA_MEM_LP_CTRL,
    ecc_mem_lp_ctrl: ECC_MEM_LP_CTRL,
    rsa_mem_lp_ctrl: RSA_MEM_LP_CTRL,
    bitscram_mem_lp_ctrl: BITSCRAM_MEM_LP_CTRL,
    _reserved59: [u8; 0x04],
    can0_mem_lp_ctrl: CAN0_MEM_LP_CTRL,
    _reserved60: [u8; 0x08],
    mspi_mem_lp_ctrl: MSPI_MEM_LP_CTRL,
    _reserved61: [u8; 0x14],
    hpcore_mem_lp_ctrl: HPCORE_MEM_LP_CTRL,
    rom_mem_lp_ctrl: ROM_MEM_LP_CTRL,
    _reserved63: [u8; 0x08],
    l1_cache_mem_lp_ctrl: L1_CACHE_MEM_LP_CTRL,
    kyber_mem_lp_ctrl: KYBER_MEM_LP_CTRL,
    lcdcam_mem_lp_ctrl: LCDCAM_MEM_LP_CTRL,
    cpu_peri0_timeout_conf: CPU_PERI0_TIMEOUT_CONF,
    cpu_peri0_timeout_addr: CPU_PERI0_TIMEOUT_ADDR,
    cpu_peri0_timeout_uid: CPU_PERI0_TIMEOUT_UID,
    hp_peri0_timeout_conf: HP_PERI0_TIMEOUT_CONF,
    hp_peri0_timeout_addr: HP_PERI0_TIMEOUT_ADDR,
    hp_peri0_timeout_uid: HP_PERI0_TIMEOUT_UID,
    hp_peri1_timeout_conf: HP_PERI1_TIMEOUT_CONF,
    hp_peri1_timeout_addr: HP_PERI1_TIMEOUT_ADDR,
    hp_peri1_timeout_uid: HP_PERI1_TIMEOUT_UID,
    uart0_mem_lp_ctrl: UART0_MEM_LP_CTRL,
    uart1_mem_lp_ctrl: UART1_MEM_LP_CTRL,
    uart2_mem_lp_ctrl: UART2_MEM_LP_CTRL,
    ledc1_mem_lp_ctrl: LEDC1_MEM_LP_CTRL,
    ppa_mem_lp_ctrl: PPA_MEM_LP_CTRL,
    jpeg_mem_lp_ctrl: JPEG_MEM_LP_CTRL,
    can1_mem_lp_ctrl: CAN1_MEM_LP_CTRL,
    cpu_peri1_timeout_conf: CPU_PERI1_TIMEOUT_CONF,
    cpu_peri1_timeout_addr: CPU_PERI1_TIMEOUT_ADDR,
    cpu_peri1_timeout_uid: CPU_PERI1_TIMEOUT_UID,
    cpu_acs_cache_mem_conf: CPU_ACS_CACHE_MEM_CONF,
    _reserved86: [u8; 0x28],
    tcm_performace_ctrl: TCM_PERFORMACE_CTRL,
    sdio_pad_bist_cfg: SDIO_PAD_BIST_CFG,
    sdio_pad_bist_int_raw: SDIO_PAD_BIST_INT_RAW,
    sdio_pad_bist_int_st: SDIO_PAD_BIST_INT_ST,
    sdio_pad_bist_int_ena: SDIO_PAD_BIST_INT_ENA,
    sdio_pad_bist_int_clr: SDIO_PAD_BIST_INT_CLR,
    sdio_pad_bist_st: SDIO_PAD_BIST_ST,
    gmac0_pad_bist_cfg: GMAC0_PAD_BIST_CFG,
    gmac0_pad_bist_int_raw: GMAC0_PAD_BIST_INT_RAW,
    gmac0_pad_bist_int_st: GMAC0_PAD_BIST_INT_ST,
    gmac0_pad_bist_int_ena: GMAC0_PAD_BIST_INT_ENA,
    gmac0_pad_bist_int_clr: GMAC0_PAD_BIST_INT_CLR,
    gmac0_pad_bist_st: GMAC0_PAD_BIST_ST,
    gmac1_pad_bist_cfg: GMAC1_PAD_BIST_CFG,
    gmac1_pad_bist_int_raw: GMAC1_PAD_BIST_INT_RAW,
    gmac1_pad_bist_int_st: GMAC1_PAD_BIST_INT_ST,
    gmac1_pad_bist_int_ena: GMAC1_PAD_BIST_INT_ENA,
    gmac1_pad_bist_int_clr: GMAC1_PAD_BIST_INT_CLR,
    gmac1_pad_bist_st: GMAC1_PAD_BIST_ST,
    sdio_pad_bist_ctrl: SDIO_PAD_BIST_CTRL,
    gmac0_pad_bist_ctrl: GMAC0_PAD_BIST_CTRL,
    gmac1_pad_bist_ctrl: GMAC1_PAD_BIST_CTRL,
    cpu_pwait_mode: CPU_PWAIT_MODE,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn hp_clk_en(&self) -> &HP_CLK_EN {
        &self.hp_clk_en
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn hp_cpu_int_from_cpu_0(&self) -> &HP_CPU_INT_FROM_CPU_0 {
        &self.hp_cpu_int_from_cpu_0
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn hp_cpu_int_from_cpu_1(&self) -> &HP_CPU_INT_FROM_CPU_1 {
        &self.hp_cpu_int_from_cpu_1
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn hp_cpu_int_from_cpu_2(&self) -> &HP_CPU_INT_FROM_CPU_2 {
        &self.hp_cpu_int_from_cpu_2
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn hp_cpu_int_from_cpu_3(&self) -> &HP_CPU_INT_FROM_CPU_3 {
        &self.hp_cpu_int_from_cpu_3
    }
    #[doc = "0x28 - NA"]
    #[inline(always)]
    pub const fn hp_modem_diag_en(&self) -> &HP_MODEM_DIAG_EN {
        &self.hp_modem_diag_en
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn hp_tcm_ram_pwr_ctrl0(&self) -> &HP_TCM_RAM_PWR_CTRL0 {
        &self.hp_tcm_ram_pwr_ctrl0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn hp_rom_pwr_ctrl0(&self) -> &HP_ROM_PWR_CTRL0 {
        &self.hp_rom_pwr_ctrl0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn hp_probea_ctrl(&self) -> &HP_PROBEA_CTRL {
        &self.hp_probea_ctrl
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn hp_probeb_ctrl(&self) -> &HP_PROBEB_CTRL {
        &self.hp_probeb_ctrl
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn hp_probe_out(&self) -> &HP_PROBE_OUT {
        &self.hp_probe_out
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn hp_cpu_corestalled_st(&self) -> &HP_CPU_CORESTALLED_ST {
        &self.hp_cpu_corestalled_st
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn hp_crypto_ctrl(&self) -> &HP_CRYPTO_CTRL {
        &self.hp_crypto_ctrl
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn hp_iomux_fpga_debug(&self) -> &HP_IOMUX_FPGA_DEBUG {
        &self.hp_iomux_fpga_debug
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn hp_tcm_rdn_eco_cs(&self) -> &HP_TCM_RDN_ECO_CS {
        &self.hp_tcm_rdn_eco_cs
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn hp_tcm_rdn_eco_low(&self) -> &HP_TCM_RDN_ECO_LOW {
        &self.hp_tcm_rdn_eco_low
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn hp_tcm_rdn_eco_high(&self) -> &HP_TCM_RDN_ECO_HIGH {
        &self.hp_tcm_rdn_eco_high
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn hp_cache_pwr_ctrl(&self) -> &HP_CACHE_PWR_CTRL {
        &self.hp_cache_pwr_ctrl
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_data_dump_ctrl(&self) -> &HP_TCM_DATA_DUMP_CTRL {
        &self.hp_tcm_data_dump_ctrl
    }
    #[doc = "0x118 - CPU_WAITI configuration register"]
    #[inline(always)]
    pub const fn hp_cpu_waiti_conf(&self) -> &HP_CPU_WAITI_CONF {
        &self.hp_cpu_waiti_conf
    }
    #[doc = "0x11c - Core Debug runstall configure register"]
    #[inline(always)]
    pub const fn core_debug_runstall_conf(&self) -> &CORE_DEBUG_RUNSTALL_CONF {
        &self.core_debug_runstall_conf
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn hp_core_ibus_timeout_conf(&self) -> &HP_CORE_IBUS_TIMEOUT_CONF {
        &self.hp_core_ibus_timeout_conf
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn hp_core_dbus_timeout_conf(&self) -> &HP_CORE_DBUS_TIMEOUT_CONF {
        &self.hp_core_dbus_timeout_conf
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn hp_icm_h2x_cfg(&self) -> &HP_ICM_H2X_CFG {
        &self.hp_icm_h2x_cfg
    }
    #[doc = "0x140 - Bitscrambler Peri Sel"]
    #[inline(always)]
    pub const fn hp_bitscrambler_peri_sel(&self) -> &HP_BITSCRAMBLER_PERI_SEL {
        &self.hp_bitscrambler_peri_sel
    }
    #[doc = "0x148 - N/A"]
    #[inline(always)]
    pub const fn gdma_ctrl(&self) -> &GDMA_CTRL {
        &self.gdma_ctrl
    }
    #[doc = "0x158 - N/A"]
    #[inline(always)]
    pub const fn vpu_ctrl(&self) -> &VPU_CTRL {
        &self.vpu_ctrl
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn hp_design_for_verification0(&self) -> &HP_DESIGN_FOR_VERIFICATION0 {
        &self.hp_design_for_verification0
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn hp_design_for_verification1(&self) -> &HP_DESIGN_FOR_VERIFICATION1 {
        &self.hp_design_for_verification1
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn hp_ahb2axi_bresp_err_int_raw(&self) -> &HP_AHB2AXI_BRESP_ERR_INT_RAW {
        &self.hp_ahb2axi_bresp_err_int_raw
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn hp_ahb2axi_bresp_err_int_st(&self) -> &HP_AHB2AXI_BRESP_ERR_INT_ST {
        &self.hp_ahb2axi_bresp_err_int_st
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn hp_ahb2axi_bresp_err_int_ena(&self) -> &HP_AHB2AXI_BRESP_ERR_INT_ENA {
        &self.hp_ahb2axi_bresp_err_int_ena
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn hp_ahb2axi_bresp_err_int_clr(&self) -> &HP_AHB2AXI_BRESP_ERR_INT_CLR {
        &self.hp_ahb2axi_bresp_err_int_clr
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn hp_core_dmactive_lpcore(&self) -> &HP_CORE_DMACTIVE_LPCORE {
        &self.hp_core_dmactive_lpcore
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn hp_core_err_resp_dis(&self) -> &HP_CORE_ERR_RESP_DIS {
        &self.hp_core_err_resp_dis
    }
    #[doc = "0x1a8 - Hp core bus timeout interrupt raw register"]
    #[inline(always)]
    pub const fn hp_core_timeout_int_raw(&self) -> &HP_CORE_TIMEOUT_INT_RAW {
        &self.hp_core_timeout_int_raw
    }
    #[doc = "0x1ac - masked interrupt register"]
    #[inline(always)]
    pub const fn hp_core_timeout_int_st(&self) -> &HP_CORE_TIMEOUT_INT_ST {
        &self.hp_core_timeout_int_st
    }
    #[doc = "0x1b0 - masked interrupt register"]
    #[inline(always)]
    pub const fn hp_core_timeout_int_ena(&self) -> &HP_CORE_TIMEOUT_INT_ENA {
        &self.hp_core_timeout_int_ena
    }
    #[doc = "0x1b4 - interrupt clear register"]
    #[inline(always)]
    pub const fn hp_core_timeout_int_clr(&self) -> &HP_CORE_TIMEOUT_INT_CLR {
        &self.hp_core_timeout_int_clr
    }
    #[doc = "0x1b8 - pad wakeup event register"]
    #[inline(always)]
    pub const fn hp_core_pad_wakeup_event(&self) -> &HP_CORE_PAD_WAKEUP_EVENT {
        &self.hp_core_pad_wakeup_event
    }
    #[doc = "0x1c0 - twai0 timestamp low bit reg"]
    #[inline(always)]
    pub const fn hp_twai0_timestamp_l(&self) -> &HP_TWAI0_TIMESTAMP_L {
        &self.hp_twai0_timestamp_l
    }
    #[doc = "0x1c4 - twai0 timestamp high bit reg"]
    #[inline(always)]
    pub const fn hp_twai0_timestamp_h(&self) -> &HP_TWAI0_TIMESTAMP_H {
        &self.hp_twai0_timestamp_h
    }
    #[doc = "0x1c8 - twai1 timestamp low bit reg"]
    #[inline(always)]
    pub const fn hp_twai1_timestamp_l(&self) -> &HP_TWAI1_TIMESTAMP_L {
        &self.hp_twai1_timestamp_l
    }
    #[doc = "0x1cc - twai1 timestamp high bit reg"]
    #[inline(always)]
    pub const fn hp_twai1_timestamp_h(&self) -> &HP_TWAI1_TIMESTAMP_H {
        &self.hp_twai1_timestamp_h
    }
    #[doc = "0x1d8 - rng cfg register"]
    #[inline(always)]
    pub const fn hp_rng_cfg(&self) -> &HP_RNG_CFG {
        &self.hp_rng_cfg
    }
    #[doc = "0x1e0 - hp peri mem clk force on regpster"]
    #[inline(always)]
    pub const fn hp_peri_mem_clk_force_on(&self) -> &HP_PERI_MEM_CLK_FORCE_ON {
        &self.hp_peri_mem_clk_force_on
    }
    #[doc = "0x1e8 - hp system prdyn status register"]
    #[inline(always)]
    pub const fn prdyn_st(&self) -> &PRDYN_ST {
        &self.prdyn_st
    }
    #[doc = "0x1ec - hp system axi icm ctrl register"]
    #[inline(always)]
    pub const fn icm_cfg(&self) -> &ICM_CFG {
        &self.icm_cfg
    }
    #[doc = "0x1f0 - uart memory power control register"]
    #[inline(always)]
    pub const fn uart3_mem_lp_ctrl(&self) -> &UART3_MEM_LP_CTRL {
        &self.uart3_mem_lp_ctrl
    }
    #[doc = "0x204 - rmt memory power control register"]
    #[inline(always)]
    pub const fn rmt_mem_lp_ctrl(&self) -> &RMT_MEM_LP_CTRL {
        &self.rmt_mem_lp_ctrl
    }
    #[doc = "0x208 - ledc0 memory power control register"]
    #[inline(always)]
    pub const fn ledc0_mem_lp_ctrl(&self) -> &LEDC0_MEM_LP_CTRL {
        &self.ledc0_mem_lp_ctrl
    }
    #[doc = "0x20c - key manager memory power control register"]
    #[inline(always)]
    pub const fn km_mem_lp_ctrl(&self) -> &KM_MEM_LP_CTRL {
        &self.km_mem_lp_ctrl
    }
    #[doc = "0x210 - vpu memory power control register"]
    #[inline(always)]
    pub const fn dma2d_mem_lp_ctrl(&self) -> &DMA2D_MEM_LP_CTRL {
        &self.dma2d_mem_lp_ctrl
    }
    #[doc = "0x214 - vpu memory power control register"]
    #[inline(always)]
    pub const fn axi_pdma_mem_lp_ctrl(&self) -> &AXI_PDMA_MEM_LP_CTRL {
        &self.axi_pdma_mem_lp_ctrl
    }
    #[doc = "0x218 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn ecc_mem_lp_ctrl(&self) -> &ECC_MEM_LP_CTRL {
        &self.ecc_mem_lp_ctrl
    }
    #[doc = "0x21c - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn rsa_mem_lp_ctrl(&self) -> &RSA_MEM_LP_CTRL {
        &self.rsa_mem_lp_ctrl
    }
    #[doc = "0x220 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn bitscram_mem_lp_ctrl(&self) -> &BITSCRAM_MEM_LP_CTRL {
        &self.bitscram_mem_lp_ctrl
    }
    #[doc = "0x228 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn can0_mem_lp_ctrl(&self) -> &CAN0_MEM_LP_CTRL {
        &self.can0_mem_lp_ctrl
    }
    #[doc = "0x234 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn mspi_mem_lp_ctrl(&self) -> &MSPI_MEM_LP_CTRL {
        &self.mspi_mem_lp_ctrl
    }
    #[doc = "0x24c - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn hpcore_mem_lp_ctrl(&self) -> &HPCORE_MEM_LP_CTRL {
        &self.hpcore_mem_lp_ctrl
    }
    #[doc = "0x250 - rom power control register"]
    #[inline(always)]
    pub const fn rom_mem_lp_ctrl(&self) -> &ROM_MEM_LP_CTRL {
        &self.rom_mem_lp_ctrl
    }
    #[doc = "0x25c - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn l1_cache_mem_lp_ctrl(&self) -> &L1_CACHE_MEM_LP_CTRL {
        &self.l1_cache_mem_lp_ctrl
    }
    #[doc = "0x260 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn kyber_mem_lp_ctrl(&self) -> &KYBER_MEM_LP_CTRL {
        &self.kyber_mem_lp_ctrl
    }
    #[doc = "0x264 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn lcdcam_mem_lp_ctrl(&self) -> &LCDCAM_MEM_LP_CTRL {
        &self.lcdcam_mem_lp_ctrl
    }
    #[doc = "0x268 - CPU_PERI0_TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn cpu_peri0_timeout_conf(&self) -> &CPU_PERI0_TIMEOUT_CONF {
        &self.cpu_peri0_timeout_conf
    }
    #[doc = "0x26c - CPU_PERI0_TIMEOUT_ADDR register"]
    #[inline(always)]
    pub const fn cpu_peri0_timeout_addr(&self) -> &CPU_PERI0_TIMEOUT_ADDR {
        &self.cpu_peri0_timeout_addr
    }
    #[doc = "0x270 - CPU_PERI0_TIMEOUT_UID register"]
    #[inline(always)]
    pub const fn cpu_peri0_timeout_uid(&self) -> &CPU_PERI0_TIMEOUT_UID {
        &self.cpu_peri0_timeout_uid
    }
    #[doc = "0x274 - HP_PERI0_TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn hp_peri0_timeout_conf(&self) -> &HP_PERI0_TIMEOUT_CONF {
        &self.hp_peri0_timeout_conf
    }
    #[doc = "0x278 - HP_PERI0_TIMEOUT_ADDR register"]
    #[inline(always)]
    pub const fn hp_peri0_timeout_addr(&self) -> &HP_PERI0_TIMEOUT_ADDR {
        &self.hp_peri0_timeout_addr
    }
    #[doc = "0x27c - HP_PERI0_TIMEOUT_UID register"]
    #[inline(always)]
    pub const fn hp_peri0_timeout_uid(&self) -> &HP_PERI0_TIMEOUT_UID {
        &self.hp_peri0_timeout_uid
    }
    #[doc = "0x280 - HP_PERI1_TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn hp_peri1_timeout_conf(&self) -> &HP_PERI1_TIMEOUT_CONF {
        &self.hp_peri1_timeout_conf
    }
    #[doc = "0x284 - HP_PERI1_TIMEOUT_ADDR register"]
    #[inline(always)]
    pub const fn hp_peri1_timeout_addr(&self) -> &HP_PERI1_TIMEOUT_ADDR {
        &self.hp_peri1_timeout_addr
    }
    #[doc = "0x288 - HP_PERI1_TIMEOUT_UID register"]
    #[inline(always)]
    pub const fn hp_peri1_timeout_uid(&self) -> &HP_PERI1_TIMEOUT_UID {
        &self.hp_peri1_timeout_uid
    }
    #[doc = "0x28c - uart memory power control register"]
    #[inline(always)]
    pub const fn uart0_mem_lp_ctrl(&self) -> &UART0_MEM_LP_CTRL {
        &self.uart0_mem_lp_ctrl
    }
    #[doc = "0x290 - uart memory power control register"]
    #[inline(always)]
    pub const fn uart1_mem_lp_ctrl(&self) -> &UART1_MEM_LP_CTRL {
        &self.uart1_mem_lp_ctrl
    }
    #[doc = "0x294 - uart memory power control register"]
    #[inline(always)]
    pub const fn uart2_mem_lp_ctrl(&self) -> &UART2_MEM_LP_CTRL {
        &self.uart2_mem_lp_ctrl
    }
    #[doc = "0x298 - ledc1 memory power control register"]
    #[inline(always)]
    pub const fn ledc1_mem_lp_ctrl(&self) -> &LEDC1_MEM_LP_CTRL {
        &self.ledc1_mem_lp_ctrl
    }
    #[doc = "0x29c - vpu memory power control register"]
    #[inline(always)]
    pub const fn ppa_mem_lp_ctrl(&self) -> &PPA_MEM_LP_CTRL {
        &self.ppa_mem_lp_ctrl
    }
    #[doc = "0x2a0 - vpu memory power control register"]
    #[inline(always)]
    pub const fn jpeg_mem_lp_ctrl(&self) -> &JPEG_MEM_LP_CTRL {
        &self.jpeg_mem_lp_ctrl
    }
    #[doc = "0x2a4 - HP CORE0 & HP CORE1 memory power control register"]
    #[inline(always)]
    pub const fn can1_mem_lp_ctrl(&self) -> &CAN1_MEM_LP_CTRL {
        &self.can1_mem_lp_ctrl
    }
    #[doc = "0x2a8 - CPU_PERI1_TIMEOUT configuration register"]
    #[inline(always)]
    pub const fn cpu_peri1_timeout_conf(&self) -> &CPU_PERI1_TIMEOUT_CONF {
        &self.cpu_peri1_timeout_conf
    }
    #[doc = "0x2ac - CPU_PERI1_TIMEOUT_ADDR register"]
    #[inline(always)]
    pub const fn cpu_peri1_timeout_addr(&self) -> &CPU_PERI1_TIMEOUT_ADDR {
        &self.cpu_peri1_timeout_addr
    }
    #[doc = "0x2b0 - CPU_PERI1_TIMEOUT_UID register"]
    #[inline(always)]
    pub const fn cpu_peri1_timeout_uid(&self) -> &CPU_PERI1_TIMEOUT_UID {
        &self.cpu_peri1_timeout_uid
    }
    #[doc = "0x2b4 - CPU access Cache data mem configuration register"]
    #[inline(always)]
    pub const fn cpu_acs_cache_mem_conf(&self) -> &CPU_ACS_CACHE_MEM_CONF {
        &self.cpu_acs_cache_mem_conf
    }
    #[doc = "0x2e0 - need_des"]
    #[inline(always)]
    pub const fn tcm_performace_ctrl(&self) -> &TCM_PERFORMACE_CTRL {
        &self.tcm_performace_ctrl
    }
    #[doc = "0x2e4 - sdio pad bist control register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_cfg(&self) -> &SDIO_PAD_BIST_CFG {
        &self.sdio_pad_bist_cfg
    }
    #[doc = "0x2e8 - sdio pad bist interupt raw register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_int_raw(&self) -> &SDIO_PAD_BIST_INT_RAW {
        &self.sdio_pad_bist_int_raw
    }
    #[doc = "0x2ec - masked interrupt register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_int_st(&self) -> &SDIO_PAD_BIST_INT_ST {
        &self.sdio_pad_bist_int_st
    }
    #[doc = "0x2f0 - interrupt enable register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_int_ena(&self) -> &SDIO_PAD_BIST_INT_ENA {
        &self.sdio_pad_bist_int_ena
    }
    #[doc = "0x2f4 - interrupt clear register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_int_clr(&self) -> &SDIO_PAD_BIST_INT_CLR {
        &self.sdio_pad_bist_int_clr
    }
    #[doc = "0x2f8 - sdio pad bist status register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_st(&self) -> &SDIO_PAD_BIST_ST {
        &self.sdio_pad_bist_st
    }
    #[doc = "0x2fc - gmac0 pad bist control register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_cfg(&self) -> &GMAC0_PAD_BIST_CFG {
        &self.gmac0_pad_bist_cfg
    }
    #[doc = "0x300 - gmac0 pad bist interupt raw register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_int_raw(&self) -> &GMAC0_PAD_BIST_INT_RAW {
        &self.gmac0_pad_bist_int_raw
    }
    #[doc = "0x304 - masked interrupt register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_int_st(&self) -> &GMAC0_PAD_BIST_INT_ST {
        &self.gmac0_pad_bist_int_st
    }
    #[doc = "0x308 - interrupt enable register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_int_ena(&self) -> &GMAC0_PAD_BIST_INT_ENA {
        &self.gmac0_pad_bist_int_ena
    }
    #[doc = "0x30c - interrupt clear register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_int_clr(&self) -> &GMAC0_PAD_BIST_INT_CLR {
        &self.gmac0_pad_bist_int_clr
    }
    #[doc = "0x310 - gmac0 pad bist status register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_st(&self) -> &GMAC0_PAD_BIST_ST {
        &self.gmac0_pad_bist_st
    }
    #[doc = "0x314 - gmac1 pad bist control register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_cfg(&self) -> &GMAC1_PAD_BIST_CFG {
        &self.gmac1_pad_bist_cfg
    }
    #[doc = "0x318 - gmac1 pad bist interupt raw register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_int_raw(&self) -> &GMAC1_PAD_BIST_INT_RAW {
        &self.gmac1_pad_bist_int_raw
    }
    #[doc = "0x31c - masked interrupt register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_int_st(&self) -> &GMAC1_PAD_BIST_INT_ST {
        &self.gmac1_pad_bist_int_st
    }
    #[doc = "0x320 - interrupt enable register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_int_ena(&self) -> &GMAC1_PAD_BIST_INT_ENA {
        &self.gmac1_pad_bist_int_ena
    }
    #[doc = "0x324 - interrupt clear register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_int_clr(&self) -> &GMAC1_PAD_BIST_INT_CLR {
        &self.gmac1_pad_bist_int_clr
    }
    #[doc = "0x328 - gmac1 pad bist status register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_st(&self) -> &GMAC1_PAD_BIST_ST {
        &self.gmac1_pad_bist_st
    }
    #[doc = "0x32c - gmac0 pad bist status register"]
    #[inline(always)]
    pub const fn sdio_pad_bist_ctrl(&self) -> &SDIO_PAD_BIST_CTRL {
        &self.sdio_pad_bist_ctrl
    }
    #[doc = "0x330 - gmac0 pad bist status register"]
    #[inline(always)]
    pub const fn gmac0_pad_bist_ctrl(&self) -> &GMAC0_PAD_BIST_CTRL {
        &self.gmac0_pad_bist_ctrl
    }
    #[doc = "0x334 - gmac0 pad bist status register"]
    #[inline(always)]
    pub const fn gmac1_pad_bist_ctrl(&self) -> &GMAC1_PAD_BIST_CTRL {
        &self.gmac1_pad_bist_ctrl
    }
    #[doc = "0x338 - Indicate status of core0 and core1 pwait"]
    #[inline(always)]
    pub const fn cpu_pwait_mode(&self) -> &CPU_PWAIT_MODE {
        &self.cpu_pwait_mode
    }
}
#[doc = "VER_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "NA"]
pub mod ver_date;
#[doc = "HP_CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_clk_en`] module"]
pub type HP_CLK_EN = crate::Reg<hp_clk_en::HP_CLK_EN_SPEC>;
#[doc = "NA"]
pub mod hp_clk_en;
#[doc = "HP_CPU_INT_FROM_CPU_0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_int_from_cpu_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_0`] module"]
pub type HP_CPU_INT_FROM_CPU_0 = crate::Reg<hp_cpu_int_from_cpu_0::HP_CPU_INT_FROM_CPU_0_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_0;
#[doc = "HP_CPU_INT_FROM_CPU_1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_int_from_cpu_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_1`] module"]
pub type HP_CPU_INT_FROM_CPU_1 = crate::Reg<hp_cpu_int_from_cpu_1::HP_CPU_INT_FROM_CPU_1_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_1;
#[doc = "HP_CPU_INT_FROM_CPU_2 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_int_from_cpu_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_2`] module"]
pub type HP_CPU_INT_FROM_CPU_2 = crate::Reg<hp_cpu_int_from_cpu_2::HP_CPU_INT_FROM_CPU_2_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_2;
#[doc = "HP_CPU_INT_FROM_CPU_3 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_int_from_cpu_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_3`] module"]
pub type HP_CPU_INT_FROM_CPU_3 = crate::Reg<hp_cpu_int_from_cpu_3::HP_CPU_INT_FROM_CPU_3_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_3;
#[doc = "HP_MODEM_DIAG_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_diag_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_diag_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_diag_en`] module"]
pub type HP_MODEM_DIAG_EN = crate::Reg<hp_modem_diag_en::HP_MODEM_DIAG_EN_SPEC>;
#[doc = "NA"]
pub mod hp_modem_diag_en;
#[doc = "HP_TCM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_ram_pwr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_ram_pwr_ctrl0`] module"]
pub type HP_TCM_RAM_PWR_CTRL0 = crate::Reg<hp_tcm_ram_pwr_ctrl0::HP_TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_ram_pwr_ctrl0;
#[doc = "HP_ROM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rom_pwr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rom_pwr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rom_pwr_ctrl0`] module"]
pub type HP_ROM_PWR_CTRL0 = crate::Reg<hp_rom_pwr_ctrl0::HP_ROM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_rom_pwr_ctrl0;
#[doc = "HP_PROBEA_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_probea_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_probea_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probea_ctrl`] module"]
pub type HP_PROBEA_CTRL = crate::Reg<hp_probea_ctrl::HP_PROBEA_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_probea_ctrl;
#[doc = "HP_PROBEB_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_probeb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_probeb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probeb_ctrl`] module"]
pub type HP_PROBEB_CTRL = crate::Reg<hp_probeb_ctrl::HP_PROBEB_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_probeb_ctrl;
#[doc = "HP_PROBE_OUT (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_probe_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probe_out`] module"]
pub type HP_PROBE_OUT = crate::Reg<hp_probe_out::HP_PROBE_OUT_SPEC>;
#[doc = "NA"]
pub mod hp_probe_out;
#[doc = "HP_CPU_CORESTALLED_ST (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_corestalled_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_corestalled_st`] module"]
pub type HP_CPU_CORESTALLED_ST = crate::Reg<hp_cpu_corestalled_st::HP_CPU_CORESTALLED_ST_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_corestalled_st;
#[doc = "HP_CRYPTO_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_crypto_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_crypto_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_crypto_ctrl`] module"]
pub type HP_CRYPTO_CTRL = crate::Reg<hp_crypto_ctrl::HP_CRYPTO_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_crypto_ctrl;
#[doc = "HP_IOMUX_FPGA_DEBUG (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_iomux_fpga_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_iomux_fpga_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_iomux_fpga_debug`] module"]
pub type HP_IOMUX_FPGA_DEBUG = crate::Reg<hp_iomux_fpga_debug::HP_IOMUX_FPGA_DEBUG_SPEC>;
#[doc = "NA"]
pub mod hp_iomux_fpga_debug;
#[doc = "RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod rdn_eco_cs;
#[doc = "HP_TCM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_cs`] module"]
pub type HP_TCM_RDN_ECO_CS = crate::Reg<hp_tcm_rdn_eco_cs::HP_TCM_RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_cs;
#[doc = "HP_TCM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_low`] module"]
pub type HP_TCM_RDN_ECO_LOW = crate::Reg<hp_tcm_rdn_eco_low::HP_TCM_RDN_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_low;
#[doc = "HP_TCM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_high`] module"]
pub type HP_TCM_RDN_ECO_HIGH = crate::Reg<hp_tcm_rdn_eco_high::HP_TCM_RDN_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_high;
#[doc = "HP_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cache_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cache_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cache_pwr_ctrl`] module"]
pub type HP_CACHE_PWR_CTRL = crate::Reg<hp_cache_pwr_ctrl::HP_CACHE_PWR_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_cache_pwr_ctrl;
#[doc = "HP_TCM_DATA_DUMP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_data_dump_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_data_dump_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_data_dump_ctrl`] module"]
pub type HP_TCM_DATA_DUMP_CTRL = crate::Reg<hp_tcm_data_dump_ctrl::HP_TCM_DATA_DUMP_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_data_dump_ctrl;
#[doc = "HP_CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_waiti_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpu_waiti_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_waiti_conf`] module"]
pub type HP_CPU_WAITI_CONF = crate::Reg<hp_cpu_waiti_conf::HP_CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod hp_cpu_waiti_conf;
#[doc = "CORE_DEBUG_RUNSTALL_CONF (rw) register accessor: Core Debug runstall configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_debug_runstall_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_debug_runstall_conf`] module"]
pub type CORE_DEBUG_RUNSTALL_CONF =
    crate::Reg<core_debug_runstall_conf::CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Core Debug runstall configure register"]
pub mod core_debug_runstall_conf;
#[doc = "HP_CORE_IBUS_TIMEOUT_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_ibus_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_ibus_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_ibus_timeout_conf`] module"]
pub type HP_CORE_IBUS_TIMEOUT_CONF =
    crate::Reg<hp_core_ibus_timeout_conf::HP_CORE_IBUS_TIMEOUT_CONF_SPEC>;
#[doc = "need_des"]
pub mod hp_core_ibus_timeout_conf;
#[doc = "HP_CORE_DBUS_TIMEOUT_CONF (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_dbus_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_dbus_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_dbus_timeout_conf`] module"]
pub type HP_CORE_DBUS_TIMEOUT_CONF =
    crate::Reg<hp_core_dbus_timeout_conf::HP_CORE_DBUS_TIMEOUT_CONF_SPEC>;
#[doc = "need_des"]
pub mod hp_core_dbus_timeout_conf;
#[doc = "HP_ICM_H2X_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_icm_h2x_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_icm_h2x_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_icm_h2x_cfg`] module"]
pub type HP_ICM_H2X_CFG = crate::Reg<hp_icm_h2x_cfg::HP_ICM_H2X_CFG_SPEC>;
#[doc = "need_des"]
pub mod hp_icm_h2x_cfg;
#[doc = "HP_BITSCRAMBLER_PERI_SEL (rw) register accessor: Bitscrambler Peri Sel\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_bitscrambler_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_bitscrambler_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_bitscrambler_peri_sel`] module"]
pub type HP_BITSCRAMBLER_PERI_SEL =
    crate::Reg<hp_bitscrambler_peri_sel::HP_BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Bitscrambler Peri Sel"]
pub mod hp_bitscrambler_peri_sel;
#[doc = "GDMA_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctrl`] module"]
pub type GDMA_CTRL = crate::Reg<gdma_ctrl::GDMA_CTRL_SPEC>;
#[doc = "N/A"]
pub mod gdma_ctrl;
#[doc = "VPU_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`vpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpu_ctrl`] module"]
pub type VPU_CTRL = crate::Reg<vpu_ctrl::VPU_CTRL_SPEC>;
#[doc = "N/A"]
pub mod vpu_ctrl;
#[doc = "HP_DESIGN_FOR_VERIFICATION0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_design_for_verification0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_design_for_verification0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_design_for_verification0`] module"]
pub type HP_DESIGN_FOR_VERIFICATION0 =
    crate::Reg<hp_design_for_verification0::HP_DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "need_des"]
pub mod hp_design_for_verification0;
#[doc = "HP_DESIGN_FOR_VERIFICATION1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_design_for_verification1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_design_for_verification1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_design_for_verification1`] module"]
pub type HP_DESIGN_FOR_VERIFICATION1 =
    crate::Reg<hp_design_for_verification1::HP_DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "need_des"]
pub mod hp_design_for_verification1;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_raw`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_RAW =
    crate::Reg<hp_ahb2axi_bresp_err_int_raw::HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "NA"]
pub mod hp_ahb2axi_bresp_err_int_raw;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_st`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_ST =
    crate::Reg<hp_ahb2axi_bresp_err_int_st::HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_st;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_ena`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_ENA =
    crate::Reg<hp_ahb2axi_bresp_err_int_ena::HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_ena;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_clr`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_CLR =
    crate::Reg<hp_ahb2axi_bresp_err_int_clr::HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_clr;
#[doc = "HP_CORE_DMACTIVE_LPCORE (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_dmactive_lpcore::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_dmactive_lpcore`] module"]
pub type HP_CORE_DMACTIVE_LPCORE =
    crate::Reg<hp_core_dmactive_lpcore::HP_CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "need_des"]
pub mod hp_core_dmactive_lpcore;
#[doc = "HP_CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_err_resp_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_err_resp_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_err_resp_dis`] module"]
pub type HP_CORE_ERR_RESP_DIS = crate::Reg<hp_core_err_resp_dis::HP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = "need_des"]
pub mod hp_core_err_resp_dis;
#[doc = "HP_CORE_TIMEOUT_INT_RAW (rw) register accessor: Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_timeout_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_timeout_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_raw`] module"]
pub type HP_CORE_TIMEOUT_INT_RAW =
    crate::Reg<hp_core_timeout_int_raw::HP_CORE_TIMEOUT_INT_RAW_SPEC>;
#[doc = "Hp core bus timeout interrupt raw register"]
pub mod hp_core_timeout_int_raw;
#[doc = "HP_CORE_TIMEOUT_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_timeout_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_st`] module"]
pub type HP_CORE_TIMEOUT_INT_ST = crate::Reg<hp_core_timeout_int_st::HP_CORE_TIMEOUT_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod hp_core_timeout_int_st;
#[doc = "HP_CORE_TIMEOUT_INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_timeout_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_timeout_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_ena`] module"]
pub type HP_CORE_TIMEOUT_INT_ENA =
    crate::Reg<hp_core_timeout_int_ena::HP_CORE_TIMEOUT_INT_ENA_SPEC>;
#[doc = "masked interrupt register"]
pub mod hp_core_timeout_int_ena;
#[doc = "HP_CORE_TIMEOUT_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_timeout_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_clr`] module"]
pub type HP_CORE_TIMEOUT_INT_CLR =
    crate::Reg<hp_core_timeout_int_clr::HP_CORE_TIMEOUT_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod hp_core_timeout_int_clr;
#[doc = "HP_CORE_PAD_WAKEUP_EVENT (rw) register accessor: pad wakeup event register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_pad_wakeup_event::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_pad_wakeup_event::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_pad_wakeup_event`] module"]
pub type HP_CORE_PAD_WAKEUP_EVENT =
    crate::Reg<hp_core_pad_wakeup_event::HP_CORE_PAD_WAKEUP_EVENT_SPEC>;
#[doc = "pad wakeup event register"]
pub mod hp_core_pad_wakeup_event;
#[doc = "HP_TWAI0_TIMESTAMP_L (rw) register accessor: twai0 timestamp low bit reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai0_timestamp_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai0_timestamp_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai0_timestamp_l`] module"]
pub type HP_TWAI0_TIMESTAMP_L = crate::Reg<hp_twai0_timestamp_l::HP_TWAI0_TIMESTAMP_L_SPEC>;
#[doc = "twai0 timestamp low bit reg"]
pub mod hp_twai0_timestamp_l;
#[doc = "HP_TWAI0_TIMESTAMP_H (rw) register accessor: twai0 timestamp high bit reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai0_timestamp_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai0_timestamp_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai0_timestamp_h`] module"]
pub type HP_TWAI0_TIMESTAMP_H = crate::Reg<hp_twai0_timestamp_h::HP_TWAI0_TIMESTAMP_H_SPEC>;
#[doc = "twai0 timestamp high bit reg"]
pub mod hp_twai0_timestamp_h;
#[doc = "HP_TWAI1_TIMESTAMP_L (rw) register accessor: twai1 timestamp low bit reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai1_timestamp_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai1_timestamp_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai1_timestamp_l`] module"]
pub type HP_TWAI1_TIMESTAMP_L = crate::Reg<hp_twai1_timestamp_l::HP_TWAI1_TIMESTAMP_L_SPEC>;
#[doc = "twai1 timestamp low bit reg"]
pub mod hp_twai1_timestamp_l;
#[doc = "HP_TWAI1_TIMESTAMP_H (rw) register accessor: twai1 timestamp high bit reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_twai1_timestamp_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_twai1_timestamp_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_twai1_timestamp_h`] module"]
pub type HP_TWAI1_TIMESTAMP_H = crate::Reg<hp_twai1_timestamp_h::HP_TWAI1_TIMESTAMP_H_SPEC>;
#[doc = "twai1 timestamp high bit reg"]
pub mod hp_twai1_timestamp_h;
#[doc = "HP_RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rng_cfg`] module"]
pub type HP_RNG_CFG = crate::Reg<hp_rng_cfg::HP_RNG_CFG_SPEC>;
#[doc = "rng cfg register"]
pub mod hp_rng_cfg;
#[doc = "HP_PERI_MEM_CLK_FORCE_ON (rw) register accessor: hp peri mem clk force on regpster\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri_mem_clk_force_on::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri_mem_clk_force_on::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri_mem_clk_force_on`] module"]
pub type HP_PERI_MEM_CLK_FORCE_ON =
    crate::Reg<hp_peri_mem_clk_force_on::HP_PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "hp peri mem clk force on regpster"]
pub mod hp_peri_mem_clk_force_on;
#[doc = "PRDYN_ST (r) register accessor: hp system prdyn status register\n\nYou can [`read`](crate::Reg::read) this register and get [`prdyn_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prdyn_st`] module"]
pub type PRDYN_ST = crate::Reg<prdyn_st::PRDYN_ST_SPEC>;
#[doc = "hp system prdyn status register"]
pub mod prdyn_st;
#[doc = "ICM_CFG (rw) register accessor: hp system axi icm ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_cfg`] module"]
pub type ICM_CFG = crate::Reg<icm_cfg::ICM_CFG_SPEC>;
#[doc = "hp system axi icm ctrl register"]
pub mod icm_cfg;
#[doc = "UART3_MEM_LP_CTRL (rw) register accessor: uart memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart3_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart3_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart3_mem_lp_ctrl`] module"]
pub type UART3_MEM_LP_CTRL = crate::Reg<uart3_mem_lp_ctrl::UART3_MEM_LP_CTRL_SPEC>;
#[doc = "uart memory power control register"]
pub mod uart3_mem_lp_ctrl;
#[doc = "RMT_MEM_LP_CTRL (rw) register accessor: rmt memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmt_mem_lp_ctrl`] module"]
pub type RMT_MEM_LP_CTRL = crate::Reg<rmt_mem_lp_ctrl::RMT_MEM_LP_CTRL_SPEC>;
#[doc = "rmt memory power control register"]
pub mod rmt_mem_lp_ctrl;
#[doc = "LEDC0_MEM_LP_CTRL (rw) register accessor: ledc0 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc0_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc0_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc0_mem_lp_ctrl`] module"]
pub type LEDC0_MEM_LP_CTRL = crate::Reg<ledc0_mem_lp_ctrl::LEDC0_MEM_LP_CTRL_SPEC>;
#[doc = "ledc0 memory power control register"]
pub mod ledc0_mem_lp_ctrl;
#[doc = "KM_MEM_LP_CTRL (rw) register accessor: key manager memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`km_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`km_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@km_mem_lp_ctrl`] module"]
pub type KM_MEM_LP_CTRL = crate::Reg<km_mem_lp_ctrl::KM_MEM_LP_CTRL_SPEC>;
#[doc = "key manager memory power control register"]
pub mod km_mem_lp_ctrl;
#[doc = "DMA2D_MEM_LP_CTRL (rw) register accessor: vpu memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2d_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2d_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2d_mem_lp_ctrl`] module"]
pub type DMA2D_MEM_LP_CTRL = crate::Reg<dma2d_mem_lp_ctrl::DMA2D_MEM_LP_CTRL_SPEC>;
#[doc = "vpu memory power control register"]
pub mod dma2d_mem_lp_ctrl;
#[doc = "AXI_PDMA_MEM_LP_CTRL (rw) register accessor: vpu memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_pdma_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_pdma_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_pdma_mem_lp_ctrl`] module"]
pub type AXI_PDMA_MEM_LP_CTRL = crate::Reg<axi_pdma_mem_lp_ctrl::AXI_PDMA_MEM_LP_CTRL_SPEC>;
#[doc = "vpu memory power control register"]
pub mod axi_pdma_mem_lp_ctrl;
#[doc = "ECC_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_mem_lp_ctrl`] module"]
pub type ECC_MEM_LP_CTRL = crate::Reg<ecc_mem_lp_ctrl::ECC_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod ecc_mem_lp_ctrl;
#[doc = "RSA_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_mem_lp_ctrl`] module"]
pub type RSA_MEM_LP_CTRL = crate::Reg<rsa_mem_lp_ctrl::RSA_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod rsa_mem_lp_ctrl;
#[doc = "BITSCRAM_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bitscram_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitscram_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitscram_mem_lp_ctrl`] module"]
pub type BITSCRAM_MEM_LP_CTRL = crate::Reg<bitscram_mem_lp_ctrl::BITSCRAM_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod bitscram_mem_lp_ctrl;
#[doc = "CAN0_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`can0_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can0_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0_mem_lp_ctrl`] module"]
pub type CAN0_MEM_LP_CTRL = crate::Reg<can0_mem_lp_ctrl::CAN0_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod can0_mem_lp_ctrl;
#[doc = "MSPI_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspi_mem_lp_ctrl`] module"]
pub type MSPI_MEM_LP_CTRL = crate::Reg<mspi_mem_lp_ctrl::MSPI_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod mspi_mem_lp_ctrl;
#[doc = "HPCORE_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore_mem_lp_ctrl`] module"]
pub type HPCORE_MEM_LP_CTRL = crate::Reg<hpcore_mem_lp_ctrl::HPCORE_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod hpcore_mem_lp_ctrl;
#[doc = "ROM_MEM_LP_CTRL (rw) register accessor: rom power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_mem_lp_ctrl`] module"]
pub type ROM_MEM_LP_CTRL = crate::Reg<rom_mem_lp_ctrl::ROM_MEM_LP_CTRL_SPEC>;
#[doc = "rom power control register"]
pub mod rom_mem_lp_ctrl;
#[doc = "L1_CACHE_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_cache_mem_lp_ctrl`] module"]
pub type L1_CACHE_MEM_LP_CTRL = crate::Reg<l1_cache_mem_lp_ctrl::L1_CACHE_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod l1_cache_mem_lp_ctrl;
#[doc = "KYBER_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`kyber_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kyber_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kyber_mem_lp_ctrl`] module"]
pub type KYBER_MEM_LP_CTRL = crate::Reg<kyber_mem_lp_ctrl::KYBER_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod kyber_mem_lp_ctrl;
#[doc = "LCDCAM_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcam_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcam_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcam_mem_lp_ctrl`] module"]
pub type LCDCAM_MEM_LP_CTRL = crate::Reg<lcdcam_mem_lp_ctrl::LCDCAM_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod lcdcam_mem_lp_ctrl;
#[doc = "CPU_PERI0_TIMEOUT_CONF (rw) register accessor: CPU_PERI0_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri0_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri0_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri0_timeout_conf`] module"]
pub type CPU_PERI0_TIMEOUT_CONF = crate::Reg<cpu_peri0_timeout_conf::CPU_PERI0_TIMEOUT_CONF_SPEC>;
#[doc = "CPU_PERI0_TIMEOUT configuration register"]
pub mod cpu_peri0_timeout_conf;
#[doc = "CPU_PERI0_TIMEOUT_ADDR (r) register accessor: CPU_PERI0_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri0_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri0_timeout_addr`] module"]
pub type CPU_PERI0_TIMEOUT_ADDR = crate::Reg<cpu_peri0_timeout_addr::CPU_PERI0_TIMEOUT_ADDR_SPEC>;
#[doc = "CPU_PERI0_TIMEOUT_ADDR register"]
pub mod cpu_peri0_timeout_addr;
#[doc = "CPU_PERI0_TIMEOUT_UID (r) register accessor: CPU_PERI0_TIMEOUT_UID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri0_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri0_timeout_uid`] module"]
pub type CPU_PERI0_TIMEOUT_UID = crate::Reg<cpu_peri0_timeout_uid::CPU_PERI0_TIMEOUT_UID_SPEC>;
#[doc = "CPU_PERI0_TIMEOUT_UID register"]
pub mod cpu_peri0_timeout_uid;
#[doc = "HP_PERI0_TIMEOUT_CONF (rw) register accessor: HP_PERI0_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri0_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_timeout_conf`] module"]
pub type HP_PERI0_TIMEOUT_CONF = crate::Reg<hp_peri0_timeout_conf::HP_PERI0_TIMEOUT_CONF_SPEC>;
#[doc = "HP_PERI0_TIMEOUT configuration register"]
pub mod hp_peri0_timeout_conf;
#[doc = "HP_PERI0_TIMEOUT_ADDR (r) register accessor: HP_PERI0_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_timeout_addr`] module"]
pub type HP_PERI0_TIMEOUT_ADDR = crate::Reg<hp_peri0_timeout_addr::HP_PERI0_TIMEOUT_ADDR_SPEC>;
#[doc = "HP_PERI0_TIMEOUT_ADDR register"]
pub mod hp_peri0_timeout_addr;
#[doc = "HP_PERI0_TIMEOUT_UID (r) register accessor: HP_PERI0_TIMEOUT_UID register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri0_timeout_uid`] module"]
pub type HP_PERI0_TIMEOUT_UID = crate::Reg<hp_peri0_timeout_uid::HP_PERI0_TIMEOUT_UID_SPEC>;
#[doc = "HP_PERI0_TIMEOUT_UID register"]
pub mod hp_peri0_timeout_uid;
#[doc = "HP_PERI1_TIMEOUT_CONF (rw) register accessor: HP_PERI1_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri1_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_timeout_conf`] module"]
pub type HP_PERI1_TIMEOUT_CONF = crate::Reg<hp_peri1_timeout_conf::HP_PERI1_TIMEOUT_CONF_SPEC>;
#[doc = "HP_PERI1_TIMEOUT configuration register"]
pub mod hp_peri1_timeout_conf;
#[doc = "HP_PERI1_TIMEOUT_ADDR (r) register accessor: HP_PERI1_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_timeout_addr`] module"]
pub type HP_PERI1_TIMEOUT_ADDR = crate::Reg<hp_peri1_timeout_addr::HP_PERI1_TIMEOUT_ADDR_SPEC>;
#[doc = "HP_PERI1_TIMEOUT_ADDR register"]
pub mod hp_peri1_timeout_addr;
#[doc = "HP_PERI1_TIMEOUT_UID (r) register accessor: HP_PERI1_TIMEOUT_UID register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri1_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_timeout_uid`] module"]
pub type HP_PERI1_TIMEOUT_UID = crate::Reg<hp_peri1_timeout_uid::HP_PERI1_TIMEOUT_UID_SPEC>;
#[doc = "HP_PERI1_TIMEOUT_UID register"]
pub mod hp_peri1_timeout_uid;
#[doc = "UART0_MEM_LP_CTRL (rw) register accessor: uart memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart0_mem_lp_ctrl`] module"]
pub type UART0_MEM_LP_CTRL = crate::Reg<uart0_mem_lp_ctrl::UART0_MEM_LP_CTRL_SPEC>;
#[doc = "uart memory power control register"]
pub mod uart0_mem_lp_ctrl;
#[doc = "UART1_MEM_LP_CTRL (rw) register accessor: uart memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart1_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart1_mem_lp_ctrl`] module"]
pub type UART1_MEM_LP_CTRL = crate::Reg<uart1_mem_lp_ctrl::UART1_MEM_LP_CTRL_SPEC>;
#[doc = "uart memory power control register"]
pub mod uart1_mem_lp_ctrl;
#[doc = "UART2_MEM_LP_CTRL (rw) register accessor: uart memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_mem_lp_ctrl`] module"]
pub type UART2_MEM_LP_CTRL = crate::Reg<uart2_mem_lp_ctrl::UART2_MEM_LP_CTRL_SPEC>;
#[doc = "uart memory power control register"]
pub mod uart2_mem_lp_ctrl;
#[doc = "LEDC1_MEM_LP_CTRL (rw) register accessor: ledc1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc1_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc1_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ledc1_mem_lp_ctrl`] module"]
pub type LEDC1_MEM_LP_CTRL = crate::Reg<ledc1_mem_lp_ctrl::LEDC1_MEM_LP_CTRL_SPEC>;
#[doc = "ledc1 memory power control register"]
pub mod ledc1_mem_lp_ctrl;
#[doc = "PPA_MEM_LP_CTRL (rw) register accessor: vpu memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppa_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppa_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppa_mem_lp_ctrl`] module"]
pub type PPA_MEM_LP_CTRL = crate::Reg<ppa_mem_lp_ctrl::PPA_MEM_LP_CTRL_SPEC>;
#[doc = "vpu memory power control register"]
pub mod ppa_mem_lp_ctrl;
#[doc = "JPEG_MEM_LP_CTRL (rw) register accessor: vpu memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jpeg_mem_lp_ctrl`] module"]
pub type JPEG_MEM_LP_CTRL = crate::Reg<jpeg_mem_lp_ctrl::JPEG_MEM_LP_CTRL_SPEC>;
#[doc = "vpu memory power control register"]
pub mod jpeg_mem_lp_ctrl;
#[doc = "CAN1_MEM_LP_CTRL (rw) register accessor: HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`can1_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1_mem_lp_ctrl`] module"]
pub type CAN1_MEM_LP_CTRL = crate::Reg<can1_mem_lp_ctrl::CAN1_MEM_LP_CTRL_SPEC>;
#[doc = "HP CORE0 & HP CORE1 memory power control register"]
pub mod can1_mem_lp_ctrl;
#[doc = "CPU_PERI1_TIMEOUT_CONF (rw) register accessor: CPU_PERI1_TIMEOUT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri1_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri1_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri1_timeout_conf`] module"]
pub type CPU_PERI1_TIMEOUT_CONF = crate::Reg<cpu_peri1_timeout_conf::CPU_PERI1_TIMEOUT_CONF_SPEC>;
#[doc = "CPU_PERI1_TIMEOUT configuration register"]
pub mod cpu_peri1_timeout_conf;
#[doc = "CPU_PERI1_TIMEOUT_ADDR (r) register accessor: CPU_PERI1_TIMEOUT_ADDR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri1_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri1_timeout_addr`] module"]
pub type CPU_PERI1_TIMEOUT_ADDR = crate::Reg<cpu_peri1_timeout_addr::CPU_PERI1_TIMEOUT_ADDR_SPEC>;
#[doc = "CPU_PERI1_TIMEOUT_ADDR register"]
pub mod cpu_peri1_timeout_addr;
#[doc = "CPU_PERI1_TIMEOUT_UID (r) register accessor: CPU_PERI1_TIMEOUT_UID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri1_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_peri1_timeout_uid`] module"]
pub type CPU_PERI1_TIMEOUT_UID = crate::Reg<cpu_peri1_timeout_uid::CPU_PERI1_TIMEOUT_UID_SPEC>;
#[doc = "CPU_PERI1_TIMEOUT_UID register"]
pub mod cpu_peri1_timeout_uid;
#[doc = "CPU_ACS_CACHE_MEM_CONF (rw) register accessor: CPU access Cache data mem configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_acs_cache_mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_acs_cache_mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_acs_cache_mem_conf`] module"]
pub type CPU_ACS_CACHE_MEM_CONF = crate::Reg<cpu_acs_cache_mem_conf::CPU_ACS_CACHE_MEM_CONF_SPEC>;
#[doc = "CPU access Cache data mem configuration register"]
pub mod cpu_acs_cache_mem_conf;
#[doc = "TCM_PERFORMACE_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_performace_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_performace_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_performace_ctrl`] module"]
pub type TCM_PERFORMACE_CTRL = crate::Reg<tcm_performace_ctrl::TCM_PERFORMACE_CTRL_SPEC>;
#[doc = "need_des"]
pub mod tcm_performace_ctrl;
#[doc = "SDIO_PAD_BIST_CFG (w) register accessor: sdio pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_cfg`] module"]
pub type SDIO_PAD_BIST_CFG = crate::Reg<sdio_pad_bist_cfg::SDIO_PAD_BIST_CFG_SPEC>;
#[doc = "sdio pad bist control register"]
pub mod sdio_pad_bist_cfg;
#[doc = "SDIO_PAD_BIST_INT_RAW (rw) register accessor: sdio pad bist interupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_int_raw`] module"]
pub type SDIO_PAD_BIST_INT_RAW = crate::Reg<sdio_pad_bist_int_raw::SDIO_PAD_BIST_INT_RAW_SPEC>;
#[doc = "sdio pad bist interupt raw register"]
pub mod sdio_pad_bist_int_raw;
#[doc = "SDIO_PAD_BIST_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_int_st`] module"]
pub type SDIO_PAD_BIST_INT_ST = crate::Reg<sdio_pad_bist_int_st::SDIO_PAD_BIST_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod sdio_pad_bist_int_st;
#[doc = "SDIO_PAD_BIST_INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_int_ena`] module"]
pub type SDIO_PAD_BIST_INT_ENA = crate::Reg<sdio_pad_bist_int_ena::SDIO_PAD_BIST_INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod sdio_pad_bist_int_ena;
#[doc = "SDIO_PAD_BIST_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_int_clr`] module"]
pub type SDIO_PAD_BIST_INT_CLR = crate::Reg<sdio_pad_bist_int_clr::SDIO_PAD_BIST_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod sdio_pad_bist_int_clr;
#[doc = "SDIO_PAD_BIST_ST (r) register accessor: sdio pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_st`] module"]
pub type SDIO_PAD_BIST_ST = crate::Reg<sdio_pad_bist_st::SDIO_PAD_BIST_ST_SPEC>;
#[doc = "sdio pad bist status register"]
pub mod sdio_pad_bist_st;
#[doc = "GMAC0_PAD_BIST_CFG (w) register accessor: gmac0 pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_cfg`] module"]
pub type GMAC0_PAD_BIST_CFG = crate::Reg<gmac0_pad_bist_cfg::GMAC0_PAD_BIST_CFG_SPEC>;
#[doc = "gmac0 pad bist control register"]
pub mod gmac0_pad_bist_cfg;
#[doc = "GMAC0_PAD_BIST_INT_RAW (rw) register accessor: gmac0 pad bist interupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_int_raw`] module"]
pub type GMAC0_PAD_BIST_INT_RAW = crate::Reg<gmac0_pad_bist_int_raw::GMAC0_PAD_BIST_INT_RAW_SPEC>;
#[doc = "gmac0 pad bist interupt raw register"]
pub mod gmac0_pad_bist_int_raw;
#[doc = "GMAC0_PAD_BIST_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_int_st`] module"]
pub type GMAC0_PAD_BIST_INT_ST = crate::Reg<gmac0_pad_bist_int_st::GMAC0_PAD_BIST_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod gmac0_pad_bist_int_st;
#[doc = "GMAC0_PAD_BIST_INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_int_ena`] module"]
pub type GMAC0_PAD_BIST_INT_ENA = crate::Reg<gmac0_pad_bist_int_ena::GMAC0_PAD_BIST_INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod gmac0_pad_bist_int_ena;
#[doc = "GMAC0_PAD_BIST_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_int_clr`] module"]
pub type GMAC0_PAD_BIST_INT_CLR = crate::Reg<gmac0_pad_bist_int_clr::GMAC0_PAD_BIST_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod gmac0_pad_bist_int_clr;
#[doc = "GMAC0_PAD_BIST_ST (r) register accessor: gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_st`] module"]
pub type GMAC0_PAD_BIST_ST = crate::Reg<gmac0_pad_bist_st::GMAC0_PAD_BIST_ST_SPEC>;
#[doc = "gmac0 pad bist status register"]
pub mod gmac0_pad_bist_st;
#[doc = "GMAC1_PAD_BIST_CFG (w) register accessor: gmac1 pad bist control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_cfg`] module"]
pub type GMAC1_PAD_BIST_CFG = crate::Reg<gmac1_pad_bist_cfg::GMAC1_PAD_BIST_CFG_SPEC>;
#[doc = "gmac1 pad bist control register"]
pub mod gmac1_pad_bist_cfg;
#[doc = "GMAC1_PAD_BIST_INT_RAW (rw) register accessor: gmac1 pad bist interupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac1_pad_bist_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_int_raw`] module"]
pub type GMAC1_PAD_BIST_INT_RAW = crate::Reg<gmac1_pad_bist_int_raw::GMAC1_PAD_BIST_INT_RAW_SPEC>;
#[doc = "gmac1 pad bist interupt raw register"]
pub mod gmac1_pad_bist_int_raw;
#[doc = "GMAC1_PAD_BIST_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac1_pad_bist_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_int_st`] module"]
pub type GMAC1_PAD_BIST_INT_ST = crate::Reg<gmac1_pad_bist_int_st::GMAC1_PAD_BIST_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod gmac1_pad_bist_int_st;
#[doc = "GMAC1_PAD_BIST_INT_ENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac1_pad_bist_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_int_ena`] module"]
pub type GMAC1_PAD_BIST_INT_ENA = crate::Reg<gmac1_pad_bist_int_ena::GMAC1_PAD_BIST_INT_ENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod gmac1_pad_bist_int_ena;
#[doc = "GMAC1_PAD_BIST_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_int_clr`] module"]
pub type GMAC1_PAD_BIST_INT_CLR = crate::Reg<gmac1_pad_bist_int_clr::GMAC1_PAD_BIST_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod gmac1_pad_bist_int_clr;
#[doc = "GMAC1_PAD_BIST_ST (r) register accessor: gmac1 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac1_pad_bist_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_st`] module"]
pub type GMAC1_PAD_BIST_ST = crate::Reg<gmac1_pad_bist_st::GMAC1_PAD_BIST_ST_SPEC>;
#[doc = "gmac1 pad bist status register"]
pub mod gmac1_pad_bist_st;
#[doc = "SDIO_PAD_BIST_CTRL (rw) register accessor: gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_pad_bist_ctrl`] module"]
pub type SDIO_PAD_BIST_CTRL = crate::Reg<sdio_pad_bist_ctrl::SDIO_PAD_BIST_CTRL_SPEC>;
#[doc = "gmac0 pad bist status register"]
pub mod sdio_pad_bist_ctrl;
#[doc = "GMAC0_PAD_BIST_CTRL (rw) register accessor: gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_pad_bist_ctrl`] module"]
pub type GMAC0_PAD_BIST_CTRL = crate::Reg<gmac0_pad_bist_ctrl::GMAC0_PAD_BIST_CTRL_SPEC>;
#[doc = "gmac0 pad bist status register"]
pub mod gmac0_pad_bist_ctrl;
#[doc = "GMAC1_PAD_BIST_CTRL (rw) register accessor: gmac0 pad bist status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac1_pad_bist_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac1_pad_bist_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_pad_bist_ctrl`] module"]
pub type GMAC1_PAD_BIST_CTRL = crate::Reg<gmac1_pad_bist_ctrl::GMAC1_PAD_BIST_CTRL_SPEC>;
#[doc = "gmac0 pad bist status register"]
pub mod gmac1_pad_bist_ctrl;
#[doc = "CPU_PWAIT_MODE (r) register accessor: Indicate status of core0 and core1 pwait\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pwait_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pwait_mode`] module"]
pub type CPU_PWAIT_MODE = crate::Reg<cpu_pwait_mode::CPU_PWAIT_MODE_SPEC>;
#[doc = "Indicate status of core0 and core1 pwait"]
pub mod cpu_pwait_mode;
