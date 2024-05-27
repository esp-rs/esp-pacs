#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster IN_INT, containing IN_INT_RAW, IN_INT_ST, IN_INT_ENA, IN_INT_CLR
pub struct IN_INT {
    raw: RAW,
    st: ST,
    ena: ENA,
    clr: CLR,
}
impl IN_INT {
    ///0x00 - Raw status interrupt of channel 0
    #[inline(always)]
    pub const fn raw(&self) -> &RAW {
        &self.raw
    }
    ///0x04 - Masked interrupt of channel 0
    #[inline(always)]
    pub const fn st(&self) -> &ST {
        &self.st
    }
    ///0x08 - Interrupt enable bits of channel 0
    #[inline(always)]
    pub const fn ena(&self) -> &ENA {
        &self.ena
    }
    ///0x0c - Interrupt clear bits of channel 0
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
}
/**RAW (rw) register accessor: Raw status interrupt of channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@raw`] module*/
pub type RAW = crate::Reg<raw::RAW_SPEC>;
///Raw status interrupt of channel 0
pub mod raw;
/**ST (r) register accessor: Masked interrupt of channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@st`] module*/
pub type ST = crate::Reg<st::ST_SPEC>;
///Masked interrupt of channel 0
pub mod st;
/**ENA (rw) register accessor: Interrupt enable bits of channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ena`] module*/
pub type ENA = crate::Reg<ena::ENA_SPEC>;
///Interrupt enable bits of channel 0
pub mod ena;
/**CLR (w) register accessor: Interrupt clear bits of channel 0

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clr`] module*/
pub type CLR = crate::Reg<clr::CLR_SPEC>;
///Interrupt clear bits of channel 0
pub mod clr;
