#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster M%s, containing M?_STATUS, M?_STATUS_CLR, M?_EXCEPTION_INFO0, M?_EXCEPTION_INFO1
pub struct M {
    status: STATUS,
    status_clr: STATUS_CLR,
    exception_info0: EXCEPTION_INFO0,
    exception_info1: EXCEPTION_INFO1,
}
impl M {
    ///0x00 - M0 status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x04 - M0 status clear register
    #[inline(always)]
    pub const fn status_clr(&self) -> &STATUS_CLR {
        &self.status_clr
    }
    ///0x08 - M0 exception_info0 register
    #[inline(always)]
    pub const fn exception_info0(&self) -> &EXCEPTION_INFO0 {
        &self.exception_info0
    }
    ///0x0c - M0 exception_info1 register
    #[inline(always)]
    pub const fn exception_info1(&self) -> &EXCEPTION_INFO1 {
        &self.exception_info1
    }
}
/**STATUS (r) register accessor: M0 status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///M0 status register
pub mod status;
/**STATUS_CLR (w) register accessor: M0 status clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_clr`] module*/
pub type STATUS_CLR = crate::Reg<status_clr::STATUS_CLR_SPEC>;
///M0 status clear register
pub mod status_clr;
/**EXCEPTION_INFO0 (r) register accessor: M0 exception_info0 register

You can [`read`](crate::generic::Reg::read) this register and get [`exception_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@exception_info0`] module*/
pub type EXCEPTION_INFO0 = crate::Reg<exception_info0::EXCEPTION_INFO0_SPEC>;
///M0 exception_info0 register
pub mod exception_info0;
/**EXCEPTION_INFO1 (r) register accessor: M0 exception_info1 register

You can [`read`](crate::generic::Reg::read) this register and get [`exception_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@exception_info1`] module*/
pub type EXCEPTION_INFO1 = crate::Reg<exception_info1::EXCEPTION_INFO1_SPEC>;
///M0 exception_info1 register
pub mod exception_info1;
