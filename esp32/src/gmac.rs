#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: CONFIG,
    ff: FF,
    _reserved2: [u8; 0x08],
    gmiiaddr: GMIIADDR,
    miidata: MIIDATA,
    fc: FC,
    _reserved5: [u8; 0x08],
    debug: DEBUG,
    pmt_rwuffr: PMT_RWUFFR,
    pmt_csr: PMT_CSR,
    lpi_crs: LPI_CRS,
    lpitimerscontrol: LPITIMERSCONTROL,
    ints: INTS,
    intmask: INTMASK,
    addr0high: ADDR0HIGH,
    addr0low: ADDR0LOW,
    addr1high: ADDR1HIGH,
    addr1low: ADDR1LOW,
    addr2high: ADDR2HIGH,
    addr2low: ADDR2LOW,
    addr3high: ADDR3HIGH,
    addr3low: ADDR3LOW,
    addr4high: ADDR4HIGH,
    addr4low: ADDR4LOW,
    addr5high: ADDR5HIGH,
    addr5low: ADDR5LOW,
    addr6high: ADDR6HIGH,
    addr6low: ADDR6LOW,
    addr7high: ADDR7HIGH,
    addr7low: ADDR7LOW,
    _reserved28: [u8; 0x58],
    cstatus: CSTATUS,
    wdogto: WDOGTO,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Frame filter settings"]
    #[inline(always)]
    pub const fn ff(&self) -> &FF {
        &self.ff
    }
    #[doc = "0x10 - PHY configuration access"]
    #[inline(always)]
    pub const fn gmiiaddr(&self) -> &GMIIADDR {
        &self.gmiiaddr
    }
    #[doc = "0x14 - PHY data read write"]
    #[inline(always)]
    pub const fn miidata(&self) -> &MIIDATA {
        &self.miidata
    }
    #[doc = "0x18 - Frame flow control"]
    #[inline(always)]
    pub const fn fc(&self) -> &FC {
        &self.fc
    }
    #[doc = "0x24 - Status debugging bits"]
    #[inline(always)]
    pub const fn debug(&self) -> &DEBUG {
        &self.debug
    }
    #[doc = "0x28 - The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
    #[inline(always)]
    pub const fn pmt_rwuffr(&self) -> &PMT_RWUFFR {
        &self.pmt_rwuffr
    }
    #[doc = "0x2c - PMT Control and Status"]
    #[inline(always)]
    pub const fn pmt_csr(&self) -> &PMT_CSR {
        &self.pmt_csr
    }
    #[doc = "0x30 - LPI Control and Status"]
    #[inline(always)]
    pub const fn lpi_crs(&self) -> &LPI_CRS {
        &self.lpi_crs
    }
    #[doc = "0x34 - LPI Timers Control"]
    #[inline(always)]
    pub const fn lpitimerscontrol(&self) -> &LPITIMERSCONTROL {
        &self.lpitimerscontrol
    }
    #[doc = "0x38 - Interrupt status"]
    #[inline(always)]
    pub const fn ints(&self) -> &INTS {
        &self.ints
    }
    #[doc = "0x3c - Interrupt mask"]
    #[inline(always)]
    pub const fn intmask(&self) -> &INTMASK {
        &self.intmask
    }
    #[doc = "0x40 - Upper 16 bits of the first 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr0high(&self) -> &ADDR0HIGH {
        &self.addr0high
    }
    #[doc = "0x44 - This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub const fn addr0low(&self) -> &ADDR0LOW {
        &self.addr0low
    }
    #[doc = "0x48 - Upper 16 bits of the second 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr1high(&self) -> &ADDR1HIGH {
        &self.addr1high
    }
    #[doc = "0x4c - This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn addr1low(&self) -> &ADDR1LOW {
        &self.addr1low
    }
    #[doc = "0x50 - Upper 16 bits of the third 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr2high(&self) -> &ADDR2HIGH {
        &self.addr2high
    }
    #[doc = "0x54 - This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn addr2low(&self) -> &ADDR2LOW {
        &self.addr2low
    }
    #[doc = "0x58 - Upper 16 bits of the fourth 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr3high(&self) -> &ADDR3HIGH {
        &self.addr3high
    }
    #[doc = "0x5c - This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn addr3low(&self) -> &ADDR3LOW {
        &self.addr3low
    }
    #[doc = "0x60 - Upper 16 bits of the fifth 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr4high(&self) -> &ADDR4HIGH {
        &self.addr4high
    }
    #[doc = "0x64 - This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn addr4low(&self) -> &ADDR4LOW {
        &self.addr4low
    }
    #[doc = "0x68 - Upper 16 bits of the sixth 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr5high(&self) -> &ADDR5HIGH {
        &self.addr5high
    }
    #[doc = "0x6c - This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
    #[inline(always)]
    pub const fn addr5low(&self) -> &ADDR5LOW {
        &self.addr5low
    }
    #[doc = "0x70 - Upper 16 bits of the seventh 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr6high(&self) -> &ADDR6HIGH {
        &self.addr6high
    }
    #[doc = "0x74 - This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn addr6low(&self) -> &ADDR6LOW {
        &self.addr6low
    }
    #[doc = "0x78 - Upper 16 bits of the eighth 6-byte MAC address"]
    #[inline(always)]
    pub const fn addr7high(&self) -> &ADDR7HIGH {
        &self.addr7high
    }
    #[doc = "0x7c - This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
    #[inline(always)]
    pub const fn addr7low(&self) -> &ADDR7LOW {
        &self.addr7low
    }
    #[doc = "0xd8 - Link communication status"]
    #[inline(always)]
    pub const fn cstatus(&self) -> &CSTATUS {
        &self.cstatus
    }
    #[doc = "0xdc - Watchdog timeout control"]
    #[inline(always)]
    pub const fn wdogto(&self) -> &WDOGTO {
        &self.wdogto
    }
}
#[doc = "CONFIG (rw) register accessor: MAC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "MAC configuration"]
pub mod config;
#[doc = "FF (rw) register accessor: Frame filter settings\n\nYou can [`read`](crate::Reg::read) this register and get [`ff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ff`] module"]
pub type FF = crate::Reg<ff::FF_SPEC>;
#[doc = "Frame filter settings"]
pub mod ff;
#[doc = "GMIIADDR (rw) register accessor: PHY configuration access\n\nYou can [`read`](crate::Reg::read) this register and get [`gmiiaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmiiaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmiiaddr`] module"]
pub type GMIIADDR = crate::Reg<gmiiaddr::GMIIADDR_SPEC>;
#[doc = "PHY configuration access"]
pub mod gmiiaddr;
#[doc = "MIIDATA (rw) register accessor: PHY data read write\n\nYou can [`read`](crate::Reg::read) this register and get [`miidata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miidata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miidata`] module"]
pub type MIIDATA = crate::Reg<miidata::MIIDATA_SPEC>;
#[doc = "PHY data read write"]
pub mod miidata;
#[doc = "FC (rw) register accessor: Frame flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fc`] module"]
pub type FC = crate::Reg<fc::FC_SPEC>;
#[doc = "Frame flow control"]
pub mod fc;
#[doc = "DEBUG (r) register accessor: Status debugging bits\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`] module"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "Status debugging bits"]
pub mod debug;
#[doc = "PMT_RWUFFR (r) register accessor: The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_rwuffr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_rwuffr`] module"]
pub type PMT_RWUFFR = crate::Reg<pmt_rwuffr::PMT_RWUFFR_SPEC>;
#[doc = "The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets"]
pub mod pmt_rwuffr;
#[doc = "PMT_CSR (r) register accessor: PMT Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pmt_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_csr`] module"]
pub type PMT_CSR = crate::Reg<pmt_csr::PMT_CSR_SPEC>;
#[doc = "PMT Control and Status"]
pub mod pmt_csr;
#[doc = "LPI_CRS (r) register accessor: LPI Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lpi_crs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpi_crs`] module"]
pub type LPI_CRS = crate::Reg<lpi_crs::LPI_CRS_SPEC>;
#[doc = "LPI Control and Status"]
pub mod lpi_crs;
#[doc = "LPITIMERSCONTROL (r) register accessor: LPI Timers Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpitimerscontrol::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpitimerscontrol`] module"]
pub type LPITIMERSCONTROL = crate::Reg<lpitimerscontrol::LPITIMERSCONTROL_SPEC>;
#[doc = "LPI Timers Control"]
pub mod lpitimerscontrol;
#[doc = "INTS (r) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ints::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ints`] module"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt status"]
pub mod ints;
#[doc = "INTMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`] module"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intmask;
#[doc = "ADDR0HIGH (rw) register accessor: Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0high`] module"]
pub type ADDR0HIGH = crate::Reg<addr0high::ADDR0HIGH_SPEC>;
#[doc = "Upper 16 bits of the first 6-byte MAC address"]
pub mod addr0high;
#[doc = "ADDR0LOW (rw) register accessor: This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0low`] module"]
pub type ADDR0LOW = crate::Reg<addr0low::ADDR0LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub mod addr0low;
#[doc = "ADDR1HIGH (rw) register accessor: Upper 16 bits of the second 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1high`] module"]
pub type ADDR1HIGH = crate::Reg<addr1high::ADDR1HIGH_SPEC>;
#[doc = "Upper 16 bits of the second 6-byte MAC address"]
pub mod addr1high;
#[doc = "ADDR1LOW (rw) register accessor: This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1low`] module"]
pub type ADDR1LOW = crate::Reg<addr1low::ADDR1LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod addr1low;
#[doc = "ADDR2HIGH (rw) register accessor: Upper 16 bits of the third 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2high`] module"]
pub type ADDR2HIGH = crate::Reg<addr2high::ADDR2HIGH_SPEC>;
#[doc = "Upper 16 bits of the third 6-byte MAC address"]
pub mod addr2high;
#[doc = "ADDR2LOW (rw) register accessor: This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2low`] module"]
pub type ADDR2LOW = crate::Reg<addr2low::ADDR2LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the third 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod addr2low;
#[doc = "ADDR3HIGH (rw) register accessor: Upper 16 bits of the fourth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr3high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3high`] module"]
pub type ADDR3HIGH = crate::Reg<addr3high::ADDR3HIGH_SPEC>;
#[doc = "Upper 16 bits of the fourth 6-byte MAC address"]
pub mod addr3high;
#[doc = "ADDR3LOW (rw) register accessor: This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr3low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3low`] module"]
pub type ADDR3LOW = crate::Reg<addr3low::ADDR3LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the fourth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod addr3low;
#[doc = "ADDR4HIGH (rw) register accessor: Upper 16 bits of the fifth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr4high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr4high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr4high`] module"]
pub type ADDR4HIGH = crate::Reg<addr4high::ADDR4HIGH_SPEC>;
#[doc = "Upper 16 bits of the fifth 6-byte MAC address"]
pub mod addr4high;
#[doc = "ADDR4LOW (rw) register accessor: This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr4low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr4low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr4low`] module"]
pub type ADDR4LOW = crate::Reg<addr4low::ADDR4LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the fifth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod addr4low;
#[doc = "ADDR5HIGH (rw) register accessor: Upper 16 bits of the sixth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr5high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr5high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr5high`] module"]
pub type ADDR5HIGH = crate::Reg<addr5high::ADDR5HIGH_SPEC>;
#[doc = "Upper 16 bits of the sixth 6-byte MAC address"]
pub mod addr5high;
#[doc = "ADDR5LOW (rw) register accessor: This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr5low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr5low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr5low`] module"]
pub type ADDR5LOW = crate::Reg<addr5low::ADDR5LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process."]
pub mod addr5low;
#[doc = "ADDR6HIGH (rw) register accessor: Upper 16 bits of the seventh 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr6high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr6high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr6high`] module"]
pub type ADDR6HIGH = crate::Reg<addr6high::ADDR6HIGH_SPEC>;
#[doc = "Upper 16 bits of the seventh 6-byte MAC address"]
pub mod addr6high;
#[doc = "ADDR6LOW (rw) register accessor: This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr6low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr6low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr6low`] module"]
pub type ADDR6LOW = crate::Reg<addr6low::ADDR6LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the seventh 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod addr6low;
#[doc = "ADDR7HIGH (rw) register accessor: Upper 16 bits of the eighth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr7high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr7high`] module"]
pub type ADDR7HIGH = crate::Reg<addr7high::ADDR7HIGH_SPEC>;
#[doc = "Upper 16 bits of the eighth 6-byte MAC address"]
pub mod addr7high;
#[doc = "ADDR7LOW (rw) register accessor: This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr7low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr7low`] module"]
pub type ADDR7LOW = crate::Reg<addr7low::ADDR7LOW_SPEC>;
#[doc = "This field contains the lower 32 bits of the eighth 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process."]
pub mod addr7low;
#[doc = "CSTATUS (r) register accessor: Link communication status\n\nYou can [`read`](crate::Reg::read) this register and get [`cstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstatus`] module"]
pub type CSTATUS = crate::Reg<cstatus::CSTATUS_SPEC>;
#[doc = "Link communication status"]
pub mod cstatus;
#[doc = "WDOGTO (rw) register accessor: Watchdog timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogto`] module"]
pub type WDOGTO = crate::Reg<wdogto::WDOGTO_SPEC>;
#[doc = "Watchdog timeout control"]
pub mod wdogto;
