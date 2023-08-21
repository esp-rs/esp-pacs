#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - UART FIFO,length 128"]
    pub uart_fifo: UART_FIFO,
    #[doc = "0x04 - UART INTERRUPT RAW STATE"]
    pub uart_int_raw: UART_INT_RAW,
    #[doc = "0x08 - UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA"]
    pub uart_int_st: UART_INT_ST,
    #[doc = "0x0c - UART INTERRUPT ENABLE REGISTER"]
    pub uart_int_ena: UART_INT_ENA,
    #[doc = "0x10 - UART INTERRUPT CLEAR REGISTER"]
    pub uart_int_clr: UART_INT_CLR,
    #[doc = "0x14 - UART CLK DIV REGISTER"]
    pub uart_clkdiv: UART_CLKDIV,
    #[doc = "0x18 - UART BAUDRATE DETECT REGISTER"]
    pub uart_autobaud: UART_AUTOBAUD,
    #[doc = "0x1c - UART STATUS REGISTER"]
    pub uart_status: UART_STATUS,
    #[doc = "0x20 - UART CONFIG0(UART0 and UART1)"]
    pub uart_conf0: UART_CONF0,
    #[doc = "0x24 - Set this bit to enable rx time-out function"]
    pub uart_conf1: UART_CONF1,
    #[doc = "0x28 - UART_LOWPULSE"]
    pub uart_lowpulse: UART_LOWPULSE,
    #[doc = "0x2c - UART_HIGHPULSE"]
    pub uart_highpulse: UART_HIGHPULSE,
    #[doc = "0x30 - UART_RXD_CNT"]
    pub uart_rxd_cnt: UART_RXD_CNT,
    _reserved13: [u8; 0x44],
    #[doc = "0x78 - UART HW INFO"]
    pub uart_date: UART_DATE,
    #[doc = "0x7c - UART_ID"]
    pub uart_id: UART_ID,
}
#[doc = "UART_FIFO (rw) register accessor: UART FIFO,length 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_fifo`] module"]
pub type UART_FIFO = crate::Reg<uart_fifo::UART_FIFO_SPEC>;
#[doc = "UART FIFO,length 128"]
pub mod uart_fifo;
#[doc = "UART_INT_RAW (r) register accessor: UART INTERRUPT RAW STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_int_raw`] module"]
pub type UART_INT_RAW = crate::Reg<uart_int_raw::UART_INT_RAW_SPEC>;
#[doc = "UART INTERRUPT RAW STATE"]
pub mod uart_int_raw;
#[doc = "UART_INT_ST (r) register accessor: UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_int_st`] module"]
pub type UART_INT_ST = crate::Reg<uart_int_st::UART_INT_ST_SPEC>;
#[doc = "UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA"]
pub mod uart_int_st;
#[doc = "UART_INT_ENA (rw) register accessor: UART INTERRUPT ENABLE REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_int_ena`] module"]
pub type UART_INT_ENA = crate::Reg<uart_int_ena::UART_INT_ENA_SPEC>;
#[doc = "UART INTERRUPT ENABLE REGISTER"]
pub mod uart_int_ena;
#[doc = "UART_INT_CLR (w) register accessor: UART INTERRUPT CLEAR REGISTER\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_int_clr`] module"]
pub type UART_INT_CLR = crate::Reg<uart_int_clr::UART_INT_CLR_SPEC>;
#[doc = "UART INTERRUPT CLEAR REGISTER"]
pub mod uart_int_clr;
#[doc = "UART_CLKDIV (rw) register accessor: UART CLK DIV REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_clkdiv`] module"]
pub type UART_CLKDIV = crate::Reg<uart_clkdiv::UART_CLKDIV_SPEC>;
#[doc = "UART CLK DIV REGISTER"]
pub mod uart_clkdiv;
#[doc = "UART_AUTOBAUD (rw) register accessor: UART BAUDRATE DETECT REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_autobaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_autobaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_autobaud`] module"]
pub type UART_AUTOBAUD = crate::Reg<uart_autobaud::UART_AUTOBAUD_SPEC>;
#[doc = "UART BAUDRATE DETECT REGISTER"]
pub mod uart_autobaud;
#[doc = "UART_STATUS (r) register accessor: UART STATUS REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_status`] module"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "UART STATUS REGISTER"]
pub mod uart_status;
#[doc = "UART_CONF0 (rw) register accessor: UART CONFIG0(UART0 and UART1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_conf0`] module"]
pub type UART_CONF0 = crate::Reg<uart_conf0::UART_CONF0_SPEC>;
#[doc = "UART CONFIG0(UART0 and UART1)"]
pub mod uart_conf0;
#[doc = "UART_CONF1 (rw) register accessor: Set this bit to enable rx time-out function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_conf1`] module"]
pub type UART_CONF1 = crate::Reg<uart_conf1::UART_CONF1_SPEC>;
#[doc = "Set this bit to enable rx time-out function"]
pub mod uart_conf1;
#[doc = "UART_LOWPULSE (r) register accessor: UART_LOWPULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_lowpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_lowpulse`] module"]
pub type UART_LOWPULSE = crate::Reg<uart_lowpulse::UART_LOWPULSE_SPEC>;
#[doc = "UART_LOWPULSE"]
pub mod uart_lowpulse;
#[doc = "UART_HIGHPULSE (r) register accessor: UART_HIGHPULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_highpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_highpulse`] module"]
pub type UART_HIGHPULSE = crate::Reg<uart_highpulse::UART_HIGHPULSE_SPEC>;
#[doc = "UART_HIGHPULSE"]
pub mod uart_highpulse;
#[doc = "UART_RXD_CNT (r) register accessor: UART_RXD_CNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rxd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_rxd_cnt`] module"]
pub type UART_RXD_CNT = crate::Reg<uart_rxd_cnt::UART_RXD_CNT_SPEC>;
#[doc = "UART_RXD_CNT"]
pub mod uart_rxd_cnt;
#[doc = "UART_DATE (rw) register accessor: UART HW INFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_date`] module"]
pub type UART_DATE = crate::Reg<uart_date::UART_DATE_SPEC>;
#[doc = "UART HW INFO"]
pub mod uart_date;
#[doc = "UART_ID (rw) register accessor: UART_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`uart_id`] module"]
pub type UART_ID = crate::Reg<uart_id::UART_ID_SPEC>;
#[doc = "UART_ID"]
pub mod uart_id;
