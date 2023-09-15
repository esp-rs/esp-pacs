#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Bus mode configuration"]
    pub dmabusmode: DMABUSMODE,
    #[doc = "0x04 - When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes."]
    pub dmatxpolldemand: DMATXPOLLDEMAND,
    #[doc = "0x08 - When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state."]
    pub dmarxpolldemand: DMARXPOLLDEMAND,
    #[doc = "0x0c - This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only."]
    pub dmarxbaseaddr: DMARXBASEADDR,
    #[doc = "0x10 - This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only."]
    pub dmatxbaseaddr: DMATXBASEADDR,
    #[doc = "0x14 - State of interrupts, errors and other events"]
    pub dmastatus: DMASTATUS,
    #[doc = "0x18 - Receive and Transmit operating modes and command"]
    pub dmaoperation_mode: DMAOPERATION_MODE,
    #[doc = "0x1c - "]
    pub dmain_en: DMAIN_EN,
    #[doc = "0x20 - Missed Frame and Buffer Overflow Counter Register"]
    pub dmamissedfr: DMAMISSEDFR,
    #[doc = "0x24 - Watchdog timer count on receive"]
    pub dmarintwdtimer: DMARINTWDTIMER,
    _reserved10: [u8; 0x20],
    #[doc = "0x48 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    pub dmatxcurrdesc: DMATXCURRDESC,
    #[doc = "0x4c - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    pub dmarxcurrdesc: DMARXCURRDESC,
    #[doc = "0x50 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    pub dmatxcurraddr_buf: DMATXCURRADDR_BUF,
    #[doc = "0x54 - The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
    pub dmarxcurraddr_buf: DMARXCURRADDR_BUF,
}
#[doc = "DMABUSMODE (rw) register accessor: Bus mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabusmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabusmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmabusmode`] module"]
pub type DMABUSMODE = crate::Reg<dmabusmode::DMABUSMODE_SPEC>;
#[doc = "Bus mode configuration"]
pub mod dmabusmode;
#[doc = "DMATXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxpolldemand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatxpolldemand`] module"]
pub type DMATXPOLLDEMAND = crate::Reg<dmatxpolldemand::DMATXPOLLDEMAND_SPEC>;
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes."]
pub mod dmatxpolldemand;
#[doc = "DMARXPOLLDEMAND (r) register accessor: When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxpolldemand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarxpolldemand`] module"]
pub type DMARXPOLLDEMAND = crate::Reg<dmarxpolldemand::DMARXPOLLDEMAND_SPEC>;
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state."]
pub mod dmarxpolldemand;
#[doc = "DMARXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxbaseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarxbaseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarxbaseaddr`] module"]
pub type DMARXBASEADDR = crate::Reg<dmarxbaseaddr::DMARXBASEADDR_SPEC>;
#[doc = "This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only."]
pub mod dmarxbaseaddr;
#[doc = "DMATXBASEADDR (rw) register accessor: This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxbaseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxbaseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatxbaseaddr`] module"]
pub type DMATXBASEADDR = crate::Reg<dmatxbaseaddr::DMATXBASEADDR_SPEC>;
#[doc = "This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only."]
pub mod dmatxbaseaddr;
#[doc = "DMASTATUS (rw) register accessor: State of interrupts, errors and other events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmastatus`] module"]
pub type DMASTATUS = crate::Reg<dmastatus::DMASTATUS_SPEC>;
#[doc = "State of interrupts, errors and other events"]
pub mod dmastatus;
#[doc = "DMAOPERATION_MODE (rw) register accessor: Receive and Transmit operating modes and command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaoperation_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaoperation_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaoperation_mode`] module"]
pub type DMAOPERATION_MODE = crate::Reg<dmaoperation_mode::DMAOPERATION_MODE_SPEC>;
#[doc = "Receive and Transmit operating modes and command"]
pub mod dmaoperation_mode;
#[doc = "DMAIN_EN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmain_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmain_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmain_en`] module"]
pub type DMAIN_EN = crate::Reg<dmain_en::DMAIN_EN_SPEC>;
#[doc = ""]
pub mod dmain_en;
#[doc = "DMAMISSEDFR (rw) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamissedfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamissedfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmamissedfr`] module"]
pub type DMAMISSEDFR = crate::Reg<dmamissedfr::DMAMISSEDFR_SPEC>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod dmamissedfr;
#[doc = "DMARINTWDTIMER (rw) register accessor: Watchdog timer count on receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarintwdtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarintwdtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarintwdtimer`] module"]
pub type DMARINTWDTIMER = crate::Reg<dmarintwdtimer::DMARINTWDTIMER_SPEC>;
#[doc = "Watchdog timer count on receive"]
pub mod dmarintwdtimer;
#[doc = "DMATXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurrdesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatxcurrdesc`] module"]
pub type DMATXCURRDESC = crate::Reg<dmatxcurrdesc::DMATXCURRDESC_SPEC>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmatxcurrdesc;
#[doc = "DMARXCURRDESC (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurrdesc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarxcurrdesc`] module"]
pub type DMARXCURRDESC = crate::Reg<dmarxcurrdesc::DMARXCURRDESC_SPEC>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmarxcurrdesc;
#[doc = "DMATXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurraddr_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatxcurraddr_buf`] module"]
pub type DMATXCURRADDR_BUF = crate::Reg<dmatxcurraddr_buf::DMATXCURRADDR_BUF_SPEC>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmatxcurraddr_buf;
#[doc = "DMARXCURRADDR_BUF (r) register accessor: The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurraddr_buf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarxcurraddr_buf`] module"]
pub type DMARXCURRADDR_BUF = crate::Reg<dmarxcurraddr_buf::DMARXCURRADDR_BUF_SPEC>;
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation."]
pub mod dmarxcurraddr_buf;
