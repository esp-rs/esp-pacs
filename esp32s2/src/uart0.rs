#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO data register"]
    pub fifo: FIFO,
    #[doc = "0x04 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x0c - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - Clock divider configuration"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - Autobaud configuration register"]
    pub autobaud: AUTOBAUD,
    #[doc = "0x1c - UART status register"]
    pub status: STATUS,
    #[doc = "0x20 - Configuration register 0"]
    pub conf0: CONF0,
    #[doc = "0x24 - Configuration register 1"]
    pub conf1: CONF1,
    #[doc = "0x28 - Autobaud minimum low pulse duration register"]
    pub lowpulse: LOWPULSE,
    #[doc = "0x2c - Autobaud minimum high pulse duration register"]
    pub highpulse: HIGHPULSE,
    #[doc = "0x30 - Autobaud edge change count register"]
    pub rxd_cnt: RXD_CNT,
    #[doc = "0x34 - Software flow control configuration"]
    pub flow_conf: FLOW_CONF,
    #[doc = "0x38 - Sleep mode configuration"]
    pub sleep_conf: SLEEP_CONF,
    #[doc = "0x3c - Software flow control character configuration"]
    pub swfc_conf0: SWFC_CONF0,
    #[doc = "0x40 - Software flow-control character configuration"]
    pub swfc_conf1: SWFC_CONF1,
    #[doc = "0x44 - Frame end idle time configuration"]
    pub idle_conf: IDLE_CONF,
    #[doc = "0x48 - RS485 mode configuration"]
    pub rs485_conf: RS485_CONF,
    #[doc = "0x4c - Pre-sequence timing configuration"]
    pub at_cmd_precnt: AT_CMD_PRECNT,
    #[doc = "0x50 - Post-sequence timing configuration"]
    pub at_cmd_postcnt: AT_CMD_POSTCNT,
    #[doc = "0x54 - Timeout configuration"]
    pub at_cmd_gaptout: AT_CMD_GAPTOUT,
    #[doc = "0x58 - AT escape sequence selection configuration"]
    pub at_cmd_char: AT_CMD_CHAR,
    #[doc = "0x5c - UART threshold and allocation configuration"]
    pub mem_conf: MEM_CONF,
    #[doc = "0x60 - TX FIFO write and read offset address"]
    pub mem_tx_status: MEM_TX_STATUS,
    #[doc = "0x64 - RX FIFO write and read offset address"]
    pub mem_rx_status: MEM_RX_STATUS,
    #[doc = "0x68 - UART transmitter and receiver status"]
    pub fsm_status: FSM_STATUS,
    #[doc = "0x6c - Autobaud high pulse register"]
    pub pospulse: POSPULSE,
    #[doc = "0x70 - Autobaud low pulse register"]
    pub negpulse: NEGPULSE,
    #[doc = "0x74 - UART version control register"]
    pub date: DATE,
    #[doc = "0x78 - UART ID register"]
    pub id: ID,
}
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock divider configuration"]
pub mod clkdiv;
#[doc = "AUTOBAUD (rw) register accessor: an alias for `Reg<AUTOBAUD_SPEC>`"]
pub type AUTOBAUD = crate::Reg<autobaud::AUTOBAUD_SPEC>;
#[doc = "Autobaud configuration register"]
pub mod autobaud;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "UART status register"]
pub mod status;
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Configuration register 0"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1"]
pub mod conf1;
#[doc = "LOWPULSE (r) register accessor: an alias for `Reg<LOWPULSE_SPEC>`"]
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
#[doc = "Autobaud minimum low pulse duration register"]
pub mod lowpulse;
#[doc = "HIGHPULSE (r) register accessor: an alias for `Reg<HIGHPULSE_SPEC>`"]
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
#[doc = "Autobaud minimum high pulse duration register"]
pub mod highpulse;
#[doc = "RXD_CNT (r) register accessor: an alias for `Reg<RXD_CNT_SPEC>`"]
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
#[doc = "Autobaud edge change count register"]
pub mod rxd_cnt;
#[doc = "FLOW_CONF (rw) register accessor: an alias for `Reg<FLOW_CONF_SPEC>`"]
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
#[doc = "Software flow control configuration"]
pub mod flow_conf;
#[doc = "SLEEP_CONF (rw) register accessor: an alias for `Reg<SLEEP_CONF_SPEC>`"]
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
#[doc = "Sleep mode configuration"]
pub mod sleep_conf;
#[doc = "SWFC_CONF0 (rw) register accessor: an alias for `Reg<SWFC_CONF0_SPEC>`"]
pub type SWFC_CONF0 = crate::Reg<swfc_conf0::SWFC_CONF0_SPEC>;
#[doc = "Software flow control character configuration"]
pub mod swfc_conf0;
#[doc = "SWFC_CONF1 (rw) register accessor: an alias for `Reg<SWFC_CONF1_SPEC>`"]
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf1;
#[doc = "IDLE_CONF (rw) register accessor: an alias for `Reg<IDLE_CONF_SPEC>`"]
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
#[doc = "Frame end idle time configuration"]
pub mod idle_conf;
#[doc = "RS485_CONF (rw) register accessor: an alias for `Reg<RS485_CONF_SPEC>`"]
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
#[doc = "RS485 mode configuration"]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT (rw) register accessor: an alias for `Reg<AT_CMD_PRECNT_SPEC>`"]
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
#[doc = "Pre-sequence timing configuration"]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT (rw) register accessor: an alias for `Reg<AT_CMD_POSTCNT_SPEC>`"]
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
#[doc = "Post-sequence timing configuration"]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT (rw) register accessor: an alias for `Reg<AT_CMD_GAPTOUT_SPEC>`"]
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
#[doc = "Timeout configuration"]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR (rw) register accessor: an alias for `Reg<AT_CMD_CHAR_SPEC>`"]
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
#[doc = "AT escape sequence selection configuration"]
pub mod at_cmd_char;
#[doc = "MEM_CONF (rw) register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "UART threshold and allocation configuration"]
pub mod mem_conf;
#[doc = "MEM_TX_STATUS (r) register accessor: an alias for `Reg<MEM_TX_STATUS_SPEC>`"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = "TX FIFO write and read offset address"]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: an alias for `Reg<MEM_RX_STATUS_SPEC>`"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = "RX FIFO write and read offset address"]
pub mod mem_rx_status;
#[doc = "FSM_STATUS (r) register accessor: an alias for `Reg<FSM_STATUS_SPEC>`"]
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
#[doc = "UART transmitter and receiver status"]
pub mod fsm_status;
#[doc = "POSPULSE (r) register accessor: an alias for `Reg<POSPULSE_SPEC>`"]
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
#[doc = "Autobaud high pulse register"]
pub mod pospulse;
#[doc = "NEGPULSE (r) register accessor: an alias for `Reg<NEGPULSE_SPEC>`"]
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
#[doc = "Autobaud low pulse register"]
pub mod negpulse;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UART version control register"]
pub mod date;
#[doc = "ID (rw) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "UART ID register"]
pub mod id;
