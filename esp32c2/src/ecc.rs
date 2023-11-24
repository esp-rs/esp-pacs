#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    mult_int_raw: MULT_INT_RAW,
    mult_int_st: MULT_INT_ST,
    mult_int_ena: MULT_INT_ENA,
    mult_int_clr: MULT_INT_CLR,
    mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    mult_date: MULT_DATE,
    k_mem: [K_MEM; 32],
    px_mem: [PX_MEM; 32],
    py_mem: [PY_MEM; 32],
}
impl RegisterBlock {
    #[doc = "0x0c - I2S interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn mult_int_raw(&self) -> &MULT_INT_RAW {
        &self.mult_int_raw
    }
    #[doc = "0x10 - I2S interrupt status register."]
    #[inline(always)]
    pub const fn mult_int_st(&self) -> &MULT_INT_ST {
        &self.mult_int_st
    }
    #[doc = "0x14 - I2S interrupt enable register."]
    #[inline(always)]
    pub const fn mult_int_ena(&self) -> &MULT_INT_ENA {
        &self.mult_int_ena
    }
    #[doc = "0x18 - I2S interrupt clear register."]
    #[inline(always)]
    pub const fn mult_int_clr(&self) -> &MULT_INT_CLR {
        &self.mult_int_clr
    }
    #[doc = "0x1c - I2S RX configure register"]
    #[inline(always)]
    pub const fn mult_conf(&self) -> &MULT_CONF {
        &self.mult_conf
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn mult_date(&self) -> &MULT_DATE {
        &self.mult_date
    }
    #[doc = "0x100..0x120 - The memory that stores k."]
    #[inline(always)]
    pub const fn k_mem(&self, n: usize) -> &K_MEM {
        &self.k_mem[n]
    }
    #[doc = "0x120..0x140 - The memory that stores Px."]
    #[inline(always)]
    pub const fn px_mem(&self, n: usize) -> &PX_MEM {
        &self.px_mem[n]
    }
    #[doc = "0x140..0x160 - The memory that stores Py."]
    #[inline(always)]
    pub const fn py_mem(&self, n: usize) -> &PY_MEM {
        &self.py_mem[n]
    }
}
#[doc = "MULT_INT_RAW (r) register accessor: I2S interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_raw`] module"]
pub type MULT_INT_RAW = crate::Reg<mult_int_raw::MULT_INT_RAW_SPEC>;
#[doc = "I2S interrupt raw register, valid in level."]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: I2S interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_st`] module"]
pub type MULT_INT_ST = crate::Reg<mult_int_st::MULT_INT_ST_SPEC>;
#[doc = "I2S interrupt status register."]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: I2S interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_ena`] module"]
pub type MULT_INT_ENA = crate::Reg<mult_int_ena::MULT_INT_ENA_SPEC>;
#[doc = "I2S interrupt enable register."]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: I2S interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_int_clr`] module"]
pub type MULT_INT_CLR = crate::Reg<mult_int_clr::MULT_INT_CLR_SPEC>;
#[doc = "I2S interrupt clear register."]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: I2S RX configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_conf`] module"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "I2S RX configure register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_date`] module"]
pub type MULT_DATE = crate::Reg<mult_date::MULT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod mult_date;
#[doc = "K_MEM (rw) register accessor: The memory that stores k.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`k_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k_mem`] module"]
pub type K_MEM = crate::Reg<k_mem::K_MEM_SPEC>;
#[doc = "The memory that stores k."]
pub mod k_mem;
#[doc = "PX_MEM (rw) register accessor: The memory that stores Px.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`px_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`px_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_mem`] module"]
pub type PX_MEM = crate::Reg<px_mem::PX_MEM_SPEC>;
#[doc = "The memory that stores Px."]
pub mod px_mem;
#[doc = "PY_MEM (rw) register accessor: The memory that stores Py.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`py_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`py_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@py_mem`] module"]
pub type PY_MEM = crate::Reg<py_mem::PY_MEM_SPEC>;
#[doc = "The memory that stores Py."]
pub mod py_mem;
