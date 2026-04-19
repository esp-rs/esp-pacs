#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    mult_int_raw: MULT_INT_RAW,
    mult_int_st: MULT_INT_ST,
    mult_int_ena: MULT_INT_ENA,
    mult_int_clr: MULT_INT_CLR,
    mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    mult_date: MULT_DATE,
    mult_k_mem: [MULT_K_MEM; 48],
    mult_px_mem: [MULT_PX_MEM; 48],
    mult_py_mem: [MULT_PY_MEM; 48],
    mult_qx_mem: [MULT_QX_MEM; 48],
    mult_qy_mem: [MULT_QY_MEM; 48],
    mult_qz_mem: [MULT_QZ_MEM; 48],
}
impl RegisterBlock {
    #[doc = "0x0c - ECC raw interrupt status register"]
    #[inline(always)]
    pub const fn mult_int_raw(&self) -> &MULT_INT_RAW {
        &self.mult_int_raw
    }
    #[doc = "0x10 - ECC masked interrupt status register"]
    #[inline(always)]
    pub const fn mult_int_st(&self) -> &MULT_INT_ST {
        &self.mult_int_st
    }
    #[doc = "0x14 - ECC interrupt enable register"]
    #[inline(always)]
    pub const fn mult_int_ena(&self) -> &MULT_INT_ENA {
        &self.mult_int_ena
    }
    #[doc = "0x18 - ECC interrupt clear register"]
    #[inline(always)]
    pub const fn mult_int_clr(&self) -> &MULT_INT_CLR {
        &self.mult_int_clr
    }
    #[doc = "0x1c - ECC configuration register"]
    #[inline(always)]
    pub const fn mult_conf(&self) -> &MULT_CONF {
        &self.mult_conf
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn mult_date(&self) -> &MULT_DATE {
        &self.mult_date
    }
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub const fn mult_k_mem(&self, n: usize) -> &MULT_K_MEM {
        &self.mult_k_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub fn mult_k_mem_iter(&self) -> impl Iterator<Item = &MULT_K_MEM> {
        self.mult_k_mem.iter()
    }
    #[doc = "0x130..0x160 - The memory that stores Px."]
    #[inline(always)]
    pub const fn mult_px_mem(&self, n: usize) -> &MULT_PX_MEM {
        &self.mult_px_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x160 - The memory that stores Px."]
    #[inline(always)]
    pub fn mult_px_mem_iter(&self) -> impl Iterator<Item = &MULT_PX_MEM> {
        self.mult_px_mem.iter()
    }
    #[doc = "0x160..0x190 - The memory that stores Py."]
    #[inline(always)]
    pub const fn mult_py_mem(&self, n: usize) -> &MULT_PY_MEM {
        &self.mult_py_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x190 - The memory that stores Py."]
    #[inline(always)]
    pub fn mult_py_mem_iter(&self) -> impl Iterator<Item = &MULT_PY_MEM> {
        self.mult_py_mem.iter()
    }
    #[doc = "0x190..0x1c0 - The memory that stores Qx."]
    #[inline(always)]
    pub const fn mult_qx_mem(&self, n: usize) -> &MULT_QX_MEM {
        &self.mult_qx_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1c0 - The memory that stores Qx."]
    #[inline(always)]
    pub fn mult_qx_mem_iter(&self) -> impl Iterator<Item = &MULT_QX_MEM> {
        self.mult_qx_mem.iter()
    }
    #[doc = "0x1c0..0x1f0 - The memory that stores Qy."]
    #[inline(always)]
    pub const fn mult_qy_mem(&self, n: usize) -> &MULT_QY_MEM {
        &self.mult_qy_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1f0 - The memory that stores Qy."]
    #[inline(always)]
    pub fn mult_qy_mem_iter(&self) -> impl Iterator<Item = &MULT_QY_MEM> {
        self.mult_qy_mem.iter()
    }
    #[doc = "0x1f0..0x220 - The memory that stores Qz."]
    #[inline(always)]
    pub const fn mult_qz_mem(&self, n: usize) -> &MULT_QZ_MEM {
        &self.mult_qz_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1f0..0x220 - The memory that stores Qz."]
    #[inline(always)]
    pub fn mult_qz_mem_iter(&self) -> impl Iterator<Item = &MULT_QZ_MEM> {
        self.mult_qz_mem.iter()
    }
}
#[doc = "MULT_INT_RAW (r) register accessor: ECC raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_raw`] module"]
pub type MULT_INT_RAW = crate::Reg<mult_int_raw::MULT_INT_RAW_SPEC>;
#[doc = "ECC raw interrupt status register"]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: ECC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_st`] module"]
pub type MULT_INT_ST = crate::Reg<mult_int_st::MULT_INT_ST_SPEC>;
#[doc = "ECC masked interrupt status register"]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_ena`] module"]
pub type MULT_INT_ENA = crate::Reg<mult_int_ena::MULT_INT_ENA_SPEC>;
#[doc = "ECC interrupt enable register"]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: ECC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_clr`] module"]
pub type MULT_INT_CLR = crate::Reg<mult_int_clr::MULT_INT_CLR_SPEC>;
#[doc = "ECC interrupt clear register"]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_conf`] module"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "ECC configuration register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_date`] module"]
pub type MULT_DATE = crate::Reg<mult_date::MULT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod mult_date;
#[doc = "MULT_K_MEM (rw) register accessor: The memory that stores k.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_k_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_k_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_k_mem`] module"]
pub type MULT_K_MEM = crate::Reg<mult_k_mem::MULT_K_MEM_SPEC>;
#[doc = "The memory that stores k."]
pub mod mult_k_mem;
#[doc = "MULT_PX_MEM (rw) register accessor: The memory that stores Px.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_px_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_px_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_px_mem`] module"]
pub type MULT_PX_MEM = crate::Reg<mult_px_mem::MULT_PX_MEM_SPEC>;
#[doc = "The memory that stores Px."]
pub mod mult_px_mem;
#[doc = "MULT_PY_MEM (rw) register accessor: The memory that stores Py.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_py_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_py_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_py_mem`] module"]
pub type MULT_PY_MEM = crate::Reg<mult_py_mem::MULT_PY_MEM_SPEC>;
#[doc = "The memory that stores Py."]
pub mod mult_py_mem;
#[doc = "MULT_QX_MEM (rw) register accessor: The memory that stores Qx.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_qx_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_qx_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_qx_mem`] module"]
pub type MULT_QX_MEM = crate::Reg<mult_qx_mem::MULT_QX_MEM_SPEC>;
#[doc = "The memory that stores Qx."]
pub mod mult_qx_mem;
#[doc = "MULT_QY_MEM (rw) register accessor: The memory that stores Qy.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_qy_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_qy_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_qy_mem`] module"]
pub type MULT_QY_MEM = crate::Reg<mult_qy_mem::MULT_QY_MEM_SPEC>;
#[doc = "The memory that stores Qy."]
pub mod mult_qy_mem;
#[doc = "MULT_QZ_MEM (rw) register accessor: The memory that stores Qz.\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_qz_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_qz_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_qz_mem`] module"]
pub type MULT_QZ_MEM = crate::Reg<mult_qz_mem::MULT_QZ_MEM_SPEC>;
#[doc = "The memory that stores Qz."]
pub mod mult_qz_mem;
