#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI0 FSM status register"]
    pub spi_mem_cmd: SPI_MEM_CMD,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - SPI0 control register."]
    pub spi_mem_ctrl: SPI_MEM_CTRL,
    #[doc = "0x0c - SPI0 control1 register."]
    pub spi_mem_ctrl1: SPI_MEM_CTRL1,
    #[doc = "0x10 - SPI0 control2 register."]
    pub spi_mem_ctrl2: SPI_MEM_CTRL2,
    #[doc = "0x14 - SPI clock division control register."]
    pub spi_mem_clock: SPI_MEM_CLOCK,
    #[doc = "0x18 - SPI0 user register."]
    pub spi_mem_user: SPI_MEM_USER,
    #[doc = "0x1c - SPI0 user1 register."]
    pub spi_mem_user1: SPI_MEM_USER1,
    #[doc = "0x20 - SPI0 user2 register."]
    pub spi_mem_user2: SPI_MEM_USER2,
    _reserved8: [u8; 0x08],
    #[doc = "0x2c - SPI0 read control register."]
    pub spi_mem_rd_status: SPI_MEM_RD_STATUS,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - SPI0 misc register"]
    pub spi_mem_misc: SPI_MEM_MISC,
    _reserved10: [u8; 0x04],
    #[doc = "0x3c - SPI0 bit mode control register."]
    pub spi_mem_cache_fctrl: SPI_MEM_CACHE_FCTRL,
    #[doc = "0x40 - SPI0 external RAM control register"]
    pub spi_mem_cache_sctrl: SPI_MEM_CACHE_SCTRL,
    #[doc = "0x44 - SPI0 external RAM mode control register"]
    pub spi_mem_sram_cmd: SPI_MEM_SRAM_CMD,
    #[doc = "0x48 - SPI0 external RAM DDR read command control register"]
    pub spi_mem_sram_drd_cmd: SPI_MEM_SRAM_DRD_CMD,
    #[doc = "0x4c - SPI0 external RAM DDR write command control register"]
    pub spi_mem_sram_dwr_cmd: SPI_MEM_SRAM_DWR_CMD,
    #[doc = "0x50 - SPI0 external RAM clock control register"]
    pub spi_mem_sram_clk: SPI_MEM_SRAM_CLK,
    #[doc = "0x54 - SPI0 FSM status register"]
    pub spi_mem_fsm: SPI_MEM_FSM,
    _reserved17: [u8; 0x68],
    #[doc = "0xc0 - SPI0 interrupt enable register"]
    pub spi_mem_int_ena: SPI_MEM_INT_ENA,
    #[doc = "0xc4 - SPI0 interrupt clear register"]
    pub spi_mem_int_clr: SPI_MEM_INT_CLR,
    #[doc = "0xc8 - SPI0 interrupt raw register"]
    pub spi_mem_int_raw: SPI_MEM_INT_RAW,
    #[doc = "0xcc - SPI0 interrupt status register"]
    pub spi_mem_int_st: SPI_MEM_INT_ST,
    _reserved21: [u8; 0x04],
    #[doc = "0xd4 - SPI0 flash DDR mode control register"]
    pub spi_mem_ddr: SPI_MEM_DDR,
    #[doc = "0xd8 - SPI0 external RAM DDR mode control register"]
    pub spi_smem_ddr: SPI_SMEM_DDR,
    _reserved23: [u8; 0x24],
    #[doc = "0x100..0x110 - MSPI flash ACE section %s attribute register"]
    pub spi_fmem_pms_attr: [SPI_FMEM_PMS_ATTR; 4],
    #[doc = "0x110..0x120 - SPI1 flash ACE section %s start address register"]
    pub spi_fmem_pms_addr: [SPI_FMEM_PMS_ADDR; 4],
    #[doc = "0x120..0x130 - SPI1 flash ACE section %s start address register"]
    pub spi_fmem_pms_size: [SPI_FMEM_PMS_SIZE; 4],
    #[doc = "0x130..0x140 - SPI1 flash ACE section %s start address register"]
    pub spi_smem_pms_attr: [SPI_SMEM_PMS_ATTR; 4],
    #[doc = "0x140..0x150 - SPI1 external RAM ACE section %s start address register"]
    pub spi_smem_pms_addr: [SPI_SMEM_PMS_ADDR; 4],
    #[doc = "0x150..0x160 - SPI1 external RAM ACE section %s start address register"]
    pub spi_smem_pms_size: [SPI_SMEM_PMS_SIZE; 4],
    _reserved29: [u8; 0x04],
    #[doc = "0x164 - SPI1 access reject register"]
    pub spi_mem_pms_reject: SPI_MEM_PMS_REJECT,
    #[doc = "0x168 - MSPI ECC control register"]
    pub spi_mem_ecc_ctrl: SPI_MEM_ECC_CTRL,
    #[doc = "0x16c - MSPI ECC error address register"]
    pub spi_mem_ecc_err_addr: SPI_MEM_ECC_ERR_ADDR,
    #[doc = "0x170 - SPI0 AXI request error address."]
    pub spi_mem_axi_err_addr: SPI_MEM_AXI_ERR_ADDR,
    #[doc = "0x174 - MSPI ECC control register"]
    pub spi_smem_ecc_ctrl: SPI_SMEM_ECC_CTRL,
    _reserved34: [u8; 0x08],
    #[doc = "0x180 - SPI0 flash timing calibration register"]
    pub spi_mem_timing_cali: SPI_MEM_TIMING_CALI,
    #[doc = "0x184 - MSPI flash input timing delay mode control register"]
    pub spi_mem_din_mode: SPI_MEM_DIN_MODE,
    #[doc = "0x188 - MSPI flash input timing delay number control register"]
    pub spi_mem_din_num: SPI_MEM_DIN_NUM,
    #[doc = "0x18c - MSPI flash output timing adjustment control register"]
    pub spi_mem_dout_mode: SPI_MEM_DOUT_MODE,
    #[doc = "0x190 - MSPI external RAM timing calibration register"]
    pub spi_smem_timing_cali: SPI_SMEM_TIMING_CALI,
    #[doc = "0x194 - MSPI external RAM input timing delay mode control register"]
    pub spi_smem_din_mode: SPI_SMEM_DIN_MODE,
    #[doc = "0x198 - MSPI external RAM input timing delay number control register"]
    pub spi_smem_din_num: SPI_SMEM_DIN_NUM,
    #[doc = "0x19c - MSPI external RAM output timing adjustment control register"]
    pub spi_smem_dout_mode: SPI_SMEM_DOUT_MODE,
    #[doc = "0x1a0 - MSPI external RAM ECC and SPI CS timing control register"]
    pub spi_smem_ac: SPI_SMEM_AC,
    _reserved43: [u8; 0x5c],
    #[doc = "0x200 - SPI0 clock gate register"]
    pub spi_mem_clock_gate: SPI_MEM_CLOCK_GATE,
    _reserved44: [u8; 0xfc],
    #[doc = "0x300 - The base address of the memory that stores plaintext in Manual Encryption"]
    pub spi_mem_xts_plain_base: SPI_MEM_XTS_PLAIN_BASE,
    _reserved45: [u8; 0x3c],
    #[doc = "0x340 - Manual Encryption Line-Size register"]
    pub spi_mem_xts_linesize: SPI_MEM_XTS_LINESIZE,
    #[doc = "0x344 - Manual Encryption destination register"]
    pub spi_mem_xts_destination: SPI_MEM_XTS_DESTINATION,
    #[doc = "0x348 - Manual Encryption physical address register"]
    pub spi_mem_xts_physical_address: SPI_MEM_XTS_PHYSICAL_ADDRESS,
    #[doc = "0x34c - Manual Encryption physical address register"]
    pub spi_mem_xts_trigger: SPI_MEM_XTS_TRIGGER,
    #[doc = "0x350 - Manual Encryption physical address register"]
    pub spi_mem_xts_release: SPI_MEM_XTS_RELEASE,
    #[doc = "0x354 - Manual Encryption physical address register"]
    pub spi_mem_xts_destroy: SPI_MEM_XTS_DESTROY,
    #[doc = "0x358 - Manual Encryption physical address register"]
    pub spi_mem_xts_state: SPI_MEM_XTS_STATE,
    #[doc = "0x35c - Manual Encryption version register"]
    pub spi_mem_xts_date: SPI_MEM_XTS_DATE,
    _reserved53: [u8; 0x1c],
    #[doc = "0x37c - MSPI-MMU item content register"]
    pub spi_mem_mmu_item_content: SPI_MEM_MMU_ITEM_CONTENT,
    #[doc = "0x380 - MSPI-MMU item index register"]
    pub spi_mem_mmu_item_index: SPI_MEM_MMU_ITEM_INDEX,
    #[doc = "0x384 - MSPI MMU power control register"]
    pub spi_mem_mmu_power_ctrl: SPI_MEM_MMU_POWER_CTRL,
    #[doc = "0x388 - SPI memory cryption DPA register"]
    pub spi_mem_dpa_ctrl: SPI_MEM_DPA_CTRL,
    _reserved57: [u8; 0x64],
    #[doc = "0x3f0 - MSPI ECO high register"]
    pub spi_mem_registerrnd_eco_high: SPI_MEM_REGISTERRND_ECO_HIGH,
    #[doc = "0x3f4 - MSPI ECO low register"]
    pub spi_mem_registerrnd_eco_low: SPI_MEM_REGISTERRND_ECO_LOW,
    _reserved59: [u8; 0x04],
    #[doc = "0x3fc - SPI0 version control register"]
    pub spi_mem_date: SPI_MEM_DATE,
}
#[doc = "SPI_MEM_CMD (r) register accessor: an alias for `Reg<SPI_MEM_CMD_SPEC>`"]
pub type SPI_MEM_CMD = crate::Reg<spi_mem_cmd::SPI_MEM_CMD_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod spi_mem_cmd;
#[doc = "SPI_MEM_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_CTRL_SPEC>`"]
pub type SPI_MEM_CTRL = crate::Reg<spi_mem_ctrl::SPI_MEM_CTRL_SPEC>;
#[doc = "SPI0 control register."]
pub mod spi_mem_ctrl;
#[doc = "SPI_MEM_CTRL1 (rw) register accessor: an alias for `Reg<SPI_MEM_CTRL1_SPEC>`"]
pub type SPI_MEM_CTRL1 = crate::Reg<spi_mem_ctrl1::SPI_MEM_CTRL1_SPEC>;
#[doc = "SPI0 control1 register."]
pub mod spi_mem_ctrl1;
#[doc = "SPI_MEM_CTRL2 (rw) register accessor: an alias for `Reg<SPI_MEM_CTRL2_SPEC>`"]
pub type SPI_MEM_CTRL2 = crate::Reg<spi_mem_ctrl2::SPI_MEM_CTRL2_SPEC>;
#[doc = "SPI0 control2 register."]
pub mod spi_mem_ctrl2;
#[doc = "SPI_MEM_CLOCK (rw) register accessor: an alias for `Reg<SPI_MEM_CLOCK_SPEC>`"]
pub type SPI_MEM_CLOCK = crate::Reg<spi_mem_clock::SPI_MEM_CLOCK_SPEC>;
#[doc = "SPI clock division control register."]
pub mod spi_mem_clock;
#[doc = "SPI_MEM_USER (rw) register accessor: an alias for `Reg<SPI_MEM_USER_SPEC>`"]
pub type SPI_MEM_USER = crate::Reg<spi_mem_user::SPI_MEM_USER_SPEC>;
#[doc = "SPI0 user register."]
pub mod spi_mem_user;
#[doc = "SPI_MEM_USER1 (rw) register accessor: an alias for `Reg<SPI_MEM_USER1_SPEC>`"]
pub type SPI_MEM_USER1 = crate::Reg<spi_mem_user1::SPI_MEM_USER1_SPEC>;
#[doc = "SPI0 user1 register."]
pub mod spi_mem_user1;
#[doc = "SPI_MEM_USER2 (rw) register accessor: an alias for `Reg<SPI_MEM_USER2_SPEC>`"]
pub type SPI_MEM_USER2 = crate::Reg<spi_mem_user2::SPI_MEM_USER2_SPEC>;
#[doc = "SPI0 user2 register."]
pub mod spi_mem_user2;
#[doc = "SPI_MEM_RD_STATUS (rw) register accessor: an alias for `Reg<SPI_MEM_RD_STATUS_SPEC>`"]
pub type SPI_MEM_RD_STATUS = crate::Reg<spi_mem_rd_status::SPI_MEM_RD_STATUS_SPEC>;
#[doc = "SPI0 read control register."]
pub mod spi_mem_rd_status;
#[doc = "SPI_MEM_MISC (rw) register accessor: an alias for `Reg<SPI_MEM_MISC_SPEC>`"]
pub type SPI_MEM_MISC = crate::Reg<spi_mem_misc::SPI_MEM_MISC_SPEC>;
#[doc = "SPI0 misc register"]
pub mod spi_mem_misc;
#[doc = "SPI_MEM_CACHE_FCTRL (rw) register accessor: an alias for `Reg<SPI_MEM_CACHE_FCTRL_SPEC>`"]
pub type SPI_MEM_CACHE_FCTRL = crate::Reg<spi_mem_cache_fctrl::SPI_MEM_CACHE_FCTRL_SPEC>;
#[doc = "SPI0 bit mode control register."]
pub mod spi_mem_cache_fctrl;
#[doc = "SPI_MEM_CACHE_SCTRL (r) register accessor: an alias for `Reg<SPI_MEM_CACHE_SCTRL_SPEC>`"]
pub type SPI_MEM_CACHE_SCTRL = crate::Reg<spi_mem_cache_sctrl::SPI_MEM_CACHE_SCTRL_SPEC>;
#[doc = "SPI0 external RAM control register"]
pub mod spi_mem_cache_sctrl;
#[doc = "SPI_MEM_SRAM_CMD (rw) register accessor: an alias for `Reg<SPI_MEM_SRAM_CMD_SPEC>`"]
pub type SPI_MEM_SRAM_CMD = crate::Reg<spi_mem_sram_cmd::SPI_MEM_SRAM_CMD_SPEC>;
#[doc = "SPI0 external RAM mode control register"]
pub mod spi_mem_sram_cmd;
#[doc = "SPI_MEM_SRAM_DRD_CMD (r) register accessor: an alias for `Reg<SPI_MEM_SRAM_DRD_CMD_SPEC>`"]
pub type SPI_MEM_SRAM_DRD_CMD = crate::Reg<spi_mem_sram_drd_cmd::SPI_MEM_SRAM_DRD_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR read command control register"]
pub mod spi_mem_sram_drd_cmd;
#[doc = "SPI_MEM_SRAM_DWR_CMD (r) register accessor: an alias for `Reg<SPI_MEM_SRAM_DWR_CMD_SPEC>`"]
pub type SPI_MEM_SRAM_DWR_CMD = crate::Reg<spi_mem_sram_dwr_cmd::SPI_MEM_SRAM_DWR_CMD_SPEC>;
#[doc = "SPI0 external RAM DDR write command control register"]
pub mod spi_mem_sram_dwr_cmd;
#[doc = "SPI_MEM_SRAM_CLK (r) register accessor: an alias for `Reg<SPI_MEM_SRAM_CLK_SPEC>`"]
pub type SPI_MEM_SRAM_CLK = crate::Reg<spi_mem_sram_clk::SPI_MEM_SRAM_CLK_SPEC>;
#[doc = "SPI0 external RAM clock control register"]
pub mod spi_mem_sram_clk;
#[doc = "SPI_MEM_FSM (rw) register accessor: an alias for `Reg<SPI_MEM_FSM_SPEC>`"]
pub type SPI_MEM_FSM = crate::Reg<spi_mem_fsm::SPI_MEM_FSM_SPEC>;
#[doc = "SPI0 FSM status register"]
pub mod spi_mem_fsm;
#[doc = "SPI_MEM_INT_ENA (rw) register accessor: an alias for `Reg<SPI_MEM_INT_ENA_SPEC>`"]
pub type SPI_MEM_INT_ENA = crate::Reg<spi_mem_int_ena::SPI_MEM_INT_ENA_SPEC>;
#[doc = "SPI0 interrupt enable register"]
pub mod spi_mem_int_ena;
#[doc = "SPI_MEM_INT_CLR (rw) register accessor: an alias for `Reg<SPI_MEM_INT_CLR_SPEC>`"]
pub type SPI_MEM_INT_CLR = crate::Reg<spi_mem_int_clr::SPI_MEM_INT_CLR_SPEC>;
#[doc = "SPI0 interrupt clear register"]
pub mod spi_mem_int_clr;
#[doc = "SPI_MEM_INT_RAW (rw) register accessor: an alias for `Reg<SPI_MEM_INT_RAW_SPEC>`"]
pub type SPI_MEM_INT_RAW = crate::Reg<spi_mem_int_raw::SPI_MEM_INT_RAW_SPEC>;
#[doc = "SPI0 interrupt raw register"]
pub mod spi_mem_int_raw;
#[doc = "SPI_MEM_INT_ST (r) register accessor: an alias for `Reg<SPI_MEM_INT_ST_SPEC>`"]
pub type SPI_MEM_INT_ST = crate::Reg<spi_mem_int_st::SPI_MEM_INT_ST_SPEC>;
#[doc = "SPI0 interrupt status register"]
pub mod spi_mem_int_st;
#[doc = "SPI_MEM_DDR (r) register accessor: an alias for `Reg<SPI_MEM_DDR_SPEC>`"]
pub type SPI_MEM_DDR = crate::Reg<spi_mem_ddr::SPI_MEM_DDR_SPEC>;
#[doc = "SPI0 flash DDR mode control register"]
pub mod spi_mem_ddr;
#[doc = "SPI_SMEM_DDR (r) register accessor: an alias for `Reg<SPI_SMEM_DDR_SPEC>`"]
pub type SPI_SMEM_DDR = crate::Reg<spi_smem_ddr::SPI_SMEM_DDR_SPEC>;
#[doc = "SPI0 external RAM DDR mode control register"]
pub mod spi_smem_ddr;
#[doc = "SPI_FMEM_PMS_ATTR (rw) register accessor: an alias for `Reg<SPI_FMEM_PMS_ATTR_SPEC>`"]
pub type SPI_FMEM_PMS_ATTR = crate::Reg<spi_fmem_pms_attr::SPI_FMEM_PMS_ATTR_SPEC>;
#[doc = "MSPI flash ACE section %s attribute register"]
pub mod spi_fmem_pms_attr;
#[doc = "SPI_FMEM_PMS_ADDR (rw) register accessor: an alias for `Reg<SPI_FMEM_PMS_ADDR_SPEC>`"]
pub type SPI_FMEM_PMS_ADDR = crate::Reg<spi_fmem_pms_addr::SPI_FMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 flash ACE section %s start address register"]
pub mod spi_fmem_pms_addr;
#[doc = "SPI_FMEM_PMS_SIZE (rw) register accessor: an alias for `Reg<SPI_FMEM_PMS_SIZE_SPEC>`"]
pub type SPI_FMEM_PMS_SIZE = crate::Reg<spi_fmem_pms_size::SPI_FMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 flash ACE section %s start address register"]
pub mod spi_fmem_pms_size;
#[doc = "SPI_SMEM_PMS_ATTR (rw) register accessor: an alias for `Reg<SPI_SMEM_PMS_ATTR_SPEC>`"]
pub type SPI_SMEM_PMS_ATTR = crate::Reg<spi_smem_pms_attr::SPI_SMEM_PMS_ATTR_SPEC>;
#[doc = "SPI1 flash ACE section %s start address register"]
pub mod spi_smem_pms_attr;
#[doc = "SPI_SMEM_PMS_ADDR (rw) register accessor: an alias for `Reg<SPI_SMEM_PMS_ADDR_SPEC>`"]
pub type SPI_SMEM_PMS_ADDR = crate::Reg<spi_smem_pms_addr::SPI_SMEM_PMS_ADDR_SPEC>;
#[doc = "SPI1 external RAM ACE section %s start address register"]
pub mod spi_smem_pms_addr;
#[doc = "SPI_SMEM_PMS_SIZE (rw) register accessor: an alias for `Reg<SPI_SMEM_PMS_SIZE_SPEC>`"]
pub type SPI_SMEM_PMS_SIZE = crate::Reg<spi_smem_pms_size::SPI_SMEM_PMS_SIZE_SPEC>;
#[doc = "SPI1 external RAM ACE section %s start address register"]
pub mod spi_smem_pms_size;
#[doc = "SPI_MEM_PMS_REJECT (rw) register accessor: an alias for `Reg<SPI_MEM_PMS_REJECT_SPEC>`"]
pub type SPI_MEM_PMS_REJECT = crate::Reg<spi_mem_pms_reject::SPI_MEM_PMS_REJECT_SPEC>;
#[doc = "SPI1 access reject register"]
pub mod spi_mem_pms_reject;
#[doc = "SPI_MEM_ECC_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_ECC_CTRL_SPEC>`"]
pub type SPI_MEM_ECC_CTRL = crate::Reg<spi_mem_ecc_ctrl::SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod spi_mem_ecc_ctrl;
#[doc = "SPI_MEM_ECC_ERR_ADDR (r) register accessor: an alias for `Reg<SPI_MEM_ECC_ERR_ADDR_SPEC>`"]
pub type SPI_MEM_ECC_ERR_ADDR = crate::Reg<spi_mem_ecc_err_addr::SPI_MEM_ECC_ERR_ADDR_SPEC>;
#[doc = "MSPI ECC error address register"]
pub mod spi_mem_ecc_err_addr;
#[doc = "SPI_MEM_AXI_ERR_ADDR (r) register accessor: an alias for `Reg<SPI_MEM_AXI_ERR_ADDR_SPEC>`"]
pub type SPI_MEM_AXI_ERR_ADDR = crate::Reg<spi_mem_axi_err_addr::SPI_MEM_AXI_ERR_ADDR_SPEC>;
#[doc = "SPI0 AXI request error address."]
pub mod spi_mem_axi_err_addr;
#[doc = "SPI_SMEM_ECC_CTRL (r) register accessor: an alias for `Reg<SPI_SMEM_ECC_CTRL_SPEC>`"]
pub type SPI_SMEM_ECC_CTRL = crate::Reg<spi_smem_ecc_ctrl::SPI_SMEM_ECC_CTRL_SPEC>;
#[doc = "MSPI ECC control register"]
pub mod spi_smem_ecc_ctrl;
#[doc = "SPI_MEM_TIMING_CALI (rw) register accessor: an alias for `Reg<SPI_MEM_TIMING_CALI_SPEC>`"]
pub type SPI_MEM_TIMING_CALI = crate::Reg<spi_mem_timing_cali::SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "SPI0 flash timing calibration register"]
pub mod spi_mem_timing_cali;
#[doc = "SPI_MEM_DIN_MODE (rw) register accessor: an alias for `Reg<SPI_MEM_DIN_MODE_SPEC>`"]
pub type SPI_MEM_DIN_MODE = crate::Reg<spi_mem_din_mode::SPI_MEM_DIN_MODE_SPEC>;
#[doc = "MSPI flash input timing delay mode control register"]
pub mod spi_mem_din_mode;
#[doc = "SPI_MEM_DIN_NUM (rw) register accessor: an alias for `Reg<SPI_MEM_DIN_NUM_SPEC>`"]
pub type SPI_MEM_DIN_NUM = crate::Reg<spi_mem_din_num::SPI_MEM_DIN_NUM_SPEC>;
#[doc = "MSPI flash input timing delay number control register"]
pub mod spi_mem_din_num;
#[doc = "SPI_MEM_DOUT_MODE (rw) register accessor: an alias for `Reg<SPI_MEM_DOUT_MODE_SPEC>`"]
pub type SPI_MEM_DOUT_MODE = crate::Reg<spi_mem_dout_mode::SPI_MEM_DOUT_MODE_SPEC>;
#[doc = "MSPI flash output timing adjustment control register"]
pub mod spi_mem_dout_mode;
#[doc = "SPI_SMEM_TIMING_CALI (r) register accessor: an alias for `Reg<SPI_SMEM_TIMING_CALI_SPEC>`"]
pub type SPI_SMEM_TIMING_CALI = crate::Reg<spi_smem_timing_cali::SPI_SMEM_TIMING_CALI_SPEC>;
#[doc = "MSPI external RAM timing calibration register"]
pub mod spi_smem_timing_cali;
#[doc = "SPI_SMEM_DIN_MODE (r) register accessor: an alias for `Reg<SPI_SMEM_DIN_MODE_SPEC>`"]
pub type SPI_SMEM_DIN_MODE = crate::Reg<spi_smem_din_mode::SPI_SMEM_DIN_MODE_SPEC>;
#[doc = "MSPI external RAM input timing delay mode control register"]
pub mod spi_smem_din_mode;
#[doc = "SPI_SMEM_DIN_NUM (r) register accessor: an alias for `Reg<SPI_SMEM_DIN_NUM_SPEC>`"]
pub type SPI_SMEM_DIN_NUM = crate::Reg<spi_smem_din_num::SPI_SMEM_DIN_NUM_SPEC>;
#[doc = "MSPI external RAM input timing delay number control register"]
pub mod spi_smem_din_num;
#[doc = "SPI_SMEM_DOUT_MODE (r) register accessor: an alias for `Reg<SPI_SMEM_DOUT_MODE_SPEC>`"]
pub type SPI_SMEM_DOUT_MODE = crate::Reg<spi_smem_dout_mode::SPI_SMEM_DOUT_MODE_SPEC>;
#[doc = "MSPI external RAM output timing adjustment control register"]
pub mod spi_smem_dout_mode;
#[doc = "SPI_SMEM_AC (r) register accessor: an alias for `Reg<SPI_SMEM_AC_SPEC>`"]
pub type SPI_SMEM_AC = crate::Reg<spi_smem_ac::SPI_SMEM_AC_SPEC>;
#[doc = "MSPI external RAM ECC and SPI CS timing control register"]
pub mod spi_smem_ac;
#[doc = "SPI_MEM_CLOCK_GATE (rw) register accessor: an alias for `Reg<SPI_MEM_CLOCK_GATE_SPEC>`"]
pub type SPI_MEM_CLOCK_GATE = crate::Reg<spi_mem_clock_gate::SPI_MEM_CLOCK_GATE_SPEC>;
#[doc = "SPI0 clock gate register"]
pub mod spi_mem_clock_gate;
#[doc = "SPI_MEM_XTS_PLAIN_BASE (rw) register accessor: an alias for `Reg<SPI_MEM_XTS_PLAIN_BASE_SPEC>`"]
pub type SPI_MEM_XTS_PLAIN_BASE = crate::Reg<spi_mem_xts_plain_base::SPI_MEM_XTS_PLAIN_BASE_SPEC>;
#[doc = "The base address of the memory that stores plaintext in Manual Encryption"]
pub mod spi_mem_xts_plain_base;
#[doc = "SPI_MEM_XTS_LINESIZE (rw) register accessor: an alias for `Reg<SPI_MEM_XTS_LINESIZE_SPEC>`"]
pub type SPI_MEM_XTS_LINESIZE = crate::Reg<spi_mem_xts_linesize::SPI_MEM_XTS_LINESIZE_SPEC>;
#[doc = "Manual Encryption Line-Size register"]
pub mod spi_mem_xts_linesize;
#[doc = "SPI_MEM_XTS_DESTINATION (rw) register accessor: an alias for `Reg<SPI_MEM_XTS_DESTINATION_SPEC>`"]
pub type SPI_MEM_XTS_DESTINATION =
    crate::Reg<spi_mem_xts_destination::SPI_MEM_XTS_DESTINATION_SPEC>;
#[doc = "Manual Encryption destination register"]
pub mod spi_mem_xts_destination;
#[doc = "SPI_MEM_XTS_PHYSICAL_ADDRESS (rw) register accessor: an alias for `Reg<SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC>`"]
pub type SPI_MEM_XTS_PHYSICAL_ADDRESS =
    crate::Reg<spi_mem_xts_physical_address::SPI_MEM_XTS_PHYSICAL_ADDRESS_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_physical_address;
#[doc = "SPI_MEM_XTS_TRIGGER (w) register accessor: an alias for `Reg<SPI_MEM_XTS_TRIGGER_SPEC>`"]
pub type SPI_MEM_XTS_TRIGGER = crate::Reg<spi_mem_xts_trigger::SPI_MEM_XTS_TRIGGER_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_trigger;
#[doc = "SPI_MEM_XTS_RELEASE (w) register accessor: an alias for `Reg<SPI_MEM_XTS_RELEASE_SPEC>`"]
pub type SPI_MEM_XTS_RELEASE = crate::Reg<spi_mem_xts_release::SPI_MEM_XTS_RELEASE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_release;
#[doc = "SPI_MEM_XTS_DESTROY (w) register accessor: an alias for `Reg<SPI_MEM_XTS_DESTROY_SPEC>`"]
pub type SPI_MEM_XTS_DESTROY = crate::Reg<spi_mem_xts_destroy::SPI_MEM_XTS_DESTROY_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_destroy;
#[doc = "SPI_MEM_XTS_STATE (r) register accessor: an alias for `Reg<SPI_MEM_XTS_STATE_SPEC>`"]
pub type SPI_MEM_XTS_STATE = crate::Reg<spi_mem_xts_state::SPI_MEM_XTS_STATE_SPEC>;
#[doc = "Manual Encryption physical address register"]
pub mod spi_mem_xts_state;
#[doc = "SPI_MEM_XTS_DATE (rw) register accessor: an alias for `Reg<SPI_MEM_XTS_DATE_SPEC>`"]
pub type SPI_MEM_XTS_DATE = crate::Reg<spi_mem_xts_date::SPI_MEM_XTS_DATE_SPEC>;
#[doc = "Manual Encryption version register"]
pub mod spi_mem_xts_date;
#[doc = "SPI_MEM_MMU_ITEM_CONTENT (rw) register accessor: an alias for `Reg<SPI_MEM_MMU_ITEM_CONTENT_SPEC>`"]
pub type SPI_MEM_MMU_ITEM_CONTENT =
    crate::Reg<spi_mem_mmu_item_content::SPI_MEM_MMU_ITEM_CONTENT_SPEC>;
#[doc = "MSPI-MMU item content register"]
pub mod spi_mem_mmu_item_content;
#[doc = "SPI_MEM_MMU_ITEM_INDEX (rw) register accessor: an alias for `Reg<SPI_MEM_MMU_ITEM_INDEX_SPEC>`"]
pub type SPI_MEM_MMU_ITEM_INDEX = crate::Reg<spi_mem_mmu_item_index::SPI_MEM_MMU_ITEM_INDEX_SPEC>;
#[doc = "MSPI-MMU item index register"]
pub mod spi_mem_mmu_item_index;
#[doc = "SPI_MEM_MMU_POWER_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_MMU_POWER_CTRL_SPEC>`"]
pub type SPI_MEM_MMU_POWER_CTRL = crate::Reg<spi_mem_mmu_power_ctrl::SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "MSPI MMU power control register"]
pub mod spi_mem_mmu_power_ctrl;
#[doc = "SPI_MEM_DPA_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_DPA_CTRL_SPEC>`"]
pub type SPI_MEM_DPA_CTRL = crate::Reg<spi_mem_dpa_ctrl::SPI_MEM_DPA_CTRL_SPEC>;
#[doc = "SPI memory cryption DPA register"]
pub mod spi_mem_dpa_ctrl;
#[doc = "SPI_MEM_REGISTERRND_ECO_HIGH (r) register accessor: an alias for `Reg<SPI_MEM_REGISTERRND_ECO_HIGH_SPEC>`"]
pub type SPI_MEM_REGISTERRND_ECO_HIGH =
    crate::Reg<spi_mem_registerrnd_eco_high::SPI_MEM_REGISTERRND_ECO_HIGH_SPEC>;
#[doc = "MSPI ECO high register"]
pub mod spi_mem_registerrnd_eco_high;
#[doc = "SPI_MEM_REGISTERRND_ECO_LOW (r) register accessor: an alias for `Reg<SPI_MEM_REGISTERRND_ECO_LOW_SPEC>`"]
pub type SPI_MEM_REGISTERRND_ECO_LOW =
    crate::Reg<spi_mem_registerrnd_eco_low::SPI_MEM_REGISTERRND_ECO_LOW_SPEC>;
#[doc = "MSPI ECO low register"]
pub mod spi_mem_registerrnd_eco_low;
#[doc = "SPI_MEM_DATE (rw) register accessor: an alias for `Reg<SPI_MEM_DATE_SPEC>`"]
pub type SPI_MEM_DATE = crate::Reg<spi_mem_date::SPI_MEM_DATE_SPEC>;
#[doc = "SPI0 version control register"]
pub mod spi_mem_date;
