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
    pub clkdiv_sync: CLKDIV_SYNC,
    #[doc = "0x18 - Rx Filter configuration"]
    pub rx_filt: RX_FILT,
    #[doc = "0x1c - UART status register"]
    pub status: STATUS,
    #[doc = "0x20 - Configuration register 0"]
    pub conf0_sync: CONF0_SYNC,
    #[doc = "0x24 - Configuration register 1"]
    pub conf1: CONF1,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - Hardware flow-control configuration"]
    pub hwfc_conf_sync: HWFC_CONF_SYNC,
    #[doc = "0x30 - UART sleep configure register 0"]
    pub sleep_conf0: SLEEP_CONF0,
    #[doc = "0x34 - UART sleep configure register 1"]
    pub sleep_conf1: SLEEP_CONF1,
    #[doc = "0x38 - UART sleep configure register 2"]
    pub sleep_conf2: SLEEP_CONF2,
    #[doc = "0x3c - Software flow-control character configuration"]
    pub swfc_conf0_sync: SWFC_CONF0_SYNC,
    #[doc = "0x40 - Software flow-control character configuration"]
    pub swfc_conf1: SWFC_CONF1,
    #[doc = "0x44 - Tx Break character configuration"]
    pub txbrk_conf_sync: TXBRK_CONF_SYNC,
    #[doc = "0x48 - Frame-end idle configuration"]
    pub idle_conf_sync: IDLE_CONF_SYNC,
    #[doc = "0x4c - RS485 mode configuration"]
    pub rs485_conf_sync: RS485_CONF_SYNC,
    #[doc = "0x50 - Pre-sequence timing configuration"]
    pub at_cmd_precnt_sync: AT_CMD_PRECNT_SYNC,
    #[doc = "0x54 - Post-sequence timing configuration"]
    pub at_cmd_postcnt_sync: AT_CMD_POSTCNT_SYNC,
    #[doc = "0x58 - Timeout configuration"]
    pub at_cmd_gaptout_sync: AT_CMD_GAPTOUT_SYNC,
    #[doc = "0x5c - AT escape sequence detection configuration"]
    pub at_cmd_char_sync: AT_CMD_CHAR_SYNC,
    #[doc = "0x60 - UART memory power configuration"]
    pub mem_conf: MEM_CONF,
    #[doc = "0x64 - UART threshold and allocation configuration"]
    pub tout_conf_sync: TOUT_CONF_SYNC,
    #[doc = "0x68 - Tx-SRAM write and read offset address."]
    pub mem_tx_status: MEM_TX_STATUS,
    #[doc = "0x6c - Rx-SRAM write and read offset address."]
    pub mem_rx_status: MEM_RX_STATUS,
    #[doc = "0x70 - UART transmit and receive status."]
    pub fsm_status: FSM_STATUS,
    _reserved28: [u8; 0x14],
    #[doc = "0x88 - UART core clock configuration"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x8c - UART Version register"]
    pub date: DATE,
    #[doc = "0x90 - UART AFIFO Status"]
    pub afifo_status: AFIFO_STATUS,
    _reserved31: [u8; 0x04],
    #[doc = "0x98 - UART Registers Configuration Update register"]
    pub reg_update: REG_UPDATE,
    #[doc = "0x9c - UART ID register"]
    pub id: ID,
}
#[doc = "FIFO (r) register accessor: FIFO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`] module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO data register"]
pub mod fifo;
#[doc = "INT_RAW (rw) register accessor: Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
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
#[doc = "CLKDIV_SYNC (rw) register accessor: Clock divider configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkdiv_sync`] module"]
pub type CLKDIV_SYNC = crate::Reg<clkdiv_sync::CLKDIV_SYNC_SPEC>;
#[doc = "Clock divider configuration"]
pub mod clkdiv_sync;
#[doc = "RX_FILT (rw) register accessor: Rx Filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_filt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_filt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_filt`] module"]
pub type RX_FILT = crate::Reg<rx_filt::RX_FILT_SPEC>;
#[doc = "Rx Filter configuration"]
pub mod rx_filt;
#[doc = "STATUS (r) register accessor: UART status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "UART status register"]
pub mod status;
#[doc = "CONF0_SYNC (rw) register accessor: Configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf0_sync`] module"]
pub type CONF0_SYNC = crate::Reg<conf0_sync::CONF0_SYNC_SPEC>;
#[doc = "Configuration register 0"]
pub mod conf0_sync;
#[doc = "CONF1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Configuration register 1"]
pub mod conf1;
#[doc = "HWFC_CONF_SYNC (rw) register accessor: Hardware flow-control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwfc_conf_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwfc_conf_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hwfc_conf_sync`] module"]
pub type HWFC_CONF_SYNC = crate::Reg<hwfc_conf_sync::HWFC_CONF_SYNC_SPEC>;
#[doc = "Hardware flow-control configuration"]
pub mod hwfc_conf_sync;
#[doc = "SLEEP_CONF0 (rw) register accessor: UART sleep configure register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sleep_conf0`] module"]
pub type SLEEP_CONF0 = crate::Reg<sleep_conf0::SLEEP_CONF0_SPEC>;
#[doc = "UART sleep configure register 0"]
pub mod sleep_conf0;
#[doc = "SLEEP_CONF1 (rw) register accessor: UART sleep configure register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sleep_conf1`] module"]
pub type SLEEP_CONF1 = crate::Reg<sleep_conf1::SLEEP_CONF1_SPEC>;
#[doc = "UART sleep configure register 1"]
pub mod sleep_conf1;
#[doc = "SLEEP_CONF2 (rw) register accessor: UART sleep configure register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sleep_conf2`] module"]
pub type SLEEP_CONF2 = crate::Reg<sleep_conf2::SLEEP_CONF2_SPEC>;
#[doc = "UART sleep configure register 2"]
pub mod sleep_conf2;
#[doc = "SWFC_CONF0_SYNC (rw) register accessor: Software flow-control character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf0_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf0_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swfc_conf0_sync`] module"]
pub type SWFC_CONF0_SYNC = crate::Reg<swfc_conf0_sync::SWFC_CONF0_SYNC_SPEC>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf0_sync;
#[doc = "SWFC_CONF1 (rw) register accessor: Software flow-control character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swfc_conf1`] module"]
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
#[doc = "Software flow-control character configuration"]
pub mod swfc_conf1;
#[doc = "TXBRK_CONF_SYNC (rw) register accessor: Tx Break character configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrk_conf_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbrk_conf_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txbrk_conf_sync`] module"]
pub type TXBRK_CONF_SYNC = crate::Reg<txbrk_conf_sync::TXBRK_CONF_SYNC_SPEC>;
#[doc = "Tx Break character configuration"]
pub mod txbrk_conf_sync;
#[doc = "IDLE_CONF_SYNC (rw) register accessor: Frame-end idle configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_conf_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_conf_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idle_conf_sync`] module"]
pub type IDLE_CONF_SYNC = crate::Reg<idle_conf_sync::IDLE_CONF_SYNC_SPEC>;
#[doc = "Frame-end idle configuration"]
pub mod idle_conf_sync;
#[doc = "RS485_CONF_SYNC (rw) register accessor: RS485 mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rs485_conf_sync`] module"]
pub type RS485_CONF_SYNC = crate::Reg<rs485_conf_sync::RS485_CONF_SYNC_SPEC>;
#[doc = "RS485 mode configuration"]
pub mod rs485_conf_sync;
#[doc = "AT_CMD_PRECNT_SYNC (rw) register accessor: Pre-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_precnt_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_precnt_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_precnt_sync`] module"]
pub type AT_CMD_PRECNT_SYNC = crate::Reg<at_cmd_precnt_sync::AT_CMD_PRECNT_SYNC_SPEC>;
#[doc = "Pre-sequence timing configuration"]
pub mod at_cmd_precnt_sync;
#[doc = "AT_CMD_POSTCNT_SYNC (rw) register accessor: Post-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_postcnt_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_postcnt_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_postcnt_sync`] module"]
pub type AT_CMD_POSTCNT_SYNC = crate::Reg<at_cmd_postcnt_sync::AT_CMD_POSTCNT_SYNC_SPEC>;
#[doc = "Post-sequence timing configuration"]
pub mod at_cmd_postcnt_sync;
#[doc = "AT_CMD_GAPTOUT_SYNC (rw) register accessor: Timeout configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_gaptout_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_gaptout_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_gaptout_sync`] module"]
pub type AT_CMD_GAPTOUT_SYNC = crate::Reg<at_cmd_gaptout_sync::AT_CMD_GAPTOUT_SYNC_SPEC>;
#[doc = "Timeout configuration"]
pub mod at_cmd_gaptout_sync;
#[doc = "AT_CMD_CHAR_SYNC (rw) register accessor: AT escape sequence detection configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`at_cmd_char_sync`] module"]
pub type AT_CMD_CHAR_SYNC = crate::Reg<at_cmd_char_sync::AT_CMD_CHAR_SYNC_SPEC>;
#[doc = "AT escape sequence detection configuration"]
pub mod at_cmd_char_sync;
#[doc = "MEM_CONF (rw) register accessor: UART memory power configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_conf`] module"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "UART memory power configuration"]
pub mod mem_conf;
#[doc = "TOUT_CONF_SYNC (rw) register accessor: UART threshold and allocation configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tout_conf_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tout_conf_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tout_conf_sync`] module"]
pub type TOUT_CONF_SYNC = crate::Reg<tout_conf_sync::TOUT_CONF_SYNC_SPEC>;
#[doc = "UART threshold and allocation configuration"]
pub mod tout_conf_sync;
#[doc = "MEM_TX_STATUS (r) register accessor: Tx-SRAM write and read offset address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_tx_status`] module"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = "Tx-SRAM write and read offset address."]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS (r) register accessor: Rx-SRAM write and read offset address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_rx_status`] module"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = "Rx-SRAM write and read offset address."]
pub mod mem_rx_status;
#[doc = "FSM_STATUS (r) register accessor: UART transmit and receive status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fsm_status`] module"]
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
#[doc = "UART transmit and receive status."]
pub mod fsm_status;
#[doc = "CLK_CONF (rw) register accessor: UART core clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_conf`] module"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "UART core clock configuration"]
pub mod clk_conf;
#[doc = "DATE (rw) register accessor: UART Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UART Version register"]
pub mod date;
#[doc = "AFIFO_STATUS (r) register accessor: UART AFIFO Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`afifo_status`] module"]
pub type AFIFO_STATUS = crate::Reg<afifo_status::AFIFO_STATUS_SPEC>;
#[doc = "UART AFIFO Status"]
pub mod afifo_status;
#[doc = "REG_UPDATE (rw) register accessor: UART Registers Configuration Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg_update`] module"]
pub type REG_UPDATE = crate::Reg<reg_update::REG_UPDATE_SPEC>;
#[doc = "UART Registers Configuration Update register"]
pub mod reg_update;
#[doc = "ID (rw) register accessor: UART ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "UART ID register"]
pub mod id;
