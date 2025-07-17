#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    smem_ddr: SMEM_DDR,
    _reserved23: [u8; 0x24],
    fmem_pms0_attr: FMEM_PMS0_ATTR,
    fmem_pms1_attr: FMEM_PMS1_ATTR,
    fmem_pms2_attr: FMEM_PMS2_ATTR,
    fmem_pms3_attr: FMEM_PMS3_ATTR,
    fmem_pms_addr: [FMEM_PMS_ADDR; 4],
    fmem_pms_size: [FMEM_PMS_SIZE; 4],
    smem_pms0_attr: SMEM_PMS0_ATTR,
    smem_pms1_attr: SMEM_PMS1_ATTR,
    smem_pms2_attr: SMEM_PMS2_ATTR,
    smem_pms3_attr: SMEM_PMS3_ATTR,
    smem_pms_addr: [SMEM_PMS_ADDR; 4],
    smem_pms_size: [SMEM_PMS_SIZE; 4],
    pms_reject: PMS_REJECT,
    pms_reject_addr: PMS_REJECT_ADDR,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    axi_err_addr: AXI_ERR_ADDR,
    smem_ecc_ctrl: SMEM_ECC_CTRL,
    smem_axi_addr_ctrl: SMEM_AXI_ADDR_CTRL,
    axi_err_resp_en: AXI_ERR_RESP_EN,
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    smem_timing_cali: SMEM_TIMING_CALI,
    smem_din_mode: SMEM_DIN_MODE,
    smem_din_num: SMEM_DIN_NUM,
    smem_dout_mode: SMEM_DOUT_MODE,
    smem_ac: SMEM_AC,
    smem_din_hex_mode: SMEM_DIN_HEX_MODE,
    smem_din_hex_num: SMEM_DIN_HEX_NUM,
    smem_dout_hex_mode: SMEM_DOUT_HEX_MODE,
    _reserved55: [u8; 0x50],
    clock_gate: CLOCK_GATE,
    nand_flash_en: NAND_FLASH_EN,
    nand_flash_sr_addr0: NAND_FLASH_SR_ADDR0,
    nand_flash_sr_din0: NAND_FLASH_SR_DIN0,
    nand_flash_cfg_data0: NAND_FLASH_CFG_DATA0,
    nand_flash_cfg_data1: NAND_FLASH_CFG_DATA1,
    nand_flash_cfg_data2: NAND_FLASH_CFG_DATA2,
    _reserved62: [u8; 0x24],
    nand_flash_cmd_lut0: NAND_FLASH_CMD_LUT0,
    _reserved63: [u8; 0x3c],
    nand_flash_spi_seq0: NAND_FLASH_SPI_SEQ0,
    _reserved64: [u8; 0x7c],
    xts_plain_base: XTS_PLAIN_BASE,
    _reserved65: [u8; 0x3c],
    xts_linesize: XTS_LINESIZE,
    xts_destination: XTS_DESTINATION,
    xts_physical_address: XTS_PHYSICAL_ADDRESS,
    xts_trigger: XTS_TRIGGER,
    xts_release: XTS_RELEASE,
    xts_destroy: XTS_DESTROY,
    xts_state: XTS_STATE,
    xts_date: XTS_DATE,
    _reserved73: [u8; 0x1c],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    dpa_ctrl: DPA_CTRL,
    xts_pseudo_round_conf: XTS_PSEUDO_ROUND_CONF,
    _reserved78: [u8; 0x60],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved80: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x08 - SPI0 control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - SPI0 control1 register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - SPI0 control2 register."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - SPI clock division control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - SPI0 user register."]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - SPI0 user1 register."]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - SPI0 user2 register."]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x2c - SPI0 read control register."]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x34 - SPI0 misc register"]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x3c - SPI0 bit mode control register."]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x40 - SPI0 external RAM control register"]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    #[doc = "0x50 - SPI0 external RAM clock control register"]
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        &self.sram_clk
    }
    #[doc = "0x54 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0xc0 - SPI0 interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xc4 - SPI0 interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xc8 - SPI0 interrupt raw register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xcc - SPI0 interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xd4 - SPI0 flash DDR mode control register"]
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    #[doc = "0xd8 - SPI0 external RAM DDR mode control register"]
    #[inline(always)]
    pub const fn smem_ddr(&self) -> &SMEM_DDR {
        &self.smem_ddr
    }
    #[doc = "0x100 - MSPI flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn fmem_pms0_attr(&self) -> &FMEM_PMS0_ATTR {
        &self.fmem_pms0_attr
    }
    #[doc = "0x104 - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn fmem_pms1_attr(&self) -> &FMEM_PMS1_ATTR {
        &self.fmem_pms1_attr
    }
    #[doc = "0x108 - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn fmem_pms2_attr(&self) -> &FMEM_PMS2_ATTR {
        &self.fmem_pms2_attr
    }
    #[doc = "0x10c - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn fmem_pms3_attr(&self) -> &FMEM_PMS3_ATTR {
        &self.fmem_pms3_attr
    }
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn fmem_pms_addr(&self, n: usize) -> &FMEM_PMS_ADDR {
        &self.fmem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn fmem_pms_addr_iter(&self) -> impl Iterator<Item = &FMEM_PMS_ADDR> {
        self.fmem_pms_addr.iter()
    }
    #[doc = "0x110 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn fmem_pms0_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(0)
    }
    #[doc = "0x114 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn fmem_pms1_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(1)
    }
    #[doc = "0x118 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn fmem_pms2_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(2)
    }
    #[doc = "0x11c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn fmem_pms3_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(3)
    }
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn fmem_pms_size(&self, n: usize) -> &FMEM_PMS_SIZE {
        &self.fmem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn fmem_pms_size_iter(&self) -> impl Iterator<Item = &FMEM_PMS_SIZE> {
        self.fmem_pms_size.iter()
    }
    #[doc = "0x120 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn fmem_pms0_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(0)
    }
    #[doc = "0x124 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn fmem_pms1_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(1)
    }
    #[doc = "0x128 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn fmem_pms2_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(2)
    }
    #[doc = "0x12c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn fmem_pms3_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(3)
    }
    #[doc = "0x130 - SPI1 flash PMS section $n start address register"]
    #[inline(always)]
    pub const fn smem_pms0_attr(&self) -> &SMEM_PMS0_ATTR {
        &self.smem_pms0_attr
    }
    #[doc = "0x134 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn smem_pms1_attr(&self) -> &SMEM_PMS1_ATTR {
        &self.smem_pms1_attr
    }
    #[doc = "0x138 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn smem_pms2_attr(&self) -> &SMEM_PMS2_ATTR {
        &self.smem_pms2_attr
    }
    #[doc = "0x13c - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn smem_pms3_attr(&self) -> &SMEM_PMS3_ATTR {
        &self.smem_pms3_attr
    }
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn smem_pms_addr(&self, n: usize) -> &SMEM_PMS_ADDR {
        &self.smem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn smem_pms_addr_iter(&self) -> impl Iterator<Item = &SMEM_PMS_ADDR> {
        self.smem_pms_addr.iter()
    }
    #[doc = "0x140 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn smem_pms0_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(0)
    }
    #[doc = "0x144 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn smem_pms1_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(1)
    }
    #[doc = "0x148 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn smem_pms2_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(2)
    }
    #[doc = "0x14c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn smem_pms3_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(3)
    }
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn smem_pms_size(&self, n: usize) -> &SMEM_PMS_SIZE {
        &self.smem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn smem_pms_size_iter(&self) -> impl Iterator<Item = &SMEM_PMS_SIZE> {
        self.smem_pms_size.iter()
    }
    #[doc = "0x150 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn smem_pms0_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(0)
    }
    #[doc = "0x154 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn smem_pms1_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(1)
    }
    #[doc = "0x158 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn smem_pms2_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(2)
    }
    #[doc = "0x15c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn smem_pms3_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(3)
    }
    #[doc = "0x160 - SPI1 access reject register"]
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PMS_REJECT {
        &self.pms_reject
    }
    #[doc = "0x164 - SPI1 access reject addr register"]
    #[inline(always)]
    pub const fn pms_reject_addr(&self) -> &PMS_REJECT_ADDR {
        &self.pms_reject_addr
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
    #[doc = "0x170 - SPI0 AXI request error address."]
    #[inline(always)]
    pub const fn axi_err_addr(&self) -> &AXI_ERR_ADDR {
        &self.axi_err_addr
    }
    #[doc = "0x174 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn smem_ecc_ctrl(&self) -> &SMEM_ECC_CTRL {
        &self.smem_ecc_ctrl
    }
    #[doc = "0x178 - SPI0 AXI address control register"]
    #[inline(always)]
    pub const fn smem_axi_addr_ctrl(&self) -> &SMEM_AXI_ADDR_CTRL {
        &self.smem_axi_addr_ctrl
    }
    #[doc = "0x17c - SPI0 AXI error response enable register"]
    #[inline(always)]
    pub const fn axi_err_resp_en(&self) -> &AXI_ERR_RESP_EN {
        &self.axi_err_resp_en
    }
    #[doc = "0x180 - SPI0 flash timing calibration register"]
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
    pub const fn smem_timing_cali(&self) -> &SMEM_TIMING_CALI {
        &self.smem_timing_cali
    }
    #[doc = "0x194 - MSPI external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn smem_din_mode(&self) -> &SMEM_DIN_MODE {
        &self.smem_din_mode
    }
    #[doc = "0x198 - MSPI external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn smem_din_num(&self) -> &SMEM_DIN_NUM {
        &self.smem_din_num
    }
    #[doc = "0x19c - MSPI external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn smem_dout_mode(&self) -> &SMEM_DOUT_MODE {
        &self.smem_dout_mode
    }
    #[doc = "0x1a0 - MSPI external RAM ECC and SPI CS timing control register"]
    #[inline(always)]
    pub const fn smem_ac(&self) -> &SMEM_AC {
        &self.smem_ac
    }
    #[doc = "0x1a4 - MSPI 16x external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn smem_din_hex_mode(&self) -> &SMEM_DIN_HEX_MODE {
        &self.smem_din_hex_mode
    }
    #[doc = "0x1a8 - MSPI 16x external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn smem_din_hex_num(&self) -> &SMEM_DIN_HEX_NUM {
        &self.smem_din_hex_num
    }
    #[doc = "0x1ac - MSPI 16x external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn smem_dout_hex_mode(&self) -> &SMEM_DOUT_HEX_MODE {
        &self.smem_dout_hex_mode
    }
    #[doc = "0x200 - SPI0 clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x204 - NAND FLASH control register"]
    #[inline(always)]
    pub const fn nand_flash_en(&self) -> &NAND_FLASH_EN {
        &self.nand_flash_en
    }
    #[doc = "0x208 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_sr_addr0(&self) -> &NAND_FLASH_SR_ADDR0 {
        &self.nand_flash_sr_addr0
    }
    #[doc = "0x20c - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_sr_din0(&self) -> &NAND_FLASH_SR_DIN0 {
        &self.nand_flash_sr_din0
    }
    #[doc = "0x210 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_cfg_data0(&self) -> &NAND_FLASH_CFG_DATA0 {
        &self.nand_flash_cfg_data0
    }
    #[doc = "0x214 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_cfg_data1(&self) -> &NAND_FLASH_CFG_DATA1 {
        &self.nand_flash_cfg_data1
    }
    #[doc = "0x218 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_cfg_data2(&self) -> &NAND_FLASH_CFG_DATA2 {
        &self.nand_flash_cfg_data2
    }
    #[doc = "0x240 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut0(&self) -> &NAND_FLASH_CMD_LUT0 {
        &self.nand_flash_cmd_lut0
    }
    #[doc = "0x280 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq0(&self) -> &NAND_FLASH_SPI_SEQ0 {
        &self.nand_flash_spi_seq0
    }
    #[doc = "0x300 - The base address of the memory that stores plaintext in Manual Encryption"]
    #[inline(always)]
    pub const fn xts_plain_base(&self) -> &XTS_PLAIN_BASE {
        &self.xts_plain_base
    }
    #[doc = "0x340 - Manual Encryption Line-Size register"]
    #[inline(always)]
    pub const fn xts_linesize(&self) -> &XTS_LINESIZE {
        &self.xts_linesize
    }
    #[doc = "0x344 - Manual Encryption destination register"]
    #[inline(always)]
    pub const fn xts_destination(&self) -> &XTS_DESTINATION {
        &self.xts_destination
    }
    #[doc = "0x348 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_physical_address(&self) -> &XTS_PHYSICAL_ADDRESS {
        &self.xts_physical_address
    }
    #[doc = "0x34c - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_trigger(&self) -> &XTS_TRIGGER {
        &self.xts_trigger
    }
    #[doc = "0x350 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_release(&self) -> &XTS_RELEASE {
        &self.xts_release
    }
    #[doc = "0x354 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_destroy(&self) -> &XTS_DESTROY {
        &self.xts_destroy
    }
    #[doc = "0x358 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn xts_state(&self) -> &XTS_STATE {
        &self.xts_state
    }
    #[doc = "0x35c - Manual Encryption version register"]
    #[inline(always)]
    pub const fn xts_date(&self) -> &XTS_DATE {
        &self.xts_date
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
    #[doc = "0x388 - SPI memory cryption DPA register"]
    #[inline(always)]
    pub const fn dpa_ctrl(&self) -> &DPA_CTRL {
        &self.dpa_ctrl
    }
    #[doc = "0x38c - SPI memory cryption PSEUDO register"]
    #[inline(always)]
    pub const fn xts_pseudo_round_conf(&self) -> &XTS_PSEUDO_ROUND_CONF {
        &self.xts_pseudo_round_conf
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
    #[doc = "0x3fc - SPI0 version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (r) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: SPI0 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: SPI0 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SPI0 control2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: SPI clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod clock;
#[doc = "USER (rw) register accessor: SPI0 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod user;
#[doc = "USER1 (rw) register accessor: SPI0 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod user1;
#[doc = "USER2 (rw) register accessor: SPI0 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod user2;
#[doc = "RD_STATUS (rw) register accessor: SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod misc;
#[doc = "CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (rw) register accessor: SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
#[doc = "SPI0 external RAM clock control register"]
pub mod sram_clk;
#[doc = "FSM (rw) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod fsm;
#[doc = "INT_ENA (rw) register accessor: SPI0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SPI0 interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: SPI0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SPI0 interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SPI0 interrupt raw register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SPI0 interrupt status register"]
pub mod int_st;
#[doc = "DDR (rw) register accessor: SPI0 flash DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod ddr;
#[doc = "SMEM_DDR (rw) register accessor: SPI0 external RAM DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ddr`] module"]
pub type SMEM_DDR = crate::Reg<smem_ddr::SMEM_DDR_SPEC>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod smem_ddr;
#[doc = "FMEM_PMS0_ATTR (rw) register accessor: MSPI flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms0_attr`] module"]
pub type FMEM_PMS0_ATTR = crate::Reg<fmem_pms0_attr::FMEM_PMS0_ATTR_SPEC>;
#[doc = "MSPI flash PMS section $n attribute register"]
pub mod fmem_pms0_attr;
#[doc = "FMEM_PMS1_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms1_attr`] module"]
pub type FMEM_PMS1_ATTR = crate::Reg<fmem_pms1_attr::FMEM_PMS1_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod fmem_pms1_attr;
#[doc = "FMEM_PMS2_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms2_attr`] module"]
pub type FMEM_PMS2_ATTR = crate::Reg<fmem_pms2_attr::FMEM_PMS2_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod fmem_pms2_attr;
#[doc = "FMEM_PMS3_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms3_attr`] module"]
pub type FMEM_PMS3_ATTR = crate::Reg<fmem_pms3_attr::FMEM_PMS3_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod fmem_pms3_attr;
#[doc = "FMEM_PMS_ADDR (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms_addr`] module"]
pub type FMEM_PMS_ADDR = crate::Reg<fmem_pms_addr::FMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod fmem_pms_addr;
#[doc = "FMEM_PMS_SIZE (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms_size`] module"]
pub type FMEM_PMS_SIZE = crate::Reg<fmem_pms_size::FMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod fmem_pms_size;
#[doc = "SMEM_PMS0_ATTR (rw) register accessor: SPI1 flash PMS section $n start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms0_attr`] module"]
pub type SMEM_PMS0_ATTR = crate::Reg<smem_pms0_attr::SMEM_PMS0_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n start address register"]
pub mod smem_pms0_attr;
#[doc = "SMEM_PMS1_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms1_attr`] module"]
pub type SMEM_PMS1_ATTR = crate::Reg<smem_pms1_attr::SMEM_PMS1_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod smem_pms1_attr;
#[doc = "SMEM_PMS2_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms2_attr`] module"]
pub type SMEM_PMS2_ATTR = crate::Reg<smem_pms2_attr::SMEM_PMS2_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod smem_pms2_attr;
#[doc = "SMEM_PMS3_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms3_attr`] module"]
pub type SMEM_PMS3_ATTR = crate::Reg<smem_pms3_attr::SMEM_PMS3_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod smem_pms3_attr;
#[doc = "SMEM_PMS_ADDR (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms_addr`] module"]
pub type SMEM_PMS_ADDR = crate::Reg<smem_pms_addr::SMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod smem_pms_addr;
#[doc = "SMEM_PMS_SIZE (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms_size`] module"]
pub type SMEM_PMS_SIZE = crate::Reg<smem_pms_size::SMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod smem_pms_size;
#[doc = "PMS_REJECT (rw) register accessor: SPI1 access reject register\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject`] module"]
pub type PMS_REJECT = crate::Reg<pms_reject::PMS_REJECT_SPEC>;
#[doc = "SPI1 access reject register"]
pub mod pms_reject;
#[doc = "PMS_REJECT_ADDR (r) register accessor: SPI1 access reject addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject_addr`] module"]
pub type PMS_REJECT_ADDR = crate::Reg<pms_reject_addr::PMS_REJECT_ADDR_SPEC>;
#[doc = "SPI1 access reject addr register"]
pub mod pms_reject_addr;
#[doc = "ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctrl`] module"]
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod ecc_ctrl;
#[doc = "ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_err_addr`] module"]
pub type ECC_ERR_ADDR = crate::Reg<ecc_err_addr::ECC_ERR_ADDR_SPEC>;
#[doc = "MSPI ECC error address register"]
pub mod ecc_err_addr;
#[doc = "AXI_ERR_ADDR (r) register accessor: SPI0 AXI request error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_addr`] module"]
pub type AXI_ERR_ADDR = crate::Reg<axi_err_addr::AXI_ERR_ADDR_SPEC>;
#[doc = "SPI0 AXI request error address."]
pub mod axi_err_addr;
#[doc = "SMEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ecc_ctrl`] module"]
pub type SMEM_ECC_CTRL = crate::Reg<smem_ecc_ctrl::SMEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod smem_ecc_ctrl;
#[doc = "SMEM_AXI_ADDR_CTRL (r) register accessor: SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_axi_addr_ctrl`] module"]
pub type SMEM_AXI_ADDR_CTRL = crate::Reg<smem_axi_addr_ctrl::SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "SPI0 AXI address control register"]
pub mod smem_axi_addr_ctrl;
#[doc = "AXI_ERR_RESP_EN (rw) register accessor: SPI0 AXI error response enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_resp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_resp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_resp_en`] module"]
pub type AXI_ERR_RESP_EN = crate::Reg<axi_err_resp_en::AXI_ERR_RESP_EN_SPEC>;
#[doc = "SPI0 AXI error response enable register"]
pub mod axi_err_resp_en;
#[doc = "TIMING_CALI (rw) register accessor: SPI0 flash timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI0 flash timing calibration register"]
pub mod timing_cali;
#[doc = "DIN_MODE (rw) register accessor: MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "MSPI flash input timing delay mode control register"]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: MSPI flash input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "MSPI flash input timing delay number control register"]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "MSPI flash output timing adjustment control register"]
pub mod dout_mode;
#[doc = "SMEM_TIMING_CALI (rw) register accessor: MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_timing_cali`] module"]
pub type SMEM_TIMING_CALI = crate::Reg<smem_timing_cali::SMEM_TIMING_CALI_SPEC>;
#[doc = "MSPI external RAM timing calibration register"]
pub mod smem_timing_cali;
#[doc = "SMEM_DIN_MODE (rw) register accessor: MSPI external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_mode`] module"]
pub type SMEM_DIN_MODE = crate::Reg<smem_din_mode::SMEM_DIN_MODE_SPEC>;
#[doc = "MSPI external RAM input timing delay mode control register"]
pub mod smem_din_mode;
#[doc = "SMEM_DIN_NUM (rw) register accessor: MSPI external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_num`] module"]
pub type SMEM_DIN_NUM = crate::Reg<smem_din_num::SMEM_DIN_NUM_SPEC>;
#[doc = "MSPI external RAM input timing delay number control register"]
pub mod smem_din_num;
#[doc = "SMEM_DOUT_MODE (rw) register accessor: MSPI external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_dout_mode`] module"]
pub type SMEM_DOUT_MODE = crate::Reg<smem_dout_mode::SMEM_DOUT_MODE_SPEC>;
#[doc = "MSPI external RAM output timing adjustment control register"]
pub mod smem_dout_mode;
#[doc = "SMEM_AC (rw) register accessor: MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ac`] module"]
pub type SMEM_AC = crate::Reg<smem_ac::SMEM_AC_SPEC>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod smem_ac;
#[doc = "SMEM_DIN_HEX_MODE (r) register accessor: MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_hex_mode`] module"]
pub type SMEM_DIN_HEX_MODE = crate::Reg<smem_din_hex_mode::SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "MSPI 16x external RAM input timing delay mode control register"]
pub mod smem_din_hex_mode;
#[doc = "SMEM_DIN_HEX_NUM (r) register accessor: MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_hex_num`] module"]
pub type SMEM_DIN_HEX_NUM = crate::Reg<smem_din_hex_num::SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "MSPI 16x external RAM input timing delay number control register"]
pub mod smem_din_hex_num;
#[doc = "SMEM_DOUT_HEX_MODE (r) register accessor: MSPI 16x external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_hex_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_dout_hex_mode`] module"]
pub type SMEM_DOUT_HEX_MODE = crate::Reg<smem_dout_hex_mode::SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = "MSPI 16x external RAM output timing adjustment control register"]
pub mod smem_dout_hex_mode;
#[doc = "CLOCK_GATE (rw) register accessor: SPI0 clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI0 clock gate register"]
pub mod clock_gate;
#[doc = "NAND_FLASH_EN (r) register accessor: NAND FLASH control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_en::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_en`] module"]
pub type NAND_FLASH_EN = crate::Reg<nand_flash_en::NAND_FLASH_EN_SPEC>;
#[doc = "NAND FLASH control register"]
pub mod nand_flash_en;
#[doc = "NAND_FLASH_SR_ADDR0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_sr_addr0`] module"]
pub type NAND_FLASH_SR_ADDR0 = crate::Reg<nand_flash_sr_addr0::NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_sr_addr0;
#[doc = "NAND_FLASH_SR_DIN0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_din0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_sr_din0`] module"]
pub type NAND_FLASH_SR_DIN0 = crate::Reg<nand_flash_sr_din0::NAND_FLASH_SR_DIN0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_sr_din0;
#[doc = "NAND_FLASH_CFG_DATA0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cfg_data0`] module"]
pub type NAND_FLASH_CFG_DATA0 = crate::Reg<nand_flash_cfg_data0::NAND_FLASH_CFG_DATA0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_cfg_data0;
#[doc = "NAND_FLASH_CFG_DATA1 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cfg_data1`] module"]
pub type NAND_FLASH_CFG_DATA1 = crate::Reg<nand_flash_cfg_data1::NAND_FLASH_CFG_DATA1_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_cfg_data1;
#[doc = "NAND_FLASH_CFG_DATA2 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cfg_data2`] module"]
pub type NAND_FLASH_CFG_DATA2 = crate::Reg<nand_flash_cfg_data2::NAND_FLASH_CFG_DATA2_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_cfg_data2;
#[doc = "NAND_FLASH_CMD_LUT0 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut0`] module"]
pub type NAND_FLASH_CMD_LUT0 = crate::Reg<nand_flash_cmd_lut0::NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut0;
#[doc = "NAND_FLASH_SPI_SEQ0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq0`] module"]
pub type NAND_FLASH_SPI_SEQ0 = crate::Reg<nand_flash_spi_seq0::NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq0;
#[doc = "XTS_PLAIN_BASE (rw) register accessor: The base address of the memory that stores plaintext in Manual Encryption\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_plain_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_plain_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_plain_base`] module"]
pub type XTS_PLAIN_BASE = crate::Reg<xts_plain_base::XTS_PLAIN_BASE_SPEC>;
#[doc = "The base address of the memory that stores plaintext in Manual Encryption"]
pub mod xts_plain_base;
#[doc = "XTS_LINESIZE (rw) register accessor: Manual Encryption Line-Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_linesize`] module"]
pub type XTS_LINESIZE = crate::Reg<xts_linesize::XTS_LINESIZE_SPEC>;
#[doc = "Manual Encryption Line-Size register"]
pub mod xts_linesize;
#[doc = "XTS_DESTINATION (rw) register accessor: Manual Encryption destination register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destination`] module"]
pub type XTS_DESTINATION = crate::Reg<xts_destination::XTS_DESTINATION_SPEC>;
#[doc = "Manual Encryption destination register"]
pub mod xts_destination;
#[doc = "XTS_PHYSICAL_ADDRESS (rw) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_physical_address`] module"]
pub type XTS_PHYSICAL_ADDRESS = crate::Reg<xts_physical_address::XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_physical_address;
#[doc = "XTS_TRIGGER (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_trigger`] module"]
pub type XTS_TRIGGER = crate::Reg<xts_trigger::XTS_TRIGGER_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_trigger;
#[doc = "XTS_RELEASE (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_release`] module"]
pub type XTS_RELEASE = crate::Reg<xts_release::XTS_RELEASE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_release;
#[doc = "XTS_DESTROY (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destroy`] module"]
pub type XTS_DESTROY = crate::Reg<xts_destroy::XTS_DESTROY_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_destroy;
#[doc = "XTS_STATE (r) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_state`] module"]
pub type XTS_STATE = crate::Reg<xts_state::XTS_STATE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod xts_state;
pub use crate::dma::date as xts_date;
pub use crate::dma::DATE as XTS_DATE;
#[doc = "MMU_ITEM_CONTENT (rw) register accessor: MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_content::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_content::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_content`] module"]
pub type MMU_ITEM_CONTENT = crate::Reg<mmu_item_content::MMU_ITEM_CONTENT_SPEC>;
#[doc = "MSPI-MMU item content register"]
pub mod mmu_item_content;
#[doc = "MMU_ITEM_INDEX (rw) register accessor: MSPI-MMU item index register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_index`] module"]
pub type MMU_ITEM_INDEX = crate::Reg<mmu_item_index::MMU_ITEM_INDEX_SPEC>;
#[doc = "MSPI-MMU item index register"]
pub mod mmu_item_index;
#[doc = "MMU_POWER_CTRL (rw) register accessor: MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_power_ctrl`] module"]
pub type MMU_POWER_CTRL = crate::Reg<mmu_power_ctrl::MMU_POWER_CTRL_SPEC>;
#[doc = "MSPI MMU power control register"]
pub mod mmu_power_ctrl;
#[doc = "DPA_CTRL (rw) register accessor: SPI memory cryption DPA register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_ctrl`] module"]
pub type DPA_CTRL = crate::Reg<dpa_ctrl::DPA_CTRL_SPEC>;
#[doc = "SPI memory cryption DPA register"]
pub mod dpa_ctrl;
#[doc = "XTS_PSEUDO_ROUND_CONF (rw) register accessor: SPI memory cryption PSEUDO register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_pseudo_round_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_pseudo_round_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_pseudo_round_conf`] module"]
pub type XTS_PSEUDO_ROUND_CONF = crate::Reg<xts_pseudo_round_conf::XTS_PSEUDO_ROUND_CONF_SPEC>;
#[doc = "SPI memory cryption PSEUDO register"]
pub mod xts_pseudo_round_conf;
#[doc = "REGISTERRND_ECO_HIGH (rw) register accessor: MSPI ECO high register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_high`] module"]
pub type REGISTERRND_ECO_HIGH = crate::Reg<registerrnd_eco_high::REGISTERRND_ECO_HIGH_SPEC>;
#[doc = "MSPI ECO high register"]
pub mod registerrnd_eco_high;
#[doc = "REGISTERRND_ECO_LOW (rw) register accessor: MSPI ECO low register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_low`] module"]
pub type REGISTERRND_ECO_LOW = crate::Reg<registerrnd_eco_low::REGISTERRND_ECO_LOW_SPEC>;
#[doc = "MSPI ECO low register"]
pub mod registerrnd_eco_low;
pub use crate::dma::date;
pub use crate::dma::DATE;
