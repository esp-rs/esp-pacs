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
    _reserved9: [u8; 0x08],
    rd_status: RD_STATUS,
    _reserved10: [u8; 0x04],
    misc: MISC,
    _reserved11: [u8; 0x04],
    cache_fctrl: CACHE_FCTRL,
    cache_sctrl: CACHE_SCTRL,
    sram_cmd: SRAM_CMD,
    sram_drd_cmd: SRAM_DRD_CMD,
    sram_dwr_cmd: SRAM_DWR_CMD,
    sram_clk: SRAM_CLK,
    fsm: FSM,
    _reserved18: [u8; 0x68],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved22: [u8; 0x04],
    ddr: DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    _reserved24: [u8; 0x24],
    spi_fmem_pms0_attr: SPI_FMEM_PMS0_ATTR,
    spi_fmem_pms1_attr: SPI_FMEM_PMS1_ATTR,
    spi_fmem_pms2_attr: SPI_FMEM_PMS2_ATTR,
    spi_fmem_pms3_attr: SPI_FMEM_PMS3_ATTR,
    spi_fmem_pms_addr: [SPI_FMEM_PMS_ADDR; 4],
    spi_fmem_pms_size: [SPI_FMEM_PMS_SIZE; 4],
    spi_smem_pms0_attr: SPI_SMEM_PMS0_ATTR,
    spi_smem_pms1_attr: SPI_SMEM_PMS1_ATTR,
    spi_smem_pms2_attr: SPI_SMEM_PMS2_ATTR,
    spi_smem_pms3_attr: SPI_SMEM_PMS3_ATTR,
    spi_smem_pms_addr: [SPI_SMEM_PMS_ADDR; 4],
    spi_smem_pms_size: [SPI_SMEM_PMS_SIZE; 4],
    pms_reject: PMS_REJECT,
    pms_reject_addr: PMS_REJECT_ADDR,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    axi_err_addr: AXI_ERR_ADDR,
    spi_smem_ecc_ctrl: SPI_SMEM_ECC_CTRL,
    spi_smem_axi_addr_ctrl: SPI_SMEM_AXI_ADDR_CTRL,
    axi_err_resp_en: AXI_ERR_RESP_EN,
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    spi_smem_ac: SPI_SMEM_AC,
    _reserved53: [u8; 0x5c],
    clock_gate: CLOCK_GATE,
    nand_flash_en: NAND_FLASH_EN,
    nand_flash_sr_addr0: NAND_FLASH_SR_ADDR0,
    nand_flash_sr_din0: NAND_FLASH_SR_DIN0,
    nand_flash_cfg_data0: NAND_FLASH_CFG_DATA0,
    nand_flash_cfg_data1: NAND_FLASH_CFG_DATA1,
    nand_flash_cfg_data2: NAND_FLASH_CFG_DATA2,
    _reserved60: [u8; 0x24],
    nand_flash_cmd_lut0: NAND_FLASH_CMD_LUT0,
    nand_flash_cmd_lut1: NAND_FLASH_CMD_LUT1,
    nand_flash_cmd_lut2: NAND_FLASH_CMD_LUT2,
    nand_flash_cmd_lut3: NAND_FLASH_CMD_LUT3,
    nand_flash_cmd_lut4: NAND_FLASH_CMD_LUT4,
    nand_flash_cmd_lut5: NAND_FLASH_CMD_LUT5,
    nand_flash_cmd_lut6: NAND_FLASH_CMD_LUT6,
    nand_flash_cmd_lut7: NAND_FLASH_CMD_LUT7,
    nand_flash_cmd_lut8: NAND_FLASH_CMD_LUT8,
    nand_flash_cmd_lut9: NAND_FLASH_CMD_LUT9,
    nand_flash_cmd_lut10: NAND_FLASH_CMD_LUT10,
    nand_flash_cmd_lut11: NAND_FLASH_CMD_LUT11,
    nand_flash_cmd_lut12: NAND_FLASH_CMD_LUT12,
    nand_flash_cmd_lut13: NAND_FLASH_CMD_LUT13,
    nand_flash_cmd_lut14: NAND_FLASH_CMD_LUT14,
    nand_flash_cmd_lut15: NAND_FLASH_CMD_LUT15,
    nand_flash_spi_seq0: NAND_FLASH_SPI_SEQ0,
    nand_flash_spi_seq1: NAND_FLASH_SPI_SEQ1,
    nand_flash_spi_seq2: NAND_FLASH_SPI_SEQ2,
    nand_flash_spi_seq3: NAND_FLASH_SPI_SEQ3,
    nand_flash_spi_seq4: NAND_FLASH_SPI_SEQ4,
    nand_flash_spi_seq5: NAND_FLASH_SPI_SEQ5,
    nand_flash_spi_seq6: NAND_FLASH_SPI_SEQ6,
    nand_flash_spi_seq7: NAND_FLASH_SPI_SEQ7,
    nand_flash_spi_seq8: NAND_FLASH_SPI_SEQ8,
    nand_flash_spi_seq9: NAND_FLASH_SPI_SEQ9,
    nand_flash_spi_seq10: NAND_FLASH_SPI_SEQ10,
    nand_flash_spi_seq11: NAND_FLASH_SPI_SEQ11,
    nand_flash_spi_seq12: NAND_FLASH_SPI_SEQ12,
    nand_flash_spi_seq13: NAND_FLASH_SPI_SEQ13,
    nand_flash_spi_seq14: NAND_FLASH_SPI_SEQ14,
    nand_flash_spi_seq15: NAND_FLASH_SPI_SEQ15,
    nand_flash_spi_seq16: NAND_FLASH_SPI_SEQ16,
    nand_flash_spi_seq17: NAND_FLASH_SPI_SEQ17,
    nand_flash_spi_seq18: NAND_FLASH_SPI_SEQ18,
    nand_flash_spi_seq19: NAND_FLASH_SPI_SEQ19,
    nand_flash_spi_seq20: NAND_FLASH_SPI_SEQ20,
    nand_flash_spi_seq21: NAND_FLASH_SPI_SEQ21,
    nand_flash_spi_seq22: NAND_FLASH_SPI_SEQ22,
    nand_flash_spi_seq23: NAND_FLASH_SPI_SEQ23,
    nand_flash_spi_seq24: NAND_FLASH_SPI_SEQ24,
    nand_flash_spi_seq25: NAND_FLASH_SPI_SEQ25,
    nand_flash_spi_seq26: NAND_FLASH_SPI_SEQ26,
    nand_flash_spi_seq27: NAND_FLASH_SPI_SEQ27,
    nand_flash_spi_seq28: NAND_FLASH_SPI_SEQ28,
    nand_flash_spi_seq29: NAND_FLASH_SPI_SEQ29,
    nand_flash_spi_seq30: NAND_FLASH_SPI_SEQ30,
    nand_flash_spi_seq31: NAND_FLASH_SPI_SEQ31,
    xts_plain_base: XTS_PLAIN_BASE,
    _reserved109: [u8; 0x3c],
    xts_linesize: XTS_LINESIZE,
    xts_destination: XTS_DESTINATION,
    xts_physical_address: XTS_PHYSICAL_ADDRESS,
    xts_trigger: XTS_TRIGGER,
    xts_release: XTS_RELEASE,
    xts_destroy: XTS_DESTROY,
    xts_state: XTS_STATE,
    xts_date: XTS_DATE,
    _reserved117: [u8; 0x1c],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    dpa_ctrl: DPA_CTRL,
    xts_pseudo_round_conf: XTS_PSEUDO_ROUND_CONF,
    _reserved122: [u8; 0x60],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved124: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - SPI0 USR_CMD address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
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
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    #[doc = "0x100 - MSPI flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_attr(&self) -> &SPI_FMEM_PMS0_ATTR {
        &self.spi_fmem_pms0_attr
    }
    #[doc = "0x104 - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_attr(&self) -> &SPI_FMEM_PMS1_ATTR {
        &self.spi_fmem_pms1_attr
    }
    #[doc = "0x108 - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_attr(&self) -> &SPI_FMEM_PMS2_ATTR {
        &self.spi_fmem_pms2_attr
    }
    #[doc = "0x10c - SPI1 flash PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_attr(&self) -> &SPI_FMEM_PMS3_ATTR {
        &self.spi_fmem_pms3_attr
    }
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_addr(&self, n: usize) -> &SPI_FMEM_PMS_ADDR {
        &self.spi_fmem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ADDR> {
        self.spi_fmem_pms_addr.iter()
    }
    #[doc = "0x110 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(0)
    }
    #[doc = "0x114 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(1)
    }
    #[doc = "0x118 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(2)
    }
    #[doc = "0x11c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_addr(&self) -> &SPI_FMEM_PMS_ADDR {
        self.spi_fmem_pms_addr(3)
    }
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_size(&self, n: usize) -> &SPI_FMEM_PMS_SIZE {
        &self.spi_fmem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_fmem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_SIZE> {
        self.spi_fmem_pms_size.iter()
    }
    #[doc = "0x120 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(0)
    }
    #[doc = "0x124 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(1)
    }
    #[doc = "0x128 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(2)
    }
    #[doc = "0x12c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_size(&self) -> &SPI_FMEM_PMS_SIZE {
        self.spi_fmem_pms_size(3)
    }
    #[doc = "0x130 - SPI1 flash PMS section $n start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_attr(&self) -> &SPI_SMEM_PMS0_ATTR {
        &self.spi_smem_pms0_attr
    }
    #[doc = "0x134 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_attr(&self) -> &SPI_SMEM_PMS1_ATTR {
        &self.spi_smem_pms1_attr
    }
    #[doc = "0x138 - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_attr(&self) -> &SPI_SMEM_PMS2_ATTR {
        &self.spi_smem_pms2_attr
    }
    #[doc = "0x13c - SPI1 external RAM PMS section $n attribute register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_attr(&self) -> &SPI_SMEM_PMS3_ATTR {
        &self.spi_smem_pms3_attr
    }
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_addr(&self, n: usize) -> &SPI_SMEM_PMS_ADDR {
        &self.spi_smem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_addr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ADDR> {
        self.spi_smem_pms_addr.iter()
    }
    #[doc = "0x140 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(0)
    }
    #[doc = "0x144 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(1)
    }
    #[doc = "0x148 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(2)
    }
    #[doc = "0x14c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_addr(&self) -> &SPI_SMEM_PMS_ADDR {
        self.spi_smem_pms_addr(3)
    }
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_size(&self, n: usize) -> &SPI_SMEM_PMS_SIZE {
        &self.spi_smem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - SPI1 external RAM PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_size_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_SIZE> {
        self.spi_smem_pms_size.iter()
    }
    #[doc = "0x150 - SPI1 external RAM PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(0)
    }
    #[doc = "0x154 - SPI1 external RAM PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(1)
    }
    #[doc = "0x158 - SPI1 external RAM PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(2)
    }
    #[doc = "0x15c - SPI1 external RAM PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_size(&self) -> &SPI_SMEM_PMS_SIZE {
        self.spi_smem_pms_size(3)
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
    pub const fn spi_smem_ecc_ctrl(&self) -> &SPI_SMEM_ECC_CTRL {
        &self.spi_smem_ecc_ctrl
    }
    #[doc = "0x178 - SPI0 AXI address control register"]
    #[inline(always)]
    pub const fn spi_smem_axi_addr_ctrl(&self) -> &SPI_SMEM_AXI_ADDR_CTRL {
        &self.spi_smem_axi_addr_ctrl
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
    #[doc = "0x244 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut1(&self) -> &NAND_FLASH_CMD_LUT1 {
        &self.nand_flash_cmd_lut1
    }
    #[doc = "0x248 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut2(&self) -> &NAND_FLASH_CMD_LUT2 {
        &self.nand_flash_cmd_lut2
    }
    #[doc = "0x24c - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut3(&self) -> &NAND_FLASH_CMD_LUT3 {
        &self.nand_flash_cmd_lut3
    }
    #[doc = "0x250 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut4(&self) -> &NAND_FLASH_CMD_LUT4 {
        &self.nand_flash_cmd_lut4
    }
    #[doc = "0x254 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut5(&self) -> &NAND_FLASH_CMD_LUT5 {
        &self.nand_flash_cmd_lut5
    }
    #[doc = "0x258 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut6(&self) -> &NAND_FLASH_CMD_LUT6 {
        &self.nand_flash_cmd_lut6
    }
    #[doc = "0x25c - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut7(&self) -> &NAND_FLASH_CMD_LUT7 {
        &self.nand_flash_cmd_lut7
    }
    #[doc = "0x260 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut8(&self) -> &NAND_FLASH_CMD_LUT8 {
        &self.nand_flash_cmd_lut8
    }
    #[doc = "0x264 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut9(&self) -> &NAND_FLASH_CMD_LUT9 {
        &self.nand_flash_cmd_lut9
    }
    #[doc = "0x268 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut10(&self) -> &NAND_FLASH_CMD_LUT10 {
        &self.nand_flash_cmd_lut10
    }
    #[doc = "0x26c - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut11(&self) -> &NAND_FLASH_CMD_LUT11 {
        &self.nand_flash_cmd_lut11
    }
    #[doc = "0x270 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut12(&self) -> &NAND_FLASH_CMD_LUT12 {
        &self.nand_flash_cmd_lut12
    }
    #[doc = "0x274 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut13(&self) -> &NAND_FLASH_CMD_LUT13 {
        &self.nand_flash_cmd_lut13
    }
    #[doc = "0x278 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut14(&self) -> &NAND_FLASH_CMD_LUT14 {
        &self.nand_flash_cmd_lut14
    }
    #[doc = "0x27c - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut15(&self) -> &NAND_FLASH_CMD_LUT15 {
        &self.nand_flash_cmd_lut15
    }
    #[doc = "0x280 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq0(&self) -> &NAND_FLASH_SPI_SEQ0 {
        &self.nand_flash_spi_seq0
    }
    #[doc = "0x284 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq1(&self) -> &NAND_FLASH_SPI_SEQ1 {
        &self.nand_flash_spi_seq1
    }
    #[doc = "0x288 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq2(&self) -> &NAND_FLASH_SPI_SEQ2 {
        &self.nand_flash_spi_seq2
    }
    #[doc = "0x28c - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq3(&self) -> &NAND_FLASH_SPI_SEQ3 {
        &self.nand_flash_spi_seq3
    }
    #[doc = "0x290 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq4(&self) -> &NAND_FLASH_SPI_SEQ4 {
        &self.nand_flash_spi_seq4
    }
    #[doc = "0x294 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq5(&self) -> &NAND_FLASH_SPI_SEQ5 {
        &self.nand_flash_spi_seq5
    }
    #[doc = "0x298 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq6(&self) -> &NAND_FLASH_SPI_SEQ6 {
        &self.nand_flash_spi_seq6
    }
    #[doc = "0x29c - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq7(&self) -> &NAND_FLASH_SPI_SEQ7 {
        &self.nand_flash_spi_seq7
    }
    #[doc = "0x2a0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq8(&self) -> &NAND_FLASH_SPI_SEQ8 {
        &self.nand_flash_spi_seq8
    }
    #[doc = "0x2a4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq9(&self) -> &NAND_FLASH_SPI_SEQ9 {
        &self.nand_flash_spi_seq9
    }
    #[doc = "0x2a8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq10(&self) -> &NAND_FLASH_SPI_SEQ10 {
        &self.nand_flash_spi_seq10
    }
    #[doc = "0x2ac - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq11(&self) -> &NAND_FLASH_SPI_SEQ11 {
        &self.nand_flash_spi_seq11
    }
    #[doc = "0x2b0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq12(&self) -> &NAND_FLASH_SPI_SEQ12 {
        &self.nand_flash_spi_seq12
    }
    #[doc = "0x2b4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq13(&self) -> &NAND_FLASH_SPI_SEQ13 {
        &self.nand_flash_spi_seq13
    }
    #[doc = "0x2b8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq14(&self) -> &NAND_FLASH_SPI_SEQ14 {
        &self.nand_flash_spi_seq14
    }
    #[doc = "0x2bc - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq15(&self) -> &NAND_FLASH_SPI_SEQ15 {
        &self.nand_flash_spi_seq15
    }
    #[doc = "0x2c0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq16(&self) -> &NAND_FLASH_SPI_SEQ16 {
        &self.nand_flash_spi_seq16
    }
    #[doc = "0x2c4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq17(&self) -> &NAND_FLASH_SPI_SEQ17 {
        &self.nand_flash_spi_seq17
    }
    #[doc = "0x2c8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq18(&self) -> &NAND_FLASH_SPI_SEQ18 {
        &self.nand_flash_spi_seq18
    }
    #[doc = "0x2cc - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq19(&self) -> &NAND_FLASH_SPI_SEQ19 {
        &self.nand_flash_spi_seq19
    }
    #[doc = "0x2d0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq20(&self) -> &NAND_FLASH_SPI_SEQ20 {
        &self.nand_flash_spi_seq20
    }
    #[doc = "0x2d4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq21(&self) -> &NAND_FLASH_SPI_SEQ21 {
        &self.nand_flash_spi_seq21
    }
    #[doc = "0x2d8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq22(&self) -> &NAND_FLASH_SPI_SEQ22 {
        &self.nand_flash_spi_seq22
    }
    #[doc = "0x2dc - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq23(&self) -> &NAND_FLASH_SPI_SEQ23 {
        &self.nand_flash_spi_seq23
    }
    #[doc = "0x2e0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq24(&self) -> &NAND_FLASH_SPI_SEQ24 {
        &self.nand_flash_spi_seq24
    }
    #[doc = "0x2e4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq25(&self) -> &NAND_FLASH_SPI_SEQ25 {
        &self.nand_flash_spi_seq25
    }
    #[doc = "0x2e8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq26(&self) -> &NAND_FLASH_SPI_SEQ26 {
        &self.nand_flash_spi_seq26
    }
    #[doc = "0x2ec - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq27(&self) -> &NAND_FLASH_SPI_SEQ27 {
        &self.nand_flash_spi_seq27
    }
    #[doc = "0x2f0 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq28(&self) -> &NAND_FLASH_SPI_SEQ28 {
        &self.nand_flash_spi_seq28
    }
    #[doc = "0x2f4 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq29(&self) -> &NAND_FLASH_SPI_SEQ29 {
        &self.nand_flash_spi_seq29
    }
    #[doc = "0x2f8 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq30(&self) -> &NAND_FLASH_SPI_SEQ30 {
        &self.nand_flash_spi_seq30
    }
    #[doc = "0x2fc - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn nand_flash_spi_seq31(&self) -> &NAND_FLASH_SPI_SEQ31 {
        &self.nand_flash_spi_seq31
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
#[doc = "ADDR (r) register accessor: SPI0 USR_CMD address register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI0 USR_CMD address register"]
pub mod addr;
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
#[doc = "CACHE_SCTRL (r) register accessor: SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (r) register accessor: SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (r) register accessor: SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (r) register accessor: SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
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
#[doc = "INT_CLR (rw) register accessor: SPI0 interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
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
#[doc = "DDR (r) register accessor: SPI0 flash DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod ddr;
#[doc = "SPI_SMEM_DDR (r) register accessor: SPI0 external RAM DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ddr`] module"]
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod spi_smem_ddr;
#[doc = "SPI_FMEM_PMS0_ATTR (rw) register accessor: MSPI flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms0_attr`] module"]
pub type SPI_FMEM_PMS0_ATTR = crate::Reg<spi_fmem_pms0_attr::SPI_FMEM_PMS0_ATTR_SPEC>;
#[doc = "MSPI flash PMS section $n attribute register"]
pub mod spi_fmem_pms0_attr;
#[doc = "SPI_FMEM_PMS1_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms1_attr`] module"]
pub type SPI_FMEM_PMS1_ATTR = crate::Reg<spi_fmem_pms1_attr::SPI_FMEM_PMS1_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod spi_fmem_pms1_attr;
#[doc = "SPI_FMEM_PMS2_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms2_attr`] module"]
pub type SPI_FMEM_PMS2_ATTR = crate::Reg<spi_fmem_pms2_attr::SPI_FMEM_PMS2_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod spi_fmem_pms2_attr;
#[doc = "SPI_FMEM_PMS3_ATTR (rw) register accessor: SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms3_attr`] module"]
pub type SPI_FMEM_PMS3_ATTR = crate::Reg<spi_fmem_pms3_attr::SPI_FMEM_PMS3_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n attribute register"]
pub mod spi_fmem_pms3_attr;
#[doc = "SPI_FMEM_PMS_ADDR (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_addr`] module"]
pub type SPI_FMEM_PMS_ADDR = crate::Reg<spi_fmem_pms_addr::SPI_FMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_addr;
#[doc = "SPI_FMEM_PMS_SIZE (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_size`] module"]
pub type SPI_FMEM_PMS_SIZE = crate::Reg<spi_fmem_pms_size::SPI_FMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_size;
#[doc = "SPI_SMEM_PMS0_ATTR (rw) register accessor: SPI1 flash PMS section $n start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms0_attr`] module"]
pub type SPI_SMEM_PMS0_ATTR = crate::Reg<spi_smem_pms0_attr::SPI_SMEM_PMS0_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section $n start address register"]
pub mod spi_smem_pms0_attr;
#[doc = "SPI_SMEM_PMS1_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms1_attr`] module"]
pub type SPI_SMEM_PMS1_ATTR = crate::Reg<spi_smem_pms1_attr::SPI_SMEM_PMS1_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms1_attr;
#[doc = "SPI_SMEM_PMS2_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms2_attr`] module"]
pub type SPI_SMEM_PMS2_ATTR = crate::Reg<spi_smem_pms2_attr::SPI_SMEM_PMS2_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms2_attr;
#[doc = "SPI_SMEM_PMS3_ATTR (rw) register accessor: SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms3_attr`] module"]
pub type SPI_SMEM_PMS3_ATTR = crate::Reg<spi_smem_pms3_attr::SPI_SMEM_PMS3_ATTR_SPEC>;
#[doc = "SPI1 external RAM PMS section $n attribute register"]
pub mod spi_smem_pms3_attr;
#[doc = "SPI_SMEM_PMS_ADDR (r) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_addr`] module"]
pub type SPI_SMEM_PMS_ADDR = crate::Reg<spi_smem_pms_addr::SPI_SMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_addr;
#[doc = "SPI_SMEM_PMS_SIZE (r) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_size`] module"]
pub type SPI_SMEM_PMS_SIZE = crate::Reg<spi_smem_pms_size::SPI_SMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_size;
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
#[doc = "SPI_SMEM_ECC_CTRL (r) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ecc_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ecc_ctrl`] module"]
pub type SPI_SMEM_ECC_CTRL = crate::Reg<spi_smem_ecc_ctrl::SPI_SMEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod spi_smem_ecc_ctrl;
#[doc = "SPI_SMEM_AXI_ADDR_CTRL (r) register accessor: SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_axi_addr_ctrl`] module"]
pub type SPI_SMEM_AXI_ADDR_CTRL = crate::Reg<spi_smem_axi_addr_ctrl::SPI_SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "SPI0 AXI address control register"]
pub mod spi_smem_axi_addr_ctrl;
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
#[doc = "SPI_SMEM_TIMING_CALI (r) register accessor: MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_timing_cali::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_timing_cali`] module"]
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "MSPI external RAM timing calibration register"]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (r) register accessor: MSPI external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_mode`] module"]
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "MSPI external RAM input timing delay mode control register"]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (r) register accessor: MSPI external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_num`] module"]
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
#[doc = "MSPI external RAM input timing delay number control register"]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (r) register accessor: MSPI external RAM output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_dout_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_mode`] module"]
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
#[doc = "MSPI external RAM output timing adjustment control register"]
pub mod spi_smem_dout_mode;
#[doc = "SPI_SMEM_AC (r) register accessor: MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_ac::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ac`] module"]
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod spi_smem_ac;
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
#[doc = "NAND_FLASH_CMD_LUT1 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut1`] module"]
pub type NAND_FLASH_CMD_LUT1 = crate::Reg<nand_flash_cmd_lut1::NAND_FLASH_CMD_LUT1_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut1;
#[doc = "NAND_FLASH_CMD_LUT2 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut2`] module"]
pub type NAND_FLASH_CMD_LUT2 = crate::Reg<nand_flash_cmd_lut2::NAND_FLASH_CMD_LUT2_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut2;
#[doc = "NAND_FLASH_CMD_LUT3 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut3`] module"]
pub type NAND_FLASH_CMD_LUT3 = crate::Reg<nand_flash_cmd_lut3::NAND_FLASH_CMD_LUT3_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut3;
#[doc = "NAND_FLASH_CMD_LUT4 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut4`] module"]
pub type NAND_FLASH_CMD_LUT4 = crate::Reg<nand_flash_cmd_lut4::NAND_FLASH_CMD_LUT4_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut4;
#[doc = "NAND_FLASH_CMD_LUT5 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut5`] module"]
pub type NAND_FLASH_CMD_LUT5 = crate::Reg<nand_flash_cmd_lut5::NAND_FLASH_CMD_LUT5_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut5;
#[doc = "NAND_FLASH_CMD_LUT6 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut6`] module"]
pub type NAND_FLASH_CMD_LUT6 = crate::Reg<nand_flash_cmd_lut6::NAND_FLASH_CMD_LUT6_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut6;
#[doc = "NAND_FLASH_CMD_LUT7 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut7`] module"]
pub type NAND_FLASH_CMD_LUT7 = crate::Reg<nand_flash_cmd_lut7::NAND_FLASH_CMD_LUT7_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut7;
#[doc = "NAND_FLASH_CMD_LUT8 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut8`] module"]
pub type NAND_FLASH_CMD_LUT8 = crate::Reg<nand_flash_cmd_lut8::NAND_FLASH_CMD_LUT8_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut8;
#[doc = "NAND_FLASH_CMD_LUT9 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut9`] module"]
pub type NAND_FLASH_CMD_LUT9 = crate::Reg<nand_flash_cmd_lut9::NAND_FLASH_CMD_LUT9_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut9;
#[doc = "NAND_FLASH_CMD_LUT10 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut10`] module"]
pub type NAND_FLASH_CMD_LUT10 = crate::Reg<nand_flash_cmd_lut10::NAND_FLASH_CMD_LUT10_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut10;
#[doc = "NAND_FLASH_CMD_LUT11 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut11`] module"]
pub type NAND_FLASH_CMD_LUT11 = crate::Reg<nand_flash_cmd_lut11::NAND_FLASH_CMD_LUT11_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut11;
#[doc = "NAND_FLASH_CMD_LUT12 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut12`] module"]
pub type NAND_FLASH_CMD_LUT12 = crate::Reg<nand_flash_cmd_lut12::NAND_FLASH_CMD_LUT12_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut12;
#[doc = "NAND_FLASH_CMD_LUT13 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut13`] module"]
pub type NAND_FLASH_CMD_LUT13 = crate::Reg<nand_flash_cmd_lut13::NAND_FLASH_CMD_LUT13_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut13;
#[doc = "NAND_FLASH_CMD_LUT14 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut14`] module"]
pub type NAND_FLASH_CMD_LUT14 = crate::Reg<nand_flash_cmd_lut14::NAND_FLASH_CMD_LUT14_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut14;
#[doc = "NAND_FLASH_CMD_LUT15 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut15`] module"]
pub type NAND_FLASH_CMD_LUT15 = crate::Reg<nand_flash_cmd_lut15::NAND_FLASH_CMD_LUT15_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod nand_flash_cmd_lut15;
#[doc = "NAND_FLASH_SPI_SEQ0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq0`] module"]
pub type NAND_FLASH_SPI_SEQ0 = crate::Reg<nand_flash_spi_seq0::NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq0;
#[doc = "NAND_FLASH_SPI_SEQ1 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq1`] module"]
pub type NAND_FLASH_SPI_SEQ1 = crate::Reg<nand_flash_spi_seq1::NAND_FLASH_SPI_SEQ1_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq1;
#[doc = "NAND_FLASH_SPI_SEQ2 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq2`] module"]
pub type NAND_FLASH_SPI_SEQ2 = crate::Reg<nand_flash_spi_seq2::NAND_FLASH_SPI_SEQ2_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq2;
#[doc = "NAND_FLASH_SPI_SEQ3 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq3`] module"]
pub type NAND_FLASH_SPI_SEQ3 = crate::Reg<nand_flash_spi_seq3::NAND_FLASH_SPI_SEQ3_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq3;
#[doc = "NAND_FLASH_SPI_SEQ4 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq4`] module"]
pub type NAND_FLASH_SPI_SEQ4 = crate::Reg<nand_flash_spi_seq4::NAND_FLASH_SPI_SEQ4_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq4;
#[doc = "NAND_FLASH_SPI_SEQ5 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq5`] module"]
pub type NAND_FLASH_SPI_SEQ5 = crate::Reg<nand_flash_spi_seq5::NAND_FLASH_SPI_SEQ5_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq5;
#[doc = "NAND_FLASH_SPI_SEQ6 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq6`] module"]
pub type NAND_FLASH_SPI_SEQ6 = crate::Reg<nand_flash_spi_seq6::NAND_FLASH_SPI_SEQ6_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq6;
#[doc = "NAND_FLASH_SPI_SEQ7 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq7`] module"]
pub type NAND_FLASH_SPI_SEQ7 = crate::Reg<nand_flash_spi_seq7::NAND_FLASH_SPI_SEQ7_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq7;
#[doc = "NAND_FLASH_SPI_SEQ8 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq8`] module"]
pub type NAND_FLASH_SPI_SEQ8 = crate::Reg<nand_flash_spi_seq8::NAND_FLASH_SPI_SEQ8_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq8;
#[doc = "NAND_FLASH_SPI_SEQ9 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq9`] module"]
pub type NAND_FLASH_SPI_SEQ9 = crate::Reg<nand_flash_spi_seq9::NAND_FLASH_SPI_SEQ9_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq9;
#[doc = "NAND_FLASH_SPI_SEQ10 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq10`] module"]
pub type NAND_FLASH_SPI_SEQ10 = crate::Reg<nand_flash_spi_seq10::NAND_FLASH_SPI_SEQ10_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq10;
#[doc = "NAND_FLASH_SPI_SEQ11 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq11`] module"]
pub type NAND_FLASH_SPI_SEQ11 = crate::Reg<nand_flash_spi_seq11::NAND_FLASH_SPI_SEQ11_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq11;
#[doc = "NAND_FLASH_SPI_SEQ12 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq12`] module"]
pub type NAND_FLASH_SPI_SEQ12 = crate::Reg<nand_flash_spi_seq12::NAND_FLASH_SPI_SEQ12_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq12;
#[doc = "NAND_FLASH_SPI_SEQ13 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq13`] module"]
pub type NAND_FLASH_SPI_SEQ13 = crate::Reg<nand_flash_spi_seq13::NAND_FLASH_SPI_SEQ13_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq13;
#[doc = "NAND_FLASH_SPI_SEQ14 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq14`] module"]
pub type NAND_FLASH_SPI_SEQ14 = crate::Reg<nand_flash_spi_seq14::NAND_FLASH_SPI_SEQ14_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq14;
#[doc = "NAND_FLASH_SPI_SEQ15 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq15`] module"]
pub type NAND_FLASH_SPI_SEQ15 = crate::Reg<nand_flash_spi_seq15::NAND_FLASH_SPI_SEQ15_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq15;
#[doc = "NAND_FLASH_SPI_SEQ16 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq16`] module"]
pub type NAND_FLASH_SPI_SEQ16 = crate::Reg<nand_flash_spi_seq16::NAND_FLASH_SPI_SEQ16_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq16;
#[doc = "NAND_FLASH_SPI_SEQ17 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq17`] module"]
pub type NAND_FLASH_SPI_SEQ17 = crate::Reg<nand_flash_spi_seq17::NAND_FLASH_SPI_SEQ17_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq17;
#[doc = "NAND_FLASH_SPI_SEQ18 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq18`] module"]
pub type NAND_FLASH_SPI_SEQ18 = crate::Reg<nand_flash_spi_seq18::NAND_FLASH_SPI_SEQ18_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq18;
#[doc = "NAND_FLASH_SPI_SEQ19 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq19`] module"]
pub type NAND_FLASH_SPI_SEQ19 = crate::Reg<nand_flash_spi_seq19::NAND_FLASH_SPI_SEQ19_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq19;
#[doc = "NAND_FLASH_SPI_SEQ20 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq20`] module"]
pub type NAND_FLASH_SPI_SEQ20 = crate::Reg<nand_flash_spi_seq20::NAND_FLASH_SPI_SEQ20_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq20;
#[doc = "NAND_FLASH_SPI_SEQ21 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq21`] module"]
pub type NAND_FLASH_SPI_SEQ21 = crate::Reg<nand_flash_spi_seq21::NAND_FLASH_SPI_SEQ21_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq21;
#[doc = "NAND_FLASH_SPI_SEQ22 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq22`] module"]
pub type NAND_FLASH_SPI_SEQ22 = crate::Reg<nand_flash_spi_seq22::NAND_FLASH_SPI_SEQ22_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq22;
#[doc = "NAND_FLASH_SPI_SEQ23 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq23`] module"]
pub type NAND_FLASH_SPI_SEQ23 = crate::Reg<nand_flash_spi_seq23::NAND_FLASH_SPI_SEQ23_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq23;
#[doc = "NAND_FLASH_SPI_SEQ24 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq24`] module"]
pub type NAND_FLASH_SPI_SEQ24 = crate::Reg<nand_flash_spi_seq24::NAND_FLASH_SPI_SEQ24_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq24;
#[doc = "NAND_FLASH_SPI_SEQ25 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq25`] module"]
pub type NAND_FLASH_SPI_SEQ25 = crate::Reg<nand_flash_spi_seq25::NAND_FLASH_SPI_SEQ25_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq25;
#[doc = "NAND_FLASH_SPI_SEQ26 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq26`] module"]
pub type NAND_FLASH_SPI_SEQ26 = crate::Reg<nand_flash_spi_seq26::NAND_FLASH_SPI_SEQ26_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq26;
#[doc = "NAND_FLASH_SPI_SEQ27 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq27`] module"]
pub type NAND_FLASH_SPI_SEQ27 = crate::Reg<nand_flash_spi_seq27::NAND_FLASH_SPI_SEQ27_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq27;
#[doc = "NAND_FLASH_SPI_SEQ28 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq28`] module"]
pub type NAND_FLASH_SPI_SEQ28 = crate::Reg<nand_flash_spi_seq28::NAND_FLASH_SPI_SEQ28_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq28;
#[doc = "NAND_FLASH_SPI_SEQ29 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq29`] module"]
pub type NAND_FLASH_SPI_SEQ29 = crate::Reg<nand_flash_spi_seq29::NAND_FLASH_SPI_SEQ29_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq29;
#[doc = "NAND_FLASH_SPI_SEQ30 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq30`] module"]
pub type NAND_FLASH_SPI_SEQ30 = crate::Reg<nand_flash_spi_seq30::NAND_FLASH_SPI_SEQ30_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq30;
#[doc = "NAND_FLASH_SPI_SEQ31 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq31`] module"]
pub type NAND_FLASH_SPI_SEQ31 = crate::Reg<nand_flash_spi_seq31::NAND_FLASH_SPI_SEQ31_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod nand_flash_spi_seq31;
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
pub use crate::dma::{date as xts_date, DATE as XTS_DATE};
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
pub use crate::dma::{date, DATE};
