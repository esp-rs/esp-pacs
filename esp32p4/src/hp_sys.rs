#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    ver_date: VER_DATE,
    clk_en: CLK_EN,
    _reserved2: [u8; 0x08],
    cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    cache_clk_config: CACHE_CLK_CONFIG,
    cache_reset_config: CACHE_RESET_CONFIG,
    _reserved8: [u8; 0x04],
    dma_addr_ctrl: DMA_ADDR_CTRL,
    _reserved9: [u8; 0x04],
    tcm_ram_wrr_config: TCM_RAM_WRR_CONFIG,
    tcm_sw_parity_bwe_mask: TCM_SW_PARITY_BWE_MASK,
    tcm_ram_pwr_ctrl0: TCM_RAM_PWR_CTRL0,
    l2_rom_pwr_ctrl0: L2_ROM_PWR_CTRL0,
    _reserved13: [u8; 0x0c],
    probea_ctrl: PROBEA_CTRL,
    probeb_ctrl: PROBEB_CTRL,
    _reserved15: [u8; 0x04],
    probe_out: PROBE_OUT,
    l2_mem_ram_pwr_ctrl0: L2_MEM_RAM_PWR_CTRL0,
    cpu_corestalled_st: CPU_CORESTALLED_ST,
    _reserved18: [u8; 0x08],
    crypto_ctrl: CRYPTO_CTRL,
    gpio_o_hold_ctrl0: GPIO_O_HOLD_CTRL0,
    gpio_o_hold_ctrl1: GPIO_O_HOLD_CTRL1,
    rdn_eco_cs: RDN_ECO_CS,
    cache_apb_postw_en: CACHE_APB_POSTW_EN,
    l2_mem_subsize: L2_MEM_SUBSIZE,
    _reserved24: [u8; 0x14],
    l2_mem_int_raw: L2_MEM_INT_RAW,
    l2_mem_int_st: L2_MEM_INT_ST,
    l2_mem_int_ena: L2_MEM_INT_ENA,
    l2_mem_int_clr: L2_MEM_INT_CLR,
    l2_mem_l2_ram_ecc: L2_MEM_L2_RAM_ECC,
    l2_mem_int_record0: L2_MEM_INT_RECORD0,
    l2_mem_int_record1: L2_MEM_INT_RECORD1,
    _reserved31: [u8; 0x0c],
    l2_mem_l2_cache_ecc: L2_MEM_L2_CACHE_ECC,
    l1cache_bus0_id: L1CACHE_BUS0_ID,
    l1cache_bus1_id: L1CACHE_BUS1_ID,
    _reserved34: [u8; 0x08],
    l2_mem_rdn_eco_cs: L2_MEM_RDN_ECO_CS,
    l2_mem_rdn_eco_low: L2_MEM_RDN_ECO_LOW,
    l2_mem_rdn_eco_high: L2_MEM_RDN_ECO_HIGH,
    tcm_rdn_eco_cs: TCM_RDN_ECO_CS,
    tcm_rdn_eco_low: TCM_RDN_ECO_LOW,
    tcm_rdn_eco_high: TCM_RDN_ECO_HIGH,
    gpio_ded_hold_ctrl: GPIO_DED_HOLD_CTRL,
    l2_mem_sw_ecc_bwe_mask: L2_MEM_SW_ECC_BWE_MASK,
    usb20otg_mem_ctrl: USB20OTG_MEM_CTRL,
    tcm_int_raw: TCM_INT_RAW,
    tcm_int_st: TCM_INT_ST,
    tcm_int_ena: TCM_INT_ENA,
    tcm_int_clr: TCM_INT_CLR,
    tcm_parity_int_record: TCM_PARITY_INT_RECORD,
    l1_cache_pwr_ctrl: L1_CACHE_PWR_CTRL,
    l2_cache_pwr_ctrl: L2_CACHE_PWR_CTRL,
    cpu_waiti_conf: CPU_WAITI_CONF,
    core_debug_runstall_conf: CORE_DEBUG_RUNSTALL_CONF,
    core_ahb_timeout: CORE_AHB_TIMEOUT,
    core_ibus_timeout: CORE_IBUS_TIMEOUT,
    core_dbus_timeout: CORE_DBUS_TIMEOUT,
    _reserved55: [u8; 0x0c],
    icm_cpu_h2x_cfg: ICM_CPU_H2X_CFG,
    peri1_apb_postw_en: PERI1_APB_POSTW_EN,
    bitscrambler_peri_sel: BITSCRAMBLER_PERI_SEL,
    apb_sync_postw_en: APB_SYNC_POSTW_EN,
    gdma_ctrl: GDMA_CTRL,
    gmac_ctrl0: GMAC_CTRL0,
    gmac_ctrl1: GMAC_CTRL1,
    gmac_ctrl2: GMAC_CTRL2,
    vpu_ctrl: VPU_CTRL,
    usbotg20_ctrl: USBOTG20_CTRL,
    tcm_err_resp_ctrl: TCM_ERR_RESP_CTRL,
    l2_mem_refresh: L2_MEM_REFRESH,
    tcm_init: TCM_INIT,
    tcm_parity_check_ctrl: TCM_PARITY_CHECK_CTRL,
    design_for_verification0: DESIGN_FOR_VERIFICATION0,
    design_for_verification1: DESIGN_FOR_VERIFICATION1,
    _reserved71: [u8; 0x08],
    psram_flash_addr_interchange: PSRAM_FLASH_ADDR_INTERCHANGE,
    _reserved72: [u8; 0x04],
    ahb2axi_bresp_err_int_raw: AHB2AXI_BRESP_ERR_INT_RAW,
    ahb2axi_bresp_err_int_st: AHB2AXI_BRESP_ERR_INT_ST,
    ahb2axi_bresp_err_int_ena: AHB2AXI_BRESP_ERR_INT_ENA,
    ahb2axi_bresp_err_int_clr: AHB2AXI_BRESP_ERR_INT_CLR,
    l2_mem_err_resp_ctrl: L2_MEM_ERR_RESP_CTRL,
    l2_mem_ahb_buffer_ctrl: L2_MEM_AHB_BUFFER_CTRL,
    core_dmactive_lpcore: CORE_DMACTIVE_LPCORE,
    core_err_resp_dis: CORE_ERR_RESP_DIS,
    core_timeout_int_raw: CORE_TIMEOUT_INT_RAW,
    core_timeout_int_st: CORE_TIMEOUT_INT_ST,
    core_timeout_int_ena: CORE_TIMEOUT_INT_ENA,
    core_timeout_int_clr: CORE_TIMEOUT_INT_CLR,
    _reserved84: [u8; 0x08],
    gpio_o_hys_ctrl0: GPIO_O_HYS_CTRL0,
    gpio_o_hys_ctrl1: GPIO_O_HYS_CTRL1,
    _reserved86: [u8; 0x08],
    rsa_pd_ctrl: RSA_PD_CTRL,
    ecc_pd_ctrl: ECC_PD_CTRL,
    rng_cfg: RNG_CFG,
    uart_pd_ctrl: UART_PD_CTRL,
    peri_mem_clk_force_on: PERI_MEM_CLK_FORCE_ON,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> &CPU_INTR_FROM_CPU_0 {
        &self.cpu_intr_from_cpu_0
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> &CPU_INTR_FROM_CPU_1 {
        &self.cpu_intr_from_cpu_1
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> &CPU_INTR_FROM_CPU_2 {
        &self.cpu_intr_from_cpu_2
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> &CPU_INTR_FROM_CPU_3 {
        &self.cpu_intr_from_cpu_3
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn cache_clk_config(&self) -> &CACHE_CLK_CONFIG {
        &self.cache_clk_config
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn cache_reset_config(&self) -> &CACHE_RESET_CONFIG {
        &self.cache_reset_config
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn dma_addr_ctrl(&self) -> &DMA_ADDR_CTRL {
        &self.dma_addr_ctrl
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn tcm_ram_wrr_config(&self) -> &TCM_RAM_WRR_CONFIG {
        &self.tcm_ram_wrr_config
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn tcm_sw_parity_bwe_mask(&self) -> &TCM_SW_PARITY_BWE_MASK {
        &self.tcm_sw_parity_bwe_mask
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn tcm_ram_pwr_ctrl0(&self) -> &TCM_RAM_PWR_CTRL0 {
        &self.tcm_ram_pwr_ctrl0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn l2_rom_pwr_ctrl0(&self) -> &L2_ROM_PWR_CTRL0 {
        &self.l2_rom_pwr_ctrl0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn probea_ctrl(&self) -> &PROBEA_CTRL {
        &self.probea_ctrl
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn probeb_ctrl(&self) -> &PROBEB_CTRL {
        &self.probeb_ctrl
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn probe_out(&self) -> &PROBE_OUT {
        &self.probe_out
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn l2_mem_ram_pwr_ctrl0(&self) -> &L2_MEM_RAM_PWR_CTRL0 {
        &self.l2_mem_ram_pwr_ctrl0
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn cpu_corestalled_st(&self) -> &CPU_CORESTALLED_ST {
        &self.cpu_corestalled_st
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn crypto_ctrl(&self) -> &CRYPTO_CTRL {
        &self.crypto_ctrl
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hold_ctrl0(&self) -> &GPIO_O_HOLD_CTRL0 {
        &self.gpio_o_hold_ctrl0
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hold_ctrl1(&self) -> &GPIO_O_HOLD_CTRL1 {
        &self.gpio_o_hold_ctrl1
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn cache_apb_postw_en(&self) -> &CACHE_APB_POSTW_EN {
        &self.cache_apb_postw_en
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn l2_mem_subsize(&self) -> &L2_MEM_SUBSIZE {
        &self.l2_mem_subsize
    }
    #[doc = "0x9c - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_raw(&self) -> &L2_MEM_INT_RAW {
        &self.l2_mem_int_raw
    }
    #[doc = "0xa0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_st(&self) -> &L2_MEM_INT_ST {
        &self.l2_mem_int_st
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_ena(&self) -> &L2_MEM_INT_ENA {
        &self.l2_mem_int_ena
    }
    #[doc = "0xa8 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_clr(&self) -> &L2_MEM_INT_CLR {
        &self.l2_mem_int_clr
    }
    #[doc = "0xac - NA"]
    #[inline(always)]
    pub const fn l2_mem_l2_ram_ecc(&self) -> &L2_MEM_L2_RAM_ECC {
        &self.l2_mem_l2_ram_ecc
    }
    #[doc = "0xb0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_record0(&self) -> &L2_MEM_INT_RECORD0 {
        &self.l2_mem_int_record0
    }
    #[doc = "0xb4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_int_record1(&self) -> &L2_MEM_INT_RECORD1 {
        &self.l2_mem_int_record1
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_l2_cache_ecc(&self) -> &L2_MEM_L2_CACHE_ECC {
        &self.l2_mem_l2_cache_ecc
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn l1cache_bus0_id(&self) -> &L1CACHE_BUS0_ID {
        &self.l1cache_bus0_id
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn l1cache_bus1_id(&self) -> &L1CACHE_BUS1_ID {
        &self.l1cache_bus1_id
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_cs(&self) -> &L2_MEM_RDN_ECO_CS {
        &self.l2_mem_rdn_eco_cs
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_low(&self) -> &L2_MEM_RDN_ECO_LOW {
        &self.l2_mem_rdn_eco_low
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn l2_mem_rdn_eco_high(&self) -> &L2_MEM_RDN_ECO_HIGH {
        &self.l2_mem_rdn_eco_high
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_cs(&self) -> &TCM_RDN_ECO_CS {
        &self.tcm_rdn_eco_cs
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_low(&self) -> &TCM_RDN_ECO_LOW {
        &self.tcm_rdn_eco_low
    }
    #[doc = "0xec - NA"]
    #[inline(always)]
    pub const fn tcm_rdn_eco_high(&self) -> &TCM_RDN_ECO_HIGH {
        &self.tcm_rdn_eco_high
    }
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn gpio_ded_hold_ctrl(&self) -> &GPIO_DED_HOLD_CTRL {
        &self.gpio_ded_hold_ctrl
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn l2_mem_sw_ecc_bwe_mask(&self) -> &L2_MEM_SW_ECC_BWE_MASK {
        &self.l2_mem_sw_ecc_bwe_mask
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn usb20otg_mem_ctrl(&self) -> &USB20OTG_MEM_CTRL {
        &self.usb20otg_mem_ctrl
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn tcm_int_raw(&self) -> &TCM_INT_RAW {
        &self.tcm_int_raw
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_st(&self) -> &TCM_INT_ST {
        &self.tcm_int_st
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_ena(&self) -> &TCM_INT_ENA {
        &self.tcm_int_ena
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn tcm_int_clr(&self) -> &TCM_INT_CLR {
        &self.tcm_int_clr
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn tcm_parity_int_record(&self) -> &TCM_PARITY_INT_RECORD {
        &self.tcm_parity_int_record
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn l1_cache_pwr_ctrl(&self) -> &L1_CACHE_PWR_CTRL {
        &self.l1_cache_pwr_ctrl
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn l2_cache_pwr_ctrl(&self) -> &L2_CACHE_PWR_CTRL {
        &self.l2_cache_pwr_ctrl
    }
    #[doc = "0x118 - CPU_WAITI configuration register"]
    #[inline(always)]
    pub const fn cpu_waiti_conf(&self) -> &CPU_WAITI_CONF {
        &self.cpu_waiti_conf
    }
    #[doc = "0x11c - Core Debug runstall configure register"]
    #[inline(always)]
    pub const fn core_debug_runstall_conf(&self) -> &CORE_DEBUG_RUNSTALL_CONF {
        &self.core_debug_runstall_conf
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn core_ahb_timeout(&self) -> &CORE_AHB_TIMEOUT {
        &self.core_ahb_timeout
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn core_ibus_timeout(&self) -> &CORE_IBUS_TIMEOUT {
        &self.core_ibus_timeout
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn core_dbus_timeout(&self) -> &CORE_DBUS_TIMEOUT {
        &self.core_dbus_timeout
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn icm_cpu_h2x_cfg(&self) -> &ICM_CPU_H2X_CFG {
        &self.icm_cpu_h2x_cfg
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn peri1_apb_postw_en(&self) -> &PERI1_APB_POSTW_EN {
        &self.peri1_apb_postw_en
    }
    #[doc = "0x140 - Bitscrambler Peri Sel"]
    #[inline(always)]
    pub const fn bitscrambler_peri_sel(&self) -> &BITSCRAMBLER_PERI_SEL {
        &self.bitscrambler_peri_sel
    }
    #[doc = "0x144 - N/A"]
    #[inline(always)]
    pub const fn apb_sync_postw_en(&self) -> &APB_SYNC_POSTW_EN {
        &self.apb_sync_postw_en
    }
    #[doc = "0x148 - N/A"]
    #[inline(always)]
    pub const fn gdma_ctrl(&self) -> &GDMA_CTRL {
        &self.gdma_ctrl
    }
    #[doc = "0x14c - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl0(&self) -> &GMAC_CTRL0 {
        &self.gmac_ctrl0
    }
    #[doc = "0x150 - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl1(&self) -> &GMAC_CTRL1 {
        &self.gmac_ctrl1
    }
    #[doc = "0x154 - N/A"]
    #[inline(always)]
    pub const fn gmac_ctrl2(&self) -> &GMAC_CTRL2 {
        &self.gmac_ctrl2
    }
    #[doc = "0x158 - N/A"]
    #[inline(always)]
    pub const fn vpu_ctrl(&self) -> &VPU_CTRL {
        &self.vpu_ctrl
    }
    #[doc = "0x15c - N/A"]
    #[inline(always)]
    pub const fn usbotg20_ctrl(&self) -> &USBOTG20_CTRL {
        &self.usbotg20_ctrl
    }
    #[doc = "0x160 - need_des"]
    #[inline(always)]
    pub const fn tcm_err_resp_ctrl(&self) -> &TCM_ERR_RESP_CTRL {
        &self.tcm_err_resp_ctrl
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn l2_mem_refresh(&self) -> &L2_MEM_REFRESH {
        &self.l2_mem_refresh
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn tcm_init(&self) -> &TCM_INIT {
        &self.tcm_init
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn tcm_parity_check_ctrl(&self) -> &TCM_PARITY_CHECK_CTRL {
        &self.tcm_parity_check_ctrl
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn design_for_verification0(&self) -> &DESIGN_FOR_VERIFICATION0 {
        &self.design_for_verification0
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn design_for_verification1(&self) -> &DESIGN_FOR_VERIFICATION1 {
        &self.design_for_verification1
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn psram_flash_addr_interchange(&self) -> &PSRAM_FLASH_ADDR_INTERCHANGE {
        &self.psram_flash_addr_interchange
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_raw(&self) -> &AHB2AXI_BRESP_ERR_INT_RAW {
        &self.ahb2axi_bresp_err_int_raw
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_st(&self) -> &AHB2AXI_BRESP_ERR_INT_ST {
        &self.ahb2axi_bresp_err_int_st
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_ena(&self) -> &AHB2AXI_BRESP_ERR_INT_ENA {
        &self.ahb2axi_bresp_err_int_ena
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn ahb2axi_bresp_err_int_clr(&self) -> &AHB2AXI_BRESP_ERR_INT_CLR {
        &self.ahb2axi_bresp_err_int_clr
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn l2_mem_err_resp_ctrl(&self) -> &L2_MEM_ERR_RESP_CTRL {
        &self.l2_mem_err_resp_ctrl
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn l2_mem_ahb_buffer_ctrl(&self) -> &L2_MEM_AHB_BUFFER_CTRL {
        &self.l2_mem_ahb_buffer_ctrl
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn core_dmactive_lpcore(&self) -> &CORE_DMACTIVE_LPCORE {
        &self.core_dmactive_lpcore
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn core_err_resp_dis(&self) -> &CORE_ERR_RESP_DIS {
        &self.core_err_resp_dis
    }
    #[doc = "0x1a8 - Hp core bus timeout interrupt raw register"]
    #[inline(always)]
    pub const fn core_timeout_int_raw(&self) -> &CORE_TIMEOUT_INT_RAW {
        &self.core_timeout_int_raw
    }
    #[doc = "0x1ac - masked interrupt register"]
    #[inline(always)]
    pub const fn core_timeout_int_st(&self) -> &CORE_TIMEOUT_INT_ST {
        &self.core_timeout_int_st
    }
    #[doc = "0x1b0 - masked interrupt register"]
    #[inline(always)]
    pub const fn core_timeout_int_ena(&self) -> &CORE_TIMEOUT_INT_ENA {
        &self.core_timeout_int_ena
    }
    #[doc = "0x1b4 - interrupt clear register"]
    #[inline(always)]
    pub const fn core_timeout_int_clr(&self) -> &CORE_TIMEOUT_INT_CLR {
        &self.core_timeout_int_clr
    }
    #[doc = "0x1c0 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hys_ctrl0(&self) -> &GPIO_O_HYS_CTRL0 {
        &self.gpio_o_hys_ctrl0
    }
    #[doc = "0x1c4 - NA"]
    #[inline(always)]
    pub const fn gpio_o_hys_ctrl1(&self) -> &GPIO_O_HYS_CTRL1 {
        &self.gpio_o_hys_ctrl1
    }
    #[doc = "0x1d0 - rsa pd ctrl register"]
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    #[doc = "0x1d4 - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn ecc_pd_ctrl(&self) -> &ECC_PD_CTRL {
        &self.ecc_pd_ctrl
    }
    #[doc = "0x1d8 - rng cfg register"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RNG_CFG {
        &self.rng_cfg
    }
    #[doc = "0x1dc - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn uart_pd_ctrl(&self) -> &UART_PD_CTRL {
        &self.uart_pd_ctrl
    }
    #[doc = "0x1e0 - hp peri mem clk force on regpster"]
    #[inline(always)]
    pub const fn peri_mem_clk_force_on(&self) -> &PERI_MEM_CLK_FORCE_ON {
        &self.peri_mem_clk_force_on
    }
}
#[doc = "VER_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "NA"]
pub mod ver_date;
#[doc = "CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "NA"]
pub mod clk_en;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_0`] module"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "NA"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_1`] module"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "NA"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_2`] module"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "NA"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_intr_from_cpu_3`] module"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "NA"]
pub mod cpu_intr_from_cpu_3;
#[doc = "CACHE_CLK_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_clk_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_clk_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_clk_config`] module"]
pub type CACHE_CLK_CONFIG = crate::Reg<cache_clk_config::CACHE_CLK_CONFIG_SPEC>;
#[doc = "NA"]
pub mod cache_clk_config;
#[doc = "CACHE_RESET_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_reset_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_reset_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_reset_config`] module"]
pub type CACHE_RESET_CONFIG = crate::Reg<cache_reset_config::CACHE_RESET_CONFIG_SPEC>;
#[doc = "NA"]
pub mod cache_reset_config;
#[doc = "DMA_ADDR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_addr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_addr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_addr_ctrl`] module"]
pub type DMA_ADDR_CTRL = crate::Reg<dma_addr_ctrl::DMA_ADDR_CTRL_SPEC>;
#[doc = "NA"]
pub mod dma_addr_ctrl;
#[doc = "TCM_RAM_WRR_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_ram_wrr_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_ram_wrr_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_ram_wrr_config`] module"]
pub type TCM_RAM_WRR_CONFIG = crate::Reg<tcm_ram_wrr_config::TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "NA"]
pub mod tcm_ram_wrr_config;
#[doc = "TCM_SW_PARITY_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_sw_parity_bwe_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_sw_parity_bwe_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_sw_parity_bwe_mask`] module"]
pub type TCM_SW_PARITY_BWE_MASK = crate::Reg<tcm_sw_parity_bwe_mask::TCM_SW_PARITY_BWE_MASK_SPEC>;
#[doc = "NA"]
pub mod tcm_sw_parity_bwe_mask;
#[doc = "TCM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_ram_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_ram_pwr_ctrl0`] module"]
pub type TCM_RAM_PWR_CTRL0 = crate::Reg<tcm_ram_pwr_ctrl0::TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod tcm_ram_pwr_ctrl0;
#[doc = "L2_ROM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_rom_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_rom_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_rom_pwr_ctrl0`] module"]
pub type L2_ROM_PWR_CTRL0 = crate::Reg<l2_rom_pwr_ctrl0::L2_ROM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod l2_rom_pwr_ctrl0;
#[doc = "PROBEA_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probea_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probea_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probea_ctrl`] module"]
pub type PROBEA_CTRL = crate::Reg<probea_ctrl::PROBEA_CTRL_SPEC>;
#[doc = "NA"]
pub mod probea_ctrl;
#[doc = "PROBEB_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probeb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probeb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probeb_ctrl`] module"]
pub type PROBEB_CTRL = crate::Reg<probeb_ctrl::PROBEB_CTRL_SPEC>;
#[doc = "NA"]
pub mod probeb_ctrl;
#[doc = "PROBE_OUT (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_out::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_out`] module"]
pub type PROBE_OUT = crate::Reg<probe_out::PROBE_OUT_SPEC>;
#[doc = "NA"]
pub mod probe_out;
#[doc = "L2_MEM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_ram_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_ram_pwr_ctrl0`] module"]
pub type L2_MEM_RAM_PWR_CTRL0 = crate::Reg<l2_mem_ram_pwr_ctrl0::L2_MEM_RAM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod l2_mem_ram_pwr_ctrl0;
#[doc = "CPU_CORESTALLED_ST (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_corestalled_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_corestalled_st`] module"]
pub type CPU_CORESTALLED_ST = crate::Reg<cpu_corestalled_st::CPU_CORESTALLED_ST_SPEC>;
#[doc = "NA"]
pub mod cpu_corestalled_st;
#[doc = "CRYPTO_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_ctrl`] module"]
pub type CRYPTO_CTRL = crate::Reg<crypto_ctrl::CRYPTO_CTRL_SPEC>;
#[doc = "NA"]
pub mod crypto_ctrl;
#[doc = "GPIO_O_HOLD_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hold_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hold_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hold_ctrl0`] module"]
pub type GPIO_O_HOLD_CTRL0 = crate::Reg<gpio_o_hold_ctrl0::GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "NA"]
pub mod gpio_o_hold_ctrl0;
#[doc = "GPIO_O_HOLD_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hold_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hold_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hold_ctrl1`] module"]
pub type GPIO_O_HOLD_CTRL1 = crate::Reg<gpio_o_hold_ctrl1::GPIO_O_HOLD_CTRL1_SPEC>;
#[doc = "NA"]
pub mod gpio_o_hold_ctrl1;
#[doc = "RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod rdn_eco_cs;
#[doc = "CACHE_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_apb_postw_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_apb_postw_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_apb_postw_en`] module"]
pub type CACHE_APB_POSTW_EN = crate::Reg<cache_apb_postw_en::CACHE_APB_POSTW_EN_SPEC>;
#[doc = "NA"]
pub mod cache_apb_postw_en;
#[doc = "L2_MEM_SUBSIZE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_subsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_subsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_subsize`] module"]
pub type L2_MEM_SUBSIZE = crate::Reg<l2_mem_subsize::L2_MEM_SUBSIZE_SPEC>;
#[doc = "NA"]
pub mod l2_mem_subsize;
#[doc = "L2_MEM_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_raw`] module"]
pub type L2_MEM_INT_RAW = crate::Reg<l2_mem_int_raw::L2_MEM_INT_RAW_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_raw;
#[doc = "L2_MEM_INT_ST (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_st`] module"]
pub type L2_MEM_INT_ST = crate::Reg<l2_mem_int_st::L2_MEM_INT_ST_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_st;
#[doc = "L2_MEM_INT_ENA (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_ena`] module"]
pub type L2_MEM_INT_ENA = crate::Reg<l2_mem_int_ena::L2_MEM_INT_ENA_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_ena;
#[doc = "L2_MEM_INT_CLR (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_clr`] module"]
pub type L2_MEM_INT_CLR = crate::Reg<l2_mem_int_clr::L2_MEM_INT_CLR_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_clr;
#[doc = "L2_MEM_L2_RAM_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_l2_ram_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_l2_ram_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_l2_ram_ecc`] module"]
pub type L2_MEM_L2_RAM_ECC = crate::Reg<l2_mem_l2_ram_ecc::L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "NA"]
pub mod l2_mem_l2_ram_ecc;
#[doc = "L2_MEM_INT_RECORD0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_record0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_record0`] module"]
pub type L2_MEM_INT_RECORD0 = crate::Reg<l2_mem_int_record0::L2_MEM_INT_RECORD0_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_record0;
#[doc = "L2_MEM_INT_RECORD1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_record1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_int_record1`] module"]
pub type L2_MEM_INT_RECORD1 = crate::Reg<l2_mem_int_record1::L2_MEM_INT_RECORD1_SPEC>;
#[doc = "NA"]
pub mod l2_mem_int_record1;
#[doc = "L2_MEM_L2_CACHE_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_l2_cache_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_l2_cache_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_l2_cache_ecc`] module"]
pub type L2_MEM_L2_CACHE_ECC = crate::Reg<l2_mem_l2_cache_ecc::L2_MEM_L2_CACHE_ECC_SPEC>;
#[doc = "NA"]
pub mod l2_mem_l2_cache_ecc;
#[doc = "L1CACHE_BUS0_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cache_bus0_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cache_bus0_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cache_bus0_id`] module"]
pub type L1CACHE_BUS0_ID = crate::Reg<l1cache_bus0_id::L1CACHE_BUS0_ID_SPEC>;
#[doc = "NA"]
pub mod l1cache_bus0_id;
#[doc = "L1CACHE_BUS1_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1cache_bus1_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cache_bus1_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1cache_bus1_id`] module"]
pub type L1CACHE_BUS1_ID = crate::Reg<l1cache_bus1_id::L1CACHE_BUS1_ID_SPEC>;
#[doc = "NA"]
pub mod l1cache_bus1_id;
#[doc = "L2_MEM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_cs`] module"]
pub type L2_MEM_RDN_ECO_CS = crate::Reg<l2_mem_rdn_eco_cs::L2_MEM_RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_cs;
#[doc = "L2_MEM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_low`] module"]
pub type L2_MEM_RDN_ECO_LOW = crate::Reg<l2_mem_rdn_eco_low::L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_low;
#[doc = "L2_MEM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_rdn_eco_high`] module"]
pub type L2_MEM_RDN_ECO_HIGH = crate::Reg<l2_mem_rdn_eco_high::L2_MEM_RDN_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod l2_mem_rdn_eco_high;
#[doc = "TCM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_cs`] module"]
pub type TCM_RDN_ECO_CS = crate::Reg<tcm_rdn_eco_cs::TCM_RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod tcm_rdn_eco_cs;
#[doc = "TCM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_low`] module"]
pub type TCM_RDN_ECO_LOW = crate::Reg<tcm_rdn_eco_low::TCM_RDN_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod tcm_rdn_eco_low;
#[doc = "TCM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_rdn_eco_high`] module"]
pub type TCM_RDN_ECO_HIGH = crate::Reg<tcm_rdn_eco_high::TCM_RDN_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod tcm_rdn_eco_high;
#[doc = "GPIO_DED_HOLD_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_ded_hold_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_ded_hold_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ded_hold_ctrl`] module"]
pub type GPIO_DED_HOLD_CTRL = crate::Reg<gpio_ded_hold_ctrl::GPIO_DED_HOLD_CTRL_SPEC>;
#[doc = "NA"]
pub mod gpio_ded_hold_ctrl;
#[doc = "L2_MEM_SW_ECC_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_sw_ecc_bwe_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_sw_ecc_bwe_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_sw_ecc_bwe_mask`] module"]
pub type L2_MEM_SW_ECC_BWE_MASK = crate::Reg<l2_mem_sw_ecc_bwe_mask::L2_MEM_SW_ECC_BWE_MASK_SPEC>;
#[doc = "NA"]
pub mod l2_mem_sw_ecc_bwe_mask;
#[doc = "USB20OTG_MEM_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20otg_mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20otg_mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20otg_mem_ctrl`] module"]
pub type USB20OTG_MEM_CTRL = crate::Reg<usb20otg_mem_ctrl::USB20OTG_MEM_CTRL_SPEC>;
#[doc = "NA"]
pub mod usb20otg_mem_ctrl;
#[doc = "TCM_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_raw`] module"]
pub type TCM_INT_RAW = crate::Reg<tcm_int_raw::TCM_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod tcm_int_raw;
#[doc = "TCM_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_st`] module"]
pub type TCM_INT_ST = crate::Reg<tcm_int_st::TCM_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod tcm_int_st;
#[doc = "TCM_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_ena`] module"]
pub type TCM_INT_ENA = crate::Reg<tcm_int_ena::TCM_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod tcm_int_ena;
#[doc = "TCM_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_int_clr`] module"]
pub type TCM_INT_CLR = crate::Reg<tcm_int_clr::TCM_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod tcm_int_clr;
#[doc = "TCM_PARITY_INT_RECORD (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_parity_int_record::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_int_record`] module"]
pub type TCM_PARITY_INT_RECORD = crate::Reg<tcm_parity_int_record::TCM_PARITY_INT_RECORD_SPEC>;
#[doc = "need_des"]
pub mod tcm_parity_int_record;
#[doc = "L1_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_pwr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_pwr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1_cache_pwr_ctrl`] module"]
pub type L1_CACHE_PWR_CTRL = crate::Reg<l1_cache_pwr_ctrl::L1_CACHE_PWR_CTRL_SPEC>;
#[doc = "NA"]
pub mod l1_cache_pwr_ctrl;
#[doc = "L2_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_pwr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_pwr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_cache_pwr_ctrl`] module"]
pub type L2_CACHE_PWR_CTRL = crate::Reg<l2_cache_pwr_ctrl::L2_CACHE_PWR_CTRL_SPEC>;
#[doc = "NA"]
pub mod l2_cache_pwr_ctrl;
#[doc = "CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_waiti_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_waiti_conf`] module"]
pub type CPU_WAITI_CONF = crate::Reg<cpu_waiti_conf::CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod cpu_waiti_conf;
#[doc = "CORE_DEBUG_RUNSTALL_CONF (rw) register accessor: Core Debug runstall configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_debug_runstall_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_debug_runstall_conf`] module"]
pub type CORE_DEBUG_RUNSTALL_CONF =
    crate::Reg<core_debug_runstall_conf::CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Core Debug runstall configure register"]
pub mod core_debug_runstall_conf;
#[doc = "CORE_AHB_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_ahb_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_ahb_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_ahb_timeout`] module"]
pub type CORE_AHB_TIMEOUT = crate::Reg<core_ahb_timeout::CORE_AHB_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod core_ahb_timeout;
#[doc = "CORE_IBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_ibus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_ibus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_ibus_timeout`] module"]
pub type CORE_IBUS_TIMEOUT = crate::Reg<core_ibus_timeout::CORE_IBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod core_ibus_timeout;
#[doc = "CORE_DBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_dbus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_dbus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_dbus_timeout`] module"]
pub type CORE_DBUS_TIMEOUT = crate::Reg<core_dbus_timeout::CORE_DBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod core_dbus_timeout;
#[doc = "ICM_CPU_H2X_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icm_cpu_h2x_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icm_cpu_h2x_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_cpu_h2x_cfg`] module"]
pub type ICM_CPU_H2X_CFG = crate::Reg<icm_cpu_h2x_cfg::ICM_CPU_H2X_CFG_SPEC>;
#[doc = "need_des"]
pub mod icm_cpu_h2x_cfg;
#[doc = "PERI1_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri1_apb_postw_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri1_apb_postw_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri1_apb_postw_en`] module"]
pub type PERI1_APB_POSTW_EN = crate::Reg<peri1_apb_postw_en::PERI1_APB_POSTW_EN_SPEC>;
#[doc = "NA"]
pub mod peri1_apb_postw_en;
#[doc = "BITSCRAMBLER_PERI_SEL (rw) register accessor: Bitscrambler Peri Sel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bitscrambler_peri_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bitscrambler_peri_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitscrambler_peri_sel`] module"]
pub type BITSCRAMBLER_PERI_SEL = crate::Reg<bitscrambler_peri_sel::BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Bitscrambler Peri Sel"]
pub mod bitscrambler_peri_sel;
#[doc = "APB_SYNC_POSTW_EN (rw) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_sync_postw_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_sync_postw_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_sync_postw_en`] module"]
pub type APB_SYNC_POSTW_EN = crate::Reg<apb_sync_postw_en::APB_SYNC_POSTW_EN_SPEC>;
#[doc = "N/A"]
pub mod apb_sync_postw_en;
#[doc = "GDMA_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdma_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdma_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctrl`] module"]
pub type GDMA_CTRL = crate::Reg<gdma_ctrl::GDMA_CTRL_SPEC>;
#[doc = "N/A"]
pub mod gdma_ctrl;
#[doc = "GMAC_CTRL0 (rw) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl0`] module"]
pub type GMAC_CTRL0 = crate::Reg<gmac_ctrl0::GMAC_CTRL0_SPEC>;
#[doc = "N/A"]
pub mod gmac_ctrl0;
#[doc = "GMAC_CTRL1 (r) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_ctrl1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl1`] module"]
pub type GMAC_CTRL1 = crate::Reg<gmac_ctrl1::GMAC_CTRL1_SPEC>;
#[doc = "N/A"]
pub mod gmac_ctrl1;
#[doc = "GMAC_CTRL2 (r) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_ctrl2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_ctrl2`] module"]
pub type GMAC_CTRL2 = crate::Reg<gmac_ctrl2::GMAC_CTRL2_SPEC>;
#[doc = "N/A"]
pub mod gmac_ctrl2;
#[doc = "VPU_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpu_ctrl`] module"]
pub type VPU_CTRL = crate::Reg<vpu_ctrl::VPU_CTRL_SPEC>;
#[doc = "N/A"]
pub mod vpu_ctrl;
#[doc = "USBOTG20_CTRL (rw) register accessor: N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbotg20_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbotg20_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbotg20_ctrl`] module"]
pub type USBOTG20_CTRL = crate::Reg<usbotg20_ctrl::USBOTG20_CTRL_SPEC>;
#[doc = "N/A"]
pub mod usbotg20_ctrl;
#[doc = "TCM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_err_resp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_err_resp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_err_resp_ctrl`] module"]
pub type TCM_ERR_RESP_CTRL = crate::Reg<tcm_err_resp_ctrl::TCM_ERR_RESP_CTRL_SPEC>;
#[doc = "need_des"]
pub mod tcm_err_resp_ctrl;
#[doc = "L2_MEM_REFRESH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_refresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_refresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_refresh`] module"]
pub type L2_MEM_REFRESH = crate::Reg<l2_mem_refresh::L2_MEM_REFRESH_SPEC>;
#[doc = "NA"]
pub mod l2_mem_refresh;
#[doc = "TCM_INIT (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_init`] module"]
pub type TCM_INIT = crate::Reg<tcm_init::TCM_INIT_SPEC>;
#[doc = "NA"]
pub mod tcm_init;
#[doc = "TCM_PARITY_CHECK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcm_parity_check_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_parity_check_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcm_parity_check_ctrl`] module"]
pub type TCM_PARITY_CHECK_CTRL = crate::Reg<tcm_parity_check_ctrl::TCM_PARITY_CHECK_CTRL_SPEC>;
#[doc = "need_des"]
pub mod tcm_parity_check_ctrl;
#[doc = "DESIGN_FOR_VERIFICATION0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`design_for_verification0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`design_for_verification0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@design_for_verification0`] module"]
pub type DESIGN_FOR_VERIFICATION0 =
    crate::Reg<design_for_verification0::DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "need_des"]
pub mod design_for_verification0;
#[doc = "DESIGN_FOR_VERIFICATION1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`design_for_verification1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`design_for_verification1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@design_for_verification1`] module"]
pub type DESIGN_FOR_VERIFICATION1 =
    crate::Reg<design_for_verification1::DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "need_des"]
pub mod design_for_verification1;
#[doc = "PSRAM_FLASH_ADDR_INTERCHANGE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psram_flash_addr_interchange::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psram_flash_addr_interchange::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_flash_addr_interchange`] module"]
pub type PSRAM_FLASH_ADDR_INTERCHANGE =
    crate::Reg<psram_flash_addr_interchange::PSRAM_FLASH_ADDR_INTERCHANGE_SPEC>;
#[doc = "need_des"]
pub mod psram_flash_addr_interchange;
#[doc = "AHB2AXI_BRESP_ERR_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2axi_bresp_err_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_raw`] module"]
pub type AHB2AXI_BRESP_ERR_INT_RAW =
    crate::Reg<ahb2axi_bresp_err_int_raw::AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "NA"]
pub mod ahb2axi_bresp_err_int_raw;
#[doc = "AHB2AXI_BRESP_ERR_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2axi_bresp_err_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_st`] module"]
pub type AHB2AXI_BRESP_ERR_INT_ST =
    crate::Reg<ahb2axi_bresp_err_int_st::AHB2AXI_BRESP_ERR_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_st;
#[doc = "AHB2AXI_BRESP_ERR_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2axi_bresp_err_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_ena`] module"]
pub type AHB2AXI_BRESP_ERR_INT_ENA =
    crate::Reg<ahb2axi_bresp_err_int_ena::AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_ena;
#[doc = "AHB2AXI_BRESP_ERR_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2axi_bresp_err_int_clr`] module"]
pub type AHB2AXI_BRESP_ERR_INT_CLR =
    crate::Reg<ahb2axi_bresp_err_int_clr::AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod ahb2axi_bresp_err_int_clr;
#[doc = "L2_MEM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_err_resp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_err_resp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_err_resp_ctrl`] module"]
pub type L2_MEM_ERR_RESP_CTRL = crate::Reg<l2_mem_err_resp_ctrl::L2_MEM_ERR_RESP_CTRL_SPEC>;
#[doc = "need_des"]
pub mod l2_mem_err_resp_ctrl;
#[doc = "L2_MEM_AHB_BUFFER_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_ahb_buffer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_ahb_buffer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2_mem_ahb_buffer_ctrl`] module"]
pub type L2_MEM_AHB_BUFFER_CTRL = crate::Reg<l2_mem_ahb_buffer_ctrl::L2_MEM_AHB_BUFFER_CTRL_SPEC>;
#[doc = "need_des"]
pub mod l2_mem_ahb_buffer_ctrl;
#[doc = "CORE_DMACTIVE_LPCORE (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_dmactive_lpcore::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_dmactive_lpcore`] module"]
pub type CORE_DMACTIVE_LPCORE = crate::Reg<core_dmactive_lpcore::CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "need_des"]
pub mod core_dmactive_lpcore;
#[doc = "CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_err_resp_dis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_err_resp_dis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_err_resp_dis`] module"]
pub type CORE_ERR_RESP_DIS = crate::Reg<core_err_resp_dis::CORE_ERR_RESP_DIS_SPEC>;
#[doc = "need_des"]
pub mod core_err_resp_dis;
#[doc = "CORE_TIMEOUT_INT_RAW (rw) register accessor: Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_timeout_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_timeout_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_raw`] module"]
pub type CORE_TIMEOUT_INT_RAW = crate::Reg<core_timeout_int_raw::CORE_TIMEOUT_INT_RAW_SPEC>;
#[doc = "Hp core bus timeout interrupt raw register"]
pub mod core_timeout_int_raw;
#[doc = "CORE_TIMEOUT_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_timeout_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_st`] module"]
pub type CORE_TIMEOUT_INT_ST = crate::Reg<core_timeout_int_st::CORE_TIMEOUT_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod core_timeout_int_st;
#[doc = "CORE_TIMEOUT_INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_timeout_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_timeout_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_ena`] module"]
pub type CORE_TIMEOUT_INT_ENA = crate::Reg<core_timeout_int_ena::CORE_TIMEOUT_INT_ENA_SPEC>;
#[doc = "masked interrupt register"]
pub mod core_timeout_int_ena;
#[doc = "CORE_TIMEOUT_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_timeout_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_timeout_int_clr`] module"]
pub type CORE_TIMEOUT_INT_CLR = crate::Reg<core_timeout_int_clr::CORE_TIMEOUT_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod core_timeout_int_clr;
#[doc = "GPIO_O_HYS_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hys_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hys_ctrl0`] module"]
pub type GPIO_O_HYS_CTRL0 = crate::Reg<gpio_o_hys_ctrl0::GPIO_O_HYS_CTRL0_SPEC>;
#[doc = "NA"]
pub mod gpio_o_hys_ctrl0;
#[doc = "GPIO_O_HYS_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hys_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_o_hys_ctrl1`] module"]
pub type GPIO_O_HYS_CTRL1 = crate::Reg<gpio_o_hys_ctrl1::GPIO_O_HYS_CTRL1_SPEC>;
#[doc = "NA"]
pub mod gpio_o_hys_ctrl1;
#[doc = "RSA_PD_CTRL (rw) register accessor: rsa pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_pd_ctrl`] module"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "rsa pd ctrl register"]
pub mod rsa_pd_ctrl;
#[doc = "ECC_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_pd_ctrl`] module"]
pub type ECC_PD_CTRL = crate::Reg<ecc_pd_ctrl::ECC_PD_CTRL_SPEC>;
#[doc = "ecc pd ctrl register"]
pub mod ecc_pd_ctrl;
#[doc = "RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
pub type RNG_CFG = crate::Reg<rng_cfg::RNG_CFG_SPEC>;
#[doc = "rng cfg register"]
pub mod rng_cfg;
#[doc = "UART_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_pd_ctrl`] module"]
pub type UART_PD_CTRL = crate::Reg<uart_pd_ctrl::UART_PD_CTRL_SPEC>;
#[doc = "ecc pd ctrl register"]
pub mod uart_pd_ctrl;
#[doc = "PERI_MEM_CLK_FORCE_ON (rw) register accessor: hp peri mem clk force on regpster\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_mem_clk_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_mem_clk_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_mem_clk_force_on`] module"]
pub type PERI_MEM_CLK_FORCE_ON = crate::Reg<peri_mem_clk_force_on::PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "hp peri mem clk force on regpster"]
pub mod peri_mem_clk_force_on;
