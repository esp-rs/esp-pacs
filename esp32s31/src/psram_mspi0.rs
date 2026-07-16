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
    spi_smem_s_ddr: SPI_SMEM_S_DDR,
    dll_dly_db: DLL_DLY_DB,
    dll_db_st0: DLL_DB_ST0,
    dll_db_st1: DLL_DB_ST1,
    _reserved26: [u8; 0x18],
    spi_fmem_s_pms0_attr: SPI_FMEM_S_PMS0_ATTR,
    spi_fmem_s_pms1_attr: SPI_FMEM_S_PMS1_ATTR,
    spi_fmem_s_pms2_attr: SPI_FMEM_S_PMS2_ATTR,
    spi_fmem_s_pms3_attr: SPI_FMEM_S_PMS3_ATTR,
    spi_fmem_s_pms0_addr: SPI_FMEM_S_PMS0_ADDR,
    spi_fmem_s_pms1_addr: SPI_FMEM_S_PMS1_ADDR,
    spi_fmem_s_pms2_addr: SPI_FMEM_S_PMS2_ADDR,
    spi_fmem_s_pms3_addr: SPI_FMEM_S_PMS3_ADDR,
    spi_fmem_s_pms0_size: SPI_FMEM_S_PMS0_SIZE,
    spi_fmem_s_pms1_size: SPI_FMEM_S_PMS1_SIZE,
    spi_fmem_s_pms2_size: SPI_FMEM_S_PMS2_SIZE,
    spi_fmem_s_pms3_size: SPI_FMEM_S_PMS3_SIZE,
    spi_smem_s_pms0_attr: SPI_SMEM_S_PMS0_ATTR,
    spi_smem_s_pms1_attr: SPI_SMEM_S_PMS1_ATTR,
    spi_smem_s_pms2_attr: SPI_SMEM_S_PMS2_ATTR,
    spi_smem_s_pms3_attr: SPI_SMEM_S_PMS3_ATTR,
    spi_smem_s_pms0_addr: SPI_SMEM_S_PMS0_ADDR,
    spi_smem_s_pms1_addr: SPI_SMEM_S_PMS1_ADDR,
    spi_smem_s_pms2_addr: SPI_SMEM_S_PMS2_ADDR,
    spi_smem_s_pms3_addr: SPI_SMEM_S_PMS3_ADDR,
    spi_smem_s_pms0_size: SPI_SMEM_S_PMS0_SIZE,
    spi_smem_s_pms1_size: SPI_SMEM_S_PMS1_SIZE,
    spi_smem_s_pms2_size: SPI_SMEM_S_PMS2_SIZE,
    spi_smem_s_pms3_size: SPI_SMEM_S_PMS3_SIZE,
    pms_reject: PMS_REJECT,
    pms_reject_addr: PMS_REJECT_ADDR,
    ecc_ctrl: ECC_CTRL,
    ecc_err_addr: ECC_ERR_ADDR,
    axi_err_addr: AXI_ERR_ADDR,
    spi_smem_s_ecc_ctrl: SPI_SMEM_S_ECC_CTRL,
    spi_smem_s_axi_addr_ctrl: SPI_SMEM_S_AXI_ADDR_CTRL,
    axi_err_resp_en: AXI_ERR_RESP_EN,
    timing_cali: TIMING_CALI,
    din_mode: DIN_MODE,
    din_num: DIN_NUM,
    dout_mode: DOUT_MODE,
    spi_smem_s_timing_cali: SPI_SMEM_S_TIMING_CALI,
    spi_smem_s_din_mode: SPI_SMEM_S_DIN_MODE,
    spi_smem_s_din_num: SPI_SMEM_S_DIN_NUM,
    spi_smem_s_dout_mode: SPI_SMEM_S_DOUT_MODE,
    spi_smem_s_ac: SPI_SMEM_S_AC,
    spi_smem_s_din_hex_mode: SPI_SMEM_S_DIN_HEX_MODE,
    spi_smem_s_din_hex_num: SPI_SMEM_S_DIN_HEX_NUM,
    spi_smem_s_dout_hex_mode: SPI_SMEM_S_DOUT_HEX_MODE,
    _reserved70: [u8; 0x50],
    clock_gate: CLOCK_GATE,
    nand_flash_en: NAND_FLASH_EN,
    nand_flash_sr_addr0: NAND_FLASH_SR_ADDR0,
    nand_flash_sr_din0: NAND_FLASH_SR_DIN0,
    nand_flash_cfg_data: [NAND_FLASH_CFG_DATA; 3],
    _reserved75: [u8; 0x24],
    nand_flash_cmd_lut0: NAND_FLASH_CMD_LUT0,
    _reserved76: [u8; 0x3c],
    nand_flash_spi_seq0: NAND_FLASH_SPI_SEQ0,
    _reserved77: [u8; 0x7c],
    xts_plain_base: XTS_PLAIN_BASE,
    _reserved78: [u8; 0x3c],
    xts_linesize: XTS_LINESIZE,
    xts_destination: XTS_DESTINATION,
    xts_physical_address: XTS_PHYSICAL_ADDRESS,
    xts_trigger: XTS_TRIGGER,
    xts_release: XTS_RELEASE,
    xts_destroy: XTS_DESTROY,
    xts_state: XTS_STATE,
    xts_date: XTS_DATE,
    _reserved86: [u8; 0x1c],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    dpa_ctrl: DPA_CTRL,
    xts_pseudo_round_conf: XTS_PSEUDO_ROUND_CONF,
    _reserved91: [u8; 0x60],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved93: [u8; 0x04],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn cache_sctrl(&self) -> &CACHE_SCTRL {
        &self.cache_sctrl
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn sram_cmd(&self) -> &SRAM_CMD {
        &self.sram_cmd
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn sram_drd_cmd(&self) -> &SRAM_DRD_CMD {
        &self.sram_drd_cmd
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sram_dwr_cmd(&self) -> &SRAM_DWR_CMD {
        &self.sram_dwr_cmd
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn sram_clk(&self) -> &SRAM_CLK {
        &self.sram_clk
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn fsm(&self) -> &FSM {
        &self.fsm
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn spi_smem_s_ddr(&self) -> &SPI_SMEM_S_DDR {
        &self.spi_smem_s_ddr
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn dll_dly_db(&self) -> &DLL_DLY_DB {
        &self.dll_dly_db
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn dll_db_st0(&self) -> &DLL_DB_ST0 {
        &self.dll_db_st0
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn dll_db_st1(&self) -> &DLL_DB_ST1 {
        &self.dll_db_st1
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms0_attr(&self) -> &SPI_FMEM_S_PMS0_ATTR {
        &self.spi_fmem_s_pms0_attr
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms1_attr(&self) -> &SPI_FMEM_S_PMS1_ATTR {
        &self.spi_fmem_s_pms1_attr
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms2_attr(&self) -> &SPI_FMEM_S_PMS2_ATTR {
        &self.spi_fmem_s_pms2_attr
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms3_attr(&self) -> &SPI_FMEM_S_PMS3_ATTR {
        &self.spi_fmem_s_pms3_attr
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms0_addr(&self) -> &SPI_FMEM_S_PMS0_ADDR {
        &self.spi_fmem_s_pms0_addr
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms1_addr(&self) -> &SPI_FMEM_S_PMS1_ADDR {
        &self.spi_fmem_s_pms1_addr
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms2_addr(&self) -> &SPI_FMEM_S_PMS2_ADDR {
        &self.spi_fmem_s_pms2_addr
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms3_addr(&self) -> &SPI_FMEM_S_PMS3_ADDR {
        &self.spi_fmem_s_pms3_addr
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms0_size(&self) -> &SPI_FMEM_S_PMS0_SIZE {
        &self.spi_fmem_s_pms0_size
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms1_size(&self) -> &SPI_FMEM_S_PMS1_SIZE {
        &self.spi_fmem_s_pms1_size
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms2_size(&self) -> &SPI_FMEM_S_PMS2_SIZE {
        &self.spi_fmem_s_pms2_size
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn spi_fmem_s_pms3_size(&self) -> &SPI_FMEM_S_PMS3_SIZE {
        &self.spi_fmem_s_pms3_size
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms0_attr(&self) -> &SPI_SMEM_S_PMS0_ATTR {
        &self.spi_smem_s_pms0_attr
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms1_attr(&self) -> &SPI_SMEM_S_PMS1_ATTR {
        &self.spi_smem_s_pms1_attr
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms2_attr(&self) -> &SPI_SMEM_S_PMS2_ATTR {
        &self.spi_smem_s_pms2_attr
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms3_attr(&self) -> &SPI_SMEM_S_PMS3_ATTR {
        &self.spi_smem_s_pms3_attr
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms0_addr(&self) -> &SPI_SMEM_S_PMS0_ADDR {
        &self.spi_smem_s_pms0_addr
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms1_addr(&self) -> &SPI_SMEM_S_PMS1_ADDR {
        &self.spi_smem_s_pms1_addr
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms2_addr(&self) -> &SPI_SMEM_S_PMS2_ADDR {
        &self.spi_smem_s_pms2_addr
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms3_addr(&self) -> &SPI_SMEM_S_PMS3_ADDR {
        &self.spi_smem_s_pms3_addr
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms0_size(&self) -> &SPI_SMEM_S_PMS0_SIZE {
        &self.spi_smem_s_pms0_size
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms1_size(&self) -> &SPI_SMEM_S_PMS1_SIZE {
        &self.spi_smem_s_pms1_size
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms2_size(&self) -> &SPI_SMEM_S_PMS2_SIZE {
        &self.spi_smem_s_pms2_size
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn spi_smem_s_pms3_size(&self) -> &SPI_SMEM_S_PMS3_SIZE {
        &self.spi_smem_s_pms3_size
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PMS_REJECT {
        &self.pms_reject
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn pms_reject_addr(&self) -> &PMS_REJECT_ADDR {
        &self.pms_reject_addr
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &ECC_CTRL {
        &self.ecc_ctrl
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn ecc_err_addr(&self) -> &ECC_ERR_ADDR {
        &self.ecc_err_addr
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn axi_err_addr(&self) -> &AXI_ERR_ADDR {
        &self.axi_err_addr
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn spi_smem_s_ecc_ctrl(&self) -> &SPI_SMEM_S_ECC_CTRL {
        &self.spi_smem_s_ecc_ctrl
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn spi_smem_s_axi_addr_ctrl(&self) -> &SPI_SMEM_S_AXI_ADDR_CTRL {
        &self.spi_smem_s_axi_addr_ctrl
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn axi_err_resp_en(&self) -> &AXI_ERR_RESP_EN {
        &self.axi_err_resp_en
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn din_mode(&self) -> &DIN_MODE {
        &self.din_mode
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn din_num(&self) -> &DIN_NUM {
        &self.din_num
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn dout_mode(&self) -> &DOUT_MODE {
        &self.dout_mode
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn spi_smem_s_timing_cali(&self) -> &SPI_SMEM_S_TIMING_CALI {
        &self.spi_smem_s_timing_cali
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn spi_smem_s_din_mode(&self) -> &SPI_SMEM_S_DIN_MODE {
        &self.spi_smem_s_din_mode
    }
    #[doc = "0x198 - "]
    #[inline(always)]
    pub const fn spi_smem_s_din_num(&self) -> &SPI_SMEM_S_DIN_NUM {
        &self.spi_smem_s_din_num
    }
    #[doc = "0x19c - "]
    #[inline(always)]
    pub const fn spi_smem_s_dout_mode(&self) -> &SPI_SMEM_S_DOUT_MODE {
        &self.spi_smem_s_dout_mode
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn spi_smem_s_ac(&self) -> &SPI_SMEM_S_AC {
        &self.spi_smem_s_ac
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn spi_smem_s_din_hex_mode(&self) -> &SPI_SMEM_S_DIN_HEX_MODE {
        &self.spi_smem_s_din_hex_mode
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn spi_smem_s_din_hex_num(&self) -> &SPI_SMEM_S_DIN_HEX_NUM {
        &self.spi_smem_s_din_hex_num
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn spi_smem_s_dout_hex_mode(&self) -> &SPI_SMEM_S_DOUT_HEX_MODE {
        &self.spi_smem_s_dout_hex_mode
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn nand_flash_en(&self) -> &NAND_FLASH_EN {
        &self.nand_flash_en
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn nand_flash_sr_addr0(&self) -> &NAND_FLASH_SR_ADDR0 {
        &self.nand_flash_sr_addr0
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn nand_flash_sr_din0(&self) -> &NAND_FLASH_SR_DIN0 {
        &self.nand_flash_sr_din0
    }
    #[doc = "0x210..0x21c - "]
    #[inline(always)]
    pub const fn nand_flash_cfg_data(&self, n: usize) -> &NAND_FLASH_CFG_DATA {
        &self.nand_flash_cfg_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x210..0x21c - "]
    #[inline(always)]
    pub fn nand_flash_cfg_data_iter(&self) -> impl Iterator<Item = &NAND_FLASH_CFG_DATA> {
        self.nand_flash_cfg_data.iter()
    }
    #[doc = "0x240 - "]
    #[inline(always)]
    pub const fn nand_flash_cmd_lut0(&self) -> &NAND_FLASH_CMD_LUT0 {
        &self.nand_flash_cmd_lut0
    }
    #[doc = "0x280 - "]
    #[inline(always)]
    pub const fn nand_flash_spi_seq0(&self) -> &NAND_FLASH_SPI_SEQ0 {
        &self.nand_flash_spi_seq0
    }
    #[doc = "0x300 - "]
    #[inline(always)]
    pub const fn xts_plain_base(&self) -> &XTS_PLAIN_BASE {
        &self.xts_plain_base
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn xts_linesize(&self) -> &XTS_LINESIZE {
        &self.xts_linesize
    }
    #[doc = "0x344 - "]
    #[inline(always)]
    pub const fn xts_destination(&self) -> &XTS_DESTINATION {
        &self.xts_destination
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn xts_physical_address(&self) -> &XTS_PHYSICAL_ADDRESS {
        &self.xts_physical_address
    }
    #[doc = "0x34c - "]
    #[inline(always)]
    pub const fn xts_trigger(&self) -> &XTS_TRIGGER {
        &self.xts_trigger
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn xts_release(&self) -> &XTS_RELEASE {
        &self.xts_release
    }
    #[doc = "0x354 - "]
    #[inline(always)]
    pub const fn xts_destroy(&self) -> &XTS_DESTROY {
        &self.xts_destroy
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn xts_state(&self) -> &XTS_STATE {
        &self.xts_state
    }
    #[doc = "0x35c - "]
    #[inline(always)]
    pub const fn xts_date(&self) -> &XTS_DATE {
        &self.xts_date
    }
    #[doc = "0x37c - "]
    #[inline(always)]
    pub const fn mmu_item_content(&self) -> &MMU_ITEM_CONTENT {
        &self.mmu_item_content
    }
    #[doc = "0x380 - "]
    #[inline(always)]
    pub const fn mmu_item_index(&self) -> &MMU_ITEM_INDEX {
        &self.mmu_item_index
    }
    #[doc = "0x384 - "]
    #[inline(always)]
    pub const fn mmu_power_ctrl(&self) -> &MMU_POWER_CTRL {
        &self.mmu_power_ctrl
    }
    #[doc = "0x388 - "]
    #[inline(always)]
    pub const fn dpa_ctrl(&self) -> &DPA_CTRL {
        &self.dpa_ctrl
    }
    #[doc = "0x38c - "]
    #[inline(always)]
    pub const fn xts_pseudo_round_conf(&self) -> &XTS_PSEUDO_ROUND_CONF {
        &self.xts_pseudo_round_conf
    }
    #[doc = "0x3f0 - "]
    #[inline(always)]
    pub const fn registerrnd_eco_high(&self) -> &REGISTERRND_ECO_HIGH {
        &self.registerrnd_eco_high
    }
    #[doc = "0x3f4 - "]
    #[inline(always)]
    pub const fn registerrnd_eco_low(&self) -> &REGISTERRND_ECO_LOW {
        &self.registerrnd_eco_low
    }
    #[doc = "0x3fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "CTRL1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = ""]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = ""]
pub mod ctrl2;
#[doc = "CLOCK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = ""]
pub mod clock;
#[doc = "USER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`] module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = ""]
pub mod user;
#[doc = "USER1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user1`] module"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = ""]
pub mod user1;
#[doc = "USER2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user2`] module"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = ""]
pub mod user2;
#[doc = "RD_STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_status`] module"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = ""]
pub mod rd_status;
#[doc = "MISC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc`] module"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = ""]
pub mod misc;
#[doc = "CACHE_FCTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_fctrl`] module"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = ""]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cache_sctrl`] module"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = ""]
pub mod cache_sctrl;
#[doc = "SRAM_CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_cmd`] module"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = ""]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_drd_cmd`] module"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = ""]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_dwr_cmd`] module"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = ""]
pub mod sram_dwr_cmd;
#[doc = "SRAM_CLK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_clk`] module"]
pub type SRAM_CLK = crate::Reg<sram_clk::SRAM_CLK_SPEC>;
#[doc = ""]
pub mod sram_clk;
#[doc = "FSM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm`] module"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = ""]
pub mod fsm;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "DDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr`] module"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = ""]
pub mod ddr;
#[doc = "SPI_SMEM_S_DDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_ddr`] module"]
pub type SPI_SMEM_S_DDR = crate::Reg<spi_smem_s_ddr::SPI_SMEM_S_DDR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_ddr;
#[doc = "DLL_DLY_DB (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dll_dly_db::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_dly_db::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_dly_db`] module"]
pub type DLL_DLY_DB = crate::Reg<dll_dly_db::DLL_DLY_DB_SPEC>;
#[doc = ""]
pub mod dll_dly_db;
#[doc = "DLL_DB_ST0 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_db_st0`] module"]
pub type DLL_DB_ST0 = crate::Reg<dll_db_st0::DLL_DB_ST0_SPEC>;
#[doc = ""]
pub mod dll_db_st0;
#[doc = "DLL_DB_ST1 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dll_db_st1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_db_st1`] module"]
pub type DLL_DB_ST1 = crate::Reg<dll_db_st1::DLL_DB_ST1_SPEC>;
#[doc = ""]
pub mod dll_db_st1;
#[doc = "SPI_FMEM_S_PMS0_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms0_attr`] module"]
pub type SPI_FMEM_S_PMS0_ATTR = crate::Reg<spi_fmem_s_pms0_attr::SPI_FMEM_S_PMS0_ATTR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms0_attr;
#[doc = "SPI_FMEM_S_PMS1_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms1_attr`] module"]
pub type SPI_FMEM_S_PMS1_ATTR = crate::Reg<spi_fmem_s_pms1_attr::SPI_FMEM_S_PMS1_ATTR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms1_attr;
#[doc = "SPI_FMEM_S_PMS2_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms2_attr`] module"]
pub type SPI_FMEM_S_PMS2_ATTR = crate::Reg<spi_fmem_s_pms2_attr::SPI_FMEM_S_PMS2_ATTR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms2_attr;
#[doc = "SPI_FMEM_S_PMS3_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms3_attr`] module"]
pub type SPI_FMEM_S_PMS3_ATTR = crate::Reg<spi_fmem_s_pms3_attr::SPI_FMEM_S_PMS3_ATTR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms3_attr;
#[doc = "SPI_FMEM_S_PMS0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms0_addr`] module"]
pub type SPI_FMEM_S_PMS0_ADDR = crate::Reg<spi_fmem_s_pms0_addr::SPI_FMEM_S_PMS0_ADDR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms0_addr;
#[doc = "SPI_FMEM_S_PMS1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms1_addr`] module"]
pub type SPI_FMEM_S_PMS1_ADDR = crate::Reg<spi_fmem_s_pms1_addr::SPI_FMEM_S_PMS1_ADDR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms1_addr;
#[doc = "SPI_FMEM_S_PMS2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms2_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms2_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms2_addr`] module"]
pub type SPI_FMEM_S_PMS2_ADDR = crate::Reg<spi_fmem_s_pms2_addr::SPI_FMEM_S_PMS2_ADDR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms2_addr;
#[doc = "SPI_FMEM_S_PMS3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms3_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms3_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms3_addr`] module"]
pub type SPI_FMEM_S_PMS3_ADDR = crate::Reg<spi_fmem_s_pms3_addr::SPI_FMEM_S_PMS3_ADDR_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms3_addr;
#[doc = "SPI_FMEM_S_PMS0_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms0_size`] module"]
pub type SPI_FMEM_S_PMS0_SIZE = crate::Reg<spi_fmem_s_pms0_size::SPI_FMEM_S_PMS0_SIZE_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms0_size;
#[doc = "SPI_FMEM_S_PMS1_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms1_size`] module"]
pub type SPI_FMEM_S_PMS1_SIZE = crate::Reg<spi_fmem_s_pms1_size::SPI_FMEM_S_PMS1_SIZE_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms1_size;
#[doc = "SPI_FMEM_S_PMS2_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms2_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms2_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms2_size`] module"]
pub type SPI_FMEM_S_PMS2_SIZE = crate::Reg<spi_fmem_s_pms2_size::SPI_FMEM_S_PMS2_SIZE_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms2_size;
#[doc = "SPI_FMEM_S_PMS3_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_s_pms3_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_s_pms3_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_s_pms3_size`] module"]
pub type SPI_FMEM_S_PMS3_SIZE = crate::Reg<spi_fmem_s_pms3_size::SPI_FMEM_S_PMS3_SIZE_SPEC>;
#[doc = ""]
pub mod spi_fmem_s_pms3_size;
#[doc = "SPI_SMEM_S_PMS0_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms0_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms0_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms0_attr`] module"]
pub type SPI_SMEM_S_PMS0_ATTR = crate::Reg<spi_smem_s_pms0_attr::SPI_SMEM_S_PMS0_ATTR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms0_attr;
#[doc = "SPI_SMEM_S_PMS1_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms1_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms1_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms1_attr`] module"]
pub type SPI_SMEM_S_PMS1_ATTR = crate::Reg<spi_smem_s_pms1_attr::SPI_SMEM_S_PMS1_ATTR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms1_attr;
#[doc = "SPI_SMEM_S_PMS2_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms2_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms2_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms2_attr`] module"]
pub type SPI_SMEM_S_PMS2_ATTR = crate::Reg<spi_smem_s_pms2_attr::SPI_SMEM_S_PMS2_ATTR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms2_attr;
#[doc = "SPI_SMEM_S_PMS3_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms3_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms3_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms3_attr`] module"]
pub type SPI_SMEM_S_PMS3_ATTR = crate::Reg<spi_smem_s_pms3_attr::SPI_SMEM_S_PMS3_ATTR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms3_attr;
#[doc = "SPI_SMEM_S_PMS0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms0_addr`] module"]
pub type SPI_SMEM_S_PMS0_ADDR = crate::Reg<spi_smem_s_pms0_addr::SPI_SMEM_S_PMS0_ADDR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms0_addr;
#[doc = "SPI_SMEM_S_PMS1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms1_addr`] module"]
pub type SPI_SMEM_S_PMS1_ADDR = crate::Reg<spi_smem_s_pms1_addr::SPI_SMEM_S_PMS1_ADDR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms1_addr;
#[doc = "SPI_SMEM_S_PMS2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms2_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms2_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms2_addr`] module"]
pub type SPI_SMEM_S_PMS2_ADDR = crate::Reg<spi_smem_s_pms2_addr::SPI_SMEM_S_PMS2_ADDR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms2_addr;
#[doc = "SPI_SMEM_S_PMS3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms3_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms3_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms3_addr`] module"]
pub type SPI_SMEM_S_PMS3_ADDR = crate::Reg<spi_smem_s_pms3_addr::SPI_SMEM_S_PMS3_ADDR_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms3_addr;
#[doc = "SPI_SMEM_S_PMS0_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms0_size`] module"]
pub type SPI_SMEM_S_PMS0_SIZE = crate::Reg<spi_smem_s_pms0_size::SPI_SMEM_S_PMS0_SIZE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms0_size;
#[doc = "SPI_SMEM_S_PMS1_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms1_size`] module"]
pub type SPI_SMEM_S_PMS1_SIZE = crate::Reg<spi_smem_s_pms1_size::SPI_SMEM_S_PMS1_SIZE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms1_size;
#[doc = "SPI_SMEM_S_PMS2_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms2_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms2_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms2_size`] module"]
pub type SPI_SMEM_S_PMS2_SIZE = crate::Reg<spi_smem_s_pms2_size::SPI_SMEM_S_PMS2_SIZE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms2_size;
#[doc = "SPI_SMEM_S_PMS3_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_pms3_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_pms3_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_pms3_size`] module"]
pub type SPI_SMEM_S_PMS3_SIZE = crate::Reg<spi_smem_s_pms3_size::SPI_SMEM_S_PMS3_SIZE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_pms3_size;
#[doc = "PMS_REJECT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject`] module"]
pub type PMS_REJECT = crate::Reg<pms_reject::PMS_REJECT_SPEC>;
#[doc = ""]
pub mod pms_reject;
#[doc = "PMS_REJECT_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject_addr`] module"]
pub type PMS_REJECT_ADDR = crate::Reg<pms_reject_addr::PMS_REJECT_ADDR_SPEC>;
#[doc = ""]
pub mod pms_reject_addr;
#[doc = "ECC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctrl`] module"]
pub type ECC_CTRL = crate::Reg<ecc_ctrl::ECC_CTRL_SPEC>;
#[doc = ""]
pub mod ecc_ctrl;
#[doc = "ECC_ERR_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_err_addr`] module"]
pub type ECC_ERR_ADDR = crate::Reg<ecc_err_addr::ECC_ERR_ADDR_SPEC>;
#[doc = ""]
pub mod ecc_err_addr;
#[doc = "AXI_ERR_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_addr`] module"]
pub type AXI_ERR_ADDR = crate::Reg<axi_err_addr::AXI_ERR_ADDR_SPEC>;
#[doc = ""]
pub mod axi_err_addr;
#[doc = "SPI_SMEM_S_ECC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_ecc_ctrl`] module"]
pub type SPI_SMEM_S_ECC_CTRL = crate::Reg<spi_smem_s_ecc_ctrl::SPI_SMEM_S_ECC_CTRL_SPEC>;
#[doc = ""]
pub mod spi_smem_s_ecc_ctrl;
#[doc = "SPI_SMEM_S_AXI_ADDR_CTRL (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_axi_addr_ctrl`] module"]
pub type SPI_SMEM_S_AXI_ADDR_CTRL =
    crate::Reg<spi_smem_s_axi_addr_ctrl::SPI_SMEM_S_AXI_ADDR_CTRL_SPEC>;
#[doc = ""]
pub mod spi_smem_s_axi_addr_ctrl;
#[doc = "AXI_ERR_RESP_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_resp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_resp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err_resp_en`] module"]
pub type AXI_ERR_RESP_EN = crate::Reg<axi_err_resp_en::AXI_ERR_RESP_EN_SPEC>;
#[doc = ""]
pub mod axi_err_resp_en;
#[doc = "TIMING_CALI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing_cali`] module"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = ""]
pub mod timing_cali;
#[doc = "DIN_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_mode`] module"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = ""]
pub mod din_mode;
#[doc = "DIN_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_num`] module"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = ""]
pub mod din_num;
#[doc = "DOUT_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_mode`] module"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = ""]
pub mod dout_mode;
#[doc = "SPI_SMEM_S_TIMING_CALI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_timing_cali`] module"]
pub type SPI_SMEM_S_TIMING_CALI = crate::Reg<spi_smem_s_timing_cali::SPI_SMEM_S_TIMING_CALI_SPEC>;
#[doc = ""]
pub mod spi_smem_s_timing_cali;
#[doc = "SPI_SMEM_S_DIN_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_din_mode`] module"]
pub type SPI_SMEM_S_DIN_MODE = crate::Reg<spi_smem_s_din_mode::SPI_SMEM_S_DIN_MODE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_din_mode;
#[doc = "SPI_SMEM_S_DIN_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_din_num`] module"]
pub type SPI_SMEM_S_DIN_NUM = crate::Reg<spi_smem_s_din_num::SPI_SMEM_S_DIN_NUM_SPEC>;
#[doc = ""]
pub mod spi_smem_s_din_num;
#[doc = "SPI_SMEM_S_DOUT_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_dout_mode`] module"]
pub type SPI_SMEM_S_DOUT_MODE = crate::Reg<spi_smem_s_dout_mode::SPI_SMEM_S_DOUT_MODE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_dout_mode;
#[doc = "SPI_SMEM_S_AC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_ac`] module"]
pub type SPI_SMEM_S_AC = crate::Reg<spi_smem_s_ac::SPI_SMEM_S_AC_SPEC>;
#[doc = ""]
pub mod spi_smem_s_ac;
#[doc = "SPI_SMEM_S_DIN_HEX_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_din_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_din_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_din_hex_mode`] module"]
pub type SPI_SMEM_S_DIN_HEX_MODE =
    crate::Reg<spi_smem_s_din_hex_mode::SPI_SMEM_S_DIN_HEX_MODE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_din_hex_mode;
#[doc = "SPI_SMEM_S_DIN_HEX_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_din_hex_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_din_hex_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_din_hex_num`] module"]
pub type SPI_SMEM_S_DIN_HEX_NUM = crate::Reg<spi_smem_s_din_hex_num::SPI_SMEM_S_DIN_HEX_NUM_SPEC>;
#[doc = ""]
pub mod spi_smem_s_din_hex_num;
#[doc = "SPI_SMEM_S_DOUT_HEX_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_dout_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_dout_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_s_dout_hex_mode`] module"]
pub type SPI_SMEM_S_DOUT_HEX_MODE =
    crate::Reg<spi_smem_s_dout_hex_mode::SPI_SMEM_S_DOUT_HEX_MODE_SPEC>;
#[doc = ""]
pub mod spi_smem_s_dout_hex_mode;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
#[doc = "NAND_FLASH_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_en`] module"]
pub type NAND_FLASH_EN = crate::Reg<nand_flash_en::NAND_FLASH_EN_SPEC>;
#[doc = ""]
pub mod nand_flash_en;
#[doc = "NAND_FLASH_SR_ADDR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_sr_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_sr_addr0`] module"]
pub type NAND_FLASH_SR_ADDR0 = crate::Reg<nand_flash_sr_addr0::NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = ""]
pub mod nand_flash_sr_addr0;
#[doc = "NAND_FLASH_SR_DIN0 (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_din0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_sr_din0`] module"]
pub type NAND_FLASH_SR_DIN0 = crate::Reg<nand_flash_sr_din0::NAND_FLASH_SR_DIN0_SPEC>;
#[doc = ""]
pub mod nand_flash_sr_din0;
#[doc = "NAND_FLASH_CFG_DATA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cfg_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_cfg_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cfg_data`] module"]
pub type NAND_FLASH_CFG_DATA = crate::Reg<nand_flash_cfg_data::NAND_FLASH_CFG_DATA_SPEC>;
#[doc = ""]
pub mod nand_flash_cfg_data;
#[doc = "NAND_FLASH_CMD_LUT0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_cmd_lut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_cmd_lut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_cmd_lut0`] module"]
pub type NAND_FLASH_CMD_LUT0 = crate::Reg<nand_flash_cmd_lut0::NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = ""]
pub mod nand_flash_cmd_lut0;
#[doc = "NAND_FLASH_SPI_SEQ0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_spi_seq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nand_flash_spi_seq0`] module"]
pub type NAND_FLASH_SPI_SEQ0 = crate::Reg<nand_flash_spi_seq0::NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = ""]
pub mod nand_flash_spi_seq0;
#[doc = "XTS_PLAIN_BASE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_plain_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_plain_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_plain_base`] module"]
pub type XTS_PLAIN_BASE = crate::Reg<xts_plain_base::XTS_PLAIN_BASE_SPEC>;
#[doc = ""]
pub mod xts_plain_base;
#[doc = "XTS_LINESIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_linesize`] module"]
pub type XTS_LINESIZE = crate::Reg<xts_linesize::XTS_LINESIZE_SPEC>;
#[doc = ""]
pub mod xts_linesize;
#[doc = "XTS_DESTINATION (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destination`] module"]
pub type XTS_DESTINATION = crate::Reg<xts_destination::XTS_DESTINATION_SPEC>;
#[doc = ""]
pub mod xts_destination;
#[doc = "XTS_PHYSICAL_ADDRESS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_physical_address`] module"]
pub type XTS_PHYSICAL_ADDRESS = crate::Reg<xts_physical_address::XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = ""]
pub mod xts_physical_address;
#[doc = "XTS_TRIGGER (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_trigger`] module"]
pub type XTS_TRIGGER = crate::Reg<xts_trigger::XTS_TRIGGER_SPEC>;
#[doc = ""]
pub mod xts_trigger;
#[doc = "XTS_RELEASE (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_release`] module"]
pub type XTS_RELEASE = crate::Reg<xts_release::XTS_RELEASE_SPEC>;
#[doc = ""]
pub mod xts_release;
#[doc = "XTS_DESTROY (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_destroy`] module"]
pub type XTS_DESTROY = crate::Reg<xts_destroy::XTS_DESTROY_SPEC>;
#[doc = ""]
pub mod xts_destroy;
#[doc = "XTS_STATE (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_state`] module"]
pub type XTS_STATE = crate::Reg<xts_state::XTS_STATE_SPEC>;
#[doc = ""]
pub mod xts_state;
#[doc = "XTS_DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_date`] module"]
pub type XTS_DATE = crate::Reg<xts_date::XTS_DATE_SPEC>;
#[doc = ""]
pub mod xts_date;
#[doc = "MMU_ITEM_CONTENT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_content::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_content::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_content`] module"]
pub type MMU_ITEM_CONTENT = crate::Reg<mmu_item_content::MMU_ITEM_CONTENT_SPEC>;
#[doc = ""]
pub mod mmu_item_content;
#[doc = "MMU_ITEM_INDEX (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_item_index`] module"]
pub type MMU_ITEM_INDEX = crate::Reg<mmu_item_index::MMU_ITEM_INDEX_SPEC>;
#[doc = ""]
pub mod mmu_item_index;
#[doc = "MMU_POWER_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_power_ctrl`] module"]
pub type MMU_POWER_CTRL = crate::Reg<mmu_power_ctrl::MMU_POWER_CTRL_SPEC>;
#[doc = ""]
pub mod mmu_power_ctrl;
#[doc = "DPA_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dpa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpa_ctrl`] module"]
pub type DPA_CTRL = crate::Reg<dpa_ctrl::DPA_CTRL_SPEC>;
#[doc = ""]
pub mod dpa_ctrl;
#[doc = "XTS_PSEUDO_ROUND_CONF (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`xts_pseudo_round_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_pseudo_round_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xts_pseudo_round_conf`] module"]
pub type XTS_PSEUDO_ROUND_CONF = crate::Reg<xts_pseudo_round_conf::XTS_PSEUDO_ROUND_CONF_SPEC>;
#[doc = ""]
pub mod xts_pseudo_round_conf;
#[doc = "REGISTERRND_ECO_HIGH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_high`] module"]
pub type REGISTERRND_ECO_HIGH = crate::Reg<registerrnd_eco_high::REGISTERRND_ECO_HIGH_SPEC>;
#[doc = ""]
pub mod registerrnd_eco_high;
#[doc = "REGISTERRND_ECO_LOW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@registerrnd_eco_low`] module"]
pub type REGISTERRND_ECO_LOW = crate::Reg<registerrnd_eco_low::REGISTERRND_ECO_LOW_SPEC>;
#[doc = ""]
pub mod registerrnd_eco_low;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
