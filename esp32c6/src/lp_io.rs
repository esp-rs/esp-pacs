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
#[doc = "OUT_DATA (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_data`] module"]
pub type OUT_DATA = crate::Reg<out_data::OUT_DATA_SPEC>;
#[doc = "need des"]
pub mod out_data;
#[doc = "OUT_DATA_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_data_w1ts`] module"]
pub type OUT_DATA_W1TS = crate::Reg<out_data_w1ts::OUT_DATA_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_data_w1ts;
#[doc = "OUT_DATA_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_data_w1tc`] module"]
pub type OUT_DATA_W1TC = crate::Reg<out_data_w1tc::OUT_DATA_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_data_w1tc;
#[doc = "OUT_ENABLE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_enable`] module"]
pub type OUT_ENABLE = crate::Reg<out_enable::OUT_ENABLE_SPEC>;
#[doc = "need des"]
pub mod out_enable;
#[doc = "OUT_ENABLE_W1TS (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_enable_w1ts`] module"]
pub type OUT_ENABLE_W1TS = crate::Reg<out_enable_w1ts::OUT_ENABLE_W1TS_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1ts;
#[doc = "OUT_ENABLE_W1TC (w) register accessor: need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_enable_w1tc`] module"]
pub type OUT_ENABLE_W1TC = crate::Reg<out_enable_w1tc::OUT_ENABLE_W1TC_SPEC>;
#[doc = "need des"]
pub mod out_enable_w1tc;
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
#[doc = "PIN0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin0`] module"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "need des"]
pub mod pin0;
#[doc = "PIN1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin1`] module"]
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
#[doc = "need des"]
pub mod pin1;
#[doc = "PIN2 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin2`] module"]
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
#[doc = "need des"]
pub mod pin2;
#[doc = "PIN3 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin3`] module"]
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
#[doc = "need des"]
pub mod pin3;
#[doc = "PIN4 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin4`] module"]
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
#[doc = "need des"]
pub mod pin4;
#[doc = "PIN5 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin5`] module"]
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
#[doc = "need des"]
pub mod pin5;
#[doc = "PIN6 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin6`] module"]
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
#[doc = "need des"]
pub mod pin6;
#[doc = "PIN7 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin7`] module"]
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
#[doc = "need des"]
pub mod pin7;
#[doc = "GPIO0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio0`] module"]
pub type GPIO0 = crate::Reg<gpio0::GPIO0_SPEC>;
#[doc = "need des"]
pub mod gpio0;
#[doc = "GPIO1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio1`] module"]
pub type GPIO1 = crate::Reg<gpio1::GPIO1_SPEC>;
#[doc = "need des"]
pub mod gpio1;
#[doc = "GPIO2 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio2`] module"]
pub type GPIO2 = crate::Reg<gpio2::GPIO2_SPEC>;
#[doc = "need des"]
pub mod gpio2;
#[doc = "GPIO3 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio3`] module"]
pub type GPIO3 = crate::Reg<gpio3::GPIO3_SPEC>;
#[doc = "need des"]
pub mod gpio3;
#[doc = "GPIO4 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio4`] module"]
pub type GPIO4 = crate::Reg<gpio4::GPIO4_SPEC>;
#[doc = "need des"]
pub mod gpio4;
#[doc = "GPIO5 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio5`] module"]
pub type GPIO5 = crate::Reg<gpio5::GPIO5_SPEC>;
#[doc = "need des"]
pub mod gpio5;
#[doc = "GPIO6 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio6`] module"]
pub type GPIO6 = crate::Reg<gpio6::GPIO6_SPEC>;
#[doc = "need des"]
pub mod gpio6;
#[doc = "GPIO7 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gpio7`] module"]
pub type GPIO7 = crate::Reg<gpio7::GPIO7_SPEC>;
#[doc = "need des"]
pub mod gpio7;
#[doc = "STATUS_INTERRUPT (r) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_interrupt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_interrupt`] module"]
pub type STATUS_INTERRUPT = crate::Reg<status_interrupt::STATUS_INTERRUPT_SPEC>;
#[doc = "need des"]
pub mod status_interrupt;
#[doc = "DEBUG_SEL0 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`debug_sel0`] module"]
pub type DEBUG_SEL0 = crate::Reg<debug_sel0::DEBUG_SEL0_SPEC>;
#[doc = "need des"]
pub mod debug_sel0;
#[doc = "DEBUG_SEL1 (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`debug_sel1`] module"]
pub type DEBUG_SEL1 = crate::Reg<debug_sel1::DEBUG_SEL1_SPEC>;
#[doc = "need des"]
pub mod debug_sel1;
#[doc = "LPI2C (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpi2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpi2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpi2c`] module"]
pub type LPI2C = crate::Reg<lpi2c::LPI2C_SPEC>;
#[doc = "need des"]
pub mod lpi2c;
#[doc = "DATE (rw) register accessor: need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need des"]
pub mod date;
