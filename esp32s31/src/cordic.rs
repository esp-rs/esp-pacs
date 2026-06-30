#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr_cfg: CSR_CFG,
    arg1: ARG1,
    arg2: ARG2,
    res1: RES1,
    res2: RES2,
    _reserved5: [u8; 0x14],
    io_int_ena: IO_INT_ENA,
    io_int_raw: IO_INT_RAW,
    io_int_st: IO_INT_ST,
    io_int_clr: IO_INT_CLR,
    fifo_cfg: FIFO_CFG,
    clk: CLK,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Cordic model config register"]
    #[inline(always)]
    pub const fn csr_cfg(&self) -> &CSR_CFG {
        &self.csr_cfg
    }
    #[doc = "0x04 - Cordic argument 1 config register"]
    #[inline(always)]
    pub const fn arg1(&self) -> &ARG1 {
        &self.arg1
    }
    #[doc = "0x08 - Cordic argument 2 config register"]
    #[inline(always)]
    pub const fn arg2(&self) -> &ARG2 {
        &self.arg2
    }
    #[doc = "0x0c - Cordic result 1 receive register"]
    #[inline(always)]
    pub const fn res1(&self) -> &RES1 {
        &self.res1
    }
    #[doc = "0x10 - Cordic result 2 receive register"]
    #[inline(always)]
    pub const fn res2(&self) -> &RES2 {
        &self.res2
    }
    #[doc = "0x28 - Cordic interrupt enable singal configuration register."]
    #[inline(always)]
    pub const fn io_int_ena(&self) -> &IO_INT_ENA {
        &self.io_int_ena
    }
    #[doc = "0x2c - Cordic interrupt raw singal status register."]
    #[inline(always)]
    pub const fn io_int_raw(&self) -> &IO_INT_RAW {
        &self.io_int_raw
    }
    #[doc = "0x30 - Cordic interrupt singal status register."]
    #[inline(always)]
    pub const fn io_int_st(&self) -> &IO_INT_ST {
        &self.io_int_st
    }
    #[doc = "0x34 - Cordic interrupt clear singal configuration register."]
    #[inline(always)]
    pub const fn io_int_clr(&self) -> &IO_INT_CLR {
        &self.io_int_clr
    }
    #[doc = "0x38 - Cordic FIFO configuration register."]
    #[inline(always)]
    pub const fn fifo_cfg(&self) -> &FIFO_CFG {
        &self.fifo_cfg
    }
    #[doc = "0x3c - Cordic clk configuration register."]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x40 - Version register."]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CSR_CFG (rw) register accessor: Cordic model config register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr_cfg`] module"]
pub type CSR_CFG = crate::Reg<csr_cfg::CSR_CFG_SPEC>;
#[doc = "Cordic model config register"]
pub mod csr_cfg;
#[doc = "ARG1 (rw) register accessor: Cordic argument 1 config register\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1`] module"]
pub type ARG1 = crate::Reg<arg1::ARG1_SPEC>;
#[doc = "Cordic argument 1 config register"]
pub mod arg1;
#[doc = "ARG2 (rw) register accessor: Cordic argument 2 config register\n\nYou can [`read`](crate::Reg::read) this register and get [`arg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg2`] module"]
pub type ARG2 = crate::Reg<arg2::ARG2_SPEC>;
#[doc = "Cordic argument 2 config register"]
pub mod arg2;
#[doc = "RES1 (r) register accessor: Cordic result 1 receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res1`] module"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "Cordic result 1 receive register"]
pub mod res1;
#[doc = "RES2 (r) register accessor: Cordic result 2 receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res2`] module"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "Cordic result 2 receive register"]
pub mod res2;
#[doc = "IO_INT_ENA (rw) register accessor: Cordic interrupt enable singal configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_int_ena`] module"]
pub type IO_INT_ENA = crate::Reg<io_int_ena::IO_INT_ENA_SPEC>;
#[doc = "Cordic interrupt enable singal configuration register."]
pub mod io_int_ena;
#[doc = "IO_INT_RAW (r) register accessor: Cordic interrupt raw singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_int_raw`] module"]
pub type IO_INT_RAW = crate::Reg<io_int_raw::IO_INT_RAW_SPEC>;
#[doc = "Cordic interrupt raw singal status register."]
pub mod io_int_raw;
#[doc = "IO_INT_ST (r) register accessor: Cordic interrupt singal status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`io_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_int_st`] module"]
pub type IO_INT_ST = crate::Reg<io_int_st::IO_INT_ST_SPEC>;
#[doc = "Cordic interrupt singal status register."]
pub mod io_int_st;
#[doc = "IO_INT_CLR (w) register accessor: Cordic interrupt clear singal configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_int_clr`] module"]
pub type IO_INT_CLR = crate::Reg<io_int_clr::IO_INT_CLR_SPEC>;
#[doc = "Cordic interrupt clear singal configuration register."]
pub mod io_int_clr;
#[doc = "FIFO_CFG (w) register accessor: Cordic FIFO configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_cfg`] module"]
pub type FIFO_CFG = crate::Reg<fifo_cfg::FIFO_CFG_SPEC>;
#[doc = "Cordic FIFO configuration register."]
pub mod fifo_cfg;
#[doc = "CLK (rw) register accessor: Cordic clk configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Cordic clk configuration register."]
pub mod clk;
#[doc = "VERSION (rw) register accessor: Version register.\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
