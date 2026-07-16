#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver_date: VER_DATE,
    clk_sel_ctrl: CLK_SEL_CTRL,
    sys_ctrl: SYS_CTRL,
    lp_clk_ctrl: LP_CLK_CTRL,
    _reserved4: [u8; 0x08],
    lp_core_boot_addr: LP_CORE_BOOT_ADDR,
    ext_wakeup1: EXT_WAKEUP1,
    ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    lp_tcm_pwr_ctrl: LP_TCM_PWR_CTRL,
    boot_addr_hp_lp_reg: BOOT_ADDR_HP_LP_REG,
    lp_store: [LP_STORE; 16],
    lp_probea_ctrl: LP_PROBEA_CTRL,
    lp_probeb_ctrl: LP_PROBEB_CTRL,
    lp_probe_out: LP_PROBE_OUT,
    _reserved13: [u8; 0x24],
    f2s_apb_brg_cntl: F2S_APB_BRG_CNTL,
    _reserved14: [u8; 0x60],
    usb_ctrl: USB_CTRL,
    _reserved15: [u8; 0x08],
    ana_xpd_pad_group: ANA_XPD_PAD_GROUP,
    lp_tcm_ram_rdn_eco_cs: LP_TCM_RAM_RDN_ECO_CS,
    lp_tcm_ram_rdn_eco_low: LP_TCM_RAM_RDN_ECO_LOW,
    lp_tcm_ram_rdn_eco_high: LP_TCM_RAM_RDN_ECO_HIGH,
    lp_tcm_rom_rdn_eco_cs: LP_TCM_ROM_RDN_ECO_CS,
    lp_tcm_rom_rdn_eco_low: LP_TCM_ROM_RDN_ECO_LOW,
    lp_tcm_rom_rdn_eco_high: LP_TCM_ROM_RDN_ECO_HIGH,
    _reserved22: [u8; 0x08],
    hp_root_clk_ctrl: HP_ROOT_CLK_CTRL,
    _reserved23: [u8; 0x04],
    lp_pmu_rdn_eco_low: LP_PMU_RDN_ECO_LOW,
    lp_pmu_rdn_eco_high: LP_PMU_RDN_ECO_HIGH,
    _reserved25: [u8; 0x14],
    backup_dma_cfg0: BACKUP_DMA_CFG0,
    backup_dma_cfg1: BACKUP_DMA_CFG1,
    backup_dma_cfg2: BACKUP_DMA_CFG2,
    _reserved28: [u8; 0x04],
    boot_addr_hp_core1: BOOT_ADDR_HP_CORE1,
    _reserved29: [u8; 0x08],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    sprf_mem_aux_ctrl: SPRF_MEM_AUX_CTRL,
    _reserved34: [u8; 0x0c],
    lp_cpu_dbg_pc: LP_CPU_DBG_PC,
    lp_cpu_exc_pc: LP_CPU_EXC_PC,
    _reserved36: [u8; 0x08],
    hp_por_rst_bypass_ctrl: HP_POR_RST_BYPASS_CTRL,
    rng_data: RNG_DATA,
    _reserved38: [u8; 0x0c],
    lp_core_ibus_timeout_conf: LP_CORE_IBUS_TIMEOUT_CONF,
    lp_core_dbus_timeout_conf: LP_CORE_DBUS_TIMEOUT_CONF,
    lp_core_err_resp_dis: LP_CORE_ERR_RESP_DIS,
    rng_cfg: RNG_CFG,
    padctrl: PADCTRL,
    _reserved43: [u8; 0x20],
    sys_power_status: SYS_POWER_STATUS,
    ana_dcdc: ANA_DCDC,
    por_cur_st: POR_CUR_ST,
    _reserved46: [u8; 0x04],
    spram_mem_aux_ctrl: SPRAM_MEM_AUX_CTRL,
    uart_mem_lp_ctrl: UART_MEM_LP_CTRL,
    huk_mem_lp_ctrl: HUK_MEM_LP_CTRL,
    efuse_mem_lp_ctrl: EFUSE_MEM_LP_CTRL,
    lp_tcm_mem_lp_ctrl: LP_TCM_MEM_LP_CTRL,
    _reserved51: [u8; 0x0c],
    modem_mem_clk_sel: MODEM_MEM_CLK_SEL,
    lp_peri_timeout_conf: LP_PERI_TIMEOUT_CONF,
    lp_peri_timeout_addr: LP_PERI_TIMEOUT_ADDR,
    lp_peri_timeout_uid: LP_PERI_TIMEOUT_UID,
    rtc_en_amux: RTC_EN_AMUX,
    rtc_en_dmux: RTC_EN_DMUX,
    digmux_pad_dsel: DIGMUX_PAD_DSEL,
    jtag_tdio_mux_sel: JTAG_TDIO_MUX_SEL,
    lp_core_dmactive_lpcore: LP_CORE_DMACTIVE_LPCORE,
    touch_pad_rtc_en_amux: TOUCH_PAD_RTC_EN_AMUX,
    _reserved61: [u8; 0x30],
    hp_gpio_o_hold_ctrl1: HP_GPIO_O_HOLD_CTRL1,
    hp_gpio_o_hold_ctrl0: HP_GPIO_O_HOLD_CTRL0,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn clk_sel_ctrl(&self) -> &CLK_SEL_CTRL {
        &self.clk_sel_ctrl
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SYS_CTRL {
        &self.sys_ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn lp_clk_ctrl(&self) -> &LP_CLK_CTRL {
        &self.lp_clk_ctrl
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn lp_core_boot_addr(&self) -> &LP_CORE_BOOT_ADDR {
        &self.lp_core_boot_addr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &EXT_WAKEUP1 {
        &self.ext_wakeup1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &EXT_WAKEUP1_STATUS {
        &self.ext_wakeup1_status
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn lp_tcm_pwr_ctrl(&self) -> &LP_TCM_PWR_CTRL {
        &self.lp_tcm_pwr_ctrl
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn boot_addr_hp_lp_reg(&self) -> &BOOT_ADDR_HP_LP_REG {
        &self.boot_addr_hp_lp_reg
    }
    #[doc = "0x2c..0x6c - "]
    #[inline(always)]
    pub const fn lp_store(&self, n: usize) -> &LP_STORE {
        &self.lp_store[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c..0x6c - "]
    #[inline(always)]
    pub fn lp_store_iter(&self) -> impl Iterator<Item = &LP_STORE> {
        self.lp_store.iter()
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn lp_probea_ctrl(&self) -> &LP_PROBEA_CTRL {
        &self.lp_probea_ctrl
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn lp_probeb_ctrl(&self) -> &LP_PROBEB_CTRL {
        &self.lp_probeb_ctrl
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn lp_probe_out(&self) -> &LP_PROBE_OUT {
        &self.lp_probe_out
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn f2s_apb_brg_cntl(&self) -> &F2S_APB_BRG_CNTL {
        &self.f2s_apb_brg_cntl
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn usb_ctrl(&self) -> &USB_CTRL {
        &self.usb_ctrl
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn ana_xpd_pad_group(&self) -> &ANA_XPD_PAD_GROUP {
        &self.ana_xpd_pad_group
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_cs(&self) -> &LP_TCM_RAM_RDN_ECO_CS {
        &self.lp_tcm_ram_rdn_eco_cs
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_low(&self) -> &LP_TCM_RAM_RDN_ECO_LOW {
        &self.lp_tcm_ram_rdn_eco_low
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_high(&self) -> &LP_TCM_RAM_RDN_ECO_HIGH {
        &self.lp_tcm_ram_rdn_eco_high
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_cs(&self) -> &LP_TCM_ROM_RDN_ECO_CS {
        &self.lp_tcm_rom_rdn_eco_cs
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_low(&self) -> &LP_TCM_ROM_RDN_ECO_LOW {
        &self.lp_tcm_rom_rdn_eco_low
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_high(&self) -> &LP_TCM_ROM_RDN_ECO_HIGH {
        &self.lp_tcm_rom_rdn_eco_high
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn hp_root_clk_ctrl(&self) -> &HP_ROOT_CLK_CTRL {
        &self.hp_root_clk_ctrl
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_low(&self) -> &LP_PMU_RDN_ECO_LOW {
        &self.lp_pmu_rdn_eco_low
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_high(&self) -> &LP_PMU_RDN_ECO_HIGH {
        &self.lp_pmu_rdn_eco_high
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn backup_dma_cfg0(&self) -> &BACKUP_DMA_CFG0 {
        &self.backup_dma_cfg0
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn backup_dma_cfg1(&self) -> &BACKUP_DMA_CFG1 {
        &self.backup_dma_cfg1
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn backup_dma_cfg2(&self) -> &BACKUP_DMA_CFG2 {
        &self.backup_dma_cfg2
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn boot_addr_hp_core1(&self) -> &BOOT_ADDR_HP_CORE1 {
        &self.boot_addr_hp_core1
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn sprf_mem_aux_ctrl(&self) -> &SPRF_MEM_AUX_CTRL {
        &self.sprf_mem_aux_ctrl
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn lp_cpu_dbg_pc(&self) -> &LP_CPU_DBG_PC {
        &self.lp_cpu_dbg_pc
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn lp_cpu_exc_pc(&self) -> &LP_CPU_EXC_PC {
        &self.lp_cpu_exc_pc
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn hp_por_rst_bypass_ctrl(&self) -> &HP_POR_RST_BYPASS_CTRL {
        &self.hp_por_rst_bypass_ctrl
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn rng_data(&self) -> &RNG_DATA {
        &self.rng_data
    }
    #[doc = "0x1b4 - "]
    #[inline(always)]
    pub const fn lp_core_ibus_timeout_conf(&self) -> &LP_CORE_IBUS_TIMEOUT_CONF {
        &self.lp_core_ibus_timeout_conf
    }
    #[doc = "0x1b8 - "]
    #[inline(always)]
    pub const fn lp_core_dbus_timeout_conf(&self) -> &LP_CORE_DBUS_TIMEOUT_CONF {
        &self.lp_core_dbus_timeout_conf
    }
    #[doc = "0x1bc - "]
    #[inline(always)]
    pub const fn lp_core_err_resp_dis(&self) -> &LP_CORE_ERR_RESP_DIS {
        &self.lp_core_err_resp_dis
    }
    #[doc = "0x1c0 - "]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RNG_CFG {
        &self.rng_cfg
    }
    #[doc = "0x1c4 - "]
    #[inline(always)]
    pub const fn padctrl(&self) -> &PADCTRL {
        &self.padctrl
    }
    #[doc = "0x1e8 - "]
    #[inline(always)]
    pub const fn sys_power_status(&self) -> &SYS_POWER_STATUS {
        &self.sys_power_status
    }
    #[doc = "0x1ec - "]
    #[inline(always)]
    pub const fn ana_dcdc(&self) -> &ANA_DCDC {
        &self.ana_dcdc
    }
    #[doc = "0x1f0 - "]
    #[inline(always)]
    pub const fn por_cur_st(&self) -> &POR_CUR_ST {
        &self.por_cur_st
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn spram_mem_aux_ctrl(&self) -> &SPRAM_MEM_AUX_CTRL {
        &self.spram_mem_aux_ctrl
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn uart_mem_lp_ctrl(&self) -> &UART_MEM_LP_CTRL {
        &self.uart_mem_lp_ctrl
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn huk_mem_lp_ctrl(&self) -> &HUK_MEM_LP_CTRL {
        &self.huk_mem_lp_ctrl
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn efuse_mem_lp_ctrl(&self) -> &EFUSE_MEM_LP_CTRL {
        &self.efuse_mem_lp_ctrl
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn lp_tcm_mem_lp_ctrl(&self) -> &LP_TCM_MEM_LP_CTRL {
        &self.lp_tcm_mem_lp_ctrl
    }
    #[doc = "0x218 - "]
    #[inline(always)]
    pub const fn modem_mem_clk_sel(&self) -> &MODEM_MEM_CLK_SEL {
        &self.modem_mem_clk_sel
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn lp_peri_timeout_conf(&self) -> &LP_PERI_TIMEOUT_CONF {
        &self.lp_peri_timeout_conf
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn lp_peri_timeout_addr(&self) -> &LP_PERI_TIMEOUT_ADDR {
        &self.lp_peri_timeout_addr
    }
    #[doc = "0x224 - "]
    #[inline(always)]
    pub const fn lp_peri_timeout_uid(&self) -> &LP_PERI_TIMEOUT_UID {
        &self.lp_peri_timeout_uid
    }
    #[doc = "0x228 - "]
    #[inline(always)]
    pub const fn rtc_en_amux(&self) -> &RTC_EN_AMUX {
        &self.rtc_en_amux
    }
    #[doc = "0x22c - "]
    #[inline(always)]
    pub const fn rtc_en_dmux(&self) -> &RTC_EN_DMUX {
        &self.rtc_en_dmux
    }
    #[doc = "0x230 - "]
    #[inline(always)]
    pub const fn digmux_pad_dsel(&self) -> &DIGMUX_PAD_DSEL {
        &self.digmux_pad_dsel
    }
    #[doc = "0x234 - "]
    #[inline(always)]
    pub const fn jtag_tdio_mux_sel(&self) -> &JTAG_TDIO_MUX_SEL {
        &self.jtag_tdio_mux_sel
    }
    #[doc = "0x238 - "]
    #[inline(always)]
    pub const fn lp_core_dmactive_lpcore(&self) -> &LP_CORE_DMACTIVE_LPCORE {
        &self.lp_core_dmactive_lpcore
    }
    #[doc = "0x23c - "]
    #[inline(always)]
    pub const fn touch_pad_rtc_en_amux(&self) -> &TOUCH_PAD_RTC_EN_AMUX {
        &self.touch_pad_rtc_en_amux
    }
    #[doc = "0x270 - "]
    #[inline(always)]
    pub const fn hp_gpio_o_hold_ctrl1(&self) -> &HP_GPIO_O_HOLD_CTRL1 {
        &self.hp_gpio_o_hold_ctrl1
    }
    #[doc = "0x274 - "]
    #[inline(always)]
    pub const fn hp_gpio_o_hold_ctrl0(&self) -> &HP_GPIO_O_HOLD_CTRL0 {
        &self.hp_gpio_o_hold_ctrl0
    }
}
#[doc = "VER_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = ""]
pub mod ver_date;
#[doc = "CLK_SEL_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_sel_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sel_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel_ctrl`] module"]
pub type CLK_SEL_CTRL = crate::Reg<clk_sel_ctrl::CLK_SEL_CTRL_SPEC>;
#[doc = ""]
pub mod clk_sel_ctrl;
#[doc = "SYS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`] module"]
pub type SYS_CTRL = crate::Reg<sys_ctrl::SYS_CTRL_SPEC>;
#[doc = ""]
pub mod sys_ctrl;
#[doc = "LP_CLK_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_ctrl`] module"]
pub type LP_CLK_CTRL = crate::Reg<lp_clk_ctrl::LP_CLK_CTRL_SPEC>;
#[doc = ""]
pub mod lp_clk_ctrl;
#[doc = "LP_CORE_BOOT_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_boot_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_boot_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_boot_addr`] module"]
pub type LP_CORE_BOOT_ADDR = crate::Reg<lp_core_boot_addr::LP_CORE_BOOT_ADDR_SPEC>;
#[doc = ""]
pub mod lp_core_boot_addr;
#[doc = "EXT_WAKEUP1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = ""]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = ""]
pub mod ext_wakeup1_status;
#[doc = "LP_TCM_PWR_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_pwr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_pwr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_pwr_ctrl`] module"]
pub type LP_TCM_PWR_CTRL = crate::Reg<lp_tcm_pwr_ctrl::LP_TCM_PWR_CTRL_SPEC>;
#[doc = ""]
pub mod lp_tcm_pwr_ctrl;
#[doc = "BOOT_ADDR_HP_LP_REG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_lp_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_lp_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_lp_reg`] module"]
pub type BOOT_ADDR_HP_LP_REG = crate::Reg<boot_addr_hp_lp_reg::BOOT_ADDR_HP_LP_REG_SPEC>;
#[doc = ""]
pub mod boot_addr_hp_lp_reg;
#[doc = "LP_STORE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_store::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_store::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store`] module"]
pub type LP_STORE = crate::Reg<lp_store::LP_STORE_SPEC>;
#[doc = ""]
pub mod lp_store;
#[doc = "LP_PROBEA_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probea_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_probea_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probea_ctrl`] module"]
pub type LP_PROBEA_CTRL = crate::Reg<lp_probea_ctrl::LP_PROBEA_CTRL_SPEC>;
#[doc = ""]
pub mod lp_probea_ctrl;
#[doc = "LP_PROBEB_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probeb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_probeb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probeb_ctrl`] module"]
pub type LP_PROBEB_CTRL = crate::Reg<lp_probeb_ctrl::LP_PROBEB_CTRL_SPEC>;
#[doc = ""]
pub mod lp_probeb_ctrl;
#[doc = "LP_PROBE_OUT (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probe_out::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probe_out`] module"]
pub type LP_PROBE_OUT = crate::Reg<lp_probe_out::LP_PROBE_OUT_SPEC>;
#[doc = ""]
pub mod lp_probe_out;
#[doc = "F2S_APB_BRG_CNTL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`f2s_apb_brg_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2s_apb_brg_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2s_apb_brg_cntl`] module"]
pub type F2S_APB_BRG_CNTL = crate::Reg<f2s_apb_brg_cntl::F2S_APB_BRG_CNTL_SPEC>;
#[doc = ""]
pub mod f2s_apb_brg_cntl;
#[doc = "USB_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`] module"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = ""]
pub mod usb_ctrl;
#[doc = "ANA_XPD_PAD_GROUP (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ana_xpd_pad_group::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_xpd_pad_group::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_xpd_pad_group`] module"]
pub type ANA_XPD_PAD_GROUP = crate::Reg<ana_xpd_pad_group::ANA_XPD_PAD_GROUP_SPEC>;
#[doc = ""]
pub mod ana_xpd_pad_group;
#[doc = "LP_TCM_RAM_RDN_ECO_CS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_cs`] module"]
pub type LP_TCM_RAM_RDN_ECO_CS = crate::Reg<lp_tcm_ram_rdn_eco_cs::LP_TCM_RAM_RDN_ECO_CS_SPEC>;
#[doc = ""]
pub mod lp_tcm_ram_rdn_eco_cs;
#[doc = "LP_TCM_RAM_RDN_ECO_LOW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_low`] module"]
pub type LP_TCM_RAM_RDN_ECO_LOW = crate::Reg<lp_tcm_ram_rdn_eco_low::LP_TCM_RAM_RDN_ECO_LOW_SPEC>;
#[doc = ""]
pub mod lp_tcm_ram_rdn_eco_low;
#[doc = "LP_TCM_RAM_RDN_ECO_HIGH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_high`] module"]
pub type LP_TCM_RAM_RDN_ECO_HIGH =
    crate::Reg<lp_tcm_ram_rdn_eco_high::LP_TCM_RAM_RDN_ECO_HIGH_SPEC>;
#[doc = ""]
pub mod lp_tcm_ram_rdn_eco_high;
#[doc = "LP_TCM_ROM_RDN_ECO_CS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_cs`] module"]
pub type LP_TCM_ROM_RDN_ECO_CS = crate::Reg<lp_tcm_rom_rdn_eco_cs::LP_TCM_ROM_RDN_ECO_CS_SPEC>;
#[doc = ""]
pub mod lp_tcm_rom_rdn_eco_cs;
#[doc = "LP_TCM_ROM_RDN_ECO_LOW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_low`] module"]
pub type LP_TCM_ROM_RDN_ECO_LOW = crate::Reg<lp_tcm_rom_rdn_eco_low::LP_TCM_ROM_RDN_ECO_LOW_SPEC>;
#[doc = ""]
pub mod lp_tcm_rom_rdn_eco_low;
#[doc = "LP_TCM_ROM_RDN_ECO_HIGH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_high`] module"]
pub type LP_TCM_ROM_RDN_ECO_HIGH =
    crate::Reg<lp_tcm_rom_rdn_eco_high::LP_TCM_ROM_RDN_ECO_HIGH_SPEC>;
#[doc = ""]
pub mod lp_tcm_rom_rdn_eco_high;
#[doc = "HP_ROOT_CLK_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_root_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_root_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_root_clk_ctrl`] module"]
pub type HP_ROOT_CLK_CTRL = crate::Reg<hp_root_clk_ctrl::HP_ROOT_CLK_CTRL_SPEC>;
#[doc = ""]
pub mod hp_root_clk_ctrl;
#[doc = "LP_PMU_RDN_ECO_LOW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_low`] module"]
pub type LP_PMU_RDN_ECO_LOW = crate::Reg<lp_pmu_rdn_eco_low::LP_PMU_RDN_ECO_LOW_SPEC>;
#[doc = ""]
pub mod lp_pmu_rdn_eco_low;
#[doc = "LP_PMU_RDN_ECO_HIGH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pmu_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_high`] module"]
pub type LP_PMU_RDN_ECO_HIGH = crate::Reg<lp_pmu_rdn_eco_high::LP_PMU_RDN_ECO_HIGH_SPEC>;
#[doc = ""]
pub mod lp_pmu_rdn_eco_high;
#[doc = "BACKUP_DMA_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg0`] module"]
pub type BACKUP_DMA_CFG0 = crate::Reg<backup_dma_cfg0::BACKUP_DMA_CFG0_SPEC>;
#[doc = ""]
pub mod backup_dma_cfg0;
#[doc = "BACKUP_DMA_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg1`] module"]
pub type BACKUP_DMA_CFG1 = crate::Reg<backup_dma_cfg1::BACKUP_DMA_CFG1_SPEC>;
#[doc = ""]
pub mod backup_dma_cfg1;
#[doc = "BACKUP_DMA_CFG2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`backup_dma_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg2`] module"]
pub type BACKUP_DMA_CFG2 = crate::Reg<backup_dma_cfg2::BACKUP_DMA_CFG2_SPEC>;
#[doc = ""]
pub mod backup_dma_cfg2;
#[doc = "BOOT_ADDR_HP_CORE1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_core1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_core1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_core1`] module"]
pub type BOOT_ADDR_HP_CORE1 = crate::Reg<boot_addr_hp_core1::BOOT_ADDR_HP_CORE1_SPEC>;
#[doc = ""]
pub mod boot_addr_hp_core1;
#[doc = "INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "SPRF_MEM_AUX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sprf_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprf_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprf_mem_aux_ctrl`] module"]
pub type SPRF_MEM_AUX_CTRL = crate::Reg<sprf_mem_aux_ctrl::SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = ""]
pub mod sprf_mem_aux_ctrl;
#[doc = "LP_CPU_DBG_PC (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_dbg_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_dbg_pc`] module"]
pub type LP_CPU_DBG_PC = crate::Reg<lp_cpu_dbg_pc::LP_CPU_DBG_PC_SPEC>;
#[doc = ""]
pub mod lp_cpu_dbg_pc;
#[doc = "LP_CPU_EXC_PC (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_exc_pc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_exc_pc`] module"]
pub type LP_CPU_EXC_PC = crate::Reg<lp_cpu_exc_pc::LP_CPU_EXC_PC_SPEC>;
#[doc = ""]
pub mod lp_cpu_exc_pc;
#[doc = "HP_POR_RST_BYPASS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_por_rst_bypass_ctrl`] module"]
pub type HP_POR_RST_BYPASS_CTRL = crate::Reg<hp_por_rst_bypass_ctrl::HP_POR_RST_BYPASS_CTRL_SPEC>;
#[doc = ""]
pub mod hp_por_rst_bypass_ctrl;
#[doc = "RNG_DATA (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data`] module"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = ""]
pub mod rng_data;
#[doc = "LP_CORE_IBUS_TIMEOUT_CONF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_ibus_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_ibus_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_ibus_timeout_conf`] module"]
pub type LP_CORE_IBUS_TIMEOUT_CONF =
    crate::Reg<lp_core_ibus_timeout_conf::LP_CORE_IBUS_TIMEOUT_CONF_SPEC>;
#[doc = ""]
pub mod lp_core_ibus_timeout_conf;
#[doc = "LP_CORE_DBUS_TIMEOUT_CONF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_dbus_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_dbus_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_dbus_timeout_conf`] module"]
pub type LP_CORE_DBUS_TIMEOUT_CONF =
    crate::Reg<lp_core_dbus_timeout_conf::LP_CORE_DBUS_TIMEOUT_CONF_SPEC>;
#[doc = ""]
pub mod lp_core_dbus_timeout_conf;
#[doc = "LP_CORE_ERR_RESP_DIS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_err_resp_dis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_core_err_resp_dis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_err_resp_dis`] module"]
pub type LP_CORE_ERR_RESP_DIS = crate::Reg<lp_core_err_resp_dis::LP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = ""]
pub mod lp_core_err_resp_dis;
#[doc = "RNG_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
pub type RNG_CFG = crate::Reg<rng_cfg::RNG_CFG_SPEC>;
#[doc = ""]
pub mod rng_cfg;
#[doc = "PADCTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`padctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padctrl`] module"]
pub type PADCTRL = crate::Reg<padctrl::PADCTRL_SPEC>;
#[doc = ""]
pub mod padctrl;
#[doc = "SYS_POWER_STATUS (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sys_power_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_power_status`] module"]
pub type SYS_POWER_STATUS = crate::Reg<sys_power_status::SYS_POWER_STATUS_SPEC>;
#[doc = ""]
pub mod sys_power_status;
#[doc = "ANA_DCDC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ana_dcdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_dcdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_dcdc`] module"]
pub type ANA_DCDC = crate::Reg<ana_dcdc::ANA_DCDC_SPEC>;
#[doc = ""]
pub mod ana_dcdc;
#[doc = "POR_CUR_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`por_cur_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_cur_st`] module"]
pub type POR_CUR_ST = crate::Reg<por_cur_st::POR_CUR_ST_SPEC>;
#[doc = ""]
pub mod por_cur_st;
#[doc = "SPRAM_MEM_AUX_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spram_mem_aux_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spram_mem_aux_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spram_mem_aux_ctrl`] module"]
pub type SPRAM_MEM_AUX_CTRL = crate::Reg<spram_mem_aux_ctrl::SPRAM_MEM_AUX_CTRL_SPEC>;
#[doc = ""]
pub mod spram_mem_aux_ctrl;
#[doc = "UART_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uart_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_mem_lp_ctrl`] module"]
pub type UART_MEM_LP_CTRL = crate::Reg<uart_mem_lp_ctrl::UART_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod uart_mem_lp_ctrl;
#[doc = "HUK_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`huk_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@huk_mem_lp_ctrl`] module"]
pub type HUK_MEM_LP_CTRL = crate::Reg<huk_mem_lp_ctrl::HUK_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod huk_mem_lp_ctrl;
#[doc = "EFUSE_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse_mem_lp_ctrl`] module"]
pub type EFUSE_MEM_LP_CTRL = crate::Reg<efuse_mem_lp_ctrl::EFUSE_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod efuse_mem_lp_ctrl;
#[doc = "LP_TCM_MEM_LP_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_tcm_mem_lp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_tcm_mem_lp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_mem_lp_ctrl`] module"]
pub type LP_TCM_MEM_LP_CTRL = crate::Reg<lp_tcm_mem_lp_ctrl::LP_TCM_MEM_LP_CTRL_SPEC>;
#[doc = ""]
pub mod lp_tcm_mem_lp_ctrl;
#[doc = "MODEM_MEM_CLK_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`modem_mem_clk_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_mem_clk_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem_mem_clk_sel`] module"]
pub type MODEM_MEM_CLK_SEL = crate::Reg<modem_mem_clk_sel::MODEM_MEM_CLK_SEL_SPEC>;
#[doc = ""]
pub mod modem_mem_clk_sel;
#[doc = "LP_PERI_TIMEOUT_CONF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_timeout_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_conf`] module"]
pub type LP_PERI_TIMEOUT_CONF = crate::Reg<lp_peri_timeout_conf::LP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = ""]
pub mod lp_peri_timeout_conf;
#[doc = "LP_PERI_TIMEOUT_ADDR (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_addr`] module"]
pub type LP_PERI_TIMEOUT_ADDR = crate::Reg<lp_peri_timeout_addr::LP_PERI_TIMEOUT_ADDR_SPEC>;
#[doc = ""]
pub mod lp_peri_timeout_addr;
#[doc = "LP_PERI_TIMEOUT_UID (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_peri_timeout_uid`] module"]
pub type LP_PERI_TIMEOUT_UID = crate::Reg<lp_peri_timeout_uid::LP_PERI_TIMEOUT_UID_SPEC>;
#[doc = ""]
pub mod lp_peri_timeout_uid;
#[doc = "RTC_EN_AMUX (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_en_amux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_en_amux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_en_amux`] module"]
pub type RTC_EN_AMUX = crate::Reg<rtc_en_amux::RTC_EN_AMUX_SPEC>;
#[doc = ""]
pub mod rtc_en_amux;
#[doc = "RTC_EN_DMUX (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_en_dmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_en_dmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_en_dmux`] module"]
pub type RTC_EN_DMUX = crate::Reg<rtc_en_dmux::RTC_EN_DMUX_SPEC>;
#[doc = ""]
pub mod rtc_en_dmux;
#[doc = "DIGMUX_PAD_DSEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`digmux_pad_dsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`digmux_pad_dsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digmux_pad_dsel`] module"]
pub type DIGMUX_PAD_DSEL = crate::Reg<digmux_pad_dsel::DIGMUX_PAD_DSEL_SPEC>;
#[doc = ""]
pub mod digmux_pad_dsel;
#[doc = "JTAG_TDIO_MUX_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_tdio_mux_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag_tdio_mux_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag_tdio_mux_sel`] module"]
pub type JTAG_TDIO_MUX_SEL = crate::Reg<jtag_tdio_mux_sel::JTAG_TDIO_MUX_SEL_SPEC>;
#[doc = ""]
pub mod jtag_tdio_mux_sel;
#[doc = "LP_CORE_DMACTIVE_LPCORE (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_dmactive_lpcore::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_dmactive_lpcore`] module"]
pub type LP_CORE_DMACTIVE_LPCORE =
    crate::Reg<lp_core_dmactive_lpcore::LP_CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = ""]
pub mod lp_core_dmactive_lpcore;
#[doc = "TOUCH_PAD_RTC_EN_AMUX (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad_rtc_en_amux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad_rtc_en_amux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pad_rtc_en_amux`] module"]
pub type TOUCH_PAD_RTC_EN_AMUX = crate::Reg<touch_pad_rtc_en_amux::TOUCH_PAD_RTC_EN_AMUX_SPEC>;
#[doc = ""]
pub mod touch_pad_rtc_en_amux;
#[doc = "HP_GPIO_O_HOLD_CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_o_hold_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hold_ctrl1`] module"]
pub type HP_GPIO_O_HOLD_CTRL1 = crate::Reg<hp_gpio_o_hold_ctrl1::HP_GPIO_O_HOLD_CTRL1_SPEC>;
#[doc = ""]
pub mod hp_gpio_o_hold_ctrl1;
#[doc = "HP_GPIO_O_HOLD_CTRL0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_o_hold_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_gpio_o_hold_ctrl0`] module"]
pub type HP_GPIO_O_HOLD_CTRL0 = crate::Reg<hp_gpio_o_hold_ctrl0::HP_GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = ""]
pub mod hp_gpio_o_hold_ctrl0;
