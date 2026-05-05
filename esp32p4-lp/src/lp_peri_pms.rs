#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pms_date: PMS_DATE,
    pms_clk_en: PMS_CLK_EN,
    lp_mm_pms_reg0: LP_MM_PMS_REG0,
    peri_region0_low: PERI_REGION0_LOW,
    peri_region0_high: PERI_REGION0_HIGH,
    peri_region1_low: PERI_REGION1_LOW,
    peri_region1_high: PERI_REGION1_HIGH,
    peri_region_pms: PERI_REGION_PMS,
    peri_region_2_to_7_pms: PERI_REGION_2_TO_7_PMS,
    peri_region2_low: PERI_REGION2_LOW,
    peri_region2_high: PERI_REGION2_HIGH,
    peri_region3_low: PERI_REGION3_LOW,
    peri_region3_high: PERI_REGION3_HIGH,
    peri_region4_low: PERI_REGION4_LOW,
    peri_region4_high: PERI_REGION4_HIGH,
    peri_region5_low: PERI_REGION5_LOW,
    peri_region5_high: PERI_REGION5_HIGH,
    peri_region6_low: PERI_REGION6_LOW,
    peri_region6_high: PERI_REGION6_HIGH,
    peri_region7_low: PERI_REGION7_LOW,
    peri_region7_high: PERI_REGION7_HIGH,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn pms_date(&self) -> &PMS_DATE {
        &self.pms_date
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
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn peri_region0_low(&self) -> &PERI_REGION0_LOW {
        &self.peri_region0_low
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn peri_region0_high(&self) -> &PERI_REGION0_HIGH {
        &self.peri_region0_high
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn peri_region1_low(&self) -> &PERI_REGION1_LOW {
        &self.peri_region1_low
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn peri_region1_high(&self) -> &PERI_REGION1_HIGH {
        &self.peri_region1_high
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn peri_region_pms(&self) -> &PERI_REGION_PMS {
        &self.peri_region_pms
    }
    #[doc = "0x20 - NA"]
    #[inline(always)]
    pub const fn peri_region_2_to_7_pms(&self) -> &PERI_REGION_2_TO_7_PMS {
        &self.peri_region_2_to_7_pms
    }
    #[doc = "0x24 - NA"]
    #[inline(always)]
    pub const fn peri_region2_low(&self) -> &PERI_REGION2_LOW {
        &self.peri_region2_low
    }
    #[doc = "0x28 - NA"]
    #[inline(always)]
    pub const fn peri_region2_high(&self) -> &PERI_REGION2_HIGH {
        &self.peri_region2_high
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn peri_region3_low(&self) -> &PERI_REGION3_LOW {
        &self.peri_region3_low
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn peri_region3_high(&self) -> &PERI_REGION3_HIGH {
        &self.peri_region3_high
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn peri_region4_low(&self) -> &PERI_REGION4_LOW {
        &self.peri_region4_low
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn peri_region4_high(&self) -> &PERI_REGION4_HIGH {
        &self.peri_region4_high
    }
    #[doc = "0x3c - NA"]
    #[inline(always)]
    pub const fn peri_region5_low(&self) -> &PERI_REGION5_LOW {
        &self.peri_region5_low
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn peri_region5_high(&self) -> &PERI_REGION5_HIGH {
        &self.peri_region5_high
    }
    #[doc = "0x44 - NA"]
    #[inline(always)]
    pub const fn peri_region6_low(&self) -> &PERI_REGION6_LOW {
        &self.peri_region6_low
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn peri_region6_high(&self) -> &PERI_REGION6_HIGH {
        &self.peri_region6_high
    }
    #[doc = "0x4c - NA"]
    #[inline(always)]
    pub const fn peri_region7_low(&self) -> &PERI_REGION7_LOW {
        &self.peri_region7_low
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn peri_region7_high(&self) -> &PERI_REGION7_HIGH {
        &self.peri_region7_high
    }
}
#[doc = "PMS_DATE (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_date`] module"]
pub type PMS_DATE = crate::Reg<pms_date::PMS_DATE_SPEC>;
#[doc = "NA"]
pub mod pms_date;
#[doc = "PMS_CLK_EN (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_clk_en`] module"]
pub type PMS_CLK_EN = crate::Reg<pms_clk_en::PMS_CLK_EN_SPEC>;
#[doc = "NA"]
pub mod pms_clk_en;
#[doc = "LP_MM_PMS_REG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_mm_pms_reg0`] module"]
pub type LP_MM_PMS_REG0 = crate::Reg<lp_mm_pms_reg0::LP_MM_PMS_REG0_SPEC>;
#[doc = "NA"]
pub mod lp_mm_pms_reg0;
#[doc = "PERI_REGION0_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region0_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region0_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region0_low`] module"]
pub type PERI_REGION0_LOW = crate::Reg<peri_region0_low::PERI_REGION0_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region0_low;
#[doc = "PERI_REGION0_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region0_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region0_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region0_high`] module"]
pub type PERI_REGION0_HIGH = crate::Reg<peri_region0_high::PERI_REGION0_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region0_high;
#[doc = "PERI_REGION1_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region1_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region1_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region1_low`] module"]
pub type PERI_REGION1_LOW = crate::Reg<peri_region1_low::PERI_REGION1_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region1_low;
#[doc = "PERI_REGION1_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region1_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region1_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region1_high`] module"]
pub type PERI_REGION1_HIGH = crate::Reg<peri_region1_high::PERI_REGION1_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region1_high;
#[doc = "PERI_REGION_PMS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region_pms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region_pms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region_pms`] module"]
pub type PERI_REGION_PMS = crate::Reg<peri_region_pms::PERI_REGION_PMS_SPEC>;
#[doc = "NA"]
pub mod peri_region_pms;
#[doc = "PERI_REGION_2_TO_7_PMS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region_2_to_7_pms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region_2_to_7_pms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region_2_to_7_pms`] module"]
pub type PERI_REGION_2_TO_7_PMS = crate::Reg<peri_region_2_to_7_pms::PERI_REGION_2_TO_7_PMS_SPEC>;
#[doc = "NA"]
pub mod peri_region_2_to_7_pms;
#[doc = "PERI_REGION2_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region2_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region2_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region2_low`] module"]
pub type PERI_REGION2_LOW = crate::Reg<peri_region2_low::PERI_REGION2_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region2_low;
#[doc = "PERI_REGION2_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region2_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region2_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region2_high`] module"]
pub type PERI_REGION2_HIGH = crate::Reg<peri_region2_high::PERI_REGION2_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region2_high;
#[doc = "PERI_REGION3_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region3_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region3_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region3_low`] module"]
pub type PERI_REGION3_LOW = crate::Reg<peri_region3_low::PERI_REGION3_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region3_low;
#[doc = "PERI_REGION3_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region3_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region3_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region3_high`] module"]
pub type PERI_REGION3_HIGH = crate::Reg<peri_region3_high::PERI_REGION3_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region3_high;
#[doc = "PERI_REGION4_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region4_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region4_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region4_low`] module"]
pub type PERI_REGION4_LOW = crate::Reg<peri_region4_low::PERI_REGION4_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region4_low;
#[doc = "PERI_REGION4_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region4_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region4_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region4_high`] module"]
pub type PERI_REGION4_HIGH = crate::Reg<peri_region4_high::PERI_REGION4_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region4_high;
#[doc = "PERI_REGION5_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region5_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region5_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region5_low`] module"]
pub type PERI_REGION5_LOW = crate::Reg<peri_region5_low::PERI_REGION5_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region5_low;
#[doc = "PERI_REGION5_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region5_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region5_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region5_high`] module"]
pub type PERI_REGION5_HIGH = crate::Reg<peri_region5_high::PERI_REGION5_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region5_high;
#[doc = "PERI_REGION6_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region6_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region6_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region6_low`] module"]
pub type PERI_REGION6_LOW = crate::Reg<peri_region6_low::PERI_REGION6_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region6_low;
#[doc = "PERI_REGION6_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region6_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region6_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region6_high`] module"]
pub type PERI_REGION6_HIGH = crate::Reg<peri_region6_high::PERI_REGION6_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region6_high;
#[doc = "PERI_REGION7_LOW (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region7_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region7_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region7_low`] module"]
pub type PERI_REGION7_LOW = crate::Reg<peri_region7_low::PERI_REGION7_LOW_SPEC>;
#[doc = "NA"]
pub mod peri_region7_low;
#[doc = "PERI_REGION7_HIGH (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region7_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region7_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_region7_high`] module"]
pub type PERI_REGION7_HIGH = crate::Reg<peri_region7_high::PERI_REGION7_HIGH_SPEC>;
#[doc = "NA"]
pub mod peri_region7_high;
