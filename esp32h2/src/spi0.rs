#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    cmd: CMD,
    _reserved1: [u8; 0x04],
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    _reserved8: [u8; 0x08],
    rd_status: RD_STATUS,
    _reserved9: [u8; 0x04],
    misc: MISC,
    _reserved10: [u8; 0x04],
    cache_fctrl: CACHE_FCTRL,
    cache_sctrl: CACHE_SCTRL,
    sram_cmd: SRAM_CMD,
    sram_drd_cmd: SRAM_DRD_CMD,
    sram_dwr_cmd: SRAM_DWR_CMD,
    sram_clk: SRAM_CLK,
    fsm: FSM,
    _reserved17: [u8; 0x68],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved21: [u8; 0x04],
    ddr: DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    _reserved23: [u8; 0x24],
    spi_fmem_pms_attr: [SPI_FMEM_PMS_ATTR; 4],
    spi_fmem_pms_addr: [SPI_FMEM_PMS_ADDR; 4],
    spi_fmem_pms_size: [SPI_FMEM_PMS_SIZE; 4],
    spi_smem_pms_attr: [SPI_SMEM_PMS_ATTR; 4],
    spi_smem_pms_addr: [SPI_SMEM_PMS_ADDR; 4],
    spi_smem_pms_size: [SPI_SMEM_PMS_SIZE; 4],
    _reserved29: [u8; 0x04],
    pms_reject: PMS_REJECT,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    axi_err_addr: AXI_ERR_ADDR,
    spi_smem_ecc_ctrl: SPI_SMEM_ECC_CTRL,
    _reserved34: [u8; 0x08],
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    spi_smem_ac: SPI_SMEM_AC,
    _reserved43: [u8; 0x5c],
    clock_gate: CLOCK_GATE,
    _reserved44: [u8; 0xfc],
    xts_plain_base: XTS_PLAIN_BASE,
    _reserved45: [u8; 0x3c],
    xts_linesize: XTS_LINESIZE,
    xts_destination: XTS_DESTINATION,
    xts_physical_address: XTS_PHYSICAL_ADDRESS,
    xts_trigger: XTS_TRIGGER,
    xts_release: XTS_RELEASE,
    xts_destroy: XTS_DESTROY,
    xts_state: XTS_STATE,
    xts_date: XTS_DATE,
    _reserved53: [u8; 0x1c],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    dpa_ctrl: DPA_CTRL,
    _reserved57: [u8; 0x64],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved59: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - SPI0 FSM status register
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x08 - SPI0 control register.
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI0 control1 register.
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    ///0x10 - SPI0 control2 register.
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x14 - SPI clock division control register.
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x18 - SPI0 user register.
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x1c - SPI0 user1 register.
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x20 - SPI0 user2 register.
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x2c - SPI0 read control register.
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    ///0x34 - SPI0 misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x3c - SPI0 bit mode control register.
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    ///0x40 - SPI0 external RAM control register
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    ///0x44 - SPI0 external RAM mode control register
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    ///0x48 - SPI0 external RAM DDR read command control register
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    ///0x4c - SPI0 external RAM DDR write command control register
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    ///0x50 - SPI0 external RAM clock control register
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        &self.sram_clk
    }
    ///0x54 - SPI0 FSM status register
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    ///0xc0 - SPI0 interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xc4 - SPI0 interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xc8 - SPI0 interrupt raw register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0xcc - SPI0 interrupt status register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0xd4 - SPI0 flash DDR mode control register
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    ///0xd8 - SPI0 external RAM DDR mode control register
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    ///0x100..0x110 - MSPI flash ACE section %s attribute register
    #[inline(always)]
    pub const fn spi_fmem_pms_attr(&self, n: usize) -> &SPI_FMEM_PMS_ATTR {
        &self.spi_fmem_pms_attr[n]
    }
    ///Iterator for array of:
    ///0x100..0x110 - MSPI flash ACE section %s attribute register
    #[inline(always)]
    pub fn spi_fmem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ATTR> {
        self.spi_fmem_pms_attr.iter()
    }
    ///0x100 - MSPI flash ACE section 0 attribute register
    #[inline(always)]
    pub const fn spi_fmem_pms0_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(0)
    }
    ///0x104 - MSPI flash ACE section 1 attribute register
    #[inline(always)]
    pub const fn spi_fmem_pms1_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(1)
    }
    ///0x108 - MSPI flash ACE section 2 attribute register
    #[inline(always)]
    pub const fn spi_fmem_pms2_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(2)
    }
    ///0x10c - MSPI flash ACE section 3 attribute register
    #[inline(always)]
    pub const fn spi_fmem_pms3_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(3)
    }
    ///0x110..0x120 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub const fn spi_fmem_pms_addr(&self, n: usize) -> &SPI_FMEM_PMS_ADDR {
        &self.spi_fmem_pms_addr[n]
    }
    ///Iterator for array of:
    ///0x110..0x120 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub fn spi_fmem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ADDR> {
        self.spi_fmem_pms_addr.iter()
    }
    ///0x110 - SPI1 flash ACE section 0 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms0_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(0)
    }
    ///0x114 - SPI1 flash ACE section 1 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms1_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(1)
    }
    ///0x118 - SPI1 flash ACE section 2 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms2_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(2)
    }
    ///0x11c - SPI1 flash ACE section 3 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms3_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(3)
    }
    ///0x120..0x130 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self, n: usize) -> &SPI_FMEM_PMS_SIZE {
        &self.spi_fmem_pms_size[n]
    }
    ///Iterator for array of:
    ///0x120..0x130 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub fn spi_fmem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_SIZE> {
        self.spi_fmem_pms_size.iter()
    }
    ///0x120 - SPI1 flash ACE section 0 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms0_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(0)
    }
    ///0x124 - SPI1 flash ACE section 1 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms1_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(1)
    }
    ///0x128 - SPI1 flash ACE section 2 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms2_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(2)
    }
    ///0x12c - SPI1 flash ACE section 3 start address register
    #[inline(always)]
    pub const fn spi_fmem_pms3_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(3)
    }
    ///0x130..0x140 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub const fn spi_smem_pms_attr(&self, n: usize) -> &SPI_SMEM_PMS_ATTR {
        &self.spi_smem_pms_attr[n]
    }
    ///Iterator for array of:
    ///0x130..0x140 - SPI1 flash ACE section %s start address register
    #[inline(always)]
    pub fn spi_smem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ATTR> {
        self.spi_smem_pms_attr.iter()
    }
    ///0x130 - SPI1 flash ACE section 0 start address register
    #[inline(always)]
    pub const fn spi_smem_pms0_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(0)
    }
    ///0x134 - SPI1 flash ACE section 1 start address register
    #[inline(always)]
    pub const fn spi_smem_pms1_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(1)
    }
    ///0x138 - SPI1 flash ACE section 2 start address register
    #[inline(always)]
    pub const fn spi_smem_pms2_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(2)
    }
    ///0x13c - SPI1 flash ACE section 3 start address register
    #[inline(always)]
    pub const fn spi_smem_pms3_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(3)
    }
    ///0x140..0x150 - SPI1 external RAM ACE section %s start address register
    #[inline(always)]
    pub const fn spi_smem_pms_addr(&self, n: usize) -> &SPI_SMEM_PMS_ADDR {
        &self.spi_smem_pms_addr[n]
    }
    ///Iterator for array of:
    ///0x140..0x150 - SPI1 external RAM ACE section %s start address register
    #[inline(always)]
    pub fn spi_smem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ADDR> {
        self.spi_smem_pms_addr.iter()
    }
    ///0x140 - SPI1 external RAM ACE section 0 start address register
    #[inline(always)]
    pub const fn spi_smem_pms0_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(0)
    }
    ///0x144 - SPI1 external RAM ACE section 1 start address register
    #[inline(always)]
    pub const fn spi_smem_pms1_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(1)
    }
    ///0x148 - SPI1 external RAM ACE section 2 start address register
    #[inline(always)]
    pub const fn spi_smem_pms2_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(2)
    }
    ///0x14c - SPI1 external RAM ACE section 3 start address register
    #[inline(always)]
    pub const fn spi_smem_pms3_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(3)
    }
    ///0x150..0x160 - SPI1 external RAM ACE section %s start address register
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self, n: usize) -> &SPI_SMEM_PMS_SIZE {
        &self.spi_smem_pms_size[n]
    }
    ///Iterator for array of:
    ///0x150..0x160 - SPI1 external RAM ACE section %s start address register
    #[inline(always)]
    pub fn spi_smem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_SIZE> {
        self.spi_smem_pms_size.iter()
    }
    ///0x150 - SPI1 external RAM ACE section 0 start address register
    #[inline(always)]
    pub const fn spi_smem_pms0_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(0)
    }
    ///0x154 - SPI1 external RAM ACE section 1 start address register
    #[inline(always)]
    pub const fn spi_smem_pms1_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(1)
    }
    ///0x158 - SPI1 external RAM ACE section 2 start address register
    #[inline(always)]
    pub const fn spi_smem_pms2_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(2)
    }
    ///0x15c - SPI1 external RAM ACE section 3 start address register
    #[inline(always)]
    pub const fn spi_smem_pms3_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(3)
    }
    ///0x164 - SPI1 access reject register
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PMS_REJECT {
        &self.pms_reject
    }
    ///0x168 - MSPI ECC control register
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &ECC_CTRL {
        &self.ecc_ctrl
    }
    ///0x16c - MSPI ECC error address register
    #[inline(always)]
    pub const fn ecc_err_addr(&self) -> &ECC_ERR_ADDR {
        &self.ecc_err_addr
    }
    ///0x170 - SPI0 AXI request error address.
    #[inline(always)]
    pub const fn axi_err_addr(&self) -> &AXI_ERR_ADDR {
        &self.axi_err_addr
    }
    ///0x174 - MSPI ECC control register
    #[inline(always)]
    pub const fn spi_smem_ecc_ctrl(&self) -> &SPI_SMEM_ECC_CTRL {
        &self.spi_smem_ecc_ctrl
    }
    ///0x180 - SPI0 flash timing calibration register
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    ///0x184 - MSPI flash input timing delay mode control register
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    ///0x188 - MSPI flash input timing delay number control register
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    ///0x18c - MSPI flash output timing adjustment control register
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    ///0x190 - MSPI external RAM timing calibration register
    #[inline(always)]
    pub const fn spi_smem_timing_cali(&self) -> &SPI_SMEM_TIMING_CALI {
        &self.spi_smem_timing_cali
    }
    ///0x194 - MSPI external RAM input timing delay mode control register
    #[inline(always)]
    pub const fn spi_smem_din_mode(&self) -> &SPI_SMEM_DIN_MODE {
        &self.spi_smem_din_mode
    }
    ///0x198 - MSPI external RAM input timing delay number control register
    #[inline(always)]
    pub const fn spi_smem_din_num(&self) -> &SPI_SMEM_DIN_NUM {
        &self.spi_smem_din_num
    }
    ///0x19c - MSPI external RAM output timing adjustment control register
    #[inline(always)]
    pub const fn spi_smem_dout_mode(&self) -> &SPI_SMEM_DOUT_MODE {
        &self.spi_smem_dout_mode
    }
    ///0x1a0 - MSPI external RAM ECC and SPI CS timing control register
    #[inline(always)]
    pub const fn spi_smem_ac(&self) -> &SPI_SMEM_AC {
        &self.spi_smem_ac
    }
    ///0x200 - SPI0 clock gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x300 - The base address of the memory that stores plaintext in Manual Encryption
    #[inline(always)]
    pub const fn xts_plain_base(&self) -> &XTS_PLAIN_BASE {
        &self.xts_plain_base
    }
    ///0x340 - Manual Encryption Line-Size register
    #[inline(always)]
    pub const fn xts_linesize(&self) -> &XTS_LINESIZE {
        &self.xts_linesize
    }
    ///0x344 - Manual Encryption destination register
    #[inline(always)]
    pub const fn xts_destination(&self) -> &XTS_DESTINATION {
        &self.xts_destination
    }
    ///0x348 - Manual Encryption physical address register
    #[inline(always)]
    pub const fn xts_physical_address(&self) -> &XTS_PHYSICAL_ADDRESS {
        &self.xts_physical_address
    }
    ///0x34c - Manual Encryption physical address register
    #[inline(always)]
    pub const fn xts_trigger(&self) -> &XTS_TRIGGER {
        &self.xts_trigger
    }
    ///0x350 - Manual Encryption physical address register
    #[inline(always)]
    pub const fn xts_release(&self) -> &XTS_RELEASE {
        &self.xts_release
    }
    ///0x354 - Manual Encryption physical address register
    #[inline(always)]
    pub const fn xts_destroy(&self) -> &XTS_DESTROY {
        &self.xts_destroy
    }
    ///0x358 - Manual Encryption physical address register
    #[inline(always)]
    pub const fn xts_state(&self) -> &XTS_STATE {
        &self.xts_state
    }
    ///0x35c - Manual Encryption version register
    #[inline(always)]
    pub const fn xts_date(&self) -> &XTS_DATE {
        &self.xts_date
    }
    ///0x37c - MSPI-MMU item content register
    #[inline(always)]
    pub const fn mmu_item_content(&self) -> &MMU_ITEM_CONTENT {
        &self.mmu_item_content
    }
    ///0x380 - MSPI-MMU item index register
    #[inline(always)]
    pub const fn mmu_item_index(&self) -> &MMU_ITEM_INDEX {
        &self.mmu_item_index
    }
    ///0x384 - MSPI MMU power control register
    #[inline(always)]
    pub const fn mmu_power_ctrl(&self) -> &MMU_POWER_CTRL {
        &self.mmu_power_ctrl
    }
    ///0x388 - SPI memory cryption DPA register
    #[inline(always)]
    pub const fn dpa_ctrl(&self) -> &DPA_CTRL {
        &self.dpa_ctrl
    }
    ///0x3f0 - MSPI ECO high register
    #[inline(always)]
    pub const fn registerrnd_eco_high(&self) -> &REGISTERRND_ECO_HIGH {
        &self.registerrnd_eco_high
    }
    ///0x3f4 - MSPI ECO low register
    #[inline(always)]
    pub const fn registerrnd_eco_low(&self) -> &REGISTERRND_ECO_LOW {
        &self.registerrnd_eco_low
    }
    ///0x3fc - SPI0 version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CMD (r) register accessor: SPI0 FSM status register

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///SPI0 FSM status register
pub mod cmd;
/**CTRL (rw) register accessor: SPI0 control register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI0 control register.
pub mod ctrl;
/**CTRL1 (rw) register accessor: SPI0 control1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
///SPI0 control1 register.
pub mod ctrl1;
/**CTRL2 (rw) register accessor: SPI0 control2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///SPI0 control2 register.
pub mod ctrl2;
/**CLOCK (rw) register accessor: SPI clock division control register.

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI clock division control register.
pub mod clock;
/**USER (rw) register accessor: SPI0 user register.

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI0 user register.
pub mod user;
/**USER1 (rw) register accessor: SPI0 user1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI0 user1 register.
pub mod user1;
/**USER2 (rw) register accessor: SPI0 user2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI0 user2 register.
pub mod user2;
/**RD_STATUS (rw) register accessor: SPI0 read control register.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_status`] module*/
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
///SPI0 read control register.
pub mod rd_status;
/**MISC (rw) register accessor: SPI0 misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI0 misc register
pub mod misc;
/**CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_fctrl`] module*/
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
///SPI0 bit mode control register.
pub mod cache_fctrl;
/**CACHE_SCTRL (r) register accessor: SPI0 external RAM control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_sctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_sctrl`] module*/
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
///SPI0 external RAM control register
pub mod cache_sctrl;
/**SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_cmd`] module*/
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
///SPI0 external RAM mode control register
pub mod sram_cmd;
/**SRAM_DRD_CMD (r) register accessor: SPI0 external RAM DDR read command control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_drd_cmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_drd_cmd`] module*/
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
///SPI0 external RAM DDR read command control register
pub mod sram_drd_cmd;
/**SRAM_DWR_CMD (r) register accessor: SPI0 external RAM DDR write command control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_dwr_cmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_dwr_cmd`] module*/
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
///SPI0 external RAM DDR write command control register
pub mod sram_dwr_cmd;
/**SRAM_CLK (r) register accessor: SPI0 external RAM clock control register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_clk::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_clk`] module*/
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
///SPI0 external RAM clock control register
pub mod sram_clk;
/**FSM (rw) register accessor: SPI0 FSM status register

You can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsm`] module*/
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
///SPI0 FSM status register
pub mod fsm;
/**INT_ENA (rw) register accessor: SPI0 interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///SPI0 interrupt enable register
pub mod int_ena;
/**INT_CLR (rw) register accessor: SPI0 interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///SPI0 interrupt clear register
pub mod int_clr;
/**INT_RAW (rw) register accessor: SPI0 interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///SPI0 interrupt raw register
pub mod int_raw;
/**INT_ST (r) register accessor: SPI0 interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///SPI0 interrupt status register
pub mod int_st;
/**DDR (r) register accessor: SPI0 flash DDR mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`ddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddr`] module*/
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
///SPI0 flash DDR mode control register
pub mod ddr;
/**SPI_SMEM_DDR (r) register accessor: SPI0 external RAM DDR mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_ddr`] module*/
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
///SPI0 external RAM DDR mode control register
pub mod spi_smem_ddr;
/**SPI_FMEM_PMS_ATTR (rw) register accessor: MSPI flash ACE section %s attribute register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_fmem_pms_attr`] module*/
pub type SPI_FMEM_PMS_ATTR = crate::Reg<spi_fmem_pms_attr::SPI_FMEM_PMS_ATTR_SPEC>;
///MSPI flash ACE section %s attribute register
pub mod spi_fmem_pms_attr;
/**SPI_FMEM_PMS_ADDR (rw) register accessor: SPI1 flash ACE section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_fmem_pms_addr`] module*/
pub type SPI_FMEM_PMS_ADDR = crate::Reg<spi_fmem_pms_addr::SPI_FMEM_PMS_ADDR_SPEC>;
///SPI1 flash ACE section %s start address register
pub mod spi_fmem_pms_addr;
/**SPI_FMEM_PMS_SIZE (rw) register accessor: SPI1 flash ACE section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_fmem_pms_size`] module*/
pub type SPI_FMEM_PMS_SIZE = crate::Reg<spi_fmem_pms_size::SPI_FMEM_PMS_SIZE_SPEC>;
///SPI1 flash ACE section %s start address register
pub mod spi_fmem_pms_size;
/**SPI_SMEM_PMS_ATTR (rw) register accessor: SPI1 flash ACE section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_pms_attr`] module*/
pub type SPI_SMEM_PMS_ATTR = crate::Reg<spi_smem_pms_attr::SPI_SMEM_PMS_ATTR_SPEC>;
///SPI1 flash ACE section %s start address register
pub mod spi_smem_pms_attr;
/**SPI_SMEM_PMS_ADDR (rw) register accessor: SPI1 external RAM ACE section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_pms_addr`] module*/
pub type SPI_SMEM_PMS_ADDR = crate::Reg<spi_smem_pms_addr::SPI_SMEM_PMS_ADDR_SPEC>;
///SPI1 external RAM ACE section %s start address register
pub mod spi_smem_pms_addr;
/**SPI_SMEM_PMS_SIZE (rw) register accessor: SPI1 external RAM ACE section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_pms_size`] module*/
pub type SPI_SMEM_PMS_SIZE = crate::Reg<spi_smem_pms_size::SPI_SMEM_PMS_SIZE_SPEC>;
///SPI1 external RAM ACE section %s start address register
pub mod spi_smem_pms_size;
/**PMS_REJECT (rw) register accessor: SPI1 access reject register

You can [`read`](crate::generic::Reg::read) this register and get [`pms_reject::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pms_reject::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pms_reject`] module*/
pub type PMS_REJECT = crate::Reg<pms_reject::PMS_REJECT_SPEC>;
///SPI1 access reject register
pub mod pms_reject;
/**ECC_CTRL (rw) register accessor: MSPI ECC control register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_ctrl`] module*/
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
///MSPI ECC control register
pub mod ecc_ctrl;
/**ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_err_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_err_addr`] module*/
pub type ECC_ERR_ADDR = crate::Reg<ecc_err_addr::ECC_ERR_ADDR_SPEC>;
///MSPI ECC error address register
pub mod ecc_err_addr;
/**AXI_ERR_ADDR (r) register accessor: SPI0 AXI request error address.

You can [`read`](crate::generic::Reg::read) this register and get [`axi_err_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@axi_err_addr`] module*/
pub type AXI_ERR_ADDR = crate::Reg<axi_err_addr::AXI_ERR_ADDR_SPEC>;
///SPI0 AXI request error address.
pub mod axi_err_addr;
/**SPI_SMEM_ECC_CTRL (r) register accessor: MSPI ECC control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ecc_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_ecc_ctrl`] module*/
pub type SPI_SMEM_ECC_CTRL = crate::Reg<spi_smem_ecc_ctrl::SPI_SMEM_ECC_CTRL_SPEC>;
///MSPI ECC control register
pub mod spi_smem_ecc_ctrl;
/**TIMING_CALI (rw) register accessor: SPI0 flash timing calibration register

You can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing_cali`] module*/
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
///SPI0 flash timing calibration register
pub mod timing_cali;
/**DIN_MODE (rw) register accessor: MSPI flash input timing delay mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_mode`] module*/
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
///MSPI flash input timing delay mode control register
pub mod din_mode;
/**DIN_NUM (rw) register accessor: MSPI flash input timing delay number control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@din_num`] module*/
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
///MSPI flash input timing delay number control register
pub mod din_num;
/**DOUT_MODE (rw) register accessor: MSPI flash output timing adjustment control register

You can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dout_mode`] module*/
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
///MSPI flash output timing adjustment control register
pub mod dout_mode;
/**SPI_SMEM_TIMING_CALI (r) register accessor: MSPI external RAM timing calibration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_timing_cali::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_timing_cali`] module*/
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
///MSPI external RAM timing calibration register
pub mod spi_smem_timing_cali;
/**SPI_SMEM_DIN_MODE (r) register accessor: MSPI external RAM input timing delay mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_din_mode`] module*/
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
///MSPI external RAM input timing delay mode control register
pub mod spi_smem_din_mode;
/**SPI_SMEM_DIN_NUM (r) register accessor: MSPI external RAM input timing delay number control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_din_num`] module*/
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
///MSPI external RAM input timing delay number control register
pub mod spi_smem_din_num;
/**SPI_SMEM_DOUT_MODE (r) register accessor: MSPI external RAM output timing adjustment control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_dout_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_dout_mode`] module*/
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
///MSPI external RAM output timing adjustment control register
pub mod spi_smem_dout_mode;
/**SPI_SMEM_AC (r) register accessor: MSPI external RAM ECC and SPI CS timing control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ac::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi_smem_ac`] module*/
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
///MSPI external RAM ECC and SPI CS timing control register
pub mod spi_smem_ac;
/**CLOCK_GATE (rw) register accessor: SPI0 clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///SPI0 clock gate register
pub mod clock_gate;
/**XTS_PLAIN_BASE (rw) register accessor: The base address of the memory that stores plaintext in Manual Encryption

You can [`read`](crate::generic::Reg::read) this register and get [`xts_plain_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_plain_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_plain_base`] module*/
pub type XTS_PLAIN_BASE = crate::Reg<xts_plain_base::XTS_PLAIN_BASE_SPEC>;
///The base address of the memory that stores plaintext in Manual Encryption
pub mod xts_plain_base;
/**XTS_LINESIZE (rw) register accessor: Manual Encryption Line-Size register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_linesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_linesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_linesize`] module*/
pub type XTS_LINESIZE = crate::Reg<xts_linesize::XTS_LINESIZE_SPEC>;
///Manual Encryption Line-Size register
pub mod xts_linesize;
/**XTS_DESTINATION (rw) register accessor: Manual Encryption destination register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_destination`] module*/
pub type XTS_DESTINATION = crate::Reg<xts_destination::XTS_DESTINATION_SPEC>;
///Manual Encryption destination register
pub mod xts_destination;
/**XTS_PHYSICAL_ADDRESS (rw) register accessor: Manual Encryption physical address register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_physical_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_physical_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_physical_address`] module*/
pub type XTS_PHYSICAL_ADDRESS = crate::Reg<xts_physical_address::XTS_PHYSICAL_ADDRESS_SPEC>;
///Manual Encryption physical address register
pub mod xts_physical_address;
/**XTS_TRIGGER (w) register accessor: Manual Encryption physical address register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_trigger`] module*/
pub type XTS_TRIGGER = crate::Reg<xts_trigger::XTS_TRIGGER_SPEC>;
///Manual Encryption physical address register
pub mod xts_trigger;
/**XTS_RELEASE (w) register accessor: Manual Encryption physical address register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_release`] module*/
pub type XTS_RELEASE = crate::Reg<xts_release::XTS_RELEASE_SPEC>;
///Manual Encryption physical address register
pub mod xts_release;
/**XTS_DESTROY (w) register accessor: Manual Encryption physical address register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_destroy`] module*/
pub type XTS_DESTROY = crate::Reg<xts_destroy::XTS_DESTROY_SPEC>;
///Manual Encryption physical address register
pub mod xts_destroy;
/**XTS_STATE (r) register accessor: Manual Encryption physical address register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_state`] module*/
pub type XTS_STATE = crate::Reg<xts_state::XTS_STATE_SPEC>;
///Manual Encryption physical address register
pub mod xts_state;
/**XTS_DATE (rw) register accessor: Manual Encryption version register

You can [`read`](crate::generic::Reg::read) this register and get [`xts_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xts_date`] module*/
pub type XTS_DATE = crate::Reg<xts_date::XTS_DATE_SPEC>;
///Manual Encryption version register
pub mod xts_date;
/**MMU_ITEM_CONTENT (rw) register accessor: MSPI-MMU item content register

You can [`read`](crate::generic::Reg::read) this register and get [`mmu_item_content::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_item_content::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmu_item_content`] module*/
pub type MMU_ITEM_CONTENT = crate::Reg<mmu_item_content::MMU_ITEM_CONTENT_SPEC>;
///MSPI-MMU item content register
pub mod mmu_item_content;
/**MMU_ITEM_INDEX (rw) register accessor: MSPI-MMU item index register

You can [`read`](crate::generic::Reg::read) this register and get [`mmu_item_index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_item_index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmu_item_index`] module*/
pub type MMU_ITEM_INDEX = crate::Reg<mmu_item_index::MMU_ITEM_INDEX_SPEC>;
///MSPI-MMU item index register
pub mod mmu_item_index;
/**MMU_POWER_CTRL (rw) register accessor: MSPI MMU power control register

You can [`read`](crate::generic::Reg::read) this register and get [`mmu_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmu_power_ctrl`] module*/
pub type MMU_POWER_CTRL = crate::Reg<mmu_power_ctrl::MMU_POWER_CTRL_SPEC>;
///MSPI MMU power control register
pub mod mmu_power_ctrl;
/**DPA_CTRL (rw) register accessor: SPI memory cryption DPA register

You can [`read`](crate::generic::Reg::read) this register and get [`dpa_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpa_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpa_ctrl`] module*/
pub type DPA_CTRL = crate::Reg<dpa_ctrl::DPA_CTRL_SPEC>;
///SPI memory cryption DPA register
pub mod dpa_ctrl;
/**REGISTERRND_ECO_HIGH (r) register accessor: MSPI ECO high register

You can [`read`](crate::generic::Reg::read) this register and get [`registerrnd_eco_high::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@registerrnd_eco_high`] module*/
pub type REGISTERRND_ECO_HIGH = crate::Reg<registerrnd_eco_high::REGISTERRND_ECO_HIGH_SPEC>;
///MSPI ECO high register
pub mod registerrnd_eco_high;
/**REGISTERRND_ECO_LOW (r) register accessor: MSPI ECO low register

You can [`read`](crate::generic::Reg::read) this register and get [`registerrnd_eco_low::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@registerrnd_eco_low`] module*/
pub type REGISTERRND_ECO_LOW = crate::Reg<registerrnd_eco_low::REGISTERRND_ECO_LOW_SPEC>;
///MSPI ECO low register
pub mod registerrnd_eco_low;
/**DATE (rw) register accessor: SPI0 version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///SPI0 version control register
pub mod date;
