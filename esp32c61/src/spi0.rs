#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mem_cmd: MEM_CMD,
    _reserved1: [u8; 0x04],
    mem_ctrl: MEM_CTRL,
    mem_ctrl1: MEM_CTRL1,
    mem_ctrl2: MEM_CTRL2,
    mem_clock: MEM_CLOCK,
    mem_user: MEM_USER,
    mem_user1: MEM_USER1,
    mem_user2: MEM_USER2,
    _reserved8: [u8; 0x08],
    mem_rd_status: MEM_RD_STATUS,
    _reserved9: [u8; 0x04],
    mem_misc: MEM_MISC,
    _reserved10: [u8; 0x04],
    mem_cache_fctrl: MEM_CACHE_FCTRL,
    mem_cache_sctrl: MEM_CACHE_SCTRL,
    mem_sram_cmd: MEM_SRAM_CMD,
    mem_sram_drd_cmd: MEM_SRAM_DRD_CMD,
    mem_sram_dwr_cmd: MEM_SRAM_DWR_CMD,
    mem_sram_clk: MEM_SRAM_CLK,
    mem_fsm: MEM_FSM,
    _reserved17: [u8; 0x68],
    mem_int_ena: MEM_INT_ENA,
    mem_int_clr: MEM_INT_CLR,
    mem_int_raw: MEM_INT_RAW,
    mem_int_st: MEM_INT_ST,
    _reserved21: [u8; 0x04],
    mem_ddr: MEM_DDR,
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
    mem_pms_reject: MEM_PMS_REJECT,
    mem_pms_reject_addr: MEM_PMS_REJECT_ADDR,
    mem_ecc_ctrl: MEM_ECC_CTRL,
    mem_ecc_err_addr: MEM_ECC_ERR_ADDR,
    mem_axi_err_addr: MEM_AXI_ERR_ADDR,
    smem_ecc_ctrl: SMEM_ECC_CTRL,
    smem_axi_addr_ctrl: SMEM_AXI_ADDR_CTRL,
    mem_axi_err_resp_en: MEM_AXI_ERR_RESP_EN,
    mem_timing_cali: MEM_TIMING_CALI,
    mem_din_mode: MEM_DIN_MODE,
    mem_din_num: MEM_DIN_NUM,
    mem_dout_mode: MEM_DOUT_MODE,
    smem_timing_cali: SMEM_TIMING_CALI,
    smem_din_mode: SMEM_DIN_MODE,
    smem_din_num: SMEM_DIN_NUM,
    smem_dout_mode: SMEM_DOUT_MODE,
    smem_ac: SMEM_AC,
    smem_din_hex_mode: SMEM_DIN_HEX_MODE,
    smem_din_hex_num: SMEM_DIN_HEX_NUM,
    smem_dout_hex_mode: SMEM_DOUT_HEX_MODE,
    _reserved55: [u8; 0x50],
    mem_clock_gate: MEM_CLOCK_GATE,
    mem_nand_flash_en: MEM_NAND_FLASH_EN,
    mem_nand_flash_sr_addr0: MEM_NAND_FLASH_SR_ADDR0,
    mem_nand_flash_sr_din0: MEM_NAND_FLASH_SR_DIN0,
    mem_nand_flash_cfg_data0: MEM_NAND_FLASH_CFG_DATA0,
    mem_nand_flash_cfg_data1: MEM_NAND_FLASH_CFG_DATA1,
    mem_nand_flash_cfg_data2: MEM_NAND_FLASH_CFG_DATA2,
    _reserved62: [u8; 0x24],
    mem_nand_flash_cmd_lut0: MEM_NAND_FLASH_CMD_LUT0,
    _reserved63: [u8; 0x3c],
    mem_nand_flash_spi_seq0: MEM_NAND_FLASH_SPI_SEQ0,
    _reserved64: [u8; 0x7c],
    mem_xts_plain_base: MEM_XTS_PLAIN_BASE,
    _reserved65: [u8; 0x3c],
    mem_xts_linesize: MEM_XTS_LINESIZE,
    mem_xts_destination: MEM_XTS_DESTINATION,
    mem_xts_physical_address: MEM_XTS_PHYSICAL_ADDRESS,
    mem_xts_trigger: MEM_XTS_TRIGGER,
    mem_xts_release: MEM_XTS_RELEASE,
    mem_xts_destroy: MEM_XTS_DESTROY,
    mem_xts_state: MEM_XTS_STATE,
    mem_xts_date: MEM_XTS_DATE,
    _reserved73: [u8; 0x1c],
    mem_mmu_item_content: MEM_MMU_ITEM_CONTENT,
    mem_mmu_item_index: MEM_MMU_ITEM_INDEX,
    mem_mmu_power_ctrl: MEM_MMU_POWER_CTRL,
    mem_dpa_ctrl: MEM_DPA_CTRL,
    mem_xts_pseudo_round_conf: MEM_XTS_PSEUDO_ROUND_CONF,
    _reserved78: [u8; 0x60],
    mem_registerrnd_eco_high: MEM_REGISTERRND_ECO_HIGH,
    mem_registerrnd_eco_low: MEM_REGISTERRND_ECO_LOW,
    _reserved80: [u8; 0x04],
    mem_date: MEM_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn mem_cmd(&self) -> &MEM_CMD {
        &self.mem_cmd
    }
    #[doc = "0x08 - SPI0 control register."]
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MEM_CTRL {
        &self.mem_ctrl
    }
    #[doc = "0x0c - SPI0 control1 register."]
    #[inline(always)]
    pub const fn mem_ctrl1(&self) -> &MEM_CTRL1 {
        &self.mem_ctrl1
    }
    #[doc = "0x10 - SPI0 control2 register."]
    #[inline(always)]
    pub const fn mem_ctrl2(&self) -> &MEM_CTRL2 {
        &self.mem_ctrl2
    }
    #[doc = "0x14 - SPI clock division control register."]
    #[inline(always)]
    pub const fn mem_clock(&self) -> &MEM_CLOCK {
        &self.mem_clock
    }
    #[doc = "0x18 - SPI0 user register."]
    #[inline(always)]
    pub const fn mem_user(&self) -> &MEM_USER {
        &self.mem_user
    }
    #[doc = "0x1c - SPI0 user1 register."]
    #[inline(always)]
    pub const fn mem_user1(&self) -> &MEM_USER1 {
        &self.mem_user1
    }
    #[doc = "0x20 - SPI0 user2 register."]
    #[inline(always)]
    pub const fn mem_user2(&self) -> &MEM_USER2 {
        &self.mem_user2
    }
    #[doc = "0x2c - SPI0 read control register."]
    #[inline(always)]
    pub const fn mem_rd_status(&self) -> &MEM_RD_STATUS {
        &self.mem_rd_status
    }
    #[doc = "0x34 - SPI0 misc register"]
    #[inline(always)]
    pub const fn mem_misc(&self) -> &MEM_MISC {
        &self.mem_misc
    }
    #[doc = "0x3c - SPI0 bit mode control register."]
    #[inline(always)]
    pub const fn mem_cache_fctrl(&self) -> &MEM_CACHE_FCTRL {
        &self.mem_cache_fctrl
    }
    #[doc = "0x40 - SPI0 external RAM control register"]
    #[inline(always)]
    pub const fn mem_cache_sctrl(&self) -> &MEM_CACHE_SCTRL {
        &self.mem_cache_sctrl
    }
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    #[inline(always)]
    pub const fn mem_sram_cmd(&self) -> &MEM_SRAM_CMD {
        &self.mem_sram_cmd
    }
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    #[inline(always)]
    pub const fn mem_sram_drd_cmd(&self) -> &MEM_SRAM_DRD_CMD {
        &self.mem_sram_drd_cmd
    }
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    #[inline(always)]
    pub const fn mem_sram_dwr_cmd(&self) -> &MEM_SRAM_DWR_CMD {
        &self.mem_sram_dwr_cmd
    }
    #[doc = "0x50 - SPI0 external RAM clock control register"]
    #[inline(always)]
    pub const fn mem_sram_clk(&self) -> &MEM_SRAM_CLK {
        &self.mem_sram_clk
    }
    #[doc = "0x54 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn mem_fsm(&self) -> &MEM_FSM {
        &self.mem_fsm
    }
    #[doc = "0xc0 - SPI0 interrupt enable register"]
    #[inline(always)]
    pub const fn mem_int_ena(&self) -> &MEM_INT_ENA {
        &self.mem_int_ena
    }
    #[doc = "0xc4 - SPI0 interrupt clear register"]
    #[inline(always)]
    pub const fn mem_int_clr(&self) -> &MEM_INT_CLR {
        &self.mem_int_clr
    }
    #[doc = "0xc8 - SPI0 interrupt raw register"]
    #[inline(always)]
    pub const fn mem_int_raw(&self) -> &MEM_INT_RAW {
        &self.mem_int_raw
    }
    #[doc = "0xcc - SPI0 interrupt status register"]
    #[inline(always)]
    pub const fn mem_int_st(&self) -> &MEM_INT_ST {
        &self.mem_int_st
    }
    #[doc = "0xd4 - SPI0 flash DDR mode control register"]
    #[inline(always)]
    pub const fn mem_ddr(&self) -> &MEM_DDR {
        &self.mem_ddr
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
    pub const fn mem_pms_reject(&self) -> &MEM_PMS_REJECT {
        &self.mem_pms_reject
    }
    #[doc = "0x164 - SPI1 access reject addr register"]
    #[inline(always)]
    pub const fn mem_pms_reject_addr(&self) -> &MEM_PMS_REJECT_ADDR {
        &self.mem_pms_reject_addr
    }
    #[doc = "0x168 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn mem_ecc_ctrl(&self) -> &MEM_ECC_CTRL {
        &self.mem_ecc_ctrl
    }
    #[doc = "0x16c - MSPI ECC error address register"]
    #[inline(always)]
    pub const fn mem_ecc_err_addr(&self) -> &MEM_ECC_ERR_ADDR {
        &self.mem_ecc_err_addr
    }
    #[doc = "0x170 - SPI0 AXI request error address."]
    #[inline(always)]
    pub const fn mem_axi_err_addr(&self) -> &MEM_AXI_ERR_ADDR {
        &self.mem_axi_err_addr
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
    pub const fn mem_axi_err_resp_en(&self) -> &MEM_AXI_ERR_RESP_EN {
        &self.mem_axi_err_resp_en
    }
    #[doc = "0x180 - SPI0 flash timing calibration register"]
    #[inline(always)]
    pub const fn mem_timing_cali(&self) -> &MEM_TIMING_CALI {
        &self.mem_timing_cali
    }
    #[doc = "0x184 - MSPI flash input timing delay mode control register"]
    #[inline(always)]
    pub const fn mem_din_mode(&self) -> &MEM_DIN_MODE {
        &self.mem_din_mode
    }
    #[doc = "0x188 - MSPI flash input timing delay number control register"]
    #[inline(always)]
    pub const fn mem_din_num(&self) -> &MEM_DIN_NUM {
        &self.mem_din_num
    }
    #[doc = "0x18c - MSPI flash output timing adjustment control register"]
    #[inline(always)]
    pub const fn mem_dout_mode(&self) -> &MEM_DOUT_MODE {
        &self.mem_dout_mode
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
    pub const fn mem_clock_gate(&self) -> &MEM_CLOCK_GATE {
        &self.mem_clock_gate
    }
    #[doc = "0x204 - NAND FLASH control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_en(&self) -> &MEM_NAND_FLASH_EN {
        &self.mem_nand_flash_en
    }
    #[doc = "0x208 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_sr_addr0(&self) -> &MEM_NAND_FLASH_SR_ADDR0 {
        &self.mem_nand_flash_sr_addr0
    }
    #[doc = "0x20c - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_sr_din0(&self) -> &MEM_NAND_FLASH_SR_DIN0 {
        &self.mem_nand_flash_sr_din0
    }
    #[doc = "0x210 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_cfg_data0(&self) -> &MEM_NAND_FLASH_CFG_DATA0 {
        &self.mem_nand_flash_cfg_data0
    }
    #[doc = "0x214 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_cfg_data1(&self) -> &MEM_NAND_FLASH_CFG_DATA1 {
        &self.mem_nand_flash_cfg_data1
    }
    #[doc = "0x218 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_cfg_data2(&self) -> &MEM_NAND_FLASH_CFG_DATA2 {
        &self.mem_nand_flash_cfg_data2
    }
    #[doc = "0x240 - MSPI NAND FLASH CMD LUT control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_cmd_lut0(&self) -> &MEM_NAND_FLASH_CMD_LUT0 {
        &self.mem_nand_flash_cmd_lut0
    }
    #[doc = "0x280 - NAND FLASH SPI SEQ control register"]
    #[inline(always)]
    pub const fn mem_nand_flash_spi_seq0(&self) -> &MEM_NAND_FLASH_SPI_SEQ0 {
        &self.mem_nand_flash_spi_seq0
    }
    #[doc = "0x300 - The base address of the memory that stores plaintext in Manual Encryption"]
    #[inline(always)]
    pub const fn mem_xts_plain_base(&self) -> &MEM_XTS_PLAIN_BASE {
        &self.mem_xts_plain_base
    }
    #[doc = "0x340 - Manual Encryption Line-Size register"]
    #[inline(always)]
    pub const fn mem_xts_linesize(&self) -> &MEM_XTS_LINESIZE {
        &self.mem_xts_linesize
    }
    #[doc = "0x344 - Manual Encryption destination register"]
    #[inline(always)]
    pub const fn mem_xts_destination(&self) -> &MEM_XTS_DESTINATION {
        &self.mem_xts_destination
    }
    #[doc = "0x348 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn mem_xts_physical_address(&self) -> &MEM_XTS_PHYSICAL_ADDRESS {
        &self.mem_xts_physical_address
    }
    #[doc = "0x34c - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn mem_xts_trigger(&self) -> &MEM_XTS_TRIGGER {
        &self.mem_xts_trigger
    }
    #[doc = "0x350 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn mem_xts_release(&self) -> &MEM_XTS_RELEASE {
        &self.mem_xts_release
    }
    #[doc = "0x354 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn mem_xts_destroy(&self) -> &MEM_XTS_DESTROY {
        &self.mem_xts_destroy
    }
    #[doc = "0x358 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn mem_xts_state(&self) -> &MEM_XTS_STATE {
        &self.mem_xts_state
    }
    #[doc = "0x35c - Manual Encryption version register"]
    #[inline(always)]
    pub const fn mem_xts_date(&self) -> &MEM_XTS_DATE {
        &self.mem_xts_date
    }
    #[doc = "0x37c - MSPI-MMU item content register"]
    #[inline(always)]
    pub const fn mem_mmu_item_content(&self) -> &MEM_MMU_ITEM_CONTENT {
        &self.mem_mmu_item_content
    }
    #[doc = "0x380 - MSPI-MMU item index register"]
    #[inline(always)]
    pub const fn mem_mmu_item_index(&self) -> &MEM_MMU_ITEM_INDEX {
        &self.mem_mmu_item_index
    }
    #[doc = "0x384 - MSPI MMU power control register"]
    #[inline(always)]
    pub const fn mem_mmu_power_ctrl(&self) -> &MEM_MMU_POWER_CTRL {
        &self.mem_mmu_power_ctrl
    }
    #[doc = "0x388 - SPI memory cryption DPA register"]
    #[inline(always)]
    pub const fn mem_dpa_ctrl(&self) -> &MEM_DPA_CTRL {
        &self.mem_dpa_ctrl
    }
    #[doc = "0x38c - SPI memory cryption PSEUDO register"]
    #[inline(always)]
    pub const fn mem_xts_pseudo_round_conf(&self) -> &MEM_XTS_PSEUDO_ROUND_CONF {
        &self.mem_xts_pseudo_round_conf
    }
    #[doc = "0x3f0 - MSPI ECO high register"]
    #[inline(always)]
    pub const fn mem_registerrnd_eco_high(&self) -> &MEM_REGISTERRND_ECO_HIGH {
        &self.mem_registerrnd_eco_high
    }
    #[doc = "0x3f4 - MSPI ECO low register"]
    #[inline(always)]
    pub const fn mem_registerrnd_eco_low(&self) -> &MEM_REGISTERRND_ECO_LOW {
        &self.mem_registerrnd_eco_low
    }
    #[doc = "0x3fc - SPI0 version control register"]
    #[inline(always)]
    pub const fn mem_date(&self) -> &MEM_DATE {
        &self.mem_date
    }
}
#[doc = "MEM_CMD (r) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_cmd`] module"]
pub type MEM_CMD = crate::Reg<mem_cmd::MEM_CMD_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod mem_cmd;
#[doc = "MEM_CTRL (rw) register accessor: SPI0 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl`] module"]
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod mem_ctrl;
#[doc = "MEM_CTRL1 (rw) register accessor: SPI0 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl1`] module"]
pub type MEM_CTRL1 = crate::Reg<mem_ctrl1::MEM_CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod mem_ctrl1;
#[doc = "MEM_CTRL2 (rw) register accessor: SPI0 control2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctrl2`] module"]
pub type MEM_CTRL2 = crate::Reg<mem_ctrl2::MEM_CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod mem_ctrl2;
#[doc = "MEM_CLOCK (rw) register accessor: SPI clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clock`] module"]
pub type MEM_CLOCK = crate::Reg<mem_clock::MEM_CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod mem_clock;
#[doc = "MEM_USER (rw) register accessor: SPI0 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_user`] module"]
pub type MEM_USER = crate::Reg<mem_user::MEM_USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod mem_user;
#[doc = "MEM_USER1 (rw) register accessor: SPI0 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_user1`] module"]
pub type MEM_USER1 = crate::Reg<mem_user1::MEM_USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod mem_user1;
#[doc = "MEM_USER2 (rw) register accessor: SPI0 user2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_user2`] module"]
pub type MEM_USER2 = crate::Reg<mem_user2::MEM_USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod mem_user2;
#[doc = "MEM_RD_STATUS (rw) register accessor: SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_rd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_rd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_rd_status`] module"]
pub type MEM_RD_STATUS = crate::Reg<mem_rd_status::MEM_RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod mem_rd_status;
#[doc = "MEM_MISC (rw) register accessor: SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_misc`] module"]
pub type MEM_MISC = crate::Reg<mem_misc::MEM_MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod mem_misc;
#[doc = "MEM_CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cache_fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_cache_fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_cache_fctrl`] module"]
pub type MEM_CACHE_FCTRL = crate::Reg<mem_cache_fctrl::MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod mem_cache_fctrl;
#[doc = "MEM_CACHE_SCTRL (rw) register accessor: SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cache_sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_cache_sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_cache_sctrl`] module"]
pub type MEM_CACHE_SCTRL = crate::Reg<mem_cache_sctrl::MEM_CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod mem_cache_sctrl;
#[doc = "MEM_SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sram_cmd`] module"]
pub type MEM_SRAM_CMD = crate::Reg<mem_sram_cmd::MEM_SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod mem_sram_cmd;
#[doc = "MEM_SRAM_DRD_CMD (rw) register accessor: SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_drd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_drd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sram_drd_cmd`] module"]
pub type MEM_SRAM_DRD_CMD = crate::Reg<mem_sram_drd_cmd::MEM_SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod mem_sram_drd_cmd;
#[doc = "MEM_SRAM_DWR_CMD (rw) register accessor: SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_dwr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_dwr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sram_dwr_cmd`] module"]
pub type MEM_SRAM_DWR_CMD = crate::Reg<mem_sram_dwr_cmd::MEM_SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod mem_sram_dwr_cmd;
#[doc = "MEM_SRAM_CLK (rw) register accessor: SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_sram_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_sram_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_sram_clk`] module"]
pub type MEM_SRAM_CLK = crate::Reg<mem_sram_clk::MEM_SRAM_CLK_SPEC>;
#[doc = "SPI0 external RAM clock control register"]
pub mod mem_sram_clk;
#[doc = "MEM_FSM (rw) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_fsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_fsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_fsm`] module"]
pub type MEM_FSM = crate::Reg<mem_fsm::MEM_FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod mem_fsm;
#[doc = "MEM_INT_ENA (rw) register accessor: SPI0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_int_ena`] module"]
pub type MEM_INT_ENA = crate::Reg<mem_int_ena::MEM_INT_ENA_SPEC>;
#[doc = "SPI0 interrupt enable register"]
pub mod mem_int_ena;
#[doc = "MEM_INT_CLR (w) register accessor: SPI0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_int_clr`] module"]
pub type MEM_INT_CLR = crate::Reg<mem_int_clr::MEM_INT_CLR_SPEC>;
#[doc = "SPI0 interrupt clear register"]
pub mod mem_int_clr;
#[doc = "MEM_INT_RAW (rw) register accessor: SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_int_raw`] module"]
pub type MEM_INT_RAW = crate::Reg<mem_int_raw::MEM_INT_RAW_SPEC>;
#[doc = "SPI0 interrupt raw register"]
pub mod mem_int_raw;
#[doc = "MEM_INT_ST (r) register accessor: SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_int_st`] module"]
pub type MEM_INT_ST = crate::Reg<mem_int_st::MEM_INT_ST_SPEC>;
#[doc = "SPI0 interrupt status register"]
pub mod mem_int_st;
#[doc = "MEM_DDR (rw) register accessor: SPI0 flash DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ddr`] module"]
pub type MEM_DDR = crate::Reg<mem_ddr::MEM_DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod mem_ddr;
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
#[doc = "MEM_PMS_REJECT (rw) register accessor: SPI1 access reject register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_pms_reject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_pms_reject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_pms_reject`] module"]
pub type MEM_PMS_REJECT = crate::Reg<mem_pms_reject::MEM_PMS_REJECT_SPEC>;
#[doc = "SPI1 access reject register"]
pub mod mem_pms_reject;
#[doc = "MEM_PMS_REJECT_ADDR (r) register accessor: SPI1 access reject addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_pms_reject_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_pms_reject_addr`] module"]
pub type MEM_PMS_REJECT_ADDR = crate::Reg<mem_pms_reject_addr::MEM_PMS_REJECT_ADDR_SPEC>;
#[doc = "SPI1 access reject addr register"]
pub mod mem_pms_reject_addr;
#[doc = "MEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ecc_ctrl`] module"]
pub type MEM_ECC_CTRL = crate::Reg<mem_ecc_ctrl::MEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod mem_ecc_ctrl;
#[doc = "MEM_ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ecc_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ecc_err_addr`] module"]
pub type MEM_ECC_ERR_ADDR = crate::Reg<mem_ecc_err_addr::MEM_ECC_ERR_ADDR_SPEC>;
#[doc = "MSPI ECC error address register"]
pub mod mem_ecc_err_addr;
#[doc = "MEM_AXI_ERR_ADDR (r) register accessor: SPI0 AXI request error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_axi_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_axi_err_addr`] module"]
pub type MEM_AXI_ERR_ADDR = crate::Reg<mem_axi_err_addr::MEM_AXI_ERR_ADDR_SPEC>;
#[doc = "SPI0 AXI request error address."]
pub mod mem_axi_err_addr;
#[doc = "SMEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ecc_ctrl`] module"]
pub type SMEM_ECC_CTRL = crate::Reg<smem_ecc_ctrl::SMEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod smem_ecc_ctrl;
#[doc = "SMEM_AXI_ADDR_CTRL (r) register accessor: SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_axi_addr_ctrl`] module"]
pub type SMEM_AXI_ADDR_CTRL = crate::Reg<smem_axi_addr_ctrl::SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "SPI0 AXI address control register"]
pub mod smem_axi_addr_ctrl;
#[doc = "MEM_AXI_ERR_RESP_EN (rw) register accessor: SPI0 AXI error response enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_axi_err_resp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_axi_err_resp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_axi_err_resp_en`] module"]
pub type MEM_AXI_ERR_RESP_EN = crate::Reg<mem_axi_err_resp_en::MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "SPI0 AXI error response enable register"]
pub mod mem_axi_err_resp_en;
#[doc = "MEM_TIMING_CALI (rw) register accessor: SPI0 flash timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_timing_cali`] module"]
pub type MEM_TIMING_CALI = crate::Reg<mem_timing_cali::MEM_TIMING_CALI_SPEC>;
#[doc = "SPI0 flash timing calibration register"]
pub mod mem_timing_cali;
#[doc = "MEM_DIN_MODE (rw) register accessor: MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_din_mode`] module"]
pub type MEM_DIN_MODE = crate::Reg<mem_din_mode::MEM_DIN_MODE_SPEC>;
#[doc = "MSPI flash input timing delay mode control register"]
pub mod mem_din_mode;
#[doc = "MEM_DIN_NUM (rw) register accessor: MSPI flash input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_din_num`] module"]
pub type MEM_DIN_NUM = crate::Reg<mem_din_num::MEM_DIN_NUM_SPEC>;
#[doc = "MSPI flash input timing delay number control register"]
pub mod mem_din_num;
#[doc = "MEM_DOUT_MODE (rw) register accessor: MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dout_mode`] module"]
pub type MEM_DOUT_MODE = crate::Reg<mem_dout_mode::MEM_DOUT_MODE_SPEC>;
#[doc = "MSPI flash output timing adjustment control register"]
pub mod mem_dout_mode;
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
#[doc = "MEM_CLOCK_GATE (rw) register accessor: SPI0 clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clock_gate`] module"]
pub type MEM_CLOCK_GATE = crate::Reg<mem_clock_gate::MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI0 clock gate register"]
pub mod mem_clock_gate;
#[doc = "MEM_NAND_FLASH_EN (r) register accessor: NAND FLASH control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_en::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_en`] module"]
pub type MEM_NAND_FLASH_EN = crate::Reg<mem_nand_flash_en::MEM_NAND_FLASH_EN_SPEC>;
#[doc = "NAND FLASH control register"]
pub mod mem_nand_flash_en;
#[doc = "MEM_NAND_FLASH_SR_ADDR0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_sr_addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_sr_addr0`] module"]
pub type MEM_NAND_FLASH_SR_ADDR0 =
    crate::Reg<mem_nand_flash_sr_addr0::MEM_NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_sr_addr0;
#[doc = "MEM_NAND_FLASH_SR_DIN0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_sr_din0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_sr_din0`] module"]
pub type MEM_NAND_FLASH_SR_DIN0 = crate::Reg<mem_nand_flash_sr_din0::MEM_NAND_FLASH_SR_DIN0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_sr_din0;
#[doc = "MEM_NAND_FLASH_CFG_DATA0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cfg_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_cfg_data0`] module"]
pub type MEM_NAND_FLASH_CFG_DATA0 =
    crate::Reg<mem_nand_flash_cfg_data0::MEM_NAND_FLASH_CFG_DATA0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_cfg_data0;
#[doc = "MEM_NAND_FLASH_CFG_DATA1 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cfg_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_cfg_data1`] module"]
pub type MEM_NAND_FLASH_CFG_DATA1 =
    crate::Reg<mem_nand_flash_cfg_data1::MEM_NAND_FLASH_CFG_DATA1_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_cfg_data1;
#[doc = "MEM_NAND_FLASH_CFG_DATA2 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cfg_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_cfg_data2`] module"]
pub type MEM_NAND_FLASH_CFG_DATA2 =
    crate::Reg<mem_nand_flash_cfg_data2::MEM_NAND_FLASH_CFG_DATA2_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_cfg_data2;
#[doc = "MEM_NAND_FLASH_CMD_LUT0 (r) register accessor: MSPI NAND FLASH CMD LUT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_cmd_lut0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_cmd_lut0`] module"]
pub type MEM_NAND_FLASH_CMD_LUT0 =
    crate::Reg<mem_nand_flash_cmd_lut0::MEM_NAND_FLASH_CMD_LUT0_SPEC>;
#[doc = "MSPI NAND FLASH CMD LUT control register"]
pub mod mem_nand_flash_cmd_lut0;
#[doc = "MEM_NAND_FLASH_SPI_SEQ0 (r) register accessor: NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_nand_flash_spi_seq0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_nand_flash_spi_seq0`] module"]
pub type MEM_NAND_FLASH_SPI_SEQ0 =
    crate::Reg<mem_nand_flash_spi_seq0::MEM_NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "NAND FLASH SPI SEQ control register"]
pub mod mem_nand_flash_spi_seq0;
#[doc = "MEM_XTS_PLAIN_BASE (rw) register accessor: The base address of the memory that stores plaintext in Manual Encryption\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_plain_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_plain_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_plain_base`] module"]
pub type MEM_XTS_PLAIN_BASE = crate::Reg<mem_xts_plain_base::MEM_XTS_PLAIN_BASE_SPEC>;
#[doc = "The base address of the memory that stores plaintext in Manual Encryption"]
pub mod mem_xts_plain_base;
#[doc = "MEM_XTS_LINESIZE (rw) register accessor: Manual Encryption Line-Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_linesize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_linesize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_linesize`] module"]
pub type MEM_XTS_LINESIZE = crate::Reg<mem_xts_linesize::MEM_XTS_LINESIZE_SPEC>;
#[doc = "Manual Encryption Line-Size register"]
pub mod mem_xts_linesize;
#[doc = "MEM_XTS_DESTINATION (rw) register accessor: Manual Encryption destination register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_destination::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_destination::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_destination`] module"]
pub type MEM_XTS_DESTINATION = crate::Reg<mem_xts_destination::MEM_XTS_DESTINATION_SPEC>;
#[doc = "Manual Encryption destination register"]
pub mod mem_xts_destination;
#[doc = "MEM_XTS_PHYSICAL_ADDRESS (rw) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_physical_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_physical_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_physical_address`] module"]
pub type MEM_XTS_PHYSICAL_ADDRESS =
    crate::Reg<mem_xts_physical_address::MEM_XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod mem_xts_physical_address;
#[doc = "MEM_XTS_TRIGGER (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_trigger`] module"]
pub type MEM_XTS_TRIGGER = crate::Reg<mem_xts_trigger::MEM_XTS_TRIGGER_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod mem_xts_trigger;
#[doc = "MEM_XTS_RELEASE (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_release`] module"]
pub type MEM_XTS_RELEASE = crate::Reg<mem_xts_release::MEM_XTS_RELEASE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod mem_xts_release;
#[doc = "MEM_XTS_DESTROY (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_destroy`] module"]
pub type MEM_XTS_DESTROY = crate::Reg<mem_xts_destroy::MEM_XTS_DESTROY_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod mem_xts_destroy;
#[doc = "MEM_XTS_STATE (r) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_state`] module"]
pub type MEM_XTS_STATE = crate::Reg<mem_xts_state::MEM_XTS_STATE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod mem_xts_state;
#[doc = "MEM_XTS_DATE (rw) register accessor: Manual Encryption version register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_date`] module"]
pub type MEM_XTS_DATE = crate::Reg<mem_xts_date::MEM_XTS_DATE_SPEC>;
#[doc = "Manual Encryption version register"]
pub mod mem_xts_date;
#[doc = "MEM_MMU_ITEM_CONTENT (rw) register accessor: MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_mmu_item_content::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_mmu_item_content::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mmu_item_content`] module"]
pub type MEM_MMU_ITEM_CONTENT = crate::Reg<mem_mmu_item_content::MEM_MMU_ITEM_CONTENT_SPEC>;
#[doc = "MSPI-MMU item content register"]
pub mod mem_mmu_item_content;
#[doc = "MEM_MMU_ITEM_INDEX (rw) register accessor: MSPI-MMU item index register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_mmu_item_index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_mmu_item_index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mmu_item_index`] module"]
pub type MEM_MMU_ITEM_INDEX = crate::Reg<mem_mmu_item_index::MEM_MMU_ITEM_INDEX_SPEC>;
#[doc = "MSPI-MMU item index register"]
pub mod mem_mmu_item_index;
#[doc = "MEM_MMU_POWER_CTRL (rw) register accessor: MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_mmu_power_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_mmu_power_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_mmu_power_ctrl`] module"]
pub type MEM_MMU_POWER_CTRL = crate::Reg<mem_mmu_power_ctrl::MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "MSPI MMU power control register"]
pub mod mem_mmu_power_ctrl;
#[doc = "MEM_DPA_CTRL (rw) register accessor: SPI memory cryption DPA register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_dpa_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_dpa_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_dpa_ctrl`] module"]
pub type MEM_DPA_CTRL = crate::Reg<mem_dpa_ctrl::MEM_DPA_CTRL_SPEC>;
#[doc = "SPI memory cryption DPA register"]
pub mod mem_dpa_ctrl;
#[doc = "MEM_XTS_PSEUDO_ROUND_CONF (rw) register accessor: SPI memory cryption PSEUDO register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_xts_pseudo_round_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_pseudo_round_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_xts_pseudo_round_conf`] module"]
pub type MEM_XTS_PSEUDO_ROUND_CONF =
    crate::Reg<mem_xts_pseudo_round_conf::MEM_XTS_PSEUDO_ROUND_CONF_SPEC>;
#[doc = "SPI memory cryption PSEUDO register"]
pub mod mem_xts_pseudo_round_conf;
#[doc = "MEM_REGISTERRND_ECO_HIGH (rw) register accessor: MSPI ECO high register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_registerrnd_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_registerrnd_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_registerrnd_eco_high`] module"]
pub type MEM_REGISTERRND_ECO_HIGH =
    crate::Reg<mem_registerrnd_eco_high::MEM_REGISTERRND_ECO_HIGH_SPEC>;
#[doc = "MSPI ECO high register"]
pub mod mem_registerrnd_eco_high;
#[doc = "MEM_REGISTERRND_ECO_LOW (rw) register accessor: MSPI ECO low register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_registerrnd_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_registerrnd_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_registerrnd_eco_low`] module"]
pub type MEM_REGISTERRND_ECO_LOW =
    crate::Reg<mem_registerrnd_eco_low::MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "MSPI ECO low register"]
pub mod mem_registerrnd_eco_low;
#[doc = "MEM_DATE (rw) register accessor: SPI0 version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_date`] module"]
pub type MEM_DATE = crate::Reg<mem_date::MEM_DATE_SPEC>;
#[doc = "SPI0 version control register"]
pub mod mem_date;
