#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - a"]
    pub conf0: CONF0,
    #[doc = "0x04 - a"]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - a"]
    pub int_st: INT_ST,
    #[doc = "0x0c - a"]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - a"]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - a"]
    pub conf1: CONF1,
    #[doc = "0x18 - a"]
    pub state0: STATE0,
    #[doc = "0x1c - a"]
    pub state1: STATE1,
    #[doc = "0x20 - a"]
    pub escape_conf: ESCAPE_CONF,
    #[doc = "0x24 - a"]
    pub hung_conf: HUNG_CONF,
    #[doc = "0x28 - a"]
    pub ack_num: ACK_NUM,
    #[doc = "0x2c - a"]
    pub rx_head: RX_HEAD,
    #[doc = "0x30 - a"]
    pub quick_sent: QUICK_SENT,
    #[doc = "0x34 - a"]
    pub reg_q0_word0: REG_Q0_WORD0,
    #[doc = "0x38 - a"]
    pub reg_q0_word1: REG_Q0_WORD1,
    #[doc = "0x3c - a"]
    pub reg_q1_word0: REG_Q1_WORD0,
    #[doc = "0x40 - a"]
    pub reg_q1_word1: REG_Q1_WORD1,
    #[doc = "0x44 - a"]
    pub reg_q2_word0: REG_Q2_WORD0,
    #[doc = "0x48 - a"]
    pub reg_q2_word1: REG_Q2_WORD1,
    #[doc = "0x4c - a"]
    pub reg_q3_word0: REG_Q3_WORD0,
    #[doc = "0x50 - a"]
    pub reg_q3_word1: REG_Q3_WORD1,
    #[doc = "0x54 - a"]
    pub reg_q4_word0: REG_Q4_WORD0,
    #[doc = "0x58 - a"]
    pub reg_q4_word1: REG_Q4_WORD1,
    #[doc = "0x5c - a"]
    pub reg_q5_word0: REG_Q5_WORD0,
    #[doc = "0x60 - a"]
    pub reg_q5_word1: REG_Q5_WORD1,
    #[doc = "0x64 - a"]
    pub reg_q6_word0: REG_Q6_WORD0,
    #[doc = "0x68 - a"]
    pub reg_q6_word1: REG_Q6_WORD1,
    #[doc = "0x6c - a"]
    pub esc_conf0: ESC_CONF0,
    #[doc = "0x70 - a"]
    pub esc_conf1: ESC_CONF1,
    #[doc = "0x74 - a"]
    pub esc_conf2: ESC_CONF2,
    #[doc = "0x78 - a"]
    pub esc_conf3: ESC_CONF3,
    #[doc = "0x7c - a"]
    pub pkt_thres: PKT_THRES,
    #[doc = "0x80 - a"]
    pub date: DATE,
}
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "a"]
pub mod conf0;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "a"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "a"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "a"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "a"]
pub mod int_clr;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "a"]
pub mod conf1;
#[doc = "STATE0 (r) register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "a"]
pub mod state0;
#[doc = "STATE1 (r) register accessor: an alias for `Reg<STATE1_SPEC>`"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "a"]
pub mod state1;
#[doc = "ESCAPE_CONF (rw) register accessor: an alias for `Reg<ESCAPE_CONF_SPEC>`"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = "a"]
pub mod escape_conf;
#[doc = "HUNG_CONF (rw) register accessor: an alias for `Reg<HUNG_CONF_SPEC>`"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = "a"]
pub mod hung_conf;
#[doc = "ACK_NUM (rw) register accessor: an alias for `Reg<ACK_NUM_SPEC>`"]
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
#[doc = "a"]
pub mod ack_num;
#[doc = "RX_HEAD (r) register accessor: an alias for `Reg<RX_HEAD_SPEC>`"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = "a"]
pub mod rx_head;
#[doc = "QUICK_SENT (rw) register accessor: an alias for `Reg<QUICK_SENT_SPEC>`"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = "a"]
pub mod quick_sent;
#[doc = "REG_Q0_WORD0 (rw) register accessor: an alias for `Reg<REG_Q0_WORD0_SPEC>`"]
pub type REG_Q0_WORD0 = crate::Reg<reg_q0_word0::REG_Q0_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q0_word0;
#[doc = "REG_Q0_WORD1 (rw) register accessor: an alias for `Reg<REG_Q0_WORD1_SPEC>`"]
pub type REG_Q0_WORD1 = crate::Reg<reg_q0_word1::REG_Q0_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q0_word1;
#[doc = "REG_Q1_WORD0 (rw) register accessor: an alias for `Reg<REG_Q1_WORD0_SPEC>`"]
pub type REG_Q1_WORD0 = crate::Reg<reg_q1_word0::REG_Q1_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q1_word0;
#[doc = "REG_Q1_WORD1 (rw) register accessor: an alias for `Reg<REG_Q1_WORD1_SPEC>`"]
pub type REG_Q1_WORD1 = crate::Reg<reg_q1_word1::REG_Q1_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q1_word1;
#[doc = "REG_Q2_WORD0 (rw) register accessor: an alias for `Reg<REG_Q2_WORD0_SPEC>`"]
pub type REG_Q2_WORD0 = crate::Reg<reg_q2_word0::REG_Q2_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q2_word0;
#[doc = "REG_Q2_WORD1 (rw) register accessor: an alias for `Reg<REG_Q2_WORD1_SPEC>`"]
pub type REG_Q2_WORD1 = crate::Reg<reg_q2_word1::REG_Q2_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q2_word1;
#[doc = "REG_Q3_WORD0 (rw) register accessor: an alias for `Reg<REG_Q3_WORD0_SPEC>`"]
pub type REG_Q3_WORD0 = crate::Reg<reg_q3_word0::REG_Q3_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q3_word0;
#[doc = "REG_Q3_WORD1 (rw) register accessor: an alias for `Reg<REG_Q3_WORD1_SPEC>`"]
pub type REG_Q3_WORD1 = crate::Reg<reg_q3_word1::REG_Q3_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q3_word1;
#[doc = "REG_Q4_WORD0 (rw) register accessor: an alias for `Reg<REG_Q4_WORD0_SPEC>`"]
pub type REG_Q4_WORD0 = crate::Reg<reg_q4_word0::REG_Q4_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q4_word0;
#[doc = "REG_Q4_WORD1 (rw) register accessor: an alias for `Reg<REG_Q4_WORD1_SPEC>`"]
pub type REG_Q4_WORD1 = crate::Reg<reg_q4_word1::REG_Q4_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q4_word1;
#[doc = "REG_Q5_WORD0 (rw) register accessor: an alias for `Reg<REG_Q5_WORD0_SPEC>`"]
pub type REG_Q5_WORD0 = crate::Reg<reg_q5_word0::REG_Q5_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q5_word0;
#[doc = "REG_Q5_WORD1 (rw) register accessor: an alias for `Reg<REG_Q5_WORD1_SPEC>`"]
pub type REG_Q5_WORD1 = crate::Reg<reg_q5_word1::REG_Q5_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q5_word1;
#[doc = "REG_Q6_WORD0 (rw) register accessor: an alias for `Reg<REG_Q6_WORD0_SPEC>`"]
pub type REG_Q6_WORD0 = crate::Reg<reg_q6_word0::REG_Q6_WORD0_SPEC>;
#[doc = "a"]
pub mod reg_q6_word0;
#[doc = "REG_Q6_WORD1 (rw) register accessor: an alias for `Reg<REG_Q6_WORD1_SPEC>`"]
pub type REG_Q6_WORD1 = crate::Reg<reg_q6_word1::REG_Q6_WORD1_SPEC>;
#[doc = "a"]
pub mod reg_q6_word1;
#[doc = "ESC_CONF0 (rw) register accessor: an alias for `Reg<ESC_CONF0_SPEC>`"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = "a"]
pub mod esc_conf0;
#[doc = "ESC_CONF1 (rw) register accessor: an alias for `Reg<ESC_CONF1_SPEC>`"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = "a"]
pub mod esc_conf1;
#[doc = "ESC_CONF2 (rw) register accessor: an alias for `Reg<ESC_CONF2_SPEC>`"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = "a"]
pub mod esc_conf2;
#[doc = "ESC_CONF3 (rw) register accessor: an alias for `Reg<ESC_CONF3_SPEC>`"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = "a"]
pub mod esc_conf3;
#[doc = "PKT_THRES (rw) register accessor: an alias for `Reg<PKT_THRES_SPEC>`"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = "a"]
pub mod pkt_thres;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "a"]
pub mod date;
