#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster OUT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK1_CH?, OUT_LINK2_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?, OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?
pub struct OUT_CH {
    out_int: OUT_INT,
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
    crc: CRC,
}
impl OUT_CH {
    ///0x00..0x10 - Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR
    #[inline(always)]
    pub const fn out_int(&self) -> &OUT_INT {
        &self.out_int
    }
    ///0x10 - Configure 0 register of Tx channel0
    #[inline(always)]
    pub const fn out_conf0(&self) -> &OUT_CONF0 {
        &self.out_conf0
    }
    ///0x14 - Configure 1 register of Tx channel0
    #[inline(always)]
    pub const fn out_conf1(&self) -> &OUT_CONF1 {
        &self.out_conf1
    }
    ///0x18 - Transmit FIFO status of Tx channel0
    #[inline(always)]
    pub const fn outfifo_status(&self) -> &OUTFIFO_STATUS {
        &self.outfifo_status
    }
    ///0x1c - Push control register of Tx channel0
    #[inline(always)]
    pub const fn out_push(&self) -> &OUT_PUSH {
        &self.out_push
    }
    ///0x20 - Link descriptor configure and control register of Tx channel0
    #[inline(always)]
    pub const fn out_link1(&self) -> &OUT_LINK1 {
        &self.out_link1
    }
    ///0x24 - Link descriptor configure and control register of Tx channel0
    #[inline(always)]
    pub const fn out_link2(&self) -> &OUT_LINK2 {
        &self.out_link2
    }
    ///0x28 - Transmit status of Tx channel0
    #[inline(always)]
    pub const fn out_state(&self) -> &OUT_STATE {
        &self.out_state
    }
    ///0x2c - Outlink descriptor address when EOF occurs of Tx channel0
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    ///0x30 - The last outlink descriptor address when EOF occurs of Tx channel0
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    ///0x34 - Current outlink descriptor address of Tx channel0
    #[inline(always)]
    pub const fn out_dscr(&self) -> &OUT_DSCR {
        &self.out_dscr
    }
    ///0x38 - The last outlink descriptor address of Tx channel0
    #[inline(always)]
    pub const fn out_dscr_bf0(&self) -> &OUT_DSCR_BF0 {
        &self.out_dscr_bf0
    }
    ///0x3c - The second-to-last outlink descriptor address of Tx channel0
    #[inline(always)]
    pub const fn out_dscr_bf1(&self) -> &OUT_DSCR_BF1 {
        &self.out_dscr_bf1
    }
    ///0x40 - Priority register of Tx channel0.
    #[inline(always)]
    pub const fn out_pri(&self) -> &OUT_PRI {
        &self.out_pri
    }
    ///0x44 - Peripheral selection of Tx channel0
    #[inline(always)]
    pub const fn out_peri_sel(&self) -> &OUT_PERI_SEL {
        &self.out_peri_sel
    }
    ///0x48..0x68 - Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR
    #[inline(always)]
    pub const fn crc(&self) -> &CRC {
        &self.crc
    }
}
///Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR
pub use self::out_int::OUT_INT;
///Cluster
///Cluster OUT_INT, containing OUT_INT_RAW, OUT_INT_ST, OUT_INT_ENA, OUT_INT_CLR
pub mod out_int;
/**OUT_CONF0 (rw) register accessor: Configure 0 register of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_conf0`] module*/
pub type OUT_CONF0 = crate::Reg<out_conf0::OUT_CONF0_SPEC>;
///Configure 0 register of Tx channel0
pub mod out_conf0;
/**OUT_CONF1 (rw) register accessor: Configure 1 register of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_conf1`] module*/
pub type OUT_CONF1 = crate::Reg<out_conf1::OUT_CONF1_SPEC>;
///Configure 1 register of Tx channel0
pub mod out_conf1;
/**OUTFIFO_STATUS (r) register accessor: Transmit FIFO status of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outfifo_status`] module*/
pub type OUTFIFO_STATUS = crate::Reg<outfifo_status::OUTFIFO_STATUS_SPEC>;
///Transmit FIFO status of Tx channel0
pub mod outfifo_status;
/**OUT_PUSH (rw) register accessor: Push control register of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_push`] module*/
pub type OUT_PUSH = crate::Reg<out_push::OUT_PUSH_SPEC>;
///Push control register of Tx channel0
pub mod out_push;
/**OUT_LINK1 (rw) register accessor: Link descriptor configure and control register of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_link1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_link1`] module*/
pub type OUT_LINK1 = crate::Reg<out_link1::OUT_LINK1_SPEC>;
///Link descriptor configure and control register of Tx channel0
pub mod out_link1;
/**OUT_LINK2 (rw) register accessor: Link descriptor configure and control register of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_link2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_link2`] module*/
pub type OUT_LINK2 = crate::Reg<out_link2::OUT_LINK2_SPEC>;
///Link descriptor configure and control register of Tx channel0
pub mod out_link2;
/**OUT_STATE (r) register accessor: Transmit status of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_state`] module*/
pub type OUT_STATE = crate::Reg<out_state::OUT_STATE_SPEC>;
///Transmit status of Tx channel0
pub mod out_state;
/**OUT_EOF_DES_ADDR (r) register accessor: Outlink descriptor address when EOF occurs of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_des_addr`] module*/
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
///Outlink descriptor address when EOF occurs of Tx channel0
pub mod out_eof_des_addr;
/**OUT_EOF_BFR_DES_ADDR (r) register accessor: The last outlink descriptor address when EOF occurs of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_bfr_des_addr`] module*/
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
///The last outlink descriptor address when EOF occurs of Tx channel0
pub mod out_eof_bfr_des_addr;
/**OUT_DSCR (r) register accessor: Current outlink descriptor address of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_dscr`] module*/
pub type OUT_DSCR = crate::Reg<out_dscr::OUT_DSCR_SPEC>;
///Current outlink descriptor address of Tx channel0
pub mod out_dscr;
/**OUT_DSCR_BF0 (r) register accessor: The last outlink descriptor address of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_dscr_bf0`] module*/
pub type OUT_DSCR_BF0 = crate::Reg<out_dscr_bf0::OUT_DSCR_BF0_SPEC>;
///The last outlink descriptor address of Tx channel0
pub mod out_dscr_bf0;
/**OUT_DSCR_BF1 (r) register accessor: The second-to-last outlink descriptor address of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_dscr_bf1`] module*/
pub type OUT_DSCR_BF1 = crate::Reg<out_dscr_bf1::OUT_DSCR_BF1_SPEC>;
///The second-to-last outlink descriptor address of Tx channel0
pub mod out_dscr_bf1;
/**OUT_PRI (rw) register accessor: Priority register of Tx channel0.

You can [`read`](crate::generic::Reg::read) this register and get [`out_pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_pri`] module*/
pub type OUT_PRI = crate::Reg<out_pri::OUT_PRI_SPEC>;
///Priority register of Tx channel0.
pub mod out_pri;
/**OUT_PERI_SEL (rw) register accessor: Peripheral selection of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_peri_sel`] module*/
pub type OUT_PERI_SEL = crate::Reg<out_peri_sel::OUT_PERI_SEL_SPEC>;
///Peripheral selection of Tx channel0
pub mod out_peri_sel;
///Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR
pub use self::crc::CRC;
///Cluster
///Cluster CRC, containing OUT_CRC_INIT_DATA, TX_CRC_WIDTH, OUT_CRC_CLEAR, OUT_CRC_FINAL_RESULT, TX_CRC_EN_WR_DATA, TX_CRC_EN_ADDR, TX_CRC_DATA_EN_WR_DATA, TX_CRC_DATA_EN_ADDR
pub mod crc;
