#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - ******* Description ***********"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x04 - ******* Description ***********"]
    pub tick_conf: TICK_CONF,
    #[doc = "0x08 - ******* Description ***********"]
    pub clk_out_en: CLK_OUT_EN,
    #[doc = "0x0c - ******* Description ***********"]
    pub wifi_bb_cfg: WIFI_BB_CFG,
    #[doc = "0x10 - ******* Description ***********"]
    pub wifi_bb_cfg_2: WIFI_BB_CFG_2,
    #[doc = "0x14 - ******* Description ***********"]
    pub wifi_clk_en: WIFI_CLK_EN,
    #[doc = "0x18 - ******* Description ***********"]
    pub wifi_rst_en: WIFI_RST_EN,
    #[doc = "0x1c - ******* Description ***********"]
    pub host_inf_sel: HOST_INF_SEL,
    #[doc = "0x20 - ******* Description ***********"]
    pub ext_mem_pms_lock: EXT_MEM_PMS_LOCK,
    #[doc = "0x24 - ******* Description ***********"]
    pub ext_mem_writeback_bypass: EXT_MEM_WRITEBACK_BYPASS,
    #[doc = "0x28 - ******* Description ***********"]
    pub flash_ace0_attr: FLASH_ACE0_ATTR,
    #[doc = "0x2c - ******* Description ***********"]
    pub flash_ace1_attr: FLASH_ACE1_ATTR,
    #[doc = "0x30 - ******* Description ***********"]
    pub flash_ace2_attr: FLASH_ACE2_ATTR,
    #[doc = "0x34 - ******* Description ***********"]
    pub flash_ace3_attr: FLASH_ACE3_ATTR,
    #[doc = "0x38 - ******* Description ***********"]
    pub flash_ace0_addr: FLASH_ACE0_ADDR,
    #[doc = "0x3c - ******* Description ***********"]
    pub flash_ace1_addr: FLASH_ACE1_ADDR,
    #[doc = "0x40 - ******* Description ***********"]
    pub flash_ace2_addr: FLASH_ACE2_ADDR,
    #[doc = "0x44 - ******* Description ***********"]
    pub flash_ace3_addr: FLASH_ACE3_ADDR,
    #[doc = "0x48 - ******* Description ***********"]
    pub flash_ace0_size: FLASH_ACE0_SIZE,
    #[doc = "0x4c - ******* Description ***********"]
    pub flash_ace1_size: FLASH_ACE1_SIZE,
    #[doc = "0x50 - ******* Description ***********"]
    pub flash_ace2_size: FLASH_ACE2_SIZE,
    #[doc = "0x54 - ******* Description ***********"]
    pub flash_ace3_size: FLASH_ACE3_SIZE,
    #[doc = "0x58 - ******* Description ***********"]
    pub sram_ace0_attr: SRAM_ACE0_ATTR,
    #[doc = "0x5c - ******* Description ***********"]
    pub sram_ace1_attr: SRAM_ACE1_ATTR,
    #[doc = "0x60 - ******* Description ***********"]
    pub sram_ace2_attr: SRAM_ACE2_ATTR,
    #[doc = "0x64 - ******* Description ***********"]
    pub sram_ace3_attr: SRAM_ACE3_ATTR,
    #[doc = "0x68 - ******* Description ***********"]
    pub sram_ace0_addr: SRAM_ACE0_ADDR,
    #[doc = "0x6c - ******* Description ***********"]
    pub sram_ace1_addr: SRAM_ACE1_ADDR,
    #[doc = "0x70 - ******* Description ***********"]
    pub sram_ace2_addr: SRAM_ACE2_ADDR,
    #[doc = "0x74 - ******* Description ***********"]
    pub sram_ace3_addr: SRAM_ACE3_ADDR,
    #[doc = "0x78 - ******* Description ***********"]
    pub sram_ace0_size: SRAM_ACE0_SIZE,
    #[doc = "0x7c - ******* Description ***********"]
    pub sram_ace1_size: SRAM_ACE1_SIZE,
    #[doc = "0x80 - ******* Description ***********"]
    pub sram_ace2_size: SRAM_ACE2_SIZE,
    #[doc = "0x84 - ******* Description ***********"]
    pub sram_ace3_size: SRAM_ACE3_SIZE,
    #[doc = "0x88 - ******* Description ***********"]
    pub spi_mem_pms_ctrl: SPI_MEM_PMS_CTRL,
    #[doc = "0x8c - ******* Description ***********"]
    pub spi_mem_reject_addr: SPI_MEM_REJECT_ADDR,
    #[doc = "0x90 - ******* Description ***********"]
    pub sdio_ctrl: SDIO_CTRL,
    #[doc = "0x94 - ******* Description ***********"]
    pub redcy_sig0: REDCY_SIG0,
    #[doc = "0x98 - ******* Description ***********"]
    pub redcy_sig1: REDCY_SIG1,
    #[doc = "0x9c - ******* Description ***********"]
    pub front_end_mem_pd: FRONT_END_MEM_PD,
    #[doc = "0xa0 - ******* Description ***********"]
    pub spi_mem_ecc_ctrl: SPI_MEM_ECC_CTRL,
    _reserved41: [u8; 0x04],
    #[doc = "0xa8 - ******* Description ***********"]
    pub clkgate_force_on: CLKGATE_FORCE_ON,
    #[doc = "0xac - ******* Description ***********"]
    pub mem_power_down: MEM_POWER_DOWN,
    #[doc = "0xb0 - ******* Description ***********"]
    pub mem_power_up: MEM_POWER_UP,
    #[doc = "0xb4 - ******* Description ***********"]
    pub retention_ctrl: RETENTION_CTRL,
    #[doc = "0xb8 - ******* Description ***********"]
    pub retention_ctrl1: RETENTION_CTRL1,
    #[doc = "0xbc - ******* Description ***********"]
    pub retention_ctrl2: RETENTION_CTRL2,
    #[doc = "0xc0 - ******* Description ***********"]
    pub retention_ctrl3: RETENTION_CTRL3,
    #[doc = "0xc4 - ******* Description ***********"]
    pub retention_ctrl4: RETENTION_CTRL4,
    #[doc = "0xc8 - ******* Description ***********"]
    pub retention_ctrl5: RETENTION_CTRL5,
    _reserved50: [u8; 0x0330],
    #[doc = "0x3fc - ******* Description ***********"]
    pub date: DATE,
}
#[doc = "SYSCLK_CONF (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "******* Description ***********"]
pub mod sysclk_conf;
#[doc = "TICK_CONF (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tick_conf`] module"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = "******* Description ***********"]
pub mod tick_conf;
#[doc = "CLK_OUT_EN (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_out_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_out_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_out_en`] module"]
pub type CLK_OUT_EN = crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>;
#[doc = "******* Description ***********"]
pub mod clk_out_en;
#[doc = "WIFI_BB_CFG (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_bb_cfg`] module"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = "******* Description ***********"]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_bb_cfg_2`] module"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = "******* Description ***********"]
pub mod wifi_bb_cfg_2;
#[doc = "WIFI_CLK_EN (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_clk_en`] module"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = "******* Description ***********"]
pub mod wifi_clk_en;
#[doc = "WIFI_RST_EN (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wifi_rst_en`] module"]
pub type WIFI_RST_EN = crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>;
#[doc = "******* Description ***********"]
pub mod wifi_rst_en;
#[doc = "HOST_INF_SEL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`host_inf_sel`] module"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = "******* Description ***********"]
pub mod host_inf_sel;
#[doc = "EXT_MEM_PMS_LOCK (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_pms_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_pms_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_mem_pms_lock`] module"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>;
#[doc = "******* Description ***********"]
pub mod ext_mem_pms_lock;
#[doc = "EXT_MEM_WRITEBACK_BYPASS (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_writeback_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_writeback_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_mem_writeback_bypass`] module"]
pub type EXT_MEM_WRITEBACK_BYPASS =
    crate::Reg<ext_mem_writeback_bypass::EXT_MEM_WRITEBACK_BYPASS_SPEC>;
#[doc = "******* Description ***********"]
pub mod ext_mem_writeback_bypass;
#[doc = "FLASH_ACE0_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_attr`] module"]
pub type FLASH_ACE0_ATTR = crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace0_attr;
#[doc = "FLASH_ACE1_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_attr`] module"]
pub type FLASH_ACE1_ATTR = crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace1_attr;
#[doc = "FLASH_ACE2_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_attr`] module"]
pub type FLASH_ACE2_ATTR = crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace2_attr;
#[doc = "FLASH_ACE3_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_attr`] module"]
pub type FLASH_ACE3_ATTR = crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace3_attr;
#[doc = "FLASH_ACE0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_addr`] module"]
pub type FLASH_ACE0_ADDR = crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace0_addr;
#[doc = "FLASH_ACE1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_addr`] module"]
pub type FLASH_ACE1_ADDR = crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace1_addr;
#[doc = "FLASH_ACE2_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_addr`] module"]
pub type FLASH_ACE2_ADDR = crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace2_addr;
#[doc = "FLASH_ACE3_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_addr`] module"]
pub type FLASH_ACE3_ADDR = crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace3_addr;
#[doc = "FLASH_ACE0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace0_size`] module"]
pub type FLASH_ACE0_SIZE = crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace0_size;
#[doc = "FLASH_ACE1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace1_size`] module"]
pub type FLASH_ACE1_SIZE = crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace1_size;
#[doc = "FLASH_ACE2_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace2_size`] module"]
pub type FLASH_ACE2_SIZE = crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace2_size;
#[doc = "FLASH_ACE3_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`flash_ace3_size`] module"]
pub type FLASH_ACE3_SIZE = crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod flash_ace3_size;
#[doc = "SRAM_ACE0_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace0_attr`] module"]
pub type SRAM_ACE0_ATTR = crate::Reg<sram_ace0_attr::SRAM_ACE0_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace0_attr;
#[doc = "SRAM_ACE1_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace1_attr`] module"]
pub type SRAM_ACE1_ATTR = crate::Reg<sram_ace1_attr::SRAM_ACE1_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace1_attr;
#[doc = "SRAM_ACE2_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace2_attr`] module"]
pub type SRAM_ACE2_ATTR = crate::Reg<sram_ace2_attr::SRAM_ACE2_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace2_attr;
#[doc = "SRAM_ACE3_ATTR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace3_attr`] module"]
pub type SRAM_ACE3_ATTR = crate::Reg<sram_ace3_attr::SRAM_ACE3_ATTR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace3_attr;
#[doc = "SRAM_ACE0_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace0_addr`] module"]
pub type SRAM_ACE0_ADDR = crate::Reg<sram_ace0_addr::SRAM_ACE0_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace0_addr;
#[doc = "SRAM_ACE1_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace1_addr`] module"]
pub type SRAM_ACE1_ADDR = crate::Reg<sram_ace1_addr::SRAM_ACE1_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace1_addr;
#[doc = "SRAM_ACE2_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace2_addr`] module"]
pub type SRAM_ACE2_ADDR = crate::Reg<sram_ace2_addr::SRAM_ACE2_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace2_addr;
#[doc = "SRAM_ACE3_ADDR (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace3_addr`] module"]
pub type SRAM_ACE3_ADDR = crate::Reg<sram_ace3_addr::SRAM_ACE3_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace3_addr;
#[doc = "SRAM_ACE0_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace0_size`] module"]
pub type SRAM_ACE0_SIZE = crate::Reg<sram_ace0_size::SRAM_ACE0_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace0_size;
#[doc = "SRAM_ACE1_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace1_size`] module"]
pub type SRAM_ACE1_SIZE = crate::Reg<sram_ace1_size::SRAM_ACE1_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace1_size;
#[doc = "SRAM_ACE2_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace2_size`] module"]
pub type SRAM_ACE2_SIZE = crate::Reg<sram_ace2_size::SRAM_ACE2_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace2_size;
#[doc = "SRAM_ACE3_SIZE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_ace3_size`] module"]
pub type SRAM_ACE3_SIZE = crate::Reg<sram_ace3_size::SRAM_ACE3_SIZE_SPEC>;
#[doc = "******* Description ***********"]
pub mod sram_ace3_size;
#[doc = "SPI_MEM_PMS_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_pms_ctrl`] module"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod spi_mem_pms_ctrl;
#[doc = "SPI_MEM_REJECT_ADDR (r) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_reject_addr`] module"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>;
#[doc = "******* Description ***********"]
pub mod spi_mem_reject_addr;
#[doc = "SDIO_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_ctrl`] module"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod sdio_ctrl;
#[doc = "REDCY_SIG0 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`redcy_sig0`] module"]
pub type REDCY_SIG0 = crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>;
#[doc = "******* Description ***********"]
pub mod redcy_sig0;
#[doc = "REDCY_SIG1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`redcy_sig1`] module"]
pub type REDCY_SIG1 = crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>;
#[doc = "******* Description ***********"]
pub mod redcy_sig1;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`front_end_mem_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`front_end_mem_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`front_end_mem_pd`] module"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = "******* Description ***********"]
pub mod front_end_mem_pd;
#[doc = "SPI_MEM_ECC_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_mem_ecc_ctrl`] module"]
pub type SPI_MEM_ECC_CTRL = crate::Reg<spi_mem_ecc_ctrl::SPI_MEM_ECC_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod spi_mem_ecc_ctrl;
#[doc = "CLKGATE_FORCE_ON (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_force_on::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_force_on::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkgate_force_on`] module"]
pub type CLKGATE_FORCE_ON = crate::Reg<clkgate_force_on::CLKGATE_FORCE_ON_SPEC>;
#[doc = "******* Description ***********"]
pub mod clkgate_force_on;
#[doc = "MEM_POWER_DOWN (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_power_down::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_down::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_power_down`] module"]
pub type MEM_POWER_DOWN = crate::Reg<mem_power_down::MEM_POWER_DOWN_SPEC>;
#[doc = "******* Description ***********"]
pub mod mem_power_down;
#[doc = "MEM_POWER_UP (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_power_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mem_power_up`] module"]
pub type MEM_POWER_UP = crate::Reg<mem_power_up::MEM_POWER_UP_SPEC>;
#[doc = "******* Description ***********"]
pub mod mem_power_up;
#[doc = "RETENTION_CTRL (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl`] module"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl;
#[doc = "RETENTION_CTRL1 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl1`] module"]
pub type RETENTION_CTRL1 = crate::Reg<retention_ctrl1::RETENTION_CTRL1_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl1;
#[doc = "RETENTION_CTRL2 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl2`] module"]
pub type RETENTION_CTRL2 = crate::Reg<retention_ctrl2::RETENTION_CTRL2_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl2;
#[doc = "RETENTION_CTRL3 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl3`] module"]
pub type RETENTION_CTRL3 = crate::Reg<retention_ctrl3::RETENTION_CTRL3_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl3;
#[doc = "RETENTION_CTRL4 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl4`] module"]
pub type RETENTION_CTRL4 = crate::Reg<retention_ctrl4::RETENTION_CTRL4_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl4;
#[doc = "RETENTION_CTRL5 (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`retention_ctrl5`] module"]
pub type RETENTION_CTRL5 = crate::Reg<retention_ctrl5::RETENTION_CTRL5_SPEC>;
#[doc = "******* Description ***********"]
pub mod retention_ctrl5;
#[doc = "DATE (rw) register accessor: ******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod date;
