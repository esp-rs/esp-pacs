#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    ver_date: VER_DATE,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status_next: STATUS_NEXT,
    in_: IN,
    pin0: PIN0,
    pin1: PIN1,
    pin2: PIN2,
    pin3: PIN3,
    pin4: PIN4,
    pin5: PIN5,
    pin6: PIN6,
    pin7: PIN7,
    pin8: PIN8,
    pin9: PIN9,
    pin10: PIN10,
    pin11: PIN11,
    pin12: PIN12,
    pin13: PIN13,
    pin14: PIN14,
    pin15: PIN15,
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC1_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC2_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC3_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC4_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC5_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC6_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC7_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC8_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC9_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC10_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC11_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC12_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC13_IN_SEL_CFG,
    _reserved43: [u8; 0x48],
    func0_out_sel_cfg: FUNC0_OUT_SEL_CFG,
    func1_out_sel_cfg: FUNC1_OUT_SEL_CFG,
    func2_out_sel_cfg: FUNC2_OUT_SEL_CFG,
    func3_out_sel_cfg: FUNC3_OUT_SEL_CFG,
    func4_out_sel_cfg: FUNC4_OUT_SEL_CFG,
    func5_out_sel_cfg: FUNC5_OUT_SEL_CFG,
    func6_out_sel_cfg: FUNC6_OUT_SEL_CFG,
    func7_out_sel_cfg: FUNC7_OUT_SEL_CFG,
    func8_out_sel_cfg: FUNC8_OUT_SEL_CFG,
    func9_out_sel_cfg: FUNC9_OUT_SEL_CFG,
    func10_out_sel_cfg: FUNC10_OUT_SEL_CFG,
    func11_out_sel_cfg: FUNC11_OUT_SEL_CFG,
    func12_out_sel_cfg: FUNC12_OUT_SEL_CFG,
    func13_out_sel_cfg: FUNC13_OUT_SEL_CFG,
    func14_out_sel_cfg: FUNC14_OUT_SEL_CFG,
    func15_out_sel_cfg: FUNC15_OUT_SEL_CFG,
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
    #[doc = "0x08 - Reserved"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x0c - Reserved"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x10 - Reserved"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x14 - Reserved"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x18 - Reserved"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x1c - Reserved"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x20 - Reserved"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x24 - Reserved"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x28 - Reserved"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x2c - Reserved"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0x30 - Reserved"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x34 - Reserved"]
    #[inline(always)]
    pub const fn pin0(&self) -> &PIN0 {
        &self.pin0
    }
    #[doc = "0x38 - Reserved"]
    #[inline(always)]
    pub const fn pin1(&self) -> &PIN1 {
        &self.pin1
    }
    #[doc = "0x3c - Reserved"]
    #[inline(always)]
    pub const fn pin2(&self) -> &PIN2 {
        &self.pin2
    }
    #[doc = "0x40 - Reserved"]
    #[inline(always)]
    pub const fn pin3(&self) -> &PIN3 {
        &self.pin3
    }
    #[doc = "0x44 - Reserved"]
    #[inline(always)]
    pub const fn pin4(&self) -> &PIN4 {
        &self.pin4
    }
    #[doc = "0x48 - Reserved"]
    #[inline(always)]
    pub const fn pin5(&self) -> &PIN5 {
        &self.pin5
    }
    #[doc = "0x4c - Reserved"]
    #[inline(always)]
    pub const fn pin6(&self) -> &PIN6 {
        &self.pin6
    }
    #[doc = "0x50 - Reserved"]
    #[inline(always)]
    pub const fn pin7(&self) -> &PIN7 {
        &self.pin7
    }
    #[doc = "0x54 - Reserved"]
    #[inline(always)]
    pub const fn pin8(&self) -> &PIN8 {
        &self.pin8
    }
    #[doc = "0x58 - Reserved"]
    #[inline(always)]
    pub const fn pin9(&self) -> &PIN9 {
        &self.pin9
    }
    #[doc = "0x5c - Reserved"]
    #[inline(always)]
    pub const fn pin10(&self) -> &PIN10 {
        &self.pin10
    }
    #[doc = "0x60 - Reserved"]
    #[inline(always)]
    pub const fn pin11(&self) -> &PIN11 {
        &self.pin11
    }
    #[doc = "0x64 - Reserved"]
    #[inline(always)]
    pub const fn pin12(&self) -> &PIN12 {
        &self.pin12
    }
    #[doc = "0x68 - Reserved"]
    #[inline(always)]
    pub const fn pin13(&self) -> &PIN13 {
        &self.pin13
    }
    #[doc = "0x6c - Reserved"]
    #[inline(always)]
    pub const fn pin14(&self) -> &PIN14 {
        &self.pin14
    }
    #[doc = "0x70 - Reserved"]
    #[inline(always)]
    pub const fn pin15(&self) -> &PIN15 {
        &self.pin15
    }
    #[doc = "0x74 - Reserved"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x78 - Reserved"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC1_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x7c - Reserved"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC2_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x80 - Reserved"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC3_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x84 - Reserved"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC4_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x88 - Reserved"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC5_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x8c - Reserved"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC6_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x90 - Reserved"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC7_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x94 - Reserved"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC8_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x98 - Reserved"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC9_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x9c - Reserved"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC10_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0xa0 - Reserved"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC11_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0xa4 - Reserved"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC12_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0xa8 - Reserved"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC13_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0xf4 - Reserved"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC0_OUT_SEL_CFG {
        &self.func0_out_sel_cfg
    }
    #[doc = "0xf8 - Reserved"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC1_OUT_SEL_CFG {
        &self.func1_out_sel_cfg
    }
    #[doc = "0xfc - Reserved"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC2_OUT_SEL_CFG {
        &self.func2_out_sel_cfg
    }
    #[doc = "0x100 - Reserved"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC3_OUT_SEL_CFG {
        &self.func3_out_sel_cfg
    }
    #[doc = "0x104 - Reserved"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC4_OUT_SEL_CFG {
        &self.func4_out_sel_cfg
    }
    #[doc = "0x108 - Reserved"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC5_OUT_SEL_CFG {
        &self.func5_out_sel_cfg
    }
    #[doc = "0x10c - Reserved"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC6_OUT_SEL_CFG {
        &self.func6_out_sel_cfg
    }
    #[doc = "0x110 - Reserved"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC7_OUT_SEL_CFG {
        &self.func7_out_sel_cfg
    }
    #[doc = "0x114 - Reserved"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC8_OUT_SEL_CFG {
        &self.func8_out_sel_cfg
    }
    #[doc = "0x118 - Reserved"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC9_OUT_SEL_CFG {
        &self.func9_out_sel_cfg
    }
    #[doc = "0x11c - Reserved"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC10_OUT_SEL_CFG {
        &self.func10_out_sel_cfg
    }
    #[doc = "0x120 - Reserved"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC11_OUT_SEL_CFG {
        &self.func11_out_sel_cfg
    }
    #[doc = "0x124 - Reserved"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC12_OUT_SEL_CFG {
        &self.func12_out_sel_cfg
    }
    #[doc = "0x128 - Reserved"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC13_OUT_SEL_CFG {
        &self.func13_out_sel_cfg
    }
    #[doc = "0x12c - Reserved"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC14_OUT_SEL_CFG {
        &self.func14_out_sel_cfg
    }
    #[doc = "0x130 - Reserved"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC15_OUT_SEL_CFG {
        &self.func15_out_sel_cfg
    }
}
#[doc = "CLK_EN (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "Reserved"]
pub mod clk_en;
#[doc = "VER_DATE (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver_date`] module"]
pub type VER_DATE = crate::Reg<ver_date::VER_DATE_SPEC>;
#[doc = "Reserved"]
pub mod ver_date;
#[doc = "OUT (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Reserved"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "Reserved"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "Reserved"]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Reserved"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "Reserved"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "Reserved"]
pub mod enable_w1tc;
#[doc = "STATUS (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Reserved"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "Reserved"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: Reserved\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "Reserved"]
pub mod status_w1tc;
#[doc = "STATUS_NEXT (r) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "Reserved"]
pub mod status_next;
#[doc = "IN (r) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Reserved"]
pub mod in_;
#[doc = "PIN0 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin0`] module"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "Reserved"]
pub mod pin0;
#[doc = "PIN1 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin1`] module"]
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
#[doc = "Reserved"]
pub mod pin1;
#[doc = "PIN2 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin2`] module"]
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
#[doc = "Reserved"]
pub mod pin2;
#[doc = "PIN3 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin3`] module"]
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
#[doc = "Reserved"]
pub mod pin3;
#[doc = "PIN4 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin4`] module"]
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
#[doc = "Reserved"]
pub mod pin4;
#[doc = "PIN5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin5`] module"]
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
#[doc = "Reserved"]
pub mod pin5;
#[doc = "PIN6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin6`] module"]
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
#[doc = "Reserved"]
pub mod pin6;
#[doc = "PIN7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin7`] module"]
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
#[doc = "Reserved"]
pub mod pin7;
#[doc = "PIN8 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin8`] module"]
pub type PIN8 = crate::Reg<pin8::PIN8_SPEC>;
#[doc = "Reserved"]
pub mod pin8;
#[doc = "PIN9 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin9`] module"]
pub type PIN9 = crate::Reg<pin9::PIN9_SPEC>;
#[doc = "Reserved"]
pub mod pin9;
#[doc = "PIN10 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin10`] module"]
pub type PIN10 = crate::Reg<pin10::PIN10_SPEC>;
#[doc = "Reserved"]
pub mod pin10;
#[doc = "PIN11 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin11`] module"]
pub type PIN11 = crate::Reg<pin11::PIN11_SPEC>;
#[doc = "Reserved"]
pub mod pin11;
#[doc = "PIN12 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin12`] module"]
pub type PIN12 = crate::Reg<pin12::PIN12_SPEC>;
#[doc = "Reserved"]
pub mod pin12;
#[doc = "PIN13 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin13`] module"]
pub type PIN13 = crate::Reg<pin13::PIN13_SPEC>;
#[doc = "Reserved"]
pub mod pin13;
#[doc = "PIN14 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin14`] module"]
pub type PIN14 = crate::Reg<pin14::PIN14_SPEC>;
#[doc = "Reserved"]
pub mod pin14;
#[doc = "PIN15 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin15`] module"]
pub type PIN15 = crate::Reg<pin15::PIN15_SPEC>;
#[doc = "Reserved"]
pub mod pin15;
#[doc = "FUNC0_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func0_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func0_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func0_in_sel_cfg`] module"]
pub type FUNC0_IN_SEL_CFG = crate::Reg<func0_in_sel_cfg::FUNC0_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func0_in_sel_cfg;
#[doc = "FUNC1_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func1_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func1_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func1_in_sel_cfg`] module"]
pub type FUNC1_IN_SEL_CFG = crate::Reg<func1_in_sel_cfg::FUNC1_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func1_in_sel_cfg;
#[doc = "FUNC2_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func2_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func2_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func2_in_sel_cfg`] module"]
pub type FUNC2_IN_SEL_CFG = crate::Reg<func2_in_sel_cfg::FUNC2_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func2_in_sel_cfg;
#[doc = "FUNC3_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func3_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func3_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func3_in_sel_cfg`] module"]
pub type FUNC3_IN_SEL_CFG = crate::Reg<func3_in_sel_cfg::FUNC3_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func3_in_sel_cfg;
#[doc = "FUNC4_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func4_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func4_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func4_in_sel_cfg`] module"]
pub type FUNC4_IN_SEL_CFG = crate::Reg<func4_in_sel_cfg::FUNC4_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func4_in_sel_cfg;
#[doc = "FUNC5_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func5_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func5_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func5_in_sel_cfg`] module"]
pub type FUNC5_IN_SEL_CFG = crate::Reg<func5_in_sel_cfg::FUNC5_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func5_in_sel_cfg;
#[doc = "FUNC6_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func6_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func6_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func6_in_sel_cfg`] module"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func6_in_sel_cfg;
#[doc = "FUNC7_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func7_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func7_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func7_in_sel_cfg`] module"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func7_in_sel_cfg;
#[doc = "FUNC8_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func8_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func8_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func8_in_sel_cfg`] module"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func8_in_sel_cfg;
#[doc = "FUNC9_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func9_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func9_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func9_in_sel_cfg`] module"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func9_in_sel_cfg;
#[doc = "FUNC10_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func10_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func10_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func10_in_sel_cfg`] module"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func10_in_sel_cfg;
#[doc = "FUNC11_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func11_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func11_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func11_in_sel_cfg`] module"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func11_in_sel_cfg;
#[doc = "FUNC12_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func12_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func12_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func12_in_sel_cfg`] module"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func12_in_sel_cfg;
#[doc = "FUNC13_IN_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func13_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func13_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func13_in_sel_cfg`] module"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func13_in_sel_cfg;
#[doc = "FUNC0_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func0_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func0_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func0_out_sel_cfg`] module"]
pub type FUNC0_OUT_SEL_CFG = crate::Reg<func0_out_sel_cfg::FUNC0_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func0_out_sel_cfg;
#[doc = "FUNC1_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func1_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func1_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func1_out_sel_cfg`] module"]
pub type FUNC1_OUT_SEL_CFG = crate::Reg<func1_out_sel_cfg::FUNC1_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func1_out_sel_cfg;
#[doc = "FUNC2_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func2_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func2_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func2_out_sel_cfg`] module"]
pub type FUNC2_OUT_SEL_CFG = crate::Reg<func2_out_sel_cfg::FUNC2_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func2_out_sel_cfg;
#[doc = "FUNC3_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func3_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func3_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func3_out_sel_cfg`] module"]
pub type FUNC3_OUT_SEL_CFG = crate::Reg<func3_out_sel_cfg::FUNC3_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func3_out_sel_cfg;
#[doc = "FUNC4_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func4_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func4_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func4_out_sel_cfg`] module"]
pub type FUNC4_OUT_SEL_CFG = crate::Reg<func4_out_sel_cfg::FUNC4_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func4_out_sel_cfg;
#[doc = "FUNC5_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func5_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func5_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func5_out_sel_cfg`] module"]
pub type FUNC5_OUT_SEL_CFG = crate::Reg<func5_out_sel_cfg::FUNC5_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func5_out_sel_cfg;
#[doc = "FUNC6_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func6_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func6_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func6_out_sel_cfg`] module"]
pub type FUNC6_OUT_SEL_CFG = crate::Reg<func6_out_sel_cfg::FUNC6_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func6_out_sel_cfg;
#[doc = "FUNC7_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func7_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func7_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func7_out_sel_cfg`] module"]
pub type FUNC7_OUT_SEL_CFG = crate::Reg<func7_out_sel_cfg::FUNC7_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func7_out_sel_cfg;
#[doc = "FUNC8_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func8_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func8_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func8_out_sel_cfg`] module"]
pub type FUNC8_OUT_SEL_CFG = crate::Reg<func8_out_sel_cfg::FUNC8_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func8_out_sel_cfg;
#[doc = "FUNC9_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func9_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func9_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func9_out_sel_cfg`] module"]
pub type FUNC9_OUT_SEL_CFG = crate::Reg<func9_out_sel_cfg::FUNC9_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func9_out_sel_cfg;
#[doc = "FUNC10_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func10_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func10_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func10_out_sel_cfg`] module"]
pub type FUNC10_OUT_SEL_CFG = crate::Reg<func10_out_sel_cfg::FUNC10_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func10_out_sel_cfg;
#[doc = "FUNC11_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func11_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func11_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func11_out_sel_cfg`] module"]
pub type FUNC11_OUT_SEL_CFG = crate::Reg<func11_out_sel_cfg::FUNC11_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func11_out_sel_cfg;
#[doc = "FUNC12_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func12_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func12_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func12_out_sel_cfg`] module"]
pub type FUNC12_OUT_SEL_CFG = crate::Reg<func12_out_sel_cfg::FUNC12_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func12_out_sel_cfg;
#[doc = "FUNC13_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func13_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func13_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func13_out_sel_cfg`] module"]
pub type FUNC13_OUT_SEL_CFG = crate::Reg<func13_out_sel_cfg::FUNC13_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func13_out_sel_cfg;
#[doc = "FUNC14_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func14_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func14_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func14_out_sel_cfg`] module"]
pub type FUNC14_OUT_SEL_CFG = crate::Reg<func14_out_sel_cfg::FUNC14_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func14_out_sel_cfg;
#[doc = "FUNC15_OUT_SEL_CFG (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func15_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func15_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func15_out_sel_cfg`] module"]
pub type FUNC15_OUT_SEL_CFG = crate::Reg<func15_out_sel_cfg::FUNC15_OUT_SEL_CFG_SPEC>;
#[doc = "Reserved"]
pub mod func15_out_sel_cfg;
