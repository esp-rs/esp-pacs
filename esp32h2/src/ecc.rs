#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "0x0c - ECC interrupt raw register, valid in level."]
    pub mult_int_raw: MULT_INT_RAW,
    #[doc = "0x10 - ECC interrupt status register."]
    pub mult_int_st: MULT_INT_ST,
    #[doc = "0x14 - ECC interrupt enable register."]
    pub mult_int_ena: MULT_INT_ENA,
    #[doc = "0x18 - ECC interrupt clear register."]
    pub mult_int_clr: MULT_INT_CLR,
    #[doc = "0x1c - ECC configure register"]
    pub mult_conf: MULT_CONF,
    _reserved5: [u8; 0xdc],
    #[doc = "0xfc - Version control register"]
    pub mult_date: MULT_DATE,
    #[doc = "0x100..0x120 - The memory that stores k."]
    pub k_mem: [K_MEM; 32],
    #[doc = "0x120..0x140 - The memory that stores Px."]
    pub px_mem: [PX_MEM; 32],
    #[doc = "0x140..0x160 - The memory that stores Py."]
    pub py_mem: [PY_MEM; 32],
    #[doc = "0x160..0x180 - The memory that stores Qx"]
    pub qx_mem: [QX_MEM; 32],
    #[doc = "0x180..0x1a0 - The memory that stores Qy"]
    pub qy_mem: [QY_MEM; 32],
    #[doc = "0x1a0..0x1c0 - The memory that stores Qz"]
    pub qz_mem: [QZ_MEM; 32],
}
#[doc = "MULT_INT_RAW (r) register accessor: ECC interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_int_raw`] module"]
pub type MULT_INT_RAW = crate::Reg<mult_int_raw::MULT_INT_RAW_SPEC>;
#[doc = "ECC interrupt raw register, valid in level."]
pub mod mult_int_raw;
#[doc = "MULT_INT_ST (r) register accessor: ECC interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_int_st`] module"]
pub type MULT_INT_ST = crate::Reg<mult_int_st::MULT_INT_ST_SPEC>;
#[doc = "ECC interrupt status register."]
pub mod mult_int_st;
#[doc = "MULT_INT_ENA (rw) register accessor: ECC interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_int_ena`] module"]
pub type MULT_INT_ENA = crate::Reg<mult_int_ena::MULT_INT_ENA_SPEC>;
#[doc = "ECC interrupt enable register."]
pub mod mult_int_ena;
#[doc = "MULT_INT_CLR (w) register accessor: ECC interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_int_clr`] module"]
pub type MULT_INT_CLR = crate::Reg<mult_int_clr::MULT_INT_CLR_SPEC>;
#[doc = "ECC interrupt clear register."]
pub mod mult_int_clr;
#[doc = "MULT_CONF (rw) register accessor: ECC configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_conf`] module"]
pub type MULT_CONF = crate::Reg<mult_conf::MULT_CONF_SPEC>;
#[doc = "ECC configure register"]
pub mod mult_conf;
#[doc = "MULT_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mult_date`] module"]
pub type MULT_DATE = crate::Reg<mult_date::MULT_DATE_SPEC>;
#[doc = "Version control register"]
pub mod mult_date;
#[doc = "K_MEM (rw) register accessor: The memory that stores k.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`k_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`k_mem`] module"]
pub type K_MEM = crate::Reg<k_mem::K_MEM_SPEC>;
#[doc = "The memory that stores k."]
pub mod k_mem;
#[doc = "PX_MEM (rw) register accessor: The memory that stores Px.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`px_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`px_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`px_mem`] module"]
pub type PX_MEM = crate::Reg<px_mem::PX_MEM_SPEC>;
#[doc = "The memory that stores Px."]
pub mod px_mem;
#[doc = "PY_MEM (rw) register accessor: The memory that stores Py.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`py_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`py_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`py_mem`] module"]
pub type PY_MEM = crate::Reg<py_mem::PY_MEM_SPEC>;
#[doc = "The memory that stores Py."]
pub mod py_mem;
#[doc = "QX_MEM (rw) register accessor: The memory that stores Qx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qx_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qx_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qx_mem`] module"]
pub type QX_MEM = crate::Reg<qx_mem::QX_MEM_SPEC>;
#[doc = "The memory that stores Qx"]
pub mod qx_mem;
#[doc = "QY_MEM (rw) register accessor: The memory that stores Qy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qy_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qy_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qy_mem`] module"]
pub type QY_MEM = crate::Reg<qy_mem::QY_MEM_SPEC>;
#[doc = "The memory that stores Qy"]
pub mod qy_mem;
#[doc = "QZ_MEM (rw) register accessor: The memory that stores Qz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qz_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qz_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`qz_mem`] module"]
pub type QZ_MEM = crate::Reg<qz_mem::QZ_MEM_SPEC>;
#[doc = "The memory that stores Qz"]
pub mod qz_mem;
