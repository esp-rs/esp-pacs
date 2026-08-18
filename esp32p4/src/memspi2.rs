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
    dll_dly_db: DLL_DLY_DB,
    dll_db_st0: DLL_DB_ST0,
    dll_db_st1: DLL_DB_ST1,
    _reserved26: [u8; 0x18],
    fmem_pms_attr: [FMEM_PMS_ATTR; 4],
    fmem_pms_addr: [FMEM_PMS_ADDR; 4],
    fmem_pms_size: [FMEM_PMS_SIZE; 4],
    smem_pms_attr: [SMEM_PMS_ATTR; 4],
    smem_pms_addr: [SMEM_PMS_ADDR; 4],
    smem_pms_size: [SMEM_PMS_SIZE; 4],
    _reserved32: [u8; 0x04],
    pms_reject: PMS_REJECT,
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
    _reserved51: [u8; 0x50],
    clock_gate: CLOCK_GATE,
    _reserved52: [u8; 0xfc],
    xts_plain_base: XTS_PLAIN_BASE,
    _reserved53: [u8; 0x3c],
    xts_linesize: XTS_LINESIZE,
    xts_destination: XTS_DESTINATION,
    xts_physical_address: XTS_PHYSICAL_ADDRESS,
    xts_trigger: XTS_TRIGGER,
    xts_release: XTS_RELEASE,
    xts_destroy: XTS_DESTROY,
    xts_state: XTS_STATE,
    xts_date: XTS_DATE,
    _reserved61: [u8; 0x1c],
    mmu_item_content: MMU_ITEM_CONTENT,
    mmu_item_index: MMU_ITEM_INDEX,
    mmu_power_ctrl: MMU_POWER_CTRL,
    dpa_ctrl: DPA_CTRL,
    xts_pseudo_round_conf: XTS_PSEUDO_ROUND_CONF,
    _reserved66: [u8; 0x60],
    registerrnd_eco_high: REGISTERRND_ECO_HIGH,
    registerrnd_eco_low: REGISTERRND_ECO_LOW,
    _reserved68: [u8; 0x04],
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
    pub const fn smem_ddr(&self) -> &SMEM_DDR {
        &self.smem_ddr
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
    #[doc = "0x100..0x110 - "]
    #[inline(always)]
    pub const fn fmem_pms_attr(&self, n: usize) -> &FMEM_PMS_ATTR {
        &self.fmem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - "]
    #[inline(always)]
    pub fn fmem_pms_attr_iter(&self) -> impl Iterator<Item = &FMEM_PMS_ATTR> {
        self.fmem_pms_attr.iter()
    }
    #[doc = "0x100 - FMEM_PMS0_ATTR"]
    #[inline(always)]
    pub const fn fmem_pms0_attr(&self) -> &FMEM_PMS_ATTR {
        self.fmem_pms_attr(0)
    }
    #[doc = "0x104 - FMEM_PMS1_ATTR"]
    #[inline(always)]
    pub const fn fmem_pms1_attr(&self) -> &FMEM_PMS_ATTR {
        self.fmem_pms_attr(1)
    }
    #[doc = "0x108 - FMEM_PMS2_ATTR"]
    #[inline(always)]
    pub const fn fmem_pms2_attr(&self) -> &FMEM_PMS_ATTR {
        self.fmem_pms_attr(2)
    }
    #[doc = "0x10c - FMEM_PMS3_ATTR"]
    #[inline(always)]
    pub const fn fmem_pms3_attr(&self) -> &FMEM_PMS_ATTR {
        self.fmem_pms_attr(3)
    }
    #[doc = "0x110..0x120 - "]
    #[inline(always)]
    pub const fn fmem_pms_addr(&self, n: usize) -> &FMEM_PMS_ADDR {
        &self.fmem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - "]
    #[inline(always)]
    pub fn fmem_pms_addr_iter(&self) -> impl Iterator<Item = &FMEM_PMS_ADDR> {
        self.fmem_pms_addr.iter()
    }
    #[doc = "0x110 - FMEM_PMS0_ADDR"]
    #[inline(always)]
    pub const fn fmem_pms0_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(0)
    }
    #[doc = "0x114 - FMEM_PMS1_ADDR"]
    #[inline(always)]
    pub const fn fmem_pms1_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(1)
    }
    #[doc = "0x118 - FMEM_PMS2_ADDR"]
    #[inline(always)]
    pub const fn fmem_pms2_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(2)
    }
    #[doc = "0x11c - FMEM_PMS3_ADDR"]
    #[inline(always)]
    pub const fn fmem_pms3_addr(&self) -> &FMEM_PMS_ADDR {
        self.fmem_pms_addr(3)
    }
    #[doc = "0x120..0x130 - "]
    #[inline(always)]
    pub const fn fmem_pms_size(&self, n: usize) -> &FMEM_PMS_SIZE {
        &self.fmem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x130 - "]
    #[inline(always)]
    pub fn fmem_pms_size_iter(&self) -> impl Iterator<Item = &FMEM_PMS_SIZE> {
        self.fmem_pms_size.iter()
    }
    #[doc = "0x120 - FMEM_PMS0_SIZE"]
    #[inline(always)]
    pub const fn fmem_pms0_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(0)
    }
    #[doc = "0x124 - FMEM_PMS1_SIZE"]
    #[inline(always)]
    pub const fn fmem_pms1_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(1)
    }
    #[doc = "0x128 - FMEM_PMS2_SIZE"]
    #[inline(always)]
    pub const fn fmem_pms2_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(2)
    }
    #[doc = "0x12c - FMEM_PMS3_SIZE"]
    #[inline(always)]
    pub const fn fmem_pms3_size(&self) -> &FMEM_PMS_SIZE {
        self.fmem_pms_size(3)
    }
    #[doc = "0x130..0x140 - "]
    #[inline(always)]
    pub const fn smem_pms_attr(&self, n: usize) -> &SMEM_PMS_ATTR {
        &self.smem_pms_attr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x140 - "]
    #[inline(always)]
    pub fn smem_pms_attr_iter(&self) -> impl Iterator<Item = &SMEM_PMS_ATTR> {
        self.smem_pms_attr.iter()
    }
    #[doc = "0x130 - SMEM_PMS0_ATTR"]
    #[inline(always)]
    pub const fn smem_pms0_attr(&self) -> &SMEM_PMS_ATTR {
        self.smem_pms_attr(0)
    }
    #[doc = "0x134 - SMEM_PMS1_ATTR"]
    #[inline(always)]
    pub const fn smem_pms1_attr(&self) -> &SMEM_PMS_ATTR {
        self.smem_pms_attr(1)
    }
    #[doc = "0x138 - SMEM_PMS2_ATTR"]
    #[inline(always)]
    pub const fn smem_pms2_attr(&self) -> &SMEM_PMS_ATTR {
        self.smem_pms_attr(2)
    }
    #[doc = "0x13c - SMEM_PMS3_ATTR"]
    #[inline(always)]
    pub const fn smem_pms3_attr(&self) -> &SMEM_PMS_ATTR {
        self.smem_pms_attr(3)
    }
    #[doc = "0x140..0x150 - "]
    #[inline(always)]
    pub const fn smem_pms_addr(&self, n: usize) -> &SMEM_PMS_ADDR {
        &self.smem_pms_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - "]
    #[inline(always)]
    pub fn smem_pms_addr_iter(&self) -> impl Iterator<Item = &SMEM_PMS_ADDR> {
        self.smem_pms_addr.iter()
    }
    #[doc = "0x140 - SMEM_PMS0_ADDR"]
    #[inline(always)]
    pub const fn smem_pms0_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(0)
    }
    #[doc = "0x144 - SMEM_PMS1_ADDR"]
    #[inline(always)]
    pub const fn smem_pms1_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(1)
    }
    #[doc = "0x148 - SMEM_PMS2_ADDR"]
    #[inline(always)]
    pub const fn smem_pms2_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(2)
    }
    #[doc = "0x14c - SMEM_PMS3_ADDR"]
    #[inline(always)]
    pub const fn smem_pms3_addr(&self) -> &SMEM_PMS_ADDR {
        self.smem_pms_addr(3)
    }
    #[doc = "0x150..0x160 - "]
    #[inline(always)]
    pub const fn smem_pms_size(&self, n: usize) -> &SMEM_PMS_SIZE {
        &self.smem_pms_size[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - "]
    #[inline(always)]
    pub fn smem_pms_size_iter(&self) -> impl Iterator<Item = &SMEM_PMS_SIZE> {
        self.smem_pms_size.iter()
    }
    #[doc = "0x150 - SMEM_PMS0_SIZE"]
    #[inline(always)]
    pub const fn smem_pms0_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(0)
    }
    #[doc = "0x154 - SMEM_PMS1_SIZE"]
    #[inline(always)]
    pub const fn smem_pms1_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(1)
    }
    #[doc = "0x158 - SMEM_PMS2_SIZE"]
    #[inline(always)]
    pub const fn smem_pms2_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(2)
    }
    #[doc = "0x15c - SMEM_PMS3_SIZE"]
    #[inline(always)]
    pub const fn smem_pms3_size(&self) -> &SMEM_PMS_SIZE {
        self.smem_pms_size(3)
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn pms_reject(&self) -> &PMS_REJECT {
        &self.pms_reject
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
    pub const fn smem_ecc_ctrl(&self) -> &SMEM_ECC_CTRL {
        &self.smem_ecc_ctrl
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn smem_axi_addr_ctrl(&self) -> &SMEM_AXI_ADDR_CTRL {
        &self.smem_axi_addr_ctrl
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
    pub const fn smem_timing_cali(&self) -> &SMEM_TIMING_CALI {
        &self.smem_timing_cali
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn smem_din_mode(&self) -> &SMEM_DIN_MODE {
        &self.smem_din_mode
    }
    #[doc = "0x198 - "]
    #[inline(always)]
    pub const fn smem_din_num(&self) -> &SMEM_DIN_NUM {
        &self.smem_din_num
    }
    #[doc = "0x19c - "]
    #[inline(always)]
    pub const fn smem_dout_mode(&self) -> &SMEM_DOUT_MODE {
        &self.smem_dout_mode
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn smem_ac(&self) -> &SMEM_AC {
        &self.smem_ac
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn smem_din_hex_mode(&self) -> &SMEM_DIN_HEX_MODE {
        &self.smem_din_hex_mode
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn smem_din_hex_num(&self) -> &SMEM_DIN_HEX_NUM {
        &self.smem_din_hex_num
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn smem_dout_hex_mode(&self) -> &SMEM_DOUT_HEX_MODE {
        &self.smem_dout_hex_mode
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
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
#[doc = "SMEM_DDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ddr`] module"]
pub type SMEM_DDR = crate::Reg<smem_ddr::SMEM_DDR_SPEC>;
#[doc = ""]
pub mod smem_ddr;
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
#[doc = "FMEM_PMS_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms_attr`] module"]
pub type FMEM_PMS_ATTR = crate::Reg<fmem_pms_attr::FMEM_PMS_ATTR_SPEC>;
#[doc = ""]
pub mod fmem_pms_attr;
#[doc = "FMEM_PMS_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms_addr`] module"]
pub type FMEM_PMS_ADDR = crate::Reg<fmem_pms_addr::FMEM_PMS_ADDR_SPEC>;
#[doc = ""]
pub mod fmem_pms_addr;
#[doc = "FMEM_PMS_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmem_pms_size`] module"]
pub type FMEM_PMS_SIZE = crate::Reg<fmem_pms_size::FMEM_PMS_SIZE_SPEC>;
#[doc = ""]
pub mod fmem_pms_size;
#[doc = "SMEM_PMS_ATTR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_attr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_attr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms_attr`] module"]
pub type SMEM_PMS_ATTR = crate::Reg<smem_pms_attr::SMEM_PMS_ATTR_SPEC>;
#[doc = ""]
pub mod smem_pms_attr;
#[doc = "SMEM_PMS_ADDR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms_addr`] module"]
pub type SMEM_PMS_ADDR = crate::Reg<smem_pms_addr::SMEM_PMS_ADDR_SPEC>;
#[doc = ""]
pub mod smem_pms_addr;
#[doc = "SMEM_PMS_SIZE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_pms_size`] module"]
pub type SMEM_PMS_SIZE = crate::Reg<smem_pms_size::SMEM_PMS_SIZE_SPEC>;
#[doc = ""]
pub mod smem_pms_size;
#[doc = "PMS_REJECT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pms_reject::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pms_reject::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pms_reject`] module"]
pub type PMS_REJECT = crate::Reg<pms_reject::PMS_REJECT_SPEC>;
#[doc = ""]
pub mod pms_reject;
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
#[doc = "SMEM_ECC_CTRL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ecc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ecc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ecc_ctrl`] module"]
pub type SMEM_ECC_CTRL = crate::Reg<smem_ecc_ctrl::SMEM_ECC_CTRL_SPEC>;
#[doc = ""]
pub mod smem_ecc_ctrl;
#[doc = "SMEM_AXI_ADDR_CTRL (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_axi_addr_ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_axi_addr_ctrl`] module"]
pub type SMEM_AXI_ADDR_CTRL = crate::Reg<smem_axi_addr_ctrl::SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = ""]
pub mod smem_axi_addr_ctrl;
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
#[doc = "SMEM_TIMING_CALI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_timing_cali::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_timing_cali::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_timing_cali`] module"]
pub type SMEM_TIMING_CALI = crate::Reg<smem_timing_cali::SMEM_TIMING_CALI_SPEC>;
#[doc = ""]
pub mod smem_timing_cali;
#[doc = "SMEM_DIN_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_mode`] module"]
pub type SMEM_DIN_MODE = crate::Reg<smem_din_mode::SMEM_DIN_MODE_SPEC>;
#[doc = ""]
pub mod smem_din_mode;
#[doc = "SMEM_DIN_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_num`] module"]
pub type SMEM_DIN_NUM = crate::Reg<smem_din_num::SMEM_DIN_NUM_SPEC>;
#[doc = ""]
pub mod smem_din_num;
#[doc = "SMEM_DOUT_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_dout_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_dout_mode`] module"]
pub type SMEM_DOUT_MODE = crate::Reg<smem_dout_mode::SMEM_DOUT_MODE_SPEC>;
#[doc = ""]
pub mod smem_dout_mode;
#[doc = "SMEM_AC (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_ac`] module"]
pub type SMEM_AC = crate::Reg<smem_ac::SMEM_AC_SPEC>;
#[doc = ""]
pub mod smem_ac;
#[doc = "SMEM_DIN_HEX_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_hex_mode`] module"]
pub type SMEM_DIN_HEX_MODE = crate::Reg<smem_din_hex_mode::SMEM_DIN_HEX_MODE_SPEC>;
#[doc = ""]
pub mod smem_din_hex_mode;
#[doc = "SMEM_DIN_HEX_NUM (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_hex_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_din_hex_num`] module"]
pub type SMEM_DIN_HEX_NUM = crate::Reg<smem_din_hex_num::SMEM_DIN_HEX_NUM_SPEC>;
#[doc = ""]
pub mod smem_din_hex_num;
#[doc = "SMEM_DOUT_HEX_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_hex_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_dout_hex_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smem_dout_hex_mode`] module"]
pub type SMEM_DOUT_HEX_MODE = crate::Reg<smem_dout_hex_mode::SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = ""]
pub mod smem_dout_hex_mode;
#[doc = "CLOCK_GATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = ""]
pub mod clock_gate;
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
