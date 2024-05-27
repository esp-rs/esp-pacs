#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    mosi_dlen: MOSI_DLEN,
    miso_dlen: MISO_DLEN,
    misc: MISC,
    slave: SLAVE,
    slave1: SLAVE1,
    slv_wrbuf_dlen: SLV_WRBUF_DLEN,
    slv_rdbuf_dlen: SLV_RDBUF_DLEN,
    slv_rd_byte: SLV_RD_BYTE,
    fsm: FSM,
    hold: HOLD,
    dma_conf: DMA_CONF,
    dma_out_link: DMA_OUT_LINK,
    dma_in_link: DMA_IN_LINK,
    dma_int_ena: DMA_INT_ENA,
    dma_int_raw: DMA_INT_RAW,
    dma_int_st: DMA_INT_ST,
    dma_int_clr: DMA_INT_CLR,
    in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    inlink_dscr: INLINK_DSCR,
    inlink_dscr_bf0: INLINK_DSCR_BF0,
    inlink_dscr_bf1: INLINK_DSCR_BF1,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    outlink_dscr: OUTLINK_DSCR,
    outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    dma_outstatus: DMA_OUTSTATUS,
    dma_instatus: DMA_INSTATUS,
    w: [W; 18],
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    dout_num: DOUT_NUM,
    lcd_ctrl: LCD_CTRL,
    lcd_ctrl1: LCD_CTRL1,
    lcd_ctrl2: LCD_CTRL2,
    lcd_d_mode: LCD_D_MODE,
    lcd_d_num: LCD_D_NUM,
    _reserved48: [u8; 0x02f8],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    ///0x00 - Command control register
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x04 - Address value
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    ///0x08 - SPI control register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI control register 1
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    ///0x10 - SPI control register 2
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x14 - SPI clock control register
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x18 - SPI USER control register
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x1c - SPI USER control register 1
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x20 - SPI USER control register 2
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x24 - MOSI length
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    ///0x28 - MISO length
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    ///0x2c - SPI misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x30 - SPI slave control register
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    ///0x34 - SPI slave control register 1
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    ///0x38 - SPI slave Wr_BUF interrupt and CONF control register
    #[inline(always)]
    pub const fn slv_wrbuf_dlen(&self) -> &SLV_WRBUF_DLEN {
        &self.slv_wrbuf_dlen
    }
    ///0x3c - SPI magic error and slave control register
    #[inline(always)]
    pub const fn slv_rdbuf_dlen(&self) -> &SLV_RDBUF_DLEN {
        &self.slv_rdbuf_dlen
    }
    ///0x40 - SPI interrupt control register
    #[inline(always)]
    pub const fn slv_rd_byte(&self) -> &SLV_RD_BYTE {
        &self.slv_rd_byte
    }
    ///0x44 - SPI master status and DMA read byte control register
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    ///0x48 - SPI hold register
    #[inline(always)]
    pub const fn hold(&self) -> &HOLD {
        &self.hold
    }
    ///0x4c - SPI DMA control register
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        &self.dma_conf
    }
    ///0x50 - SPI DMA TX link configuration
    #[inline(always)]
    pub const fn dma_out_link(&self) -> &DMA_OUT_LINK {
        &self.dma_out_link
    }
    ///0x54 - SPI DMA RX link configuration
    #[inline(always)]
    pub const fn dma_in_link(&self) -> &DMA_IN_LINK {
        &self.dma_in_link
    }
    ///0x58 - SPI DMA interrupt enable register
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    ///0x5c - SPI DMA interrupt raw register
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    ///0x60 - SPI DMA interrupt status register
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    ///0x64 - SPI DMA interrupt clear register
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    ///0x68 - The latest SPI DMA RX descriptor address receiving error
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    ///0x6c - The latest SPI DMA eof RX descriptor address
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    ///0x70 - Current SPI DMA RX descriptor pointer
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    ///0x74 - Next SPI DMA RX descriptor pointer
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    ///0x78 - Current SPI DMA RX buffer pointer
    #[inline(always)]
    pub const fn inlink_dscr_bf1(&self) -> &INLINK_DSCR_BF1 {
        &self.inlink_dscr_bf1
    }
    ///0x7c - The latest SPI DMA eof TX buffer address
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    ///0x80 - The latest SPI DMA eof TX descriptor address
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    ///0x84 - Current SPI DMA TX descriptor pointer
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    ///0x88 - Next SPI DMA TX descriptor pointer
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    ///0x8c - Current SPI DMA TX buffer pointer
    #[inline(always)]
    pub const fn outlink_dscr_bf1(&self) -> &OUTLINK_DSCR_BF1 {
        &self.outlink_dscr_bf1
    }
    ///0x90 - SPI DMA TX status
    #[inline(always)]
    pub const fn dma_outstatus(&self) -> &DMA_OUTSTATUS {
        &self.dma_outstatus
    }
    ///0x94 - SPI DMA RX status
    #[inline(always)]
    pub const fn dma_instatus(&self) -> &DMA_INSTATUS {
        &self.dma_instatus
    }
    ///0x98..0xe0 - Data buffer %s
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    ///Iterator for array of:
    ///0x98..0xe0 - Data buffer %s
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    ///0xe0 - SPI input delay mode configuration
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    ///0xe4 - SPI input delay number configuration
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    ///0xe8 - SPI output delay mode configuration
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    ///0xec - SPI output delay number configuration
    #[inline(always)]
    pub const fn dout_num(&self) -> &DOUT_NUM {
        &self.dout_num
    }
    ///0xf0 - LCD frame control register
    #[inline(always)]
    pub const fn lcd_ctrl(&self) -> &LCD_CTRL {
        &self.lcd_ctrl
    }
    ///0xf4 - LCD frame control1 register
    #[inline(always)]
    pub const fn lcd_ctrl1(&self) -> &LCD_CTRL1 {
        &self.lcd_ctrl1
    }
    ///0xf8 - LCD frame control2 register
    #[inline(always)]
    pub const fn lcd_ctrl2(&self) -> &LCD_CTRL2 {
        &self.lcd_ctrl2
    }
    ///0xfc - LCD delay number
    #[inline(always)]
    pub const fn lcd_d_mode(&self) -> &LCD_D_MODE {
        &self.lcd_d_mode
    }
    ///0x100 - LCD delay mode
    #[inline(always)]
    pub const fn lcd_d_num(&self) -> &LCD_D_NUM {
        &self.lcd_d_num
    }
    ///0x3fc - SPI version control
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
/**CMD (rw) register accessor: Command control register

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///Command control register
pub mod cmd;
/**ADDR (rw) register accessor: Address value

You can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///Address value
pub mod addr;
/**CTRL (rw) register accessor: SPI control register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI control register
pub mod ctrl;
/**CTRL1 (rw) register accessor: SPI control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
///SPI control register 1
pub mod ctrl1;
/**CTRL2 (rw) register accessor: SPI control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///SPI control register 2
pub mod ctrl2;
/**CLOCK (rw) register accessor: SPI clock control register

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI clock control register
pub mod clock;
/**USER (rw) register accessor: SPI USER control register

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI USER control register
pub mod user;
/**USER1 (rw) register accessor: SPI USER control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI USER control register 1
pub mod user1;
/**USER2 (rw) register accessor: SPI USER control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI USER control register 2
pub mod user2;
/**MOSI_DLEN (rw) register accessor: MOSI length

You can [`read`](crate::generic::Reg::read) this register and get [`mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mosi_dlen`] module*/
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
///MOSI length
pub mod mosi_dlen;
/**MISO_DLEN (rw) register accessor: MISO length

You can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@miso_dlen`] module*/
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
///MISO length
pub mod miso_dlen;
/**MISC (rw) register accessor: SPI misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI misc register
pub mod misc;
/**SLAVE (rw) register accessor: SPI slave control register

You can [`read`](crate::generic::Reg::read) this register and get [`slave::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave`] module*/
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
///SPI slave control register
pub mod slave;
/**SLAVE1 (rw) register accessor: SPI slave control register 1

You can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave1`] module*/
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
///SPI slave control register 1
pub mod slave1;
/**SLV_WRBUF_DLEN (rw) register accessor: SPI slave Wr_BUF interrupt and CONF control register

You can [`read`](crate::generic::Reg::read) this register and get [`slv_wrbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_wrbuf_dlen`] module*/
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
///SPI slave Wr_BUF interrupt and CONF control register
pub mod slv_wrbuf_dlen;
/**SLV_RDBUF_DLEN (rw) register accessor: SPI magic error and slave control register

You can [`read`](crate::generic::Reg::read) this register and get [`slv_rdbuf_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_rdbuf_dlen`] module*/
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
///SPI magic error and slave control register
pub mod slv_rdbuf_dlen;
/**SLV_RD_BYTE (rw) register accessor: SPI interrupt control register

You can [`read`](crate::generic::Reg::read) this register and get [`slv_rd_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_rd_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slv_rd_byte`] module*/
pub type SLV_RD_BYTE = crate::Reg<slv_rd_byte::SLV_RD_BYTE_SPEC>;
///SPI interrupt control register
pub mod slv_rd_byte;
/**FSM (rw) register accessor: SPI master status and DMA read byte control register

You can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsm`] module*/
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
///SPI master status and DMA read byte control register
pub mod fsm;
/**HOLD (rw) register accessor: SPI hold register

You can [`read`](crate::generic::Reg::read) this register and get [`hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hold`] module*/
pub type HOLD = crate::Reg<hold::HOLD_SPEC>;
///SPI hold register
pub mod hold;
/**DMA_CONF (rw) register accessor: SPI DMA control register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_conf`] module*/
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
///SPI DMA control register
pub mod dma_conf;
/**DMA_OUT_LINK (rw) register accessor: SPI DMA TX link configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_out_link`] module*/
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
///SPI DMA TX link configuration
pub mod dma_out_link;
/**DMA_IN_LINK (rw) register accessor: SPI DMA RX link configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_in_link`] module*/
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
///SPI DMA RX link configuration
pub mod dma_in_link;
/**DMA_INT_ENA (rw) register accessor: SPI DMA interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_ena`] module*/
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
///SPI DMA interrupt enable register
pub mod dma_int_ena;
/**DMA_INT_RAW (rw) register accessor: SPI DMA interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_raw`] module*/
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
///SPI DMA interrupt raw register
pub mod dma_int_raw;
/**DMA_INT_ST (rw) register accessor: SPI DMA interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_st`] module*/
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
///SPI DMA interrupt status register
pub mod dma_int_st;
/**DMA_INT_CLR (rw) register accessor: SPI DMA interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_int_clr`] module*/
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
///SPI DMA interrupt clear register
pub mod dma_int_clr;
/**IN_ERR_EOF_DES_ADDR (r) register accessor: The latest SPI DMA RX descriptor address receiving error

You can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_err_eof_des_addr`] module*/
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
///The latest SPI DMA RX descriptor address receiving error
pub mod in_err_eof_des_addr;
/**IN_SUC_EOF_DES_ADDR (r) register accessor: The latest SPI DMA eof RX descriptor address

You can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_suc_eof_des_addr`] module*/
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
///The latest SPI DMA eof RX descriptor address
pub mod in_suc_eof_des_addr;
/**INLINK_DSCR (r) register accessor: Current SPI DMA RX descriptor pointer

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr`] module*/
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
///Current SPI DMA RX descriptor pointer
pub mod inlink_dscr;
/**INLINK_DSCR_BF0 (r) register accessor: Next SPI DMA RX descriptor pointer

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr_bf0`] module*/
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
///Next SPI DMA RX descriptor pointer
pub mod inlink_dscr_bf0;
/**INLINK_DSCR_BF1 (r) register accessor: Current SPI DMA RX buffer pointer

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr_bf1`] module*/
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
///Current SPI DMA RX buffer pointer
pub mod inlink_dscr_bf1;
/**OUT_EOF_BFR_DES_ADDR (r) register accessor: The latest SPI DMA eof TX buffer address

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_bfr_des_addr`] module*/
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
///The latest SPI DMA eof TX buffer address
pub mod out_eof_bfr_des_addr;
/**OUT_EOF_DES_ADDR (r) register accessor: The latest SPI DMA eof TX descriptor address

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_des_addr`] module*/
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
///The latest SPI DMA eof TX descriptor address
pub mod out_eof_des_addr;
/**OUTLINK_DSCR (r) register accessor: Current SPI DMA TX descriptor pointer

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr`] module*/
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
///Current SPI DMA TX descriptor pointer
pub mod outlink_dscr;
/**OUTLINK_DSCR_BF0 (r) register accessor: Next SPI DMA TX descriptor pointer

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr_bf0`] module*/
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
///Next SPI DMA TX descriptor pointer
pub mod outlink_dscr_bf0;
/**OUTLINK_DSCR_BF1 (r) register accessor: Current SPI DMA TX buffer pointer

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr_bf1`] module*/
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
///Current SPI DMA TX buffer pointer
pub mod outlink_dscr_bf1;
/**DMA_OUTSTATUS (r) register accessor: SPI DMA TX status

You can [`read`](crate::generic::Reg::read) this register and get [`dma_outstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_outstatus`] module*/
pub type DMA_OUTSTATUS = crate::Reg<dma_outstatus::DMA_OUTSTATUS_SPEC>;
///SPI DMA TX status
pub mod dma_outstatus;
/**DMA_INSTATUS (r) register accessor: SPI DMA RX status

You can [`read`](crate::generic::Reg::read) this register and get [`dma_instatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_instatus`] module*/
pub type DMA_INSTATUS = crate::Reg<dma_instatus::DMA_INSTATUS_SPEC>;
///SPI DMA RX status
pub mod dma_instatus;
/**W (rw) register accessor: Data buffer %s

You can [`read`](crate::generic::Reg::read) this register and get [`w::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@w`] module*/
pub type W = crate::Reg<w::W_SPEC>;
///Data buffer %s
pub mod w;
/**DIN_MODE (rw) register accessor: SPI input delay mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_mode`] module*/
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
///SPI input delay mode configuration
pub mod din_mode;
/**DIN_NUM (rw) register accessor: SPI input delay number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_num`] module*/
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
///SPI input delay number configuration
pub mod din_num;
/**DOUT_MODE (rw) register accessor: SPI output delay mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_mode`] module*/
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
///SPI output delay mode configuration
pub mod dout_mode;
/**DOUT_NUM (rw) register accessor: SPI output delay number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dout_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_num`] module*/
pub type DOUT_NUM = crate::Reg<dout_num::DOUT_NUM_SPEC>;
///SPI output delay number configuration
pub mod dout_num;
/**LCD_CTRL (rw) register accessor: LCD frame control register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl`] module*/
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
///LCD frame control register
pub mod lcd_ctrl;
/**LCD_CTRL1 (rw) register accessor: LCD frame control1 register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl1`] module*/
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
///LCD frame control1 register
pub mod lcd_ctrl1;
/**LCD_CTRL2 (rw) register accessor: LCD frame control2 register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl2`] module*/
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
///LCD frame control2 register
pub mod lcd_ctrl2;
/**LCD_D_MODE (rw) register accessor: LCD delay number

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_d_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_d_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_d_mode`] module*/
pub type LCD_D_MODE = crate::Reg<lcd_d_mode::LCD_D_MODE_SPEC>;
///LCD delay number
pub mod lcd_d_mode;
/**LCD_D_NUM (rw) register accessor: LCD delay mode

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_d_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_d_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_d_num`] module*/
pub type LCD_D_NUM = crate::Reg<lcd_d_num::LCD_D_NUM_SPEC>;
///LCD delay mode
pub mod lcd_d_num;
/**REG_DATE (rw) register accessor: SPI version control

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_date`] module*/
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
///SPI version control
pub mod reg_date;
