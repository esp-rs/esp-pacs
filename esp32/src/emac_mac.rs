#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    emacconfig: EMACCONFIG,
    emacff: EMACFF,
    _reserved2: [u8; 0x08],
    emacgmiiaddr: EMACGMIIADDR,
    emacmiidata: EMACMIIDATA,
    emacfc: EMACFC,
    _reserved5: [u8; 0x08],
    emacdebug: EMACDEBUG,
    pmt_rwuffr: PMT_RWUFFR,
    pmt_csr: PMT_CSR,
    emaclpi_crs: EMACLPI_CRS,
    emaclpitimerscontrol: EMACLPITIMERSCONTROL,
    emacints: EMACINTS,
    emacintmask: EMACINTMASK,
    emacaddr0high: EMACADDR0HIGH,
    emacaddr0low: EMACADDR0LOW,
    emacaddr1high: EMACADDR1HIGH,
    emacaddr1low: EMACADDR1LOW,
    emacaddr2high: EMACADDR2HIGH,
    emacaddr2low: EMACADDR2LOW,
    emacaddr3high: EMACADDR3HIGH,
    emacaddr3low: EMACADDR3LOW,
    emacaddr4high: EMACADDR4HIGH,
    emacaddr4low: EMACADDR4LOW,
    emacaddr5high: EMACADDR5HIGH,
    emacaddr5low: EMACADDR5LOW,
    emacaddr6high: EMACADDR6HIGH,
    emacaddr6low: EMACADDR6LOW,
    emacaddr7high: EMACADDR7HIGH,
    emacaddr7low: EMACADDR7LOW,
    _reserved28: [u8; 0x58],
    emaccstatus: EMACCSTATUS,
    emacwdogto: EMACWDOGTO,
}
impl RegisterBlock {
    ///0x00 - MAC configuration
    #[inline(always)]
    pub const fn emacconfig(&self) -> &EMACCONFIG {
        &self.emacconfig
    }
    ///0x04 - Frame filter settings
    #[inline(always)]
    pub const fn emacff(&self) -> &EMACFF {
        &self.emacff
    }
    ///0x10 - PHY configuration access
    #[inline(always)]
    pub const fn emacgmiiaddr(&self) -> &EMACGMIIADDR {
        &self.emacgmiiaddr
    }
    ///0x14 - PHY data read write
    #[inline(always)]
    pub const fn emacmiidata(&self) -> &EMACMIIDATA {
        &self.emacmiidata
    }
    ///0x18 - Frame flow control
    #[inline(always)]
    pub const fn emacfc(&self) -> &EMACFC {
        &self.emacfc
    }
    ///0x24 - Status debugging bits
    #[inline(always)]
    pub const fn emacdebug(&self) -> &EMACDEBUG {
        &self.emacdebug
    }
    ///0x28 - The MSB (31st bit) must be zero.Bit j\[30:0\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets
    #[inline(always)]
    pub const fn pmt_rwuffr(&self) -> &PMT_RWUFFR {
        &self.pmt_rwuffr
    }
    ///0x2c - PMT Control and Status
    #[inline(always)]
    pub const fn pmt_csr(&self) -> &PMT_CSR {
        &self.pmt_csr
    }
    ///0x30 - LPI Control and Status
    #[inline(always)]
    pub const fn emaclpi_crs(&self) -> &EMACLPI_CRS {
        &self.emaclpi_crs
    }
    ///0x34 - LPI Timers Control
    #[inline(always)]
    pub const fn emaclpitimerscontrol(&self) -> &EMACLPITIMERSCONTROL {
        &self.emaclpitimerscontrol
    }
    ///0x38 - Interrupt status
    #[inline(always)]
    pub const fn emacints(&self) -> &EMACINTS {
        &self.emacints
    }
    ///0x3c - Interrupt mask
    #[inline(always)]
    pub const fn emacintmask(&self) -> &EMACINTMASK {
        &self.emacintmask
    }
    ///0x40 - Upper 16 bits of the first 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr0high(&self) -> &EMACADDR0HIGH {
        &self.emacaddr0high
    }
    ///0x44 - This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.
    #[inline(always)]
    pub const fn emacaddr0low(&self) -> &EMACADDR0LOW {
        &self.emacaddr0low
    }
    ///0x48 - Upper 16 bits of the second 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr1high(&self) -> &EMACADDR1HIGH {
        &self.emacaddr1high
    }
    ///0x4c - This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
    #[inline(always)]
    pub const fn emacaddr1low(&self) -> &EMACADDR1LOW {
        &self.emacaddr1low
    }
    ///0x50 - Upper 16 bits of the third 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr2high(&self) -> &EMACADDR2HIGH {
        &self.emacaddr2high
    }
    ///0x54 - This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
    #[inline(always)]
    pub const fn emacaddr2low(&self) -> &EMACADDR2LOW {
        &self.emacaddr2low
    }
    ///0x58 - Upper 16 bits of the fourth 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr3high(&self) -> &EMACADDR3HIGH {
        &self.emacaddr3high
    }
    ///0x5c - This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
    #[inline(always)]
    pub const fn emacaddr3low(&self) -> &EMACADDR3LOW {
        &self.emacaddr3low
    }
    ///0x60 - Upper 16 bits of the fifth 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr4high(&self) -> &EMACADDR4HIGH {
        &self.emacaddr4high
    }
    ///0x64 - This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
    #[inline(always)]
    pub const fn emacaddr4low(&self) -> &EMACADDR4LOW {
        &self.emacaddr4low
    }
    ///0x68 - Upper 16 bits of the sixth 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr5high(&self) -> &EMACADDR5HIGH {
        &self.emacaddr5high
    }
    ///0x6c - This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
    #[inline(always)]
    pub const fn emacaddr5low(&self) -> &EMACADDR5LOW {
        &self.emacaddr5low
    }
    ///0x70 - Upper 16 bits of the seventh 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr6high(&self) -> &EMACADDR6HIGH {
        &self.emacaddr6high
    }
    ///0x74 - This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
    #[inline(always)]
    pub const fn emacaddr6low(&self) -> &EMACADDR6LOW {
        &self.emacaddr6low
    }
    ///0x78 - Upper 16 bits of the eighth 6-byte MAC address
    #[inline(always)]
    pub const fn emacaddr7high(&self) -> &EMACADDR7HIGH {
        &self.emacaddr7high
    }
    ///0x7c - This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
    #[inline(always)]
    pub const fn emacaddr7low(&self) -> &EMACADDR7LOW {
        &self.emacaddr7low
    }
    ///0xd8 - Link communication status
    #[inline(always)]
    pub const fn emaccstatus(&self) -> &EMACCSTATUS {
        &self.emaccstatus
    }
    ///0xdc - Watchdog timeout control
    #[inline(always)]
    pub const fn emacwdogto(&self) -> &EMACWDOGTO {
        &self.emacwdogto
    }
}
/**EMACCONFIG (rw) register accessor: MAC configuration

You can [`read`](crate::generic::Reg::read) this register and get [`emacconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacconfig`] module*/
pub type EMACCONFIG = crate::Reg<emacconfig::EMACCONFIG_SPEC>;
///MAC configuration
pub mod emacconfig;
/**EMACFF (rw) register accessor: Frame filter settings

You can [`read`](crate::generic::Reg::read) this register and get [`emacff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacff`] module*/
pub type EMACFF = crate::Reg<emacff::EMACFF_SPEC>;
///Frame filter settings
pub mod emacff;
/**EMACGMIIADDR (rw) register accessor: PHY configuration access

You can [`read`](crate::generic::Reg::read) this register and get [`emacgmiiaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacgmiiaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacgmiiaddr`] module*/
pub type EMACGMIIADDR = crate::Reg<emacgmiiaddr::EMACGMIIADDR_SPEC>;
///PHY configuration access
pub mod emacgmiiaddr;
/**EMACMIIDATA (rw) register accessor: PHY data read write

You can [`read`](crate::generic::Reg::read) this register and get [`emacmiidata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacmiidata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacmiidata`] module*/
pub type EMACMIIDATA = crate::Reg<emacmiidata::EMACMIIDATA_SPEC>;
///PHY data read write
pub mod emacmiidata;
/**EMACFC (rw) register accessor: Frame flow control

You can [`read`](crate::generic::Reg::read) this register and get [`emacfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacfc`] module*/
pub type EMACFC = crate::Reg<emacfc::EMACFC_SPEC>;
///Frame flow control
pub mod emacfc;
/**EMACDEBUG (r) register accessor: Status debugging bits

You can [`read`](crate::generic::Reg::read) this register and get [`emacdebug::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacdebug`] module*/
pub type EMACDEBUG = crate::Reg<emacdebug::EMACDEBUG_SPEC>;
///Status debugging bits
pub mod emacdebug;
/**PMT_RWUFFR (r) register accessor: The MSB (31st bit) must be zero.Bit j\[30:0\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets

You can [`read`](crate::generic::Reg::read) this register and get [`pmt_rwuffr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmt_rwuffr`] module*/
pub type PMT_RWUFFR = crate::Reg<pmt_rwuffr::PMT_RWUFFR_SPEC>;
///The MSB (31st bit) must be zero.Bit j\[30:0\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets
pub mod pmt_rwuffr;
/**PMT_CSR (r) register accessor: PMT Control and Status

You can [`read`](crate::generic::Reg::read) this register and get [`pmt_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pmt_csr`] module*/
pub type PMT_CSR = crate::Reg<pmt_csr::PMT_CSR_SPEC>;
///PMT Control and Status
pub mod pmt_csr;
/**EMACLPI_CRS (r) register accessor: LPI Control and Status

You can [`read`](crate::generic::Reg::read) this register and get [`emaclpi_crs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emaclpi_crs`] module*/
pub type EMACLPI_CRS = crate::Reg<emaclpi_crs::EMACLPI_CRS_SPEC>;
///LPI Control and Status
pub mod emaclpi_crs;
/**EMACLPITIMERSCONTROL (r) register accessor: LPI Timers Control

You can [`read`](crate::generic::Reg::read) this register and get [`emaclpitimerscontrol::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emaclpitimerscontrol`] module*/
pub type EMACLPITIMERSCONTROL = crate::Reg<emaclpitimerscontrol::EMACLPITIMERSCONTROL_SPEC>;
///LPI Timers Control
pub mod emaclpitimerscontrol;
/**EMACINTS (r) register accessor: Interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`emacints::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacints`] module*/
pub type EMACINTS = crate::Reg<emacints::EMACINTS_SPEC>;
///Interrupt status
pub mod emacints;
/**EMACINTMASK (rw) register accessor: Interrupt mask

You can [`read`](crate::generic::Reg::read) this register and get [`emacintmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacintmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacintmask`] module*/
pub type EMACINTMASK = crate::Reg<emacintmask::EMACINTMASK_SPEC>;
///Interrupt mask
pub mod emacintmask;
/**EMACADDR0HIGH (rw) register accessor: Upper 16 bits of the first 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr0high`] module*/
pub type EMACADDR0HIGH = crate::Reg<emacaddr0high::EMACADDR0HIGH_SPEC>;
///Upper 16 bits of the first 6-byte MAC address
pub mod emacaddr0high;
/**EMACADDR0LOW (rw) register accessor: This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr0low`] module*/
pub type EMACADDR0LOW = crate::Reg<emacaddr0low::EMACADDR0LOW_SPEC>;
///This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.
pub mod emacaddr0low;
/**EMACADDR1HIGH (rw) register accessor: Upper 16 bits of the second 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr1high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr1high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr1high`] module*/
pub type EMACADDR1HIGH = crate::Reg<emacaddr1high::EMACADDR1HIGH_SPEC>;
///Upper 16 bits of the second 6-byte MAC address
pub mod emacaddr1high;
/**EMACADDR1LOW (rw) register accessor: This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr1low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr1low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr1low`] module*/
pub type EMACADDR1LOW = crate::Reg<emacaddr1low::EMACADDR1LOW_SPEC>;
///This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
pub mod emacaddr1low;
/**EMACADDR2HIGH (rw) register accessor: Upper 16 bits of the third 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr2high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr2high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr2high`] module*/
pub type EMACADDR2HIGH = crate::Reg<emacaddr2high::EMACADDR2HIGH_SPEC>;
///Upper 16 bits of the third 6-byte MAC address
pub mod emacaddr2high;
/**EMACADDR2LOW (rw) register accessor: This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr2low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr2low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr2low`] module*/
pub type EMACADDR2LOW = crate::Reg<emacaddr2low::EMACADDR2LOW_SPEC>;
///This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
pub mod emacaddr2low;
/**EMACADDR3HIGH (rw) register accessor: Upper 16 bits of the fourth 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr3high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr3high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr3high`] module*/
pub type EMACADDR3HIGH = crate::Reg<emacaddr3high::EMACADDR3HIGH_SPEC>;
///Upper 16 bits of the fourth 6-byte MAC address
pub mod emacaddr3high;
/**EMACADDR3LOW (rw) register accessor: This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr3low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr3low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr3low`] module*/
pub type EMACADDR3LOW = crate::Reg<emacaddr3low::EMACADDR3LOW_SPEC>;
///This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
pub mod emacaddr3low;
/**EMACADDR4HIGH (rw) register accessor: Upper 16 bits of the fifth 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr4high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr4high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr4high`] module*/
pub type EMACADDR4HIGH = crate::Reg<emacaddr4high::EMACADDR4HIGH_SPEC>;
///Upper 16 bits of the fifth 6-byte MAC address
pub mod emacaddr4high;
/**EMACADDR4LOW (rw) register accessor: This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr4low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr4low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr4low`] module*/
pub type EMACADDR4LOW = crate::Reg<emacaddr4low::EMACADDR4LOW_SPEC>;
///This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
pub mod emacaddr4low;
/**EMACADDR5HIGH (rw) register accessor: Upper 16 bits of the sixth 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr5high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr5high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr5high`] module*/
pub type EMACADDR5HIGH = crate::Reg<emacaddr5high::EMACADDR5HIGH_SPEC>;
///Upper 16 bits of the sixth 6-byte MAC address
pub mod emacaddr5high;
/**EMACADDR5LOW (rw) register accessor: This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr5low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr5low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr5low`] module*/
pub type EMACADDR5LOW = crate::Reg<emacaddr5low::EMACADDR5LOW_SPEC>;
///This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.
pub mod emacaddr5low;
/**EMACADDR6HIGH (rw) register accessor: Upper 16 bits of the seventh 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr6high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr6high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr6high`] module*/
pub type EMACADDR6HIGH = crate::Reg<emacaddr6high::EMACADDR6HIGH_SPEC>;
///Upper 16 bits of the seventh 6-byte MAC address
pub mod emacaddr6high;
/**EMACADDR6LOW (rw) register accessor: This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr6low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr6low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr6low`] module*/
pub type EMACADDR6LOW = crate::Reg<emacaddr6low::EMACADDR6LOW_SPEC>;
///This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
pub mod emacaddr6low;
/**EMACADDR7HIGH (rw) register accessor: Upper 16 bits of the eighth 6-byte MAC address

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr7high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr7high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr7high`] module*/
pub type EMACADDR7HIGH = crate::Reg<emacaddr7high::EMACADDR7HIGH_SPEC>;
///Upper 16 bits of the eighth 6-byte MAC address
pub mod emacaddr7high;
/**EMACADDR7LOW (rw) register accessor: This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr7low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr7low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacaddr7low`] module*/
pub type EMACADDR7LOW = crate::Reg<emacaddr7low::EMACADDR7LOW_SPEC>;
///This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.
pub mod emacaddr7low;
/**EMACCSTATUS (r) register accessor: Link communication status

You can [`read`](crate::generic::Reg::read) this register and get [`emaccstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emaccstatus`] module*/
pub type EMACCSTATUS = crate::Reg<emaccstatus::EMACCSTATUS_SPEC>;
///Link communication status
pub mod emaccstatus;
/**EMACWDOGTO (rw) register accessor: Watchdog timeout control

You can [`read`](crate::generic::Reg::read) this register and get [`emacwdogto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacwdogto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@emacwdogto`] module*/
pub type EMACWDOGTO = crate::Reg<emacwdogto::EMACWDOGTO_SPEC>;
///Watchdog timeout control
pub mod emacwdogto;
