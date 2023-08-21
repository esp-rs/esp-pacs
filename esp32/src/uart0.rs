#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub fifo: FIFO,
    #[doc = "0x04 - "]
    pub int_raw: INT_RAW,
    #[doc = "0x08 - "]
    pub int_st: INT_ST,
    #[doc = "0x0c - "]
    pub int_ena: INT_ENA,
    #[doc = "0x10 - "]
    pub int_clr: INT_CLR,
    #[doc = "0x14 - "]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - "]
    pub autobaud: AUTOBAUD,
    #[doc = "0x1c - "]
    pub status: STATUS,
    #[doc = "0x20 - "]
    pub conf0: CONF0,
    #[doc = "0x24 - "]
    pub conf1: CONF1,
    #[doc = "0x28 - "]
    pub lowpulse: LOWPULSE,
    #[doc = "0x2c - "]
    pub highpulse: HIGHPULSE,
    #[doc = "0x30 - "]
    pub rxd_cnt: RXD_CNT,
    #[doc = "0x34 - "]
    pub flow_conf: FLOW_CONF,
    #[doc = "0x38 - "]
    pub sleep_conf: SLEEP_CONF,
    #[doc = "0x3c - "]
    pub swfc_conf: SWFC_CONF,
    #[doc = "0x40 - "]
    pub idle_conf: IDLE_CONF,
    #[doc = "0x44 - "]
    pub rs485_conf: RS485_CONF,
    #[doc = "0x48 - "]
    pub at_cmd_precnt: AT_CMD_PRECNT,
    #[doc = "0x4c - "]
    pub at_cmd_postcnt: AT_CMD_POSTCNT,
    #[doc = "0x50 - "]
    pub at_cmd_gaptout: AT_CMD_GAPTOUT,
    #[doc = "0x54 - "]
    pub at_cmd_char: AT_CMD_CHAR,
    #[doc = "0x58 - "]
    pub mem_conf: MEM_CONF,
    #[doc = "0x5c - "]
    pub mem_tx_status: MEM_TX_STATUS,
    #[doc = "0x60 - "]
    pub mem_rx_status: MEM_RX_STATUS,
    #[doc = "0x64 - "]
    pub mem_cnt_status: MEM_CNT_STATUS,
    #[doc = "0x68 - "]
    pub pospulse: POSPULSE,
    #[doc = "0x6c - "]
    pub negpulse: NEGPULSE,
    _reserved28: [u8; 0x08],
    #[doc = "0x78 - "]
    pub date: DATE,
    #[doc = "0x7c - "]
    pub id: ID,
}
#[doc = "FIFO (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = ""]
pub mod fifo;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CLKDIV (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkdiv`] module"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = ""]
pub mod clkdiv;
#[doc = "AUTOBAUD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autobaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autobaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`autobaud`] module"]
pub type AUTOBAUD = crate::Reg<autobaud::AUTOBAUD_SPEC>;
#[doc = ""]
pub mod autobaud;
#[doc = "STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "LOWPULSE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lowpulse`] module"]
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
#[doc = ""]
pub mod lowpulse;
#[doc = "HIGHPULSE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`highpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`highpulse`] module"]
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
#[doc = ""]
pub mod highpulse;
#[doc = "RXD_CNT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxd_cnt`] module"]
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
#[doc = ""]
pub mod rxd_cnt;
#[doc = "FLOW_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flow_conf`] module"]
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
#[doc = ""]
pub mod flow_conf;
#[doc = "SLEEP_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sleep_conf`] module"]
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
#[doc = ""]
pub mod sleep_conf;
#[doc = "SWFC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swfc_conf`] module"]
pub type SWFC_CONF = crate::Reg<swfc_conf::SWFC_CONF_SPEC>;
#[doc = ""]
pub mod swfc_conf;
#[doc = "IDLE_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idle_conf`] module"]
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
#[doc = ""]
pub mod idle_conf;
#[doc = "RS485_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rs485_conf`] module"]
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
#[doc = ""]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_precnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_precnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_precnt`] module"]
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
#[doc = ""]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_postcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_postcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_postcnt`] module"]
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
#[doc = ""]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_gaptout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_gaptout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_gaptout`] module"]
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
#[doc = ""]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_char`] module"]
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
#[doc = ""]
pub mod at_cmd_char;
#[doc = "MEM_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = ""]
pub mod mem_conf;
#[doc = "MEM_TX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_tx_status`] module"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = ""]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_rx_status`] module"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = ""]
pub mod mem_rx_status;
#[doc = "MEM_CNT_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_cnt_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_cnt_status`] module"]
pub type MEM_CNT_STATUS = crate::Reg<mem_cnt_status::MEM_CNT_STATUS_SPEC>;
#[doc = ""]
pub mod mem_cnt_status;
#[doc = "POSPULSE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pospulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pospulse`] module"]
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
#[doc = ""]
pub mod pospulse;
#[doc = "NEGPULSE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`negpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`negpulse`] module"]
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
#[doc = ""]
pub mod negpulse;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ID (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = ""]
pub mod id;
