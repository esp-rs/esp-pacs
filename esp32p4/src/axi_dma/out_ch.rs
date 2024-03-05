#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?"]
pub struct OUT_CH {
    out_int_raw: OUT_INT_RAW,
    out_int_st: OUT_INT_ST,
    out_int_ena: OUT_INT_ENA,
    out_int_clr: OUT_INT_CLR,
    out_conf0: OUT_CONF0,
    out_conf1: OUT_CONF1,
    outfifo_status: OUTFIFO_STATUS,
    out_push: OUT_PUSH,
    out_link1: OUT_LINK1,
    out_link2: OUT_LINK2,
    out_state: OUT_STATE,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    out_dscr: OUT_DSCR,
    out_dscr_bf0: OUT_DSCR_BF0,
    out_dscr_bf1: OUT_DSCR_BF1,
    out_pri: OUT_PRI,
    out_peri_sel: OUT_PERI_SEL,
    out_crc_init_data: OUT_CRC_INIT_DATA,
    tx_crc_width: TX_CRC_WIDTH,
    out_crc_clear: OUT_CRC_CLEAR,
    out_crc_final_result: OUT_CRC_FINAL_RESULT,
    tx_crc_en_wr_data: TX_CRC_EN_WR_DATA,
    tx_crc_en_addr: TX_CRC_EN_ADDR,
    tx_crc_data_en_wr_data: TX_CRC_DATA_EN_WR_DATA,
    tx_crc_data_en_addr: TX_CRC_DATA_EN_ADDR,
}
impl OUT_CH {
    #[doc = "0x00 - Raw status interrupt of channel0"]
    #[inline(always)]
    pub const fn out_int_raw(&self) -> &OUT_INT_RAW {
        &self.out_int_raw
    }
    #[doc = "0x04 - Masked interrupt of channel0"]
    #[inline(always)]
    pub const fn out_int_st(&self) -> &OUT_INT_ST {
        &self.out_int_st
    }
    #[doc = "0x08 - Interrupt enable bits of channel0"]
    #[inline(always)]
    pub const fn out_int_ena(&self) -> &OUT_INT_ENA {
        &self.out_int_ena
    }
    #[doc = "0x0c - Interrupt clear bits of channel0"]
    #[inline(always)]
    pub const fn out_int_clr(&self) -> &OUT_INT_CLR {
        &self.out_int_clr
    }
    #[doc = "0x10 - Configure 0 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf0(&self) -> &OUT_CONF0 {
        &self.out_conf0
    }
    #[doc = "0x14 - Configure 1 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf1(&self) -> &OUT_CONF1 {
        &self.out_conf1
    }
    #[doc = "0x18 - Transmit FIFO status of Tx channel0"]
    #[inline(always)]
    pub const fn outfifo_status(&self) -> &OUTFIFO_STATUS {
        &self.outfifo_status
    }
    #[doc = "0x1c - Push control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_push(&self) -> &OUT_PUSH {
        &self.out_push
    }
    #[doc = "0x20 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link1(&self) -> &OUT_LINK1 {
        &self.out_link1
    }
    #[doc = "0x24 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link2(&self) -> &OUT_LINK2 {
        &self.out_link2
    }
    #[doc = "0x28 - Transmit status of Tx channel0"]
    #[inline(always)]
    pub const fn out_state(&self) -> &OUT_STATE {
        &self.out_state
    }
    #[doc = "0x2c - Outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x30 - The last outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x34 - Current outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr(&self) -> &OUT_DSCR {
        &self.out_dscr
    }
    #[doc = "0x38 - The last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf0(&self) -> &OUT_DSCR_BF0 {
        &self.out_dscr_bf0
    }
    #[doc = "0x3c - The second-to-last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf1(&self) -> &OUT_DSCR_BF1 {
        &self.out_dscr_bf1
    }
    #[doc = "0x40 - Priority register of Tx channel0."]
    #[inline(always)]
    pub const fn out_pri(&self) -> &OUT_PRI {
        &self.out_pri
    }
    #[doc = "0x44 - Peripheral selection of Tx channel0"]
    #[inline(always)]
    pub const fn out_peri_sel(&self) -> &OUT_PERI_SEL {
        &self.out_peri_sel
    }
    #[doc = "0x48 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data(&self) -> &OUT_CRC_INIT_DATA {
        &self.out_crc_init_data
    }
    #[doc = "0x4c - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn tx_crc_width(&self) -> &TX_CRC_WIDTH {
        &self.tx_crc_width
    }
    #[doc = "0x50 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_clear(&self) -> &OUT_CRC_CLEAR {
        &self.out_crc_clear
    }
    #[doc = "0x54 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_final_result(&self) -> &OUT_CRC_FINAL_RESULT {
        &self.out_crc_final_result
    }
    #[doc = "0x58 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn tx_crc_en_wr_data(&self) -> &TX_CRC_EN_WR_DATA {
        &self.tx_crc_en_wr_data
    }
    #[doc = "0x5c - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn tx_crc_en_addr(&self) -> &TX_CRC_EN_ADDR {
        &self.tx_crc_en_addr
    }
    #[doc = "0x60 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data(&self) -> &TX_CRC_DATA_EN_WR_DATA {
        &self.tx_crc_data_en_wr_data
    }
    #[doc = "0x64 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_addr(&self) -> &TX_CRC_DATA_EN_ADDR {
        &self.tx_crc_data_en_addr
    }
}
#[doc = "OUT_INT_RAW (rw) register accessor: Raw status interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw`] module"]
pub type OUT_INT_RAW = crate::Reg<out_int_raw::OUT_INT_RAW_SPEC>;
#[doc = "Raw status interrupt of channel0"]
pub mod out_int_raw;
#[doc = "OUT_INT_ST (r) register accessor: Masked interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st`] module"]
pub type OUT_INT_ST = crate::Reg<out_int_st::OUT_INT_ST_SPEC>;
#[doc = "Masked interrupt of channel0"]
pub mod out_int_st;
#[doc = "OUT_INT_ENA (rw) register accessor: Interrupt enable bits of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena`] module"]
pub type OUT_INT_ENA = crate::Reg<out_int_ena::OUT_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of channel0"]
pub mod out_int_ena;
#[doc = "OUT_INT_CLR (w) register accessor: Interrupt clear bits of channel0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr`] module"]
pub type OUT_INT_CLR = crate::Reg<out_int_clr::OUT_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of channel0"]
pub mod out_int_clr;
#[doc = "OUT_CONF0 (rw) register accessor: Configure 0 register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0`] module"]
pub type OUT_CONF0 = crate::Reg<out_conf0::OUT_CONF0_SPEC>;
#[doc = "Configure 0 register of Tx channel0"]
pub mod out_conf0;
#[doc = "OUT_CONF1 (rw) register accessor: Configure 1 register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1`] module"]
pub type OUT_CONF1 = crate::Reg<out_conf1::OUT_CONF1_SPEC>;
#[doc = "Configure 1 register of Tx channel0"]
pub mod out_conf1;
#[doc = "OUTFIFO_STATUS (r) register accessor: Transmit FIFO status of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status`] module"]
pub type OUTFIFO_STATUS = crate::Reg<outfifo_status::OUTFIFO_STATUS_SPEC>;
#[doc = "Transmit FIFO status of Tx channel0"]
pub mod outfifo_status;
#[doc = "OUT_PUSH (rw) register accessor: Push control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push`] module"]
pub type OUT_PUSH = crate::Reg<out_push::OUT_PUSH_SPEC>;
#[doc = "Push control register of Tx channel0"]
pub mod out_push;
#[doc = "OUT_LINK1 (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link1`] module"]
pub type OUT_LINK1 = crate::Reg<out_link1::OUT_LINK1_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link1;
#[doc = "OUT_LINK2 (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link2`] module"]
pub type OUT_LINK2 = crate::Reg<out_link2::OUT_LINK2_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link2;
#[doc = "OUT_STATE (r) register accessor: Transmit status of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state`] module"]
pub type OUT_STATE = crate::Reg<out_state::OUT_STATE_SPEC>;
#[doc = "Transmit status of Tx channel0"]
pub mod out_state;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: Outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: The last outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_DSCR (r) register accessor: Current outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr`] module"]
pub type OUT_DSCR = crate::Reg<out_dscr::OUT_DSCR_SPEC>;
#[doc = "Current outlink descriptor address of Tx channel0"]
pub mod out_dscr;
#[doc = "OUT_DSCR_BF0 (r) register accessor: The last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0`] module"]
pub type OUT_DSCR_BF0 = crate::Reg<out_dscr_bf0::OUT_DSCR_BF0_SPEC>;
#[doc = "The last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf0;
#[doc = "OUT_DSCR_BF1 (r) register accessor: The second-to-last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1`] module"]
pub type OUT_DSCR_BF1 = crate::Reg<out_dscr_bf1::OUT_DSCR_BF1_SPEC>;
#[doc = "The second-to-last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf1;
#[doc = "OUT_PRI (rw) register accessor: Priority register of Tx channel0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri`] module"]
pub type OUT_PRI = crate::Reg<out_pri::OUT_PRI_SPEC>;
#[doc = "Priority register of Tx channel0."]
pub mod out_pri;
#[doc = "OUT_PERI_SEL (rw) register accessor: Peripheral selection of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel`] module"]
pub type OUT_PERI_SEL = crate::Reg<out_peri_sel::OUT_PERI_SEL_SPEC>;
#[doc = "Peripheral selection of Tx channel0"]
pub mod out_peri_sel;
#[doc = "OUT_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_init_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_init_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_init_data`] module"]
pub type OUT_CRC_INIT_DATA = crate::Reg<out_crc_init_data::OUT_CRC_INIT_DATA_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod out_crc_init_data;
#[doc = "TX_CRC_WIDTH (rw) register accessor: This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_width`] module"]
pub type TX_CRC_WIDTH = crate::Reg<tx_crc_width::TX_CRC_WIDTH_SPEC>;
#[doc = "This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
pub mod tx_crc_width;
#[doc = "OUT_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_clear`] module"]
pub type OUT_CRC_CLEAR = crate::Reg<out_crc_clear::OUT_CRC_CLEAR_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod out_crc_clear;
#[doc = "OUT_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_final_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_final_result`] module"]
pub type OUT_CRC_FINAL_RESULT = crate::Reg<out_crc_final_result::OUT_CRC_FINAL_RESULT_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod out_crc_final_result;
#[doc = "TX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_wr_data`] module"]
pub type TX_CRC_EN_WR_DATA = crate::Reg<tx_crc_en_wr_data::TX_CRC_EN_WR_DATA_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod tx_crc_en_wr_data;
#[doc = "TX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_addr`] module"]
pub type TX_CRC_EN_ADDR = crate::Reg<tx_crc_en_addr::TX_CRC_EN_ADDR_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod tx_crc_en_addr;
#[doc = "TX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_wr_data`] module"]
pub type TX_CRC_DATA_EN_WR_DATA = crate::Reg<tx_crc_data_en_wr_data::TX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod tx_crc_data_en_wr_data;
#[doc = "TX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_addr`] module"]
pub type TX_CRC_DATA_EN_ADDR = crate::Reg<tx_crc_data_en_addr::TX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod tx_crc_data_en_addr;
