#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration"]
    pub emacconfig: EMACCONFIG,
    #[doc = "0x04 - Frame filter settings"]
    pub emacff: EMACFF,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - PHY configuration access"]
    pub emacgmiiaddr: EMACGMIIADDR,
    #[doc = "0x14 - PHY data read write"]
    pub emacmiidata: EMACMIIDATA,
    #[doc = "0x18 - Frame flow control"]
    pub emacfc: EMACFC,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Status debugging bits"]
    pub emacdebug: EMACDEBUG,
    #[doc = "0x28 - The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
    pub pmt_rwuffr: PMT_RWUFFR,
    #[doc = "0x2c - PMT Control and Status"]
    pub pmt_csr: PMT_CSR,
    #[doc = "0x30 - LPI Control and Status"]
    pub emaclpi_crs: EMACLPI_CRS,
    #[doc = "0x34 - LPI Timers Control"]
    pub emaclpitimerscontrol: EMACLPITIMERSCONTROL,
    #[doc = "0x38 - Interrupt status"]
    pub emacints: EMACINTS,
    #[doc = "0x3c - Interrupt mask"]
    pub emacintmask: EMACINTMASK,
    #[doc = "0x40 - Upper 16 bits of the first 6-byte MAC address"]
    pub emacaddr0high: EMACADDR0HIGH,
    #[doc = "0x44 - This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    pub emacaddr0low: EMACADDR0LOW,
    #[doc = "0x48 - Upper 16 bits of the second 6-byte MAC address"]
    pub emacaddr1high: EMACADDR1HIGH,
    #[doc = "0x4c - This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    pub emacaddr1low: EMACADDR1LOW,
    #[doc = "0x50 - Upper 16 bits of the third 6-byte MAC address"]
    pub emacaddr2high: EMACADDR2HIGH,
    #[doc = "0x54 - This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    pub emacaddr2low: EMACADDR2LOW,
    #[doc = "0x58 - Upper 16 bits of the fourth 6-byte MAC address"]
    pub emacaddr3high: EMACADDR3HIGH,
    #[doc = "0x5c - This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    pub emacaddr3low: EMACADDR3LOW,
    #[doc = "0x60 - Upper 16 bits of the fifth 6-byte MAC address"]
    pub emacaddr4high: EMACADDR4HIGH,
    #[doc = "0x64 - This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    pub emacaddr4low: EMACADDR4LOW,
    #[doc = "0x68 - Upper 16 bits of the sixth 6-byte MAC address"]
    pub emacaddr5high: EMACADDR5HIGH,
    #[doc = "0x6c - This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    pub emacaddr5low: EMACADDR5LOW,
    #[doc = "0x70 - Upper 16 bits of the seventh 6-byte MAC address"]
    pub emacaddr6high: EMACADDR6HIGH,
    #[doc = "0x74 - This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    pub emacaddr6low: EMACADDR6LOW,
    #[doc = "0x78 - Upper 16 bits of the eighth 6-byte MAC address"]
    pub emacaddr7high: EMACADDR7HIGH,
    #[doc = "0x7c - This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    pub emacaddr7low: EMACADDR7LOW,
    _reserved28: [u8; 0x58],
    #[doc = "0xd8 - Link communication status"]
    pub emaccstatus: EMACCSTATUS,
    #[doc = "0xdc - Watchdog timeout control"]
    pub emacwdogto: EMACWDOGTO,
}
#[doc = "EMACCONFIG (rw) register accessor: MAC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacconfig`] module"]
pub type EMACCONFIG = crate::Reg<emacconfig::EMACCONFIG_SPEC>;
#[doc = "MAC configuration"]
pub mod emacconfig;
#[doc = "EMACFF (rw) register accessor: Frame filter settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacff`] module"]
pub type EMACFF = crate::Reg<emacff::EMACFF_SPEC>;
#[doc = "Frame filter settings"]
pub mod emacff;
#[doc = "EMACGMIIADDR (rw) register accessor: PHY configuration access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacgmiiaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacgmiiaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacgmiiaddr`] module"]
pub type EMACGMIIADDR = crate::Reg<emacgmiiaddr::EMACGMIIADDR_SPEC>;
#[doc = "PHY configuration access"]
pub mod emacgmiiaddr;
#[doc = "EMACMIIDATA (rw) register accessor: PHY data read write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacmiidata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacmiidata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacmiidata`] module"]
pub type EMACMIIDATA = crate::Reg<emacmiidata::EMACMIIDATA_SPEC>;
#[doc = "PHY data read write"]
pub mod emacmiidata;
#[doc = "EMACFC (rw) register accessor: Frame flow control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacfc`] module"]
pub type EMACFC = crate::Reg<emacfc::EMACFC_SPEC>;
#[doc = "Frame flow control"]
pub mod emacfc;
#[doc = "EMACDEBUG (r) register accessor: Status debugging bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacdebug::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacdebug`] module"]
pub type EMACDEBUG = crate::Reg<emacdebug::EMACDEBUG_SPEC>;
#[doc = "Status debugging bits"]
pub mod emacdebug;
#[doc = "PMT_RWUFFR (r) register accessor: The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_rwuffr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmt_rwuffr`] module"]
pub type PMT_RWUFFR = crate::Reg<pmt_rwuffr::PMT_RWUFFR_SPEC>;
#[doc = "The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
pub mod pmt_rwuffr;
#[doc = "PMT_CSR (r) register accessor: PMT Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmt_csr`] module"]
pub type PMT_CSR = crate::Reg<pmt_csr::PMT_CSR_SPEC>;
#[doc = "PMT Control and Status"]
pub mod pmt_csr;
#[doc = "EMACLPI_CRS (r) register accessor: LPI Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaclpi_crs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emaclpi_crs`] module"]
pub type EMACLPI_CRS = crate::Reg<emaclpi_crs::EMACLPI_CRS_SPEC>;
#[doc = "LPI Control and Status"]
pub mod emaclpi_crs;
#[doc = "EMACLPITIMERSCONTROL (r) register accessor: LPI Timers Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaclpitimerscontrol::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emaclpitimerscontrol`] module"]
pub type EMACLPITIMERSCONTROL = crate::Reg<emaclpitimerscontrol::EMACLPITIMERSCONTROL_SPEC>;
#[doc = "LPI Timers Control"]
pub mod emaclpitimerscontrol;
#[doc = "EMACINTS (r) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacints::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacints`] module"]
pub type EMACINTS = crate::Reg<emacints::EMACINTS_SPEC>;
#[doc = "Interrupt status"]
pub mod emacints;
#[doc = "EMACINTMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacintmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacintmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacintmask`] module"]
pub type EMACINTMASK = crate::Reg<emacintmask::EMACINTMASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod emacintmask;
#[doc = "EMACADDR0HIGH (rw) register accessor: Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr0high`] module"]
pub type EMACADDR0HIGH = crate::Reg<emacaddr0high::EMACADDR0HIGH_SPEC>;
#[doc = "Upper 16 bits of the first 6-byte MAC address"]
pub mod emacaddr0high;
#[doc = "EMACADDR0LOW (rw) register accessor: This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr0low`] module"]
pub type EMACADDR0LOW = crate::Reg<emacaddr0low::EMACADDR0LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub mod emacaddr0low;
#[doc = "EMACADDR1HIGH (rw) register accessor: Upper 16 bits of the second 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr1high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr1high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr1high`] module"]
pub type EMACADDR1HIGH = crate::Reg<emacaddr1high::EMACADDR1HIGH_SPEC>;
#[doc = "Upper 16 bits of the second 6-byte MAC address"]
pub mod emacaddr1high;
#[doc = "EMACADDR1LOW (rw) register accessor: This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr1low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr1low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr1low`] module"]
pub type EMACADDR1LOW = crate::Reg<emacaddr1low::EMACADDR1LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr1low;
#[doc = "EMACADDR2HIGH (rw) register accessor: Upper 16 bits of the third 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr2high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr2high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr2high`] module"]
pub type EMACADDR2HIGH = crate::Reg<emacaddr2high::EMACADDR2HIGH_SPEC>;
#[doc = "Upper 16 bits of the third 6-byte MAC address"]
pub mod emacaddr2high;
#[doc = "EMACADDR2LOW (rw) register accessor: This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr2low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr2low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr2low`] module"]
pub type EMACADDR2LOW = crate::Reg<emacaddr2low::EMACADDR2LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr2low;
#[doc = "EMACADDR3HIGH (rw) register accessor: Upper 16 bits of the fourth 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr3high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr3high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr3high`] module"]
pub type EMACADDR3HIGH = crate::Reg<emacaddr3high::EMACADDR3HIGH_SPEC>;
#[doc = "Upper 16 bits of the fourth 6-byte MAC address"]
pub mod emacaddr3high;
#[doc = "EMACADDR3LOW (rw) register accessor: This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr3low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr3low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr3low`] module"]
pub type EMACADDR3LOW = crate::Reg<emacaddr3low::EMACADDR3LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr3low;
#[doc = "EMACADDR4HIGH (rw) register accessor: Upper 16 bits of the fifth 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr4high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr4high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr4high`] module"]
pub type EMACADDR4HIGH = crate::Reg<emacaddr4high::EMACADDR4HIGH_SPEC>;
#[doc = "Upper 16 bits of the fifth 6-byte MAC address"]
pub mod emacaddr4high;
#[doc = "EMACADDR4LOW (rw) register accessor: This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr4low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr4low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr4low`] module"]
pub type EMACADDR4LOW = crate::Reg<emacaddr4low::EMACADDR4LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr4low;
#[doc = "EMACADDR5HIGH (rw) register accessor: Upper 16 bits of the sixth 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr5high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr5high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr5high`] module"]
pub type EMACADDR5HIGH = crate::Reg<emacaddr5high::EMACADDR5HIGH_SPEC>;
#[doc = "Upper 16 bits of the sixth 6-byte MAC address"]
pub mod emacaddr5high;
#[doc = "EMACADDR5LOW (rw) register accessor: This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr5low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr5low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr5low`] module"]
pub type EMACADDR5LOW = crate::Reg<emacaddr5low::EMACADDR5LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod emacaddr5low;
#[doc = "EMACADDR6HIGH (rw) register accessor: Upper 16 bits of the seventh 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr6high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr6high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr6high`] module"]
pub type EMACADDR6HIGH = crate::Reg<emacaddr6high::EMACADDR6HIGH_SPEC>;
#[doc = "Upper 16 bits of the seventh 6-byte MAC address"]
pub mod emacaddr6high;
#[doc = "EMACADDR6LOW (rw) register accessor: This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr6low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr6low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr6low`] module"]
pub type EMACADDR6LOW = crate::Reg<emacaddr6low::EMACADDR6LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr6low;
#[doc = "EMACADDR7HIGH (rw) register accessor: Upper 16 bits of the eighth 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr7high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr7high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr7high`] module"]
pub type EMACADDR7HIGH = crate::Reg<emacaddr7high::EMACADDR7HIGH_SPEC>;
#[doc = "Upper 16 bits of the eighth 6-byte MAC address"]
pub mod emacaddr7high;
#[doc = "EMACADDR7LOW (rw) register accessor: This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr7low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr7low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacaddr7low`] module"]
pub type EMACADDR7LOW = crate::Reg<emacaddr7low::EMACADDR7LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod emacaddr7low;
#[doc = "EMACCSTATUS (r) register accessor: Link communication status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaccstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emaccstatus`] module"]
pub type EMACCSTATUS = crate::Reg<emaccstatus::EMACCSTATUS_SPEC>;
#[doc = "Link communication status"]
pub mod emaccstatus;
#[doc = "EMACWDOGTO (rw) register accessor: Watchdog timeout control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacwdogto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacwdogto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`emacwdogto`] module"]
pub type EMACWDOGTO = crate::Reg<emacwdogto::EMACWDOGTO_SPEC>;
#[doc = "Watchdog timeout control"]
pub mod emacwdogto;
