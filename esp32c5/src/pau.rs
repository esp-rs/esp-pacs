#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    regdma_conf: REGDMA_CONF,
    regdma_clk_conf: REGDMA_CLK_CONF,
    regdma_etm_ctrl: REGDMA_ETM_CTRL,
    regdma_current_link_addr: REGDMA_CURRENT_LINK_ADDR,
    regdma_peri_addr: REGDMA_PERI_ADDR,
    regdma_mem_addr: REGDMA_MEM_ADDR,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    _reserved10: [u8; 0x03d4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Peri backup control register"]
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &REGDMA_CONF {
        &self.regdma_conf
    }
    #[doc = "0x04 - Clock control register"]
    #[inline(always)]
    pub const fn regdma_clk_conf(&self) -> &REGDMA_CLK_CONF {
        &self.regdma_clk_conf
    }
    #[doc = "0x08 - ETM start ctrl reg"]
    #[inline(always)]
    pub const fn regdma_etm_ctrl(&self) -> &REGDMA_ETM_CTRL {
        &self.regdma_etm_ctrl
    }
    #[doc = "0x0c - current link addr"]
    #[inline(always)]
    pub const fn regdma_current_link_addr(&self) -> &REGDMA_CURRENT_LINK_ADDR {
        &self.regdma_current_link_addr
    }
    #[doc = "0x10 - Backup addr"]
    #[inline(always)]
    pub const fn regdma_peri_addr(&self) -> &REGDMA_PERI_ADDR {
        &self.regdma_peri_addr
    }
    #[doc = "0x14 - mem addr"]
    #[inline(always)]
    pub const fn regdma_mem_addr(&self) -> &REGDMA_MEM_ADDR {
        &self.regdma_mem_addr
    }
    #[doc = "0x18 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x1c - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x20 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x24 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x3fc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "REGDMA_CONF (rw) register accessor: Peri backup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_conf`] module"]
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
#[doc = "Peri backup control register"]
pub mod regdma_conf;
#[doc = "REGDMA_CLK_CONF (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_clk_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_clk_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_clk_conf`] module"]
pub type REGDMA_CLK_CONF = crate::Reg<regdma_clk_conf::REGDMA_CLK_CONF_SPEC>;
#[doc = "Clock control register"]
pub mod regdma_clk_conf;
#[doc = "REGDMA_ETM_CTRL (rw) register accessor: ETM start ctrl reg\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_etm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_etm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_etm_ctrl`] module"]
pub type REGDMA_ETM_CTRL = crate::Reg<regdma_etm_ctrl::REGDMA_ETM_CTRL_SPEC>;
#[doc = "ETM start ctrl reg"]
pub mod regdma_etm_ctrl;
#[doc = "REGDMA_CURRENT_LINK_ADDR (r) register accessor: current link addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_current_link_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_current_link_addr`] module"]
pub type REGDMA_CURRENT_LINK_ADDR =
    crate::Reg<regdma_current_link_addr::REGDMA_CURRENT_LINK_ADDR_SPEC>;
#[doc = "current link addr"]
pub mod regdma_current_link_addr;
#[doc = "REGDMA_PERI_ADDR (r) register accessor: Backup addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_peri_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_peri_addr`] module"]
pub type REGDMA_PERI_ADDR = crate::Reg<regdma_peri_addr::REGDMA_PERI_ADDR_SPEC>;
#[doc = "Backup addr"]
pub mod regdma_peri_addr;
#[doc = "REGDMA_MEM_ADDR (r) register accessor: mem addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_mem_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regdma_mem_addr`] module"]
pub type REGDMA_MEM_ADDR = crate::Reg<regdma_mem_addr::REGDMA_MEM_ADDR_SPEC>;
#[doc = "mem addr"]
pub mod regdma_mem_addr;
#[doc = "INT_ENA (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: Read only register for error and done\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_st;
pub use crate::aes::date;
pub use crate::aes::DATE;
