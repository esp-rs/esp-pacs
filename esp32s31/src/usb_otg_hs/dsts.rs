#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SUSPSTS` reader - Suspend Status (SuspSts) In Device mode, this bit is set as long as a Suspend condition is detected on the USB. The core enters the Suspend state when there is no activity on the phy_line_state_i signal for an extended period of time. The core comes out of the suspend under the following conditions : - If there is any activity on the phy_line_state_i signal, or - If the application writes to the Remote Wakeup Signaling bit in the Device Control register (DCTL.RmtWkUpSig). When the core comes out of the suspend, this bit is set to 1'b0."]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated Speed (EnumSpd) Indicates the speed at which the controller has come up after speed detection through a connect or reset sequence. - 2'b00: High speed (PHY clock is running at 30 or 60 MHz) - 2'b01: Full speed (PHY clock is running at 30 or 60 MHz) - 2'b10: Low speed (PHY clock is running at 6 MHz) - 2'b11: Full speed (PHY clock is running at 48 MHz) Low speed is not supported for devices using a UTMI+ PHY."]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `ERRTICERR` reader - Erratic Error (ErrticErr) The core sets this bit to report any erratic errors (phy_rxvalid_i/phy_rxvldh_i or phy_rxactive_i is asserted for at least 2 ms, due to PHY error) seen on the UTMI+. Due to erratic errors, the core goes into Suspended state and an interrupt is generated to the application with Early Suspend bit of the Core Interrupt register (GINTSTS.ErlySusp). If the early suspend is asserted due to an erratic error, the application must perform a PHY reset followed by a soft reset to controller."]
pub type ERRTICERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame or Microframe Number of the Received SOF (SOFFN) When the core is operating at high speed, this field contains a microframe number. When the core is operating at full or low speed, this field contains a Frame number. Note: This register may return a non-zero value if read immediately after power-on reset. In case the register bit reads non-zero immediately after power-on reset, it does not indicate that SOF has been received from the host. The read value of this interrupt is valid only after a valid connection between host and device is established."]
pub type SOFFN_R = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - Device Line Status (DevLnSts) Indicates the current logic level USB data lines - DevLnSts\\[1\\]: Logic level of D+ - DevLnSts\\[0\\]: Logic level of D-"]
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Suspend Status (SuspSts) In Device mode, this bit is set as long as a Suspend condition is detected on the USB. The core enters the Suspend state when there is no activity on the phy_line_state_i signal for an extended period of time. The core comes out of the suspend under the following conditions : - If there is any activity on the phy_line_state_i signal, or - If the application writes to the Remote Wakeup Signaling bit in the Device Control register (DCTL.RmtWkUpSig). When the core comes out of the suspend, this bit is set to 1'b0."]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed (EnumSpd) Indicates the speed at which the controller has come up after speed detection through a connect or reset sequence. - 2'b00: High speed (PHY clock is running at 30 or 60 MHz) - 2'b01: Full speed (PHY clock is running at 30 or 60 MHz) - 2'b10: Low speed (PHY clock is running at 6 MHz) - 2'b11: Full speed (PHY clock is running at 48 MHz) Low speed is not supported for devices using a UTMI+ PHY."]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic Error (ErrticErr) The core sets this bit to report any erratic errors (phy_rxvalid_i/phy_rxvldh_i or phy_rxactive_i is asserted for at least 2 ms, due to PHY error) seen on the UTMI+. Due to erratic errors, the core goes into Suspended state and an interrupt is generated to the application with Early Suspend bit of the Core Interrupt register (GINTSTS.ErlySusp). If the early suspend is asserted due to an erratic error, the application must perform a PHY reset followed by a soft reset to controller."]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame or Microframe Number of the Received SOF (SOFFN) When the core is operating at high speed, this field contains a microframe number. When the core is operating at full or low speed, this field contains a Frame number. Note: This register may return a non-zero value if read immediately after power-on reset. In case the register bit reads non-zero immediately after power-on reset, it does not indicate that SOF has been received from the host. The read value of this interrupt is valid only after a valid connection between host and device is established."]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - Device Line Status (DevLnSts) Indicates the current logic level USB data lines - DevLnSts\\[1\\]: Logic level of D+ - DevLnSts\\[0\\]: Logic level of D-"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("errticerr", &self.errticerr())
            .field("soffn", &self.soffn())
            .field("devlnsts", &self.devlnsts())
            .finish()
    }
}
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from Device All Interrupts (DAINT) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
