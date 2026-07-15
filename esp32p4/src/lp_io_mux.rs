#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    ver_date: VER_DATE,
    pad: [PAD; 16],
    ext_wakeup0_sel: EXT_WAKEUP0_SEL,
    lp_pad_hold: LP_PAD_HOLD,
    lp_pad_hys: LP_PAD_HYS,
}
impl RegisterBlock {
    #[doc = "0x00 - Reserved"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - Reserved"]
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    #[doc = "0x08..0x48 - LP IO MUX configuration register for pad %s"]
    #[inline(always)]
    pub const fn pad(&self, n: usize) -> &PAD {
        &self.pad[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x48 - LP IO MUX configuration register for pad %s"]
    #[inline(always)]
    pub fn pad_iter(&self) -> impl Iterator<Item = &PAD> {
        self.pad.iter()
    }
    #[doc = "0x48 - Reserved"]
    #[inline(always)]
    pub const fn ext_wakeup0_sel(&self) -> &EXT_WAKEUP0_SEL {
        &self.ext_wakeup0_sel
    }
    #[doc = "0x4c - Reserved"]
    #[inline(always)]
    pub const fn lp_pad_hold(&self) -> &LP_PAD_HOLD {
        &self.lp_pad_hold
    }
    #[doc = "0x50 - Reserved"]
    #[inline(always)]
    pub const fn lp_pad_hys(&self) -> &LP_PAD_HYS {
        &self.lp_pad_hys
    }
}
#[doc = "CLK_EN (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "Reserved"]
pub mod clk_en;
#[doc = "VER_DATE (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "Reserved"]
pub mod ver_date;
#[doc = "PAD (rw) register accessor: LP IO MUX configuration register for pad %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad`] module"]
pub type PAD = crate::Reg<pad::PAD_SPEC>;
#[doc = "LP IO MUX configuration register for pad %s"]
pub mod pad;
#[doc = "EXT_WAKEUP0_SEL (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup0_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup0_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup0_sel`] module"]
pub type EXT_WAKEUP0_SEL = crate::Reg<ext_wakeup0_sel::EXT_WAKEUP0_SEL_SPEC>;
#[doc = "Reserved"]
pub mod ext_wakeup0_sel;
#[doc = "LP_PAD_HOLD (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pad_hold`] module"]
pub type LP_PAD_HOLD = crate::Reg<lp_pad_hold::LP_PAD_HOLD_SPEC>;
#[doc = "Reserved"]
pub mod lp_pad_hold;
#[doc = "LP_PAD_HYS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_pad_hys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_pad_hys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_pad_hys`] module"]
pub type LP_PAD_HYS = crate::Reg<lp_pad_hys::LP_PAD_HYS_SPEC>;
#[doc = "Reserved"]
pub mod lp_pad_hys;
