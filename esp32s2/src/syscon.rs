#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    sysclk_conf: SYSCLK_CONF,
    tick_conf: TICK_CONF,
    clk_out_en: CLK_OUT_EN,
    host_inf_sel: HOST_INF_SEL,
    ext_mem_pms_lock: EXT_MEM_PMS_LOCK,
    flash_ace0_attr: FLASH_ACE0_ATTR,
    flash_ace1_attr: FLASH_ACE1_ATTR,
    flash_ace2_attr: FLASH_ACE2_ATTR,
    flash_ace3_attr: FLASH_ACE3_ATTR,
    flash_ace0_addr: FLASH_ACE0_ADDR,
    flash_ace1_addr: FLASH_ACE1_ADDR,
    flash_ace2_addr: FLASH_ACE2_ADDR,
    flash_ace3_addr: FLASH_ACE3_ADDR,
    flash_ace0_size: FLASH_ACE0_SIZE,
    flash_ace1_size: FLASH_ACE1_SIZE,
    flash_ace2_size: FLASH_ACE2_SIZE,
    flash_ace3_size: FLASH_ACE3_SIZE,
    sram_ace0_attr: SRAM_ACE0_ATTR,
    sram_ace1_attr: SRAM_ACE1_ATTR,
    sram_ace2_attr: SRAM_ACE2_ATTR,
    sram_ace3_attr: SRAM_ACE3_ATTR,
    sram_ace0_addr: SRAM_ACE0_ADDR,
    sram_ace1_addr: SRAM_ACE1_ADDR,
    sram_ace2_addr: SRAM_ACE2_ADDR,
    sram_ace3_addr: SRAM_ACE3_ADDR,
    sram_ace0_size: SRAM_ACE0_SIZE,
    sram_ace1_size: SRAM_ACE1_SIZE,
    sram_ace2_size: SRAM_ACE2_SIZE,
    sram_ace3_size: SRAM_ACE3_SIZE,
    spi_mem_pms_ctrl: SPI_MEM_PMS_CTRL,
    spi_mem_reject_addr: SPI_MEM_REJECT_ADDR,
    sdio_ctrl: SDIO_CTRL,
    redcy_sig0: REDCY_SIG0,
    redcy_sig1: REDCY_SIG1,
    wifi_bb_cfg: WIFI_BB_CFG,
    wifi_bb_cfg_2: WIFI_BB_CFG_2,
    wifi_clk_en: WIFI_CLK_EN,
    wifi_rst_en: WIFI_RST_EN,
    front_end_mem_pd: FRONT_END_MEM_PD,
    _reserved39: [u8; 0x0360],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn tick_conf(&self) -> &TICK_CONF {
        &self.tick_conf
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn clk_out_en(&self) -> &CLK_OUT_EN {
        &self.clk_out_en
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn host_inf_sel(&self) -> &HOST_INF_SEL {
        &self.host_inf_sel
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn ext_mem_pms_lock(&self) -> &EXT_MEM_PMS_LOCK {
        &self.ext_mem_pms_lock
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn flash_ace0_attr(&self) -> &FLASH_ACE0_ATTR {
        &self.flash_ace0_attr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn flash_ace1_attr(&self) -> &FLASH_ACE1_ATTR {
        &self.flash_ace1_attr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn flash_ace2_attr(&self) -> &FLASH_ACE2_ATTR {
        &self.flash_ace2_attr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn flash_ace3_attr(&self) -> &FLASH_ACE3_ATTR {
        &self.flash_ace3_attr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn flash_ace0_addr(&self) -> &FLASH_ACE0_ADDR {
        &self.flash_ace0_addr
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn flash_ace1_addr(&self) -> &FLASH_ACE1_ADDR {
        &self.flash_ace1_addr
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn flash_ace2_addr(&self) -> &FLASH_ACE2_ADDR {
        &self.flash_ace2_addr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn flash_ace3_addr(&self) -> &FLASH_ACE3_ADDR {
        &self.flash_ace3_addr
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn flash_ace0_size(&self) -> &FLASH_ACE0_SIZE {
        &self.flash_ace0_size
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn flash_ace1_size(&self) -> &FLASH_ACE1_SIZE {
        &self.flash_ace1_size
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn flash_ace2_size(&self) -> &FLASH_ACE2_SIZE {
        &self.flash_ace2_size
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn flash_ace3_size(&self) -> &FLASH_ACE3_SIZE {
        &self.flash_ace3_size
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn sram_ace0_attr(&self) -> &SRAM_ACE0_ATTR {
        &self.sram_ace0_attr
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn sram_ace1_attr(&self) -> &SRAM_ACE1_ATTR {
        &self.sram_ace1_attr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn sram_ace2_attr(&self) -> &SRAM_ACE2_ATTR {
        &self.sram_ace2_attr
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn sram_ace3_attr(&self) -> &SRAM_ACE3_ATTR {
        &self.sram_ace3_attr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn sram_ace0_addr(&self) -> &SRAM_ACE0_ADDR {
        &self.sram_ace0_addr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn sram_ace1_addr(&self) -> &SRAM_ACE1_ADDR {
        &self.sram_ace1_addr
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn sram_ace2_addr(&self) -> &SRAM_ACE2_ADDR {
        &self.sram_ace2_addr
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn sram_ace3_addr(&self) -> &SRAM_ACE3_ADDR {
        &self.sram_ace3_addr
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn sram_ace0_size(&self) -> &SRAM_ACE0_SIZE {
        &self.sram_ace0_size
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn sram_ace1_size(&self) -> &SRAM_ACE1_SIZE {
        &self.sram_ace1_size
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn sram_ace2_size(&self) -> &SRAM_ACE2_SIZE {
        &self.sram_ace2_size
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn sram_ace3_size(&self) -> &SRAM_ACE3_SIZE {
        &self.sram_ace3_size
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn spi_mem_pms_ctrl(&self) -> &SPI_MEM_PMS_CTRL {
        &self.spi_mem_pms_ctrl
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn spi_mem_reject_addr(&self) -> &SPI_MEM_REJECT_ADDR {
        &self.spi_mem_reject_addr
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn sdio_ctrl(&self) -> &SDIO_CTRL {
        &self.sdio_ctrl
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn redcy_sig0(&self) -> &REDCY_SIG0 {
        &self.redcy_sig0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn redcy_sig1(&self) -> &REDCY_SIG1 {
        &self.redcy_sig1
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn wifi_bb_cfg(&self) -> &WIFI_BB_CFG {
        &self.wifi_bb_cfg
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn wifi_bb_cfg_2(&self) -> &WIFI_BB_CFG_2 {
        &self.wifi_bb_cfg_2
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn wifi_clk_en(&self) -> &WIFI_CLK_EN {
        &self.wifi_clk_en
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn wifi_rst_en(&self) -> &WIFI_RST_EN {
        &self.wifi_rst_en
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn front_end_mem_pd(&self) -> &FRONT_END_MEM_PD {
        &self.front_end_mem_pd
    }
    #[doc = "0x3fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SYSCLK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclk_conf`] module"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = ""]
pub mod sysclk_conf;
#[doc = "TICK_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tick_conf`] module"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = ""]
pub mod tick_conf;
#[doc = "CLK_OUT_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_out_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_out_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_out_en`] module"]
pub type CLK_OUT_EN = crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>;
#[doc = ""]
pub mod clk_out_en;
#[doc = "HOST_INF_SEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_inf_sel`] module"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = ""]
pub mod host_inf_sel;
#[doc = "EXT_MEM_PMS_LOCK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_mem_pms_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_mem_pms_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_mem_pms_lock`] module"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>;
#[doc = ""]
pub mod ext_mem_pms_lock;
#[doc = "FLASH_ACE0_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace0_attr`] module"]
pub type FLASH_ACE0_ATTR = crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace0_attr;
#[doc = "FLASH_ACE1_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace1_attr`] module"]
pub type FLASH_ACE1_ATTR = crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace1_attr;
#[doc = "FLASH_ACE2_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace2_attr`] module"]
pub type FLASH_ACE2_ATTR = crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace2_attr;
#[doc = "FLASH_ACE3_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace3_attr`] module"]
pub type FLASH_ACE3_ATTR = crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>;
#[doc = ""]
pub mod flash_ace3_attr;
#[doc = "FLASH_ACE0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace0_addr`] module"]
pub type FLASH_ACE0_ADDR = crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace0_addr;
#[doc = "FLASH_ACE1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace1_addr`] module"]
pub type FLASH_ACE1_ADDR = crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace1_addr;
#[doc = "FLASH_ACE2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace2_addr`] module"]
pub type FLASH_ACE2_ADDR = crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace2_addr;
#[doc = "FLASH_ACE3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace3_addr`] module"]
pub type FLASH_ACE3_ADDR = crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>;
#[doc = ""]
pub mod flash_ace3_addr;
#[doc = "FLASH_ACE0_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace0_size`] module"]
pub type FLASH_ACE0_SIZE = crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace0_size;
#[doc = "FLASH_ACE1_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace1_size`] module"]
pub type FLASH_ACE1_SIZE = crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace1_size;
#[doc = "FLASH_ACE2_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace2_size`] module"]
pub type FLASH_ACE2_SIZE = crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace2_size;
#[doc = "FLASH_ACE3_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ace3_size`] module"]
pub type FLASH_ACE3_SIZE = crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>;
#[doc = ""]
pub mod flash_ace3_size;
#[doc = "SRAM_ACE0_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace0_attr`] module"]
pub type SRAM_ACE0_ATTR = crate::Reg<sram_ace0_attr::SRAM_ACE0_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace0_attr;
#[doc = "SRAM_ACE1_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace1_attr`] module"]
pub type SRAM_ACE1_ATTR = crate::Reg<sram_ace1_attr::SRAM_ACE1_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace1_attr;
#[doc = "SRAM_ACE2_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace2_attr`] module"]
pub type SRAM_ACE2_ATTR = crate::Reg<sram_ace2_attr::SRAM_ACE2_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace2_attr;
#[doc = "SRAM_ACE3_ATTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_attr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_attr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace3_attr`] module"]
pub type SRAM_ACE3_ATTR = crate::Reg<sram_ace3_attr::SRAM_ACE3_ATTR_SPEC>;
#[doc = ""]
pub mod sram_ace3_attr;
#[doc = "SRAM_ACE0_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace0_addr`] module"]
pub type SRAM_ACE0_ADDR = crate::Reg<sram_ace0_addr::SRAM_ACE0_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace0_addr;
#[doc = "SRAM_ACE1_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace1_addr`] module"]
pub type SRAM_ACE1_ADDR = crate::Reg<sram_ace1_addr::SRAM_ACE1_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace1_addr;
#[doc = "SRAM_ACE2_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace2_addr`] module"]
pub type SRAM_ACE2_ADDR = crate::Reg<sram_ace2_addr::SRAM_ACE2_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace2_addr;
#[doc = "SRAM_ACE3_ADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace3_addr`] module"]
pub type SRAM_ACE3_ADDR = crate::Reg<sram_ace3_addr::SRAM_ACE3_ADDR_SPEC>;
#[doc = ""]
pub mod sram_ace3_addr;
#[doc = "SRAM_ACE0_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace0_size`] module"]
pub type SRAM_ACE0_SIZE = crate::Reg<sram_ace0_size::SRAM_ACE0_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace0_size;
#[doc = "SRAM_ACE1_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace1_size`] module"]
pub type SRAM_ACE1_SIZE = crate::Reg<sram_ace1_size::SRAM_ACE1_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace1_size;
#[doc = "SRAM_ACE2_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace2_size`] module"]
pub type SRAM_ACE2_SIZE = crate::Reg<sram_ace2_size::SRAM_ACE2_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace2_size;
#[doc = "SRAM_ACE3_SIZE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_ace3_size`] module"]
pub type SRAM_ACE3_SIZE = crate::Reg<sram_ace3_size::SRAM_ACE3_SIZE_SPEC>;
#[doc = ""]
pub mod sram_ace3_size;
#[doc = "SPI_MEM_PMS_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_pms_ctrl`] module"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>;
#[doc = ""]
pub mod spi_mem_pms_ctrl;
#[doc = "SPI_MEM_REJECT_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_reject_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_mem_reject_addr`] module"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>;
#[doc = ""]
pub mod spi_mem_reject_addr;
#[doc = "SDIO_CTRL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_ctrl`] module"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = ""]
pub mod sdio_ctrl;
#[doc = "REDCY_SIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@redcy_sig0`] module"]
pub type REDCY_SIG0 = crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>;
#[doc = ""]
pub mod redcy_sig0;
#[doc = "REDCY_SIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@redcy_sig1`] module"]
pub type REDCY_SIG1 = crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>;
#[doc = ""]
pub mod redcy_sig1;
#[doc = "WIFI_BB_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_cfg`] module"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_bb_cfg_2`] module"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = ""]
pub mod wifi_bb_cfg_2;
#[doc = "WIFI_CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_clk_en`] module"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = ""]
pub mod wifi_clk_en;
#[doc = "WIFI_RST_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_rst_en`] module"]
pub type WIFI_RST_EN = crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>;
#[doc = ""]
pub mod wifi_rst_en;
#[doc = "FRONT_END_MEM_PD (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`front_end_mem_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`front_end_mem_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@front_end_mem_pd`] module"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = ""]
pub mod front_end_mem_pd;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
