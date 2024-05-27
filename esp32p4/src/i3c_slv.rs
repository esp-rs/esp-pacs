#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    config: CONFIG,
    status: STATUS,
    ctrl: CTRL,
    intset: INTSET,
    intclr: INTCLR,
    intmasked: INTMASKED,
    _reserved6: [u8; 0x10],
    datactrl: DATACTRL,
    wdatab: WDATAB,
    wdatabe: WDATABE,
    _reserved9: [u8; 0x08],
    rdarab: RDARAB,
    _reserved10: [u8; 0x04],
    rdatah: RDATAH,
    _reserved11: [u8; 0x10],
    capabilities2: CAPABILITIES2,
    capabilities: CAPABILITIES,
    _reserved13: [u8; 0x08],
    idpartno: IDPARTNO,
    idext: IDEXT,
    vendorid: VENDORID,
}
impl RegisterBlock {
    ///0x04 - NA
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    ///0x08 - NA
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x0c - NA
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x10 - INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)
    #[inline(always)]
    pub const fn intset(&self) -> &INTSET {
        &self.intset
    }
    ///0x14 - NA
    #[inline(always)]
    pub const fn intclr(&self) -> &INTCLR {
        &self.intclr
    }
    ///0x18 - NA
    #[inline(always)]
    pub const fn intmasked(&self) -> &INTMASKED {
        &self.intmasked
    }
    ///0x2c - NA
    #[inline(always)]
    pub const fn datactrl(&self) -> &DATACTRL {
        &self.datactrl
    }
    ///0x30 - NA
    #[inline(always)]
    pub const fn wdatab(&self) -> &WDATAB {
        &self.wdatab
    }
    ///0x34 - NA
    #[inline(always)]
    pub const fn wdatabe(&self) -> &WDATABE {
        &self.wdatabe
    }
    ///0x40 - Read Byte Data (from-bus) register
    #[inline(always)]
    pub const fn rdarab(&self) -> &RDARAB {
        &self.rdarab
    }
    ///0x48 - Read Half-word Data (from-bus) register
    #[inline(always)]
    pub const fn rdatah(&self) -> &RDATAH {
        &self.rdatah
    }
    ///0x5c - NA
    #[inline(always)]
    pub const fn capabilities2(&self) -> &CAPABILITIES2 {
        &self.capabilities2
    }
    ///0x60 - NA
    #[inline(always)]
    pub const fn capabilities(&self) -> &CAPABILITIES {
        &self.capabilities
    }
    ///0x6c - NA
    #[inline(always)]
    pub const fn idpartno(&self) -> &IDPARTNO {
        &self.idpartno
    }
    ///0x70 - NA
    #[inline(always)]
    pub const fn idext(&self) -> &IDEXT {
        &self.idext
    }
    ///0x74 - NA
    #[inline(always)]
    pub const fn vendorid(&self) -> &VENDORID {
        &self.vendorid
    }
}
/**CONFIG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@config`] module*/
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
///NA
pub mod config;
/**STATUS (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///NA
pub mod status;
/**CTRL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///NA
pub mod ctrl;
/**INTSET (rw) register accessor: INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)

You can [`read`](crate::generic::Reg::read) this register and get [`intset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intset`] module*/
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
///INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)
pub mod intset;
/**INTCLR (w) register accessor: NA

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intclr`] module*/
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
///NA
pub mod intclr;
/**INTMASKED (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`intmasked::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intmasked`] module*/
pub type INTMASKED = crate::Reg<intmasked::INTMASKED_SPEC>;
///NA
pub mod intmasked;
/**DATACTRL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`datactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@datactrl`] module*/
pub type DATACTRL = crate::Reg<datactrl::DATACTRL_SPEC>;
///NA
pub mod datactrl;
/**WDATAB (w) register accessor: NA

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdatab::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdatab`] module*/
pub type WDATAB = crate::Reg<wdatab::WDATAB_SPEC>;
///NA
pub mod wdatab;
/**WDATABE (w) register accessor: NA

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdatabe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdatabe`] module*/
pub type WDATABE = crate::Reg<wdatabe::WDATABE_SPEC>;
///NA
pub mod wdatabe;
/**RDARAB (r) register accessor: Read Byte Data (from-bus) register

You can [`read`](crate::generic::Reg::read) this register and get [`rdarab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdarab`] module*/
pub type RDARAB = crate::Reg<rdarab::RDARAB_SPEC>;
///Read Byte Data (from-bus) register
pub mod rdarab;
/**RDATAH (r) register accessor: Read Half-word Data (from-bus) register

You can [`read`](crate::generic::Reg::read) this register and get [`rdatah::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdatah`] module*/
pub type RDATAH = crate::Reg<rdatah::RDATAH_SPEC>;
///Read Half-word Data (from-bus) register
pub mod rdatah;
/**CAPABILITIES2 (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`capabilities2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capabilities2`] module*/
pub type CAPABILITIES2 = crate::Reg<capabilities2::CAPABILITIES2_SPEC>;
///NA
pub mod capabilities2;
/**CAPABILITIES (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capabilities`] module*/
pub type CAPABILITIES = crate::Reg<capabilities::CAPABILITIES_SPEC>;
///NA
pub mod capabilities;
/**IDPARTNO (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`idpartno::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idpartno::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idpartno`] module*/
pub type IDPARTNO = crate::Reg<idpartno::IDPARTNO_SPEC>;
///NA
pub mod idpartno;
/**IDEXT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`idext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@idext`] module*/
pub type IDEXT = crate::Reg<idext::IDEXT_SPEC>;
///NA
pub mod idext;
/**VENDORID (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vendorid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vendorid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vendorid`] module*/
pub type VENDORID = crate::Reg<vendorid::VENDORID_SPEC>;
///NA
pub mod vendorid;
