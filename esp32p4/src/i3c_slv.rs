#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x10 - INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)"]
    #[inline(always)]
    pub const fn intset(&self) -> &INTSET {
        &self.intset
    }
    #[doc = "0x14 - NA"]
    #[inline(always)]
    pub const fn intclr(&self) -> &INTCLR {
        &self.intclr
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn intmasked(&self) -> &INTMASKED {
        &self.intmasked
    }
    #[doc = "0x2c - NA"]
    #[inline(always)]
    pub const fn datactrl(&self) -> &DATACTRL {
        &self.datactrl
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn wdatab(&self) -> &WDATAB {
        &self.wdatab
    }
    #[doc = "0x34 - NA"]
    #[inline(always)]
    pub const fn wdatabe(&self) -> &WDATABE {
        &self.wdatabe
    }
    #[doc = "0x40 - Read Byte Data (from-bus) register"]
    #[inline(always)]
    pub const fn rdarab(&self) -> &RDARAB {
        &self.rdarab
    }
    #[doc = "0x48 - Read Half-word Data (from-bus) register"]
    #[inline(always)]
    pub const fn rdatah(&self) -> &RDATAH {
        &self.rdatah
    }
    #[doc = "0x5c - NA"]
    #[inline(always)]
    pub const fn capabilities2(&self) -> &CAPABILITIES2 {
        &self.capabilities2
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn capabilities(&self) -> &CAPABILITIES {
        &self.capabilities
    }
    #[doc = "0x6c - NA"]
    #[inline(always)]
    pub const fn idpartno(&self) -> &IDPARTNO {
        &self.idpartno
    }
    #[doc = "0x70 - NA"]
    #[inline(always)]
    pub const fn idext(&self) -> &IDEXT {
        &self.idext
    }
    #[doc = "0x74 - NA"]
    #[inline(always)]
    pub const fn vendorid(&self) -> &VENDORID {
        &self.vendorid
    }
}
#[doc = "CONFIG (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "NA"]
pub mod config;
#[doc = "STATUS (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "NA"]
pub mod status;
#[doc = "CTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "NA"]
pub mod ctrl;
#[doc = "INTSET (rw) register accessor: INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`] module"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "INSET allows setting enables for interrupts(connecting the corresponding STATUS source to causing an IRQ to the processor)"]
pub mod intset;
#[doc = "INTCLR (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`] module"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "NA"]
pub mod intclr;
#[doc = "INTMASKED (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intmasked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmasked`] module"]
pub type INTMASKED = crate::Reg<intmasked::INTMASKED_SPEC>;
#[doc = "NA"]
pub mod intmasked;
#[doc = "DATACTRL (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`datactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datactrl`] module"]
pub type DATACTRL = crate::Reg<datactrl::DATACTRL_SPEC>;
#[doc = "NA"]
pub mod datactrl;
#[doc = "WDATAB (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatab::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdatab`] module"]
pub type WDATAB = crate::Reg<wdatab::WDATAB_SPEC>;
#[doc = "NA"]
pub mod wdatab;
#[doc = "WDATABE (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatabe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdatabe`] module"]
pub type WDATABE = crate::Reg<wdatabe::WDATABE_SPEC>;
#[doc = "NA"]
pub mod wdatabe;
#[doc = "RDARAB (r) register accessor: Read Byte Data (from-bus) register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdarab::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdarab`] module"]
pub type RDARAB = crate::Reg<rdarab::RDARAB_SPEC>;
#[doc = "Read Byte Data (from-bus) register"]
pub mod rdarab;
#[doc = "RDATAH (r) register accessor: Read Half-word Data (from-bus) register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdatah::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatah`] module"]
pub type RDATAH = crate::Reg<rdatah::RDATAH_SPEC>;
#[doc = "Read Half-word Data (from-bus) register"]
pub mod rdatah;
#[doc = "CAPABILITIES2 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities2`] module"]
pub type CAPABILITIES2 = crate::Reg<capabilities2::CAPABILITIES2_SPEC>;
#[doc = "NA"]
pub mod capabilities2;
#[doc = "CAPABILITIES (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities`] module"]
pub type CAPABILITIES = crate::Reg<capabilities::CAPABILITIES_SPEC>;
#[doc = "NA"]
pub mod capabilities;
#[doc = "IDPARTNO (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`idpartno::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idpartno::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idpartno`] module"]
pub type IDPARTNO = crate::Reg<idpartno::IDPARTNO_SPEC>;
#[doc = "NA"]
pub mod idpartno;
#[doc = "IDEXT (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`idext::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idext::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idext`] module"]
pub type IDEXT = crate::Reg<idext::IDEXT_SPEC>;
#[doc = "NA"]
pub mod idext;
#[doc = "VENDORID (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vendorid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendorid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vendorid`] module"]
pub type VENDORID = crate::Reg<vendorid::VENDORID_SPEC>;
#[doc = "NA"]
pub mod vendorid;
