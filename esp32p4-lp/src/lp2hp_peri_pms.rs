#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp2hp_pms_date: LP2HP_PMS_DATE,
    pms_clk_en: PMS_CLK_EN,
    lp_mm_pms_reg0: LP_MM_PMS_REG0,
    _reserved3: [u8; 0x24],
    lp_mm_pms_reg1: LP_MM_PMS_REG1,
    _reserved4: [u8; 0x70],
    lp_mm_pms_reg2: LP_MM_PMS_REG2,
    _reserved5: [u8; 0x74],
    lp_mm_pms_reg3: LP_MM_PMS_REG3,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn lp2hp_pms_date(&self) -> &LP2HP_PMS_DATE {
        &self.lp2hp_pms_date
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn pms_clk_en(&self) -> &PMS_CLK_EN {
        &self.pms_clk_en
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn lp_mm_pms_reg0(&self) -> &LP_MM_PMS_REG0 {
        &self.lp_mm_pms_reg0
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn lp_mm_pms_reg1(&self) -> &LP_MM_PMS_REG1 {
        &self.lp_mm_pms_reg1
    }
    #[doc = "0xa4 - NA"]
    #[inline(always)]
    pub const fn lp_mm_pms_reg2(&self) -> &LP_MM_PMS_REG2 {
        &self.lp_mm_pms_reg2
    }
    #[doc = "0x11c - NA"]
    #[inline(always)]
    pub const fn lp_mm_pms_reg3(&self) -> &LP_MM_PMS_REG3 {
        &self.lp_mm_pms_reg3
    }
}
#[doc = "LP2HP_PMS_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp2hp_pms_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp2hp_pms_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp2hp_pms_date`] module"]
pub type LP2HP_PMS_DATE = crate::Reg<lp2hp_pms_date::LP2HP_PMS_DATE_SPEC>;
#[doc = "NA"]
pub mod lp2hp_pms_date;
#[doc = "PMS_CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_clk_en`] module"]
pub type PMS_CLK_EN = crate::Reg<pms_clk_en::PMS_CLK_EN_SPEC>;
#[doc = "NA"]
pub mod pms_clk_en;
#[doc = "LP_MM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mm_pms_reg0`] module"]
pub type LP_MM_PMS_REG0 = crate::Reg<lp_mm_pms_reg0::LP_MM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod lp_mm_pms_reg0;
#[doc = "LP_MM_PMS_REG1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mm_pms_reg1`] module"]
pub type LP_MM_PMS_REG1 = crate::Reg<lp_mm_pms_reg1::LP_MM_PMS_REG1_SPEC>;
#[doc = "NA"]
pub mod lp_mm_pms_reg1;
#[doc = "LP_MM_PMS_REG2 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mm_pms_reg2`] module"]
pub type LP_MM_PMS_REG2 = crate::Reg<lp_mm_pms_reg2::LP_MM_PMS_REG2_SPEC>;
#[doc = "NA"]
pub mod lp_mm_pms_reg2;
#[doc = "LP_MM_PMS_REG3 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mm_pms_reg3`] module"]
pub type LP_MM_PMS_REG3 = crate::Reg<lp_mm_pms_reg3::LP_MM_PMS_REG3_SPEC>;
#[doc = "NA"]
pub mod lp_mm_pms_reg3;
