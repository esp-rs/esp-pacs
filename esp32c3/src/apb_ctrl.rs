#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_CTRL_SYSCLK_CONF_REG"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - APB_CTRL_TICK_CONF_REG"]
    pub tick_conf: TICK_CONF,
    #[doc = "0x08 - APB_CTRL_CLK_OUT_EN_REG"]
    pub clk_out_en: CLK_OUT_EN,
    #[doc = "0x0c - APB_CTRL_WIFI_BB_CFG_REG"]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x10 - APB_CTRL_WIFI_BB_CFG_2_REG"]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x14 - APB_CTRL_WIFI_CLK_EN_REG"]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0x18 - APB_CTRL_WIFI_RST_EN_REG"]
    pub wifi_rst_en: WIFI_RST_EN,
    #[doc = "0x1c - APB_CTRL_HOST_INF_SEL_REG"]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0x20 - APB_CTRL_EXT_MEM_PMS_LOCK_REG"]
    pub ext_mem_pms_lock: EXT_MEM_PMS_LOCK,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - APB_CTRL_FLASH_ACE0_ATTR_REG"]
    pub flash_ace0_attr: FLASH_ACE0_ATTR,
    #[doc = "0x2c - APB_CTRL_FLASH_ACE1_ATTR_REG"]
    pub flash_ace1_attr: FLASH_ACE1_ATTR,
    #[doc = "0x30 - APB_CTRL_FLASH_ACE2_ATTR_REG"]
    pub flash_ace2_attr: FLASH_ACE2_ATTR,
    #[doc = "0x34 - APB_CTRL_FLASH_ACE3_ATTR_REG"]
    pub flash_ace3_attr: FLASH_ACE3_ATTR,
    #[doc = "0x38 - APB_CTRL_FLASH_ACE0_ADDR_REG"]
    pub flash_ace0_addr: FLASH_ACE0_ADDR,
    #[doc = "0x3c - APB_CTRL_FLASH_ACE1_ADDR_REG"]
    pub flash_ace1_addr: FLASH_ACE1_ADDR,
    #[doc = "0x40 - APB_CTRL_FLASH_ACE2_ADDR_REG"]
    pub flash_ace2_addr: FLASH_ACE2_ADDR,
    #[doc = "0x44 - APB_CTRL_FLASH_ACE3_ADDR_REG"]
    pub flash_ace3_addr: FLASH_ACE3_ADDR,
    #[doc = "0x48 - APB_CTRL_FLASH_ACE0_SIZE_REG"]
    pub flash_ace0_size: FLASH_ACE0_SIZE,
    #[doc = "0x4c - APB_CTRL_FLASH_ACE1_SIZE_REG"]
    pub flash_ace1_size: FLASH_ACE1_SIZE,
    #[doc = "0x50 - APB_CTRL_FLASH_ACE2_SIZE_REG"]
    pub flash_ace2_size: FLASH_ACE2_SIZE,
    #[doc = "0x54 - APB_CTRL_FLASH_ACE3_SIZE_REG"]
    pub flash_ace3_size: FLASH_ACE3_SIZE,
    _reserved21: [u8; 0x30],
    #[doc = "0x88 - APB_CTRL_SPI_MEM_PMS_CTRL_REG"]
    pub spi_mem_pms_ctrl: SPI_MEM_PMS_CTRL,
    #[doc = "0x8c - APB_CTRL_SPI_MEM_REJECT_ADDR_REG"]
    pub spi_mem_reject_addr: SPI_MEM_REJECT_ADDR,
    #[doc = "0x90 - APB_CTRL_SDIO_CTRL_REG"]
    pub sdio_ctrl: SDIO_CTRL,
    #[doc = "0x94 - APB_CTRL_REDCY_SIG0_REG_REG"]
    pub redcy_sig0: REDCY_SIG0,
    #[doc = "0x98 - APB_CTRL_REDCY_SIG1_REG_REG"]
    pub redcy_sig1: REDCY_SIG1,
    #[doc = "0x9c - APB_CTRL_FRONT_END_MEM_PD_REG"]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    #[doc = "0xa0 - APB_CTRL_RETENTION_CTRL_REG"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0xa4 - APB_CTRL_CLKGATE_FORCE_ON_REG"]
    pub clkgate_force_on: CLKGATE_FORCE_ON,
    #[doc = "0xa8 - APB_CTRL_MEM_POWER_DOWN_REG"]
    pub mem_power_down: MEM_POWER_DOWN,
    #[doc = "0xac - APB_CTRL_MEM_POWER_UP_REG"]
    pub mem_power_up: MEM_POWER_UP,
    #[doc = "0xb0 - APB_CTRL_RND_DATA_REG"]
    pub rnd_data: RND_DATA,
    #[doc = "0xb4 - APB_CTRL_PERI_BACKUP_CONFIG_REG_REG"]
    pub peri_backup_config: PERI_BACKUP_CONFIG,
    #[doc = "0xb8 - APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG"]
    pub peri_backup_apb_addr: PERI_BACKUP_APB_ADDR,
    #[doc = "0xbc - APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG"]
    pub peri_backup_mem_addr: PERI_BACKUP_MEM_ADDR,
    #[doc = "0xc0 - APB_CTRL_PERI_BACKUP_INT_RAW_REG"]
    pub peri_backup_int_raw: PERI_BACKUP_INT_RAW,
    #[doc = "0xc4 - APB_CTRL_PERI_BACKUP_INT_ST_REG"]
    pub peri_backup_int_st: PERI_BACKUP_INT_ST,
    #[doc = "0xc8 - APB_CTRL_PERI_BACKUP_INT_ENA_REG"]
    pub peri_backup_int_ena: PERI_BACKUP_INT_ENA,
    _reserved38: [u8; 0x04],
    #[doc = "0xd0 - APB_CTRL_PERI_BACKUP_INT_CLR_REG"]
    pub peri_backup_int_clr: PERI_BACKUP_INT_CLR,
    _reserved39: [u8; 0x0328],
    #[doc = "0x3fc - APB_CTRL_DATE_REG"]
    pub date: DATE,
}
#[doc = "SYSCLK_CONF (rw) register accessor: APB_CTRL_SYSCLK_CONF_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "APB_CTRL_SYSCLK_CONF_REG"]
pub mod sysclk_conf;
#[doc = "TICK_CONF (rw) register accessor: APB_CTRL_TICK_CONF_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tick_conf`] module"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = "APB_CTRL_TICK_CONF_REG"]
pub mod tick_conf;
#[doc = "CLK_OUT_EN (rw) register accessor: APB_CTRL_CLK_OUT_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_out_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_out_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_out_en`] module"]
pub type CLK_OUT_EN = crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>;
#[doc = "APB_CTRL_CLK_OUT_EN_REG"]
pub mod clk_out_en;
#[doc = "WIFI_BB_CFG (rw) register accessor: APB_CTRL_WIFI_BB_CFG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_bb_cfg`] module"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = "APB_CTRL_WIFI_BB_CFG_REG"]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: APB_CTRL_WIFI_BB_CFG_2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_bb_cfg_2`] module"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = "APB_CTRL_WIFI_BB_CFG_2_REG"]
pub mod wifi_bb_cfg_2;
#[doc = "WIFI_CLK_EN (rw) register accessor: APB_CTRL_WIFI_CLK_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_clk_en`] module"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = "APB_CTRL_WIFI_CLK_EN_REG"]
pub mod wifi_clk_en;
#[doc = "WIFI_RST_EN (rw) register accessor: APB_CTRL_WIFI_RST_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_rst_en`] module"]
pub type WIFI_RST_EN = crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>;
#[doc = "APB_CTRL_WIFI_RST_EN_REG"]
pub mod wifi_rst_en;
#[doc = "HOST_INF_SEL (rw) register accessor: APB_CTRL_HOST_INF_SEL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`host_inf_sel`] module"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = "APB_CTRL_HOST_INF_SEL_REG"]
pub mod host_inf_sel;
#[doc = "EXT_MEM_PMS_LOCK (rw) register accessor: APB_CTRL_EXT_MEM_PMS_LOCK_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_pms_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_pms_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_mem_pms_lock`] module"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>;
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK_REG"]
pub mod ext_mem_pms_lock;
#[doc = "FLASH_ACE0_ATTR (rw) register accessor: APB_CTRL_FLASH_ACE0_ATTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_attr`] module"]
pub type FLASH_ACE0_ATTR = crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE0_ATTR_REG"]
pub mod flash_ace0_attr;
#[doc = "FLASH_ACE1_ATTR (rw) register accessor: APB_CTRL_FLASH_ACE1_ATTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_attr`] module"]
pub type FLASH_ACE1_ATTR = crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE1_ATTR_REG"]
pub mod flash_ace1_attr;
#[doc = "FLASH_ACE2_ATTR (rw) register accessor: APB_CTRL_FLASH_ACE2_ATTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_attr`] module"]
pub type FLASH_ACE2_ATTR = crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE2_ATTR_REG"]
pub mod flash_ace2_attr;
#[doc = "FLASH_ACE3_ATTR (rw) register accessor: APB_CTRL_FLASH_ACE3_ATTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_attr`] module"]
pub type FLASH_ACE3_ATTR = crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE3_ATTR_REG"]
pub mod flash_ace3_attr;
#[doc = "FLASH_ACE0_ADDR (rw) register accessor: APB_CTRL_FLASH_ACE0_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_addr`] module"]
pub type FLASH_ACE0_ADDR = crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE0_ADDR_REG"]
pub mod flash_ace0_addr;
#[doc = "FLASH_ACE1_ADDR (rw) register accessor: APB_CTRL_FLASH_ACE1_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_addr`] module"]
pub type FLASH_ACE1_ADDR = crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE1_ADDR_REG"]
pub mod flash_ace1_addr;
#[doc = "FLASH_ACE2_ADDR (rw) register accessor: APB_CTRL_FLASH_ACE2_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_addr`] module"]
pub type FLASH_ACE2_ADDR = crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE2_ADDR_REG"]
pub mod flash_ace2_addr;
#[doc = "FLASH_ACE3_ADDR (rw) register accessor: APB_CTRL_FLASH_ACE3_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_addr`] module"]
pub type FLASH_ACE3_ADDR = crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE3_ADDR_REG"]
pub mod flash_ace3_addr;
#[doc = "FLASH_ACE0_SIZE (rw) register accessor: APB_CTRL_FLASH_ACE0_SIZE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_size`] module"]
pub type FLASH_ACE0_SIZE = crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE0_SIZE_REG"]
pub mod flash_ace0_size;
#[doc = "FLASH_ACE1_SIZE (rw) register accessor: APB_CTRL_FLASH_ACE1_SIZE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_size`] module"]
pub type FLASH_ACE1_SIZE = crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE1_SIZE_REG"]
pub mod flash_ace1_size;
#[doc = "FLASH_ACE2_SIZE (rw) register accessor: APB_CTRL_FLASH_ACE2_SIZE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_size`] module"]
pub type FLASH_ACE2_SIZE = crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE2_SIZE_REG"]
pub mod flash_ace2_size;
#[doc = "FLASH_ACE3_SIZE (rw) register accessor: APB_CTRL_FLASH_ACE3_SIZE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_size`] module"]
pub type FLASH_ACE3_SIZE = crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>;
#[doc = "APB_CTRL_FLASH_ACE3_SIZE_REG"]
pub mod flash_ace3_size;
#[doc = "SPI_MEM_PMS_CTRL (rw) register accessor: APB_CTRL_SPI_MEM_PMS_CTRL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_pms_ctrl`] module"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>;
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL_REG"]
pub mod spi_mem_pms_ctrl;
#[doc = "SPI_MEM_REJECT_ADDR (r) register accessor: APB_CTRL_SPI_MEM_REJECT_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_reject_addr`] module"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>;
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR_REG"]
pub mod spi_mem_reject_addr;
#[doc = "SDIO_CTRL (rw) register accessor: APB_CTRL_SDIO_CTRL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_ctrl`] module"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = "APB_CTRL_SDIO_CTRL_REG"]
pub mod sdio_ctrl;
#[doc = "REDCY_SIG0 (rw) register accessor: APB_CTRL_REDCY_SIG0_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`redcy_sig0`] module"]
pub type REDCY_SIG0 = crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>;
#[doc = "APB_CTRL_REDCY_SIG0_REG_REG"]
pub mod redcy_sig0;
#[doc = "REDCY_SIG1 (rw) register accessor: APB_CTRL_REDCY_SIG1_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`redcy_sig1`] module"]
pub type REDCY_SIG1 = crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>;
#[doc = "APB_CTRL_REDCY_SIG1_REG_REG"]
pub mod redcy_sig1;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: APB_CTRL_FRONT_END_MEM_PD_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`front_end_mem_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`front_end_mem_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`front_end_mem_pd`] module"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = "APB_CTRL_FRONT_END_MEM_PD_REG"]
pub mod front_end_mem_pd;
#[doc = "RETENTION_CTRL (rw) register accessor: APB_CTRL_RETENTION_CTRL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl`] module"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "APB_CTRL_RETENTION_CTRL_REG"]
pub mod retention_ctrl;
#[doc = "CLKGATE_FORCE_ON (rw) register accessor: APB_CTRL_CLKGATE_FORCE_ON_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkgate_force_on`] module"]
pub type CLKGATE_FORCE_ON = crate::Reg<clkgate_force_on::CLKGATE_FORCE_ON_SPEC>;
#[doc = "APB_CTRL_CLKGATE_FORCE_ON_REG"]
pub mod clkgate_force_on;
#[doc = "MEM_POWER_DOWN (rw) register accessor: APB_CTRL_MEM_POWER_DOWN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_power_down::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_down::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_power_down`] module"]
pub type MEM_POWER_DOWN = crate::Reg<mem_power_down::MEM_POWER_DOWN_SPEC>;
#[doc = "APB_CTRL_MEM_POWER_DOWN_REG"]
pub mod mem_power_down;
#[doc = "MEM_POWER_UP (rw) register accessor: APB_CTRL_MEM_POWER_UP_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_power_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_power_up`] module"]
pub type MEM_POWER_UP = crate::Reg<mem_power_up::MEM_POWER_UP_SPEC>;
#[doc = "APB_CTRL_MEM_POWER_UP_REG"]
pub mod mem_power_up;
#[doc = "RND_DATA (r) register accessor: APB_CTRL_RND_DATA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rnd_data`] module"]
pub type RND_DATA = crate::Reg<rnd_data::RND_DATA_SPEC>;
#[doc = "APB_CTRL_RND_DATA_REG"]
pub mod rnd_data;
#[doc = "PERI_BACKUP_CONFIG (rw) register accessor: APB_CTRL_PERI_BACKUP_CONFIG_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_config`] module"]
pub type PERI_BACKUP_CONFIG = crate::Reg<peri_backup_config::PERI_BACKUP_CONFIG_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG_REG_REG"]
pub mod peri_backup_config;
#[doc = "PERI_BACKUP_APB_ADDR (rw) register accessor: APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_apb_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_apb_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_apb_addr`] module"]
pub type PERI_BACKUP_APB_ADDR = crate::Reg<peri_backup_apb_addr::PERI_BACKUP_APB_ADDR_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG"]
pub mod peri_backup_apb_addr;
#[doc = "PERI_BACKUP_MEM_ADDR (rw) register accessor: APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_mem_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_mem_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_mem_addr`] module"]
pub type PERI_BACKUP_MEM_ADDR = crate::Reg<peri_backup_mem_addr::PERI_BACKUP_MEM_ADDR_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG"]
pub mod peri_backup_mem_addr;
#[doc = "PERI_BACKUP_INT_RAW (r) register accessor: APB_CTRL_PERI_BACKUP_INT_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_int_raw`] module"]
pub type PERI_BACKUP_INT_RAW = crate::Reg<peri_backup_int_raw::PERI_BACKUP_INT_RAW_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW_REG"]
pub mod peri_backup_int_raw;
#[doc = "PERI_BACKUP_INT_ST (r) register accessor: APB_CTRL_PERI_BACKUP_INT_ST_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_int_st`] module"]
pub type PERI_BACKUP_INT_ST = crate::Reg<peri_backup_int_st::PERI_BACKUP_INT_ST_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST_REG"]
pub mod peri_backup_int_st;
#[doc = "PERI_BACKUP_INT_ENA (rw) register accessor: APB_CTRL_PERI_BACKUP_INT_ENA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_int_ena`] module"]
pub type PERI_BACKUP_INT_ENA = crate::Reg<peri_backup_int_ena::PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG"]
pub mod peri_backup_int_ena;
#[doc = "PERI_BACKUP_INT_CLR (w) register accessor: APB_CTRL_PERI_BACKUP_INT_CLR_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peri_backup_int_clr`] module"]
pub type PERI_BACKUP_INT_CLR = crate::Reg<peri_backup_int_clr::PERI_BACKUP_INT_CLR_SPEC>;
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR_REG"]
pub mod peri_backup_int_clr;
#[doc = "DATE (rw) register accessor: APB_CTRL_DATE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "APB_CTRL_DATE_REG"]
pub mod date;
