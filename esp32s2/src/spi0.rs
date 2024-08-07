#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    _reserved_16_cache_sctrl: [u8; 0x04],
    fsm: FSM,
    _reserved_18_hold: [u8; 0x04],
    _reserved_19_dma_conf: [u8; 0x04],
    _reserved_20_sram_clk: [u8; 0x04],
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
    #[doc = "0x00 - Command control register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - Address value"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - SPI control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI clock control register"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - SPI USER control register"]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - SPI USER control register 1"]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - SPI USER control register 2"]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x24 - MOSI length"]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x28 - MISO length"]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x2c - SPI misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x30 - SPI slave control register"]
    #[inline(always)]
    pub const fn slave(&self) -> &SLAVE {
        &self.slave
    }
    #[doc = "0x34 - SPI slave control register 1"]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        &self.slave1
    }
    #[doc = "0x38 - SPI slave Wr_BUF interrupt and CONF control register"]
    #[inline(always)]
    pub const fn slv_wrbuf_dlen(&self) -> &SLV_WRBUF_DLEN {
        &self.slv_wrbuf_dlen
    }
    #[doc = "0x3c - SPI magic error and slave control register"]
    #[inline(always)]
    pub const fn slv_rdbuf_dlen(&self) -> &SLV_RDBUF_DLEN {
        &self.slv_rdbuf_dlen
    }
    #[doc = "0x40 - SPI Memory Cache SCTRL Register"]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - SPI interrupt control register"]
    #[inline(always)]
    pub const fn slv_rd_byte(&self) -> &SLV_RD_BYTE {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - SPI master status and DMA read byte control register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0x48 - SPI Memory SRAM DRD CMD Register"]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - SPI hold register"]
    #[inline(always)]
    pub const fn hold(&self) -> &HOLD {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - SPI Memory SRAM DWR CMD Register"]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - SPI DMA control register"]
    #[inline(always)]
    pub const fn dma_conf(&self) -> &DMA_CONF {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - SPI Memory SRAM Clock Register"]
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - SPI DMA TX link configuration"]
    #[inline(always)]
    pub const fn dma_out_link(&self) -> &DMA_OUT_LINK {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - SPI DMA RX link configuration"]
    #[inline(always)]
    pub const fn dma_in_link(&self) -> &DMA_IN_LINK {
        &self.dma_in_link
    }
    #[doc = "0x58 - SPI DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dma_int_ena(&self) -> &DMA_INT_ENA {
        &self.dma_int_ena
    }
    #[doc = "0x5c - SPI DMA interrupt raw register"]
    #[inline(always)]
    pub const fn dma_int_raw(&self) -> &DMA_INT_RAW {
        &self.dma_int_raw
    }
    #[doc = "0x60 - SPI DMA interrupt status register"]
    #[inline(always)]
    pub const fn dma_int_st(&self) -> &DMA_INT_ST {
        &self.dma_int_st
    }
    #[doc = "0x64 - SPI DMA interrupt clear register"]
    #[inline(always)]
    pub const fn dma_int_clr(&self) -> &DMA_INT_CLR {
        &self.dma_int_clr
    }
    #[doc = "0x68 - The latest SPI DMA RX descriptor address receiving error"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    #[doc = "0x6c - The latest SPI DMA eof RX descriptor address"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    #[doc = "0x70 - Current SPI DMA RX descriptor pointer"]
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    #[doc = "0x74 - Next SPI DMA RX descriptor pointer"]
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    #[doc = "0x78 - Current SPI DMA RX buffer pointer"]
    #[inline(always)]
    pub const fn inlink_dscr_bf1(&self) -> &INLINK_DSCR_BF1 {
        &self.inlink_dscr_bf1
    }
    #[doc = "0x7c - The latest SPI DMA eof TX buffer address"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x80 - The latest SPI DMA eof TX descriptor address"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x84 - Current SPI DMA TX descriptor pointer"]
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    #[doc = "0x88 - Next SPI DMA TX descriptor pointer"]
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    #[doc = "0x8c - Current SPI DMA TX buffer pointer"]
    #[inline(always)]
    pub const fn outlink_dscr_bf1(&self) -> &OUTLINK_DSCR_BF1 {
        &self.outlink_dscr_bf1
    }
    #[doc = "0x90 - SPI DMA TX status"]
    #[inline(always)]
    pub const fn dma_outstatus(&self) -> &DMA_OUTSTATUS {
        &self.dma_outstatus
    }
    #[doc = "0x94 - SPI DMA RX status"]
    #[inline(always)]
    pub const fn dma_instatus(&self) -> &DMA_INSTATUS {
        &self.dma_instatus
    }
    #[doc = "0x98..0xe0 - Data buffer %s"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xe0 - Data buffer %s"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0xe0 - SPI input delay mode configuration"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0xe4 - SPI input delay number configuration"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0xe8 - SPI output delay mode configuration"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0xec - SPI output delay number configuration"]
    #[inline(always)]
    pub const fn dout_num(&self) -> &DOUT_NUM {
        &self.dout_num
    }
    #[doc = "0xf0 - LCD frame control register"]
    #[inline(always)]
    pub const fn lcd_ctrl(&self) -> &LCD_CTRL {
        &self.lcd_ctrl
    }
    #[doc = "0xf4 - LCD frame control1 register"]
    #[inline(always)]
    pub const fn lcd_ctrl1(&self) -> &LCD_CTRL1 {
        &self.lcd_ctrl1
    }
    #[doc = "0xf8 - LCD frame control2 register"]
    #[inline(always)]
    pub const fn lcd_ctrl2(&self) -> &LCD_CTRL2 {
        &self.lcd_ctrl2
    }
    #[doc = "0xfc - LCD delay number"]
    #[inline(always)]
    pub const fn lcd_d_mode(&self) -> &LCD_D_MODE {
        &self.lcd_d_mode
    }
    #[doc = "0x100 - LCD delay mode"]
    #[inline(always)]
    pub const fn lcd_d_num(&self) -> &LCD_D_NUM {
        &self.lcd_d_num
    }
    #[doc = "0x3fc - SPI version control"]
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
#[doc = "CMD (rw) register accessor: Command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command control register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address value"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI control register"]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SPI control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI control register 2"]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock control register"]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI USER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI USER control register"]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI USER control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI USER control register 1"]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI USER control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI USER control register 2"]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: MOSI length\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "MOSI length"]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: MISO length\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "MISO length"]
pub mod miso_dlen;
#[doc = "MISC (rw) register accessor: SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI misc register"]
pub mod misc;
#[doc = "SLAVE (rw) register accessor: SPI slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave`] module"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = "SPI slave control register"]
pub mod slave;
#[doc = "SLAVE1 (rw) register accessor: SPI slave control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`] module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "SPI slave control register 1"]
pub mod slave1;
#[doc = "SLV_WRBUF_DLEN (rw) register accessor: SPI slave Wr_BUF interrupt and CONF control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_wrbuf_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_wrbuf_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_wrbuf_dlen`] module"]
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
#[doc = "SPI slave Wr_BUF interrupt and CONF control register"]
pub mod slv_wrbuf_dlen;
#[doc = "SLV_RDBUF_DLEN (rw) register accessor: SPI magic error and slave control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_rdbuf_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_rdbuf_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_rdbuf_dlen`] module"]
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
#[doc = "SPI magic error and slave control register"]
pub mod slv_rdbuf_dlen;
#[doc = "SLV_RD_BYTE (rw) register accessor: SPI interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`slv_rd_byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slv_rd_byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_rd_byte`] module"]
pub type SLV_RD_BYTE = crate::Reg<slv_rd_byte::SLV_RD_BYTE_SPEC>;
#[doc = "SPI interrupt control register"]
pub mod slv_rd_byte;
#[doc = "FSM (rw) register accessor: SPI master status and DMA read byte control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI master status and DMA read byte control register"]
pub mod fsm;
#[doc = "HOLD (rw) register accessor: SPI hold register\n\nYou can [`read`](crate::Reg::read) this register and get [`hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hold`] module"]
pub type HOLD = crate::Reg<hold::HOLD_SPEC>;
#[doc = "SPI hold register"]
pub mod hold;
#[doc = "DMA_CONF (rw) register accessor: SPI DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_conf`] module"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "SPI DMA control register"]
pub mod dma_conf;
#[doc = "DMA_OUT_LINK (rw) register accessor: SPI DMA TX link configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_out_link`] module"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = "SPI DMA TX link configuration"]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK (rw) register accessor: SPI DMA RX link configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_in_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_in_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_in_link`] module"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = "SPI DMA RX link configuration"]
pub mod dma_in_link;
#[doc = "DMA_INT_ENA (rw) register accessor: SPI DMA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ena`] module"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = "SPI DMA interrupt enable register"]
pub mod dma_int_ena;
#[doc = "DMA_INT_RAW (rw) register accessor: SPI DMA interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_raw`] module"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = "SPI DMA interrupt raw register"]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST (rw) register accessor: SPI DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_st`] module"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = "SPI DMA interrupt status register"]
pub mod dma_int_st;
#[doc = "DMA_INT_CLR (rw) register accessor: SPI DMA interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_clr`] module"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = "SPI DMA interrupt clear register"]
pub mod dma_int_clr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: The latest SPI DMA RX descriptor address receiving error\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA RX descriptor address receiving error"]
pub mod in_err_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: The latest SPI DMA eof RX descriptor address\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof RX descriptor address"]
pub mod in_suc_eof_des_addr;
#[doc = "INLINK_DSCR (r) register accessor: Current SPI DMA RX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr`] module"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = "Current SPI DMA RX descriptor pointer"]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 (r) register accessor: Next SPI DMA RX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr_bf0`] module"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = "Next SPI DMA RX descriptor pointer"]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 (r) register accessor: Current SPI DMA RX buffer pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`inlink_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inlink_dscr_bf1`] module"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = "Current SPI DMA RX buffer pointer"]
pub mod inlink_dscr_bf1;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: The latest SPI DMA eof TX buffer address\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof TX buffer address"]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: The latest SPI DMA eof TX descriptor address\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "The latest SPI DMA eof TX descriptor address"]
pub mod out_eof_des_addr;
#[doc = "OUTLINK_DSCR (r) register accessor: Current SPI DMA TX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr`] module"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = "Current SPI DMA TX descriptor pointer"]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 (r) register accessor: Next SPI DMA TX descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr_bf0`] module"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = "Next SPI DMA TX descriptor pointer"]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 (r) register accessor: Current SPI DMA TX buffer pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`outlink_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outlink_dscr_bf1`] module"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = "Current SPI DMA TX buffer pointer"]
pub mod outlink_dscr_bf1;
#[doc = "DMA_OUTSTATUS (r) register accessor: SPI DMA TX status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_outstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_outstatus`] module"]
pub type DMA_OUTSTATUS = crate::Reg<dma_outstatus::DMA_OUTSTATUS_SPEC>;
#[doc = "SPI DMA TX status"]
pub mod dma_outstatus;
#[doc = "DMA_INSTATUS (r) register accessor: SPI DMA RX status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_instatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_instatus`] module"]
pub type DMA_INSTATUS = crate::Reg<dma_instatus::DMA_INSTATUS_SPEC>;
#[doc = "SPI DMA RX status"]
pub mod dma_instatus;
#[doc = "W (rw) register accessor: Data buffer %s\n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = "Data buffer %s"]
pub mod w;
#[doc = "DIN_MODE (rw) register accessor: SPI input delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI input delay mode configuration"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: SPI input delay number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI input delay number configuration"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: SPI output delay mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI output delay mode configuration"]
pub mod dout_mode;
#[doc = "DOUT_NUM (rw) register accessor: SPI output delay number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_num`] module"]
pub type DOUT_NUM = crate::Reg<dout_num::DOUT_NUM_SPEC>;
#[doc = "SPI output delay number configuration"]
pub mod dout_num;
#[doc = "LCD_CTRL (rw) register accessor: LCD frame control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl`] module"]
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
#[doc = "LCD frame control register"]
pub mod lcd_ctrl;
#[doc = "LCD_CTRL1 (rw) register accessor: LCD frame control1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl1`] module"]
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
#[doc = "LCD frame control1 register"]
pub mod lcd_ctrl1;
#[doc = "LCD_CTRL2 (rw) register accessor: LCD frame control2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl2`] module"]
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
#[doc = "LCD frame control2 register"]
pub mod lcd_ctrl2;
#[doc = "LCD_D_MODE (rw) register accessor: LCD delay number\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_d_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_d_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_d_mode`] module"]
pub type LCD_D_MODE = crate::Reg<lcd_d_mode::LCD_D_MODE_SPEC>;
#[doc = "LCD delay number"]
pub mod lcd_d_mode;
#[doc = "LCD_D_NUM (rw) register accessor: LCD delay mode\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_d_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_d_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_d_num`] module"]
pub type LCD_D_NUM = crate::Reg<lcd_d_num::LCD_D_NUM_SPEC>;
#[doc = "LCD delay mode"]
pub mod lcd_d_num;
#[doc = "REG_DATE (rw) register accessor: SPI version control\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_date`] module"]
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
#[doc = "SPI version control"]
pub mod reg_date;
#[doc = "CACHE_SCTRL (rw) register accessor: SPI Memory Cache SCTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = "SPI Memory Cache SCTRL Register"]
pub mod cache_sctrl;
#[doc = "SRAM_DWR_CMD (rw) register accessor: SPI Memory SRAM DWR CMD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = "SPI Memory SRAM DWR CMD Register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: SPI Memory SRAM DRD CMD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = "SPI Memory SRAM DRD CMD Register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_CLK (rw) register accessor: SPI Memory SRAM Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
#[doc = "SPI Memory SRAM Clock Register"]
pub mod sram_clk;
