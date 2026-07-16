#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HPRT_SPEC>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HPRT_SPEC>;
#[doc = "Field `PRTCONNSTS` reader - Port Connect Status (PrtConnSts) - 0: No device is attached to the port. - 1: A device is attached to the port."]
pub type PRTCONNSTS_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` reader - Port Connect Detected (PrtConnDet) The core sets this bit when a device connection is detected to trigger an interrupt to the application using the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt).This bit can be set only by the core and the application must write 1 to clear it.The application must write a 1 to this bit to clear the interrupt."]
pub type PRTCONNDET_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` writer - Port Connect Detected (PrtConnDet) The core sets this bit when a device connection is detected to trigger an interrupt to the application using the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt).This bit can be set only by the core and the application must write 1 to clear it.The application must write a 1 to this bit to clear the interrupt."]
pub type PRTCONNDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - Port Enable (PrtEna) A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot Set this bit by a register write. It can only clear it to disable the port by writing 1. This bit does not trigger any interrupt to the application. - 1'b0: Port disabled - 1'b1: Port enabled"]
pub type PRTENA_R = crate::BitReader;
#[doc = "Field `PRTENA` writer - Port Enable (PrtEna) A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot Set this bit by a register write. It can only clear it to disable the port by writing 1. This bit does not trigger any interrupt to the application. - 1'b0: Port disabled - 1'b1: Port enabled"]
pub type PRTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - Port Enable/Disable Change (PrtEnChng) The core sets this bit when the status of the Port Enable bit \\[2\\] of this register changes.This bit can be set only by the core and the application must write 1 to clear it."]
pub type PRTENCHNG_R = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - Port Enable/Disable Change (PrtEnChng) The core sets this bit when the status of the Port Enable bit \\[2\\] of this register changes.This bit can be set only by the core and the application must write 1 to clear it."]
pub type PRTENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCURRACT` reader - Port Overcurrent Active (PrtOvrCurrAct) Indicates the overcurrent condition of the port. - 1'b0: No overcurrent condition - 1'b1: Overcurrent condition"]
pub type PRTOVRCURRACT_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` reader - Port Overcurrent Change (PrtOvrCurrChng) The core sets this bit when the status of the Port Overcurrent Active bit (bit 4) in this register changes.This bit can be set only by the core and the application must write 1 to clear it"]
pub type PRTOVRCURRCHNG_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` writer - Port Overcurrent Change (PrtOvrCurrChng) The core sets this bit when the status of the Port Overcurrent Active bit (bit 4) in this register changes.This bit can be set only by the core and the application must write 1 to clear it"]
pub type PRTOVRCURRCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - Port Resume (PrtRes) The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the Port Resume/Remote Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.WkUpInt), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. - 1'b0: No resume driven - 1'b1: Resume driven When LPM is enabled, In L1 state the behavior of this bit is as follows: The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until a pre-determined time specified in GLPMCFG.HIRD_Thres\\[3:0\\] field. If the core detects a USB remote wakeup sequence, as indicated by the Port L1Resume/Remote L1Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.L1WkUpInt), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set by both core or application and also cleared by core or application. This bit is cleared by the core even if there is no device connected to the Host."]
pub type PRTRES_R = crate::BitReader;
#[doc = "Field `PRTRES` writer - Port Resume (PrtRes) The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the Port Resume/Remote Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.WkUpInt), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. - 1'b0: No resume driven - 1'b1: Resume driven When LPM is enabled, In L1 state the behavior of this bit is as follows: The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until a pre-determined time specified in GLPMCFG.HIRD_Thres\\[3:0\\] field. If the core detects a USB remote wakeup sequence, as indicated by the Port L1Resume/Remote L1Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.L1WkUpInt), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set by both core or application and also cleared by core or application. This bit is cleared by the core even if there is no device connected to the Host."]
pub type PRTRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - Port Suspend (PrtSusp) The application sets this bit to put this port in Suspend mode. The core only stops sending SOFs when this is Set. To stop the PHY clock, the application must Set the Port Clock Stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the Port Reset bit or Port Resume bit in this register or the Resume/Remote Wakeup Detected Interrupt bit or Disconnect Detected Interrupt bit in the Core Interrupt register (GINTSTS.WkUpInt or GINTSTS.DisconnInt, respectively).This bit is cleared by the core even if there is no device connected to the Host. - 1'b0: Port not in Suspend mode - 1'b1: Port in Suspend mode"]
pub type PRTSUSP_R = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - Port Suspend (PrtSusp) The application sets this bit to put this port in Suspend mode. The core only stops sending SOFs when this is Set. To stop the PHY clock, the application must Set the Port Clock Stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the Port Reset bit or Port Resume bit in this register or the Resume/Remote Wakeup Detected Interrupt bit or Disconnect Detected Interrupt bit in the Core Interrupt register (GINTSTS.WkUpInt or GINTSTS.DisconnInt, respectively).This bit is cleared by the core even if there is no device connected to the Host. - 1'b0: Port not in Suspend mode - 1'b1: Port in Suspend mode"]
pub type PRTSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - Port Reset (PrtRst) When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. - 1'b0: Port not in reset - 1'b1: Port in reset The application must leave this bit set for at least a minimum duration mentioned below to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit Set by the USB standard.This bit is cleared by the core even if there is no device connected to the Host. - High speed: 50 ms - Full speed/Low speed: 10 ms"]
pub type PRTRST_R = crate::BitReader;
#[doc = "Field `PRTRST` writer - Port Reset (PrtRst) When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. - 1'b0: Port not in reset - 1'b1: Port in reset The application must leave this bit set for at least a minimum duration mentioned below to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit Set by the USB standard.This bit is cleared by the core even if there is no device connected to the Host. - High speed: 50 ms - Full speed/Low speed: 10 ms"]
pub type PRTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - Port Line Status (PrtLnSts) Indicates the current logic level USB data lines - Bit \\[10\\]: Logic level of D+ - Bit \\[11\\]: Logic level of D-"]
pub type PRTLNSTS_R = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - Port Power (PrtPwr) The application uses this field to control power to this port (write 1'b1 to set to 1'b1 and write 1'b0 to set to 1'b0), and the core can clear this bit on an over current condition. - 1'b0: Power off - 1'b1: Power on Note: This bit is interface independent. The application needs to program this bit for all interfaces as described in the host programming flow in the Programming Guide."]
pub type PRTPWR_R = crate::BitReader;
#[doc = "Field `PRTPWR` writer - Port Power (PrtPwr) The application uses this field to control power to this port (write 1'b1 to set to 1'b1 and write 1'b0 to set to 1'b0), and the core can clear this bit on an over current condition. - 1'b0: Power off - 1'b1: Power on Note: This bit is interface independent. The application needs to program this bit for all interfaces as described in the host programming flow in the Programming Guide."]
pub type PRTPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTTSTCTL` reader - Port Test Control (PrtTstCtl) The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. - 4'b0000: Test mode disabled - 4'b0001: Test_J mode - 4'b0010: Test_K mode - 4'b0011: Test_SE0_NAK mode - 4'b0100: Test_Packet mode - 4'b0101: Test_Force_Enable - Others: Reserved To move the DWC_otg controller to test mode, you must set this field. Complete the following steps to move the DWC_otg core to test mode: - 1. Power on the core. - 2. Load the DWC_otg driver. - 3. Connect an HS device and enumerate to HS mode. - 4. Access the HPRT register to send test packets. - 5. Remove the device and connect to fixture (OPT) port. The DWC_otg host core continues sending out test packets. - 6. Test the eye diagram."]
pub type PRTTSTCTL_R = crate::FieldReader;
#[doc = "Field `PRTTSTCTL` writer - Port Test Control (PrtTstCtl) The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. - 4'b0000: Test mode disabled - 4'b0001: Test_J mode - 4'b0010: Test_K mode - 4'b0011: Test_SE0_NAK mode - 4'b0100: Test_Packet mode - 4'b0101: Test_Force_Enable - Others: Reserved To move the DWC_otg controller to test mode, you must set this field. Complete the following steps to move the DWC_otg core to test mode: - 1. Power on the core. - 2. Load the DWC_otg driver. - 3. Connect an HS device and enumerate to HS mode. - 4. Access the HPRT register to send test packets. - 5. Remove the device and connect to fixture (OPT) port. The DWC_otg host core continues sending out test packets. - 6. Test the eye diagram."]
pub type PRTTSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTSPD` reader - Port Speed (PrtSpd) Indicates the speed of the device attached to this port. - 2'b00: High speed - 2'b01: Full speed - 2'b10: Low speed - 2'b11: Reserved"]
pub type PRTSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port Connect Status (PrtConnSts) - 0: No device is attached to the port. - 1: A device is attached to the port."]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected (PrtConnDet) The core sets this bit when a device connection is detected to trigger an interrupt to the application using the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt).This bit can be set only by the core and the application must write 1 to clear it.The application must write a 1 to this bit to clear the interrupt."]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable (PrtEna) A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot Set this bit by a register write. It can only clear it to disable the port by writing 1. This bit does not trigger any interrupt to the application. - 1'b0: Port disabled - 1'b1: Port enabled"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (PrtEnChng) The core sets this bit when the status of the Port Enable bit \\[2\\] of this register changes.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active (PrtOvrCurrAct) Indicates the overcurrent condition of the port. - 1'b0: No overcurrent condition - 1'b1: Overcurrent condition"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change (PrtOvrCurrChng) The core sets this bit when the status of the Port Overcurrent Active bit (bit 4) in this register changes.This bit can be set only by the core and the application must write 1 to clear it"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume (PrtRes) The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the Port Resume/Remote Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.WkUpInt), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. - 1'b0: No resume driven - 1'b1: Resume driven When LPM is enabled, In L1 state the behavior of this bit is as follows: The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until a pre-determined time specified in GLPMCFG.HIRD_Thres\\[3:0\\] field. If the core detects a USB remote wakeup sequence, as indicated by the Port L1Resume/Remote L1Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.L1WkUpInt), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set by both core or application and also cleared by core or application. This bit is cleared by the core even if there is no device connected to the Host."]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend (PrtSusp) The application sets this bit to put this port in Suspend mode. The core only stops sending SOFs when this is Set. To stop the PHY clock, the application must Set the Port Clock Stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the Port Reset bit or Port Resume bit in this register or the Resume/Remote Wakeup Detected Interrupt bit or Disconnect Detected Interrupt bit in the Core Interrupt register (GINTSTS.WkUpInt or GINTSTS.DisconnInt, respectively).This bit is cleared by the core even if there is no device connected to the Host. - 1'b0: Port not in Suspend mode - 1'b1: Port in Suspend mode"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset (PrtRst) When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. - 1'b0: Port not in reset - 1'b1: Port in reset The application must leave this bit set for at least a minimum duration mentioned below to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit Set by the USB standard.This bit is cleared by the core even if there is no device connected to the Host. - High speed: 50 ms - Full speed/Low speed: 10 ms"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status (PrtLnSts) Indicates the current logic level USB data lines - Bit \\[10\\]: Logic level of D+ - Bit \\[11\\]: Logic level of D-"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power (PrtPwr) The application uses this field to control power to this port (write 1'b1 to set to 1'b1 and write 1'b0 to set to 1'b0), and the core can clear this bit on an over current condition. - 1'b0: Power off - 1'b1: Power on Note: This bit is interface independent. The application needs to program this bit for all interfaces as described in the host programming flow in the Programming Guide."]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control (PrtTstCtl) The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. - 4'b0000: Test mode disabled - 4'b0001: Test_J mode - 4'b0010: Test_K mode - 4'b0011: Test_SE0_NAK mode - 4'b0100: Test_Packet mode - 4'b0101: Test_Force_Enable - Others: Reserved To move the DWC_otg controller to test mode, you must set this field. Complete the following steps to move the DWC_otg core to test mode: - 1. Power on the core. - 2. Load the DWC_otg driver. - 3. Connect an HS device and enumerate to HS mode. - 4. Access the HPRT register to send test packets. - 5. Remove the device and connect to fixture (OPT) port. The DWC_otg host core continues sending out test packets. - 6. Test the eye diagram."]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed (PrtSpd) Indicates the speed of the device attached to this port. - 2'b00: High speed - 2'b01: Full speed - 2'b10: Low speed - 2'b11: Reserved"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("prtconnsts", &self.prtconnsts())
            .field("prtconndet", &self.prtconndet())
            .field("prtena", &self.prtena())
            .field("prtenchng", &self.prtenchng())
            .field("prtovrcurract", &self.prtovrcurract())
            .field("prtovrcurrchng", &self.prtovrcurrchng())
            .field("prtres", &self.prtres())
            .field("prtsusp", &self.prtsusp())
            .field("prtrst", &self.prtrst())
            .field("prtlnsts", &self.prtlnsts())
            .field("prtpwr", &self.prtpwr())
            .field("prttstctl", &self.prttstctl())
            .field("prtspd", &self.prtspd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected (PrtConnDet) The core sets this bit when a device connection is detected to trigger an interrupt to the application using the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt).This bit can be set only by the core and the application must write 1 to clear it.The application must write a 1 to this bit to clear the interrupt."]
    #[inline(always)]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W<'_, HPRT_SPEC> {
        PRTCONNDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable (PrtEna) A port is enabled only by the core after a reset sequence, and is disabled by an overcurrent condition, a disconnect condition, or by the application clearing this bit. The application cannot Set this bit by a register write. It can only clear it to disable the port by writing 1. This bit does not trigger any interrupt to the application. - 1'b0: Port disabled - 1'b1: Port enabled"]
    #[inline(always)]
    pub fn prtena(&mut self) -> PRTENA_W<'_, HPRT_SPEC> {
        PRTENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (PrtEnChng) The core sets this bit when the status of the Port Enable bit \\[2\\] of this register changes.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W<'_, HPRT_SPEC> {
        PRTENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - Port Overcurrent Change (PrtOvrCurrChng) The core sets this bit when the status of the Port Overcurrent Active bit (bit 4) in this register changes.This bit can be set only by the core and the application must write 1 to clear it"]
    #[inline(always)]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W<'_, HPRT_SPEC> {
        PRTOVRCURRCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Resume (PrtRes) The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until the application clears this bit. If the core detects a USB remote wakeup sequence, as indicated by the Port Resume/Remote Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.WkUpInt), the core starts driving resume signaling without application intervention and clears this bit when it detects a disconnect condition. The read value of this bit indicates whether the core is currently driving resume signaling. - 1'b0: No resume driven - 1'b1: Resume driven When LPM is enabled, In L1 state the behavior of this bit is as follows: The application sets this bit to drive resume signaling on the port. The core continues to drive the resume signal until a pre-determined time specified in GLPMCFG.HIRD_Thres\\[3:0\\] field. If the core detects a USB remote wakeup sequence, as indicated by the Port L1Resume/Remote L1Wakeup Detected Interrupt bit of the Core Interrupt register (GINTSTS.L1WkUpInt), the core starts driving resume signaling without application intervention and clears this bit at the end of resume.This bit can be set by both core or application and also cleared by core or application. This bit is cleared by the core even if there is no device connected to the Host."]
    #[inline(always)]
    pub fn prtres(&mut self) -> PRTRES_W<'_, HPRT_SPEC> {
        PRTRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend (PrtSusp) The application sets this bit to put this port in Suspend mode. The core only stops sending SOFs when this is Set. To stop the PHY clock, the application must Set the Port Clock Stop bit, which asserts the suspend input pin of the PHY. The read value of this bit reflects the current suspend status of the port. This bit is cleared by the core after a remote wakeup signal is detected or the application sets the Port Reset bit or Port Resume bit in this register or the Resume/Remote Wakeup Detected Interrupt bit or Disconnect Detected Interrupt bit in the Core Interrupt register (GINTSTS.WkUpInt or GINTSTS.DisconnInt, respectively).This bit is cleared by the core even if there is no device connected to the Host. - 1'b0: Port not in Suspend mode - 1'b1: Port in Suspend mode"]
    #[inline(always)]
    pub fn prtsusp(&mut self) -> PRTSUSP_W<'_, HPRT_SPEC> {
        PRTSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset (PrtRst) When the application sets this bit, a reset sequence is started on this port. The application must time the reset period and clear this bit after the reset sequence is complete. - 1'b0: Port not in reset - 1'b1: Port in reset The application must leave this bit set for at least a minimum duration mentioned below to start a reset on the port. The application can leave it set for another 10 ms in addition to the required minimum duration, before clearing the bit, even though there is no maximum limit Set by the USB standard.This bit is cleared by the core even if there is no device connected to the Host. - High speed: 50 ms - Full speed/Low speed: 10 ms"]
    #[inline(always)]
    pub fn prtrst(&mut self) -> PRTRST_W<'_, HPRT_SPEC> {
        PRTRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power (PrtPwr) The application uses this field to control power to this port (write 1'b1 to set to 1'b1 and write 1'b0 to set to 1'b0), and the core can clear this bit on an over current condition. - 1'b0: Power off - 1'b1: Power on Note: This bit is interface independent. The application needs to program this bit for all interfaces as described in the host programming flow in the Programming Guide."]
    #[inline(always)]
    pub fn prtpwr(&mut self) -> PRTPWR_W<'_, HPRT_SPEC> {
        PRTPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port Test Control (PrtTstCtl) The application writes a nonzero value to this field to put the port into a Test mode, and the corresponding pattern is signaled on the port. - 4'b0000: Test mode disabled - 4'b0001: Test_J mode - 4'b0010: Test_K mode - 4'b0011: Test_SE0_NAK mode - 4'b0100: Test_Packet mode - 4'b0101: Test_Force_Enable - Others: Reserved To move the DWC_otg controller to test mode, you must set this field. Complete the following steps to move the DWC_otg core to test mode: - 1. Power on the core. - 2. Load the DWC_otg driver. - 3. Connect an HS device and enumerate to HS mode. - 4. Access the HPRT register to send test packets. - 5. Remove the device and connect to fixture (OPT) port. The DWC_otg host core continues sending out test packets. - 6. Test the eye diagram."]
    #[inline(always)]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W<'_, HPRT_SPEC> {
        PRTTSTCTL_W::new(self, 13)
    }
}
#[doc = "This register is available only in Host mode. Currently, the OTG Host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in the Interrupt Hierarchy figure in the databook. The R_SS_WC bits in this register can trigger an interrupt to the application through the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt). On a Port Interrupt, the application must read this register and clear the bit that caused the interrupt. For the R_SS_WC bits, the application must write a 1 to the bit to clear the interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HPRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {}
