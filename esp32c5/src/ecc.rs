#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    date: DATE,
    _reserved_6_k_mem: [u8; 0xd0],
}
impl RegisterBlock {
    #[doc = "0x0c - ECC raw interrupt status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x10 - ECC masked interrupt status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x14 - ECC interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18 - ECC interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1c - ECC configuration register"]
    #[inline(always)]
    pub const fn mult_conf(&self) -> &MULT_CONF {
        &self.mult_conf
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub const fn k_mem(&self, n: usize) -> &K_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x130 - The memory that stores k."]
    #[inline(always)]
    pub fn k_mem_iter(&self) -> impl Iterator<Item = &K_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x120..0x150 - The memory that stores Px."]
    #[inline(always)]
    pub const fn px_mem(&self, n: usize) -> &PX_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x150 - The memory that stores Px."]
    #[inline(always)]
    pub fn px_mem_iter(&self) -> impl Iterator<Item = &PX_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x140..0x170 - The memory that stores Py."]
    #[inline(always)]
    pub const fn py_mem(&self, n: usize) -> &PY_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x170 - The memory that stores Py."]
    #[inline(always)]
    pub fn py_mem_iter(&self) -> impl Iterator<Item = &PY_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x160..0x190 - The memory that stores Qx."]
    #[inline(always)]
    pub const fn qx_mem(&self, n: usize) -> &QX_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x190 - The memory that stores Qx."]
    #[inline(always)]
    pub fn qx_mem_iter(&self) -> impl Iterator<Item = &QX_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x180..0x1b0 - The memory that stores Qy."]
    #[inline(always)]
    pub const fn qy_mem(&self, n: usize) -> &QY_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(384)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1b0 - The memory that stores Qy."]
    #[inline(always)]
    pub fn qy_mem_iter(&self) -> impl Iterator<Item = &QY_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(384)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1a0..0x1d0 - The memory that stores Qz."]
    #[inline(always)]
    pub const fn qz_mem(&self, n: usize) -> &QZ_MEM {
        #[allow(clippy::no_effect)]
        [(); 12][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1d0 - The memory that stores Qz."]
    #[inline(always)]
    pub fn qz_mem_iter(&self) -> impl Iterator<Item = &QZ_MEM> {
        (0..12).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(4 * n)
                .cast()
        })
    }
}
#[doc = "INT_RAW (r) register accessor: ECC raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "ECC raw interrupt status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: ECC masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "ECC masked interrupt status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: ECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "ECC interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: ECC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "ECC interrupt clear register"]
pub mod int_clr;
#[doc = "MULT_CONF (rw) register accessor: ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mult_conf`] module"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "ECC configuration register"]
pub mod mult_conf;
pub use crate::aes::date;
pub use crate::aes::DATE;
#[doc = "K_MEM (rw) register accessor: The memory that stores k.\n\nYou can [`read`](crate::Reg::read) this register and get [`k_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@k_mem`] module"]
pub type K_MEM = crate::Reg<k_mem::K_MEM_SPEC>;
#[doc = "The memory that stores k."]
pub mod k_mem;
#[doc = "PX_MEM (rw) register accessor: The memory that stores Px.\n\nYou can [`read`](crate::Reg::read) this register and get [`px_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@px_mem`] module"]
pub type PX_MEM = crate::Reg<px_mem::PX_MEM_SPEC>;
#[doc = "The memory that stores Px."]
pub mod px_mem;
#[doc = "PY_MEM (rw) register accessor: The memory that stores Py.\n\nYou can [`read`](crate::Reg::read) this register and get [`py_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`py_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@py_mem`] module"]
pub type PY_MEM = crate::Reg<py_mem::PY_MEM_SPEC>;
#[doc = "The memory that stores Py."]
pub mod py_mem;
#[doc = "QX_MEM (rw) register accessor: The memory that stores Qx.\n\nYou can [`read`](crate::Reg::read) this register and get [`qx_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qx_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qx_mem`] module"]
pub type QX_MEM = crate::Reg<qx_mem::QX_MEM_SPEC>;
#[doc = "The memory that stores Qx."]
pub mod qx_mem;
#[doc = "QY_MEM (rw) register accessor: The memory that stores Qy.\n\nYou can [`read`](crate::Reg::read) this register and get [`qy_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qy_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qy_mem`] module"]
pub type QY_MEM = crate::Reg<qy_mem::QY_MEM_SPEC>;
#[doc = "The memory that stores Qy."]
pub mod qy_mem;
#[doc = "QZ_MEM (rw) register accessor: The memory that stores Qz.\n\nYou can [`read`](crate::Reg::read) this register and get [`qz_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qz_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qz_mem`] module"]
pub type QZ_MEM = crate::Reg<qz_mem::QZ_MEM_SPEC>;
#[doc = "The memory that stores Qz."]
pub mod qz_mem;
