#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    ver_date: VER_DATE,
    hp_clk_en: HP_CLK_EN,
    _reserved2: [u8; 0x08],
    hp_cpu_int_from_cpu_0: HP_CPU_INT_FROM_CPU_0,
    hp_cpu_int_from_cpu_1: HP_CPU_INT_FROM_CPU_1,
    hp_cpu_int_from_cpu_2: HP_CPU_INT_FROM_CPU_2,
    hp_cpu_int_from_cpu_3: HP_CPU_INT_FROM_CPU_3,
    hp_cache_clk_config: HP_CACHE_CLK_CONFIG,
    hp_cache_reset_config: HP_CACHE_RESET_CONFIG,
    _reserved8: [u8; 0x04],
    dma_addr_ctrl: DMA_ADDR_CTRL,
    _reserved9: [u8; 0x04],
    hp_tcm_ram_wrr_config: HP_TCM_RAM_WRR_CONFIG,
    hp_tcm_sw_parity_bwe_mask: HP_TCM_SW_PARITY_BWE_MASK,
    hp_tcm_ram_pwr_ctrl0: HP_TCM_RAM_PWR_CTRL0,
    hp_l2_rom_pwr_ctrl0: HP_L2_ROM_PWR_CTRL0,
    _reserved13: [u8; 0x0c],
    hp_probea_ctrl: HP_PROBEA_CTRL,
    hp_probeb_ctrl: HP_PROBEB_CTRL,
    _reserved15: [u8; 0x04],
    hp_probe_out: HP_PROBE_OUT,
    hp_l2_mem_ram_pwr_ctrl0: HP_L2_MEM_RAM_PWR_CTRL0,
    hp_cpu_corestalled_st: HP_CPU_CORESTALLED_ST,
    _reserved18: [u8; 0x08],
    hp_crypto_ctrl: HP_CRYPTO_CTRL,
    hp_gpio_o_hold_ctrl0: HP_GPIO_O_HOLD_CTRL0,
    hp_gpio_o_hold_ctrl1: HP_GPIO_O_HOLD_CTRL1,
    rdn_eco_cs: RDN_ECO_CS,
    hp_cache_apb_postw_en: HP_CACHE_APB_POSTW_EN,
    hp_l2_mem_subsize: HP_L2_MEM_SUBSIZE,
    _reserved24: [u8; 0x14],
    hp_l2_mem_int_raw: HP_L2_MEM_INT_RAW,
    hp_l2_mem_int_st: HP_L2_MEM_INT_ST,
    hp_l2_mem_int_ena: HP_L2_MEM_INT_ENA,
    hp_l2_mem_int_clr: HP_L2_MEM_INT_CLR,
    hp_l2_mem_l2_ram_ecc: HP_L2_MEM_L2_RAM_ECC,
    hp_l2_mem_int_record0: HP_L2_MEM_INT_RECORD0,
    hp_l2_mem_int_record1: HP_L2_MEM_INT_RECORD1,
    _reserved31: [u8; 0x0c],
    hp_l2_mem_l2_cache_ecc: HP_L2_MEM_L2_CACHE_ECC,
    hp_l1cache_bus0_id: HP_L1CACHE_BUS0_ID,
    hp_l1cache_bus1_id: HP_L1CACHE_BUS1_ID,
    _reserved34: [u8; 0x08],
    hp_l2_mem_rdn_eco_cs: HP_L2_MEM_RDN_ECO_CS,
    hp_l2_mem_rdn_eco_low: HP_L2_MEM_RDN_ECO_LOW,
    hp_l2_mem_rdn_eco_high: HP_L2_MEM_RDN_ECO_HIGH,
    hp_tcm_rdn_eco_cs: HP_TCM_RDN_ECO_CS,
    hp_tcm_rdn_eco_low: HP_TCM_RDN_ECO_LOW,
    hp_tcm_rdn_eco_high: HP_TCM_RDN_ECO_HIGH,
    hp_gpio_ded_hold_ctrl: HP_GPIO_DED_HOLD_CTRL,
    hp_l2_mem_sw_ecc_bwe_mask: HP_L2_MEM_SW_ECC_BWE_MASK,
    hp_usb20otg_mem_ctrl: HP_USB20OTG_MEM_CTRL,
    hp_tcm_int_raw: HP_TCM_INT_RAW,
    hp_tcm_int_st: HP_TCM_INT_ST,
    hp_tcm_int_ena: HP_TCM_INT_ENA,
    hp_tcm_int_clr: HP_TCM_INT_CLR,
    hp_tcm_parity_int_record: HP_TCM_PARITY_INT_RECORD,
    hp_l1_cache_pwr_ctrl: HP_L1_CACHE_PWR_CTRL,
    hp_l2_cache_pwr_ctrl: HP_L2_CACHE_PWR_CTRL,
    hp_cpu_waiti_conf: HP_CPU_WAITI_CONF,
    core_debug_runstall_conf: CORE_DEBUG_RUNSTALL_CONF,
    hp_core_ahb_timeout: HP_CORE_AHB_TIMEOUT,
    hp_core_ibus_timeout: HP_CORE_IBUS_TIMEOUT,
    hp_core_dbus_timeout: HP_CORE_DBUS_TIMEOUT,
    _reserved55: [u8; 0x0c],
    hp_icm_cpu_h2x_cfg: HP_ICM_CPU_H2X_CFG,
    hp_peri1_apb_postw_en: HP_PERI1_APB_POSTW_EN,
    hp_bitscrambler_peri_sel: HP_BITSCRAMBLER_PERI_SEL,
    apb_sync_postw_en: APB_SYNC_POSTW_EN,
    gdma_ctrl: GDMA_CTRL,
    gmac_ctrl0: GMAC_CTRL0,
    gmac_ctrl1: GMAC_CTRL1,
    gmac_ctrl2: GMAC_CTRL2,
    vpu_ctrl: VPU_CTRL,
    usbotg20_ctrl: USBOTG20_CTRL,
    hp_tcm_err_resp_ctrl: HP_TCM_ERR_RESP_CTRL,
    hp_l2_mem_refresh: HP_L2_MEM_REFRESH,
    hp_tcm_init: HP_TCM_INIT,
    hp_tcm_parity_check_ctrl: HP_TCM_PARITY_CHECK_CTRL,
    hp_design_for_verification0: HP_DESIGN_FOR_VERIFICATION0,
    hp_design_for_verification1: HP_DESIGN_FOR_VERIFICATION1,
    _reserved71: [u8; 0x08],
    hp_psram_flash_addr_interchange: HP_PSRAM_FLASH_ADDR_INTERCHANGE,
    _reserved72: [u8; 0x04],
    hp_ahb2axi_bresp_err_int_raw: HP_AHB2AXI_BRESP_ERR_INT_RAW,
    hp_ahb2axi_bresp_err_int_st: HP_AHB2AXI_BRESP_ERR_INT_ST,
    hp_ahb2axi_bresp_err_int_ena: HP_AHB2AXI_BRESP_ERR_INT_ENA,
    hp_ahb2axi_bresp_err_int_clr: HP_AHB2AXI_BRESP_ERR_INT_CLR,
    hp_l2_mem_err_resp_ctrl: HP_L2_MEM_ERR_RESP_CTRL,
    hp_l2_mem_ahb_buffer_ctrl: HP_L2_MEM_AHB_BUFFER_CTRL,
    hp_core_dmactive_lpcore: HP_CORE_DMACTIVE_LPCORE,
    hp_core_err_resp_dis: HP_CORE_ERR_RESP_DIS,
    hp_core_timeout_int_raw: HP_CORE_TIMEOUT_INT_RAW,
    hp_core_timeout_int_st: HP_CORE_TIMEOUT_INT_ST,
    hp_core_timeout_int_ena: HP_CORE_TIMEOUT_INT_ENA,
    hp_core_timeout_int_clr: HP_CORE_TIMEOUT_INT_CLR,
    _reserved84: [u8; 0x08],
    hp_gpio_o_hys_ctrl0: HP_GPIO_O_HYS_CTRL0,
    hp_gpio_o_hys_ctrl1: HP_GPIO_O_HYS_CTRL1,
    _reserved86: [u8; 0x08],
    hp_rsa_pd_ctrl: HP_RSA_PD_CTRL,
    hp_ecc_pd_ctrl: HP_ECC_PD_CTRL,
    hp_rng_cfg: HP_RNG_CFG,
    hp_uart_pd_ctrl: HP_UART_PD_CTRL,
    hp_peri_mem_clk_force_on: HP_PERI_MEM_CLK_FORCE_ON,
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
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn hp_cache_clk_config(&self) -> &HP_CACHE_CLK_CONFIG {
        &self.hp_cache_clk_config
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn hp_cache_reset_config(&self) -> &HP_CACHE_RESET_CONFIG {
        &self.hp_cache_reset_config
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn dma_addr_ctrl(&self) -> &DMA_ADDR_CTRL {
        &self.dma_addr_ctrl
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn hp_tcm_ram_wrr_config(&self) -> &HP_TCM_RAM_WRR_CONFIG {
        &self.hp_tcm_ram_wrr_config
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn hp_tcm_sw_parity_bwe_mask(&self) -> &HP_TCM_SW_PARITY_BWE_MASK {
        &self.hp_tcm_sw_parity_bwe_mask
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn hp_tcm_ram_pwr_ctrl0(&self) -> &HP_TCM_RAM_PWR_CTRL0 {
        &self.hp_tcm_ram_pwr_ctrl0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn hp_l2_rom_pwr_ctrl0(&self) -> &HP_L2_ROM_PWR_CTRL0 {
        &self.hp_l2_rom_pwr_ctrl0
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
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_ram_pwr_ctrl0(&self) -> &HP_L2_MEM_RAM_PWR_CTRL0 {
        &self.hp_l2_mem_ram_pwr_ctrl0
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
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn hp_gpio_o_hold_ctrl0(&self) -> &HP_GPIO_O_HOLD_CTRL0 {
        &self.hp_gpio_o_hold_ctrl0
    }
    #[doc = "0x78 - NA"]
    #[inline(always)]
    pub const fn hp_gpio_o_hold_ctrl1(&self) -> &HP_GPIO_O_HOLD_CTRL1 {
        &self.hp_gpio_o_hold_ctrl1
    }
    #[doc = "0x7c - NA"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    #[doc = "0x80 - NA"]
    #[inline(always)]
    pub const fn hp_cache_apb_postw_en(&self) -> &HP_CACHE_APB_POSTW_EN {
        &self.hp_cache_apb_postw_en
    }
    #[doc = "0x84 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_subsize(&self) -> &HP_L2_MEM_SUBSIZE {
        &self.hp_l2_mem_subsize
    }
    #[doc = "0x9c - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_raw(&self) -> &HP_L2_MEM_INT_RAW {
        &self.hp_l2_mem_int_raw
    }
    #[doc = "0xa0 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_st(&self) -> &HP_L2_MEM_INT_ST {
        &self.hp_l2_mem_int_st
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_ena(&self) -> &HP_L2_MEM_INT_ENA {
        &self.hp_l2_mem_int_ena
    }
    #[doc = "0xa8 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_clr(&self) -> &HP_L2_MEM_INT_CLR {
        &self.hp_l2_mem_int_clr
    }
    #[doc = "0xac - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_l2_ram_ecc(&self) -> &HP_L2_MEM_L2_RAM_ECC {
        &self.hp_l2_mem_l2_ram_ecc
    }
    #[doc = "0xb0 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_record0(&self) -> &HP_L2_MEM_INT_RECORD0 {
        &self.hp_l2_mem_int_record0
    }
    #[doc = "0xb4 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_int_record1(&self) -> &HP_L2_MEM_INT_RECORD1 {
        &self.hp_l2_mem_int_record1
    }
    #[doc = "0xc4 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_l2_cache_ecc(&self) -> &HP_L2_MEM_L2_CACHE_ECC {
        &self.hp_l2_mem_l2_cache_ecc
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn hp_l1cache_bus0_id(&self) -> &HP_L1CACHE_BUS0_ID {
        &self.hp_l1cache_bus0_id
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn hp_l1cache_bus1_id(&self) -> &HP_L1CACHE_BUS1_ID {
        &self.hp_l1cache_bus1_id
    }
    #[doc = "0xd8 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_rdn_eco_cs(&self) -> &HP_L2_MEM_RDN_ECO_CS {
        &self.hp_l2_mem_rdn_eco_cs
    }
    #[doc = "0xdc - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_rdn_eco_low(&self) -> &HP_L2_MEM_RDN_ECO_LOW {
        &self.hp_l2_mem_rdn_eco_low
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_rdn_eco_high(&self) -> &HP_L2_MEM_RDN_ECO_HIGH {
        &self.hp_l2_mem_rdn_eco_high
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
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn hp_gpio_ded_hold_ctrl(&self) -> &HP_GPIO_DED_HOLD_CTRL {
        &self.hp_gpio_ded_hold_ctrl
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_sw_ecc_bwe_mask(&self) -> &HP_L2_MEM_SW_ECC_BWE_MASK {
        &self.hp_l2_mem_sw_ecc_bwe_mask
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn hp_usb20otg_mem_ctrl(&self) -> &HP_USB20OTG_MEM_CTRL {
        &self.hp_usb20otg_mem_ctrl
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_int_raw(&self) -> &HP_TCM_INT_RAW {
        &self.hp_tcm_int_raw
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_int_st(&self) -> &HP_TCM_INT_ST {
        &self.hp_tcm_int_st
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_int_ena(&self) -> &HP_TCM_INT_ENA {
        &self.hp_tcm_int_ena
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_int_clr(&self) -> &HP_TCM_INT_CLR {
        &self.hp_tcm_int_clr
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_parity_int_record(&self) -> &HP_TCM_PARITY_INT_RECORD {
        &self.hp_tcm_parity_int_record
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn hp_l1_cache_pwr_ctrl(&self) -> &HP_L1_CACHE_PWR_CTRL {
        &self.hp_l1_cache_pwr_ctrl
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn hp_l2_cache_pwr_ctrl(&self) -> &HP_L2_CACHE_PWR_CTRL {
        &self.hp_l2_cache_pwr_ctrl
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
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn hp_core_ahb_timeout(&self) -> &HP_CORE_AHB_TIMEOUT {
        &self.hp_core_ahb_timeout
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn hp_core_ibus_timeout(&self) -> &HP_CORE_IBUS_TIMEOUT {
        &self.hp_core_ibus_timeout
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn hp_core_dbus_timeout(&self) -> &HP_CORE_DBUS_TIMEOUT {
        &self.hp_core_dbus_timeout
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn hp_icm_cpu_h2x_cfg(&self) -> &HP_ICM_CPU_H2X_CFG {
        &self.hp_icm_cpu_h2x_cfg
    }
    #[doc = "0x13c - NA"]
    #[inline(always)]
    pub const fn hp_peri1_apb_postw_en(&self) -> &HP_PERI1_APB_POSTW_EN {
        &self.hp_peri1_apb_postw_en
    }
    #[doc = "0x140 - Bitscrambler Peri Sel"]
    #[inline(always)]
    pub const fn hp_bitscrambler_peri_sel(&self) -> &HP_BITSCRAMBLER_PERI_SEL {
        &self.hp_bitscrambler_peri_sel
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
    pub const fn hp_tcm_err_resp_ctrl(&self) -> &HP_TCM_ERR_RESP_CTRL {
        &self.hp_tcm_err_resp_ctrl
    }
    #[doc = "0x164 - NA"]
    #[inline(always)]
    pub const fn hp_l2_mem_refresh(&self) -> &HP_L2_MEM_REFRESH {
        &self.hp_l2_mem_refresh
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn hp_tcm_init(&self) -> &HP_TCM_INIT {
        &self.hp_tcm_init
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn hp_tcm_parity_check_ctrl(&self) -> &HP_TCM_PARITY_CHECK_CTRL {
        &self.hp_tcm_parity_check_ctrl
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
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn hp_psram_flash_addr_interchange(&self) -> &HP_PSRAM_FLASH_ADDR_INTERCHANGE {
        &self.hp_psram_flash_addr_interchange
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
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn hp_l2_mem_err_resp_ctrl(&self) -> &HP_L2_MEM_ERR_RESP_CTRL {
        &self.hp_l2_mem_err_resp_ctrl
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn hp_l2_mem_ahb_buffer_ctrl(&self) -> &HP_L2_MEM_AHB_BUFFER_CTRL {
        &self.hp_l2_mem_ahb_buffer_ctrl
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
    #[doc = "0x1c0 - NA"]
    #[inline(always)]
    pub const fn hp_gpio_o_hys_ctrl0(&self) -> &HP_GPIO_O_HYS_CTRL0 {
        &self.hp_gpio_o_hys_ctrl0
    }
    #[doc = "0x1c4 - NA"]
    #[inline(always)]
    pub const fn hp_gpio_o_hys_ctrl1(&self) -> &HP_GPIO_O_HYS_CTRL1 {
        &self.hp_gpio_o_hys_ctrl1
    }
    #[doc = "0x1d0 - rsa pd ctrl register"]
    #[inline(always)]
    pub const fn hp_rsa_pd_ctrl(&self) -> &HP_RSA_PD_CTRL {
        &self.hp_rsa_pd_ctrl
    }
    #[doc = "0x1d4 - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn hp_ecc_pd_ctrl(&self) -> &HP_ECC_PD_CTRL {
        &self.hp_ecc_pd_ctrl
    }
    #[doc = "0x1d8 - rng cfg register"]
    #[inline(always)]
    pub const fn hp_rng_cfg(&self) -> &HP_RNG_CFG {
        &self.hp_rng_cfg
    }
    #[doc = "0x1dc - ecc pd ctrl register"]
    #[inline(always)]
    pub const fn hp_uart_pd_ctrl(&self) -> &HP_UART_PD_CTRL {
        &self.hp_uart_pd_ctrl
    }
    #[doc = "0x1e0 - hp peri mem clk force on regpster"]
    #[inline(always)]
    pub const fn hp_peri_mem_clk_force_on(&self) -> &HP_PERI_MEM_CLK_FORCE_ON {
        &self.hp_peri_mem_clk_force_on
    }
}
#[doc = "VER_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "NA"]
pub mod ver_date;
#[doc = "HP_CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_clk_en`] module"]
pub type HP_CLK_EN = crate::Reg<hp_clk_en::HP_CLK_EN_SPEC>;
#[doc = "NA"]
pub mod hp_clk_en;
#[doc = "HP_CPU_INT_FROM_CPU_0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_int_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_0`] module"]
pub type HP_CPU_INT_FROM_CPU_0 = crate::Reg<hp_cpu_int_from_cpu_0::HP_CPU_INT_FROM_CPU_0_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_0;
#[doc = "HP_CPU_INT_FROM_CPU_1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_int_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_1`] module"]
pub type HP_CPU_INT_FROM_CPU_1 = crate::Reg<hp_cpu_int_from_cpu_1::HP_CPU_INT_FROM_CPU_1_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_1;
#[doc = "HP_CPU_INT_FROM_CPU_2 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_int_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_2`] module"]
pub type HP_CPU_INT_FROM_CPU_2 = crate::Reg<hp_cpu_int_from_cpu_2::HP_CPU_INT_FROM_CPU_2_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_2;
#[doc = "HP_CPU_INT_FROM_CPU_3 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_int_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_int_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_int_from_cpu_3`] module"]
pub type HP_CPU_INT_FROM_CPU_3 = crate::Reg<hp_cpu_int_from_cpu_3::HP_CPU_INT_FROM_CPU_3_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_int_from_cpu_3;
#[doc = "HP_CACHE_CLK_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cache_clk_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cache_clk_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cache_clk_config`] module"]
pub type HP_CACHE_CLK_CONFIG = crate::Reg<hp_cache_clk_config::HP_CACHE_CLK_CONFIG_SPEC>;
#[doc = "NA"]
pub mod hp_cache_clk_config;
#[doc = "HP_CACHE_RESET_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cache_reset_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cache_reset_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cache_reset_config`] module"]
pub type HP_CACHE_RESET_CONFIG = crate::Reg<hp_cache_reset_config::HP_CACHE_RESET_CONFIG_SPEC>;
#[doc = "NA"]
pub mod hp_cache_reset_config;
#[doc = "DMA_ADDR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_addr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_addr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_addr_ctrl`] module"]
pub type DMA_ADDR_CTRL = crate::Reg<dma_addr_ctrl::DMA_ADDR_CTRL_SPEC>;
#[doc = "NA"]
pub mod dma_addr_ctrl;
#[doc = "HP_TCM_RAM_WRR_CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_ram_wrr_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_ram_wrr_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_ram_wrr_config`] module"]
pub type HP_TCM_RAM_WRR_CONFIG = crate::Reg<hp_tcm_ram_wrr_config::HP_TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_ram_wrr_config;
#[doc = "HP_TCM_SW_PARITY_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_sw_parity_bwe_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_sw_parity_bwe_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_sw_parity_bwe_mask`] module"]
pub type HP_TCM_SW_PARITY_BWE_MASK =
    crate::Reg<hp_tcm_sw_parity_bwe_mask::HP_TCM_SW_PARITY_BWE_MASK_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_sw_parity_bwe_mask;
#[doc = "HP_TCM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_ram_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_ram_pwr_ctrl0`] module"]
pub type HP_TCM_RAM_PWR_CTRL0 = crate::Reg<hp_tcm_ram_pwr_ctrl0::HP_TCM_RAM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_ram_pwr_ctrl0;
#[doc = "HP_L2_ROM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_rom_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_rom_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_rom_pwr_ctrl0`] module"]
pub type HP_L2_ROM_PWR_CTRL0 = crate::Reg<hp_l2_rom_pwr_ctrl0::HP_L2_ROM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_l2_rom_pwr_ctrl0;
#[doc = "HP_PROBEA_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_probea_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_probea_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probea_ctrl`] module"]
pub type HP_PROBEA_CTRL = crate::Reg<hp_probea_ctrl::HP_PROBEA_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_probea_ctrl;
#[doc = "HP_PROBEB_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_probeb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_probeb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probeb_ctrl`] module"]
pub type HP_PROBEB_CTRL = crate::Reg<hp_probeb_ctrl::HP_PROBEB_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_probeb_ctrl;
#[doc = "HP_PROBE_OUT (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_probe_out::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_probe_out`] module"]
pub type HP_PROBE_OUT = crate::Reg<hp_probe_out::HP_PROBE_OUT_SPEC>;
#[doc = "NA"]
pub mod hp_probe_out;
#[doc = "HP_L2_MEM_RAM_PWR_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_ram_pwr_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_ram_pwr_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_ram_pwr_ctrl0`] module"]
pub type HP_L2_MEM_RAM_PWR_CTRL0 =
    crate::Reg<hp_l2_mem_ram_pwr_ctrl0::HP_L2_MEM_RAM_PWR_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_ram_pwr_ctrl0;
#[doc = "HP_CPU_CORESTALLED_ST (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_corestalled_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_corestalled_st`] module"]
pub type HP_CPU_CORESTALLED_ST = crate::Reg<hp_cpu_corestalled_st::HP_CPU_CORESTALLED_ST_SPEC>;
#[doc = "NA"]
pub mod hp_cpu_corestalled_st;
#[doc = "HP_CRYPTO_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_crypto_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_crypto_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_crypto_ctrl`] module"]
pub type HP_CRYPTO_CTRL = crate::Reg<hp_crypto_ctrl::HP_CRYPTO_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_crypto_ctrl;
#[doc = "HP_GPIO_O_HOLD_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_gpio_o_hold_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hold_ctrl0`] module"]
pub type HP_GPIO_O_HOLD_CTRL0 = crate::Reg<hp_gpio_o_hold_ctrl0::HP_GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_gpio_o_hold_ctrl0;
#[doc = "HP_GPIO_O_HOLD_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_gpio_o_hold_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hold_ctrl1`] module"]
pub type HP_GPIO_O_HOLD_CTRL1 = crate::Reg<hp_gpio_o_hold_ctrl1::HP_GPIO_O_HOLD_CTRL1_SPEC>;
#[doc = "NA"]
pub mod hp_gpio_o_hold_ctrl1;
#[doc = "RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod rdn_eco_cs;
#[doc = "HP_CACHE_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cache_apb_postw_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cache_apb_postw_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cache_apb_postw_en`] module"]
pub type HP_CACHE_APB_POSTW_EN = crate::Reg<hp_cache_apb_postw_en::HP_CACHE_APB_POSTW_EN_SPEC>;
#[doc = "NA"]
pub mod hp_cache_apb_postw_en;
#[doc = "HP_L2_MEM_SUBSIZE (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_subsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_subsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_subsize`] module"]
pub type HP_L2_MEM_SUBSIZE = crate::Reg<hp_l2_mem_subsize::HP_L2_MEM_SUBSIZE_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_subsize;
#[doc = "HP_L2_MEM_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_raw`] module"]
pub type HP_L2_MEM_INT_RAW = crate::Reg<hp_l2_mem_int_raw::HP_L2_MEM_INT_RAW_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_raw;
#[doc = "HP_L2_MEM_INT_ST (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_st`] module"]
pub type HP_L2_MEM_INT_ST = crate::Reg<hp_l2_mem_int_st::HP_L2_MEM_INT_ST_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_st;
#[doc = "HP_L2_MEM_INT_ENA (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_ena`] module"]
pub type HP_L2_MEM_INT_ENA = crate::Reg<hp_l2_mem_int_ena::HP_L2_MEM_INT_ENA_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_ena;
#[doc = "HP_L2_MEM_INT_CLR (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_clr`] module"]
pub type HP_L2_MEM_INT_CLR = crate::Reg<hp_l2_mem_int_clr::HP_L2_MEM_INT_CLR_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_clr;
#[doc = "HP_L2_MEM_L2_RAM_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_l2_ram_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_l2_ram_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_l2_ram_ecc`] module"]
pub type HP_L2_MEM_L2_RAM_ECC = crate::Reg<hp_l2_mem_l2_ram_ecc::HP_L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_l2_ram_ecc;
#[doc = "HP_L2_MEM_INT_RECORD0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_record0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_record0`] module"]
pub type HP_L2_MEM_INT_RECORD0 = crate::Reg<hp_l2_mem_int_record0::HP_L2_MEM_INT_RECORD0_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_record0;
#[doc = "HP_L2_MEM_INT_RECORD1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_record1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_int_record1`] module"]
pub type HP_L2_MEM_INT_RECORD1 = crate::Reg<hp_l2_mem_int_record1::HP_L2_MEM_INT_RECORD1_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_int_record1;
#[doc = "HP_L2_MEM_L2_CACHE_ECC (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_l2_cache_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_l2_cache_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_l2_cache_ecc`] module"]
pub type HP_L2_MEM_L2_CACHE_ECC = crate::Reg<hp_l2_mem_l2_cache_ecc::HP_L2_MEM_L2_CACHE_ECC_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_l2_cache_ecc;
#[doc = "HP_L1CACHE_BUS0_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l1cache_bus0_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l1cache_bus0_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l1cache_bus0_id`] module"]
pub type HP_L1CACHE_BUS0_ID = crate::Reg<hp_l1cache_bus0_id::HP_L1CACHE_BUS0_ID_SPEC>;
#[doc = "NA"]
pub mod hp_l1cache_bus0_id;
#[doc = "HP_L1CACHE_BUS1_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l1cache_bus1_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l1cache_bus1_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l1cache_bus1_id`] module"]
pub type HP_L1CACHE_BUS1_ID = crate::Reg<hp_l1cache_bus1_id::HP_L1CACHE_BUS1_ID_SPEC>;
#[doc = "NA"]
pub mod hp_l1cache_bus1_id;
#[doc = "HP_L2_MEM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_rdn_eco_cs`] module"]
pub type HP_L2_MEM_RDN_ECO_CS = crate::Reg<hp_l2_mem_rdn_eco_cs::HP_L2_MEM_RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_rdn_eco_cs;
#[doc = "HP_L2_MEM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_rdn_eco_low`] module"]
pub type HP_L2_MEM_RDN_ECO_LOW = crate::Reg<hp_l2_mem_rdn_eco_low::HP_L2_MEM_RDN_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_rdn_eco_low;
#[doc = "HP_L2_MEM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_rdn_eco_high`] module"]
pub type HP_L2_MEM_RDN_ECO_HIGH = crate::Reg<hp_l2_mem_rdn_eco_high::HP_L2_MEM_RDN_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_rdn_eco_high;
#[doc = "HP_TCM_RDN_ECO_CS (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_cs`] module"]
pub type HP_TCM_RDN_ECO_CS = crate::Reg<hp_tcm_rdn_eco_cs::HP_TCM_RDN_ECO_CS_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_cs;
#[doc = "HP_TCM_RDN_ECO_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_low`] module"]
pub type HP_TCM_RDN_ECO_LOW = crate::Reg<hp_tcm_rdn_eco_low::HP_TCM_RDN_ECO_LOW_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_low;
#[doc = "HP_TCM_RDN_ECO_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_rdn_eco_high`] module"]
pub type HP_TCM_RDN_ECO_HIGH = crate::Reg<hp_tcm_rdn_eco_high::HP_TCM_RDN_ECO_HIGH_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_rdn_eco_high;
#[doc = "HP_GPIO_DED_HOLD_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_gpio_ded_hold_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_gpio_ded_hold_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_ded_hold_ctrl`] module"]
pub type HP_GPIO_DED_HOLD_CTRL = crate::Reg<hp_gpio_ded_hold_ctrl::HP_GPIO_DED_HOLD_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_gpio_ded_hold_ctrl;
#[doc = "HP_L2_MEM_SW_ECC_BWE_MASK (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_sw_ecc_bwe_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_sw_ecc_bwe_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_sw_ecc_bwe_mask`] module"]
pub type HP_L2_MEM_SW_ECC_BWE_MASK =
    crate::Reg<hp_l2_mem_sw_ecc_bwe_mask::HP_L2_MEM_SW_ECC_BWE_MASK_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_sw_ecc_bwe_mask;
#[doc = "HP_USB20OTG_MEM_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_usb20otg_mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_usb20otg_mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_usb20otg_mem_ctrl`] module"]
pub type HP_USB20OTG_MEM_CTRL = crate::Reg<hp_usb20otg_mem_ctrl::HP_USB20OTG_MEM_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_usb20otg_mem_ctrl;
#[doc = "HP_TCM_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_int_raw`] module"]
pub type HP_TCM_INT_RAW = crate::Reg<hp_tcm_int_raw::HP_TCM_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_int_raw;
#[doc = "HP_TCM_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_int_st`] module"]
pub type HP_TCM_INT_ST = crate::Reg<hp_tcm_int_st::HP_TCM_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_int_st;
#[doc = "HP_TCM_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_int_ena`] module"]
pub type HP_TCM_INT_ENA = crate::Reg<hp_tcm_int_ena::HP_TCM_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_int_ena;
#[doc = "HP_TCM_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_int_clr`] module"]
pub type HP_TCM_INT_CLR = crate::Reg<hp_tcm_int_clr::HP_TCM_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_int_clr;
#[doc = "HP_TCM_PARITY_INT_RECORD (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_parity_int_record::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_parity_int_record`] module"]
pub type HP_TCM_PARITY_INT_RECORD =
    crate::Reg<hp_tcm_parity_int_record::HP_TCM_PARITY_INT_RECORD_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_parity_int_record;
#[doc = "HP_L1_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l1_cache_pwr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l1_cache_pwr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l1_cache_pwr_ctrl`] module"]
pub type HP_L1_CACHE_PWR_CTRL = crate::Reg<hp_l1_cache_pwr_ctrl::HP_L1_CACHE_PWR_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_l1_cache_pwr_ctrl;
#[doc = "HP_L2_CACHE_PWR_CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_cache_pwr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_cache_pwr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_cache_pwr_ctrl`] module"]
pub type HP_L2_CACHE_PWR_CTRL = crate::Reg<hp_l2_cache_pwr_ctrl::HP_L2_CACHE_PWR_CTRL_SPEC>;
#[doc = "NA"]
pub mod hp_l2_cache_pwr_ctrl;
#[doc = "HP_CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_waiti_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_waiti_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_cpu_waiti_conf`] module"]
pub type HP_CPU_WAITI_CONF = crate::Reg<hp_cpu_waiti_conf::HP_CPU_WAITI_CONF_SPEC>;
#[doc = "CPU_WAITI configuration register"]
pub mod hp_cpu_waiti_conf;
#[doc = "CORE_DEBUG_RUNSTALL_CONF (rw) register accessor: Core Debug runstall configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_debug_runstall_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_debug_runstall_conf`] module"]
pub type CORE_DEBUG_RUNSTALL_CONF =
    crate::Reg<core_debug_runstall_conf::CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Core Debug runstall configure register"]
pub mod core_debug_runstall_conf;
#[doc = "HP_CORE_AHB_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_ahb_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_ahb_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_ahb_timeout`] module"]
pub type HP_CORE_AHB_TIMEOUT = crate::Reg<hp_core_ahb_timeout::HP_CORE_AHB_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod hp_core_ahb_timeout;
#[doc = "HP_CORE_IBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_ibus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_ibus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_ibus_timeout`] module"]
pub type HP_CORE_IBUS_TIMEOUT = crate::Reg<hp_core_ibus_timeout::HP_CORE_IBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod hp_core_ibus_timeout;
#[doc = "HP_CORE_DBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_dbus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_dbus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_dbus_timeout`] module"]
pub type HP_CORE_DBUS_TIMEOUT = crate::Reg<hp_core_dbus_timeout::HP_CORE_DBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod hp_core_dbus_timeout;
#[doc = "HP_ICM_CPU_H2X_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_icm_cpu_h2x_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_icm_cpu_h2x_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_icm_cpu_h2x_cfg`] module"]
pub type HP_ICM_CPU_H2X_CFG = crate::Reg<hp_icm_cpu_h2x_cfg::HP_ICM_CPU_H2X_CFG_SPEC>;
#[doc = "need_des"]
pub mod hp_icm_cpu_h2x_cfg;
#[doc = "HP_PERI1_APB_POSTW_EN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_peri1_apb_postw_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri1_apb_postw_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri1_apb_postw_en`] module"]
pub type HP_PERI1_APB_POSTW_EN = crate::Reg<hp_peri1_apb_postw_en::HP_PERI1_APB_POSTW_EN_SPEC>;
#[doc = "NA"]
pub mod hp_peri1_apb_postw_en;
#[doc = "HP_BITSCRAMBLER_PERI_SEL (rw) register accessor: Bitscrambler Peri Sel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_bitscrambler_peri_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_bitscrambler_peri_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_bitscrambler_peri_sel`] module"]
pub type HP_BITSCRAMBLER_PERI_SEL =
    crate::Reg<hp_bitscrambler_peri_sel::HP_BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Bitscrambler Peri Sel"]
pub mod hp_bitscrambler_peri_sel;
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
#[doc = "HP_TCM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_err_resp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_err_resp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_err_resp_ctrl`] module"]
pub type HP_TCM_ERR_RESP_CTRL = crate::Reg<hp_tcm_err_resp_ctrl::HP_TCM_ERR_RESP_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_err_resp_ctrl;
#[doc = "HP_L2_MEM_REFRESH (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_refresh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_refresh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_refresh`] module"]
pub type HP_L2_MEM_REFRESH = crate::Reg<hp_l2_mem_refresh::HP_L2_MEM_REFRESH_SPEC>;
#[doc = "NA"]
pub mod hp_l2_mem_refresh;
#[doc = "HP_TCM_INIT (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_init`] module"]
pub type HP_TCM_INIT = crate::Reg<hp_tcm_init::HP_TCM_INIT_SPEC>;
#[doc = "NA"]
pub mod hp_tcm_init;
#[doc = "HP_TCM_PARITY_CHECK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_parity_check_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_parity_check_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_tcm_parity_check_ctrl`] module"]
pub type HP_TCM_PARITY_CHECK_CTRL =
    crate::Reg<hp_tcm_parity_check_ctrl::HP_TCM_PARITY_CHECK_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_tcm_parity_check_ctrl;
#[doc = "HP_DESIGN_FOR_VERIFICATION0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_design_for_verification0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_design_for_verification0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_design_for_verification0`] module"]
pub type HP_DESIGN_FOR_VERIFICATION0 =
    crate::Reg<hp_design_for_verification0::HP_DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "need_des"]
pub mod hp_design_for_verification0;
#[doc = "HP_DESIGN_FOR_VERIFICATION1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_design_for_verification1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_design_for_verification1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_design_for_verification1`] module"]
pub type HP_DESIGN_FOR_VERIFICATION1 =
    crate::Reg<hp_design_for_verification1::HP_DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "need_des"]
pub mod hp_design_for_verification1;
#[doc = "HP_PSRAM_FLASH_ADDR_INTERCHANGE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_psram_flash_addr_interchange::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_psram_flash_addr_interchange::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_psram_flash_addr_interchange`] module"]
pub type HP_PSRAM_FLASH_ADDR_INTERCHANGE =
    crate::Reg<hp_psram_flash_addr_interchange::HP_PSRAM_FLASH_ADDR_INTERCHANGE_SPEC>;
#[doc = "need_des"]
pub mod hp_psram_flash_addr_interchange;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_RAW (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_raw`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_RAW =
    crate::Reg<hp_ahb2axi_bresp_err_int_raw::HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "NA"]
pub mod hp_ahb2axi_bresp_err_int_raw;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_st`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_ST =
    crate::Reg<hp_ahb2axi_bresp_err_int_st::HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_st;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_ena`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_ENA =
    crate::Reg<hp_ahb2axi_bresp_err_int_ena::HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_ena;
#[doc = "HP_AHB2AXI_BRESP_ERR_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ahb2axi_bresp_err_int_clr`] module"]
pub type HP_AHB2AXI_BRESP_ERR_INT_CLR =
    crate::Reg<hp_ahb2axi_bresp_err_int_clr::HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod hp_ahb2axi_bresp_err_int_clr;
#[doc = "HP_L2_MEM_ERR_RESP_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_err_resp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_err_resp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_err_resp_ctrl`] module"]
pub type HP_L2_MEM_ERR_RESP_CTRL =
    crate::Reg<hp_l2_mem_err_resp_ctrl::HP_L2_MEM_ERR_RESP_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_l2_mem_err_resp_ctrl;
#[doc = "HP_L2_MEM_AHB_BUFFER_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_ahb_buffer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_ahb_buffer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_l2_mem_ahb_buffer_ctrl`] module"]
pub type HP_L2_MEM_AHB_BUFFER_CTRL =
    crate::Reg<hp_l2_mem_ahb_buffer_ctrl::HP_L2_MEM_AHB_BUFFER_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_l2_mem_ahb_buffer_ctrl;
#[doc = "HP_CORE_DMACTIVE_LPCORE (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_dmactive_lpcore::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_dmactive_lpcore`] module"]
pub type HP_CORE_DMACTIVE_LPCORE =
    crate::Reg<hp_core_dmactive_lpcore::HP_CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "need_des"]
pub mod hp_core_dmactive_lpcore;
#[doc = "HP_CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_err_resp_dis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_err_resp_dis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_err_resp_dis`] module"]
pub type HP_CORE_ERR_RESP_DIS = crate::Reg<hp_core_err_resp_dis::HP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = "need_des"]
pub mod hp_core_err_resp_dis;
#[doc = "HP_CORE_TIMEOUT_INT_RAW (rw) register accessor: Hp core bus timeout interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_timeout_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_raw`] module"]
pub type HP_CORE_TIMEOUT_INT_RAW =
    crate::Reg<hp_core_timeout_int_raw::HP_CORE_TIMEOUT_INT_RAW_SPEC>;
#[doc = "Hp core bus timeout interrupt raw register"]
pub mod hp_core_timeout_int_raw;
#[doc = "HP_CORE_TIMEOUT_INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_st`] module"]
pub type HP_CORE_TIMEOUT_INT_ST = crate::Reg<hp_core_timeout_int_st::HP_CORE_TIMEOUT_INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod hp_core_timeout_int_st;
#[doc = "HP_CORE_TIMEOUT_INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_timeout_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_ena`] module"]
pub type HP_CORE_TIMEOUT_INT_ENA =
    crate::Reg<hp_core_timeout_int_ena::HP_CORE_TIMEOUT_INT_ENA_SPEC>;
#[doc = "masked interrupt register"]
pub mod hp_core_timeout_int_ena;
#[doc = "HP_CORE_TIMEOUT_INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_core_timeout_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core_timeout_int_clr`] module"]
pub type HP_CORE_TIMEOUT_INT_CLR =
    crate::Reg<hp_core_timeout_int_clr::HP_CORE_TIMEOUT_INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod hp_core_timeout_int_clr;
#[doc = "HP_GPIO_O_HYS_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_gpio_o_hys_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_gpio_o_hys_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hys_ctrl0`] module"]
pub type HP_GPIO_O_HYS_CTRL0 = crate::Reg<hp_gpio_o_hys_ctrl0::HP_GPIO_O_HYS_CTRL0_SPEC>;
#[doc = "NA"]
pub mod hp_gpio_o_hys_ctrl0;
#[doc = "HP_GPIO_O_HYS_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_gpio_o_hys_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_gpio_o_hys_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hys_ctrl1`] module"]
pub type HP_GPIO_O_HYS_CTRL1 = crate::Reg<hp_gpio_o_hys_ctrl1::HP_GPIO_O_HYS_CTRL1_SPEC>;
#[doc = "NA"]
pub mod hp_gpio_o_hys_ctrl1;
#[doc = "HP_RSA_PD_CTRL (rw) register accessor: rsa pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rsa_pd_ctrl`] module"]
pub type HP_RSA_PD_CTRL = crate::Reg<hp_rsa_pd_ctrl::HP_RSA_PD_CTRL_SPEC>;
#[doc = "rsa pd ctrl register"]
pub mod hp_rsa_pd_ctrl;
#[doc = "HP_ECC_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ecc_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ecc_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ecc_pd_ctrl`] module"]
pub type HP_ECC_PD_CTRL = crate::Reg<hp_ecc_pd_ctrl::HP_ECC_PD_CTRL_SPEC>;
#[doc = "ecc pd ctrl register"]
pub mod hp_ecc_pd_ctrl;
#[doc = "HP_RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_rng_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_rng_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rng_cfg`] module"]
pub type HP_RNG_CFG = crate::Reg<hp_rng_cfg::HP_RNG_CFG_SPEC>;
#[doc = "rng cfg register"]
pub mod hp_rng_cfg;
#[doc = "HP_UART_PD_CTRL (rw) register accessor: ecc pd ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_uart_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_uart_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_uart_pd_ctrl`] module"]
pub type HP_UART_PD_CTRL = crate::Reg<hp_uart_pd_ctrl::HP_UART_PD_CTRL_SPEC>;
#[doc = "ecc pd ctrl register"]
pub mod hp_uart_pd_ctrl;
#[doc = "HP_PERI_MEM_CLK_FORCE_ON (rw) register accessor: hp peri mem clk force on regpster\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_mem_clk_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri_mem_clk_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_peri_mem_clk_force_on`] module"]
pub type HP_PERI_MEM_CLK_FORCE_ON =
    crate::Reg<hp_peri_mem_clk_force_on::HP_PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "hp peri mem clk force on regpster"]
pub mod hp_peri_mem_clk_force_on;
