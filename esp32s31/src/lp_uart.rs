#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    fifo: FIFO,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    clkdiv_sync: CLKDIV_SYNC,
    rx_filt: RX_FILT,
    status: STATUS,
    conf0_sync: CONF0_SYNC,
    conf1: CONF1,
    _reserved10: [u8; 0x04],
    hwfc_conf_sync: HWFC_CONF_SYNC,
    sleep_conf0: SLEEP_CONF0,
    sleep_conf1: SLEEP_CONF1,
    sleep_conf2: SLEEP_CONF2,
    swfc_conf0_sync: SWFC_CONF0_SYNC,
    swfc_conf1: SWFC_CONF1,
    txbrk_conf_sync: TXBRK_CONF_SYNC,
    idle_conf_sync: IDLE_CONF_SYNC,
    delay_conf_sync: DELAY_CONF_SYNC,
    at_cmd_precnt_sync: AT_CMD_PRECNT_SYNC,
    at_cmd_postcnt_sync: AT_CMD_POSTCNT_SYNC,
    at_cmd_gaptout_sync: AT_CMD_GAPTOUT_SYNC,
    at_cmd_char_sync: AT_CMD_CHAR_SYNC,
    mem_conf: MEM_CONF,
    tout_conf_sync: TOUT_CONF_SYNC,
    mem_tx_status: MEM_TX_STATUS,
    mem_rx_status: MEM_RX_STATUS,
    fsm_status: FSM_STATUS,
    _reserved28: [u8; 0x14],
    clk_conf: CLK_CONF,
    date: DATE,
    afifo_status: AFIFO_STATUS,
    _reserved31: [u8; 0x04],
    reg_update: REG_UPDATE,
    id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - FIFO data register"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
    #[doc = "0x04 - Raw interrupt status"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x08 - Masked interrupt status"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x0c - Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x10 - Interrupt clear bits"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x14 - Clock divider configuration"]
    #[inline(always)]
    pub const fn clkdiv_sync(&self) -> &CLKDIV_SYNC {
        &self.clkdiv_sync
    }
    #[doc = "0x18 - RX filter configuration"]
    #[inline(always)]
    pub const fn rx_filt(&self) -> &RX_FILT {
        &self.rx_filt
    }
    #[doc = "0x1c - LP UART status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x20 - Configuration register 0"]
    #[inline(always)]
    pub const fn conf0_sync(&self) -> &CONF0_SYNC {
        &self.conf0_sync
    }
    #[doc = "0x24 - Configuration register 1"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x2c - Hardware flow control configuration"]
    #[inline(always)]
    pub const fn hwfc_conf_sync(&self) -> &HWFC_CONF_SYNC {
        &self.hwfc_conf_sync
    }
    #[doc = "0x30 - LP UART sleep configuration register 0"]
    #[inline(always)]
    pub const fn sleep_conf0(&self) -> &SLEEP_CONF0 {
        &self.sleep_conf0
    }
    #[doc = "0x34 - LP UART sleep configuration register 1"]
    #[inline(always)]
    pub const fn sleep_conf1(&self) -> &SLEEP_CONF1 {
        &self.sleep_conf1
    }
    #[doc = "0x38 - LP UART sleep configuration register 2"]
    #[inline(always)]
    pub const fn sleep_conf2(&self) -> &SLEEP_CONF2 {
        &self.sleep_conf2
    }
    #[doc = "0x3c - Software flow control character configuration"]
    #[inline(always)]
    pub const fn swfc_conf0_sync(&self) -> &SWFC_CONF0_SYNC {
        &self.swfc_conf0_sync
    }
    #[doc = "0x40 - Software flow control character configuration"]
    #[inline(always)]
    pub const fn swfc_conf1(&self) -> &SWFC_CONF1 {
        &self.swfc_conf1
    }
    #[doc = "0x44 - TX break character configuration"]
    #[inline(always)]
    pub const fn txbrk_conf_sync(&self) -> &TXBRK_CONF_SYNC {
        &self.txbrk_conf_sync
    }
    #[doc = "0x48 - Frame end idle time configuration"]
    #[inline(always)]
    pub const fn idle_conf_sync(&self) -> &IDLE_CONF_SYNC {
        &self.idle_conf_sync
    }
    #[doc = "0x4c - RS485 mode configuration"]
    #[inline(always)]
    pub const fn delay_conf_sync(&self) -> &DELAY_CONF_SYNC {
        &self.delay_conf_sync
    }
    #[doc = "0x50 - Pre-sequence timing configuration"]
    #[inline(always)]
    pub const fn at_cmd_precnt_sync(&self) -> &AT_CMD_PRECNT_SYNC {
        &self.at_cmd_precnt_sync
    }
    #[doc = "0x54 - Post-sequence timing configuration"]
    #[inline(always)]
    pub const fn at_cmd_postcnt_sync(&self) -> &AT_CMD_POSTCNT_SYNC {
        &self.at_cmd_postcnt_sync
    }
    #[doc = "0x58 - Timeout configuration"]
    #[inline(always)]
    pub const fn at_cmd_gaptout_sync(&self) -> &AT_CMD_GAPTOUT_SYNC {
        &self.at_cmd_gaptout_sync
    }
    #[doc = "0x5c - AT escape sequence detection configuration"]
    #[inline(always)]
    pub const fn at_cmd_char_sync(&self) -> &AT_CMD_CHAR_SYNC {
        &self.at_cmd_char_sync
    }
    #[doc = "0x60 - UART memory power configuration"]
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MEM_CONF {
        &self.mem_conf
    }
    #[doc = "0x64 - LP UART threshold and allocation configuration"]
    #[inline(always)]
    pub const fn tout_conf_sync(&self) -> &TOUT_CONF_SYNC {
        &self.tout_conf_sync
    }
    #[doc = "0x68 - TX FIFO write and read offset address"]
    #[inline(always)]
    pub const fn mem_tx_status(&self) -> &MEM_TX_STATUS {
        &self.mem_tx_status
    }
    #[doc = "0x6c - RX FIFO write and read offset address"]
    #[inline(always)]
    pub const fn mem_rx_status(&self) -> &MEM_RX_STATUS {
        &self.mem_rx_status
    }
    #[doc = "0x70 - LP UART transmit and receive status"]
    #[inline(always)]
    pub const fn fsm_status(&self) -> &FSM_STATUS {
        &self.fsm_status
    }
    #[doc = "0x88 - LP UART core clock configuration"]
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    #[doc = "0x8c - LP UART version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x90 - LP UART asynchronous FIFO Status"]
    #[inline(always)]
    pub const fn afifo_status(&self) -> &AFIFO_STATUS {
        &self.afifo_status
    }
    #[doc = "0x98 - LP UART register configuration update register"]
    #[inline(always)]
    pub const fn reg_update(&self) -> &REG_UPDATE {
        &self.reg_update
    }
    #[doc = "0x9c - LP UART ID register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}
#[doc = "FIFO (r) register accessor: FIFO data register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CLKDIV_SYNC (rw) register accessor: Clock divider configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv_sync`] module"]
pub type CLKDIV_SYNC = crate::Reg<clkdiv_sync::CLKDIV_SYNC_SPEC>;
#[doc = "Clock divider configuration"]
pub mod clkdiv_sync;
#[doc = "RX_FILT (rw) register accessor: RX filter configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_filt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_filt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_filt`] module"]
pub type RX_FILT = crate::Reg<rx_filt::RX_FILT_SPEC>;
#[doc = "RX filter configuration"]
pub mod rx_filt;
#[doc = "STATUS (r) register accessor: LP UART status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "LP UART status register"]
pub mod status;
#[doc = "CONF0_SYNC (rw) register accessor: Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0_sync`] module"]
pub type CONF0_SYNC = crate::Reg<conf0_sync::CONF0_SYNC_SPEC>;
#[doc = "Configuration register 0"]
pub mod conf0_sync;
#[doc = "CONF1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1"]
pub mod conf1;
#[doc = "HWFC_CONF_SYNC (rw) register accessor: Hardware flow control configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hwfc_conf_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwfc_conf_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwfc_conf_sync`] module"]
pub type HWFC_CONF_SYNC = crate::Reg<hwfc_conf_sync::HWFC_CONF_SYNC_SPEC>;
#[doc = "Hardware flow control configuration"]
pub mod hwfc_conf_sync;
#[doc = "SLEEP_CONF0 (rw) register accessor: LP UART sleep configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf0`] module"]
pub type SLEEP_CONF0 = crate::Reg<sleep_conf0::SLEEP_CONF0_SPEC>;
#[doc = "LP UART sleep configuration register 0"]
pub mod sleep_conf0;
#[doc = "SLEEP_CONF1 (rw) register accessor: LP UART sleep configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf1`] module"]
pub type SLEEP_CONF1 = crate::Reg<sleep_conf1::SLEEP_CONF1_SPEC>;
#[doc = "LP UART sleep configuration register 1"]
pub mod sleep_conf1;
#[doc = "SLEEP_CONF2 (rw) register accessor: LP UART sleep configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_conf2`] module"]
pub type SLEEP_CONF2 = crate::Reg<sleep_conf2::SLEEP_CONF2_SPEC>;
#[doc = "LP UART sleep configuration register 2"]
pub mod sleep_conf2;
#[doc = "SWFC_CONF0_SYNC (rw) register accessor: Software flow control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf0_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf0_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swfc_conf0_sync`] module"]
pub type SWFC_CONF0_SYNC = crate::Reg<swfc_conf0_sync::SWFC_CONF0_SYNC_SPEC>;
#[doc = "Software flow control character configuration"]
pub mod swfc_conf0_sync;
#[doc = "SWFC_CONF1 (rw) register accessor: Software flow control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swfc_conf1`] module"]
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
#[doc = "Software flow control character configuration"]
pub mod swfc_conf1;
#[doc = "TXBRK_CONF_SYNC (rw) register accessor: TX break character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrk_conf_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbrk_conf_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrk_conf_sync`] module"]
pub type TXBRK_CONF_SYNC = crate::Reg<txbrk_conf_sync::TXBRK_CONF_SYNC_SPEC>;
#[doc = "TX break character configuration"]
pub mod txbrk_conf_sync;
#[doc = "IDLE_CONF_SYNC (rw) register accessor: Frame end idle time configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_conf_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_conf_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_conf_sync`] module"]
pub type IDLE_CONF_SYNC = crate::Reg<idle_conf_sync::IDLE_CONF_SYNC_SPEC>;
#[doc = "Frame end idle time configuration"]
pub mod idle_conf_sync;
#[doc = "DELAY_CONF_SYNC (rw) register accessor: RS485 mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_conf_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_conf_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay_conf_sync`] module"]
pub type DELAY_CONF_SYNC = crate::Reg<delay_conf_sync::DELAY_CONF_SYNC_SPEC>;
#[doc = "RS485 mode configuration"]
pub mod delay_conf_sync;
#[doc = "AT_CMD_PRECNT_SYNC (rw) register accessor: Pre-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_precnt_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_precnt_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_precnt_sync`] module"]
pub type AT_CMD_PRECNT_SYNC = crate::Reg<at_cmd_precnt_sync::AT_CMD_PRECNT_SYNC_SPEC>;
#[doc = "Pre-sequence timing configuration"]
pub mod at_cmd_precnt_sync;
#[doc = "AT_CMD_POSTCNT_SYNC (rw) register accessor: Post-sequence timing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_postcnt_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_postcnt_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_postcnt_sync`] module"]
pub type AT_CMD_POSTCNT_SYNC = crate::Reg<at_cmd_postcnt_sync::AT_CMD_POSTCNT_SYNC_SPEC>;
#[doc = "Post-sequence timing configuration"]
pub mod at_cmd_postcnt_sync;
#[doc = "AT_CMD_GAPTOUT_SYNC (rw) register accessor: Timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_gaptout_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_gaptout_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_gaptout_sync`] module"]
pub type AT_CMD_GAPTOUT_SYNC = crate::Reg<at_cmd_gaptout_sync::AT_CMD_GAPTOUT_SYNC_SPEC>;
#[doc = "Timeout configuration"]
pub mod at_cmd_gaptout_sync;
#[doc = "AT_CMD_CHAR_SYNC (rw) register accessor: AT escape sequence detection configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_char_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_char_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at_cmd_char_sync`] module"]
pub type AT_CMD_CHAR_SYNC = crate::Reg<at_cmd_char_sync::AT_CMD_CHAR_SYNC_SPEC>;
#[doc = "AT escape sequence detection configuration"]
pub mod at_cmd_char_sync;
#[doc = "MEM_CONF (rw) register accessor: UART memory power configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "UART memory power configuration"]
pub mod mem_conf;
#[doc = "TOUT_CONF_SYNC (rw) register accessor: LP UART threshold and allocation configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tout_conf_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout_conf_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tout_conf_sync`] module"]
pub type TOUT_CONF_SYNC = crate::Reg<tout_conf_sync::TOUT_CONF_SYNC_SPEC>;
#[doc = "LP UART threshold and allocation configuration"]
pub mod tout_conf_sync;
#[doc = "MEM_TX_STATUS (r) register accessor: TX FIFO write and read offset address\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_tx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_tx_status`] module"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = "TX FIFO write and read offset address"]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: RX FIFO write and read offset address\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rx_status`] module"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = "RX FIFO write and read offset address"]
pub mod mem_rx_status;
#[doc = "FSM_STATUS (r) register accessor: LP UART transmit and receive status\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_status`] module"]
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
#[doc = "LP UART transmit and receive status"]
pub mod fsm_status;
#[doc = "CLK_CONF (rw) register accessor: LP UART core clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "LP UART core clock configuration"]
pub mod clk_conf;
#[doc = "DATE (rw) register accessor: LP UART version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "LP UART version register"]
pub mod date;
#[doc = "AFIFO_STATUS (r) register accessor: LP UART asynchronous FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`afifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afifo_status`] module"]
pub type AFIFO_STATUS = crate::Reg<afifo_status::AFIFO_STATUS_SPEC>;
#[doc = "LP UART asynchronous FIFO Status"]
pub mod afifo_status;
#[doc = "REG_UPDATE (rw) register accessor: LP UART register configuration update register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_update::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_update::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_update`] module"]
pub type REG_UPDATE = crate::Reg<reg_update::REG_UPDATE_SPEC>;
#[doc = "LP UART register configuration update register"]
pub mod reg_update;
#[doc = "ID (rw) register accessor: LP UART ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "LP UART ID register"]
pub mod id;
