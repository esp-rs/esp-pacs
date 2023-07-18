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
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "need des"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "need des"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "need des"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "need des"]
pub mod enable_w1tc;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "need des"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "need des"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "need des"]
pub mod status_w1tc;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "need des"]
pub mod in_;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "need des"]
pub mod pin;
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "need des"]
pub mod gpio;
#[doc = "STATUS_INT (r) register accessor: an alias for `Reg<STATUS_INT_SPEC>`"]
pub type STATUS_INT = crate::Reg<status_int::STATUS_INT_SPEC>;
#[doc = "need des"]
pub mod status_int;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
