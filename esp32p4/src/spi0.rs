#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_mem_cmd: SPI_MEM_CMD,
    _reserved1: [u8; 0x04],
    spi_mem_ctrl: SPI_MEM_CTRL,
    spi_mem_ctrl1: SPI_MEM_CTRL1,
    spi_mem_ctrl2: SPI_MEM_CTRL2,
    spi_mem_clock: SPI_MEM_CLOCK,
    spi_mem_user: SPI_MEM_USER,
    spi_mem_user1: SPI_MEM_USER1,
    spi_mem_user2: SPI_MEM_USER2,
    _reserved8: [u8; 0x08],
    spi_mem_rd_status: SPI_MEM_RD_STATUS,
    _reserved9: [u8; 0x04],
    spi_mem_misc: SPI_MEM_MISC,
    _reserved10: [u8; 0x04],
    spi_mem_cache_fctrl: SPI_MEM_CACHE_FCTRL,
    spi_mem_cache_sctrl: SPI_MEM_CACHE_SCTRL,
    spi_mem_sram_cmd: SPI_MEM_SRAM_CMD,
    spi_mem_sram_drd_cmd: SPI_MEM_SRAM_DRD_CMD,
    spi_mem_sram_dwr_cmd: SPI_MEM_SRAM_DWR_CMD,
    spi_mem_sram_clk: SPI_MEM_SRAM_CLK,
    spi_mem_fsm: SPI_MEM_FSM,
    _reserved17: [u8; 0x68],
    spi_mem_int_ena: SPI_MEM_INT_ENA,
    spi_mem_int_clr: SPI_MEM_INT_CLR,
    spi_mem_int_raw: SPI_MEM_INT_RAW,
    spi_mem_int_st: SPI_MEM_INT_ST,
    _reserved21: [u8; 0x04],
    spi_mem_ddr: SPI_MEM_DDR,
    spi_smem_ddr: SPI_SMEM_DDR,
    _reserved23: [u8; 0x24],
    spi_fmem_pms_attr: [SPI_FMEM_PMS_ATTR; 4],
    spi_fmem_pms_addr: [SPI_FMEM_PMS_ADDR; 4],
    spi_fmem_pms_size: [SPI_FMEM_PMS_SIZE; 4],
    spi_smem_pms_attr: [SPI_SMEM_PMS_ATTR; 4],
    spi_smem_pms_addr: [SPI_SMEM_PMS_ADDR; 4],
    spi_smem_pms_size: [SPI_SMEM_PMS_SIZE; 4],
    _reserved29: [u8; 0x04],
    spi_mem_pms_reject: SPI_MEM_PMS_REJECT,
    spi_mem_ecc_ctrl: SPI_MEM_ECC_CTRL,
    spi_mem_ecc_err_addr: SPI_MEM_ECC_ERR_ADDR,
    spi_mem_axi_err_addr: SPI_MEM_AXI_ERR_ADDR,
    spi_smem_ecc_ctrl: SPI_SMEM_ECC_CTRL,
    spi_smem_axi_addr_ctrl: SPI_SMEM_AXI_ADDR_CTRL,
    spi_mem_axi_err_resp_en: SPI_MEM_AXI_ERR_RESP_EN,
    spi_mem_timing_cali: SPI_MEM_TIMING_CALI,
    spi_mem_din_mode: SPI_MEM_DIN_MODE,
    spi_mem_din_num: SPI_MEM_DIN_NUM,
    spi_mem_dout_mode: SPI_MEM_DOUT_MODE,
    spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    spi_smem_din_num: SPI_SMEM_DIN_NUM,
    spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    spi_smem_ac: SPI_SMEM_AC,
    spi_smem_din_hex_mode: SPI_SMEM_DIN_HEX_MODE,
    spi_smem_din_hex_num: SPI_SMEM_DIN_HEX_NUM,
    spi_smem_dout_hex_mode: SPI_SMEM_DOUT_HEX_MODE,
    _reserved48: [u8; 0x50],
    spi_mem_clock_gate: SPI_MEM_CLOCK_GATE,
    _reserved49: [u8; 0xfc],
    spi_mem_xts_plain_base: SPI_MEM_XTS_PLAIN_BASE,
    _reserved50: [u8; 0x3c],
    spi_mem_xts_linesize: SPI_MEM_XTS_LINESIZE,
    spi_mem_xts_destination: SPI_MEM_XTS_DESTINATION,
    spi_mem_xts_physical_address: SPI_MEM_XTS_PHYSICAL_ADDRESS,
    spi_mem_xts_trigger: SPI_MEM_XTS_TRIGGER,
    spi_mem_xts_release: SPI_MEM_XTS_RELEASE,
    spi_mem_xts_destroy: SPI_MEM_XTS_DESTROY,
    spi_mem_xts_state: SPI_MEM_XTS_STATE,
    spi_mem_xts_date: SPI_MEM_XTS_DATE,
    _reserved58: [u8; 0x1c],
    spi_mem_mmu_item_content: SPI_MEM_MMU_ITEM_CONTENT,
    spi_mem_mmu_item_index: SPI_MEM_MMU_ITEM_INDEX,
    spi_mem_mmu_power_ctrl: SPI_MEM_MMU_POWER_CTRL,
    spi_mem_dpa_ctrl: SPI_MEM_DPA_CTRL,
    _reserved62: [u8; 0x64],
    spi_mem_registerrnd_eco_high: SPI_MEM_REGISTERRND_ECO_HIGH,
    spi_mem_registerrnd_eco_low: SPI_MEM_REGISTERRND_ECO_LOW,
    _reserved64: [u8; 0x04],
    spi_mem_date: SPI_MEM_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn spi_mem_cmd(&self) -> &SPI_MEM_CMD {
        &self.spi_mem_cmd
    }
    #[doc = "0x08 - SPI0 control register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl(&self) -> &SPI_MEM_CTRL {
        &self.spi_mem_ctrl
    }
    #[doc = "0x0c - SPI0 control1 register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl1(&self) -> &SPI_MEM_CTRL1 {
        &self.spi_mem_ctrl1
    }
    #[doc = "0x10 - SPI0 control2 register."]
    #[inline(always)]
    pub const fn spi_mem_ctrl2(&self) -> &SPI_MEM_CTRL2 {
        &self.spi_mem_ctrl2
    }
    #[doc = "0x14 - SPI clock division control register."]
    #[inline(always)]
    pub const fn spi_mem_clock(&self) -> &SPI_MEM_CLOCK {
        &self.spi_mem_clock
    }
    #[doc = "0x18 - SPI0 user register."]
    #[inline(always)]
    pub const fn spi_mem_user(&self) -> &SPI_MEM_USER {
        &self.spi_mem_user
    }
    #[doc = "0x1c - SPI0 user1 register."]
    #[inline(always)]
    pub const fn spi_mem_user1(&self) -> &SPI_MEM_USER1 {
        &self.spi_mem_user1
    }
    #[doc = "0x20 - SPI0 user2 register."]
    #[inline(always)]
    pub const fn spi_mem_user2(&self) -> &SPI_MEM_USER2 {
        &self.spi_mem_user2
    }
    #[doc = "0x2c - SPI0 read control register."]
    #[inline(always)]
    pub const fn spi_mem_rd_status(&self) -> &SPI_MEM_RD_STATUS {
        &self.spi_mem_rd_status
    }
    #[doc = "0x34 - SPI0 misc register"]
    #[inline(always)]
    pub const fn spi_mem_misc(&self) -> &SPI_MEM_MISC {
        &self.spi_mem_misc
    }
    #[doc = "0x3c - SPI0 bit mode control register."]
    #[inline(always)]
    pub const fn spi_mem_cache_fctrl(&self) -> &SPI_MEM_CACHE_FCTRL {
        &self.spi_mem_cache_fctrl
    }
    #[doc = "0x40 - SPI0 external RAM control register"]
    #[inline(always)]
    pub const fn spi_mem_cache_sctrl(&self) -> &SPI_MEM_CACHE_SCTRL {
        &self.spi_mem_cache_sctrl
    }
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    #[inline(always)]
    pub const fn spi_mem_sram_cmd(&self) -> &SPI_MEM_SRAM_CMD {
        &self.spi_mem_sram_cmd
    }
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    #[inline(always)]
    pub const fn spi_mem_sram_drd_cmd(&self) -> &SPI_MEM_SRAM_DRD_CMD {
        &self.spi_mem_sram_drd_cmd
    }
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    #[inline(always)]
    pub const fn spi_mem_sram_dwr_cmd(&self) -> &SPI_MEM_SRAM_DWR_CMD {
        &self.spi_mem_sram_dwr_cmd
    }
    #[doc = "0x50 - SPI0 external RAM clock control register"]
    #[inline(always)]
    pub const fn spi_mem_sram_clk(&self) -> &SPI_MEM_SRAM_CLK {
        &self.spi_mem_sram_clk
    }
    #[doc = "0x54 - SPI0 FSM status register"]
    #[inline(always)]
    pub const fn spi_mem_fsm(&self) -> &SPI_MEM_FSM {
        &self.spi_mem_fsm
    }
    #[doc = "0xc0 - SPI0 interrupt enable register"]
    #[inline(always)]
    pub const fn spi_mem_int_ena(&self) -> &SPI_MEM_INT_ENA {
        &self.spi_mem_int_ena
    }
    #[doc = "0xc4 - SPI0 interrupt clear register"]
    #[inline(always)]
    pub const fn spi_mem_int_clr(&self) -> &SPI_MEM_INT_CLR {
        &self.spi_mem_int_clr
    }
    #[doc = "0xc8 - SPI0 interrupt raw register"]
    #[inline(always)]
    pub const fn spi_mem_int_raw(&self) -> &SPI_MEM_INT_RAW {
        &self.spi_mem_int_raw
    }
    #[doc = "0xcc - SPI0 interrupt status register"]
    #[inline(always)]
    pub const fn spi_mem_int_st(&self) -> &SPI_MEM_INT_ST {
        &self.spi_mem_int_st
    }
    #[doc = "0xd4 - SPI0 flash DDR mode control register"]
    #[inline(always)]
    pub const fn spi_mem_ddr(&self) -> &SPI_MEM_DDR {
        &self.spi_mem_ddr
    }
    #[doc = "0xd8 - SPI0 external RAM DDR mode control register"]
    #[inline(always)]
    pub const fn spi_smem_ddr(&self) -> &SPI_SMEM_DDR {
        &self.spi_smem_ddr
    }
    #[doc = "0x100..0x110 - MSPI flash PMS section %s attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms_attr(&self, n: usize) -> &SPI_FMEM_PMS_ATTR {
        &self.spi_fmem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - MSPI flash PMS section %s attribute register"]
    #[inline(always)]
    pub fn spi_fmem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_FMEM_PMS_ATTR> {
        self.spi_fmem_pms_attr.iter()
    }
    #[doc = "0x100 - MSPI flash PMS section 0 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms0_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(0)
    }
    #[doc = "0x104 - MSPI flash PMS section 1 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms1_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(1)
    }
    #[doc = "0x108 - MSPI flash PMS section 2 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms2_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(2)
    }
    #[doc = "0x10c - MSPI flash PMS section 3 attribute register"]
    #[inline(always)]
    pub const fn spi_fmem_pms3_attr(&self) -> &SPI_FMEM_PMS_ATTR {
        self.spi_fmem_pms_attr(3)
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
    #[doc = "0x130..0x140 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms_attr(&self, n: usize) -> &SPI_SMEM_PMS_ATTR {
        &self.spi_smem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x140 - SPI1 flash PMS section %s start address register"]
    #[inline(always)]
    pub fn spi_smem_pms_attr_iter(&self) -> impl Iterator<Item = &SPI_SMEM_PMS_ATTR> {
        self.spi_smem_pms_attr.iter()
    }
    #[doc = "0x130 - SPI1 flash PMS section 0 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms0_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(0)
    }
    #[doc = "0x134 - SPI1 flash PMS section 1 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms1_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(1)
    }
    #[doc = "0x138 - SPI1 flash PMS section 2 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms2_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(2)
    }
    #[doc = "0x13c - SPI1 flash PMS section 3 start address register"]
    #[inline(always)]
    pub const fn spi_smem_pms3_attr(&self) -> &SPI_SMEM_PMS_ATTR {
        self.spi_smem_pms_attr(3)
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
    #[doc = "0x164 - SPI1 access reject register"]
    #[inline(always)]
    pub const fn spi_mem_pms_reject(&self) -> &SPI_MEM_PMS_REJECT {
        &self.spi_mem_pms_reject
    }
    #[doc = "0x168 - MSPI ECC control register"]
    #[inline(always)]
    pub const fn spi_mem_ecc_ctrl(&self) -> &SPI_MEM_ECC_CTRL {
        &self.spi_mem_ecc_ctrl
    }
    #[doc = "0x16c - MSPI ECC error address register"]
    #[inline(always)]
    pub const fn spi_mem_ecc_err_addr(&self) -> &SPI_MEM_ECC_ERR_ADDR {
        &self.spi_mem_ecc_err_addr
    }
    #[doc = "0x170 - SPI0 AXI request error address."]
    #[inline(always)]
    pub const fn spi_mem_axi_err_addr(&self) -> &SPI_MEM_AXI_ERR_ADDR {
        &self.spi_mem_axi_err_addr
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
    pub const fn spi_mem_axi_err_resp_en(&self) -> &SPI_MEM_AXI_ERR_RESP_EN {
        &self.spi_mem_axi_err_resp_en
    }
    #[doc = "0x180 - SPI0 flash timing calibration register"]
    #[inline(always)]
    pub const fn spi_mem_timing_cali(&self) -> &SPI_MEM_TIMING_CALI {
        &self.spi_mem_timing_cali
    }
    #[doc = "0x184 - MSPI flash input timing delay mode control register"]
    #[inline(always)]
    pub const fn spi_mem_din_mode(&self) -> &SPI_MEM_DIN_MODE {
        &self.spi_mem_din_mode
    }
    #[doc = "0x188 - MSPI flash input timing delay number control register"]
    #[inline(always)]
    pub const fn spi_mem_din_num(&self) -> &SPI_MEM_DIN_NUM {
        &self.spi_mem_din_num
    }
    #[doc = "0x18c - MSPI flash output timing adjustment control register"]
    #[inline(always)]
    pub const fn spi_mem_dout_mode(&self) -> &SPI_MEM_DOUT_MODE {
        &self.spi_mem_dout_mode
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
    #[doc = "0x1a4 - MSPI 16x external RAM input timing delay mode control register"]
    #[inline(always)]
    pub const fn spi_smem_din_hex_mode(&self) -> &SPI_SMEM_DIN_HEX_MODE {
        &self.spi_smem_din_hex_mode
    }
    #[doc = "0x1a8 - MSPI 16x external RAM input timing delay number control register"]
    #[inline(always)]
    pub const fn spi_smem_din_hex_num(&self) -> &SPI_SMEM_DIN_HEX_NUM {
        &self.spi_smem_din_hex_num
    }
    #[doc = "0x1ac - MSPI 16x external RAM output timing adjustment control register"]
    #[inline(always)]
    pub const fn spi_smem_dout_hex_mode(&self) -> &SPI_SMEM_DOUT_HEX_MODE {
        &self.spi_smem_dout_hex_mode
    }
    #[doc = "0x200 - SPI0 clock gate register"]
    #[inline(always)]
    pub const fn spi_mem_clock_gate(&self) -> &SPI_MEM_CLOCK_GATE {
        &self.spi_mem_clock_gate
    }
    #[doc = "0x300 - The base address of the memory that stores plaintext in Manual Encryption"]
    #[inline(always)]
    pub const fn spi_mem_xts_plain_base(&self) -> &SPI_MEM_XTS_PLAIN_BASE {
        &self.spi_mem_xts_plain_base
    }
    #[doc = "0x340 - Manual Encryption Line-Size register"]
    #[inline(always)]
    pub const fn spi_mem_xts_linesize(&self) -> &SPI_MEM_XTS_LINESIZE {
        &self.spi_mem_xts_linesize
    }
    #[doc = "0x344 - Manual Encryption destination register"]
    #[inline(always)]
    pub const fn spi_mem_xts_destination(&self) -> &SPI_MEM_XTS_DESTINATION {
        &self.spi_mem_xts_destination
    }
    #[doc = "0x348 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn spi_mem_xts_physical_address(&self) -> &SPI_MEM_XTS_PHYSICAL_ADDRESS {
        &self.spi_mem_xts_physical_address
    }
    #[doc = "0x34c - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn spi_mem_xts_trigger(&self) -> &SPI_MEM_XTS_TRIGGER {
        &self.spi_mem_xts_trigger
    }
    #[doc = "0x350 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn spi_mem_xts_release(&self) -> &SPI_MEM_XTS_RELEASE {
        &self.spi_mem_xts_release
    }
    #[doc = "0x354 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn spi_mem_xts_destroy(&self) -> &SPI_MEM_XTS_DESTROY {
        &self.spi_mem_xts_destroy
    }
    #[doc = "0x358 - Manual Encryption physical address register"]
    #[inline(always)]
    pub const fn spi_mem_xts_state(&self) -> &SPI_MEM_XTS_STATE {
        &self.spi_mem_xts_state
    }
    #[doc = "0x35c - Manual Encryption version register"]
    #[inline(always)]
    pub const fn spi_mem_xts_date(&self) -> &SPI_MEM_XTS_DATE {
        &self.spi_mem_xts_date
    }
    #[doc = "0x37c - MSPI-MMU item content register"]
    #[inline(always)]
    pub const fn spi_mem_mmu_item_content(&self) -> &SPI_MEM_MMU_ITEM_CONTENT {
        &self.spi_mem_mmu_item_content
    }
    #[doc = "0x380 - MSPI-MMU item index register"]
    #[inline(always)]
    pub const fn spi_mem_mmu_item_index(&self) -> &SPI_MEM_MMU_ITEM_INDEX {
        &self.spi_mem_mmu_item_index
    }
    #[doc = "0x384 - MSPI MMU power control register"]
    #[inline(always)]
    pub const fn spi_mem_mmu_power_ctrl(&self) -> &SPI_MEM_MMU_POWER_CTRL {
        &self.spi_mem_mmu_power_ctrl
    }
    #[doc = "0x388 - SPI memory cryption DPA register"]
    #[inline(always)]
    pub const fn spi_mem_dpa_ctrl(&self) -> &SPI_MEM_DPA_CTRL {
        &self.spi_mem_dpa_ctrl
    }
    #[doc = "0x3f0 - MSPI ECO high register"]
    #[inline(always)]
    pub const fn spi_mem_registerrnd_eco_high(&self) -> &SPI_MEM_REGISTERRND_ECO_HIGH {
        &self.spi_mem_registerrnd_eco_high
    }
    #[doc = "0x3f4 - MSPI ECO low register"]
    #[inline(always)]
    pub const fn spi_mem_registerrnd_eco_low(&self) -> &SPI_MEM_REGISTERRND_ECO_LOW {
        &self.spi_mem_registerrnd_eco_low
    }
    #[doc = "0x3fc - SPI0 version control register"]
    #[inline(always)]
    pub const fn spi_mem_date(&self) -> &SPI_MEM_DATE {
        &self.spi_mem_date
    }
}
#[doc = "SPI_MEM_CMD (r) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_cmd`] module"]
pub type SPI_MEM_CMD = crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_CTRL (rw) register accessor: SPI0 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl`] module"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 (rw) register accessor: SPI0 control1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl1`] module"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 (rw) register accessor: SPI0 control2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ctrl2`] module"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK (rw) register accessor: SPI clock division control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_clock`] module"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER (rw) register accessor: SPI0 user register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user`] module"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 (rw) register accessor: SPI0 user1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user1`] module"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 (rw) register accessor: SPI0 user2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_user2`] module"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_RD_STATUS (rw) register accessor: SPI0 read control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_rd_status`] module"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC (rw) register accessor: SPI0 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_misc`] module"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_CACHE_FCTRL (rw) register accessor: SPI0 bit mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_cache_fctrl`] module"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_CACHE_SCTRL (rw) register accessor: SPI0 external RAM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_cache_sctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_cache_sctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_cache_sctrl`] module"]
pub type SPI_MEM_CACHE_SCTRL = crate::Reg<spi_mem_cache_sctrl::SPI_MEM_CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod spi_mem_cache_sctrl;
#[doc = "SPI_MEM_SRAM_CMD (rw) register accessor: SPI0 external RAM mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sram_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sram_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_sram_cmd`] module"]
pub type SPI_MEM_SRAM_CMD = crate::Reg<spi_mem_sram_cmd::SPI_MEM_SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod spi_mem_sram_cmd;
#[doc = "SPI_MEM_SRAM_DRD_CMD (rw) register accessor: SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sram_drd_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sram_drd_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_sram_drd_cmd`] module"]
pub type SPI_MEM_SRAM_DRD_CMD = crate::Reg<spi_mem_sram_drd_cmd::SPI_MEM_SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod spi_mem_sram_drd_cmd;
#[doc = "SPI_MEM_SRAM_DWR_CMD (rw) register accessor: SPI0 external RAM DDR write command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sram_dwr_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sram_dwr_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_sram_dwr_cmd`] module"]
pub type SPI_MEM_SRAM_DWR_CMD = crate::Reg<spi_mem_sram_dwr_cmd::SPI_MEM_SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod spi_mem_sram_dwr_cmd;
#[doc = "SPI_MEM_SRAM_CLK (rw) register accessor: SPI0 external RAM clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_sram_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_sram_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_sram_clk`] module"]
pub type SPI_MEM_SRAM_CLK = crate::Reg<spi_mem_sram_clk::SPI_MEM_SRAM_CLK_SPEC>;
#[doc = "SPI0 external RAM clock control register"]
pub mod spi_mem_sram_clk;
#[doc = "SPI_MEM_FSM (rw) register accessor: SPI0 FSM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_fsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_fsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_fsm`] module"]
pub type SPI_MEM_FSM = crate::Reg<spi_mem_fsm::SPI_MEM_FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod spi_mem_fsm;
#[doc = "SPI_MEM_INT_ENA (rw) register accessor: SPI0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_ena`] module"]
pub type SPI_MEM_INT_ENA = crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>;
#[doc = "SPI0 interrupt enable register"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR (w) register accessor: SPI0 interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_clr`] module"]
pub type SPI_MEM_INT_CLR = crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>;
#[doc = "SPI0 interrupt clear register"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW (rw) register accessor: SPI0 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_raw`] module"]
pub type SPI_MEM_INT_RAW = crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>;
#[doc = "SPI0 interrupt raw register"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST (r) register accessor: SPI0 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_int_st`] module"]
pub type SPI_MEM_INT_ST = crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>;
#[doc = "SPI0 interrupt status register"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_DDR (rw) register accessor: SPI0 flash DDR mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ddr`] module"]
pub type SPI_MEM_DDR = crate::Reg<spi_mem_ddr::SPI_MEM_DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod spi_mem_ddr;
#[doc = "SPI_SMEM_DDR (rw) register accessor: SPI0 external RAM DDR mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ddr`] module"]
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod spi_smem_ddr;
#[doc = "SPI_FMEM_PMS_ATTR (rw) register accessor: MSPI flash PMS section %s attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_attr`] module"]
pub type SPI_FMEM_PMS_ATTR = crate::Reg<spi_fmem_pms_attr::SPI_FMEM_PMS_ATTR_SPEC>;
#[doc = "MSPI flash PMS section %s attribute register"]
pub mod spi_fmem_pms_attr;
#[doc = "SPI_FMEM_PMS_ADDR (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_addr`] module"]
pub type SPI_FMEM_PMS_ADDR = crate::Reg<spi_fmem_pms_addr::SPI_FMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_addr;
#[doc = "SPI_FMEM_PMS_SIZE (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fmem_pms_size`] module"]
pub type SPI_FMEM_PMS_SIZE = crate::Reg<spi_fmem_pms_size::SPI_FMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_fmem_pms_size;
#[doc = "SPI_SMEM_PMS_ATTR (rw) register accessor: SPI1 flash PMS section %s start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_attr`] module"]
pub type SPI_SMEM_PMS_ATTR = crate::Reg<spi_smem_pms_attr::SPI_SMEM_PMS_ATTR_SPEC>;
#[doc = "SPI1 flash PMS section %s start address register"]
pub mod spi_smem_pms_attr;
#[doc = "SPI_SMEM_PMS_ADDR (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_addr`] module"]
pub type SPI_SMEM_PMS_ADDR = crate::Reg<spi_smem_pms_addr::SPI_SMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_addr;
#[doc = "SPI_SMEM_PMS_SIZE (rw) register accessor: SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_pms_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_pms_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_pms_size`] module"]
pub type SPI_SMEM_PMS_SIZE = crate::Reg<spi_smem_pms_size::SPI_SMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 external RAM PMS section %s start address register"]
pub mod spi_smem_pms_size;
#[doc = "SPI_MEM_PMS_REJECT (rw) register accessor: SPI1 access reject register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_reject::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_reject::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_pms_reject`] module"]
pub type SPI_MEM_PMS_REJECT = crate::Reg<spi_mem_pms_reject::SPI_MEM_PMS_REJECT_SPEC>;
#[doc = "SPI1 access reject register"]
pub mod spi_mem_pms_reject;
#[doc = "SPI_MEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ecc_ctrl`] module"]
pub type SPI_MEM_ECC_CTRL = crate::Reg<spi_mem_ecc_ctrl::SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod spi_mem_ecc_ctrl;
#[doc = "SPI_MEM_ECC_ERR_ADDR (r) register accessor: MSPI ECC error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ecc_err_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_ecc_err_addr`] module"]
pub type SPI_MEM_ECC_ERR_ADDR = crate::Reg<spi_mem_ecc_err_addr::SPI_MEM_ECC_ERR_ADDR_SPEC>;
#[doc = "MSPI ECC error address register"]
pub mod spi_mem_ecc_err_addr;
#[doc = "SPI_MEM_AXI_ERR_ADDR (r) register accessor: SPI0 AXI request error address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_axi_err_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_axi_err_addr`] module"]
pub type SPI_MEM_AXI_ERR_ADDR = crate::Reg<spi_mem_axi_err_addr::SPI_MEM_AXI_ERR_ADDR_SPEC>;
#[doc = "SPI0 AXI request error address."]
pub mod spi_mem_axi_err_addr;
#[doc = "SPI_SMEM_ECC_CTRL (rw) register accessor: MSPI ECC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ecc_ctrl`] module"]
pub type SPI_SMEM_ECC_CTRL = crate::Reg<spi_smem_ecc_ctrl::SPI_SMEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod spi_smem_ecc_ctrl;
#[doc = "SPI_SMEM_AXI_ADDR_CTRL (r) register accessor: SPI0 AXI address control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_axi_addr_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_axi_addr_ctrl`] module"]
pub type SPI_SMEM_AXI_ADDR_CTRL = crate::Reg<spi_smem_axi_addr_ctrl::SPI_SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "SPI0 AXI address control register"]
pub mod spi_smem_axi_addr_ctrl;
#[doc = "SPI_MEM_AXI_ERR_RESP_EN (rw) register accessor: SPI0 AXI error response enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_axi_err_resp_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_axi_err_resp_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_axi_err_resp_en`] module"]
pub type SPI_MEM_AXI_ERR_RESP_EN =
    crate::Reg<spi_mem_axi_err_resp_en::SPI_MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "SPI0 AXI error response enable register"]
pub mod spi_mem_axi_err_resp_en;
#[doc = "SPI_MEM_TIMING_CALI (rw) register accessor: SPI0 flash timing calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_timing_cali`] module"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI0 flash timing calibration register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_DIN_MODE (rw) register accessor: MSPI flash input timing delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_din_mode`] module"]
pub type SPI_MEM_DIN_MODE = crate::Reg<spi_mem_din_mode::SPI_MEM_DIN_MODE_SPEC>;
#[doc = "MSPI flash input timing delay mode control register"]
pub mod spi_mem_din_mode;
#[doc = "SPI_MEM_DIN_NUM (rw) register accessor: MSPI flash input timing delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_din_num`] module"]
pub type SPI_MEM_DIN_NUM = crate::Reg<spi_mem_din_num::SPI_MEM_DIN_NUM_SPEC>;
#[doc = "MSPI flash input timing delay number control register"]
pub mod spi_mem_din_num;
#[doc = "SPI_MEM_DOUT_MODE (rw) register accessor: MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_dout_mode`] module"]
pub type SPI_MEM_DOUT_MODE = crate::Reg<spi_mem_dout_mode::SPI_MEM_DOUT_MODE_SPEC>;
#[doc = "MSPI flash output timing adjustment control register"]
pub mod spi_mem_dout_mode;
#[doc = "SPI_SMEM_TIMING_CALI (rw) register accessor: MSPI external RAM timing calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_timing_cali`] module"]
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "MSPI external RAM timing calibration register"]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (rw) register accessor: MSPI external RAM input timing delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_mode`] module"]
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "MSPI external RAM input timing delay mode control register"]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (rw) register accessor: MSPI external RAM input timing delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_num`] module"]
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
#[doc = "MSPI external RAM input timing delay number control register"]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (rw) register accessor: MSPI external RAM output timing adjustment control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_mode`] module"]
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
#[doc = "MSPI external RAM output timing adjustment control register"]
pub mod spi_smem_dout_mode;
#[doc = "SPI_SMEM_AC (rw) register accessor: MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_ac`] module"]
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod spi_smem_ac;
#[doc = "SPI_SMEM_DIN_HEX_MODE (rw) register accessor: MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_hex_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_hex_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_hex_mode`] module"]
pub type SPI_SMEM_DIN_HEX_MODE = crate::Reg<spi_smem_din_hex_mode::SPI_SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "MSPI 16x external RAM input timing delay mode control register"]
pub mod spi_smem_din_hex_mode;
#[doc = "SPI_SMEM_DIN_HEX_NUM (rw) register accessor: MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_din_hex_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_din_hex_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_din_hex_num`] module"]
pub type SPI_SMEM_DIN_HEX_NUM = crate::Reg<spi_smem_din_hex_num::SPI_SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "MSPI 16x external RAM input timing delay number control register"]
pub mod spi_smem_din_hex_num;
#[doc = "SPI_SMEM_DOUT_HEX_MODE (rw) register accessor: MSPI 16x external RAM output timing adjustment control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_dout_hex_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_dout_hex_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_smem_dout_hex_mode`] module"]
pub type SPI_SMEM_DOUT_HEX_MODE = crate::Reg<spi_smem_dout_hex_mode::SPI_SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = "MSPI 16x external RAM output timing adjustment control register"]
pub mod spi_smem_dout_hex_mode;
#[doc = "SPI_MEM_CLOCK_GATE (rw) register accessor: SPI0 clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_clock_gate`] module"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI0 clock gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_XTS_PLAIN_BASE (rw) register accessor: The base address of the memory that stores plaintext in Manual Encryption\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_plain_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_plain_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_plain_base`] module"]
pub type SPI_MEM_XTS_PLAIN_BASE = crate::Reg<spi_mem_xts_plain_base::SPI_MEM_XTS_PLAIN_BASE_SPEC>;
#[doc = "The base address of the memory that stores plaintext in Manual Encryption"]
pub mod spi_mem_xts_plain_base;
#[doc = "SPI_MEM_XTS_LINESIZE (rw) register accessor: Manual Encryption Line-Size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_linesize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_linesize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_linesize`] module"]
pub type SPI_MEM_XTS_LINESIZE = crate::Reg<spi_mem_xts_linesize::SPI_MEM_XTS_LINESIZE_SPEC>;
#[doc = "Manual Encryption Line-Size register"]
pub mod spi_mem_xts_linesize;
#[doc = "SPI_MEM_XTS_DESTINATION (rw) register accessor: Manual Encryption destination register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_destination`] module"]
pub type SPI_MEM_XTS_DESTINATION =
    crate::Reg<spi_mem_xts_destination::SPI_MEM_XTS_DESTINATION_SPEC>;
#[doc = "Manual Encryption destination register"]
pub mod spi_mem_xts_destination;
#[doc = "SPI_MEM_XTS_PHYSICAL_ADDRESS (rw) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_physical_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_physical_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_physical_address`] module"]
pub type SPI_MEM_XTS_PHYSICAL_ADDRESS =
    crate::Reg<spi_mem_xts_physical_address::SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_physical_address;
#[doc = "SPI_MEM_XTS_TRIGGER (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_trigger`] module"]
pub type SPI_MEM_XTS_TRIGGER = crate::Reg<spi_mem_xts_trigger::SPI_MEM_XTS_TRIGGER_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_trigger;
#[doc = "SPI_MEM_XTS_RELEASE (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_release`] module"]
pub type SPI_MEM_XTS_RELEASE = crate::Reg<spi_mem_xts_release::SPI_MEM_XTS_RELEASE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_release;
#[doc = "SPI_MEM_XTS_DESTROY (w) register accessor: Manual Encryption physical address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_destroy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_destroy`] module"]
pub type SPI_MEM_XTS_DESTROY = crate::Reg<spi_mem_xts_destroy::SPI_MEM_XTS_DESTROY_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_destroy;
#[doc = "SPI_MEM_XTS_STATE (r) register accessor: Manual Encryption physical address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_state`] module"]
pub type SPI_MEM_XTS_STATE = crate::Reg<spi_mem_xts_state::SPI_MEM_XTS_STATE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_state;
#[doc = "SPI_MEM_XTS_DATE (rw) register accessor: Manual Encryption version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_xts_date`] module"]
pub type SPI_MEM_XTS_DATE = crate::Reg<spi_mem_xts_date::SPI_MEM_XTS_DATE_SPEC>;
#[doc = "Manual Encryption version register"]
pub mod spi_mem_xts_date;
#[doc = "SPI_MEM_MMU_ITEM_CONTENT (rw) register accessor: MSPI-MMU item content register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mmu_item_content::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mmu_item_content::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_mmu_item_content`] module"]
pub type SPI_MEM_MMU_ITEM_CONTENT =
    crate::Reg<spi_mem_mmu_item_content::SPI_MEM_MMU_ITEM_CONTENT_SPEC>;
#[doc = "MSPI-MMU item content register"]
pub mod spi_mem_mmu_item_content;
#[doc = "SPI_MEM_MMU_ITEM_INDEX (rw) register accessor: MSPI-MMU item index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mmu_item_index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mmu_item_index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_mmu_item_index`] module"]
pub type SPI_MEM_MMU_ITEM_INDEX = crate::Reg<spi_mem_mmu_item_index::SPI_MEM_MMU_ITEM_INDEX_SPEC>;
#[doc = "MSPI-MMU item index register"]
pub mod spi_mem_mmu_item_index;
#[doc = "SPI_MEM_MMU_POWER_CTRL (rw) register accessor: MSPI MMU power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mmu_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mmu_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_mmu_power_ctrl`] module"]
pub type SPI_MEM_MMU_POWER_CTRL = crate::Reg<spi_mem_mmu_power_ctrl::SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "MSPI MMU power control register"]
pub mod spi_mem_mmu_power_ctrl;
#[doc = "SPI_MEM_DPA_CTRL (rw) register accessor: SPI memory cryption DPA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_dpa_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_dpa_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_dpa_ctrl`] module"]
pub type SPI_MEM_DPA_CTRL = crate::Reg<spi_mem_dpa_ctrl::SPI_MEM_DPA_CTRL_SPEC>;
#[doc = "SPI memory cryption DPA register"]
pub mod spi_mem_dpa_ctrl;
#[doc = "SPI_MEM_REGISTERRND_ECO_HIGH (rw) register accessor: MSPI ECO high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_registerrnd_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_registerrnd_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_registerrnd_eco_high`] module"]
pub type SPI_MEM_REGISTERRND_ECO_HIGH =
    crate::Reg<spi_mem_registerrnd_eco_high::SPI_MEM_REGISTERRND_ECO_HIGH_SPEC>;
#[doc = "MSPI ECO high register"]
pub mod spi_mem_registerrnd_eco_high;
#[doc = "SPI_MEM_REGISTERRND_ECO_LOW (rw) register accessor: MSPI ECO low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_registerrnd_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_registerrnd_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_registerrnd_eco_low`] module"]
pub type SPI_MEM_REGISTERRND_ECO_LOW =
    crate::Reg<spi_mem_registerrnd_eco_low::SPI_MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "MSPI ECO low register"]
pub mod spi_mem_registerrnd_eco_low;
#[doc = "SPI_MEM_DATE (rw) register accessor: SPI0 version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_date`] module"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "SPI0 version control register"]
pub mod spi_mem_date;
