#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    in_: IN,
    pin: [PIN; 8],
    gpio: [GPIO; 8],
    status_int: STATUS_INT,
    _reserved13: [u8; 0x0390],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need des
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x04 - need des
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x08 - need des
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x0c - need des
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x10 - need des
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x14 - need des
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x18 - need des
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x1c - need des
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x20 - need des
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x24 - need des
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x28..0x48 - need des
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x28..0x48 - need des
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    ///0x48..0x68 - need des
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    ///Iterator for array of:
    ///0x48..0x68 - need des
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    ///0x68 - need des
    #[inline(always)]
    pub const fn status_int(&self) -> &STATUS_INT {
        &self.status_int
    }
    ///0x3fc - need des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OUT (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///need des
pub mod out;
/**OUT_W1TS (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///need des
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///need des
pub mod out_w1tc;
/**ENABLE (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///need des
pub mod enable;
/**ENABLE_W1TS (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///need des
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///need des
pub mod enable_w1tc;
/**STATUS (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///need des
pub mod status;
/**STATUS_W1TS (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///need des
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///need des
pub mod status_w1tc;
/**IN (r) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///need des
pub mod in_;
/**PIN (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///need des
pub mod pin;
/**GPIO (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio`] module*/
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
///need des
pub mod gpio;
/**STATUS_INT (r) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`status_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_int`] module*/
pub type STATUS_INT = crate::Reg<status_int::STATUS_INT_SPEC>;
///need des
pub mod status_int;
/**DATE (rw) register accessor: need des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need des
pub mod date;
