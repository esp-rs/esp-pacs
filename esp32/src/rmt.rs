#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub ch0data: CH0DATA,
    #[doc = "0x04 - "]
    pub ch1data: CH1DATA,
    #[doc = "0x08 - "]
    pub ch2data: CH2DATA,
    #[doc = "0x0c - "]
    pub ch3data: CH3DATA,
    #[doc = "0x10 - "]
    pub ch4data: CH4DATA,
    #[doc = "0x14 - "]
    pub ch5data: CH5DATA,
    #[doc = "0x18 - "]
    pub ch6data: CH6DATA,
    #[doc = "0x1c - "]
    pub ch7data: CH7DATA,
    #[doc = "0x20 - "]
    pub ch0conf0: CHCONF0,
    #[doc = "0x24 - "]
    pub ch0conf1: CHCONF1,
    #[doc = "0x28 - "]
    pub ch1conf0: CHCONF0,
    #[doc = "0x2c - "]
    pub ch1conf1: CHCONF1,
    #[doc = "0x30 - "]
    pub ch2conf0: CHCONF0,
    #[doc = "0x34 - "]
    pub ch2conf1: CHCONF1,
    #[doc = "0x38 - "]
    pub ch3conf0: CHCONF0,
    #[doc = "0x3c - "]
    pub ch3conf1: CHCONF1,
    #[doc = "0x40 - "]
    pub ch4conf0: CHCONF0,
    #[doc = "0x44 - "]
    pub ch4conf1: CHCONF1,
    #[doc = "0x48 - "]
    pub ch5conf0: CHCONF0,
    #[doc = "0x4c - "]
    pub ch5conf1: CHCONF1,
    #[doc = "0x50 - "]
    pub ch6conf0: CHCONF0,
    #[doc = "0x54 - "]
    pub ch6conf1: CHCONF1,
    #[doc = "0x58 - "]
    pub ch7conf0: CHCONF0,
    #[doc = "0x5c - "]
    pub ch7conf1: CHCONF1,
    #[doc = "0x60 - "]
    pub ch0status: CH0STATUS,
    #[doc = "0x64 - "]
    pub ch1status: CH1STATUS,
    #[doc = "0x68 - "]
    pub ch2status: CH2STATUS,
    #[doc = "0x6c - "]
    pub ch3status: CH3STATUS,
    #[doc = "0x70 - "]
    pub ch4status: CH4STATUS,
    #[doc = "0x74 - "]
    pub ch5status: CH5STATUS,
    #[doc = "0x78 - "]
    pub ch6status: CH6STATUS,
    #[doc = "0x7c - "]
    pub ch7status: CH7STATUS,
    #[doc = "0x80 - "]
    pub ch0addr: CH0ADDR,
    #[doc = "0x84 - "]
    pub ch1addr: CH1ADDR,
    #[doc = "0x88 - "]
    pub ch2addr: CH2ADDR,
    #[doc = "0x8c - "]
    pub ch3addr: CH3ADDR,
    #[doc = "0x90 - "]
    pub ch4addr: CH4ADDR,
    #[doc = "0x94 - "]
    pub ch5addr: CH5ADDR,
    #[doc = "0x98 - "]
    pub ch6addr: CH6ADDR,
    #[doc = "0x9c - "]
    pub ch7addr: CH7ADDR,
    #[doc = "0xa0 - "]
    pub int_raw: INT_RAW,
    #[doc = "0xa4 - "]
    pub int_st: INT_ST,
    #[doc = "0xa8 - "]
    pub int_ena: INT_ENA,
    #[doc = "0xac - "]
    pub int_clr: INT_CLR,
    #[doc = "0xb0 - "]
    pub ch0carrier_duty: CH0CARRIER_DUTY,
    #[doc = "0xb4 - "]
    pub ch1carrier_duty: CH1CARRIER_DUTY,
    #[doc = "0xb8 - "]
    pub ch2carrier_duty: CH2CARRIER_DUTY,
    #[doc = "0xbc - "]
    pub ch3carrier_duty: CH3CARRIER_DUTY,
    #[doc = "0xc0 - "]
    pub ch4carrier_duty: CH4CARRIER_DUTY,
    #[doc = "0xc4 - "]
    pub ch5carrier_duty: CH5CARRIER_DUTY,
    #[doc = "0xc8 - "]
    pub ch6carrier_duty: CH6CARRIER_DUTY,
    #[doc = "0xcc - "]
    pub ch7carrier_duty: CH7CARRIER_DUTY,
    #[doc = "0xd0..0xf0 - "]
    pub ch_tx_lim: [CH_TX_LIM; 8],
    #[doc = "0xf0 - "]
    pub apb_conf: APB_CONF,
    _reserved54: [u8; 0x08],
    #[doc = "0xfc - "]
    pub date: DATE,
}
#[doc = "CH0DATA (rw) register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = ""]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = ""]
pub mod ch1data;
#[doc = "CH2DATA (rw) register accessor: an alias for `Reg<CH2DATA_SPEC>`"]
pub type CH2DATA = crate::Reg<ch2data::CH2DATA_SPEC>;
#[doc = ""]
pub mod ch2data;
#[doc = "CH3DATA (rw) register accessor: an alias for `Reg<CH3DATA_SPEC>`"]
pub type CH3DATA = crate::Reg<ch3data::CH3DATA_SPEC>;
#[doc = ""]
pub mod ch3data;
#[doc = "CH4DATA (rw) register accessor: an alias for `Reg<CH4DATA_SPEC>`"]
pub type CH4DATA = crate::Reg<ch4data::CH4DATA_SPEC>;
#[doc = ""]
pub mod ch4data;
#[doc = "CH5DATA (rw) register accessor: an alias for `Reg<CH5DATA_SPEC>`"]
pub type CH5DATA = crate::Reg<ch5data::CH5DATA_SPEC>;
#[doc = ""]
pub mod ch5data;
#[doc = "CH6DATA (rw) register accessor: an alias for `Reg<CH6DATA_SPEC>`"]
pub type CH6DATA = crate::Reg<ch6data::CH6DATA_SPEC>;
#[doc = ""]
pub mod ch6data;
#[doc = "CH7DATA (rw) register accessor: an alias for `Reg<CH7DATA_SPEC>`"]
pub type CH7DATA = crate::Reg<ch7data::CH7DATA_SPEC>;
#[doc = ""]
pub mod ch7data;
#[doc = "CHCONF0 (rw) register accessor: an alias for `Reg<CHCONF0_SPEC>`"]
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
#[doc = ""]
pub mod chconf0;
#[doc = "CHCONF1 (rw) register accessor: an alias for `Reg<CHCONF1_SPEC>`"]
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
#[doc = ""]
pub mod chconf1;
#[doc = "CH0STATUS (r) register accessor: an alias for `Reg<CH0STATUS_SPEC>`"]
pub type CH0STATUS = crate::Reg<ch0status::CH0STATUS_SPEC>;
#[doc = ""]
pub mod ch0status;
#[doc = "CH1STATUS (r) register accessor: an alias for `Reg<CH1STATUS_SPEC>`"]
pub type CH1STATUS = crate::Reg<ch1status::CH1STATUS_SPEC>;
#[doc = ""]
pub mod ch1status;
#[doc = "CH2STATUS (r) register accessor: an alias for `Reg<CH2STATUS_SPEC>`"]
pub type CH2STATUS = crate::Reg<ch2status::CH2STATUS_SPEC>;
#[doc = ""]
pub mod ch2status;
#[doc = "CH3STATUS (r) register accessor: an alias for `Reg<CH3STATUS_SPEC>`"]
pub type CH3STATUS = crate::Reg<ch3status::CH3STATUS_SPEC>;
#[doc = ""]
pub mod ch3status;
#[doc = "CH4STATUS (r) register accessor: an alias for `Reg<CH4STATUS_SPEC>`"]
pub type CH4STATUS = crate::Reg<ch4status::CH4STATUS_SPEC>;
#[doc = ""]
pub mod ch4status;
#[doc = "CH5STATUS (r) register accessor: an alias for `Reg<CH5STATUS_SPEC>`"]
pub type CH5STATUS = crate::Reg<ch5status::CH5STATUS_SPEC>;
#[doc = ""]
pub mod ch5status;
#[doc = "CH6STATUS (r) register accessor: an alias for `Reg<CH6STATUS_SPEC>`"]
pub type CH6STATUS = crate::Reg<ch6status::CH6STATUS_SPEC>;
#[doc = ""]
pub mod ch6status;
#[doc = "CH7STATUS (r) register accessor: an alias for `Reg<CH7STATUS_SPEC>`"]
pub type CH7STATUS = crate::Reg<ch7status::CH7STATUS_SPEC>;
#[doc = ""]
pub mod ch7status;
#[doc = "CH0ADDR (r) register accessor: an alias for `Reg<CH0ADDR_SPEC>`"]
pub type CH0ADDR = crate::Reg<ch0addr::CH0ADDR_SPEC>;
#[doc = ""]
pub mod ch0addr;
#[doc = "CH1ADDR (r) register accessor: an alias for `Reg<CH1ADDR_SPEC>`"]
pub type CH1ADDR = crate::Reg<ch1addr::CH1ADDR_SPEC>;
#[doc = ""]
pub mod ch1addr;
#[doc = "CH2ADDR (r) register accessor: an alias for `Reg<CH2ADDR_SPEC>`"]
pub type CH2ADDR = crate::Reg<ch2addr::CH2ADDR_SPEC>;
#[doc = ""]
pub mod ch2addr;
#[doc = "CH3ADDR (r) register accessor: an alias for `Reg<CH3ADDR_SPEC>`"]
pub type CH3ADDR = crate::Reg<ch3addr::CH3ADDR_SPEC>;
#[doc = ""]
pub mod ch3addr;
#[doc = "CH4ADDR (r) register accessor: an alias for `Reg<CH4ADDR_SPEC>`"]
pub type CH4ADDR = crate::Reg<ch4addr::CH4ADDR_SPEC>;
#[doc = ""]
pub mod ch4addr;
#[doc = "CH5ADDR (r) register accessor: an alias for `Reg<CH5ADDR_SPEC>`"]
pub type CH5ADDR = crate::Reg<ch5addr::CH5ADDR_SPEC>;
#[doc = ""]
pub mod ch5addr;
#[doc = "CH6ADDR (r) register accessor: an alias for `Reg<CH6ADDR_SPEC>`"]
pub type CH6ADDR = crate::Reg<ch6addr::CH6ADDR_SPEC>;
#[doc = ""]
pub mod ch6addr;
#[doc = "CH7ADDR (r) register accessor: an alias for `Reg<CH7ADDR_SPEC>`"]
pub type CH7ADDR = crate::Reg<ch7addr::CH7ADDR_SPEC>;
#[doc = ""]
pub mod ch7addr;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CH0CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH0CARRIER_DUTY_SPEC>`"]
pub type CH0CARRIER_DUTY = crate::Reg<ch0carrier_duty::CH0CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch0carrier_duty;
#[doc = "CH1CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH1CARRIER_DUTY_SPEC>`"]
pub type CH1CARRIER_DUTY = crate::Reg<ch1carrier_duty::CH1CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch1carrier_duty;
#[doc = "CH2CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH2CARRIER_DUTY_SPEC>`"]
pub type CH2CARRIER_DUTY = crate::Reg<ch2carrier_duty::CH2CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch2carrier_duty;
#[doc = "CH3CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH3CARRIER_DUTY_SPEC>`"]
pub type CH3CARRIER_DUTY = crate::Reg<ch3carrier_duty::CH3CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch3carrier_duty;
#[doc = "CH4CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH4CARRIER_DUTY_SPEC>`"]
pub type CH4CARRIER_DUTY = crate::Reg<ch4carrier_duty::CH4CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch4carrier_duty;
#[doc = "CH5CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH5CARRIER_DUTY_SPEC>`"]
pub type CH5CARRIER_DUTY = crate::Reg<ch5carrier_duty::CH5CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch5carrier_duty;
#[doc = "CH6CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH6CARRIER_DUTY_SPEC>`"]
pub type CH6CARRIER_DUTY = crate::Reg<ch6carrier_duty::CH6CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch6carrier_duty;
#[doc = "CH7CARRIER_DUTY (rw) register accessor: an alias for `Reg<CH7CARRIER_DUTY_SPEC>`"]
pub type CH7CARRIER_DUTY = crate::Reg<ch7carrier_duty::CH7CARRIER_DUTY_SPEC>;
#[doc = ""]
pub mod ch7carrier_duty;
#[doc = "CH_TX_LIM (rw) register accessor: an alias for `Reg<CH_TX_LIM_SPEC>`"]
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
#[doc = ""]
pub mod ch_tx_lim;
#[doc = "APB_CONF (rw) register accessor: an alias for `Reg<APB_CONF_SPEC>`"]
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
#[doc = ""]
pub mod apb_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
