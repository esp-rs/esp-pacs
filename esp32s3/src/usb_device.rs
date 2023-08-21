#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Endpoint 1 FIFO register"]
    pub ep1: EP1,
    #[doc = "0x04 - Endpoint 1 configure and status register"]
    pub ep1_conf: EP1_CONF,
    #[doc = "0x08 - Raw status interrupt"]
    pub int_raw: INT_RAW,
    #[doc = "0x0c - Masked interrupt"]
    pub int_st: INT_ST,
    #[doc = "0x10 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x14 - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x18 - Configure 0 register"]
    pub conf0: CONF0,
    #[doc = "0x1c - USB Internal PHY test register"]
    pub test: TEST,
    #[doc = "0x20 - USB-JTAG FIFO status"]
    pub jfifo_st: JFIFO_ST,
    #[doc = "0x24 - SOF frame number"]
    pub fram_num: FRAM_NUM,
    #[doc = "0x28 - IN Endpoint 0 status"]
    pub in_ep0_st: IN_EP0_ST,
    #[doc = "0x2c - IN Endpoint 1 status"]
    pub in_ep1_st: IN_EP1_ST,
    #[doc = "0x30 - IN Endpoint 2 status"]
    pub in_ep2_st: IN_EP2_ST,
    #[doc = "0x34 - IN Endpoint 3 status"]
    pub in_ep3_st: IN_EP3_ST,
    #[doc = "0x38 - OUT Endpoint 0 status"]
    pub out_ep0_st: OUT_EP0_ST,
    #[doc = "0x3c - OUT Endpoint 1 status"]
    pub out_ep1_st: OUT_EP1_ST,
    #[doc = "0x40 - OUT Endpoint 2 status"]
    pub out_ep2_st: OUT_EP2_ST,
    #[doc = "0x44 - MISC register"]
    pub misc_conf: MISC_CONF,
    #[doc = "0x48 - Power control"]
    pub mem_conf: MEM_CONF,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - Version control register"]
    pub date: DATE,
}
#[doc = "EP1 (rw) register accessor: Endpoint 1 FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep1`] module"]
pub type EP1 = crate::Reg<ep1::EP1_SPEC>;
#[doc = "Endpoint 1 FIFO register"]
pub mod ep1;
#[doc = "EP1_CONF (rw) register accessor: Endpoint 1 configure and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ep1_conf`] module"]
pub type EP1_CONF = crate::Reg<ep1_conf::EP1_CONF_SPEC>;
#[doc = "Endpoint 1 configure and status register"]
pub mod ep1_conf;
#[doc = "INT_RAW (rw) register accessor: Raw status interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw status interrupt"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CONF0 (rw) register accessor: Configure 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Configure 0 register"]
pub mod conf0;
#[doc = "TEST (rw) register accessor: USB Internal PHY test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`test`] module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "USB Internal PHY test register"]
pub mod test;
#[doc = "JFIFO_ST (rw) register accessor: USB-JTAG FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jfifo_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jfifo_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`jfifo_st`] module"]
pub type JFIFO_ST = crate::Reg<jfifo_st::JFIFO_ST_SPEC>;
#[doc = "USB-JTAG FIFO status"]
pub mod jfifo_st;
#[doc = "FRAM_NUM (r) register accessor: SOF frame number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fram_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fram_num`] module"]
pub type FRAM_NUM = crate::Reg<fram_num::FRAM_NUM_SPEC>;
#[doc = "SOF frame number"]
pub mod fram_num;
#[doc = "IN_EP0_ST (r) register accessor: IN Endpoint 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ep0_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_ep0_st`] module"]
pub type IN_EP0_ST = crate::Reg<in_ep0_st::IN_EP0_ST_SPEC>;
#[doc = "IN Endpoint 0 status"]
pub mod in_ep0_st;
#[doc = "IN_EP1_ST (r) register accessor: IN Endpoint 1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ep1_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_ep1_st`] module"]
pub type IN_EP1_ST = crate::Reg<in_ep1_st::IN_EP1_ST_SPEC>;
#[doc = "IN Endpoint 1 status"]
pub mod in_ep1_st;
#[doc = "IN_EP2_ST (r) register accessor: IN Endpoint 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ep2_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_ep2_st`] module"]
pub type IN_EP2_ST = crate::Reg<in_ep2_st::IN_EP2_ST_SPEC>;
#[doc = "IN Endpoint 2 status"]
pub mod in_ep2_st;
#[doc = "IN_EP3_ST (r) register accessor: IN Endpoint 3 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ep3_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_ep3_st`] module"]
pub type IN_EP3_ST = crate::Reg<in_ep3_st::IN_EP3_ST_SPEC>;
#[doc = "IN Endpoint 3 status"]
pub mod in_ep3_st;
#[doc = "OUT_EP0_ST (r) register accessor: OUT Endpoint 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ep0_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_ep0_st`] module"]
pub type OUT_EP0_ST = crate::Reg<out_ep0_st::OUT_EP0_ST_SPEC>;
#[doc = "OUT Endpoint 0 status"]
pub mod out_ep0_st;
#[doc = "OUT_EP1_ST (r) register accessor: OUT Endpoint 1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ep1_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_ep1_st`] module"]
pub type OUT_EP1_ST = crate::Reg<out_ep1_st::OUT_EP1_ST_SPEC>;
#[doc = "OUT Endpoint 1 status"]
pub mod out_ep1_st;
#[doc = "OUT_EP2_ST (r) register accessor: OUT Endpoint 2 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_ep2_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_ep2_st`] module"]
pub type OUT_EP2_ST = crate::Reg<out_ep2_st::OUT_EP2_ST_SPEC>;
#[doc = "OUT Endpoint 2 status"]
pub mod out_ep2_st;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "MEM_CONF (rw) register accessor: Power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "Power control"]
pub mod mem_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
