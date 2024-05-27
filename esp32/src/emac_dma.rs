#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    dmabusmode: DMABUSMODE,
    dmatxpolldemand: DMATXPOLLDEMAND,
    dmarxpolldemand: DMARXPOLLDEMAND,
    dmarxbaseaddr: DMARXBASEADDR,
    dmatxbaseaddr: DMATXBASEADDR,
    dmastatus: DMASTATUS,
    dmaoperation_mode: DMAOPERATION_MODE,
    dmain_en: DMAIN_EN,
    dmamissedfr: DMAMISSEDFR,
    dmarintwdtimer: DMARINTWDTIMER,
    _reserved10: [u8; 0x20],
    dmatxcurrdesc: DMATXCURRDESC,
    dmarxcurrdesc: DMARXCURRDESC,
    dmatxcurraddr_buf: DMATXCURRADDR_BUF,
    dmarxcurraddr_buf: DMARXCURRADDR_BUF,
}
impl RegisterBlock {
    ///0x00 - Bus mode configuration
    #[inline(always)]
    pub const fn dmabusmode(&self) -> &DMABUSMODE {
        &self.dmabusmode
    }
    ///0x04 - When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\[2\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.
    #[inline(always)]
    pub const fn dmatxpolldemand(&self) -> &DMATXPOLLDEMAND {
        &self.dmatxpolldemand
    }
    ///0x08 - When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\[7\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.
    #[inline(always)]
    pub const fn dmarxpolldemand(&self) -> &DMARXPOLLDEMAND {
        &self.dmarxpolldemand
    }
    ///0x0c - This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\[1:0\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.
    #[inline(always)]
    pub const fn dmarxbaseaddr(&self) -> &DMARXBASEADDR {
        &self.dmarxbaseaddr
    }
    ///0x10 - This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\[1:0\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.
    #[inline(always)]
    pub const fn dmatxbaseaddr(&self) -> &DMATXBASEADDR {
        &self.dmatxbaseaddr
    }
    ///0x14 - State of interrupts, errors and other events
    #[inline(always)]
    pub const fn dmastatus(&self) -> &DMASTATUS {
        &self.dmastatus
    }
    ///0x18 - Receive and Transmit operating modes and command
    #[inline(always)]
    pub const fn dmaoperation_mode(&self) -> &DMAOPERATION_MODE {
        &self.dmaoperation_mode
    }
    ///0x1c -
    #[inline(always)]
    pub const fn dmain_en(&self) -> &DMAIN_EN {
        &self.dmain_en
    }
    ///0x20 - Missed Frame and Buffer Overflow Counter Register
    #[inline(always)]
    pub const fn dmamissedfr(&self) -> &DMAMISSEDFR {
        &self.dmamissedfr
    }
    ///0x24 - Watchdog timer count on receive
    #[inline(always)]
    pub const fn dmarintwdtimer(&self) -> &DMARINTWDTIMER {
        &self.dmarintwdtimer
    }
    ///0x48 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
    #[inline(always)]
    pub const fn dmatxcurrdesc(&self) -> &DMATXCURRDESC {
        &self.dmatxcurrdesc
    }
    ///0x4c - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
    #[inline(always)]
    pub const fn dmarxcurrdesc(&self) -> &DMARXCURRDESC {
        &self.dmarxcurrdesc
    }
    ///0x50 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
    #[inline(always)]
    pub const fn dmatxcurraddr_buf(&self) -> &DMATXCURRADDR_BUF {
        &self.dmatxcurraddr_buf
    }
    ///0x54 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
    #[inline(always)]
    pub const fn dmarxcurraddr_buf(&self) -> &DMARXCURRADDR_BUF {
        &self.dmarxcurraddr_buf
    }
}
/**DMABUSMODE (rw) register accessor: Bus mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dmabusmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabusmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmabusmode`] module*/
pub type DMABUSMODE = crate::Reg<dmabusmode::DMABUSMODE_SPEC>;
///Bus mode configuration
pub mod dmabusmode;
/**DMATXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\[2\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.

You can [`read`](crate::generic::Reg::read) this register and get [`dmatxpolldemand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmatxpolldemand`] module*/
pub type DMATXPOLLDEMAND = crate::Reg<dmatxpolldemand::DMATXPOLLDEMAND_SPEC>;
///When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\[2\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.
pub mod dmatxpolldemand;
/**DMARXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\[7\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.

You can [`read`](crate::generic::Reg::read) this register and get [`dmarxpolldemand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmarxpolldemand`] module*/
pub type DMARXPOLLDEMAND = crate::Reg<dmarxpolldemand::DMARXPOLLDEMAND_SPEC>;
///When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\[7\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.
pub mod dmarxpolldemand;
/**DMARXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\[1:0\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.

You can [`read`](crate::generic::Reg::read) this register and get [`dmarxbaseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarxbaseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmarxbaseaddr`] module*/
pub type DMARXBASEADDR = crate::Reg<dmarxbaseaddr::DMARXBASEADDR_SPEC>;
///This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\[1:0\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.
pub mod dmarxbaseaddr;
/**DMATXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\[1:0\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.

You can [`read`](crate::generic::Reg::read) this register and get [`dmatxbaseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxbaseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmatxbaseaddr`] module*/
pub type DMATXBASEADDR = crate::Reg<dmatxbaseaddr::DMATXBASEADDR_SPEC>;
///This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\[1:0\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.
pub mod dmatxbaseaddr;
/**DMASTATUS (rw) register accessor: State of interrupts, errors and other events

You can [`read`](crate::generic::Reg::read) this register and get [`dmastatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmastatus`] module*/
pub type DMASTATUS = crate::Reg<dmastatus::DMASTATUS_SPEC>;
///State of interrupts, errors and other events
pub mod dmastatus;
/**DMAOPERATION_MODE (rw) register accessor: Receive and Transmit operating modes and command

You can [`read`](crate::generic::Reg::read) this register and get [`dmaoperation_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaoperation_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmaoperation_mode`] module*/
pub type DMAOPERATION_MODE = crate::Reg<dmaoperation_mode::DMAOPERATION_MODE_SPEC>;
///Receive and Transmit operating modes and command
pub mod dmaoperation_mode;
/**DMAIN_EN (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dmain_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmain_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmain_en`] module*/
pub type DMAIN_EN = crate::Reg<dmain_en::DMAIN_EN_SPEC>;
///
pub mod dmain_en;
/**DMAMISSEDFR (rw) register accessor: Missed Frame and Buffer Overflow Counter Register

You can [`read`](crate::generic::Reg::read) this register and get [`dmamissedfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamissedfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmamissedfr`] module*/
pub type DMAMISSEDFR = crate::Reg<dmamissedfr::DMAMISSEDFR_SPEC>;
///Missed Frame and Buffer Overflow Counter Register
pub mod dmamissedfr;
/**DMARINTWDTIMER (rw) register accessor: Watchdog timer count on receive

You can [`read`](crate::generic::Reg::read) this register and get [`dmarintwdtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarintwdtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmarintwdtimer`] module*/
pub type DMARINTWDTIMER = crate::Reg<dmarintwdtimer::DMARINTWDTIMER_SPEC>;
///Watchdog timer count on receive
pub mod dmarintwdtimer;
/**DMATXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.

You can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurrdesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmatxcurrdesc`] module*/
pub type DMATXCURRDESC = crate::Reg<dmatxcurrdesc::DMATXCURRDESC_SPEC>;
///The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
pub mod dmatxcurrdesc;
/**DMARXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.

You can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurrdesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmarxcurrdesc`] module*/
pub type DMARXCURRDESC = crate::Reg<dmarxcurrdesc::DMARXCURRDESC_SPEC>;
///The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
pub mod dmarxcurrdesc;
/**DMATXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.

You can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurraddr_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmatxcurraddr_buf`] module*/
pub type DMATXCURRADDR_BUF = crate::Reg<dmatxcurraddr_buf::DMATXCURRADDR_BUF_SPEC>;
///The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
pub mod dmatxcurraddr_buf;
/**DMARXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.

You can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurraddr_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmarxcurraddr_buf`] module*/
pub type DMARXCURRADDR_BUF = crate::Reg<dmarxcurraddr_buf::DMARXCURRADDR_BUF_SPEC>;
///The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.
pub mod dmarxcurraddr_buf;
