#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster IN_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK1_CH?, IN_LINK2_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, IN_CRC_INIT_DATA_CH?, RX_CRC_WIDTH_CH?, IN_CRC_CLEAR_CH?, IN_CRC_FINAL_RESULT_CH?, RX_CRC_EN_WR_DATA_CH?, RX_CRC_EN_ADDR_CH?, RX_CRC_DATA_EN_WR_DATA_CH?, RX_CRC_DATA_EN_ADDR_CH?"]
pub struct IN_CH {
    in_int: IN_INT,
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
    crc: CRC,
}
impl IN_CH {
    #[doc = "0x00..0x10 - Cluster IN_INT, containing IN_INT_RAW, IN_INT_ST, IN_INT_ENA, IN_INT_CLR"]
    #[inline(always)]
    pub const fn in_int(&self) -> &IN_INT {
        &self.in_int
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
    #[doc = "0x48..0x68 - Cluster CRC, containing IN_CRC_INIT_DATA, RX_CRC_WIDTH, IN_CRC_CLEAR, IN_CRC_FINAL_RESULT, RX_CRC_EN_WR_DATA, RX_CRC_EN_ADDR, RX_CRC_DATA_EN_WR_DATA, RX_CRC_DATA_EN_ADDR"]
    #[inline(always)]
    pub const fn crc(&self) -> &CRC {
        &self.crc
    }
}
#[doc = "Cluster IN_INT, containing IN_INT_RAW, IN_INT_ST, IN_INT_ENA, IN_INT_CLR"]
pub use self::in_int::IN_INT;
#[doc = r"Cluster"]
#[doc = "Cluster IN_INT, containing IN_INT_RAW, IN_INT_ST, IN_INT_ENA, IN_INT_CLR"]
pub mod in_int;
#[doc = "IN_CONF0 (rw) register accessor: Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0`] module"]
pub type IN_CONF0 = crate::Reg<in_conf0::IN_CONF0_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0;
#[doc = "IN_CONF1 (rw) register accessor: Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1`] module"]
pub type IN_CONF1 = crate::Reg<in_conf1::IN_CONF1_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1;
#[doc = "INFIFO_STATUS (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status`] module"]
pub type INFIFO_STATUS = crate::Reg<infifo_status::INFIFO_STATUS_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status;
#[doc = "IN_POP (rw) register accessor: Pop control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop`] module"]
pub type IN_POP = crate::Reg<in_pop::IN_POP_SPEC>;
#[doc = "Pop control register of Rx channel 0"]
pub mod in_pop;
#[doc = "IN_LINK1 (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link1`] module"]
pub type IN_LINK1 = crate::Reg<in_link1::IN_LINK1_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link1;
#[doc = "IN_LINK2 (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link2`] module"]
pub type IN_LINK2 = crate::Reg<in_link2::IN_LINK2_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link2;
#[doc = "IN_STATE (r) register accessor: Receive status of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state`] module"]
pub type IN_STATE = crate::Reg<in_state::IN_STATE_SPEC>;
#[doc = "Receive status of Rx channel 0"]
pub mod in_state;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: Inlink descriptor address when EOF occurs of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0"]
pub mod in_suc_eof_des_addr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: Inlink descriptor address when errors occur of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Inlink descriptor address when errors occur of Rx channel 0"]
pub mod in_err_eof_des_addr;
#[doc = "IN_DSCR (r) register accessor: Current inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr`] module"]
pub type IN_DSCR = crate::Reg<in_dscr::IN_DSCR_SPEC>;
#[doc = "Current inlink descriptor address of Rx channel 0"]
pub mod in_dscr;
#[doc = "IN_DSCR_BF0 (r) register accessor: The last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0`] module"]
pub type IN_DSCR_BF0 = crate::Reg<in_dscr_bf0::IN_DSCR_BF0_SPEC>;
#[doc = "The last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf0;
#[doc = "IN_DSCR_BF1 (r) register accessor: The second-to-last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1`] module"]
pub type IN_DSCR_BF1 = crate::Reg<in_dscr_bf1::IN_DSCR_BF1_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf1;
#[doc = "IN_PRI (rw) register accessor: Priority register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri`] module"]
pub type IN_PRI = crate::Reg<in_pri::IN_PRI_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri;
#[doc = "IN_PERI_SEL (rw) register accessor: Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel`] module"]
pub type IN_PERI_SEL = crate::Reg<in_peri_sel::IN_PERI_SEL_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel;
#[doc = "Cluster CRC, containing IN_CRC_INIT_DATA, RX_CRC_WIDTH, IN_CRC_CLEAR, IN_CRC_FINAL_RESULT, RX_CRC_EN_WR_DATA, RX_CRC_EN_ADDR, RX_CRC_DATA_EN_WR_DATA, RX_CRC_DATA_EN_ADDR"]
pub use self::crc::CRC;
#[doc = r"Cluster"]
#[doc = "Cluster CRC, containing IN_CRC_INIT_DATA, RX_CRC_WIDTH, IN_CRC_CLEAR, IN_CRC_FINAL_RESULT, RX_CRC_EN_WR_DATA, RX_CRC_EN_ADDR, RX_CRC_DATA_EN_WR_DATA, RX_CRC_DATA_EN_ADDR"]
pub mod crc;
