#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need des"]
    pub out_data: OUT_DATA,
    #[doc = "0x04 - need des"]
    pub out_data_w1ts: OUT_DATA_W1TS,
    #[doc = "0x08 - need des"]
    pub out_data_w1tc: OUT_DATA_W1TC,
    #[doc = "0x0c - need des"]
    pub out_enable: OUT_ENABLE,
    #[doc = "0x10 - need des"]
    pub out_enable_w1ts: OUT_ENABLE_W1TS,
    #[doc = "0x14 - need des"]
    pub out_enable_w1tc: OUT_ENABLE_W1TC,
    #[doc = "0x18 - need des"]
    pub status: STATUS,
    #[doc = "0x1c - need des"]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x20 - need des"]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x24 - need des"]
    pub in_: IN,
    #[doc = "0x28 - need des"]
    pub pin0: PIN0,
    #[doc = "0x2c - need des"]
    pub pin1: PIN1,
    #[doc = "0x30 - need des"]
    pub pin2: PIN2,
    #[doc = "0x34 - need des"]
    pub pin3: PIN3,
    #[doc = "0x38 - need des"]
    pub pin4: PIN4,
    #[doc = "0x3c - need des"]
    pub pin5: PIN5,
    #[doc = "0x40 - need des"]
    pub pin6: PIN6,
    #[doc = "0x44 - need des"]
    pub pin7: PIN7,
    #[doc = "0x48 - need des"]
    pub gpio0: GPIO0,
    #[doc = "0x4c - need des"]
    pub gpio1: GPIO1,
    #[doc = "0x50 - need des"]
    pub gpio2: GPIO2,
    #[doc = "0x54 - need des"]
    pub gpio3: GPIO3,
    #[doc = "0x58 - need des"]
    pub gpio4: GPIO4,
    #[doc = "0x5c - need des"]
    pub gpio5: GPIO5,
    #[doc = "0x60 - need des"]
    pub gpio6: GPIO6,
    #[doc = "0x64 - need des"]
    pub gpio7: GPIO7,
    #[doc = "0x68 - need des"]
    pub status_interrupt: STATUS_INTERRUPT,
    #[doc = "0x6c - need des"]
    pub debug_sel0: DEBUG_SEL0,
    #[doc = "0x70 - need des"]
    pub debug_sel1: DEBUG_SEL1,
    #[doc = "0x74 - need des"]
    pub lpi2c: LPI2C,
    _reserved30: [u8; 0x0384],
    #[doc = "0x3fc - need des"]
    pub date: DATE,
}
#[doc = "OUT_DATA (rw) register accessor: an alias for `Reg<OUT_DATA_SPEC>`"]
pub type OUT_DATA = crate::Reg<out_data::OUT_DATA_SPEC>;
#[doc = "need des"]
pub mod out_data;
#[doc = "OUT_DATA_W1TS (w) register accessor: an alias for `Reg<OUT_DATA_W1TS_SPEC>`"]
pub type OUT_DATA_W1TS = crate::Reg<out_data_w1ts::OUT_DATA_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_data_w1ts;
#[doc = "OUT_DATA_W1TC (w) register accessor: an alias for `Reg<OUT_DATA_W1TC_SPEC>`"]
pub type OUT_DATA_W1TC = crate::Reg<out_data_w1tc::OUT_DATA_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_data_w1tc;
#[doc = "OUT_ENABLE (rw) register accessor: an alias for `Reg<OUT_ENABLE_SPEC>`"]
pub type OUT_ENABLE = crate::Reg<out_enable::OUT_ENABLE_SPEC>;
#[doc = "need des"]
pub mod out_enable;
#[doc = "OUT_ENABLE_W1TS (w) register accessor: an alias for `Reg<OUT_ENABLE_W1TS_SPEC>`"]
pub type OUT_ENABLE_W1TS = crate::Reg<out_enable_w1ts::OUT_ENABLE_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1ts;
#[doc = "OUT_ENABLE_W1TC (w) register accessor: an alias for `Reg<OUT_ENABLE_W1TC_SPEC>`"]
pub type OUT_ENABLE_W1TC = crate::Reg<out_enable_w1tc::OUT_ENABLE_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1tc;
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
#[doc = "PIN0 (rw) register accessor: an alias for `Reg<PIN0_SPEC>`"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "need des"]
pub mod pin0;
#[doc = "PIN1 (rw) register accessor: an alias for `Reg<PIN1_SPEC>`"]
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
#[doc = "need des"]
pub mod pin1;
#[doc = "PIN2 (rw) register accessor: an alias for `Reg<PIN2_SPEC>`"]
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
#[doc = "need des"]
pub mod pin2;
#[doc = "PIN3 (rw) register accessor: an alias for `Reg<PIN3_SPEC>`"]
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
#[doc = "need des"]
pub mod pin3;
#[doc = "PIN4 (rw) register accessor: an alias for `Reg<PIN4_SPEC>`"]
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
#[doc = "need des"]
pub mod pin4;
#[doc = "PIN5 (rw) register accessor: an alias for `Reg<PIN5_SPEC>`"]
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
#[doc = "need des"]
pub mod pin5;
#[doc = "PIN6 (rw) register accessor: an alias for `Reg<PIN6_SPEC>`"]
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
#[doc = "need des"]
pub mod pin6;
#[doc = "PIN7 (rw) register accessor: an alias for `Reg<PIN7_SPEC>`"]
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
#[doc = "need des"]
pub mod pin7;
#[doc = "GPIO0 (rw) register accessor: an alias for `Reg<GPIO0_SPEC>`"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "need des"]
pub mod gpio0;
#[doc = "GPIO1 (rw) register accessor: an alias for `Reg<GPIO1_SPEC>`"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "need des"]
pub mod gpio1;
#[doc = "GPIO2 (rw) register accessor: an alias for `Reg<GPIO2_SPEC>`"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "need des"]
pub mod gpio2;
#[doc = "GPIO3 (rw) register accessor: an alias for `Reg<GPIO3_SPEC>`"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "need des"]
pub mod gpio3;
#[doc = "GPIO4 (rw) register accessor: an alias for `Reg<GPIO4_SPEC>`"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "need des"]
pub mod gpio4;
#[doc = "GPIO5 (rw) register accessor: an alias for `Reg<GPIO5_SPEC>`"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "need des"]
pub mod gpio5;
#[doc = "GPIO6 (rw) register accessor: an alias for `Reg<GPIO6_SPEC>`"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "need des"]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: an alias for `Reg<GPIO7_SPEC>`"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "need des"]
pub mod gpio7;
#[doc = "STATUS_INTERRUPT (r) register accessor: an alias for `Reg<STATUS_INTERRUPT_SPEC>`"]
pub type STATUS_INTERRUPT = crate::Reg<status_interrupt::STATUS_INTERRUPT_SPEC>;
#[doc = "need des"]
pub mod status_interrupt;
#[doc = "DEBUG_SEL0 (rw) register accessor: an alias for `Reg<DEBUG_SEL0_SPEC>`"]
pub type DEBUG_SEL0 = crate::Reg<debug_sel0::DEBUG_SEL0_SPEC>;
#[doc = "need des"]
pub mod debug_sel0;
#[doc = "DEBUG_SEL1 (rw) register accessor: an alias for `Reg<DEBUG_SEL1_SPEC>`"]
pub type DEBUG_SEL1 = crate::Reg<debug_sel1::DEBUG_SEL1_SPEC>;
#[doc = "need des"]
pub mod debug_sel1;
#[doc = "LPI2C (rw) register accessor: an alias for `Reg<LPI2C_SPEC>`"]
pub type LPI2C = crate::Reg<lpi2c::LPI2C_SPEC>;
#[doc = "need des"]
pub mod lpi2c;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
