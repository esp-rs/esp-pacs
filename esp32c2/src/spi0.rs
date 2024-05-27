#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    _reserved7: [u8; 0x08],
    rd_status: RD_STATUS,
    _reserved8: [u8; 0x04],
    misc: MISC,
    _reserved9: [u8; 0x04],
    cache_fctrl: CACHE_FCTRL,
    _reserved10: [u8; 0x14],
    fsm: FSM,
    _reserved11: [u8; 0x50],
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    _reserved15: [u8; 0x24],
    clock_gate: CLOCK_GATE,
    core_clk_sel: CORE_CLK_SEL,
    _reserved17: [u8; 0x0318],
    date: DATE,
}
impl RegisterBlock {
    ///0x08 - SPI0 control register.
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI0 control1 register.
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    ///0x10 - SPI0 control2 register.
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x14 - SPI clock division control register.
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x18 - SPI0 user register.
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x1c - SPI0 user1 register.
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x20 - SPI0 user2 register.
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x2c - SPI0 read control register.
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    ///0x34 - SPI0 misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x3c - SPI0 bit mode control register.
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    ///0x54 - SPI0 FSM status register
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    ///0xa8 - SPI0 timing calibration register
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    ///0xac - SPI0 input delay mode control register
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    ///0xb0 - SPI0 input delay number control register
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    ///0xb4 - SPI0 output delay mode control register
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    ///0xdc - SPI0 clk_gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0xe0 - SPI0 module clock select register
    #[inline(always)]
    pub const fn core_clk_sel(&self) -> &CORE_CLK_SEL {
        &self.core_clk_sel
    }
    ///0x3fc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CTRL (rw) register accessor: SPI0 control register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI0 control register.
pub mod ctrl;
/**CTRL1 (rw) register accessor: SPI0 control1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
///SPI0 control1 register.
pub mod ctrl1;
/**CTRL2 (rw) register accessor: SPI0 control2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///SPI0 control2 register.
pub mod ctrl2;
/**CLOCK (rw) register accessor: SPI clock division control register.

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI clock division control register.
pub mod clock;
/**USER (rw) register accessor: SPI0 user register.

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI0 user register.
pub mod user;
/**USER1 (rw) register accessor: SPI0 user1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI0 user1 register.
pub mod user1;
/**USER2 (rw) register accessor: SPI0 user2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI0 user2 register.
pub mod user2;
/**RD_STATUS (rw) register accessor: SPI0 read control register.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_status`] module*/
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
///SPI0 read control register.
pub mod rd_status;
/**MISC (rw) register accessor: SPI0 misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI0 misc register
pub mod misc;
/**CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_fctrl`] module*/
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
///SPI0 bit mode control register.
pub mod cache_fctrl;
/**FSM (rw) register accessor: SPI0 FSM status register

You can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsm`] module*/
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
///SPI0 FSM status register
pub mod fsm;
/**TIMING_CALI (r) register accessor: SPI0 timing calibration register

You can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing_cali`] module*/
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
///SPI0 timing calibration register
pub mod timing_cali;
/**DIN_MODE (r) register accessor: SPI0 input delay mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_mode`] module*/
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
///SPI0 input delay mode control register
pub mod din_mode;
/**DIN_NUM (r) register accessor: SPI0 input delay number control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_num`] module*/
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
///SPI0 input delay number control register
pub mod din_num;
/**DOUT_MODE (r) register accessor: SPI0 output delay mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_mode`] module*/
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
///SPI0 output delay mode control register
pub mod dout_mode;
/**CLOCK_GATE (rw) register accessor: SPI0 clk_gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///SPI0 clk_gate register
pub mod clock_gate;
/**CORE_CLK_SEL (rw) register accessor: SPI0 module clock select register

You can [`read`](crate::generic::Reg::read) this register and get [`core_clk_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_clk_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_clk_sel`] module*/
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
///SPI0 module clock select register
pub mod core_clk_sel;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
