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
#[doc = "FIFO (rw) register accessor: FIFO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
#[doc = "INT_RAW (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CLKDIV (rw) register accessor: Clock divider configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkdiv`] module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock divider configuration"]
pub mod clkdiv;
#[doc = "AUTOBAUD (rw) register accessor: Autobaud configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autobaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autobaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`autobaud`] module"]
pub type AUTOBAUD = crate::Reg<autobaud::AUTOBAUD_SPEC>;
#[doc = "Autobaud configuration register"]
pub mod autobaud;
#[doc = "STATUS (r) register accessor: UART status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "UART status register"]
pub mod status;
#[doc = "CONF0 (rw) register accessor: Configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Configuration register 0"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1"]
pub mod conf1;
#[doc = "LOWPULSE (r) register accessor: Autobaud minimum low pulse duration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lowpulse`] module"]
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
#[doc = "Autobaud minimum low pulse duration register"]
pub mod lowpulse;
#[doc = "HIGHPULSE (r) register accessor: Autobaud minimum high pulse duration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`highpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`highpulse`] module"]
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
#[doc = "Autobaud minimum high pulse duration register"]
pub mod highpulse;
#[doc = "RXD_CNT (r) register accessor: Autobaud edge change count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxd_cnt`] module"]
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
#[doc = "Autobaud edge change count register"]
pub mod rxd_cnt;
#[doc = "FLOW_CONF (rw) register accessor: Software flow control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flow_conf`] module"]
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
#[doc = "Software flow control configuration"]
pub mod flow_conf;
#[doc = "SLEEP_CONF (rw) register accessor: Sleep mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sleep_conf`] module"]
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
#[doc = "Sleep mode configuration"]
pub mod sleep_conf;
#[doc = "SWFC_CONF0 (rw) register accessor: Software flow control character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swfc_conf0`] module"]
pub type SWFC_CONF0 = crate::Reg<swfc_conf0::SWFC_CONF0_SPEC>;
#[doc = "Software flow control character configuration"]
pub mod swfc_conf0;
#[doc = "SWFC_CONF1 (rw) register accessor: Software flow-control character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swfc_conf1`] module"]
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf1;
#[doc = "IDLE_CONF (rw) register accessor: Frame end idle time configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idle_conf`] module"]
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
#[doc = "Frame end idle time configuration"]
pub mod idle_conf;
#[doc = "RS485_CONF (rw) register accessor: RS485 mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rs485_conf`] module"]
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
#[doc = "RS485 mode configuration"]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT (rw) register accessor: Pre-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_precnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_precnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_precnt`] module"]
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
#[doc = "Pre-sequence timing configuration"]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT (rw) register accessor: Post-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_postcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_postcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_postcnt`] module"]
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
#[doc = "Post-sequence timing configuration"]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT (rw) register accessor: Timeout configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_gaptout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_gaptout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_gaptout`] module"]
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
#[doc = "Timeout configuration"]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR (rw) register accessor: AT escape sequence selection configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_char`] module"]
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
#[doc = "AT escape sequence selection configuration"]
pub mod at_cmd_char;
#[doc = "MEM_CONF (rw) register accessor: UART threshold and allocation configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "UART threshold and allocation configuration"]
pub mod mem_conf;
#[doc = "MEM_TX_STATUS (r) register accessor: TX FIFO write and read offset address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_tx_status`] module"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = "TX FIFO write and read offset address"]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: RX FIFO write and read offset address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_rx_status`] module"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = "RX FIFO write and read offset address"]
pub mod mem_rx_status;
#[doc = "FSM_STATUS (r) register accessor: UART transmitter and receiver status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm_status`] module"]
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
#[doc = "UART transmitter and receiver status"]
pub mod fsm_status;
#[doc = "POSPULSE (r) register accessor: Autobaud high pulse register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pospulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pospulse`] module"]
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
#[doc = "Autobaud high pulse register"]
pub mod pospulse;
#[doc = "NEGPULSE (r) register accessor: Autobaud low pulse register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`negpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`negpulse`] module"]
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
#[doc = "Autobaud low pulse register"]
pub mod negpulse;
#[doc = "DATE (rw) register accessor: UART version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UART version control register"]
pub mod date;
#[doc = "ID (rw) register accessor: UART ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "UART ID register"]
pub mod id;
