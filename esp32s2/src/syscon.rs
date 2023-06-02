#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - "]
    pub tick_conf: TICK_CONF,
    #[doc = "0x08 - "]
    pub clk_out_en: CLK_OUT_EN,
    #[doc = "0x0c - "]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0x10 - "]
    pub ext_mem_pms_lock: EXT_MEM_PMS_LOCK,
    #[doc = "0x14 - "]
    pub flash_ace0_attr: FLASH_ACE0_ATTR,
    #[doc = "0x18 - "]
    pub flash_ace1_attr: FLASH_ACE1_ATTR,
    #[doc = "0x1c - "]
    pub flash_ace2_attr: FLASH_ACE2_ATTR,
    #[doc = "0x20 - "]
    pub flash_ace3_attr: FLASH_ACE3_ATTR,
    #[doc = "0x24 - "]
    pub flash_ace0_addr: FLASH_ACE0_ADDR,
    #[doc = "0x28 - "]
    pub flash_ace1_addr: FLASH_ACE1_ADDR,
    #[doc = "0x2c - "]
    pub flash_ace2_addr: FLASH_ACE2_ADDR,
    #[doc = "0x30 - "]
    pub flash_ace3_addr: FLASH_ACE3_ADDR,
    #[doc = "0x34 - "]
    pub flash_ace0_size: FLASH_ACE0_SIZE,
    #[doc = "0x38 - "]
    pub flash_ace1_size: FLASH_ACE1_SIZE,
    #[doc = "0x3c - "]
    pub flash_ace2_size: FLASH_ACE2_SIZE,
    #[doc = "0x40 - "]
    pub flash_ace3_size: FLASH_ACE3_SIZE,
    #[doc = "0x44 - "]
    pub sram_ace0_attr: SRAM_ACE0_ATTR,
    #[doc = "0x48 - "]
    pub sram_ace1_attr: SRAM_ACE1_ATTR,
    #[doc = "0x4c - "]
    pub sram_ace2_attr: SRAM_ACE2_ATTR,
    #[doc = "0x50 - "]
    pub sram_ace3_attr: SRAM_ACE3_ATTR,
    #[doc = "0x54 - "]
    pub sram_ace0_addr: SRAM_ACE0_ADDR,
    #[doc = "0x58 - "]
    pub sram_ace1_addr: SRAM_ACE1_ADDR,
    #[doc = "0x5c - "]
    pub sram_ace2_addr: SRAM_ACE2_ADDR,
    #[doc = "0x60 - "]
    pub sram_ace3_addr: SRAM_ACE3_ADDR,
    #[doc = "0x64 - "]
    pub sram_ace0_size: SRAM_ACE0_SIZE,
    #[doc = "0x68 - "]
    pub sram_ace1_size: SRAM_ACE1_SIZE,
    #[doc = "0x6c - "]
    pub sram_ace2_size: SRAM_ACE2_SIZE,
    #[doc = "0x70 - "]
    pub sram_ace3_size: SRAM_ACE3_SIZE,
    #[doc = "0x74 - "]
    pub spi_mem_pms_ctrl: SPI_MEM_PMS_CTRL,
    #[doc = "0x78 - "]
    pub spi_mem_reject_addr: SPI_MEM_REJECT_ADDR,
    #[doc = "0x7c - "]
    pub sdio_ctrl: SDIO_CTRL,
    #[doc = "0x80 - "]
    pub redcy_sig0: REDCY_SIG0,
    #[doc = "0x84 - "]
    pub redcy_sig1: REDCY_SIG1,
    #[doc = "0x88 - "]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x8c - "]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x90 - "]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0x94 - "]
    pub wifi_rst_en: WIFI_RST_EN,
    #[doc = "0x98 - "]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    _reserved39: [u8; 0x0360],
    #[doc = "0x3fc - "]
    pub date: DATE,
}
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = ""]
pub mod sysclk_conf;
#[doc = "TICK_CONF (rw) register accessor: an alias for `Reg<TICK_CONF_SPEC>`"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = ""]
pub mod tick_conf;
#[doc = "CLK_OUT_EN (rw) register accessor: an alias for `Reg<CLK_OUT_EN_SPEC>`"]
pub type CLK_OUT_EN = crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>;
#[doc = ""]
pub mod clk_out_en;
#[doc = "HOST_INF_SEL (rw) register accessor: an alias for `Reg<HOST_INF_SEL_SPEC>`"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = ""]
pub mod host_inf_sel;
#[doc = "EXT_MEM_PMS_LOCK (rw) register accessor: an alias for `Reg<EXT_MEM_PMS_LOCK_SPEC>`"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>;
#[doc = ""]
pub mod ext_mem_pms_lock;
#[doc = "FLASH_ACE0_ATTR (rw) register accessor: an alias for `Reg<FLASH_ACE0_ATTR_SPEC>`"]
pub type FLASH_ACE0_ATTR = crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace0_attr;
#[doc = "FLASH_ACE1_ATTR (rw) register accessor: an alias for `Reg<FLASH_ACE1_ATTR_SPEC>`"]
pub type FLASH_ACE1_ATTR = crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace1_attr;
#[doc = "FLASH_ACE2_ATTR (rw) register accessor: an alias for `Reg<FLASH_ACE2_ATTR_SPEC>`"]
pub type FLASH_ACE2_ATTR = crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace2_attr;
#[doc = "FLASH_ACE3_ATTR (rw) register accessor: an alias for `Reg<FLASH_ACE3_ATTR_SPEC>`"]
pub type FLASH_ACE3_ATTR = crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace3_attr;
#[doc = "FLASH_ACE0_ADDR (rw) register accessor: an alias for `Reg<FLASH_ACE0_ADDR_SPEC>`"]
pub type FLASH_ACE0_ADDR = crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace0_addr;
#[doc = "FLASH_ACE1_ADDR (rw) register accessor: an alias for `Reg<FLASH_ACE1_ADDR_SPEC>`"]
pub type FLASH_ACE1_ADDR = crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace1_addr;
#[doc = "FLASH_ACE2_ADDR (rw) register accessor: an alias for `Reg<FLASH_ACE2_ADDR_SPEC>`"]
pub type FLASH_ACE2_ADDR = crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace2_addr;
#[doc = "FLASH_ACE3_ADDR (rw) register accessor: an alias for `Reg<FLASH_ACE3_ADDR_SPEC>`"]
pub type FLASH_ACE3_ADDR = crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace3_addr;
#[doc = "FLASH_ACE0_SIZE (rw) register accessor: an alias for `Reg<FLASH_ACE0_SIZE_SPEC>`"]
pub type FLASH_ACE0_SIZE = crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace0_size;
#[doc = "FLASH_ACE1_SIZE (rw) register accessor: an alias for `Reg<FLASH_ACE1_SIZE_SPEC>`"]
pub type FLASH_ACE1_SIZE = crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace1_size;
#[doc = "FLASH_ACE2_SIZE (rw) register accessor: an alias for `Reg<FLASH_ACE2_SIZE_SPEC>`"]
pub type FLASH_ACE2_SIZE = crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace2_size;
#[doc = "FLASH_ACE3_SIZE (rw) register accessor: an alias for `Reg<FLASH_ACE3_SIZE_SPEC>`"]
pub type FLASH_ACE3_SIZE = crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace3_size;
#[doc = "SRAM_ACE0_ATTR (rw) register accessor: an alias for `Reg<SRAM_ACE0_ATTR_SPEC>`"]
pub type SRAM_ACE0_ATTR = crate::Reg<sram_ace0_attr::SRAM_ACE0_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace0_attr;
#[doc = "SRAM_ACE1_ATTR (rw) register accessor: an alias for `Reg<SRAM_ACE1_ATTR_SPEC>`"]
pub type SRAM_ACE1_ATTR = crate::Reg<sram_ace1_attr::SRAM_ACE1_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace1_attr;
#[doc = "SRAM_ACE2_ATTR (rw) register accessor: an alias for `Reg<SRAM_ACE2_ATTR_SPEC>`"]
pub type SRAM_ACE2_ATTR = crate::Reg<sram_ace2_attr::SRAM_ACE2_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace2_attr;
#[doc = "SRAM_ACE3_ATTR (rw) register accessor: an alias for `Reg<SRAM_ACE3_ATTR_SPEC>`"]
pub type SRAM_ACE3_ATTR = crate::Reg<sram_ace3_attr::SRAM_ACE3_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace3_attr;
#[doc = "SRAM_ACE0_ADDR (rw) register accessor: an alias for `Reg<SRAM_ACE0_ADDR_SPEC>`"]
pub type SRAM_ACE0_ADDR = crate::Reg<sram_ace0_addr::SRAM_ACE0_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace0_addr;
#[doc = "SRAM_ACE1_ADDR (rw) register accessor: an alias for `Reg<SRAM_ACE1_ADDR_SPEC>`"]
pub type SRAM_ACE1_ADDR = crate::Reg<sram_ace1_addr::SRAM_ACE1_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace1_addr;
#[doc = "SRAM_ACE2_ADDR (rw) register accessor: an alias for `Reg<SRAM_ACE2_ADDR_SPEC>`"]
pub type SRAM_ACE2_ADDR = crate::Reg<sram_ace2_addr::SRAM_ACE2_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace2_addr;
#[doc = "SRAM_ACE3_ADDR (rw) register accessor: an alias for `Reg<SRAM_ACE3_ADDR_SPEC>`"]
pub type SRAM_ACE3_ADDR = crate::Reg<sram_ace3_addr::SRAM_ACE3_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace3_addr;
#[doc = "SRAM_ACE0_SIZE (rw) register accessor: an alias for `Reg<SRAM_ACE0_SIZE_SPEC>`"]
pub type SRAM_ACE0_SIZE = crate::Reg<sram_ace0_size::SRAM_ACE0_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace0_size;
#[doc = "SRAM_ACE1_SIZE (rw) register accessor: an alias for `Reg<SRAM_ACE1_SIZE_SPEC>`"]
pub type SRAM_ACE1_SIZE = crate::Reg<sram_ace1_size::SRAM_ACE1_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace1_size;
#[doc = "SRAM_ACE2_SIZE (rw) register accessor: an alias for `Reg<SRAM_ACE2_SIZE_SPEC>`"]
pub type SRAM_ACE2_SIZE = crate::Reg<sram_ace2_size::SRAM_ACE2_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace2_size;
#[doc = "SRAM_ACE3_SIZE (rw) register accessor: an alias for `Reg<SRAM_ACE3_SIZE_SPEC>`"]
pub type SRAM_ACE3_SIZE = crate::Reg<sram_ace3_size::SRAM_ACE3_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace3_size;
#[doc = "SPI_MEM_PMS_CTRL (rw) register accessor: an alias for `Reg<SPI_MEM_PMS_CTRL_SPEC>`"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>;
#[doc = ""]
pub mod spi_mem_pms_ctrl;
#[doc = "SPI_MEM_REJECT_ADDR (r) register accessor: an alias for `Reg<SPI_MEM_REJECT_ADDR_SPEC>`"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>;
#[doc = ""]
pub mod spi_mem_reject_addr;
#[doc = "SDIO_CTRL (rw) register accessor: an alias for `Reg<SDIO_CTRL_SPEC>`"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = ""]
pub mod sdio_ctrl;
#[doc = "REDCY_SIG0 (rw) register accessor: an alias for `Reg<REDCY_SIG0_SPEC>`"]
pub type REDCY_SIG0 = crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>;
#[doc = ""]
pub mod redcy_sig0;
#[doc = "REDCY_SIG1 (rw) register accessor: an alias for `Reg<REDCY_SIG1_SPEC>`"]
pub type REDCY_SIG1 = crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>;
#[doc = ""]
pub mod redcy_sig1;
#[doc = "WIFI_BB_CFG (rw) register accessor: an alias for `Reg<WIFI_BB_CFG_SPEC>`"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: an alias for `Reg<WIFI_BB_CFG_2_SPEC>`"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg_2;
#[doc = "WIFI_CLK_EN (rw) register accessor: an alias for `Reg<WIFI_CLK_EN_SPEC>`"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = ""]
pub mod wifi_clk_en;
#[doc = "WIFI_RST_EN (rw) register accessor: an alias for `Reg<WIFI_RST_EN_SPEC>`"]
pub type WIFI_RST_EN = crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>;
#[doc = ""]
pub mod wifi_rst_en;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: an alias for `Reg<FRONT_END_MEM_PD_SPEC>`"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = ""]
pub mod front_end_mem_pd;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
