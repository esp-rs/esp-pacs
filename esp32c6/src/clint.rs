#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip: MSIP,
    _reserved1: [u8; 0x3ffc],
    mtimecmp: MTIMECMP,
    mtimeload: MTIMELOAD,
    mtimectl: MTIMECTL,
    _reserved4: [u8; 0x7fe4],
    mtime: MTIME,
}
impl RegisterBlock {
    #[doc = "0x00 - Core-local machine software interrupt pending register"]
    #[inline(always)]
    pub const fn msip(&self) -> &MSIP {
        &self.msip
    }
    #[doc = "0x4000..0x4008 - Core-local machine timer compare value"]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &MTIMECMP {
        &self.mtimecmp
    }
    #[doc = "0x4008..0x4010 - Core-local machine timer load value"]
    #[inline(always)]
    pub const fn mtimeload(&self) -> &MTIMELOAD {
        &self.mtimeload
    }
    #[doc = "0x4010 - Core-local machine timer interrupt control/status register"]
    #[inline(always)]
    pub const fn mtimectl(&self) -> &MTIMECTL {
        &self.mtimectl
    }
    #[doc = "0xbff8..0xc000 - Core-local system counter value"]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
}
#[doc = "MSIP (rw) register accessor: Core-local machine software interrupt pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`] module"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Core-local machine software interrupt pending register"]
pub mod msip;
#[doc = "MTIMECMP (rw) register accessor: Core-local machine timer compare value\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`] module"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Core-local machine timer compare value"]
pub mod mtimecmp;
#[doc = "MTIMELOAD (rw) register accessor: Core-local machine timer load value\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimeload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimeload`] module"]
pub type MTIMELOAD = crate::Reg<mtimeload::MTIMELOAD_SPEC>;
#[doc = "Core-local machine timer load value"]
pub mod mtimeload;
#[doc = "MTIMECTL (rw) register accessor: Core-local machine timer interrupt control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimectl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimectl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimectl`] module"]
pub type MTIMECTL = crate::Reg<mtimectl::MTIMECTL_SPEC>;
#[doc = "Core-local machine timer interrupt control/status register"]
pub mod mtimectl;
#[doc = "MTIME (r) register accessor: Core-local system counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`] module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Core-local system counter value"]
pub mod mtime;
