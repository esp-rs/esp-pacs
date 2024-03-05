#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub struct IN_CH {
    in_int_raw: IN_INT_RAW,
    in_int_st: IN_INT_ST,
    in_int_ena: IN_INT_ENA,
    in_int_clr: IN_INT_CLR,
    in_conf0: IN_CONF0,
    in_conf1: IN_CONF1,
    infifo_status: INFIFO_STATUS,
    in_pop: IN_POP,
    in_link1: IN_LINK1,
    in_link2: IN_LINK2,
    in_state: IN_STATE,
    in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    in_dscr: IN_DSCR,
    in_dscr_bf0: IN_DSCR_BF0,
    in_dscr_bf1: IN_DSCR_BF1,
    in_pri: IN_PRI,
    in_peri_sel: IN_PERI_SEL,
    in_crc_init_data: IN_CRC_INIT_DATA,
    rx_crc_width: RX_CRC_WIDTH,
    in_crc_clear: IN_CRC_CLEAR,
    in_crc_final_result: IN_CRC_FINAL_RESULT,
    rx_crc_en_wr_data: RX_CRC_EN_WR_DATA,
    rx_crc_en_addr: RX_CRC_EN_ADDR,
    rx_crc_data_en_wr_data: RX_CRC_DATA_EN_WR_DATA,
    rx_crc_data_en_addr: RX_CRC_DATA_EN_ADDR,
}
impl IN_CH {
    #[doc = "0x00 - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_raw(&self) -> &IN_INT_RAW {
        &self.in_int_raw
    }
    #[doc = "0x04 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_st(&self) -> &IN_INT_ST {
        &self.in_int_st
    }
    #[doc = "0x08 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_ena(&self) -> &IN_INT_ENA {
        &self.in_int_ena
    }
    #[doc = "0x0c - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_clr(&self) -> &IN_INT_CLR {
        &self.in_int_clr
    }
    #[doc = "0x10 - Configure 0 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf0(&self) -> &IN_CONF0 {
        &self.in_conf0
    }
    #[doc = "0x14 - Configure 1 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf1(&self) -> &IN_CONF1 {
        &self.in_conf1
    }
    #[doc = "0x18 - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status(&self) -> &INFIFO_STATUS {
        &self.infifo_status
    }
    #[doc = "0x1c - Pop control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pop(&self) -> &IN_POP {
        &self.in_pop
    }
    #[doc = "0x20 - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link1(&self) -> &IN_LINK1 {
        &self.in_link1
    }
    #[doc = "0x24 - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link2(&self) -> &IN_LINK2 {
        &self.in_link2
    }
    #[doc = "0x28 - Receive status of Rx channel 0"]
    #[inline(always)]
    pub const fn in_state(&self) -> &IN_STATE {
        &self.in_state
    }
    #[doc = "0x2c - Inlink descriptor address when EOF occurs of Rx channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    #[doc = "0x30 - Inlink descriptor address when errors occur of Rx channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    #[doc = "0x34 - Current inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr(&self) -> &IN_DSCR {
        &self.in_dscr
    }
    #[doc = "0x38 - The last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0(&self) -> &IN_DSCR_BF0 {
        &self.in_dscr_bf0
    }
    #[doc = "0x3c - The second-to-last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1(&self) -> &IN_DSCR_BF1 {
        &self.in_dscr_bf1
    }
    #[doc = "0x40 - Priority register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pri(&self) -> &IN_PRI {
        &self.in_pri
    }
    #[doc = "0x44 - Peripheral selection of Rx channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel(&self) -> &IN_PERI_SEL {
        &self.in_peri_sel
    }
    #[doc = "0x48 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data(&self) -> &IN_CRC_INIT_DATA {
        &self.in_crc_init_data
    }
    #[doc = "0x4c - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn rx_crc_width(&self) -> &RX_CRC_WIDTH {
        &self.rx_crc_width
    }
    #[doc = "0x50 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_clear(&self) -> &IN_CRC_CLEAR {
        &self.in_crc_clear
    }
    #[doc = "0x54 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result(&self) -> &IN_CRC_FINAL_RESULT {
        &self.in_crc_final_result
    }
    #[doc = "0x58 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data(&self) -> &RX_CRC_EN_WR_DATA {
        &self.rx_crc_en_wr_data
    }
    #[doc = "0x5c - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr(&self) -> &RX_CRC_EN_ADDR {
        &self.rx_crc_en_addr
    }
    #[doc = "0x60 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data(&self) -> &RX_CRC_DATA_EN_WR_DATA {
        &self.rx_crc_data_en_wr_data
    }
    #[doc = "0x64 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_addr(&self) -> &RX_CRC_DATA_EN_ADDR {
        &self.rx_crc_data_en_addr
    }
}
#[doc = "IN_INT_RAW (rw) register accessor: Raw status interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw`] module"]
pub type IN_INT_RAW = crate::Reg<in_int_raw::IN_INT_RAW_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod in_int_raw;
#[doc = "IN_INT_ST (r) register accessor: Masked interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st`] module"]
pub type IN_INT_ST = crate::Reg<in_int_st::IN_INT_ST_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod in_int_st;
#[doc = "IN_INT_ENA (rw) register accessor: Interrupt enable bits of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena`] module"]
pub type IN_INT_ENA = crate::Reg<in_int_ena::IN_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod in_int_ena;
#[doc = "IN_INT_CLR (w) register accessor: Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr`] module"]
pub type IN_INT_CLR = crate::Reg<in_int_clr::IN_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod in_int_clr;
#[doc = "IN_CONF0 (rw) register accessor: Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0`] module"]
pub type IN_CONF0 = crate::Reg<in_conf0::IN_CONF0_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0;
#[doc = "IN_CONF1 (rw) register accessor: Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1`] module"]
pub type IN_CONF1 = crate::Reg<in_conf1::IN_CONF1_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1;
#[doc = "INFIFO_STATUS (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status`] module"]
pub type INFIFO_STATUS = crate::Reg<infifo_status::INFIFO_STATUS_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status;
#[doc = "IN_POP (rw) register accessor: Pop control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop`] module"]
pub type IN_POP = crate::Reg<in_pop::IN_POP_SPEC>;
#[doc = "Pop control register of Rx channel 0"]
pub mod in_pop;
#[doc = "IN_LINK1 (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link1`] module"]
pub type IN_LINK1 = crate::Reg<in_link1::IN_LINK1_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link1;
#[doc = "IN_LINK2 (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link2`] module"]
pub type IN_LINK2 = crate::Reg<in_link2::IN_LINK2_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link2;
#[doc = "IN_STATE (r) register accessor: Receive status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state`] module"]
pub type IN_STATE = crate::Reg<in_state::IN_STATE_SPEC>;
#[doc = "Receive status of Rx channel 0"]
pub mod in_state;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: Inlink descriptor address when EOF occurs of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0"]
pub mod in_suc_eof_des_addr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: Inlink descriptor address when errors occur of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when errors occur of Rx channel 0"]
pub mod in_err_eof_des_addr;
#[doc = "IN_DSCR (r) register accessor: Current inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr`] module"]
pub type IN_DSCR = crate::Reg<in_dscr::IN_DSCR_SPEC>;
#[doc = "Current inlink descriptor address of Rx channel 0"]
pub mod in_dscr;
#[doc = "IN_DSCR_BF0 (r) register accessor: The last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0`] module"]
pub type IN_DSCR_BF0 = crate::Reg<in_dscr_bf0::IN_DSCR_BF0_SPEC>;
#[doc = "The last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf0;
#[doc = "IN_DSCR_BF1 (r) register accessor: The second-to-last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1`] module"]
pub type IN_DSCR_BF1 = crate::Reg<in_dscr_bf1::IN_DSCR_BF1_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf1;
#[doc = "IN_PRI (rw) register accessor: Priority register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri`] module"]
pub type IN_PRI = crate::Reg<in_pri::IN_PRI_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri;
#[doc = "IN_PERI_SEL (rw) register accessor: Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel`] module"]
pub type IN_PERI_SEL = crate::Reg<in_peri_sel::IN_PERI_SEL_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel;
#[doc = "IN_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_init_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_init_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_init_data`] module"]
pub type IN_CRC_INIT_DATA = crate::Reg<in_crc_init_data::IN_CRC_INIT_DATA_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod in_crc_init_data;
#[doc = "RX_CRC_WIDTH (rw) register accessor: This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_width`] module"]
pub type RX_CRC_WIDTH = crate::Reg<rx_crc_width::RX_CRC_WIDTH_SPEC>;
#[doc = "This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
pub mod rx_crc_width;
#[doc = "IN_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_clear`] module"]
pub type IN_CRC_CLEAR = crate::Reg<in_crc_clear::IN_CRC_CLEAR_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod in_crc_clear;
#[doc = "IN_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_final_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_final_result`] module"]
pub type IN_CRC_FINAL_RESULT = crate::Reg<in_crc_final_result::IN_CRC_FINAL_RESULT_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod in_crc_final_result;
#[doc = "RX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_wr_data`] module"]
pub type RX_CRC_EN_WR_DATA = crate::Reg<rx_crc_en_wr_data::RX_CRC_EN_WR_DATA_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod rx_crc_en_wr_data;
#[doc = "RX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_addr`] module"]
pub type RX_CRC_EN_ADDR = crate::Reg<rx_crc_en_addr::RX_CRC_EN_ADDR_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod rx_crc_en_addr;
#[doc = "RX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_wr_data`] module"]
pub type RX_CRC_DATA_EN_WR_DATA = crate::Reg<rx_crc_data_en_wr_data::RX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod rx_crc_data_en_wr_data;
#[doc = "RX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_addr`] module"]
pub type RX_CRC_DATA_EN_ADDR = crate::Reg<rx_crc_data_en_addr::RX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod rx_crc_data_en_addr;
