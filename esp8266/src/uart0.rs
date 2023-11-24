#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    uart_fifo: UART_FIFO,
    uart_int_raw: UART_INT_RAW,
    uart_int_st: UART_INT_ST,
    uart_int_ena: UART_INT_ENA,
    uart_int_clr: UART_INT_CLR,
    uart_clkdiv: UART_CLKDIV,
    uart_autobaud: UART_AUTOBAUD,
    uart_status: UART_STATUS,
    uart_conf0: UART_CONF0,
    uart_conf1: UART_CONF1,
    uart_lowpulse: UART_LOWPULSE,
    uart_highpulse: UART_HIGHPULSE,
    uart_rxd_cnt: UART_RXD_CNT,
    _reserved13: [u8; 0x44],
    uart_date: UART_DATE,
    uart_id: UART_ID,
}
impl RegisterBlock {
    #[doc = "0x00 - UART FIFO,length 128"]
    #[inline(always)]
    pub const fn uart_fifo(&self) -> &UART_FIFO {
        &self.uart_fifo
    }
    #[doc = "0x04 - UART INTERRUPT RAW STATE"]
    #[inline(always)]
    pub const fn uart_int_raw(&self) -> &UART_INT_RAW {
        &self.uart_int_raw
    }
    #[doc = "0x08 - UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA"]
    #[inline(always)]
    pub const fn uart_int_st(&self) -> &UART_INT_ST {
        &self.uart_int_st
    }
    #[doc = "0x0c - UART INTERRUPT ENABLE REGISTER"]
    #[inline(always)]
    pub const fn uart_int_ena(&self) -> &UART_INT_ENA {
        &self.uart_int_ena
    }
    #[doc = "0x10 - UART INTERRUPT CLEAR REGISTER"]
    #[inline(always)]
    pub const fn uart_int_clr(&self) -> &UART_INT_CLR {
        &self.uart_int_clr
    }
    #[doc = "0x14 - UART CLK DIV REGISTER"]
    #[inline(always)]
    pub const fn uart_clkdiv(&self) -> &UART_CLKDIV {
        &self.uart_clkdiv
    }
    #[doc = "0x18 - UART BAUDRATE DETECT REGISTER"]
    #[inline(always)]
    pub const fn uart_autobaud(&self) -> &UART_AUTOBAUD {
        &self.uart_autobaud
    }
    #[doc = "0x1c - UART STATUS REGISTER"]
    #[inline(always)]
    pub const fn uart_status(&self) -> &UART_STATUS {
        &self.uart_status
    }
    #[doc = "0x20 - UART CONFIG0(UART0 and UART1)"]
    #[inline(always)]
    pub const fn uart_conf0(&self) -> &UART_CONF0 {
        &self.uart_conf0
    }
    #[doc = "0x24 - Set this bit to enable rx time-out function"]
    #[inline(always)]
    pub const fn uart_conf1(&self) -> &UART_CONF1 {
        &self.uart_conf1
    }
    #[doc = "0x28 - UART_LOWPULSE"]
    #[inline(always)]
    pub const fn uart_lowpulse(&self) -> &UART_LOWPULSE {
        &self.uart_lowpulse
    }
    #[doc = "0x2c - UART_HIGHPULSE"]
    #[inline(always)]
    pub const fn uart_highpulse(&self) -> &UART_HIGHPULSE {
        &self.uart_highpulse
    }
    #[doc = "0x30 - UART_RXD_CNT"]
    #[inline(always)]
    pub const fn uart_rxd_cnt(&self) -> &UART_RXD_CNT {
        &self.uart_rxd_cnt
    }
    #[doc = "0x78 - UART HW INFO"]
    #[inline(always)]
    pub const fn uart_date(&self) -> &UART_DATE {
        &self.uart_date
    }
    #[doc = "0x7c - UART_ID"]
    #[inline(always)]
    pub const fn uart_id(&self) -> &UART_ID {
        &self.uart_id
    }
}
#[doc = "UART_FIFO (rw) register accessor: UART FIFO,length 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo`] module"]
pub type UART_FIFO = crate::Reg<uart_fifo::UART_FIFO_SPEC>;
#[doc = "UART FIFO,length 128"]
pub mod uart_fifo;
#[doc = "UART_INT_RAW (r) register accessor: UART INTERRUPT RAW STATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_raw`] module"]
pub type UART_INT_RAW = crate::Reg<uart_int_raw::UART_INT_RAW_SPEC>;
#[doc = "UART INTERRUPT RAW STATE"]
pub mod uart_int_raw;
#[doc = "UART_INT_ST (r) register accessor: UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_st`] module"]
pub type UART_INT_ST = crate::Reg<uart_int_st::UART_INT_ST_SPEC>;
#[doc = "UART INTERRUPT STATEREGISTERUART_INT_RAW&amp;UART_INT_ENA"]
pub mod uart_int_st;
#[doc = "UART_INT_ENA (rw) register accessor: UART INTERRUPT ENABLE REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_ena`] module"]
pub type UART_INT_ENA = crate::Reg<uart_int_ena::UART_INT_ENA_SPEC>;
#[doc = "UART INTERRUPT ENABLE REGISTER"]
pub mod uart_int_ena;
#[doc = "UART_INT_CLR (w) register accessor: UART INTERRUPT CLEAR REGISTER\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_clr`] module"]
pub type UART_INT_CLR = crate::Reg<uart_int_clr::UART_INT_CLR_SPEC>;
#[doc = "UART INTERRUPT CLEAR REGISTER"]
pub mod uart_int_clr;
#[doc = "UART_CLKDIV (rw) register accessor: UART CLK DIV REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_clkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_clkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_clkdiv`] module"]
pub type UART_CLKDIV = crate::Reg<uart_clkdiv::UART_CLKDIV_SPEC>;
#[doc = "UART CLK DIV REGISTER"]
pub mod uart_clkdiv;
#[doc = "UART_AUTOBAUD (rw) register accessor: UART BAUDRATE DETECT REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_autobaud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_autobaud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_autobaud`] module"]
pub type UART_AUTOBAUD = crate::Reg<uart_autobaud::UART_AUTOBAUD_SPEC>;
#[doc = "UART BAUDRATE DETECT REGISTER"]
pub mod uart_autobaud;
#[doc = "UART_STATUS (r) register accessor: UART STATUS REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_status`] module"]
pub type UART_STATUS = crate::Reg<uart_status::UART_STATUS_SPEC>;
#[doc = "UART STATUS REGISTER"]
pub mod uart_status;
#[doc = "UART_CONF0 (rw) register accessor: UART CONFIG0(UART0 and UART1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_conf0`] module"]
pub type UART_CONF0 = crate::Reg<uart_conf0::UART_CONF0_SPEC>;
#[doc = "UART CONFIG0(UART0 and UART1)"]
pub mod uart_conf0;
#[doc = "UART_CONF1 (rw) register accessor: Set this bit to enable rx time-out function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_conf1`] module"]
pub type UART_CONF1 = crate::Reg<uart_conf1::UART_CONF1_SPEC>;
#[doc = "Set this bit to enable rx time-out function"]
pub mod uart_conf1;
#[doc = "UART_LOWPULSE (r) register accessor: UART_LOWPULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_lowpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_lowpulse`] module"]
pub type UART_LOWPULSE = crate::Reg<uart_lowpulse::UART_LOWPULSE_SPEC>;
#[doc = "UART_LOWPULSE"]
pub mod uart_lowpulse;
#[doc = "UART_HIGHPULSE (r) register accessor: UART_HIGHPULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_highpulse::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_highpulse`] module"]
pub type UART_HIGHPULSE = crate::Reg<uart_highpulse::UART_HIGHPULSE_SPEC>;
#[doc = "UART_HIGHPULSE"]
pub mod uart_highpulse;
#[doc = "UART_RXD_CNT (r) register accessor: UART_RXD_CNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rxd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rxd_cnt`] module"]
pub type UART_RXD_CNT = crate::Reg<uart_rxd_cnt::UART_RXD_CNT_SPEC>;
#[doc = "UART_RXD_CNT"]
pub mod uart_rxd_cnt;
#[doc = "UART_DATE (rw) register accessor: UART HW INFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_date`] module"]
pub type UART_DATE = crate::Reg<uart_date::UART_DATE_SPEC>;
#[doc = "UART HW INFO"]
pub mod uart_date;
#[doc = "UART_ID (rw) register accessor: UART_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_id`] module"]
pub type UART_ID = crate::Reg<uart_id::UART_ID_SPEC>;
#[doc = "UART_ID"]
pub mod uart_id;
