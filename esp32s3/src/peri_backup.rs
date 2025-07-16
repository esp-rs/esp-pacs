#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: CONFIG,
    apb_addr: APB_ADDR,
    mem_addr: MEM_ADDR,
    reg_map: [REG_MAP; 4],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved8: [u8; 0xd0],
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
    #[doc = "0x0c..0x1c - x"]
    #[inline(always)]
    pub const fn reg_map(&self, n: usize) -> &REG_MAP {
        &self.reg_map[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - x"]
    #[inline(always)]
    pub fn reg_map_iter(&self) -> impl Iterator<Item = &REG_MAP> {
        self.reg_map.iter()
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
#[doc = "CONFIG (rw) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "x"]
pub mod config;
#[doc = "APB_ADDR (rw) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_addr`] module"]
pub type APB_ADDR = crate::Reg<apb_addr::APB_ADDR_SPEC>;
#[doc = "x"]
pub mod apb_addr;
#[doc = "MEM_ADDR (rw) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_addr`] module"]
pub type MEM_ADDR = crate::Reg<mem_addr::MEM_ADDR_SPEC>;
#[doc = "x"]
pub mod mem_addr;
#[doc = "REG_MAP (rw) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_map::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_map::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_map`] module"]
pub type REG_MAP = crate::Reg<reg_map::REG_MAP_SPEC>;
#[doc = "x"]
pub mod reg_map;
#[doc = "INT_RAW (r) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "x"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "x"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: x\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "x"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: x\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "x"]
pub mod int_clr;
pub use crate::aes::date;
pub use crate::aes::DATE;
