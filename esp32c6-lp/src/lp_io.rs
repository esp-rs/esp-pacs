#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need des"]
    pub out: OUT,
    #[doc = "0x04 - need des"]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x08 - need des"]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x0c - need des"]
    pub enable: ENABLE,
    #[doc = "0x10 - need des"]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x14 - need des"]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x18 - need des"]
    pub status: STATUS,
    #[doc = "0x1c - need des"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x20 - need des"]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x24 - need des"]
    pub in_: IN,
    #[doc = "0x28..0x48 - need des"]
    pub pin: [PIN; 8],
    #[doc = "0x48..0x68 - need des"]
    pub gpio: [GPIO; 8],
    #[doc = "0x68 - need des"]
    pub status_int: STATUS_INT,
    _reserved13: [u8; 0x0390],
    #[doc = "0x3fc - need des"]
    pub date: DATE,
}
#[doc = "OUT (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "need des"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "need des"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "need des"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "need des"]
pub mod enable_w1tc;
#[doc = "STATUS (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "need des"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "need des"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "need des"]
pub mod status_w1tc;
#[doc = "IN (r) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "need des"]
pub mod in_;
#[doc = "PIN (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "need des"]
pub mod pin;
#[doc = "GPIO (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio`] module"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "need des"]
pub mod gpio;
#[doc = "STATUS_INT (r) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_int`] module"]
pub type STATUS_INT = crate::Reg<status_int::STATUS_INT_SPEC>;
#[doc = "need des"]
pub mod status_int;
#[doc = "DATE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
