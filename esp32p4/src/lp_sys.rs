#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    lp_sys_ver_date: LP_SYS_VER_DATE,
    clk_sel_ctrl: CLK_SEL_CTRL,
    sys_ctrl: SYS_CTRL,
    lp_clk_ctrl: LP_CLK_CTRL,
    lp_rst_ctrl: LP_RST_CTRL,
    _reserved5: [u8; 0x04],
    lp_core_boot_addr: LP_CORE_BOOT_ADDR,
    ext_wakeup1: EXT_WAKEUP1,
    ext_wakeup1_status: EXT_WAKEUP1_STATUS,
    lp_tcm_pwr_ctrl: LP_TCM_PWR_CTRL,
    boot_addr_hp_lp: BOOT_ADDR_HP_LP,
    lp_store0: LP_STORE0,
    lp_store1: LP_STORE1,
    lp_store2: LP_STORE2,
    lp_store3: LP_STORE3,
    lp_store4: LP_STORE4,
    lp_store5: LP_STORE5,
    lp_store6: LP_STORE6,
    lp_store7: LP_STORE7,
    lp_store8: LP_STORE8,
    lp_store9: LP_STORE9,
    lp_store10: LP_STORE10,
    lp_store11: LP_STORE11,
    lp_store12: LP_STORE12,
    lp_store13: LP_STORE13,
    lp_store14: LP_STORE14,
    lp_store15: LP_STORE15,
    lp_probea_ctrl: LP_PROBEA_CTRL,
    lp_probeb_ctrl: LP_PROBEB_CTRL,
    lp_probe_out: LP_PROBE_OUT,
    _reserved29: [u8; 0x24],
    f2s_apb_brg_cntl: F2S_APB_BRG_CNTL,
    _reserved30: [u8; 0x60],
    usb_ctrl: USB_CTRL,
    _reserved31: [u8; 0x08],
    ana_xpd_pad_group: ANA_XPD_PAD_GROUP,
    lp_tcm_ram_rdn_eco_cs: LP_TCM_RAM_RDN_ECO_CS,
    lp_tcm_ram_rdn_eco_low: LP_TCM_RAM_RDN_ECO_LOW,
    lp_tcm_ram_rdn_eco_high: LP_TCM_RAM_RDN_ECO_HIGH,
    lp_tcm_rom_rdn_eco_cs: LP_TCM_ROM_RDN_ECO_CS,
    lp_tcm_rom_rdn_eco_low: LP_TCM_ROM_RDN_ECO_LOW,
    lp_tcm_rom_rdn_eco_high: LP_TCM_ROM_RDN_ECO_HIGH,
    _reserved38: [u8; 0x08],
    hp_root_clk_ctrl: HP_ROOT_CLK_CTRL,
    _reserved39: [u8; 0x04],
    lp_pmu_rdn_eco_low: LP_PMU_RDN_ECO_LOW,
    lp_pmu_rdn_eco_high: LP_PMU_RDN_ECO_HIGH,
    _reserved41: [u8; 0x08],
    pad_comp0: PAD_COMP0,
    pad_comp1: PAD_COMP1,
    _reserved43: [u8; 0x04],
    backup_dma_cfg0: BACKUP_DMA_CFG0,
    backup_dma_cfg1: BACKUP_DMA_CFG1,
    backup_dma_cfg2: BACKUP_DMA_CFG2,
    _reserved46: [u8; 0x04],
    boot_addr_hp_core1: BOOT_ADDR_HP_CORE1,
    lp_addrhole_addr: LP_ADDRHOLE_ADDR,
    lp_addrhole_info: LP_ADDRHOLE_INFO,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    hp_mem_aux_ctrl: HP_MEM_AUX_CTRL,
    lp_mem_aux_ctrl: LP_MEM_AUX_CTRL,
    hp_rom_aux_ctrl: HP_ROM_AUX_CTRL,
    lp_rom_aux_ctrl: LP_ROM_AUX_CTRL,
    lp_cpu_dbg_pc: LP_CPU_DBG_PC,
    lp_cpu_exc_pc: LP_CPU_EXC_PC,
    idbus_addrhole_addr: IDBUS_ADDRHOLE_ADDR,
    idbus_addrhole_info: IDBUS_ADDRHOLE_INFO,
    hp_por_rst_bypass_ctrl: HP_POR_RST_BYPASS_CTRL,
    rng_data: RNG_DATA,
    _reserved63: [u8; 0x08],
    lp_core_ahb_timeout: LP_CORE_AHB_TIMEOUT,
    lp_core_ibus_timeout: LP_CORE_IBUS_TIMEOUT,
    lp_core_dbus_timeout: LP_CORE_DBUS_TIMEOUT,
    lp_core_err_resp_dis: LP_CORE_ERR_RESP_DIS,
    rng_cfg: RNG_CFG,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_sys_ver_date(&self) -> &LP_SYS_VER_DATE {
        &self.lp_sys_ver_date
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn clk_sel_ctrl(&self) -> &CLK_SEL_CTRL {
        &self.clk_sel_ctrl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SYS_CTRL {
        &self.sys_ctrl
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_clk_ctrl(&self) -> &LP_CLK_CTRL {
        &self.lp_clk_ctrl
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_rst_ctrl(&self) -> &LP_RST_CTRL {
        &self.lp_rst_ctrl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_core_boot_addr(&self) -> &LP_CORE_BOOT_ADDR {
        &self.lp_core_boot_addr
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup1(&self) -> &EXT_WAKEUP1 {
        &self.ext_wakeup1
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> &EXT_WAKEUP1_STATUS {
        &self.ext_wakeup1_status
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_pwr_ctrl(&self) -> &LP_TCM_PWR_CTRL {
        &self.lp_tcm_pwr_ctrl
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn boot_addr_hp_lp(&self) -> &BOOT_ADDR_HP_LP {
        &self.boot_addr_hp_lp
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_store0(&self) -> &LP_STORE0 {
        &self.lp_store0
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_store1(&self) -> &LP_STORE1 {
        &self.lp_store1
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_store2(&self) -> &LP_STORE2 {
        &self.lp_store2
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_store3(&self) -> &LP_STORE3 {
        &self.lp_store3
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_store4(&self) -> &LP_STORE4 {
        &self.lp_store4
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn lp_store5(&self) -> &LP_STORE5 {
        &self.lp_store5
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn lp_store6(&self) -> &LP_STORE6 {
        &self.lp_store6
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn lp_store7(&self) -> &LP_STORE7 {
        &self.lp_store7
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn lp_store8(&self) -> &LP_STORE8 {
        &self.lp_store8
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn lp_store9(&self) -> &LP_STORE9 {
        &self.lp_store9
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn lp_store10(&self) -> &LP_STORE10 {
        &self.lp_store10
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn lp_store11(&self) -> &LP_STORE11 {
        &self.lp_store11
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn lp_store12(&self) -> &LP_STORE12 {
        &self.lp_store12
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn lp_store13(&self) -> &LP_STORE13 {
        &self.lp_store13
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn lp_store14(&self) -> &LP_STORE14 {
        &self.lp_store14
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn lp_store15(&self) -> &LP_STORE15 {
        &self.lp_store15
    }
    #[doc = "0x6c - need_des"]
    #[inline(always)]
    pub const fn lp_probea_ctrl(&self) -> &LP_PROBEA_CTRL {
        &self.lp_probea_ctrl
    }
    #[doc = "0x70 - need_des"]
    #[inline(always)]
    pub const fn lp_probeb_ctrl(&self) -> &LP_PROBEB_CTRL {
        &self.lp_probeb_ctrl
    }
    #[doc = "0x74 - need_des"]
    #[inline(always)]
    pub const fn lp_probe_out(&self) -> &LP_PROBE_OUT {
        &self.lp_probe_out
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn f2s_apb_brg_cntl(&self) -> &F2S_APB_BRG_CNTL {
        &self.f2s_apb_brg_cntl
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn usb_ctrl(&self) -> &USB_CTRL {
        &self.usb_ctrl
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn ana_xpd_pad_group(&self) -> &ANA_XPD_PAD_GROUP {
        &self.ana_xpd_pad_group
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_cs(&self) -> &LP_TCM_RAM_RDN_ECO_CS {
        &self.lp_tcm_ram_rdn_eco_cs
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_low(&self) -> &LP_TCM_RAM_RDN_ECO_LOW {
        &self.lp_tcm_ram_rdn_eco_low
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_high(&self) -> &LP_TCM_RAM_RDN_ECO_HIGH {
        &self.lp_tcm_ram_rdn_eco_high
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_cs(&self) -> &LP_TCM_ROM_RDN_ECO_CS {
        &self.lp_tcm_rom_rdn_eco_cs
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_low(&self) -> &LP_TCM_ROM_RDN_ECO_LOW {
        &self.lp_tcm_rom_rdn_eco_low
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_high(&self) -> &LP_TCM_ROM_RDN_ECO_HIGH {
        &self.lp_tcm_rom_rdn_eco_high
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn hp_root_clk_ctrl(&self) -> &HP_ROOT_CLK_CTRL {
        &self.hp_root_clk_ctrl
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_low(&self) -> &LP_PMU_RDN_ECO_LOW {
        &self.lp_pmu_rdn_eco_low
    }
    #[doc = "0x13c - need_des"]
    #[inline(always)]
    pub const fn lp_pmu_rdn_eco_high(&self) -> &LP_PMU_RDN_ECO_HIGH {
        &self.lp_pmu_rdn_eco_high
    }
    #[doc = "0x148 - need_des"]
    #[inline(always)]
    pub const fn pad_comp0(&self) -> &PAD_COMP0 {
        &self.pad_comp0
    }
    #[doc = "0x14c - need_des"]
    #[inline(always)]
    pub const fn pad_comp1(&self) -> &PAD_COMP1 {
        &self.pad_comp1
    }
    #[doc = "0x154 - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg0(&self) -> &BACKUP_DMA_CFG0 {
        &self.backup_dma_cfg0
    }
    #[doc = "0x158 - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg1(&self) -> &BACKUP_DMA_CFG1 {
        &self.backup_dma_cfg1
    }
    #[doc = "0x15c - need_des"]
    #[inline(always)]
    pub const fn backup_dma_cfg2(&self) -> &BACKUP_DMA_CFG2 {
        &self.backup_dma_cfg2
    }
    #[doc = "0x164 - need_des"]
    #[inline(always)]
    pub const fn boot_addr_hp_core1(&self) -> &BOOT_ADDR_HP_CORE1 {
        &self.boot_addr_hp_core1
    }
    #[doc = "0x168 - need_des"]
    #[inline(always)]
    pub const fn lp_addrhole_addr(&self) -> &LP_ADDRHOLE_ADDR {
        &self.lp_addrhole_addr
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn lp_addrhole_info(&self) -> &LP_ADDRHOLE_INFO {
        &self.lp_addrhole_info
    }
    #[doc = "0x170 - raw interrupt register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x174 - masked interrupt register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x178 - masked interrupt register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x17c - interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn hp_mem_aux_ctrl(&self) -> &HP_MEM_AUX_CTRL {
        &self.hp_mem_aux_ctrl
    }
    #[doc = "0x184 - need_des"]
    #[inline(always)]
    pub const fn lp_mem_aux_ctrl(&self) -> &LP_MEM_AUX_CTRL {
        &self.lp_mem_aux_ctrl
    }
    #[doc = "0x188 - need_des"]
    #[inline(always)]
    pub const fn hp_rom_aux_ctrl(&self) -> &HP_ROM_AUX_CTRL {
        &self.hp_rom_aux_ctrl
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn lp_rom_aux_ctrl(&self) -> &LP_ROM_AUX_CTRL {
        &self.lp_rom_aux_ctrl
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_dbg_pc(&self) -> &LP_CPU_DBG_PC {
        &self.lp_cpu_dbg_pc
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn lp_cpu_exc_pc(&self) -> &LP_CPU_EXC_PC {
        &self.lp_cpu_exc_pc
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn idbus_addrhole_addr(&self) -> &IDBUS_ADDRHOLE_ADDR {
        &self.idbus_addrhole_addr
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn idbus_addrhole_info(&self) -> &IDBUS_ADDRHOLE_INFO {
        &self.idbus_addrhole_info
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn hp_por_rst_bypass_ctrl(&self) -> &HP_POR_RST_BYPASS_CTRL {
        &self.hp_por_rst_bypass_ctrl
    }
    #[doc = "0x1a4 - rng data register"]
    #[inline(always)]
    pub const fn rng_data(&self) -> &RNG_DATA {
        &self.rng_data
    }
    #[doc = "0x1b0 - need_des"]
    #[inline(always)]
    pub const fn lp_core_ahb_timeout(&self) -> &LP_CORE_AHB_TIMEOUT {
        &self.lp_core_ahb_timeout
    }
    #[doc = "0x1b4 - need_des"]
    #[inline(always)]
    pub const fn lp_core_ibus_timeout(&self) -> &LP_CORE_IBUS_TIMEOUT {
        &self.lp_core_ibus_timeout
    }
    #[doc = "0x1b8 - need_des"]
    #[inline(always)]
    pub const fn lp_core_dbus_timeout(&self) -> &LP_CORE_DBUS_TIMEOUT {
        &self.lp_core_dbus_timeout
    }
    #[doc = "0x1bc - need_des"]
    #[inline(always)]
    pub const fn lp_core_err_resp_dis(&self) -> &LP_CORE_ERR_RESP_DIS {
        &self.lp_core_err_resp_dis
    }
    #[doc = "0x1c0 - rng cfg register"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RNG_CFG {
        &self.rng_cfg
    }
}
#[doc = "LP_SYS_VER_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sys_ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sys_ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sys_ver_date`] module"]
pub type LP_SYS_VER_DATE = crate::Reg<lp_sys_ver_date::LP_SYS_VER_DATE_SPEC>;
#[doc = "need_des"]
pub mod lp_sys_ver_date;
#[doc = "CLK_SEL_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sel_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sel_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel_ctrl`] module"]
pub type CLK_SEL_CTRL = crate::Reg<clk_sel_ctrl::CLK_SEL_CTRL_SPEC>;
#[doc = "need_des"]
pub mod clk_sel_ctrl;
#[doc = "SYS_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`] module"]
pub type SYS_CTRL = crate::Reg<sys_ctrl::SYS_CTRL_SPEC>;
#[doc = "need_des"]
pub mod sys_ctrl;
#[doc = "LP_CLK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_clk_ctrl`] module"]
pub type LP_CLK_CTRL = crate::Reg<lp_clk_ctrl::LP_CLK_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_clk_ctrl;
#[doc = "LP_RST_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rst_ctrl`] module"]
pub type LP_RST_CTRL = crate::Reg<lp_rst_ctrl::LP_RST_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_rst_ctrl;
#[doc = "LP_CORE_BOOT_ADDR (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_boot_addr`] module"]
pub type LP_CORE_BOOT_ADDR = crate::Reg<lp_core_boot_addr::LP_CORE_BOOT_ADDR_SPEC>;
#[doc = "need_des"]
pub mod lp_core_boot_addr;
#[doc = "EXT_WAKEUP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1`] module"]
pub type EXT_WAKEUP1 = crate::Reg<ext_wakeup1::EXT_WAKEUP1_SPEC>;
#[doc = "need_des"]
pub mod ext_wakeup1;
#[doc = "EXT_WAKEUP1_STATUS (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup1_status`] module"]
pub type EXT_WAKEUP1_STATUS = crate::Reg<ext_wakeup1_status::EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "need_des"]
pub mod ext_wakeup1_status;
#[doc = "LP_TCM_PWR_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_pwr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_pwr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_pwr_ctrl`] module"]
pub type LP_TCM_PWR_CTRL = crate::Reg<lp_tcm_pwr_ctrl::LP_TCM_PWR_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_pwr_ctrl;
#[doc = "BOOT_ADDR_HP_LP (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_addr_hp_lp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_addr_hp_lp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_lp`] module"]
pub type BOOT_ADDR_HP_LP = crate::Reg<boot_addr_hp_lp::BOOT_ADDR_HP_LP_SPEC>;
#[doc = "need_des"]
pub mod boot_addr_hp_lp;
#[doc = "LP_STORE0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store0`] module"]
pub type LP_STORE0 = crate::Reg<lp_store0::LP_STORE0_SPEC>;
#[doc = "need_des"]
pub mod lp_store0;
#[doc = "LP_STORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store1`] module"]
pub type LP_STORE1 = crate::Reg<lp_store1::LP_STORE1_SPEC>;
#[doc = "need_des"]
pub mod lp_store1;
#[doc = "LP_STORE2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store2`] module"]
pub type LP_STORE2 = crate::Reg<lp_store2::LP_STORE2_SPEC>;
#[doc = "need_des"]
pub mod lp_store2;
#[doc = "LP_STORE3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store3`] module"]
pub type LP_STORE3 = crate::Reg<lp_store3::LP_STORE3_SPEC>;
#[doc = "need_des"]
pub mod lp_store3;
#[doc = "LP_STORE4 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store4`] module"]
pub type LP_STORE4 = crate::Reg<lp_store4::LP_STORE4_SPEC>;
#[doc = "need_des"]
pub mod lp_store4;
#[doc = "LP_STORE5 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store5`] module"]
pub type LP_STORE5 = crate::Reg<lp_store5::LP_STORE5_SPEC>;
#[doc = "need_des"]
pub mod lp_store5;
#[doc = "LP_STORE6 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store6`] module"]
pub type LP_STORE6 = crate::Reg<lp_store6::LP_STORE6_SPEC>;
#[doc = "need_des"]
pub mod lp_store6;
#[doc = "LP_STORE7 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store7`] module"]
pub type LP_STORE7 = crate::Reg<lp_store7::LP_STORE7_SPEC>;
#[doc = "need_des"]
pub mod lp_store7;
#[doc = "LP_STORE8 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store8`] module"]
pub type LP_STORE8 = crate::Reg<lp_store8::LP_STORE8_SPEC>;
#[doc = "need_des"]
pub mod lp_store8;
#[doc = "LP_STORE9 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store9`] module"]
pub type LP_STORE9 = crate::Reg<lp_store9::LP_STORE9_SPEC>;
#[doc = "need_des"]
pub mod lp_store9;
#[doc = "LP_STORE10 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store10`] module"]
pub type LP_STORE10 = crate::Reg<lp_store10::LP_STORE10_SPEC>;
#[doc = "need_des"]
pub mod lp_store10;
#[doc = "LP_STORE11 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store11`] module"]
pub type LP_STORE11 = crate::Reg<lp_store11::LP_STORE11_SPEC>;
#[doc = "need_des"]
pub mod lp_store11;
#[doc = "LP_STORE12 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store12`] module"]
pub type LP_STORE12 = crate::Reg<lp_store12::LP_STORE12_SPEC>;
#[doc = "need_des"]
pub mod lp_store12;
#[doc = "LP_STORE13 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store13`] module"]
pub type LP_STORE13 = crate::Reg<lp_store13::LP_STORE13_SPEC>;
#[doc = "need_des"]
pub mod lp_store13;
#[doc = "LP_STORE14 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store14`] module"]
pub type LP_STORE14 = crate::Reg<lp_store14::LP_STORE14_SPEC>;
#[doc = "need_des"]
pub mod lp_store14;
#[doc = "LP_STORE15 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_store15`] module"]
pub type LP_STORE15 = crate::Reg<lp_store15::LP_STORE15_SPEC>;
#[doc = "need_des"]
pub mod lp_store15;
#[doc = "LP_PROBEA_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_probea_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_probea_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probea_ctrl`] module"]
pub type LP_PROBEA_CTRL = crate::Reg<lp_probea_ctrl::LP_PROBEA_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_probea_ctrl;
#[doc = "LP_PROBEB_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_probeb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_probeb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probeb_ctrl`] module"]
pub type LP_PROBEB_CTRL = crate::Reg<lp_probeb_ctrl::LP_PROBEB_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_probeb_ctrl;
#[doc = "LP_PROBE_OUT (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_probe_out::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_probe_out`] module"]
pub type LP_PROBE_OUT = crate::Reg<lp_probe_out::LP_PROBE_OUT_SPEC>;
#[doc = "need_des"]
pub mod lp_probe_out;
#[doc = "F2S_APB_BRG_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2s_apb_brg_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2s_apb_brg_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2s_apb_brg_cntl`] module"]
pub type F2S_APB_BRG_CNTL = crate::Reg<f2s_apb_brg_cntl::F2S_APB_BRG_CNTL_SPEC>;
#[doc = "need_des"]
pub mod f2s_apb_brg_cntl;
#[doc = "USB_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`] module"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "need_des"]
pub mod usb_ctrl;
#[doc = "ANA_XPD_PAD_GROUP (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_xpd_pad_group::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_xpd_pad_group::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_xpd_pad_group`] module"]
pub type ANA_XPD_PAD_GROUP = crate::Reg<ana_xpd_pad_group::ANA_XPD_PAD_GROUP_SPEC>;
#[doc = "need_des"]
pub mod ana_xpd_pad_group;
#[doc = "LP_TCM_RAM_RDN_ECO_CS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_cs`] module"]
pub type LP_TCM_RAM_RDN_ECO_CS = crate::Reg<lp_tcm_ram_rdn_eco_cs::LP_TCM_RAM_RDN_ECO_CS_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_cs;
#[doc = "LP_TCM_RAM_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_low`] module"]
pub type LP_TCM_RAM_RDN_ECO_LOW = crate::Reg<lp_tcm_ram_rdn_eco_low::LP_TCM_RAM_RDN_ECO_LOW_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_low;
#[doc = "LP_TCM_RAM_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_ram_rdn_eco_high`] module"]
pub type LP_TCM_RAM_RDN_ECO_HIGH =
    crate::Reg<lp_tcm_ram_rdn_eco_high::LP_TCM_RAM_RDN_ECO_HIGH_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_ram_rdn_eco_high;
#[doc = "LP_TCM_ROM_RDN_ECO_CS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_cs`] module"]
pub type LP_TCM_ROM_RDN_ECO_CS = crate::Reg<lp_tcm_rom_rdn_eco_cs::LP_TCM_ROM_RDN_ECO_CS_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_cs;
#[doc = "LP_TCM_ROM_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_low`] module"]
pub type LP_TCM_ROM_RDN_ECO_LOW = crate::Reg<lp_tcm_rom_rdn_eco_low::LP_TCM_ROM_RDN_ECO_LOW_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_low;
#[doc = "LP_TCM_ROM_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_rom_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_rom_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_tcm_rom_rdn_eco_high`] module"]
pub type LP_TCM_ROM_RDN_ECO_HIGH =
    crate::Reg<lp_tcm_rom_rdn_eco_high::LP_TCM_ROM_RDN_ECO_HIGH_SPEC>;
#[doc = "need_des"]
pub mod lp_tcm_rom_rdn_eco_high;
#[doc = "HP_ROOT_CLK_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_root_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_root_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_root_clk_ctrl`] module"]
pub type HP_ROOT_CLK_CTRL = crate::Reg<hp_root_clk_ctrl::HP_ROOT_CLK_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_root_clk_ctrl;
#[doc = "LP_PMU_RDN_ECO_LOW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_pmu_rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_low`] module"]
pub type LP_PMU_RDN_ECO_LOW = crate::Reg<lp_pmu_rdn_eco_low::LP_PMU_RDN_ECO_LOW_SPEC>;
#[doc = "need_des"]
pub mod lp_pmu_rdn_eco_low;
#[doc = "LP_PMU_RDN_ECO_HIGH (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_pmu_rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pmu_rdn_eco_high`] module"]
pub type LP_PMU_RDN_ECO_HIGH = crate::Reg<lp_pmu_rdn_eco_high::LP_PMU_RDN_ECO_HIGH_SPEC>;
#[doc = "need_des"]
pub mod lp_pmu_rdn_eco_high;
#[doc = "PAD_COMP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp0`] module"]
pub type PAD_COMP0 = crate::Reg<pad_comp0::PAD_COMP0_SPEC>;
#[doc = "need_des"]
pub mod pad_comp0;
#[doc = "PAD_COMP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_comp1`] module"]
pub type PAD_COMP1 = crate::Reg<pad_comp1::PAD_COMP1_SPEC>;
#[doc = "need_des"]
pub mod pad_comp1;
#[doc = "BACKUP_DMA_CFG0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg0`] module"]
pub type BACKUP_DMA_CFG0 = crate::Reg<backup_dma_cfg0::BACKUP_DMA_CFG0_SPEC>;
#[doc = "need_des"]
pub mod backup_dma_cfg0;
#[doc = "BACKUP_DMA_CFG1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg1`] module"]
pub type BACKUP_DMA_CFG1 = crate::Reg<backup_dma_cfg1::BACKUP_DMA_CFG1_SPEC>;
#[doc = "need_des"]
pub mod backup_dma_cfg1;
#[doc = "BACKUP_DMA_CFG2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_dma_cfg2`] module"]
pub type BACKUP_DMA_CFG2 = crate::Reg<backup_dma_cfg2::BACKUP_DMA_CFG2_SPEC>;
#[doc = "need_des"]
pub mod backup_dma_cfg2;
#[doc = "BOOT_ADDR_HP_CORE1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_addr_hp_core1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_addr_hp_core1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_addr_hp_core1`] module"]
pub type BOOT_ADDR_HP_CORE1 = crate::Reg<boot_addr_hp_core1::BOOT_ADDR_HP_CORE1_SPEC>;
#[doc = "need_des"]
pub mod boot_addr_hp_core1;
#[doc = "LP_ADDRHOLE_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_addrhole_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_addrhole_addr`] module"]
pub type LP_ADDRHOLE_ADDR = crate::Reg<lp_addrhole_addr::LP_ADDRHOLE_ADDR_SPEC>;
#[doc = "need_des"]
pub mod lp_addrhole_addr;
#[doc = "LP_ADDRHOLE_INFO (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_addrhole_info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_addrhole_info`] module"]
pub type LP_ADDRHOLE_INFO = crate::Reg<lp_addrhole_info::LP_ADDRHOLE_INFO_SPEC>;
#[doc = "need_des"]
pub mod lp_addrhole_info;
#[doc = "INT_RAW (r) register accessor: raw interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "raw interrupt register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "masked interrupt register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "masked interrupt register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "interrupt clear register"]
pub mod int_clr;
#[doc = "HP_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_mem_aux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_mem_aux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_mem_aux_ctrl`] module"]
pub type HP_MEM_AUX_CTRL = crate::Reg<hp_mem_aux_ctrl::HP_MEM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_mem_aux_ctrl;
#[doc = "LP_MEM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_mem_aux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_mem_aux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mem_aux_ctrl`] module"]
pub type LP_MEM_AUX_CTRL = crate::Reg<lp_mem_aux_ctrl::LP_MEM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_mem_aux_ctrl;
#[doc = "HP_ROM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_rom_aux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_rom_aux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rom_aux_ctrl`] module"]
pub type HP_ROM_AUX_CTRL = crate::Reg<hp_rom_aux_ctrl::HP_ROM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_rom_aux_ctrl;
#[doc = "LP_ROM_AUX_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rom_aux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rom_aux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_rom_aux_ctrl`] module"]
pub type LP_ROM_AUX_CTRL = crate::Reg<lp_rom_aux_ctrl::LP_ROM_AUX_CTRL_SPEC>;
#[doc = "need_des"]
pub mod lp_rom_aux_ctrl;
#[doc = "LP_CPU_DBG_PC (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_dbg_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_dbg_pc`] module"]
pub type LP_CPU_DBG_PC = crate::Reg<lp_cpu_dbg_pc::LP_CPU_DBG_PC_SPEC>;
#[doc = "need_des"]
pub mod lp_cpu_dbg_pc;
#[doc = "LP_CPU_EXC_PC (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_exc_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_exc_pc`] module"]
pub type LP_CPU_EXC_PC = crate::Reg<lp_cpu_exc_pc::LP_CPU_EXC_PC_SPEC>;
#[doc = "need_des"]
pub mod lp_cpu_exc_pc;
#[doc = "IDBUS_ADDRHOLE_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idbus_addrhole_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idbus_addrhole_addr`] module"]
pub type IDBUS_ADDRHOLE_ADDR = crate::Reg<idbus_addrhole_addr::IDBUS_ADDRHOLE_ADDR_SPEC>;
#[doc = "need_des"]
pub mod idbus_addrhole_addr;
#[doc = "IDBUS_ADDRHOLE_INFO (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idbus_addrhole_info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idbus_addrhole_info`] module"]
pub type IDBUS_ADDRHOLE_INFO = crate::Reg<idbus_addrhole_info::IDBUS_ADDRHOLE_INFO_SPEC>;
#[doc = "need_des"]
pub mod idbus_addrhole_info;
#[doc = "HP_POR_RST_BYPASS_CTRL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_por_rst_bypass_ctrl`] module"]
pub type HP_POR_RST_BYPASS_CTRL = crate::Reg<hp_por_rst_bypass_ctrl::HP_POR_RST_BYPASS_CTRL_SPEC>;
#[doc = "need_des"]
pub mod hp_por_rst_bypass_ctrl;
#[doc = "RNG_DATA (r) register accessor: rng data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data`] module"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "rng data register"]
pub mod rng_data;
#[doc = "LP_CORE_AHB_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_ahb_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_ahb_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_ahb_timeout`] module"]
pub type LP_CORE_AHB_TIMEOUT = crate::Reg<lp_core_ahb_timeout::LP_CORE_AHB_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod lp_core_ahb_timeout;
#[doc = "LP_CORE_IBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_ibus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_ibus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_ibus_timeout`] module"]
pub type LP_CORE_IBUS_TIMEOUT = crate::Reg<lp_core_ibus_timeout::LP_CORE_IBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod lp_core_ibus_timeout;
#[doc = "LP_CORE_DBUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_dbus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_dbus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_dbus_timeout`] module"]
pub type LP_CORE_DBUS_TIMEOUT = crate::Reg<lp_core_dbus_timeout::LP_CORE_DBUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod lp_core_dbus_timeout;
#[doc = "LP_CORE_ERR_RESP_DIS (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_err_resp_dis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_err_resp_dis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_core_err_resp_dis`] module"]
pub type LP_CORE_ERR_RESP_DIS = crate::Reg<lp_core_err_resp_dis::LP_CORE_ERR_RESP_DIS_SPEC>;
#[doc = "need_des"]
pub mod lp_core_err_resp_dis;
#[doc = "RNG_CFG (rw) register accessor: rng cfg register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
pub type RNG_CFG = crate::Reg<rng_cfg::RNG_CFG_SPEC>;
#[doc = "rng cfg register"]
pub mod rng_cfg;
