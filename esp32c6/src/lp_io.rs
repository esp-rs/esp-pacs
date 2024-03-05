#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    out_data: OUT_DATA,
    out_data_w1ts: OUT_DATA_W1TS,
    out_data_w1tc: OUT_DATA_W1TC,
    out_enable: OUT_ENABLE,
    out_enable_w1ts: OUT_ENABLE_W1TS,
    out_enable_w1tc: OUT_ENABLE_W1TC,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    in_: IN,
    pin0: PIN0,
    pin1: PIN1,
    pin2: PIN2,
    pin3: PIN3,
    pin4: PIN4,
    pin5: PIN5,
    pin6: PIN6,
    pin7: PIN7,
    gpio0: GPIO0,
    gpio1: GPIO1,
    gpio2: GPIO2,
    gpio3: GPIO3,
    gpio4: GPIO4,
    gpio5: GPIO5,
    gpio6: GPIO6,
    gpio7: GPIO7,
    status_interrupt: STATUS_INTERRUPT,
    debug_sel0: DEBUG_SEL0,
    debug_sel1: DEBUG_SEL1,
    lpi2c: LPI2C,
    _reserved30: [u8; 0x0384],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need des"]
    #[inline(always)]
    pub const fn out_data(&self) -> &OUT_DATA {
        &self.out_data
    }
    #[doc = "0x04 - need des"]
    #[inline(always)]
    pub const fn out_data_w1ts(&self) -> &OUT_DATA_W1TS {
        &self.out_data_w1ts
    }
    #[doc = "0x08 - need des"]
    #[inline(always)]
    pub const fn out_data_w1tc(&self) -> &OUT_DATA_W1TC {
        &self.out_data_w1tc
    }
    #[doc = "0x0c - need des"]
    #[inline(always)]
    pub const fn out_enable(&self) -> &OUT_ENABLE {
        &self.out_enable
    }
    #[doc = "0x10 - need des"]
    #[inline(always)]
    pub const fn out_enable_w1ts(&self) -> &OUT_ENABLE_W1TS {
        &self.out_enable_w1ts
    }
    #[doc = "0x14 - need des"]
    #[inline(always)]
    pub const fn out_enable_w1tc(&self) -> &OUT_ENABLE_W1TC {
        &self.out_enable_w1tc
    }
    #[doc = "0x18 - need des"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x1c - need des"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x20 - need des"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x24 - need des"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x28 - need des"]
    #[inline(always)]
    pub const fn pin0(&self) -> &PIN0 {
        &self.pin0
    }
    #[doc = "0x2c - need des"]
    #[inline(always)]
    pub const fn pin1(&self) -> &PIN1 {
        &self.pin1
    }
    #[doc = "0x30 - need des"]
    #[inline(always)]
    pub const fn pin2(&self) -> &PIN2 {
        &self.pin2
    }
    #[doc = "0x34 - need des"]
    #[inline(always)]
    pub const fn pin3(&self) -> &PIN3 {
        &self.pin3
    }
    #[doc = "0x38 - need des"]
    #[inline(always)]
    pub const fn pin4(&self) -> &PIN4 {
        &self.pin4
    }
    #[doc = "0x3c - need des"]
    #[inline(always)]
    pub const fn pin5(&self) -> &PIN5 {
        &self.pin5
    }
    #[doc = "0x40 - need des"]
    #[inline(always)]
    pub const fn pin6(&self) -> &PIN6 {
        &self.pin6
    }
    #[doc = "0x44 - need des"]
    #[inline(always)]
    pub const fn pin7(&self) -> &PIN7 {
        &self.pin7
    }
    #[doc = "0x48 - need des"]
    #[inline(always)]
    pub const fn gpio0(&self) -> &GPIO0 {
        &self.gpio0
    }
    #[doc = "0x4c - need des"]
    #[inline(always)]
    pub const fn gpio1(&self) -> &GPIO1 {
        &self.gpio1
    }
    #[doc = "0x50 - need des"]
    #[inline(always)]
    pub const fn gpio2(&self) -> &GPIO2 {
        &self.gpio2
    }
    #[doc = "0x54 - need des"]
    #[inline(always)]
    pub const fn gpio3(&self) -> &GPIO3 {
        &self.gpio3
    }
    #[doc = "0x58 - need des"]
    #[inline(always)]
    pub const fn gpio4(&self) -> &GPIO4 {
        &self.gpio4
    }
    #[doc = "0x5c - need des"]
    #[inline(always)]
    pub const fn gpio5(&self) -> &GPIO5 {
        &self.gpio5
    }
    #[doc = "0x60 - need des"]
    #[inline(always)]
    pub const fn gpio6(&self) -> &GPIO6 {
        &self.gpio6
    }
    #[doc = "0x64 - need des"]
    #[inline(always)]
    pub const fn gpio7(&self) -> &GPIO7 {
        &self.gpio7
    }
    #[doc = "0x68 - need des"]
    #[inline(always)]
    pub const fn status_interrupt(&self) -> &STATUS_INTERRUPT {
        &self.status_interrupt
    }
    #[doc = "0x6c - need des"]
    #[inline(always)]
    pub const fn debug_sel0(&self) -> &DEBUG_SEL0 {
        &self.debug_sel0
    }
    #[doc = "0x70 - need des"]
    #[inline(always)]
    pub const fn debug_sel1(&self) -> &DEBUG_SEL1 {
        &self.debug_sel1
    }
    #[doc = "0x74 - need des"]
    #[inline(always)]
    pub const fn lpi2c(&self) -> &LPI2C {
        &self.lpi2c
    }
    #[doc = "0x3fc - need des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OUT_DATA (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_data`] module"]
pub type OUT_DATA = crate::Reg<out_data::OUT_DATA_SPEC>;
#[doc = "need des"]
pub mod out_data;
#[doc = "OUT_DATA_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_data_w1ts`] module"]
pub type OUT_DATA_W1TS = crate::Reg<out_data_w1ts::OUT_DATA_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_data_w1ts;
#[doc = "OUT_DATA_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_data_w1tc`] module"]
pub type OUT_DATA_W1TC = crate::Reg<out_data_w1tc::OUT_DATA_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_data_w1tc;
#[doc = "OUT_ENABLE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_enable`] module"]
pub type OUT_ENABLE = crate::Reg<out_enable::OUT_ENABLE_SPEC>;
#[doc = "need des"]
pub mod out_enable;
#[doc = "OUT_ENABLE_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_enable_w1ts`] module"]
pub type OUT_ENABLE_W1TS = crate::Reg<out_enable_w1ts::OUT_ENABLE_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1ts;
#[doc = "OUT_ENABLE_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_enable_w1tc`] module"]
pub type OUT_ENABLE_W1TC = crate::Reg<out_enable_w1tc::OUT_ENABLE_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1tc;
#[doc = "STATUS (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "need des"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "need des"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "need des"]
pub mod status_w1tc;
#[doc = "IN (r) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "need des"]
pub mod in_;
#[doc = "PIN0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin0`] module"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "need des"]
pub mod pin0;
#[doc = "PIN1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin1`] module"]
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
#[doc = "need des"]
pub mod pin1;
#[doc = "PIN2 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin2`] module"]
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
#[doc = "need des"]
pub mod pin2;
#[doc = "PIN3 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin3`] module"]
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
#[doc = "need des"]
pub mod pin3;
#[doc = "PIN4 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin4`] module"]
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
#[doc = "need des"]
pub mod pin4;
#[doc = "PIN5 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin5`] module"]
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
#[doc = "need des"]
pub mod pin5;
#[doc = "PIN6 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin6`] module"]
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
#[doc = "need des"]
pub mod pin6;
#[doc = "PIN7 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin7`] module"]
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
#[doc = "need des"]
pub mod pin7;
#[doc = "GPIO0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "need des"]
pub mod gpio0;
#[doc = "GPIO1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "need des"]
pub mod gpio1;
#[doc = "GPIO2 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "need des"]
pub mod gpio2;
#[doc = "GPIO3 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "need des"]
pub mod gpio3;
#[doc = "GPIO4 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "need des"]
pub mod gpio4;
#[doc = "GPIO5 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "need des"]
pub mod gpio5;
#[doc = "GPIO6 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "need des"]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "need des"]
pub mod gpio7;
#[doc = "STATUS_INTERRUPT (r) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_interrupt`] module"]
pub type STATUS_INTERRUPT = crate::Reg<status_interrupt::STATUS_INTERRUPT_SPEC>;
#[doc = "need des"]
pub mod status_interrupt;
#[doc = "DEBUG_SEL0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel0`] module"]
pub type DEBUG_SEL0 = crate::Reg<debug_sel0::DEBUG_SEL0_SPEC>;
#[doc = "need des"]
pub mod debug_sel0;
#[doc = "DEBUG_SEL1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel1`] module"]
pub type DEBUG_SEL1 = crate::Reg<debug_sel1::DEBUG_SEL1_SPEC>;
#[doc = "need des"]
pub mod debug_sel1;
#[doc = "LPI2C (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpi2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpi2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpi2c`] module"]
pub type LPI2C = crate::Reg<lpi2c::LPI2C_SPEC>;
#[doc = "need des"]
pub mod lpi2c;
#[doc = "DATE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
