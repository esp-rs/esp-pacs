#[doc = "Register `GHWCFG4` reader"]
pub type R = crate::R<GHWCFG4_SPEC>;
#[doc = "Field `NUMDEVPERIOEPS` reader - Number of Device Mode Periodic IN Endpoints (NumDevPerioEps) Range: 0-15"]
pub type NUMDEVPERIOEPS_R = crate::FieldReader;
#[doc = "Field `PARTIALPWRDN` reader - Enable Partial Power Down (PartialPwrDn) - 1'b0: Partial Power Down Not Enabled - 1'b1: Partial Power Down Enabled"]
pub type PARTIALPWRDN_R = crate::BitReader;
#[doc = "Field `AHBFREQ` reader - Minimum AHB Frequency Less Than 60 MHz (AhbFreq) - 1'b0: No - 1'b1: Yes"]
pub type AHBFREQ_R = crate::BitReader;
#[doc = "Field `HIBERNATION` reader - Enable Hibernation (Hibernation) - 1'b0: Hibernation feature not enabled - 1'b1: Hibernation feature enabled"]
pub type HIBERNATION_R = crate::BitReader;
#[doc = "Field `EXTENDEDHIBERNATION` reader - Enable Hibernation - 1'b0: Extended Hibernation feature not enabled - 1'b1: Extended Hibernation feature enabled"]
pub type EXTENDEDHIBERNATION_R = crate::BitReader;
#[doc = "Field `ENHANCEDLPMSUPT1` reader - Enhanced LPM Support1 (EnhancedLPMSupt1) - This bit indicates that the controller supports L1 entry based on FIFO status. - Accept L1 Request even if Bulk/Interrupt TxFIFO is not empty."]
pub type ENHANCEDLPMSUPT1_R = crate::BitReader;
#[doc = "Field `SERVINTFLOW` reader - Service Interval Flow This bit indicates that the controller supports Service-Interval based scheduling flow for ISOC IN EPs."]
pub type SERVINTFLOW_R = crate::BitReader;
#[doc = "Field `IPGISOCSUPT` reader - Interpacket Gap ISOC OUT Worst-case Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter Packet Gap (IPG) (32-bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency.By default IPG Support is enabled."]
pub type IPGISOCSUPT_R = crate::BitReader;
#[doc = "Field `ACGSUPT` reader - Active Clock Gating Support This bit indicates that the controller supports the Dynamic (Switching) Power Reduction during periods when there is no USB and AHB Traffic. - 1'b0: Active Clock Gating is not enabled. - 1'b1: Active Clock Gating Enabled."]
pub type ACGSUPT_R = crate::BitReader;
#[doc = "Field `ENHANCEDLPMSUPT` reader - Enhanced LPM Support (EnhancedLPMSupt) This bit indicates that the controller supports the following behavior: L1 Entry Behavior based on FIFO Status - TX FIFO - Accept L1 Request even if ISOC IN TX FIFO is not empty. - Reject L1 Request if Non-Periodic TX FIFO is not empty. - Ensure application can flush the TX FIFO while the Controller is in L1. - RX FIFO - Accept L1 Request even if RX FIFO (common to Periodic and Non-Periodic) is not empty. - Accept L1 Request but delay SLEEPM assertion until RX SINK Buffer is empty. Prevent L1 Entry if a Control Transfer is in progress on any Control Endpoint. Ability to Flush TxFIFO even if PHY Clock is gated."]
pub type ENHANCEDLPMSUPT_R = crate::BitReader;
#[doc = "Field `PHYDATAWIDTH` reader - UTMI+ PHY/ULPI-to-Internal UTMI+ Wrapper Data Width (PhyDataWidth)<vr>When a ULPI PHY is used, an internal wrapper converts ULPI to UTMI+. - 2'b00: 8 bits - 2'b01: 16 bits - 2'b10: 8/16 bits, software selectable - Others: Reserved"]
pub type PHYDATAWIDTH_R = crate::FieldReader;
#[doc = "Field `NUMCTLEPS` reader - Number of Device Mode Control Endpoints in Addition to Endpoint 0 (NumCtlEps) Range: 0-15"]
pub type NUMCTLEPS_R = crate::FieldReader;
#[doc = "Field `IDDGFLTR` reader - IDDIG Filter Enable (IddgFltr) - 1'b0: No filter - 1'b1: Filter"]
pub type IDDGFLTR_R = crate::BitReader;
#[doc = "Field `VBUSVALIDFLTR` reader - VBUS Valid Filter Enabled (VBusValidFltr) - 1'b0: No filter - 1'b1: Filter"]
pub type VBUSVALIDFLTR_R = crate::BitReader;
#[doc = "Field `AVALIDFLTR` reader - a_valid Filter Enabled (AValidFltr) - 1'b0: No filter - 1'b1: Filter"]
pub type AVALIDFLTR_R = crate::BitReader;
#[doc = "Field `BVALIDFLTR` reader - b_valid Filter Enabled (BValidFltr) - 1'b0: No filter - 1'b1: Filter"]
pub type BVALIDFLTR_R = crate::BitReader;
#[doc = "Field `SESSENDFLTR` reader - session_end Filter Enabled (SessEndFltr) - 1'b0: No filter - 1'b1: Filter"]
pub type SESSENDFLTR_R = crate::BitReader;
#[doc = "Field `DEDFIFOMODE` reader - Enable Dedicated Transmit FIFO for device IN Endpoints (DedFifoMode) - 1'b0 : Dedicated Transmit FIFO Operation not enabled. - 1'b1 : Dedicated Transmit FIFO Operation enabled."]
pub type DEDFIFOMODE_R = crate::BitReader;
#[doc = "Field `INEPS` reader - Number of Device Mode IN Endpoints Including Control Endpoints (INEps) - 0: 1 IN Endpoint - 1: 2 IN Endpoints .... - 15: 16 IN Endpoints"]
pub type INEPS_R = crate::FieldReader;
#[doc = "Field `DESCDMAENABLED` reader - Scatter/Gather DMA configuration - 1'b0: Non-Scatter/Gather DMA configuration - 1'b1: Scatter/Gather DMA configuration"]
pub type DESCDMAENABLED_R = crate::BitReader;
#[doc = "Field `DESCDMA` reader - Scatter/Gather DMA configuration - 1'b0: Non Dynamic configuration - 1'b1: Dynamic configuration Note: This field is configured using the OTG_EN_DESC_DMA parameter."]
pub type DESCDMA_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of Device Mode Periodic IN Endpoints (NumDevPerioEps) Range: 0-15"]
    #[inline(always)]
    pub fn numdevperioeps(&self) -> NUMDEVPERIOEPS_R {
        NUMDEVPERIOEPS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable Partial Power Down (PartialPwrDn) - 1'b0: Partial Power Down Not Enabled - 1'b1: Partial Power Down Enabled"]
    #[inline(always)]
    pub fn partialpwrdn(&self) -> PARTIALPWRDN_R {
        PARTIALPWRDN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Minimum AHB Frequency Less Than 60 MHz (AhbFreq) - 1'b0: No - 1'b1: Yes"]
    #[inline(always)]
    pub fn ahbfreq(&self) -> AHBFREQ_R {
        AHBFREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Hibernation (Hibernation) - 1'b0: Hibernation feature not enabled - 1'b1: Hibernation feature enabled"]
    #[inline(always)]
    pub fn hibernation(&self) -> HIBERNATION_R {
        HIBERNATION_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Hibernation - 1'b0: Extended Hibernation feature not enabled - 1'b1: Extended Hibernation feature enabled"]
    #[inline(always)]
    pub fn extendedhibernation(&self) -> EXTENDEDHIBERNATION_R {
        EXTENDEDHIBERNATION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Enhanced LPM Support1 (EnhancedLPMSupt1) - This bit indicates that the controller supports L1 entry based on FIFO status. - Accept L1 Request even if Bulk/Interrupt TxFIFO is not empty."]
    #[inline(always)]
    pub fn enhancedlpmsupt1(&self) -> ENHANCEDLPMSUPT1_R {
        ENHANCEDLPMSUPT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Service Interval Flow This bit indicates that the controller supports Service-Interval based scheduling flow for ISOC IN EPs."]
    #[inline(always)]
    pub fn servintflow(&self) -> SERVINTFLOW_R {
        SERVINTFLOW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interpacket Gap ISOC OUT Worst-case Support (ipgisocSupt) This bit indicates that the controller supports the worst-case scenario of Rx followed by Rx Inter Packet Gap (IPG) (32-bit times) as per the UTMI Specification for any token following an ISOC OUT token. Without this support, when any token follows an ISOC OUT token with the worst-case IPG, the controller does not detect the followed token. The worst-case IPG of the controller without this support depends on the AHB and PHY clock frequency.By default IPG Support is enabled."]
    #[inline(always)]
    pub fn ipgisocsupt(&self) -> IPGISOCSUPT_R {
        IPGISOCSUPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Active Clock Gating Support This bit indicates that the controller supports the Dynamic (Switching) Power Reduction during periods when there is no USB and AHB Traffic. - 1'b0: Active Clock Gating is not enabled. - 1'b1: Active Clock Gating Enabled."]
    #[inline(always)]
    pub fn acgsupt(&self) -> ACGSUPT_R {
        ACGSUPT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enhanced LPM Support (EnhancedLPMSupt) This bit indicates that the controller supports the following behavior: L1 Entry Behavior based on FIFO Status - TX FIFO - Accept L1 Request even if ISOC IN TX FIFO is not empty. - Reject L1 Request if Non-Periodic TX FIFO is not empty. - Ensure application can flush the TX FIFO while the Controller is in L1. - RX FIFO - Accept L1 Request even if RX FIFO (common to Periodic and Non-Periodic) is not empty. - Accept L1 Request but delay SLEEPM assertion until RX SINK Buffer is empty. Prevent L1 Entry if a Control Transfer is in progress on any Control Endpoint. Ability to Flush TxFIFO even if PHY Clock is gated."]
    #[inline(always)]
    pub fn enhancedlpmsupt(&self) -> ENHANCEDLPMSUPT_R {
        ENHANCEDLPMSUPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - UTMI+ PHY/ULPI-to-Internal UTMI+ Wrapper Data Width (PhyDataWidth)<vr>When a ULPI PHY is used, an internal wrapper converts ULPI to UTMI+. - 2'b00: 8 bits - 2'b01: 16 bits - 2'b10: 8/16 bits, software selectable - Others: Reserved"]
    #[inline(always)]
    pub fn phydatawidth(&self) -> PHYDATAWIDTH_R {
        PHYDATAWIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Number of Device Mode Control Endpoints in Addition to Endpoint 0 (NumCtlEps) Range: 0-15"]
    #[inline(always)]
    pub fn numctleps(&self) -> NUMCTLEPS_R {
        NUMCTLEPS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - IDDIG Filter Enable (IddgFltr) - 1'b0: No filter - 1'b1: Filter"]
    #[inline(always)]
    pub fn iddgfltr(&self) -> IDDGFLTR_R {
        IDDGFLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS Valid Filter Enabled (VBusValidFltr) - 1'b0: No filter - 1'b1: Filter"]
    #[inline(always)]
    pub fn vbusvalidfltr(&self) -> VBUSVALIDFLTR_R {
        VBUSVALIDFLTR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - a_valid Filter Enabled (AValidFltr) - 1'b0: No filter - 1'b1: Filter"]
    #[inline(always)]
    pub fn avalidfltr(&self) -> AVALIDFLTR_R {
        AVALIDFLTR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - b_valid Filter Enabled (BValidFltr) - 1'b0: No filter - 1'b1: Filter"]
    #[inline(always)]
    pub fn bvalidfltr(&self) -> BVALIDFLTR_R {
        BVALIDFLTR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - session_end Filter Enabled (SessEndFltr) - 1'b0: No filter - 1'b1: Filter"]
    #[inline(always)]
    pub fn sessendfltr(&self) -> SESSENDFLTR_R {
        SESSENDFLTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Dedicated Transmit FIFO for device IN Endpoints (DedFifoMode) - 1'b0 : Dedicated Transmit FIFO Operation not enabled. - 1'b1 : Dedicated Transmit FIFO Operation enabled."]
    #[inline(always)]
    pub fn dedfifomode(&self) -> DEDFIFOMODE_R {
        DEDFIFOMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - Number of Device Mode IN Endpoints Including Control Endpoints (INEps) - 0: 1 IN Endpoint - 1: 2 IN Endpoints .... - 15: 16 IN Endpoints"]
    #[inline(always)]
    pub fn ineps(&self) -> INEPS_R {
        INEPS_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Scatter/Gather DMA configuration - 1'b0: Non-Scatter/Gather DMA configuration - 1'b1: Scatter/Gather DMA configuration"]
    #[inline(always)]
    pub fn descdmaenabled(&self) -> DESCDMAENABLED_R {
        DESCDMAENABLED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Scatter/Gather DMA configuration - 1'b0: Non Dynamic configuration - 1'b1: Dynamic configuration Note: This field is configured using the OTG_EN_DESC_DMA parameter."]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG4")
            .field("numdevperioeps", &self.numdevperioeps())
            .field("partialpwrdn", &self.partialpwrdn())
            .field("ahbfreq", &self.ahbfreq())
            .field("hibernation", &self.hibernation())
            .field("extendedhibernation", &self.extendedhibernation())
            .field("enhancedlpmsupt1", &self.enhancedlpmsupt1())
            .field("servintflow", &self.servintflow())
            .field("ipgisocsupt", &self.ipgisocsupt())
            .field("acgsupt", &self.acgsupt())
            .field("enhancedlpmsupt", &self.enhancedlpmsupt())
            .field("phydatawidth", &self.phydatawidth())
            .field("numctleps", &self.numctleps())
            .field("iddgfltr", &self.iddgfltr())
            .field("vbusvalidfltr", &self.vbusvalidfltr())
            .field("avalidfltr", &self.avalidfltr())
            .field("bvalidfltr", &self.bvalidfltr())
            .field("sessendfltr", &self.sessendfltr())
            .field("dedfifomode", &self.dedfifomode())
            .field("ineps", &self.ineps())
            .field("descdmaenabled", &self.descdmaenabled())
            .field("descdma", &self.descdma())
            .finish()
    }
}
#[doc = "This register contains configuration options selected using coreConsultant. Note: Bit \\[31\\] is available only when Scatter/Gather DMA mode is enabled. When Scatter/Gather DMA mode is disabled, this field is reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHWCFG4_SPEC;
impl crate::RegisterSpec for GHWCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg4::R`](R) reader structure"]
impl crate::Readable for GHWCFG4_SPEC {}
#[doc = "`reset()` method sets GHWCFG4 to value 0xde11_aa30"]
impl crate::Resettable for GHWCFG4_SPEC {
    const RESET_VALUE: u32 = 0xde11_aa30;
}
