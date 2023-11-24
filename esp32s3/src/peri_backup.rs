#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    config: CONFIG,
    apb_addr: APB_ADDR,
    mem_addr: MEM_ADDR,
    reg_map0: REG_MAP0,
    reg_map1: REG_MAP1,
    reg_map2: REG_MAP2,
    reg_map3: REG_MAP3,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved11: [u8; 0xd0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - x"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - x"]
    #[inline(always)]
    pub const fn apb_addr(&self) -> &APB_ADDR {
        &self.apb_addr
    }
    #[doc = "0x08 - x"]
    #[inline(always)]
    pub const fn mem_addr(&self) -> &MEM_ADDR {
        &self.mem_addr
    }
    #[doc = "0x0c - x"]
    #[inline(always)]
    pub const fn reg_map0(&self) -> &REG_MAP0 {
        &self.reg_map0
    }
    #[doc = "0x10 - x"]
    #[inline(always)]
    pub const fn reg_map1(&self) -> &REG_MAP1 {
        &self.reg_map1
    }
    #[doc = "0x14 - x"]
    #[inline(always)]
    pub const fn reg_map2(&self) -> &REG_MAP2 {
        &self.reg_map2
    }
    #[doc = "0x18 - x"]
    #[inline(always)]
    pub const fn reg_map3(&self) -> &REG_MAP3 {
        &self.reg_map3
    }
    #[doc = "0x1c - x"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x20 - x"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x24 - x"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x28 - x"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xfc - x"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONFIG (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "x"]
pub mod config;
#[doc = "APB_ADDR (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_addr`] module"]
pub type APB_ADDR = crate::Reg<apb_addr::APB_ADDR_SPEC>;
#[doc = "x"]
pub mod apb_addr;
#[doc = "MEM_ADDR (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_addr`] module"]
pub type MEM_ADDR = crate::Reg<mem_addr::MEM_ADDR_SPEC>;
#[doc = "x"]
pub mod mem_addr;
#[doc = "REG_MAP0 (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_map0`] module"]
pub type REG_MAP0 = crate::Reg<reg_map0::REG_MAP0_SPEC>;
#[doc = "x"]
pub mod reg_map0;
#[doc = "REG_MAP1 (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_map1`] module"]
pub type REG_MAP1 = crate::Reg<reg_map1::REG_MAP1_SPEC>;
#[doc = "x"]
pub mod reg_map1;
#[doc = "REG_MAP2 (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_map2`] module"]
pub type REG_MAP2 = crate::Reg<reg_map2::REG_MAP2_SPEC>;
#[doc = "x"]
pub mod reg_map2;
#[doc = "REG_MAP3 (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_map3`] module"]
pub type REG_MAP3 = crate::Reg<reg_map3::REG_MAP3_SPEC>;
#[doc = "x"]
pub mod reg_map3;
#[doc = "INT_RAW (r) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "x"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "x"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "x"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: x\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "x"]
pub mod int_clr;
#[doc = "DATE (rw) register accessor: x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "x"]
pub mod date;
