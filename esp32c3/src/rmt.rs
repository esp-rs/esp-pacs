#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - RMT_CH0DATA_REG."]
    pub ch0data: CH0DATA,
    #[doc = "0x04 - RMT_CH1DATA_REG."]
    pub ch1data: CH1DATA,
    #[doc = "0x08 - RMT_CH2DATA_REG."]
    pub ch2data: CH2DATA,
    #[doc = "0x0c - RMT_CH3DATA_REG."]
    pub ch3data: CH3DATA,
    #[doc = "0x10..0x18 - RMT_CH%sCONF%s_REG."]
    pub ch_tx_conf0: [CH_TX_CONF0; 2],
    #[doc = "0x18 - RMT_CH2CONF0_REG."]
    pub ch2_rx_conf0: CH_RX_CONF0,
    #[doc = "0x1c - RMT_CH2CONF1_REG."]
    pub ch2conf1: CH2CONF1,
    #[doc = "0x20 - RMT_CH2CONF0_REG."]
    pub ch3_rx_conf0: CH_RX_CONF0,
    #[doc = "0x24 - RMT_CH3CONF1_REG."]
    pub ch3conf1: CH3CONF1,
    #[doc = "0x28 - RMT_CH0STATUS_REG."]
    pub ch0status: CH0STATUS,
    #[doc = "0x2c - RMT_CH1STATUS_REG."]
    pub ch1status: CH1STATUS,
    #[doc = "0x30 - RMT_CH2STATUS_REG."]
    pub ch2status: CH2STATUS,
    #[doc = "0x34 - RMT_CH3STATUS_REG."]
    pub ch3status: CH3STATUS,
    #[doc = "0x38 - RMT_INT_RAW_REG."]
    pub int_raw: INT_RAW,
    #[doc = "0x3c - RMT_INT_ST_REG."]
    pub int_st: INT_ST,
    #[doc = "0x40 - RMT_INT_ENA_REG."]
    pub int_ena: INT_ENA,
    #[doc = "0x44 - RMT_INT_CLR_REG."]
    pub int_clr: INT_CLR,
    #[doc = "0x48 - RMT_CH0CARRIER_DUTY_REG."]
    pub ch0carrier_duty: CH0CARRIER_DUTY,
    #[doc = "0x4c - RMT_CH1CARRIER_DUTY_REG."]
    pub ch1carrier_duty: CH1CARRIER_DUTY,
    #[doc = "0x50 - RMT_CH2_RX_CARRIER_RM_REG."]
    pub ch2_rx_carrier_rm: CH2_RX_CARRIER_RM,
    #[doc = "0x54 - RMT_CH3_RX_CARRIER_RM_REG."]
    pub ch3_rx_carrier_rm: CH3_RX_CARRIER_RM,
    #[doc = "0x58..0x60 - RMT_CH%s_TX_LIM_REG."]
    pub ch_tx_lim: [CH_TX_LIM; 2],
    #[doc = "0x60..0x68 - RMT_CH2_RX_LIM_REG."]
    pub ch_rx_lim: [CH_RX_LIM; 2],
    #[doc = "0x68 - RMT_SYS_CONF_REG."]
    pub sys_conf: SYS_CONF,
    #[doc = "0x6c - RMT_TX_SIM_REG."]
    pub tx_sim: TX_SIM,
    #[doc = "0x70 - RMT_REF_CNT_RST_REG."]
    pub ref_cnt_rst: REF_CNT_RST,
    _reserved26: [u8; 0x58],
    #[doc = "0xcc - RMT_DATE_REG."]
    pub date: DATE,
}
impl RegisterBlock {
    #[doc = "0x60 - RMT_CH2_RX_LIM_REG."]
    #[inline(always)]
    pub fn ch2_rx_lim(&self) -> &CH_RX_LIM {
        &self.ch_rx_lim[0]
    }
    #[doc = "0x64 - RMT_CH2_RX_LIM_REG."]
    #[inline(always)]
    pub fn ch3_rx_lim(&self) -> &CH_RX_LIM {
        &self.ch_rx_lim[1]
    }
}
#[doc = "CH0DATA (rw) register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "RMT_CH0DATA_REG."]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "RMT_CH1DATA_REG."]
pub mod ch1data;
#[doc = "CH2DATA (rw) register accessor: an alias for `Reg<CH2DATA_SPEC>`"]
pub type CH2DATA = crate::Reg<ch2data::CH2DATA_SPEC>;
#[doc = "RMT_CH2DATA_REG."]
pub mod ch2data;
#[doc = "CH3DATA (rw) register accessor: an alias for `Reg<CH3DATA_SPEC>`"]
pub type CH3DATA = crate::Reg<ch3data::CH3DATA_SPEC>;
#[doc = "RMT_CH3DATA_REG."]
pub mod ch3data;
#[doc = "CH_TX_CONF0 (rw) register accessor: an alias for `Reg<CH_TX_CONF0_SPEC>`"]
pub type CH_TX_CONF0 = crate::Reg<ch_tx_conf0::CH_TX_CONF0_SPEC>;
#[doc = "RMT_CH%sCONF%s_REG."]
pub mod ch_tx_conf0;
#[doc = "CH_RX_CONF0 (rw) register accessor: an alias for `Reg<CH_RX_CONF0_SPEC>`"]
pub type CH_RX_CONF0 = crate::Reg<ch_rx_conf0::CH_RX_CONF0_SPEC>;
#[doc = "RMT_CH2CONF0_REG."]
pub mod ch_rx_conf0;
#[doc = "CH2CONF1 (rw) register accessor: an alias for `Reg<CH2CONF1_SPEC>`"]
pub type CH2CONF1 = crate::Reg<ch2conf1::CH2CONF1_SPEC>;
#[doc = "RMT_CH2CONF1_REG."]
pub mod ch2conf1;
#[doc = "CH3CONF1 (rw) register accessor: an alias for `Reg<CH3CONF1_SPEC>`"]
pub type CH3CONF1 = crate::Reg<ch3conf1::CH3CONF1_SPEC>;
#[doc = "RMT_CH3CONF1_REG."]
pub mod ch3conf1;
#[doc = "CH0STATUS (r) register accessor: an alias for `Reg<CH0STATUS_SPEC>`"]
pub type CH0STATUS = crate::Reg<ch0status::CH0STATUS_SPEC>;
#[doc = "RMT_CH0STATUS_REG."]
pub mod ch0status;
#[doc = "CH1STATUS (r) register accessor: an alias for `Reg<CH1STATUS_SPEC>`"]
pub type CH1STATUS = crate::Reg<ch1status::CH1STATUS_SPEC>;
#[doc = "RMT_CH1STATUS_REG."]
pub mod ch1status;
#[doc = "CH2STATUS (r) register accessor: an alias for `Reg<CH2STATUS_SPEC>`"]
pub type CH2STATUS = crate::Reg<ch2status::CH2STATUS_SPEC>;
#[doc = "RMT_CH2STATUS_REG."]
pub mod ch2status;
#[doc = "CH3STATUS (r) register accessor: an alias for `Reg<CH3STATUS_SPEC>`"]
pub type CH3STATUS = crate::Reg<ch3status::CH3STATUS_SPEC>;
#[doc = "RMT_CH3STATUS_REG."]
pub mod ch3status;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RMT_INT_RAW_REG."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RMT_INT_ST_REG."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RMT_INT_ENA_REG."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RMT_INT_CLR_REG."]
pub mod int_clr;
#[doc = "CH0CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH0CARRIER_DUTY_SPEC>`"]
pub type CH0CARRIER_DUTY = crate::Reg<ch0carrier_duty::CH0CARRIER_DUTY_SPEC>;
#[doc = "RMT_CH0CARRIER_DUTY_REG."]
pub mod ch0carrier_duty;
#[doc = "CH1CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH1CARRIER_DUTY_SPEC>`"]
pub type CH1CARRIER_DUTY = crate::Reg<ch1carrier_duty::CH1CARRIER_DUTY_SPEC>;
#[doc = "RMT_CH1CARRIER_DUTY_REG."]
pub mod ch1carrier_duty;
#[doc = "CH2_RX_CARRIER_RM (rw) register accessor: an alias for `Reg<CH2_RX_CARRIER_RM_SPEC>`"]
pub type CH2_RX_CARRIER_RM = crate::Reg<ch2_rx_carrier_rm::CH2_RX_CARRIER_RM_SPEC>;
#[doc = "RMT_CH2_RX_CARRIER_RM_REG."]
pub mod ch2_rx_carrier_rm;
#[doc = "CH3_RX_CARRIER_RM (rw) register accessor: an alias for `Reg<CH3_RX_CARRIER_RM_SPEC>`"]
pub type CH3_RX_CARRIER_RM = crate::Reg<ch3_rx_carrier_rm::CH3_RX_CARRIER_RM_SPEC>;
#[doc = "RMT_CH3_RX_CARRIER_RM_REG."]
pub mod ch3_rx_carrier_rm;
#[doc = "CH_TX_LIM (rw) register accessor: an alias for `Reg<CH_TX_LIM_SPEC>`"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = "RMT_CH%s_TX_LIM_REG."]
pub mod ch_tx_lim;
#[doc = "CH_RX_LIM (rw) register accessor: an alias for `Reg<CH_RX_LIM_SPEC>`"]
pub type CH_RX_LIM = crate::Reg<ch_rx_lim::CH_RX_LIM_SPEC>;
#[doc = "RMT_CH2_RX_LIM_REG."]
pub mod ch_rx_lim;
#[doc = "SYS_CONF (rw) register accessor: an alias for `Reg<SYS_CONF_SPEC>`"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT_SYS_CONF_REG."]
pub mod sys_conf;
#[doc = "TX_SIM (rw) register accessor: an alias for `Reg<TX_SIM_SPEC>`"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT_TX_SIM_REG."]
pub mod tx_sim;
#[doc = "REF_CNT_RST (w) register accessor: an alias for `Reg<REF_CNT_RST_SPEC>`"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT_REF_CNT_RST_REG."]
pub mod ref_cnt_rst;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT_DATE_REG."]
pub mod date;
