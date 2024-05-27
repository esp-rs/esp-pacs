#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    fifo: FIFO,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    clkdiv: CLKDIV,
    autobaud: AUTOBAUD,
    status: STATUS,
    conf0: CONF0,
    conf1: CONF1,
    lowpulse: LOWPULSE,
    highpulse: HIGHPULSE,
    rxd_cnt: RXD_CNT,
    flow_conf: FLOW_CONF,
    sleep_conf: SLEEP_CONF,
    swfc_conf0: SWFC_CONF0,
    swfc_conf1: SWFC_CONF1,
    idle_conf: IDLE_CONF,
    rs485_conf: RS485_CONF,
    at_cmd_precnt: AT_CMD_PRECNT,
    at_cmd_postcnt: AT_CMD_POSTCNT,
    at_cmd_gaptout: AT_CMD_GAPTOUT,
    at_cmd_char: AT_CMD_CHAR,
    mem_conf: MEM_CONF,
    mem_tx_status: MEM_TX_STATUS,
    mem_rx_status: MEM_RX_STATUS,
    fsm_status: FSM_STATUS,
    pospulse: POSPULSE,
    negpulse: NEGPULSE,
    date: DATE,
    id: ID,
}
impl RegisterBlock {
    ///0x00 - FIFO data register
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
    ///0x04 - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x08 - Masked interrupt status
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x0c - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x10 - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x14 - Clock divider configuration
    #[inline(always)]
    pub const fn clkdiv(&self) -> &CLKDIV {
        &self.clkdiv
    }
    ///0x18 - Autobaud configuration register
    #[inline(always)]
    pub const fn autobaud(&self) -> &AUTOBAUD {
        &self.autobaud
    }
    ///0x1c - UART status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x20 - Configuration register 0
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    ///0x24 - Configuration register 1
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    ///0x28 - Autobaud minimum low pulse duration register
    #[inline(always)]
    pub const fn lowpulse(&self) -> &LOWPULSE {
        &self.lowpulse
    }
    ///0x2c - Autobaud minimum high pulse duration register
    #[inline(always)]
    pub const fn highpulse(&self) -> &HIGHPULSE {
        &self.highpulse
    }
    ///0x30 - Autobaud edge change count register
    #[inline(always)]
    pub const fn rxd_cnt(&self) -> &RXD_CNT {
        &self.rxd_cnt
    }
    ///0x34 - Software flow control configuration
    #[inline(always)]
    pub const fn flow_conf(&self) -> &FLOW_CONF {
        &self.flow_conf
    }
    ///0x38 - Sleep mode configuration
    #[inline(always)]
    pub const fn sleep_conf(&self) -> &SLEEP_CONF {
        &self.sleep_conf
    }
    ///0x3c - Software flow control character configuration
    #[inline(always)]
    pub const fn swfc_conf0(&self) -> &SWFC_CONF0 {
        &self.swfc_conf0
    }
    ///0x40 - Software flow-control character configuration
    #[inline(always)]
    pub const fn swfc_conf1(&self) -> &SWFC_CONF1 {
        &self.swfc_conf1
    }
    ///0x44 - Frame end idle time configuration
    #[inline(always)]
    pub const fn idle_conf(&self) -> &IDLE_CONF {
        &self.idle_conf
    }
    ///0x48 - RS485 mode configuration
    #[inline(always)]
    pub const fn rs485_conf(&self) -> &RS485_CONF {
        &self.rs485_conf
    }
    ///0x4c - Pre-sequence timing configuration
    #[inline(always)]
    pub const fn at_cmd_precnt(&self) -> &AT_CMD_PRECNT {
        &self.at_cmd_precnt
    }
    ///0x50 - Post-sequence timing configuration
    #[inline(always)]
    pub const fn at_cmd_postcnt(&self) -> &AT_CMD_POSTCNT {
        &self.at_cmd_postcnt
    }
    ///0x54 - Timeout configuration
    #[inline(always)]
    pub const fn at_cmd_gaptout(&self) -> &AT_CMD_GAPTOUT {
        &self.at_cmd_gaptout
    }
    ///0x58 - AT escape sequence selection configuration
    #[inline(always)]
    pub const fn at_cmd_char(&self) -> &AT_CMD_CHAR {
        &self.at_cmd_char
    }
    ///0x5c - UART threshold and allocation configuration
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MEM_CONF {
        &self.mem_conf
    }
    ///0x60 - TX FIFO write and read offset address
    #[inline(always)]
    pub const fn mem_tx_status(&self) -> &MEM_TX_STATUS {
        &self.mem_tx_status
    }
    ///0x64 - RX FIFO write and read offset address
    #[inline(always)]
    pub const fn mem_rx_status(&self) -> &MEM_RX_STATUS {
        &self.mem_rx_status
    }
    ///0x68 - UART transmitter and receiver status
    #[inline(always)]
    pub const fn fsm_status(&self) -> &FSM_STATUS {
        &self.fsm_status
    }
    ///0x6c - Autobaud high pulse register
    #[inline(always)]
    pub const fn pospulse(&self) -> &POSPULSE {
        &self.pospulse
    }
    ///0x70 - Autobaud low pulse register
    #[inline(always)]
    pub const fn negpulse(&self) -> &NEGPULSE {
        &self.negpulse
    }
    ///0x74 - UART version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x78 - UART ID register
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}
/**FIFO (rw) register accessor: FIFO data register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo`] module*/
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///FIFO data register
pub mod fifo;
/**INT_RAW (r) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Raw interrupt status
pub mod int_raw;
/**INT_ST (r) register accessor: Masked interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Masked interrupt status
pub mod int_st;
/**INT_ENA (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable bits
pub mod int_ena;
/**INT_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear bits
pub mod int_clr;
/**CLKDIV (rw) register accessor: Clock divider configuration

You can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clkdiv`] module*/
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
///Clock divider configuration
pub mod clkdiv;
/**AUTOBAUD (rw) register accessor: Autobaud configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`autobaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autobaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@autobaud`] module*/
pub type AUTOBAUD = crate::Reg<autobaud::AUTOBAUD_SPEC>;
///Autobaud configuration register
pub mod autobaud;
/**STATUS (r) register accessor: UART status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///UART status register
pub mod status;
/**CONF0 (rw) register accessor: Configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf0`] module*/
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
///Configuration register 0
pub mod conf0;
/**CONF1 (rw) register accessor: Configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf1`] module*/
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
///Configuration register 1
pub mod conf1;
/**LOWPULSE (r) register accessor: Autobaud minimum low pulse duration register

You can [`read`](crate::generic::Reg::read) this register and get [`lowpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lowpulse`] module*/
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
///Autobaud minimum low pulse duration register
pub mod lowpulse;
/**HIGHPULSE (r) register accessor: Autobaud minimum high pulse duration register

You can [`read`](crate::generic::Reg::read) this register and get [`highpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@highpulse`] module*/
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
///Autobaud minimum high pulse duration register
pub mod highpulse;
/**RXD_CNT (r) register accessor: Autobaud edge change count register

You can [`read`](crate::generic::Reg::read) this register and get [`rxd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxd_cnt`] module*/
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
///Autobaud edge change count register
pub mod rxd_cnt;
/**FLOW_CONF (rw) register accessor: Software flow control configuration

You can [`read`](crate::generic::Reg::read) this register and get [`flow_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flow_conf`] module*/
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
///Software flow control configuration
pub mod flow_conf;
/**SLEEP_CONF (rw) register accessor: Sleep mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sleep_conf`] module*/
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
///Sleep mode configuration
pub mod sleep_conf;
/**SWFC_CONF0 (rw) register accessor: Software flow control character configuration

You can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swfc_conf0`] module*/
pub type SWFC_CONF0 = crate::Reg<swfc_conf0::SWFC_CONF0_SPEC>;
///Software flow control character configuration
pub mod swfc_conf0;
/**SWFC_CONF1 (rw) register accessor: Software flow-control character configuration

You can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swfc_conf1`] module*/
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
///Software flow-control character configuration
pub mod swfc_conf1;
/**IDLE_CONF (rw) register accessor: Frame end idle time configuration

You can [`read`](crate::generic::Reg::read) this register and get [`idle_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idle_conf`] module*/
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
///Frame end idle time configuration
pub mod idle_conf;
/**RS485_CONF (rw) register accessor: RS485 mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rs485_conf`] module*/
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
///RS485 mode configuration
pub mod rs485_conf;
/**AT_CMD_PRECNT (rw) register accessor: Pre-sequence timing configuration

You can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_precnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_precnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@at_cmd_precnt`] module*/
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
///Pre-sequence timing configuration
pub mod at_cmd_precnt;
/**AT_CMD_POSTCNT (rw) register accessor: Post-sequence timing configuration

You can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_postcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_postcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@at_cmd_postcnt`] module*/
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
///Post-sequence timing configuration
pub mod at_cmd_postcnt;
/**AT_CMD_GAPTOUT (rw) register accessor: Timeout configuration

You can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_gaptout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_gaptout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@at_cmd_gaptout`] module*/
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
///Timeout configuration
pub mod at_cmd_gaptout;
/**AT_CMD_CHAR (rw) register accessor: AT escape sequence selection configuration

You can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@at_cmd_char`] module*/
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
///AT escape sequence selection configuration
pub mod at_cmd_char;
/**MEM_CONF (rw) register accessor: UART threshold and allocation configuration

You can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_conf`] module*/
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
///UART threshold and allocation configuration
pub mod mem_conf;
/**MEM_TX_STATUS (r) register accessor: TX FIFO write and read offset address

You can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_tx_status`] module*/
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
///TX FIFO write and read offset address
pub mod mem_tx_status;
/**MEM_RX_STATUS (r) register accessor: RX FIFO write and read offset address

You can [`read`](crate::generic::Reg::read) this register and get [`mem_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_rx_status`] module*/
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
///RX FIFO write and read offset address
pub mod mem_rx_status;
/**FSM_STATUS (r) register accessor: UART transmitter and receiver status

You can [`read`](crate::generic::Reg::read) this register and get [`fsm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsm_status`] module*/
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
///UART transmitter and receiver status
pub mod fsm_status;
/**POSPULSE (r) register accessor: Autobaud high pulse register

You can [`read`](crate::generic::Reg::read) this register and get [`pospulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pospulse`] module*/
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
///Autobaud high pulse register
pub mod pospulse;
/**NEGPULSE (r) register accessor: Autobaud low pulse register

You can [`read`](crate::generic::Reg::read) this register and get [`negpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@negpulse`] module*/
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
///Autobaud low pulse register
pub mod negpulse;
/**DATE (rw) register accessor: UART version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///UART version control register
pub mod date;
/**ID (rw) register accessor: UART ID register

You can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@id`] module*/
pub type ID = crate::Reg<id::ID_SPEC>;
///UART ID register
pub mod id;
