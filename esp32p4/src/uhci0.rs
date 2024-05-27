#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    reg_q: [REG_Q; 7],
    esc_conf: [ESC_CONF; 4],
    pkt_thres: PKT_THRES,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - UHCI Configuration Register0
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    ///0x04 - UHCI Interrupt Raw Register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x08 - UHCI Interrupt Status Register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x0c - UHCI Interrupt Enable Register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x10 - UHCI Interrupt Clear Register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x14 - UHCI Configuration Register1
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    ///0x18 - UHCI Receive Status Register
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    ///0x1c - UHCI Transmit Status Register
    #[inline(always)]
    pub const fn state1(&self) -> &STATE1 {
        &self.state1
    }
    ///0x20 - UHCI Escapes Configuration Register0
    #[inline(always)]
    pub const fn escape_conf(&self) -> &ESCAPE_CONF {
        &self.escape_conf
    }
    ///0x24 - UHCI Hung Configuration Register0
    #[inline(always)]
    pub const fn hung_conf(&self) -> &HUNG_CONF {
        &self.hung_conf
    }
    ///0x28 - UHCI Ack Value Configuration Register0
    #[inline(always)]
    pub const fn ack_num(&self) -> &ACK_NUM {
        &self.ack_num
    }
    ///0x2c - UHCI Head Register
    #[inline(always)]
    pub const fn rx_head(&self) -> &RX_HEAD {
        &self.rx_head
    }
    ///0x30 - UCHI Quick send Register
    #[inline(always)]
    pub const fn quick_sent(&self) -> &QUICK_SENT {
        &self.quick_sent
    }
    ///0x34..0x6c - Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1
    #[inline(always)]
    pub const fn reg_q(&self, n: usize) -> &REG_Q {
        &self.reg_q[n]
    }
    ///Iterator for array of:
    ///0x34..0x6c - Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1
    #[inline(always)]
    pub fn reg_q_iter(&self) -> impl Iterator<Item = &REG_Q> {
        self.reg_q.iter()
    }
    ///0x6c..0x7c - UHCI Escapes Sequence Configuration Register%s
    #[inline(always)]
    pub const fn esc_conf(&self, n: usize) -> &ESC_CONF {
        &self.esc_conf[n]
    }
    ///Iterator for array of:
    ///0x6c..0x7c - UHCI Escapes Sequence Configuration Register%s
    #[inline(always)]
    pub fn esc_conf_iter(&self) -> impl Iterator<Item = &ESC_CONF> {
        self.esc_conf.iter()
    }
    ///0x7c - UCHI Packet Length Configuration Register
    #[inline(always)]
    pub const fn pkt_thres(&self) -> &PKT_THRES {
        &self.pkt_thres
    }
    ///0x80 - UHCI Version Register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CONF0 (rw) register accessor: UHCI Configuration Register0

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf0`] module*/
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
///UHCI Configuration Register0
pub mod conf0;
/**INT_RAW (rw) register accessor: UHCI Interrupt Raw Register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///UHCI Interrupt Raw Register
pub mod int_raw;
/**INT_ST (r) register accessor: UHCI Interrupt Status Register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///UHCI Interrupt Status Register
pub mod int_st;
/**INT_ENA (rw) register accessor: UHCI Interrupt Enable Register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///UHCI Interrupt Enable Register
pub mod int_ena;
/**INT_CLR (w) register accessor: UHCI Interrupt Clear Register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///UHCI Interrupt Clear Register
pub mod int_clr;
/**CONF1 (rw) register accessor: UHCI Configuration Register1

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf1`] module*/
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
///UHCI Configuration Register1
pub mod conf1;
/**STATE0 (r) register accessor: UHCI Receive Status Register

You can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state0`] module*/
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
///UHCI Receive Status Register
pub mod state0;
/**STATE1 (r) register accessor: UHCI Transmit Status Register

You can [`read`](crate::generic::Reg::read) this register and get [`state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state1`] module*/
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
///UHCI Transmit Status Register
pub mod state1;
/**ESCAPE_CONF (rw) register accessor: UHCI Escapes Configuration Register0

You can [`read`](crate::generic::Reg::read) this register and get [`escape_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escape_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@escape_conf`] module*/
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
///UHCI Escapes Configuration Register0
pub mod escape_conf;
/**HUNG_CONF (rw) register accessor: UHCI Hung Configuration Register0

You can [`read`](crate::generic::Reg::read) this register and get [`hung_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hung_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hung_conf`] module*/
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
///UHCI Hung Configuration Register0
pub mod hung_conf;
/**ACK_NUM (rw) register accessor: UHCI Ack Value Configuration Register0

You can [`read`](crate::generic::Reg::read) this register and get [`ack_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ack_num`] module*/
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
///UHCI Ack Value Configuration Register0
pub mod ack_num;
/**RX_HEAD (r) register accessor: UHCI Head Register

You can [`read`](crate::generic::Reg::read) this register and get [`rx_head::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_head`] module*/
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
///UHCI Head Register
pub mod rx_head;
/**QUICK_SENT (rw) register accessor: UCHI Quick send Register

You can [`read`](crate::generic::Reg::read) this register and get [`quick_sent::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quick_sent::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@quick_sent`] module*/
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
///UCHI Quick send Register
pub mod quick_sent;
///Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1
pub use self::reg_q::REG_Q;
///Cluster
///Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1
pub mod reg_q;
/**ESC_CONF (rw) register accessor: UHCI Escapes Sequence Configuration Register%s

You can [`read`](crate::generic::Reg::read) this register and get [`esc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@esc_conf`] module*/
pub type ESC_CONF = crate::Reg<esc_conf::ESC_CONF_SPEC>;
///UHCI Escapes Sequence Configuration Register%s
pub mod esc_conf;
/**PKT_THRES (rw) register accessor: UCHI Packet Length Configuration Register

You can [`read`](crate::generic::Reg::read) this register and get [`pkt_thres::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkt_thres::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pkt_thres`] module*/
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
///UCHI Packet Length Configuration Register
pub mod pkt_thres;
/**DATE (rw) register accessor: UHCI Version Register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///UHCI Version Register
pub mod date;
