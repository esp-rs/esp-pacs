#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINT_SPEC>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINT_SPEC>;
#[doc = "Field `SESENDDET` reader - Mode: Host and Device Session End Detected (SesEndDet) The controller sets this bit when the utmiotg_bvalid signal is deasserted. This bit can be set only by the core and the application must write 1 to clear it."]
pub type SESENDDET_R = crate::BitReader;
#[doc = "Field `SESENDDET` writer - Mode: Host and Device Session End Detected (SesEndDet) The controller sets this bit when the utmiotg_bvalid signal is deasserted. This bit can be set only by the core and the application must write 1 to clear it."]
pub type SESENDDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESREQSUCSTSCHNG` reader - Mode: Host and Device Session Request Success Status Change (SesReqSucStsChng) The core sets this bit on the success or failure of a session request. The application must read the Session Request Success bit in the OTG Control and Status register (GOTGCTL.SesReqScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
pub type SESREQSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `SESREQSUCSTSCHNG` writer - Mode: Host and Device Session Request Success Status Change (SesReqSucStsChng) The core sets this bit on the success or failure of a session request. The application must read the Session Request Success bit in the OTG Control and Status register (GOTGCTL.SesReqScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
pub type SESREQSUCSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGSUCSTSCHNG` reader - Mode: Host and Device Host Negotiation Success Status Change (HstNegSucStsChng) The core sets this bit on the success or failure of a USB host negotiation request. The application must read the Host Negotiation Success bit of the OTG Control and Status register (GOTGCTL.HstNegScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
pub type HSTNEGSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `HSTNEGSUCSTSCHNG` writer - Mode: Host and Device Host Negotiation Success Status Change (HstNegSucStsChng) The core sets this bit on the success or failure of a USB host negotiation request. The application must read the Host Negotiation Success bit of the OTG Control and Status register (GOTGCTL.HstNegScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
pub type HSTNEGSUCSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGDET` reader - Mode:Host and Device Host Negotiation Detected (HstNegDet) The core sets this bit when it detects a host negotiation request on the USB. This bit can be set only by the core and the application must write 1 to clear it."]
pub type HSTNEGDET_R = crate::BitReader;
#[doc = "Field `HSTNEGDET` writer - Mode:Host and Device Host Negotiation Detected (HstNegDet) The core sets this bit when it detects a host negotiation request on the USB. This bit can be set only by the core and the application must write 1 to clear it."]
pub type HSTNEGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEVTOUTCHG` reader - Mode: Host and Device A-Device Timeout Change (ADevTOUTChg) The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect.This bit can be set only by the core and the application must write 1 to clear it."]
pub type ADEVTOUTCHG_R = crate::BitReader;
#[doc = "Field `ADEVTOUTCHG` writer - Mode: Host and Device A-Device Timeout Change (ADevTOUTChg) The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect.This bit can be set only by the core and the application must write 1 to clear it."]
pub type ADEVTOUTCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEDONE` reader - Mode: Host only Debounce Done (DbnceDone) The core sets this bit when the debounce is completed after the device connect. The application can start driving USB reset after seeing this interrupt. This bit is only valid when the HNP Capable or SRP Capable bit is SET in the Core USB Configuration register (GUSBCFG.HNPCap or GUSBCFG.SRPCap, respectively). This bit can be set only by the core and the application must write 1 to clear it."]
pub type DBNCEDONE_R = crate::BitReader;
#[doc = "Field `DBNCEDONE` writer - Mode: Host only Debounce Done (DbnceDone) The core sets this bit when the debounce is completed after the device connect. The application can start driving USB reset after seeing this interrupt. This bit is only valid when the HNP Capable or SRP Capable bit is SET in the Core USB Configuration register (GUSBCFG.HNPCap or GUSBCFG.SRPCap, respectively). This bit can be set only by the core and the application must write 1 to clear it."]
pub type DBNCEDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTVALIPCHNG` reader - This bit when set indicates that there is a change in the value of at least one ACA pin value. This bit is present only if OTG_BC_SUPPORT=1, otherwise it is reserved."]
pub type MULTVALIPCHNG_R = crate::BitReader;
#[doc = "Field `MULTVALIPCHNG` writer - This bit when set indicates that there is a change in the value of at least one ACA pin value. This bit is present only if OTG_BC_SUPPORT=1, otherwise it is reserved."]
pub type MULTVALIPCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Mode: Host and Device Session End Detected (SesEndDet) The controller sets this bit when the utmiotg_bvalid signal is deasserted. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mode: Host and Device Session Request Success Status Change (SesReqSucStsChng) The core sets this bit on the success or failure of a session request. The application must read the Session Request Success bit in the OTG Control and Status register (GOTGCTL.SesReqScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode: Host and Device Host Negotiation Success Status Change (HstNegSucStsChng) The core sets this bit on the success or failure of a USB host negotiation request. The application must read the Host Negotiation Success bit of the OTG Control and Status register (GOTGCTL.HstNegScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Mode:Host and Device Host Negotiation Detected (HstNegDet) The core sets this bit when it detects a host negotiation request on the USB. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mode: Host and Device A-Device Timeout Change (ADevTOUTChg) The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mode: Host only Debounce Done (DbnceDone) The core sets this bit when the debounce is completed after the device connect. The application can start driving USB reset after seeing this interrupt. This bit is only valid when the HNP Capable or SRP Capable bit is SET in the Core USB Configuration register (GUSBCFG.HNPCap or GUSBCFG.SRPCap, respectively). This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn dbncedone(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit when set indicates that there is a change in the value of at least one ACA pin value. This bit is present only if OTG_BC_SUPPORT=1, otherwise it is reserved."]
    #[inline(always)]
    pub fn multvalipchng(&self) -> MULTVALIPCHNG_R {
        MULTVALIPCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sesenddet", &self.sesenddet())
            .field("sesreqsucstschng", &self.sesreqsucstschng())
            .field("hstnegsucstschng", &self.hstnegsucstschng())
            .field("hstnegdet", &self.hstnegdet())
            .field("adevtoutchg", &self.adevtoutchg())
            .field("dbncedone", &self.dbncedone())
            .field("multvalipchng", &self.multvalipchng())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Mode: Host and Device Session End Detected (SesEndDet) The controller sets this bit when the utmiotg_bvalid signal is deasserted. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn sesenddet(&mut self) -> SESENDDET_W<'_, GOTGINT_SPEC> {
        SESENDDET_W::new(self, 2)
    }
    #[doc = "Bit 8 - Mode: Host and Device Session Request Success Status Change (SesReqSucStsChng) The core sets this bit on the success or failure of a session request. The application must read the Session Request Success bit in the OTG Control and Status register (GOTGCTL.SesReqScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn sesreqsucstschng(&mut self) -> SESREQSUCSTSCHNG_W<'_, GOTGINT_SPEC> {
        SESREQSUCSTSCHNG_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mode: Host and Device Host Negotiation Success Status Change (HstNegSucStsChng) The core sets this bit on the success or failure of a USB host negotiation request. The application must read the Host Negotiation Success bit of the OTG Control and Status register (GOTGCTL.HstNegScs) to check for success or failure. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn hstnegsucstschng(&mut self) -> HSTNEGSUCSTSCHNG_W<'_, GOTGINT_SPEC> {
        HSTNEGSUCSTSCHNG_W::new(self, 9)
    }
    #[doc = "Bit 17 - Mode:Host and Device Host Negotiation Detected (HstNegDet) The core sets this bit when it detects a host negotiation request on the USB. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn hstnegdet(&mut self) -> HSTNEGDET_W<'_, GOTGINT_SPEC> {
        HSTNEGDET_W::new(self, 17)
    }
    #[doc = "Bit 18 - Mode: Host and Device A-Device Timeout Change (ADevTOUTChg) The core sets this bit to indicate that the A-device has timed out while waiting for the B-device to connect.This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn adevtoutchg(&mut self) -> ADEVTOUTCHG_W<'_, GOTGINT_SPEC> {
        ADEVTOUTCHG_W::new(self, 18)
    }
    #[doc = "Bit 19 - Mode: Host only Debounce Done (DbnceDone) The core sets this bit when the debounce is completed after the device connect. The application can start driving USB reset after seeing this interrupt. This bit is only valid when the HNP Capable or SRP Capable bit is SET in the Core USB Configuration register (GUSBCFG.HNPCap or GUSBCFG.SRPCap, respectively). This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn dbncedone(&mut self) -> DBNCEDONE_W<'_, GOTGINT_SPEC> {
        DBNCEDONE_W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit when set indicates that there is a change in the value of at least one ACA pin value. This bit is present only if OTG_BC_SUPPORT=1, otherwise it is reserved."]
    #[inline(always)]
    pub fn multvalipchng(&mut self) -> MULTVALIPCHNG_W<'_, GOTGINT_SPEC> {
        MULTVALIPCHNG_W::new(self, 20)
    }
}
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {}
