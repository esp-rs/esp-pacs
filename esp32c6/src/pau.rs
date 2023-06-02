#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Peri backup control register"]
    pub regdma_conf: REGDMA_CONF,
    #[doc = "0x04 - Clock control register"]
    pub regdma_clk_conf: REGDMA_CLK_CONF,
    #[doc = "0x08 - ETM start ctrl reg"]
    pub regdma_etm_ctrl: REGDMA_ETM_CTRL,
    #[doc = "0x0c - link_0_addr"]
    pub regdma_link_0_addr: REGDMA_LINK_0_ADDR,
    #[doc = "0x10 - Link_1_addr"]
    pub regdma_link_1_addr: REGDMA_LINK_1_ADDR,
    #[doc = "0x14 - Link_2_addr"]
    pub regdma_link_2_addr: REGDMA_LINK_2_ADDR,
    #[doc = "0x18 - Link_3_addr"]
    pub regdma_link_3_addr: REGDMA_LINK_3_ADDR,
    #[doc = "0x1c - Link_mac_addr"]
    pub regdma_link_mac_addr: REGDMA_LINK_MAC_ADDR,
    #[doc = "0x20 - current link addr"]
    pub regdma_current_link_addr: REGDMA_CURRENT_LINK_ADDR,
    #[doc = "0x24 - Backup addr"]
    pub regdma_backup_addr: REGDMA_BACKUP_ADDR,
    #[doc = "0x28 - mem addr"]
    pub regdma_mem_addr: REGDMA_MEM_ADDR,
    #[doc = "0x2c - backup config"]
    pub regdma_bkp_conf: REGDMA_BKP_CONF,
    #[doc = "0x30 - retention dma link base"]
    pub retention_link_base: RETENTION_LINK_BASE,
    #[doc = "0x34 - retention_cfg"]
    pub retention_cfg: RETENTION_CFG,
    #[doc = "0x38 - Read only register for error and done"]
    pub int_ena: INT_ENA,
    #[doc = "0x3c - Read only register for error and done"]
    pub int_raw: INT_RAW,
    #[doc = "0x40 - Read only register for error and done"]
    pub int_clr: INT_CLR,
    #[doc = "0x44 - Read only register for error and done"]
    pub int_st: INT_ST,
    _reserved18: [u8; 0x03b4],
    #[doc = "0x3fc - Date register."]
    pub date: DATE,
}
#[doc = "REGDMA_CONF (rw) register accessor: an alias for `Reg<REGDMA_CONF_SPEC>`"]
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
#[doc = "Peri backup control register"]
pub mod regdma_conf;
#[doc = "REGDMA_CLK_CONF (rw) register accessor: an alias for `Reg<REGDMA_CLK_CONF_SPEC>`"]
pub type REGDMA_CLK_CONF = crate::Reg<regdma_clk_conf::REGDMA_CLK_CONF_SPEC>;
#[doc = "Clock control register"]
pub mod regdma_clk_conf;
#[doc = "REGDMA_ETM_CTRL (w) register accessor: an alias for `Reg<REGDMA_ETM_CTRL_SPEC>`"]
pub type REGDMA_ETM_CTRL = crate::Reg<regdma_etm_ctrl::REGDMA_ETM_CTRL_SPEC>;
#[doc = "ETM start ctrl reg"]
pub mod regdma_etm_ctrl;
#[doc = "REGDMA_LINK_0_ADDR (rw) register accessor: an alias for `Reg<REGDMA_LINK_0_ADDR_SPEC>`"]
pub type REGDMA_LINK_0_ADDR = crate::Reg<regdma_link_0_addr::REGDMA_LINK_0_ADDR_SPEC>;
#[doc = "link_0_addr"]
pub mod regdma_link_0_addr;
#[doc = "REGDMA_LINK_1_ADDR (rw) register accessor: an alias for `Reg<REGDMA_LINK_1_ADDR_SPEC>`"]
pub type REGDMA_LINK_1_ADDR = crate::Reg<regdma_link_1_addr::REGDMA_LINK_1_ADDR_SPEC>;
#[doc = "Link_1_addr"]
pub mod regdma_link_1_addr;
#[doc = "REGDMA_LINK_2_ADDR (rw) register accessor: an alias for `Reg<REGDMA_LINK_2_ADDR_SPEC>`"]
pub type REGDMA_LINK_2_ADDR = crate::Reg<regdma_link_2_addr::REGDMA_LINK_2_ADDR_SPEC>;
#[doc = "Link_2_addr"]
pub mod regdma_link_2_addr;
#[doc = "REGDMA_LINK_3_ADDR (rw) register accessor: an alias for `Reg<REGDMA_LINK_3_ADDR_SPEC>`"]
pub type REGDMA_LINK_3_ADDR = crate::Reg<regdma_link_3_addr::REGDMA_LINK_3_ADDR_SPEC>;
#[doc = "Link_3_addr"]
pub mod regdma_link_3_addr;
#[doc = "REGDMA_LINK_MAC_ADDR (rw) register accessor: an alias for `Reg<REGDMA_LINK_MAC_ADDR_SPEC>`"]
pub type REGDMA_LINK_MAC_ADDR = crate::Reg<regdma_link_mac_addr::REGDMA_LINK_MAC_ADDR_SPEC>;
#[doc = "Link_mac_addr"]
pub mod regdma_link_mac_addr;
#[doc = "REGDMA_CURRENT_LINK_ADDR (r) register accessor: an alias for `Reg<REGDMA_CURRENT_LINK_ADDR_SPEC>`"]
pub type REGDMA_CURRENT_LINK_ADDR =
    crate::Reg<regdma_current_link_addr::REGDMA_CURRENT_LINK_ADDR_SPEC>;
#[doc = "current link addr"]
pub mod regdma_current_link_addr;
#[doc = "REGDMA_BACKUP_ADDR (r) register accessor: an alias for `Reg<REGDMA_BACKUP_ADDR_SPEC>`"]
pub type REGDMA_BACKUP_ADDR = crate::Reg<regdma_backup_addr::REGDMA_BACKUP_ADDR_SPEC>;
#[doc = "Backup addr"]
pub mod regdma_backup_addr;
#[doc = "REGDMA_MEM_ADDR (r) register accessor: an alias for `Reg<REGDMA_MEM_ADDR_SPEC>`"]
pub type REGDMA_MEM_ADDR = crate::Reg<regdma_mem_addr::REGDMA_MEM_ADDR_SPEC>;
#[doc = "mem addr"]
pub mod regdma_mem_addr;
#[doc = "REGDMA_BKP_CONF (rw) register accessor: an alias for `Reg<REGDMA_BKP_CONF_SPEC>`"]
pub type REGDMA_BKP_CONF = crate::Reg<regdma_bkp_conf::REGDMA_BKP_CONF_SPEC>;
#[doc = "backup config"]
pub mod regdma_bkp_conf;
#[doc = "RETENTION_LINK_BASE (rw) register accessor: an alias for `Reg<RETENTION_LINK_BASE_SPEC>`"]
pub type RETENTION_LINK_BASE = crate::Reg<retention_link_base::RETENTION_LINK_BASE_SPEC>;
#[doc = "retention dma link base"]
pub mod retention_link_base;
#[doc = "RETENTION_CFG (rw) register accessor: an alias for `Reg<RETENTION_CFG_SPEC>`"]
pub type RETENTION_CFG = crate::Reg<retention_cfg::RETENTION_CFG_SPEC>;
#[doc = "retention_cfg"]
pub mod retention_cfg;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_st;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
