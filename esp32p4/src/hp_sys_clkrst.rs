#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en0: CLK_EN0,
    root_clk_ctrl0: ROOT_CLK_CTRL0,
    root_clk_ctrl1: ROOT_CLK_CTRL1,
    root_clk_ctrl2: ROOT_CLK_CTRL2,
    root_clk_ctrl3: ROOT_CLK_CTRL3,
    soc_clk_ctrl0: SOC_CLK_CTRL0,
    soc_clk_ctrl1: SOC_CLK_CTRL1,
    soc_clk_ctrl2: SOC_CLK_CTRL2,
    soc_clk_ctrl3: SOC_CLK_CTRL3,
    ref_clk_ctrl0: REF_CLK_CTRL0,
    ref_clk_ctrl1: REF_CLK_CTRL1,
    ref_clk_ctrl2: REF_CLK_CTRL2,
    peri_clk_ctrl00: PERI_CLK_CTRL00,
    peri_clk_ctrl01: PERI_CLK_CTRL01,
    peri_clk_ctrl02: PERI_CLK_CTRL02,
    peri_clk_ctrl03: PERI_CLK_CTRL03,
    peri_clk_ctrl10: PERI_CLK_CTRL10,
    peri_clk_ctrl11: PERI_CLK_CTRL11,
    peri_clk_ctrl12: PERI_CLK_CTRL12,
    peri_clk_ctrl13: PERI_CLK_CTRL13,
    peri_clk_ctrl14: PERI_CLK_CTRL14,
    peri_clk_ctrl15: PERI_CLK_CTRL15,
    peri_clk_ctrl16: PERI_CLK_CTRL16,
    peri_clk_ctrl17: PERI_CLK_CTRL17,
    peri_clk_ctrl18: PERI_CLK_CTRL18,
    peri_clk_ctrl19: PERI_CLK_CTRL19,
    peri_clk_ctrl110: PERI_CLK_CTRL110,
    peri_clk_ctrl111: PERI_CLK_CTRL111,
    peri_clk_ctrl112: PERI_CLK_CTRL112,
    peri_clk_ctrl113: PERI_CLK_CTRL113,
    peri_clk_ctrl114: PERI_CLK_CTRL114,
    peri_clk_ctrl115: PERI_CLK_CTRL115,
    peri_clk_ctrl116: PERI_CLK_CTRL116,
    peri_clk_ctrl117: PERI_CLK_CTRL117,
    peri_clk_ctrl118: PERI_CLK_CTRL118,
    peri_clk_ctrl119: PERI_CLK_CTRL119,
    peri_clk_ctrl120: PERI_CLK_CTRL120,
    peri_clk_ctrl20: PERI_CLK_CTRL20,
    peri_clk_ctrl21: PERI_CLK_CTRL21,
    peri_clk_ctrl22: PERI_CLK_CTRL22,
    peri_clk_ctrl23: PERI_CLK_CTRL23,
    peri_clk_ctrl24: PERI_CLK_CTRL24,
    peri_clk_ctrl25: PERI_CLK_CTRL25,
    peri_clk_ctrl26: PERI_CLK_CTRL26,
    peri_clk_ctrl27: PERI_CLK_CTRL27,
    clk_force_on_ctrl0: CLK_FORCE_ON_CTRL0,
    dpa_ctrl0: DPA_CTRL0,
    ana_pll_ctrl0: ANA_PLL_CTRL0,
    hp_rst_en0: HP_RST_EN0,
    hp_rst_en1: HP_RST_EN1,
    hp_rst_en2: HP_RST_EN2,
    hp_force_norst0: HP_FORCE_NORST0,
    hp_force_norst1: HP_FORCE_NORST1,
    hpwdt_core0_rst_ctrl0: HPWDT_CORE0_RST_CTRL0,
    hpwdt_core1_rst_ctrl0: HPWDT_CORE1_RST_CTRL0,
    cpu_src_freq0: CPU_SRC_FREQ0,
    cpu_clk_status0: CPU_CLK_STATUS0,
    dbg_clk_ctrl0: DBG_CLK_CTRL0,
    dbg_clk_ctrl1: DBG_CLK_CTRL1,
    hpcore_wdt_reset_source0: HPCORE_WDT_RESET_SOURCE0,
}
impl RegisterBlock {
    #[doc = "0x00 - Reserved"]
    #[inline(always)]
    pub const fn clk_en0(&self) -> &CLK_EN0 {
        &self.clk_en0
    }
    #[doc = "0x04 - Reserved"]
    #[inline(always)]
    pub const fn root_clk_ctrl0(&self) -> &ROOT_CLK_CTRL0 {
        &self.root_clk_ctrl0
    }
    #[doc = "0x08 - Reserved"]
    #[inline(always)]
    pub const fn root_clk_ctrl1(&self) -> &ROOT_CLK_CTRL1 {
        &self.root_clk_ctrl1
    }
    #[doc = "0x0c - Reserved"]
    #[inline(always)]
    pub const fn root_clk_ctrl2(&self) -> &ROOT_CLK_CTRL2 {
        &self.root_clk_ctrl2
    }
    #[doc = "0x10 - Reserved"]
    #[inline(always)]
    pub const fn root_clk_ctrl3(&self) -> &ROOT_CLK_CTRL3 {
        &self.root_clk_ctrl3
    }
    #[doc = "0x14 - Reserved"]
    #[inline(always)]
    pub const fn soc_clk_ctrl0(&self) -> &SOC_CLK_CTRL0 {
        &self.soc_clk_ctrl0
    }
    #[doc = "0x18 - Reserved"]
    #[inline(always)]
    pub const fn soc_clk_ctrl1(&self) -> &SOC_CLK_CTRL1 {
        &self.soc_clk_ctrl1
    }
    #[doc = "0x1c - Reserved"]
    #[inline(always)]
    pub const fn soc_clk_ctrl2(&self) -> &SOC_CLK_CTRL2 {
        &self.soc_clk_ctrl2
    }
    #[doc = "0x20 - Reserved"]
    #[inline(always)]
    pub const fn soc_clk_ctrl3(&self) -> &SOC_CLK_CTRL3 {
        &self.soc_clk_ctrl3
    }
    #[doc = "0x24 - Reserved"]
    #[inline(always)]
    pub const fn ref_clk_ctrl0(&self) -> &REF_CLK_CTRL0 {
        &self.ref_clk_ctrl0
    }
    #[doc = "0x28 - Reserved"]
    #[inline(always)]
    pub const fn ref_clk_ctrl1(&self) -> &REF_CLK_CTRL1 {
        &self.ref_clk_ctrl1
    }
    #[doc = "0x2c - Reserved"]
    #[inline(always)]
    pub const fn ref_clk_ctrl2(&self) -> &REF_CLK_CTRL2 {
        &self.ref_clk_ctrl2
    }
    #[doc = "0x30 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl00(&self) -> &PERI_CLK_CTRL00 {
        &self.peri_clk_ctrl00
    }
    #[doc = "0x34 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl01(&self) -> &PERI_CLK_CTRL01 {
        &self.peri_clk_ctrl01
    }
    #[doc = "0x38 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl02(&self) -> &PERI_CLK_CTRL02 {
        &self.peri_clk_ctrl02
    }
    #[doc = "0x3c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl03(&self) -> &PERI_CLK_CTRL03 {
        &self.peri_clk_ctrl03
    }
    #[doc = "0x40 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl10(&self) -> &PERI_CLK_CTRL10 {
        &self.peri_clk_ctrl10
    }
    #[doc = "0x44 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl11(&self) -> &PERI_CLK_CTRL11 {
        &self.peri_clk_ctrl11
    }
    #[doc = "0x48 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl12(&self) -> &PERI_CLK_CTRL12 {
        &self.peri_clk_ctrl12
    }
    #[doc = "0x4c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl13(&self) -> &PERI_CLK_CTRL13 {
        &self.peri_clk_ctrl13
    }
    #[doc = "0x50 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl14(&self) -> &PERI_CLK_CTRL14 {
        &self.peri_clk_ctrl14
    }
    #[doc = "0x54 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl15(&self) -> &PERI_CLK_CTRL15 {
        &self.peri_clk_ctrl15
    }
    #[doc = "0x58 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl16(&self) -> &PERI_CLK_CTRL16 {
        &self.peri_clk_ctrl16
    }
    #[doc = "0x5c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl17(&self) -> &PERI_CLK_CTRL17 {
        &self.peri_clk_ctrl17
    }
    #[doc = "0x60 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl18(&self) -> &PERI_CLK_CTRL18 {
        &self.peri_clk_ctrl18
    }
    #[doc = "0x64 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl19(&self) -> &PERI_CLK_CTRL19 {
        &self.peri_clk_ctrl19
    }
    #[doc = "0x68 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl110(&self) -> &PERI_CLK_CTRL110 {
        &self.peri_clk_ctrl110
    }
    #[doc = "0x6c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl111(&self) -> &PERI_CLK_CTRL111 {
        &self.peri_clk_ctrl111
    }
    #[doc = "0x70 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl112(&self) -> &PERI_CLK_CTRL112 {
        &self.peri_clk_ctrl112
    }
    #[doc = "0x74 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl113(&self) -> &PERI_CLK_CTRL113 {
        &self.peri_clk_ctrl113
    }
    #[doc = "0x78 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl114(&self) -> &PERI_CLK_CTRL114 {
        &self.peri_clk_ctrl114
    }
    #[doc = "0x7c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl115(&self) -> &PERI_CLK_CTRL115 {
        &self.peri_clk_ctrl115
    }
    #[doc = "0x80 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl116(&self) -> &PERI_CLK_CTRL116 {
        &self.peri_clk_ctrl116
    }
    #[doc = "0x84 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl117(&self) -> &PERI_CLK_CTRL117 {
        &self.peri_clk_ctrl117
    }
    #[doc = "0x88 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl118(&self) -> &PERI_CLK_CTRL118 {
        &self.peri_clk_ctrl118
    }
    #[doc = "0x8c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl119(&self) -> &PERI_CLK_CTRL119 {
        &self.peri_clk_ctrl119
    }
    #[doc = "0x90 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl120(&self) -> &PERI_CLK_CTRL120 {
        &self.peri_clk_ctrl120
    }
    #[doc = "0x94 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl20(&self) -> &PERI_CLK_CTRL20 {
        &self.peri_clk_ctrl20
    }
    #[doc = "0x98 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl21(&self) -> &PERI_CLK_CTRL21 {
        &self.peri_clk_ctrl21
    }
    #[doc = "0x9c - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl22(&self) -> &PERI_CLK_CTRL22 {
        &self.peri_clk_ctrl22
    }
    #[doc = "0xa0 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl23(&self) -> &PERI_CLK_CTRL23 {
        &self.peri_clk_ctrl23
    }
    #[doc = "0xa4 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl24(&self) -> &PERI_CLK_CTRL24 {
        &self.peri_clk_ctrl24
    }
    #[doc = "0xa8 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl25(&self) -> &PERI_CLK_CTRL25 {
        &self.peri_clk_ctrl25
    }
    #[doc = "0xac - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl26(&self) -> &PERI_CLK_CTRL26 {
        &self.peri_clk_ctrl26
    }
    #[doc = "0xb0 - Reserved"]
    #[inline(always)]
    pub const fn peri_clk_ctrl27(&self) -> &PERI_CLK_CTRL27 {
        &self.peri_clk_ctrl27
    }
    #[doc = "0xb4 - Reserved"]
    #[inline(always)]
    pub const fn clk_force_on_ctrl0(&self) -> &CLK_FORCE_ON_CTRL0 {
        &self.clk_force_on_ctrl0
    }
    #[doc = "0xb8 - Reserved"]
    #[inline(always)]
    pub const fn dpa_ctrl0(&self) -> &DPA_CTRL0 {
        &self.dpa_ctrl0
    }
    #[doc = "0xbc - Reserved"]
    #[inline(always)]
    pub const fn ana_pll_ctrl0(&self) -> &ANA_PLL_CTRL0 {
        &self.ana_pll_ctrl0
    }
    #[doc = "0xc0 - Reserved"]
    #[inline(always)]
    pub const fn hp_rst_en0(&self) -> &HP_RST_EN0 {
        &self.hp_rst_en0
    }
    #[doc = "0xc4 - Reserved"]
    #[inline(always)]
    pub const fn hp_rst_en1(&self) -> &HP_RST_EN1 {
        &self.hp_rst_en1
    }
    #[doc = "0xc8 - Reserved"]
    #[inline(always)]
    pub const fn hp_rst_en2(&self) -> &HP_RST_EN2 {
        &self.hp_rst_en2
    }
    #[doc = "0xcc - Reserved"]
    #[inline(always)]
    pub const fn hp_force_norst0(&self) -> &HP_FORCE_NORST0 {
        &self.hp_force_norst0
    }
    #[doc = "0xd0 - Reserved"]
    #[inline(always)]
    pub const fn hp_force_norst1(&self) -> &HP_FORCE_NORST1 {
        &self.hp_force_norst1
    }
    #[doc = "0xd4 - Reserved"]
    #[inline(always)]
    pub const fn hpwdt_core0_rst_ctrl0(&self) -> &HPWDT_CORE0_RST_CTRL0 {
        &self.hpwdt_core0_rst_ctrl0
    }
    #[doc = "0xd8 - Reserved"]
    #[inline(always)]
    pub const fn hpwdt_core1_rst_ctrl0(&self) -> &HPWDT_CORE1_RST_CTRL0 {
        &self.hpwdt_core1_rst_ctrl0
    }
    #[doc = "0xdc - CPU Source Frequency"]
    #[inline(always)]
    pub const fn cpu_src_freq0(&self) -> &CPU_SRC_FREQ0 {
        &self.cpu_src_freq0
    }
    #[doc = "0xe0 - CPU Clock Status"]
    #[inline(always)]
    pub const fn cpu_clk_status0(&self) -> &CPU_CLK_STATUS0 {
        &self.cpu_clk_status0
    }
    #[doc = "0xe4 - Reserved"]
    #[inline(always)]
    pub const fn dbg_clk_ctrl0(&self) -> &DBG_CLK_CTRL0 {
        &self.dbg_clk_ctrl0
    }
    #[doc = "0xe8 - Reserved"]
    #[inline(always)]
    pub const fn dbg_clk_ctrl1(&self) -> &DBG_CLK_CTRL1 {
        &self.dbg_clk_ctrl1
    }
    #[doc = "0xec - Reserved"]
    #[inline(always)]
    pub const fn hpcore_wdt_reset_source0(&self) -> &HPCORE_WDT_RESET_SOURCE0 {
        &self.hpcore_wdt_reset_source0
    }
}
#[doc = "CLK_EN0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en0`] module"]
pub type CLK_EN0 = crate::Reg<clk_en0::CLK_EN0_SPEC>;
#[doc = "Reserved"]
pub mod clk_en0;
#[doc = "ROOT_CLK_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_ctrl0`] module"]
pub type ROOT_CLK_CTRL0 = crate::Reg<root_clk_ctrl0::ROOT_CLK_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod root_clk_ctrl0;
#[doc = "ROOT_CLK_CTRL1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_ctrl1`] module"]
pub type ROOT_CLK_CTRL1 = crate::Reg<root_clk_ctrl1::ROOT_CLK_CTRL1_SPEC>;
#[doc = "Reserved"]
pub mod root_clk_ctrl1;
#[doc = "ROOT_CLK_CTRL2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_ctrl2`] module"]
pub type ROOT_CLK_CTRL2 = crate::Reg<root_clk_ctrl2::ROOT_CLK_CTRL2_SPEC>;
#[doc = "Reserved"]
pub mod root_clk_ctrl2;
#[doc = "ROOT_CLK_CTRL3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root_clk_ctrl3`] module"]
pub type ROOT_CLK_CTRL3 = crate::Reg<root_clk_ctrl3::ROOT_CLK_CTRL3_SPEC>;
#[doc = "Reserved"]
pub mod root_clk_ctrl3;
#[doc = "SOC_CLK_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_clk_ctrl0`] module"]
pub type SOC_CLK_CTRL0 = crate::Reg<soc_clk_ctrl0::SOC_CLK_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod soc_clk_ctrl0;
#[doc = "SOC_CLK_CTRL1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_clk_ctrl1`] module"]
pub type SOC_CLK_CTRL1 = crate::Reg<soc_clk_ctrl1::SOC_CLK_CTRL1_SPEC>;
#[doc = "Reserved"]
pub mod soc_clk_ctrl1;
#[doc = "SOC_CLK_CTRL2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_clk_ctrl2`] module"]
pub type SOC_CLK_CTRL2 = crate::Reg<soc_clk_ctrl2::SOC_CLK_CTRL2_SPEC>;
#[doc = "Reserved"]
pub mod soc_clk_ctrl2;
#[doc = "SOC_CLK_CTRL3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_clk_ctrl3`] module"]
pub type SOC_CLK_CTRL3 = crate::Reg<soc_clk_ctrl3::SOC_CLK_CTRL3_SPEC>;
#[doc = "Reserved"]
pub mod soc_clk_ctrl3;
#[doc = "REF_CLK_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_clk_ctrl0`] module"]
pub type REF_CLK_CTRL0 = crate::Reg<ref_clk_ctrl0::REF_CLK_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod ref_clk_ctrl0;
#[doc = "REF_CLK_CTRL1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_clk_ctrl1`] module"]
pub type REF_CLK_CTRL1 = crate::Reg<ref_clk_ctrl1::REF_CLK_CTRL1_SPEC>;
#[doc = "Reserved"]
pub mod ref_clk_ctrl1;
#[doc = "REF_CLK_CTRL2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clk_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clk_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_clk_ctrl2`] module"]
pub type REF_CLK_CTRL2 = crate::Reg<ref_clk_ctrl2::REF_CLK_CTRL2_SPEC>;
#[doc = "Reserved"]
pub mod ref_clk_ctrl2;
#[doc = "PERI_CLK_CTRL00 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl00`] module"]
pub type PERI_CLK_CTRL00 = crate::Reg<peri_clk_ctrl00::PERI_CLK_CTRL00_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl00;
#[doc = "PERI_CLK_CTRL01 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl01`] module"]
pub type PERI_CLK_CTRL01 = crate::Reg<peri_clk_ctrl01::PERI_CLK_CTRL01_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl01;
#[doc = "PERI_CLK_CTRL02 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl02`] module"]
pub type PERI_CLK_CTRL02 = crate::Reg<peri_clk_ctrl02::PERI_CLK_CTRL02_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl02;
#[doc = "PERI_CLK_CTRL03 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl03`] module"]
pub type PERI_CLK_CTRL03 = crate::Reg<peri_clk_ctrl03::PERI_CLK_CTRL03_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl03;
#[doc = "PERI_CLK_CTRL10 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl10`] module"]
pub type PERI_CLK_CTRL10 = crate::Reg<peri_clk_ctrl10::PERI_CLK_CTRL10_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl10;
#[doc = "PERI_CLK_CTRL11 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl11`] module"]
pub type PERI_CLK_CTRL11 = crate::Reg<peri_clk_ctrl11::PERI_CLK_CTRL11_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl11;
#[doc = "PERI_CLK_CTRL12 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl12`] module"]
pub type PERI_CLK_CTRL12 = crate::Reg<peri_clk_ctrl12::PERI_CLK_CTRL12_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl12;
#[doc = "PERI_CLK_CTRL13 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl13`] module"]
pub type PERI_CLK_CTRL13 = crate::Reg<peri_clk_ctrl13::PERI_CLK_CTRL13_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl13;
#[doc = "PERI_CLK_CTRL14 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl14`] module"]
pub type PERI_CLK_CTRL14 = crate::Reg<peri_clk_ctrl14::PERI_CLK_CTRL14_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl14;
#[doc = "PERI_CLK_CTRL15 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl15`] module"]
pub type PERI_CLK_CTRL15 = crate::Reg<peri_clk_ctrl15::PERI_CLK_CTRL15_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl15;
#[doc = "PERI_CLK_CTRL16 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl16`] module"]
pub type PERI_CLK_CTRL16 = crate::Reg<peri_clk_ctrl16::PERI_CLK_CTRL16_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl16;
#[doc = "PERI_CLK_CTRL17 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl17`] module"]
pub type PERI_CLK_CTRL17 = crate::Reg<peri_clk_ctrl17::PERI_CLK_CTRL17_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl17;
#[doc = "PERI_CLK_CTRL18 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl18`] module"]
pub type PERI_CLK_CTRL18 = crate::Reg<peri_clk_ctrl18::PERI_CLK_CTRL18_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl18;
#[doc = "PERI_CLK_CTRL19 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl19`] module"]
pub type PERI_CLK_CTRL19 = crate::Reg<peri_clk_ctrl19::PERI_CLK_CTRL19_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl19;
#[doc = "PERI_CLK_CTRL110 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl110`] module"]
pub type PERI_CLK_CTRL110 = crate::Reg<peri_clk_ctrl110::PERI_CLK_CTRL110_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl110;
#[doc = "PERI_CLK_CTRL111 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl111`] module"]
pub type PERI_CLK_CTRL111 = crate::Reg<peri_clk_ctrl111::PERI_CLK_CTRL111_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl111;
#[doc = "PERI_CLK_CTRL112 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl112`] module"]
pub type PERI_CLK_CTRL112 = crate::Reg<peri_clk_ctrl112::PERI_CLK_CTRL112_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl112;
#[doc = "PERI_CLK_CTRL113 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl113`] module"]
pub type PERI_CLK_CTRL113 = crate::Reg<peri_clk_ctrl113::PERI_CLK_CTRL113_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl113;
#[doc = "PERI_CLK_CTRL114 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl114`] module"]
pub type PERI_CLK_CTRL114 = crate::Reg<peri_clk_ctrl114::PERI_CLK_CTRL114_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl114;
#[doc = "PERI_CLK_CTRL115 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl115`] module"]
pub type PERI_CLK_CTRL115 = crate::Reg<peri_clk_ctrl115::PERI_CLK_CTRL115_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl115;
#[doc = "PERI_CLK_CTRL116 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl116`] module"]
pub type PERI_CLK_CTRL116 = crate::Reg<peri_clk_ctrl116::PERI_CLK_CTRL116_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl116;
#[doc = "PERI_CLK_CTRL117 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl117`] module"]
pub type PERI_CLK_CTRL117 = crate::Reg<peri_clk_ctrl117::PERI_CLK_CTRL117_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl117;
#[doc = "PERI_CLK_CTRL118 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl118`] module"]
pub type PERI_CLK_CTRL118 = crate::Reg<peri_clk_ctrl118::PERI_CLK_CTRL118_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl118;
#[doc = "PERI_CLK_CTRL119 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl119`] module"]
pub type PERI_CLK_CTRL119 = crate::Reg<peri_clk_ctrl119::PERI_CLK_CTRL119_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl119;
#[doc = "PERI_CLK_CTRL120 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl120`] module"]
pub type PERI_CLK_CTRL120 = crate::Reg<peri_clk_ctrl120::PERI_CLK_CTRL120_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl120;
#[doc = "PERI_CLK_CTRL20 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl20`] module"]
pub type PERI_CLK_CTRL20 = crate::Reg<peri_clk_ctrl20::PERI_CLK_CTRL20_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl20;
#[doc = "PERI_CLK_CTRL21 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl21`] module"]
pub type PERI_CLK_CTRL21 = crate::Reg<peri_clk_ctrl21::PERI_CLK_CTRL21_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl21;
#[doc = "PERI_CLK_CTRL22 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl22`] module"]
pub type PERI_CLK_CTRL22 = crate::Reg<peri_clk_ctrl22::PERI_CLK_CTRL22_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl22;
#[doc = "PERI_CLK_CTRL23 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl23`] module"]
pub type PERI_CLK_CTRL23 = crate::Reg<peri_clk_ctrl23::PERI_CLK_CTRL23_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl23;
#[doc = "PERI_CLK_CTRL24 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl24`] module"]
pub type PERI_CLK_CTRL24 = crate::Reg<peri_clk_ctrl24::PERI_CLK_CTRL24_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl24;
#[doc = "PERI_CLK_CTRL25 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl25`] module"]
pub type PERI_CLK_CTRL25 = crate::Reg<peri_clk_ctrl25::PERI_CLK_CTRL25_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl25;
#[doc = "PERI_CLK_CTRL26 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl26`] module"]
pub type PERI_CLK_CTRL26 = crate::Reg<peri_clk_ctrl26::PERI_CLK_CTRL26_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl26;
#[doc = "PERI_CLK_CTRL27 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_clk_ctrl27`] module"]
pub type PERI_CLK_CTRL27 = crate::Reg<peri_clk_ctrl27::PERI_CLK_CTRL27_SPEC>;
#[doc = "Reserved"]
pub mod peri_clk_ctrl27;
#[doc = "CLK_FORCE_ON_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_force_on_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_force_on_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_force_on_ctrl0`] module"]
pub type CLK_FORCE_ON_CTRL0 = crate::Reg<clk_force_on_ctrl0::CLK_FORCE_ON_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod clk_force_on_ctrl0;
#[doc = "DPA_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_ctrl0`] module"]
pub type DPA_CTRL0 = crate::Reg<dpa_ctrl0::DPA_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod dpa_ctrl0;
#[doc = "ANA_PLL_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_pll_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_pll_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_pll_ctrl0`] module"]
pub type ANA_PLL_CTRL0 = crate::Reg<ana_pll_ctrl0::ANA_PLL_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod ana_pll_ctrl0;
#[doc = "HP_RST_EN0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rst_en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rst_en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rst_en0`] module"]
pub type HP_RST_EN0 = crate::Reg<hp_rst_en0::HP_RST_EN0_SPEC>;
#[doc = "Reserved"]
pub mod hp_rst_en0;
#[doc = "HP_RST_EN1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rst_en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rst_en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rst_en1`] module"]
pub type HP_RST_EN1 = crate::Reg<hp_rst_en1::HP_RST_EN1_SPEC>;
#[doc = "Reserved"]
pub mod hp_rst_en1;
#[doc = "HP_RST_EN2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rst_en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rst_en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_rst_en2`] module"]
pub type HP_RST_EN2 = crate::Reg<hp_rst_en2::HP_RST_EN2_SPEC>;
#[doc = "Reserved"]
pub mod hp_rst_en2;
#[doc = "HP_FORCE_NORST0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_force_norst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_force_norst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_force_norst0`] module"]
pub type HP_FORCE_NORST0 = crate::Reg<hp_force_norst0::HP_FORCE_NORST0_SPEC>;
#[doc = "Reserved"]
pub mod hp_force_norst0;
#[doc = "HP_FORCE_NORST1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_force_norst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_force_norst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_force_norst1`] module"]
pub type HP_FORCE_NORST1 = crate::Reg<hp_force_norst1::HP_FORCE_NORST1_SPEC>;
#[doc = "Reserved"]
pub mod hp_force_norst1;
#[doc = "HPWDT_CORE0_RST_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hpwdt_core0_rst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpwdt_core0_rst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpwdt_core0_rst_ctrl0`] module"]
pub type HPWDT_CORE0_RST_CTRL0 = crate::Reg<hpwdt_core0_rst_ctrl0::HPWDT_CORE0_RST_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod hpwdt_core0_rst_ctrl0;
#[doc = "HPWDT_CORE1_RST_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hpwdt_core1_rst_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpwdt_core1_rst_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpwdt_core1_rst_ctrl0`] module"]
pub type HPWDT_CORE1_RST_CTRL0 = crate::Reg<hpwdt_core1_rst_ctrl0::HPWDT_CORE1_RST_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod hpwdt_core1_rst_ctrl0;
#[doc = "CPU_SRC_FREQ0 (r) register accessor: CPU Source Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_src_freq0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_src_freq0`] module"]
pub type CPU_SRC_FREQ0 = crate::Reg<cpu_src_freq0::CPU_SRC_FREQ0_SPEC>;
#[doc = "CPU Source Frequency"]
pub mod cpu_src_freq0;
#[doc = "CPU_CLK_STATUS0 (r) register accessor: CPU Clock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_clk_status0`] module"]
pub type CPU_CLK_STATUS0 = crate::Reg<cpu_clk_status0::CPU_CLK_STATUS0_SPEC>;
#[doc = "CPU Clock Status"]
pub mod cpu_clk_status0;
#[doc = "DBG_CLK_CTRL0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_clk_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_clk_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_clk_ctrl0`] module"]
pub type DBG_CLK_CTRL0 = crate::Reg<dbg_clk_ctrl0::DBG_CLK_CTRL0_SPEC>;
#[doc = "Reserved"]
pub mod dbg_clk_ctrl0;
#[doc = "DBG_CLK_CTRL1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_clk_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_clk_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_clk_ctrl1`] module"]
pub type DBG_CLK_CTRL1 = crate::Reg<dbg_clk_ctrl1::DBG_CLK_CTRL1_SPEC>;
#[doc = "Reserved"]
pub mod dbg_clk_ctrl1;
#[doc = "HPCORE_WDT_RESET_SOURCE0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_wdt_reset_source0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_wdt_reset_source0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpcore_wdt_reset_source0`] module"]
pub type HPCORE_WDT_RESET_SOURCE0 =
    crate::Reg<hpcore_wdt_reset_source0::HPCORE_WDT_RESET_SOURCE0_SPEC>;
#[doc = "Reserved"]
pub mod hpcore_wdt_reset_source0;
