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
    rd_status: RD_STATUS,
    _reserved12: [u8; 0x04],
    misc: MISC,
    tx_crc: TX_CRC,
    cache_fctrl: CACHE_FCTRL,
    _reserved15: [u8; 0x18],
    w: [W; 16],
    flash_waiti_ctrl: FLASH_WAITI_CTRL,
    flash_sus_ctrl: FLASH_SUS_CTRL,
    flash_sus_cmd: FLASH_SUS_CMD,
    sus_status: SUS_STATUS,
    _reserved20: [u8; 0x18],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved24: [u8; 0x04],
    ddr: DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    _reserved26: [u8; 0x24],
    spi_fmem_pms_attr: [SPI_FMEM_PMS_ATTR; 4],
    spi_fmem_pms_addr: [SPI_FMEM_PMS_ADDR; 4],
    spi_fmem_pms_size: [SPI_FMEM_PMS_SIZE; 4],
    spi_smem_pms_attr: [SPI_SMEM_PMS_ATTR; 4],
    spi_smem_pms_addr: [SPI_SMEM_PMS_ADDR; 4],
    spi_smem_pms_size: [SPI_SMEM_PMS_SIZE; 4],
    _reserved32: [u8; 0x04],
    pms_reject: PMS_REJECT,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    axi_err_addr: AXI_ERR_ADDR,
    spi_smem_ecc_ctrl: SPI_SMEM_ECC_CTRL,
    _reserved37: [u8; 0x08],
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    spi_smem_ac: SPI_SMEM_AC,
    _reserved46: [u8; 0x5c],
    clock_gate: CLOCK_GATE,
    _reserved47: [u8; 0x0178],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    _reserved50: [u8; 0x68],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved52: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI1 memory command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - SPI1 address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x08 - SPI1 control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI1 control1 register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI1 control2 register."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI1 clock division control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - SPI1 user register."]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - SPI1 user1 register."]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - SPI1 user2 register."]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x24 - SPI1 send data bit length control register."]
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    #[doc = "0x28 - SPI1 receive data bit length control register."]
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    #[doc = "0x2c - SPI1 status register."]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x34 - SPI1 misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x38 - SPI1 TX CRC data register."]
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    #[doc = "0x3c - SPI1 bit mode control register."]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x58..0x98 - SPI1 memory data buffer%s"]
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x98 - SPI1 memory data buffer%s"]
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    #[doc = "0x98 - SPI1 wait idle control register"]
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FLASH_WAITI_CTRL {
        &self.flash_waiti_ctrl
    }
    #[doc = "0x9c - SPI1 flash suspend control register"]
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FLASH_SUS_CTRL {
        &self.flash_sus_ctrl
    }
    #[doc = "0xa0 - SPI1 flash suspend command register"]
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FLASH_SUS_CMD {
        &self.flash_sus_cmd
    }
    #[doc = "0xa4 - SPI1 flash suspend status register"]
    #[inline(always)]
    pub const fn sus_status(&self) -> &SUS_STATUS {
        &self.sus_status
    }
    #[doc = "0xc0 - SPI1 interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xc4 - SPI1 interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xc8 - SPI1 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xcc - SPI1 interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xd4 - SPI1 DDR control register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    #[doc = "0xd8 - MSPI external RAM DDR mode control register"]
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    #[doc = "0x100..0x110 - MSPI flash ACE section %s attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_attr(&self, n: usize) -> &SPI_FMEM_PMS_ATTR {
        &self.spi_fmem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - MSPI flash ACE section %s attribute register"]
    #[inline(always)]
    pub fn spi_fmem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ATTR> {
        self.spi_fmem_pms_attr.iter()
    }
    #[doc = "0x100 - MSPI flash ACE section 0 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(0)
    }
    #[doc = "0x104 - MSPI flash ACE section 1 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(1)
    }
    #[doc = "0x108 - MSPI flash ACE section 2 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(2)
    }
    #[doc = "0x10c - MSPI flash ACE section 3 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(3)
    }
    #[doc = "0x110..0x120 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_addr(&self, n: usize) -> &SPI_FMEM_PMS_ADDR {
        &self.spi_fmem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ADDR> {
        self.spi_fmem_pms_addr.iter()
    }
    #[doc = "0x110 - SPI1 flash ACE section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(0)
    }
    #[doc = "0x114 - SPI1 flash ACE section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(1)
    }
    #[doc = "0x118 - SPI1 flash ACE section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(2)
    }
    #[doc = "0x11c - SPI1 flash ACE section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(3)
    }
    #[doc = "0x120..0x130 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self, n: usize) -> &SPI_FMEM_PMS_SIZE {
        &self.spi_fmem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_SIZE> {
        self.spi_fmem_pms_size.iter()
    }
    #[doc = "0x120 - SPI1 flash ACE section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(0)
    }
    #[doc = "0x124 - SPI1 flash ACE section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(1)
    }
    #[doc = "0x128 - SPI1 flash ACE section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(2)
    }
    #[doc = "0x12c - SPI1 flash ACE section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(3)
    }
    #[doc = "0x130..0x140 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_attr(&self, n: usize) -> &SPI_SMEM_PMS_ATTR {
        &self.spi_smem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x140 - SPI1 flash ACE section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ATTR> {
        self.spi_smem_pms_attr.iter()
    }
    #[doc = "0x130 - SPI1 flash ACE section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(0)
    }
    #[doc = "0x134 - SPI1 flash ACE section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(1)
    }
    #[doc = "0x138 - SPI1 flash ACE section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(2)
    }
    #[doc = "0x13c - SPI1 flash ACE section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(3)
    }
    #[doc = "0x140..0x150 - SPI1 external RAM ACE section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_addr(&self, n: usize) -> &SPI_SMEM_PMS_ADDR {
        &self.spi_smem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - SPI1 external RAM ACE section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ADDR> {
        self.spi_smem_pms_addr.iter()
    }
    #[doc = "0x140 - SPI1 external RAM ACE section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(0)
    }
    #[doc = "0x144 - SPI1 external RAM ACE section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(1)
    }
    #[doc = "0x148 - SPI1 external RAM ACE section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(2)
    }
    #[doc = "0x14c - SPI1 external RAM ACE section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(3)
    }
    #[doc = "0x150..0x160 - SPI1 external RAM ACE section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self, n: usize) -> &SPI_SMEM_PMS_SIZE {
        &self.spi_smem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - SPI1 external RAM ACE section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_SIZE> {
        self.spi_smem_pms_size.iter()
    }
    #[doc = "0x150 - SPI1 external RAM ACE section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(0)
    }
    #[doc = "0x154 - SPI1 external RAM ACE section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(1)
    }
    #[doc = "0x158 - SPI1 external RAM ACE section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(2)
    }
    #[doc = "0x15c - SPI1 external RAM ACE section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(3)
    }
    #[doc = "0x164 - SPI1 access reject register"]
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PMS_REJECT {
        &self.pms_reject
    }
    #[doc = "0x168 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &ECC_CTRL {
        &self.ecc_ctrl
    }
    #[doc = "0x16c - MSPI ECC error address register"]
    #[inline(always)]
    pub const fn ecc_err_addr(&self) -> &ECC_ERR_ADDR {
        &self.ecc_err_addr
    }
    #[doc = "0x170 - MSPI AXI request error address"]
    #[inline(always)]
    pub const fn axi_err_addr(&self) -> &AXI_ERR_ADDR {
        &self.axi_err_addr
    }
    #[doc = "0x174 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn spi_smem_ecc_ctrl(&self) -> &SPI_SMEM_ECC_CTRL {
        &self.spi_smem_ecc_ctrl
    }
    #[doc = "0x180 - SPI1 timing control register"]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    #[doc = "0x184 - MSPI flash input timing delay mode control register"]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0x188 - MSPI flash input timing delay number control register"]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0x18c - MSPI flash output timing adjustment control register"]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0x190 - MSPI external RAM timing calibration register"]
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> &SPI_SMEM_TIMING_CALI {
        &self.spi_smem_timing_cali
    }
    #[doc = "0x194 - MSPI external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn spi_smem_din_mode(&self) -> &SPI_SMEM_DIN_MODE {
        &self.spi_smem_din_mode
    }
    #[doc = "0x198 - MSPI external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn spi_smem_din_num(&self) -> &SPI_SMEM_DIN_NUM {
        &self.spi_smem_din_num
    }
    #[doc = "0x19c - MSPI external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn spi_smem_dout_mode(&self) -> &SPI_SMEM_DOUT_MODE {
        &self.spi_smem_dout_mode
    }
    #[doc = "0x1a0 - MSPI external RAM ECC and SPI CS timing control register"]
    #[inline(always)]
    pub const fn spi_smem_ac(&self) -> &SPI_SMEM_AC {
        &self.spi_smem_ac
    }
    #[doc = "0x200 - SPI1 clk_gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x37c - MSPI-MMU item content register"]
    #[inline(always)]
    pub const fn mmu_item_content(&self) -> &MMU_ITEM_CONTENT {
        &self.mmu_item_content
    }
    #[doc = "0x380 - MSPI-MMU item index register"]
    #[inline(always)]
    pub const fn mmu_item_index(&self) -> &MMU_ITEM_INDEX {
        &self.mmu_item_index
    }
    #[doc = "0x384 - MSPI MMU power control register"]
    #[inline(always)]
    pub const fn mmu_power_ctrl(&self) -> &MMU_POWER_CTRL {
        &self.mmu_power_ctrl
    }
    #[doc = "0x3f0 - MSPI ECO high register"]
    #[inline(always)]
    pub const fn registerrnd_eco_high(&self) -> &REGISTERRND_ECO_HIGH {
        &self.registerrnd_eco_high
    }
    #[doc = "0x3f4 - MSPI ECO low register"]
    #[inline(always)]
    pub const fn registerrnd_eco_low(&self) -> &REGISTERRND_ECO_LOW {
        &self.registerrnd_eco_low
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: SPI1 memory command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI1 memory command register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: SPI1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI1 address register"]
pub mod addr;
#[doc = "CTRL (rw) register accessor: SPI1 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI1 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI1 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI1 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (w) register accessor: SPI1 control2 register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI1 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI1 clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI1 clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI1 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI1 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI1 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI1 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI1 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI1 user2 register."]
pub mod user2;
#[doc = "MOSI_DLEN (rw) register accessor: SPI1 send data bit length control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mosi_dlen`] module"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "SPI1 send data bit length control register."]
pub mod mosi_dlen;
#[doc = "MISO_DLEN (rw) register accessor: SPI1 receive data bit length control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miso_dlen`] module"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "SPI1 receive data bit length control register."]
pub mod miso_dlen;
#[doc = "RD_STATUS (rw) register accessor: SPI1 status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI1 status register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI1 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI1 misc register"]
pub mod misc;
#[doc = "TX_CRC (r) register accessor: SPI1 TX CRC data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc`] module"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = "SPI1 TX CRC data register."]
pub mod tx_crc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI1 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI1 bit mode control register."]
pub mod cache_fctrl;
#[doc = "W (rw) register accessor: SPI1 memory data buffer%s\n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`] module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = "SPI1 memory data buffer%s"]
pub mod w;
#[doc = "FLASH_WAITI_CTRL (rw) register accessor: SPI1 wait idle control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_waiti_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_waiti_ctrl`] module"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI1 wait idle control register"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL (rw) register accessor: SPI1 flash suspend control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_ctrl`] module"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI1 flash suspend control register"]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD (rw) register accessor: SPI1 flash suspend command register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_sus_cmd`] module"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = "SPI1 flash suspend command register"]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS (rw) register accessor: SPI1 flash suspend status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sus_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sus_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sus_status`] module"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = "SPI1 flash suspend status register"]
pub mod sus_status;
#[doc = "INT_ENA (rw) register accessor: SPI1 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SPI1 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: SPI1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SPI1 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: SPI1 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SPI1 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: SPI1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SPI1 interrupt status register"]
pub mod int_st;
#[doc = "DDR (r) register accessor: SPI1 DDR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "SPI1 DDR control register"]
pub mod ddr;
#[doc = "TIMING_CALI (rw) register accessor: SPI1 timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI1 timing control register"]
pub mod timing_cali;
#[doc = "CLOCK_GATE (rw) register accessor: SPI1 clk_gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI1 clk_gate register"]
pub mod clock_gate;
pub use crate::{
    dma::{date, DATE},
    spi0::{
        axi_err_addr,
        din_mode,
        din_num,
        dout_mode,
        ecc_ctrl,
        ecc_err_addr,
        mmu_item_content,
        mmu_item_index,
        mmu_power_ctrl,
        pms_reject,
        registerrnd_eco_high,
        registerrnd_eco_low,
        spi_fmem_pms_addr,
        spi_fmem_pms_attr,
        spi_fmem_pms_size,
        spi_smem_ac,
        spi_smem_ddr,
        spi_smem_din_mode,
        spi_smem_din_num,
        spi_smem_dout_mode,
        spi_smem_ecc_ctrl,
        spi_smem_pms_addr,
        spi_smem_pms_attr,
        spi_smem_pms_size,
        spi_smem_timing_cali,
        AXI_ERR_ADDR,
        DIN_MODE,
        DIN_NUM,
        DOUT_MODE,
        ECC_CTRL,
        ECC_ERR_ADDR,
        MMU_ITEM_CONTENT,
        MMU_ITEM_INDEX,
        MMU_POWER_CTRL,
        PMS_REJECT,
        REGISTERRND_ECO_HIGH,
        REGISTERRND_ECO_LOW,
        SPI_FMEM_PMS_ADDR,
        SPI_FMEM_PMS_ATTR,
        SPI_FMEM_PMS_SIZE,
        SPI_SMEM_AC,
        SPI_SMEM_DDR,
        SPI_SMEM_DIN_MODE,
        SPI_SMEM_DIN_NUM,
        SPI_SMEM_DOUT_MODE,
        SPI_SMEM_ECC_CTRL,
        SPI_SMEM_PMS_ADDR,
        SPI_SMEM_PMS_ATTR,
        SPI_SMEM_PMS_SIZE,
        SPI_SMEM_TIMING_CALI,
    },
};
