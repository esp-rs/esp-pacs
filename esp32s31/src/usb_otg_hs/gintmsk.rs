#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSK_SPEC>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSK_SPEC>;
#[doc = "Field `MODEMISMSK` reader - Mode: Host and Device Mode Mismatch Interrupt Mask (ModeMisMsk)"]
pub type MODEMISMSK_R = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode: Host and Device Mode Mismatch Interrupt Mask (ModeMisMsk)"]
pub type MODEMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINTMSK` reader - Mode: Host and Device OTG Interrupt Mask (OTGIntMsk)"]
pub type OTGINTMSK_R = crate::BitReader;
#[doc = "Field `OTGINTMSK` writer - Mode: Host and Device OTG Interrupt Mask (OTGIntMsk)"]
pub type OTGINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Mode: Host and Device Start of (micro)Frame Mask (SofMsk)"]
pub type SOFMSK_R = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Mode: Host and Device Start of (micro)Frame Mask (SofMsk)"]
pub type SOFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Mode: Host and Device Receive FIFO Non-Empty Mask (RxFLvlMsk)"]
pub type RXFLVLMSK_R = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Mode: Host and Device Receive FIFO Non-Empty Mask (RxFLvlMsk)"]
pub type RXFLVLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPMSK` reader - Mode: Host and Device Non-periodic TxFIFO Empty Mask (NPTxFEmpMsk)"]
pub type NPTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `NPTXFEMPMSK` writer - Mode: Host and Device Non-periodic TxFIFO Empty Mask (NPTxFEmpMsk)"]
pub type NPTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Mode: Device only, Global Non-periodic IN NAK Effective Mask (GINNakEffMsk)"]
pub type GINNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Mode: Device only, Global Non-periodic IN NAK Effective Mask (GINNakEffMsk)"]
pub type GINNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Mode: Device only Global OUT NAK Effective Mask (GOUTNakEffMsk)"]
pub type GOUTNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Mode: Device only Global OUT NAK Effective Mask (GOUTNakEffMsk)"]
pub type GOUTNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Mode: Device only Early Suspend Mask (ErlySuspMsk)"]
pub type ERLYSUSPMSK_R = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Mode: Device only Early Suspend Mask (ErlySuspMsk)"]
pub type ERLYSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - Mode: Device only USB Suspend Mask (USBSuspMsk)"]
pub type USBSUSPMSK_R = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - Mode: Device only USB Suspend Mask (USBSuspMsk)"]
pub type USBSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - Mode: Device only USB Reset Mask (USBRstMsk)"]
pub type USBRSTMSK_R = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - Mode: Device only USB Reset Mask (USBRstMsk)"]
pub type USBRSTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Mode: Device only Enumeration Done Mask (EnumDoneMsk)"]
pub type ENUMDONEMSK_R = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Mode: Device only Enumeration Done Mask (EnumDoneMsk)"]
pub type ENUMDONEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Mode: Device only Isochronous OUT Packet Dropped Interrupt Mask (ISOOutDropMsk)"]
pub type ISOOUTDROPMSK_R = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Mode: Device only Isochronous OUT Packet Dropped Interrupt Mask (ISOOutDropMsk)"]
pub type ISOOUTDROPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - Mode: Device only End of Periodic Frame Interrupt Mask (EOPFMsk)"]
pub type EOPFMSK_R = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - Mode: Device only End of Periodic Frame Interrupt Mask (EOPFMsk)"]
pub type EOPFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMISMSK` reader - Mode: Device only Endpoint Mismatch Interrupt Mask (EPMisMsk)"]
pub type EPMISMSK_R = crate::BitReader;
#[doc = "Field `EPMISMSK` writer - Mode: Device only Endpoint Mismatch Interrupt Mask (EPMisMsk)"]
pub type EPMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINTMSK` reader - Mode: Device only IN Endpoints Interrupt Mask (IEPIntMsk)"]
pub type IEPINTMSK_R = crate::BitReader;
#[doc = "Field `IEPINTMSK` writer - Mode: Device only IN Endpoints Interrupt Mask (IEPIntMsk)"]
pub type IEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPINTMSK` reader - Mode: Device only OUT Endpoints Interrupt Mask (OEPIntMsk)"]
pub type OEPINTMSK_R = crate::BitReader;
#[doc = "Field `OEPINTMSK` writer - Mode: Device only OUT Endpoints Interrupt Mask (OEPIntMsk)"]
pub type OEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask (incomplPMsk) Mode: Host only Incomplete Isochronous OUT Transfer Interrupt Mask (incompISOOUTMsk) Mode: Device only"]
pub type INCOMPLPMSK_R = crate::BitReader;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask (incomplPMsk) Mode: Host only Incomplete Isochronous OUT Transfer Interrupt Mask (incompISOOUTMsk) Mode: Device only"]
pub type INCOMPLPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSPMSK` reader - Mode: Device only Data Fetch Suspended Mask (FetSuspMsk)"]
pub type FETSUSPMSK_R = crate::BitReader;
#[doc = "Field `FETSUSPMSK` writer - Mode: Device only Data Fetch Suspended Mask (FetSuspMsk)"]
pub type FETSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDETMSK` reader - Mode: Device only Reset detected Interrupt Mask (ResetDetMsk)"]
pub type RESETDETMSK_R = crate::BitReader;
#[doc = "Field `RESETDETMSK` writer - Mode: Device only Reset detected Interrupt Mask (ResetDetMsk)"]
pub type RESETDETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINTMSK` reader - Mode: Host only Host Port Interrupt Mask (PrtIntMsk)"]
pub type PRTINTMSK_R = crate::BitReader;
#[doc = "Field `PRTINTMSK` writer - Mode: Host only Host Port Interrupt Mask (PrtIntMsk)"]
pub type PRTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCHINTMSK` reader - Mode: Host only Host Channels Interrupt Mask (HChIntMsk)"]
pub type HCHINTMSK_R = crate::BitReader;
#[doc = "Field `HCHINTMSK` writer - Mode: Host only Host Channels Interrupt Mask (HChIntMsk)"]
pub type HCHINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPMSK` reader - Mode: Host only Periodic TxFIFO Empty Mask (PTxFEmpMsk)"]
pub type PTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `PTXFEMPMSK` writer - Mode: Host only Periodic TxFIFO Empty Mask (PTxFEmpMsk)"]
pub type PTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSTSCHNGMSK` reader - Mode: Host and Device Connector ID Status Change Mask (ConIDStsChngMsk)"]
pub type CONIDSTSCHNGMSK_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNGMSK` writer - Mode: Host and Device Connector ID Status Change Mask (ConIDStsChngMsk)"]
pub type CONIDSTSCHNGMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONNINTMSK` reader - Mode: Host and Device Disconnect Detected Interrupt Mask (DisconnIntMsk)"]
pub type DISCONNINTMSK_R = crate::BitReader;
#[doc = "Field `DISCONNINTMSK` writer - Mode: Host and Device Disconnect Detected Interrupt Mask (DisconnIntMsk)"]
pub type DISCONNINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSREQINTMSK` reader - Mode: Host and Device Session Request/New Session Detected Interrupt Mask (SessReqIntMsk)"]
pub type SESSREQINTMSK_R = crate::BitReader;
#[doc = "Field `SESSREQINTMSK` writer - Mode: Host and Device Session Request/New Session Detected Interrupt Mask (SessReqIntMsk)"]
pub type SESSREQINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt Mask (WkUpIntMsk) The WakeUp bit is used for LPM state wake up in a way similar to that of wake up in suspend state."]
pub type WKUPINTMSK_R = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt Mask (WkUpIntMsk) The WakeUp bit is used for LPM state wake up in a way similar to that of wake up in suspend state."]
pub type WKUPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode: Host and Device Mode Mismatch Interrupt Mask (ModeMisMsk)"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode: Host and Device OTG Interrupt Mask (OTGIntMsk)"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mode: Host and Device Start of (micro)Frame Mask (SofMsk)"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode: Host and Device Receive FIFO Non-Empty Mask (RxFLvlMsk)"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode: Host and Device Non-periodic TxFIFO Empty Mask (NPTxFEmpMsk)"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode: Device only, Global Non-periodic IN NAK Effective Mask (GINNakEffMsk)"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mode: Device only Global OUT NAK Effective Mask (GOUTNakEffMsk)"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Mode: Device only Early Suspend Mask (ErlySuspMsk)"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mode: Device only USB Suspend Mask (USBSuspMsk)"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mode: Device only USB Reset Mask (USBRstMsk)"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mode: Device only Enumeration Done Mask (EnumDoneMsk)"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mode: Device only Isochronous OUT Packet Dropped Interrupt Mask (ISOOutDropMsk)"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mode: Device only End of Periodic Frame Interrupt Mask (EOPFMsk)"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Mode: Device only Endpoint Mismatch Interrupt Mask (EPMisMsk)"]
    #[inline(always)]
    pub fn epmismsk(&self) -> EPMISMSK_R {
        EPMISMSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mode: Device only IN Endpoints Interrupt Mask (IEPIntMsk)"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mode: Device only OUT Endpoints Interrupt Mask (OEPIntMsk)"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (incomplPMsk) Mode: Host only Incomplete Isochronous OUT Transfer Interrupt Mask (incompISOOUTMsk) Mode: Device only"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mode: Device only Data Fetch Suspended Mask (FetSuspMsk)"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mode: Device only Reset detected Interrupt Mask (ResetDetMsk)"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mode: Host only Host Port Interrupt Mask (PrtIntMsk)"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mode: Host only Host Channels Interrupt Mask (HChIntMsk)"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode: Host only Periodic TxFIFO Empty Mask (PTxFEmpMsk)"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Mode: Host and Device Connector ID Status Change Mask (ConIDStsChngMsk)"]
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mode: Host and Device Disconnect Detected Interrupt Mask (DisconnIntMsk)"]
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mode: Host and Device Session Request/New Session Detected Interrupt Mask (SessReqIntMsk)"]
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt Mask (WkUpIntMsk) The WakeUp bit is used for LPM state wake up in a way similar to that of wake up in suspend state."]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("modemismsk", &self.modemismsk())
            .field("otgintmsk", &self.otgintmsk())
            .field("sofmsk", &self.sofmsk())
            .field("rxflvlmsk", &self.rxflvlmsk())
            .field("nptxfempmsk", &self.nptxfempmsk())
            .field("ginnakeffmsk", &self.ginnakeffmsk())
            .field("goutnakeffmsk", &self.goutnakeffmsk())
            .field("erlysuspmsk", &self.erlysuspmsk())
            .field("usbsuspmsk", &self.usbsuspmsk())
            .field("usbrstmsk", &self.usbrstmsk())
            .field("enumdonemsk", &self.enumdonemsk())
            .field("isooutdropmsk", &self.isooutdropmsk())
            .field("eopfmsk", &self.eopfmsk())
            .field("epmismsk", &self.epmismsk())
            .field("iepintmsk", &self.iepintmsk())
            .field("oepintmsk", &self.oepintmsk())
            .field("incomplpmsk", &self.incomplpmsk())
            .field("fetsuspmsk", &self.fetsuspmsk())
            .field("resetdetmsk", &self.resetdetmsk())
            .field("prtintmsk", &self.prtintmsk())
            .field("hchintmsk", &self.hchintmsk())
            .field("ptxfempmsk", &self.ptxfempmsk())
            .field("conidstschngmsk", &self.conidstschngmsk())
            .field("disconnintmsk", &self.disconnintmsk())
            .field("sessreqintmsk", &self.sessreqintmsk())
            .field("wkupintmsk", &self.wkupintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Mode: Host and Device Mode Mismatch Interrupt Mask (ModeMisMsk)"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<'_, GINTMSK_SPEC> {
        MODEMISMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mode: Host and Device OTG Interrupt Mask (OTGIntMsk)"]
    #[inline(always)]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W<'_, GINTMSK_SPEC> {
        OTGINTMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mode: Host and Device Start of (micro)Frame Mask (SofMsk)"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SOFMSK_W<'_, GINTMSK_SPEC> {
        SOFMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mode: Host and Device Receive FIFO Non-Empty Mask (RxFLvlMsk)"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W<'_, GINTMSK_SPEC> {
        RXFLVLMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mode: Host and Device Non-periodic TxFIFO Empty Mask (NPTxFEmpMsk)"]
    #[inline(always)]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W<'_, GINTMSK_SPEC> {
        NPTXFEMPMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mode: Device only, Global Non-periodic IN NAK Effective Mask (GINNakEffMsk)"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<'_, GINTMSK_SPEC> {
        GINNAKEFFMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mode: Device only Global OUT NAK Effective Mask (GOUTNakEffMsk)"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W<'_, GINTMSK_SPEC> {
        GOUTNAKEFFMSK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Mode: Device only Early Suspend Mask (ErlySuspMsk)"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<'_, GINTMSK_SPEC> {
        ERLYSUSPMSK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Mode: Device only USB Suspend Mask (USBSuspMsk)"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<'_, GINTMSK_SPEC> {
        USBSUSPMSK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Mode: Device only USB Reset Mask (USBRstMsk)"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<'_, GINTMSK_SPEC> {
        USBRSTMSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Mode: Device only Enumeration Done Mask (EnumDoneMsk)"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<'_, GINTMSK_SPEC> {
        ENUMDONEMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Mode: Device only Isochronous OUT Packet Dropped Interrupt Mask (ISOOutDropMsk)"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<'_, GINTMSK_SPEC> {
        ISOOUTDROPMSK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Mode: Device only End of Periodic Frame Interrupt Mask (EOPFMsk)"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<'_, GINTMSK_SPEC> {
        EOPFMSK_W::new(self, 15)
    }
    #[doc = "Bit 17 - Mode: Device only Endpoint Mismatch Interrupt Mask (EPMisMsk)"]
    #[inline(always)]
    pub fn epmismsk(&mut self) -> EPMISMSK_W<'_, GINTMSK_SPEC> {
        EPMISMSK_W::new(self, 17)
    }
    #[doc = "Bit 18 - Mode: Device only IN Endpoints Interrupt Mask (IEPIntMsk)"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<'_, GINTMSK_SPEC> {
        IEPINTMSK_W::new(self, 18)
    }
    #[doc = "Bit 19 - Mode: Device only OUT Endpoints Interrupt Mask (OEPIntMsk)"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<'_, GINTMSK_SPEC> {
        OEPINTMSK_W::new(self, 19)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask (incomplPMsk) Mode: Host only Incomplete Isochronous OUT Transfer Interrupt Mask (incompISOOUTMsk) Mode: Device only"]
    #[inline(always)]
    pub fn incomplpmsk(&mut self) -> INCOMPLPMSK_W<'_, GINTMSK_SPEC> {
        INCOMPLPMSK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Mode: Device only Data Fetch Suspended Mask (FetSuspMsk)"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<'_, GINTMSK_SPEC> {
        FETSUSPMSK_W::new(self, 22)
    }
    #[doc = "Bit 23 - Mode: Device only Reset detected Interrupt Mask (ResetDetMsk)"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<'_, GINTMSK_SPEC> {
        RESETDETMSK_W::new(self, 23)
    }
    #[doc = "Bit 24 - Mode: Host only Host Port Interrupt Mask (PrtIntMsk)"]
    #[inline(always)]
    pub fn prtintmsk(&mut self) -> PRTINTMSK_W<'_, GINTMSK_SPEC> {
        PRTINTMSK_W::new(self, 24)
    }
    #[doc = "Bit 25 - Mode: Host only Host Channels Interrupt Mask (HChIntMsk)"]
    #[inline(always)]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W<'_, GINTMSK_SPEC> {
        HCHINTMSK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mode: Host only Periodic TxFIFO Empty Mask (PTxFEmpMsk)"]
    #[inline(always)]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W<'_, GINTMSK_SPEC> {
        PTXFEMPMSK_W::new(self, 26)
    }
    #[doc = "Bit 28 - Mode: Host and Device Connector ID Status Change Mask (ConIDStsChngMsk)"]
    #[inline(always)]
    pub fn conidstschngmsk(&mut self) -> CONIDSTSCHNGMSK_W<'_, GINTMSK_SPEC> {
        CONIDSTSCHNGMSK_W::new(self, 28)
    }
    #[doc = "Bit 29 - Mode: Host and Device Disconnect Detected Interrupt Mask (DisconnIntMsk)"]
    #[inline(always)]
    pub fn disconnintmsk(&mut self) -> DISCONNINTMSK_W<'_, GINTMSK_SPEC> {
        DISCONNINTMSK_W::new(self, 29)
    }
    #[doc = "Bit 30 - Mode: Host and Device Session Request/New Session Detected Interrupt Mask (SessReqIntMsk)"]
    #[inline(always)]
    pub fn sessreqintmsk(&mut self) -> SESSREQINTMSK_W<'_, GINTMSK_SPEC> {
        SESSREQINTMSK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt Mask (WkUpIntMsk) The WakeUp bit is used for LPM state wake up in a way similar to that of wake up in suspend state."]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<'_, GINTMSK_SPEC> {
        WKUPINTMSK_W::new(self, 31)
    }
}
#[doc = "This register works with the Interrupt Register (GINTSTS) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {}
