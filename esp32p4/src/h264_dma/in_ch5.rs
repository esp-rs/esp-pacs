#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_CH5, containing IN_CONF0_CH5, IN_CONF1_CH5, IN_CONF2_CH5, IN_CONF3_CH5, IN_INT_RAW_CH5, IN_INT_ENA_CH5, IN_INT_ST_CH5, IN_INT_CLR_CH5, INFIFO_STATUS_CH5, IN_POP_CH5, IN_STATE_CH5, IN_ARB_CH5, IN_FIFO_CNT_CH5, IN_POP_DATA_CNT_CH5, IN_XADDR_CH5, IN_BUF_HB_RCV_CH5"]
pub struct IN_CH5 {
    conf0: CONF0,
    conf1: CONF1,
    conf2: CONF2,
    conf3: CONF3,
    int_raw: INT_RAW,
    int_ena: INT_ENA,
    int_st: INT_ST,
    int_clr: INT_CLR,
    fifo_status: FIFO_STATUS,
    pop: POP,
    state: STATE,
    _reserved11: [u8; 0x14],
    arb: ARB,
    _reserved12: [u8; 0x3c],
    fifo_cnt: FIFO_CNT,
    pop_data_cnt: POP_DATA_CNT,
    xaddr: XADDR,
    buf_hb_rcv: BUF_HB_RCV,
}
impl IN_CH5 {
    #[doc = "0x00 - RX CH5 config0 register"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - RX CH5 config1 register"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x08 - RX CH5 config2 register"]
    #[inline(always)]
    pub const fn conf2(&self) -> &CONF2 {
        &self.conf2
    }
    #[doc = "0x0c - RX CH5 config3 register"]
    #[inline(always)]
    pub const fn conf3(&self) -> &CONF3 {
        &self.conf3
    }
    #[doc = "0x10 - RX CH5 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x14 - RX CH5 interrupt ena register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - RX CH5 interrupt st register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x1c - RX CH5 interrupt clr register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x20 - RX CH5 INFIFO status register"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FIFO_STATUS {
        &self.fifo_status
    }
    #[doc = "0x24 - RX CH5 INFIFO pop register"]
    #[inline(always)]
    pub const fn pop(&self) -> &POP {
        &self.pop
    }
    #[doc = "0x28 - RX CH5 state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x40 - RX CH5 arb register"]
    #[inline(always)]
    pub const fn arb(&self) -> &ARB {
        &self.arb
    }
    #[doc = "0x80 - rx CH5 fifo cnt register"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> &FIFO_CNT {
        &self.fifo_cnt
    }
    #[doc = "0x84 - rx CH5 pop data cnt register"]
    #[inline(always)]
    pub const fn pop_data_cnt(&self) -> &POP_DATA_CNT {
        &self.pop_data_cnt
    }
    #[doc = "0x88 - rx CH5 xaddr register"]
    #[inline(always)]
    pub const fn xaddr(&self) -> &XADDR {
        &self.xaddr
    }
    #[doc = "0x8c - rx CH5 buf len hb rcv register"]
    #[inline(always)]
    pub const fn buf_hb_rcv(&self) -> &BUF_HB_RCV {
        &self.buf_hb_rcv
    }
}
#[doc = "CONF0 (rw) register accessor: RX CH5 config0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "RX CH5 config0 register"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: RX CH5 config1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "RX CH5 config1 register"]
pub mod conf1;
#[doc = "CONF2 (rw) register accessor: RX CH5 config2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf2`] module"]
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
#[doc = "RX CH5 config2 register"]
pub mod conf2;
#[doc = "CONF3 (rw) register accessor: RX CH5 config3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf3`] module"]
pub type CONF3 = crate::Reg<conf3::CONF3_SPEC>;
#[doc = "RX CH5 config3 register"]
pub mod conf3;
#[doc = "INT_RAW (rw) register accessor: RX CH5 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RX CH5 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ENA (rw) register accessor: RX CH5 interrupt ena register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RX CH5 interrupt ena register"]
pub mod int_ena;
#[doc = "INT_ST (r) register accessor: RX CH5 interrupt st register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RX CH5 interrupt st register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: RX CH5 interrupt clr register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RX CH5 interrupt clr register"]
pub mod int_clr;
#[doc = "FIFO_STATUS (r) register accessor: RX CH5 INFIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`] module"]
pub type FIFO_STATUS = crate::Reg<fifo_status::FIFO_STATUS_SPEC>;
#[doc = "RX CH5 INFIFO status register"]
pub mod fifo_status;
#[doc = "POP (rw) register accessor: RX CH5 INFIFO pop register\n\nYou can [`read`](crate::Reg::read) this register and get [`pop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pop`] module"]
pub type POP = crate::Reg<pop::POP_SPEC>;
#[doc = "RX CH5 INFIFO pop register"]
pub mod pop;
#[doc = "STATE (r) register accessor: RX CH5 state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "RX CH5 state register"]
pub mod state;
#[doc = "ARB (rw) register accessor: RX CH5 arb register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb`] module"]
pub type ARB = crate::Reg<arb::ARB_SPEC>;
#[doc = "RX CH5 arb register"]
pub mod arb;
#[doc = "FIFO_CNT (r) register accessor: rx CH5 fifo cnt register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_cnt`] module"]
pub type FIFO_CNT = crate::Reg<fifo_cnt::FIFO_CNT_SPEC>;
#[doc = "rx CH5 fifo cnt register"]
pub mod fifo_cnt;
#[doc = "POP_DATA_CNT (r) register accessor: rx CH5 pop data cnt register\n\nYou can [`read`](crate::Reg::read) this register and get [`pop_data_cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pop_data_cnt`] module"]
pub type POP_DATA_CNT = crate::Reg<pop_data_cnt::POP_DATA_CNT_SPEC>;
#[doc = "rx CH5 pop data cnt register"]
pub mod pop_data_cnt;
#[doc = "XADDR (r) register accessor: rx CH5 xaddr register\n\nYou can [`read`](crate::Reg::read) this register and get [`xaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xaddr`] module"]
pub type XADDR = crate::Reg<xaddr::XADDR_SPEC>;
#[doc = "rx CH5 xaddr register"]
pub mod xaddr;
#[doc = "BUF_HB_RCV (r) register accessor: rx CH5 buf len hb rcv register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_hb_rcv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_hb_rcv`] module"]
pub type BUF_HB_RCV = crate::Reg<buf_hb_rcv::BUF_HB_RCV_SPEC>;
#[doc = "rx CH5 buf len hb rcv register"]
pub mod buf_hb_rcv;
