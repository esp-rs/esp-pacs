#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    clk_en: CLK_EN,
    ver_date: VER_DATE,
    pad0: PAD0,
    pad1: PAD1,
    pad2: PAD2,
    pad3: PAD3,
    pad4: PAD4,
    pad5: PAD5,
    pad6: PAD6,
    pad7: PAD7,
    pad8: PAD8,
    pad9: PAD9,
    pad10: PAD10,
    pad11: PAD11,
    pad120: PAD120,
    pad13: PAD13,
    pad14: PAD14,
    pad15: PAD15,
    ext_wakeup0_sel: EXT_WAKEUP0_SEL,
    lp_pad_hold: LP_PAD_HOLD,
    lp_pad_hys: LP_PAD_HYS,
}
impl RegisterBlock {
    ///0x00 - Reserved
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    ///0x04 - Reserved
    #[inline(always)]
    pub const fn ver_date(&self) -> &VER_DATE {
        &self.ver_date
    }
    ///0x08 - Reserved
    #[inline(always)]
    pub const fn pad0(&self) -> &PAD0 {
        &self.pad0
    }
    ///0x0c - Reserved
    #[inline(always)]
    pub const fn pad1(&self) -> &PAD1 {
        &self.pad1
    }
    ///0x10 - Reserved
    #[inline(always)]
    pub const fn pad2(&self) -> &PAD2 {
        &self.pad2
    }
    ///0x14 - Reserved
    #[inline(always)]
    pub const fn pad3(&self) -> &PAD3 {
        &self.pad3
    }
    ///0x18 - Reserved
    #[inline(always)]
    pub const fn pad4(&self) -> &PAD4 {
        &self.pad4
    }
    ///0x1c - Reserved
    #[inline(always)]
    pub const fn pad5(&self) -> &PAD5 {
        &self.pad5
    }
    ///0x20 - Reserved
    #[inline(always)]
    pub const fn pad6(&self) -> &PAD6 {
        &self.pad6
    }
    ///0x24 - Reserved
    #[inline(always)]
    pub const fn pad7(&self) -> &PAD7 {
        &self.pad7
    }
    ///0x28 - Reserved
    #[inline(always)]
    pub const fn pad8(&self) -> &PAD8 {
        &self.pad8
    }
    ///0x2c - Reserved
    #[inline(always)]
    pub const fn pad9(&self) -> &PAD9 {
        &self.pad9
    }
    ///0x30 - Reserved
    #[inline(always)]
    pub const fn pad10(&self) -> &PAD10 {
        &self.pad10
    }
    ///0x34 - Reserved
    #[inline(always)]
    pub const fn pad11(&self) -> &PAD11 {
        &self.pad11
    }
    ///0x38 - Reserved
    #[inline(always)]
    pub const fn pad120(&self) -> &PAD120 {
        &self.pad120
    }
    ///0x3c - Reserved
    #[inline(always)]
    pub const fn pad13(&self) -> &PAD13 {
        &self.pad13
    }
    ///0x40 - Reserved
    #[inline(always)]
    pub const fn pad14(&self) -> &PAD14 {
        &self.pad14
    }
    ///0x44 - Reserved
    #[inline(always)]
    pub const fn pad15(&self) -> &PAD15 {
        &self.pad15
    }
    ///0x48 - Reserved
    #[inline(always)]
    pub const fn ext_wakeup0_sel(&self) -> &EXT_WAKEUP0_SEL {
        &self.ext_wakeup0_sel
    }
    ///0x4c - Reserved
    #[inline(always)]
    pub const fn lp_pad_hold(&self) -> &LP_PAD_HOLD {
        &self.lp_pad_hold
    }
    ///0x50 - Reserved
    #[inline(always)]
    pub const fn lp_pad_hys(&self) -> &LP_PAD_HYS {
        &self.lp_pad_hys
    }
}
/**CLK_EN (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_en`] module*/
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
///Reserved
pub mod clk_en;
/**VER_DATE (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ver_date`] module*/
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
///Reserved
pub mod ver_date;
/**PAD0 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad0`] module*/
pub type PAD0 = crate::Reg<pad0::PAD0_SPEC>;
///Reserved
pub mod pad0;
/**PAD1 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad1`] module*/
pub type PAD1 = crate::Reg<pad1::PAD1_SPEC>;
///Reserved
pub mod pad1;
/**PAD2 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad2`] module*/
pub type PAD2 = crate::Reg<pad2::PAD2_SPEC>;
///Reserved
pub mod pad2;
/**PAD3 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad3`] module*/
pub type PAD3 = crate::Reg<pad3::PAD3_SPEC>;
///Reserved
pub mod pad3;
/**PAD4 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad4`] module*/
pub type PAD4 = crate::Reg<pad4::PAD4_SPEC>;
///Reserved
pub mod pad4;
/**PAD5 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad5`] module*/
pub type PAD5 = crate::Reg<pad5::PAD5_SPEC>;
///Reserved
pub mod pad5;
/**PAD6 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad6`] module*/
pub type PAD6 = crate::Reg<pad6::PAD6_SPEC>;
///Reserved
pub mod pad6;
/**PAD7 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad7`] module*/
pub type PAD7 = crate::Reg<pad7::PAD7_SPEC>;
///Reserved
pub mod pad7;
/**PAD8 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad8`] module*/
pub type PAD8 = crate::Reg<pad8::PAD8_SPEC>;
///Reserved
pub mod pad8;
/**PAD9 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad9`] module*/
pub type PAD9 = crate::Reg<pad9::PAD9_SPEC>;
///Reserved
pub mod pad9;
/**PAD10 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad10`] module*/
pub type PAD10 = crate::Reg<pad10::PAD10_SPEC>;
///Reserved
pub mod pad10;
/**PAD11 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad11`] module*/
pub type PAD11 = crate::Reg<pad11::PAD11_SPEC>;
///Reserved
pub mod pad11;
/**PAD120 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad120`] module*/
pub type PAD120 = crate::Reg<pad120::PAD120_SPEC>;
///Reserved
pub mod pad120;
/**PAD13 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad13`] module*/
pub type PAD13 = crate::Reg<pad13::PAD13_SPEC>;
///Reserved
pub mod pad13;
/**PAD14 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad14`] module*/
pub type PAD14 = crate::Reg<pad14::PAD14_SPEC>;
///Reserved
pub mod pad14;
/**PAD15 (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad15`] module*/
pub type PAD15 = crate::Reg<pad15::PAD15_SPEC>;
///Reserved
pub mod pad15;
/**EXT_WAKEUP0_SEL (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup0_sel`] module*/
pub type EXT_WAKEUP0_SEL = crate::Reg<ext_wakeup0_sel::EXT_WAKEUP0_SEL_SPEC>;
///Reserved
pub mod ext_wakeup0_sel;
/**LP_PAD_HOLD (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`lp_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_pad_hold`] module*/
pub type LP_PAD_HOLD = crate::Reg<lp_pad_hold::LP_PAD_HOLD_SPEC>;
///Reserved
pub mod lp_pad_hold;
/**LP_PAD_HYS (rw) register accessor: Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`lp_pad_hys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pad_hys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_pad_hys`] module*/
pub type LP_PAD_HYS = crate::Reg<lp_pad_hys::LP_PAD_HYS_SPEC>;
///Reserved
pub mod lp_pad_hys;
