#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf0: CONF0,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    conf1: CONF1,
    state0: STATE0,
    state1: STATE1,
    escape_conf: ESCAPE_CONF,
    hung_conf: HUNG_CONF,
    ack_num: ACK_NUM,
    rx_head: RX_HEAD,
    quick_sent: QUICK_SENT,
    reg_q0_word0: REG_Q0_WORD0,
    reg_q0_word1: REG_Q0_WORD1,
    reg_q1_word0: REG_Q1_WORD0,
    reg_q1_word1: REG_Q1_WORD1,
    reg_q2_word0: REG_Q2_WORD0,
    reg_q2_word1: REG_Q2_WORD1,
    reg_q3_word0: REG_Q3_WORD0,
    reg_q3_word1: REG_Q3_WORD1,
    reg_q4_word0: REG_Q4_WORD0,
    reg_q4_word1: REG_Q4_WORD1,
    reg_q5_word0: REG_Q5_WORD0,
    reg_q5_word1: REG_Q5_WORD1,
    reg_q6_word0: REG_Q6_WORD0,
    reg_q6_word1: REG_Q6_WORD1,
    esc_conf0: ESC_CONF0,
    esc_conf1: ESC_CONF1,
    esc_conf2: ESC_CONF2,
    esc_conf3: ESC_CONF3,
    pkt_thres: PKT_THRES,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - UHCI configuration register"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x0c - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x10 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - UHCI configuration register"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x18 - UHCI receive status"]
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    #[doc = "0x1c - UHCI transmit status"]
    #[inline(always)]
    pub const fn state1(&self) -> &STATE1 {
        &self.state1
    }
    #[doc = "0x20 - Escape character configuration"]
    #[inline(always)]
    pub const fn escape_conf(&self) -> &ESCAPE_CONF {
        &self.escape_conf
    }
    #[doc = "0x24 - Timeout configuration"]
    #[inline(always)]
    pub const fn hung_conf(&self) -> &HUNG_CONF {
        &self.hung_conf
    }
    #[doc = "0x28 - UHCI ACK number configuration"]
    #[inline(always)]
    pub const fn ack_num(&self) -> &ACK_NUM {
        &self.ack_num
    }
    #[doc = "0x2c - UHCI packet header register"]
    #[inline(always)]
    pub const fn rx_head(&self) -> &RX_HEAD {
        &self.rx_head
    }
    #[doc = "0x30 - UHCI quick send configuration register"]
    #[inline(always)]
    pub const fn quick_sent(&self) -> &QUICK_SENT {
        &self.quick_sent
    }
    #[doc = "0x34 - Q0 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q0_word0(&self) -> &REG_Q0_WORD0 {
        &self.reg_q0_word0
    }
    #[doc = "0x38 - Q0 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q0_word1(&self) -> &REG_Q0_WORD1 {
        &self.reg_q0_word1
    }
    #[doc = "0x3c - Q1 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q1_word0(&self) -> &REG_Q1_WORD0 {
        &self.reg_q1_word0
    }
    #[doc = "0x40 - Q1 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q1_word1(&self) -> &REG_Q1_WORD1 {
        &self.reg_q1_word1
    }
    #[doc = "0x44 - Q2 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q2_word0(&self) -> &REG_Q2_WORD0 {
        &self.reg_q2_word0
    }
    #[doc = "0x48 - Q2 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q2_word1(&self) -> &REG_Q2_WORD1 {
        &self.reg_q2_word1
    }
    #[doc = "0x4c - Q3 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q3_word0(&self) -> &REG_Q3_WORD0 {
        &self.reg_q3_word0
    }
    #[doc = "0x50 - Q3 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q3_word1(&self) -> &REG_Q3_WORD1 {
        &self.reg_q3_word1
    }
    #[doc = "0x54 - Q4 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q4_word0(&self) -> &REG_Q4_WORD0 {
        &self.reg_q4_word0
    }
    #[doc = "0x58 - Q4 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q4_word1(&self) -> &REG_Q4_WORD1 {
        &self.reg_q4_word1
    }
    #[doc = "0x5c - Q5 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q5_word0(&self) -> &REG_Q5_WORD0 {
        &self.reg_q5_word0
    }
    #[doc = "0x60 - Q5 WORD1 quick send register"]
    #[inline(always)]
    pub const fn reg_q5_word1(&self) -> &REG_Q5_WORD1 {
        &self.reg_q5_word1
    }
    #[doc = "0x64 - Q6 WORD0 quick send register"]
    #[inline(always)]
    pub const fn reg_q6_word0(&self) -> &REG_Q6_WORD0 {
        &self.reg_q6_word0
    }
    #[doc = "0x68 - Q6 WORD1 quick register"]
    #[inline(always)]
    pub const fn reg_q6_word1(&self) -> &REG_Q6_WORD1 {
        &self.reg_q6_word1
    }
    #[doc = "0x6c - Escape sequence configuration register 0"]
    #[inline(always)]
    pub const fn esc_conf0(&self) -> &ESC_CONF0 {
        &self.esc_conf0
    }
    #[doc = "0x70 - Escape sequence configuration register 1"]
    #[inline(always)]
    pub const fn esc_conf1(&self) -> &ESC_CONF1 {
        &self.esc_conf1
    }
    #[doc = "0x74 - Escape sequence configuration register 2"]
    #[inline(always)]
    pub const fn esc_conf2(&self) -> &ESC_CONF2 {
        &self.esc_conf2
    }
    #[doc = "0x78 - Escape sequence configuration register 3"]
    #[inline(always)]
    pub const fn esc_conf3(&self) -> &ESC_CONF3 {
        &self.esc_conf3
    }
    #[doc = "0x7c - Configuration register for packet length"]
    #[inline(always)]
    pub const fn pkt_thres(&self) -> &PKT_THRES {
        &self.pkt_thres
    }
    #[doc = "0x80 - UHCI version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF0 (rw) register accessor: UHCI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf0;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CONF1 (rw) register accessor: UHCI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "UHCI configuration register"]
pub mod conf1;
#[doc = "STATE0 (r) register accessor: UHCI receive status\n\nYou can [`read`](crate::Reg::read) this register and get [`state0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state0`] module"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "UHCI receive status"]
pub mod state0;
#[doc = "STATE1 (r) register accessor: UHCI transmit status\n\nYou can [`read`](crate::Reg::read) this register and get [`state1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state1`] module"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "UHCI transmit status"]
pub mod state1;
#[doc = "ESCAPE_CONF (rw) register accessor: Escape character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`escape_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`escape_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@escape_conf`] module"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = "Escape character configuration"]
pub mod escape_conf;
#[doc = "HUNG_CONF (rw) register accessor: Timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hung_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hung_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hung_conf`] module"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = "Timeout configuration"]
pub mod hung_conf;
#[doc = "ACK_NUM (rw) register accessor: UHCI ACK number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ack_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ack_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_num`] module"]
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
#[doc = "UHCI ACK number configuration"]
pub mod ack_num;
#[doc = "RX_HEAD (r) register accessor: UHCI packet header register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_head::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_head`] module"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = "UHCI packet header register"]
pub mod rx_head;
#[doc = "QUICK_SENT (rw) register accessor: UHCI quick send configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`quick_sent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quick_sent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quick_sent`] module"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = "UHCI quick send configuration register"]
pub mod quick_sent;
#[doc = "REG_Q0_WORD0 (rw) register accessor: Q0 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q0_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q0_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q0_word0`] module"]
pub type REG_Q0_WORD0 = crate::Reg<reg_q0_word0::REG_Q0_WORD0_SPEC>;
#[doc = "Q0 WORD0 quick send register"]
pub mod reg_q0_word0;
#[doc = "REG_Q0_WORD1 (rw) register accessor: Q0 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q0_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q0_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q0_word1`] module"]
pub type REG_Q0_WORD1 = crate::Reg<reg_q0_word1::REG_Q0_WORD1_SPEC>;
#[doc = "Q0 WORD1 quick send register"]
pub mod reg_q0_word1;
#[doc = "REG_Q1_WORD0 (rw) register accessor: Q1 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q1_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q1_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q1_word0`] module"]
pub type REG_Q1_WORD0 = crate::Reg<reg_q1_word0::REG_Q1_WORD0_SPEC>;
#[doc = "Q1 WORD0 quick send register"]
pub mod reg_q1_word0;
#[doc = "REG_Q1_WORD1 (rw) register accessor: Q1 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q1_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q1_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q1_word1`] module"]
pub type REG_Q1_WORD1 = crate::Reg<reg_q1_word1::REG_Q1_WORD1_SPEC>;
#[doc = "Q1 WORD1 quick send register"]
pub mod reg_q1_word1;
#[doc = "REG_Q2_WORD0 (rw) register accessor: Q2 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q2_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q2_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q2_word0`] module"]
pub type REG_Q2_WORD0 = crate::Reg<reg_q2_word0::REG_Q2_WORD0_SPEC>;
#[doc = "Q2 WORD0 quick send register"]
pub mod reg_q2_word0;
#[doc = "REG_Q2_WORD1 (rw) register accessor: Q2 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q2_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q2_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q2_word1`] module"]
pub type REG_Q2_WORD1 = crate::Reg<reg_q2_word1::REG_Q2_WORD1_SPEC>;
#[doc = "Q2 WORD1 quick send register"]
pub mod reg_q2_word1;
#[doc = "REG_Q3_WORD0 (rw) register accessor: Q3 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q3_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q3_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q3_word0`] module"]
pub type REG_Q3_WORD0 = crate::Reg<reg_q3_word0::REG_Q3_WORD0_SPEC>;
#[doc = "Q3 WORD0 quick send register"]
pub mod reg_q3_word0;
#[doc = "REG_Q3_WORD1 (rw) register accessor: Q3 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q3_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q3_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q3_word1`] module"]
pub type REG_Q3_WORD1 = crate::Reg<reg_q3_word1::REG_Q3_WORD1_SPEC>;
#[doc = "Q3 WORD1 quick send register"]
pub mod reg_q3_word1;
#[doc = "REG_Q4_WORD0 (rw) register accessor: Q4 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q4_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q4_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q4_word0`] module"]
pub type REG_Q4_WORD0 = crate::Reg<reg_q4_word0::REG_Q4_WORD0_SPEC>;
#[doc = "Q4 WORD0 quick send register"]
pub mod reg_q4_word0;
#[doc = "REG_Q4_WORD1 (rw) register accessor: Q4 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q4_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q4_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q4_word1`] module"]
pub type REG_Q4_WORD1 = crate::Reg<reg_q4_word1::REG_Q4_WORD1_SPEC>;
#[doc = "Q4 WORD1 quick send register"]
pub mod reg_q4_word1;
#[doc = "REG_Q5_WORD0 (rw) register accessor: Q5 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q5_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q5_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q5_word0`] module"]
pub type REG_Q5_WORD0 = crate::Reg<reg_q5_word0::REG_Q5_WORD0_SPEC>;
#[doc = "Q5 WORD0 quick send register"]
pub mod reg_q5_word0;
#[doc = "REG_Q5_WORD1 (rw) register accessor: Q5 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q5_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q5_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q5_word1`] module"]
pub type REG_Q5_WORD1 = crate::Reg<reg_q5_word1::REG_Q5_WORD1_SPEC>;
#[doc = "Q5 WORD1 quick send register"]
pub mod reg_q5_word1;
#[doc = "REG_Q6_WORD0 (rw) register accessor: Q6 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q6_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q6_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q6_word0`] module"]
pub type REG_Q6_WORD0 = crate::Reg<reg_q6_word0::REG_Q6_WORD0_SPEC>;
#[doc = "Q6 WORD0 quick send register"]
pub mod reg_q6_word0;
#[doc = "REG_Q6_WORD1 (rw) register accessor: Q6 WORD1 quick register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q6_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q6_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_q6_word1`] module"]
pub type REG_Q6_WORD1 = crate::Reg<reg_q6_word1::REG_Q6_WORD1_SPEC>;
#[doc = "Q6 WORD1 quick register"]
pub mod reg_q6_word1;
#[doc = "ESC_CONF0 (rw) register accessor: Escape sequence configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf0`] module"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = "Escape sequence configuration register 0"]
pub mod esc_conf0;
#[doc = "ESC_CONF1 (rw) register accessor: Escape sequence configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf1`] module"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = "Escape sequence configuration register 1"]
pub mod esc_conf1;
#[doc = "ESC_CONF2 (rw) register accessor: Escape sequence configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf2`] module"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = "Escape sequence configuration register 2"]
pub mod esc_conf2;
#[doc = "ESC_CONF3 (rw) register accessor: Escape sequence configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`esc_conf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esc_conf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esc_conf3`] module"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = "Escape sequence configuration register 3"]
pub mod esc_conf3;
#[doc = "PKT_THRES (rw) register accessor: Configuration register for packet length\n\nYou can [`read`](crate::Reg::read) this register and get [`pkt_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkt_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkt_thres`] module"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = "Configuration register for packet length"]
pub mod pkt_thres;
pub use crate::aes::date;
pub use crate::aes::DATE;
