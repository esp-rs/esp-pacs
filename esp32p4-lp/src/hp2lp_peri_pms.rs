#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hp2lp_tee_pms_date: HP2LP_TEE_PMS_DATE,
    pms_clk_en: PMS_CLK_EN,
    hp_core0_mm_pms_reg0: HP_CORE0_MM_PMS_REG0,
    hp_core0_um_pms_reg0: HP_CORE0_UM_PMS_REG0,
    hp_core1_mm_pms_reg0: HP_CORE1_MM_PMS_REG0,
    hp_core1_um_pms_reg0: HP_CORE1_UM_PMS_REG0,
    regdma_peri_pms: REGDMA_PERI_PMS,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn hp2lp_tee_pms_date(&self) -> &HP2LP_TEE_PMS_DATE {
        &self.hp2lp_tee_pms_date
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn pms_clk_en(&self) -> &PMS_CLK_EN {
        &self.pms_clk_en
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn hp_core0_mm_pms_reg0(&self) -> &HP_CORE0_MM_PMS_REG0 {
        &self.hp_core0_mm_pms_reg0
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn hp_core0_um_pms_reg0(&self) -> &HP_CORE0_UM_PMS_REG0 {
        &self.hp_core0_um_pms_reg0
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn hp_core1_mm_pms_reg0(&self) -> &HP_CORE1_MM_PMS_REG0 {
        &self.hp_core1_mm_pms_reg0
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn hp_core1_um_pms_reg0(&self) -> &HP_CORE1_UM_PMS_REG0 {
        &self.hp_core1_um_pms_reg0
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn regdma_peri_pms(&self) -> &REGDMA_PERI_PMS {
        &self.regdma_peri_pms
    }
}
#[doc = "HP2LP_TEE_PMS_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_tee_pms_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp2lp_tee_pms_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp2lp_tee_pms_date`] module"]
pub type HP2LP_TEE_PMS_DATE = crate::Reg<hp2lp_tee_pms_date::HP2LP_TEE_PMS_DATE_SPEC>;
#[doc = "NA"]
pub mod hp2lp_tee_pms_date;
#[doc = "PMS_CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_clk_en`] module"]
pub type PMS_CLK_EN = crate::Reg<pms_clk_en::PMS_CLK_EN_SPEC>;
#[doc = "NA"]
pub mod pms_clk_en;
#[doc = "HP_CORE0_MM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core0_mm_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core0_mm_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core0_mm_pms_reg0`] module"]
pub type HP_CORE0_MM_PMS_REG0 = crate::Reg<hp_core0_mm_pms_reg0::HP_CORE0_MM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod hp_core0_mm_pms_reg0;
#[doc = "HP_CORE0_UM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core0_um_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core0_um_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core0_um_pms_reg0`] module"]
pub type HP_CORE0_UM_PMS_REG0 = crate::Reg<hp_core0_um_pms_reg0::HP_CORE0_UM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod hp_core0_um_pms_reg0;
#[doc = "HP_CORE1_MM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core1_mm_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core1_mm_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core1_mm_pms_reg0`] module"]
pub type HP_CORE1_MM_PMS_REG0 = crate::Reg<hp_core1_mm_pms_reg0::HP_CORE1_MM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod hp_core1_mm_pms_reg0;
#[doc = "HP_CORE1_UM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core1_um_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core1_um_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_core1_um_pms_reg0`] module"]
pub type HP_CORE1_UM_PMS_REG0 = crate::Reg<hp_core1_um_pms_reg0::HP_CORE1_UM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod hp_core1_um_pms_reg0;
#[doc = "REGDMA_PERI_PMS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_peri_pms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_peri_pms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_peri_pms`] module"]
pub type REGDMA_PERI_PMS = crate::Reg<regdma_peri_pms::REGDMA_PERI_PMS_SPEC>;
#[doc = "NA"]
pub mod regdma_peri_pms;
