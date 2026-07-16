#[doc = "Register `DOEPEACHMSK15` reader"]
pub type R = crate::R<DOEPEACHMSK15_SPEC>;
#[doc = "Register `DOEPEACHMSK15` writer"]
pub type W = crate::W<DOEPEACHMSK15_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
pub type EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
pub type EPDISBLDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error (AHBErrMsk)"]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error (AHBErrMsk)"]
pub type AHBERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPMSK` reader - SETUP Phase Done Mask (SetUPMsk) Applies to control endpoints only."]
pub type SETUPMSK_R = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - SETUP Phase Done Mask (SetUPMsk) Applies to control endpoints only."]
pub type SETUPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDISMSK` reader - OUT Token Received when Endpoint Disabled Mask (OUTTknEPdisMsk) Applies to control OUT endpoints only."]
pub type OUTTKNEPDISMSK_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDISMSK` writer - OUT Token Received when Endpoint Disabled Mask (OUTTknEPdisMsk) Applies to control OUT endpoints only."]
pub type OUTTKNEPDISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSRCVDMSK` reader - Status Phase Received Mask (StsPhsRcvdMsk) Applies to control OUT endpoints only."]
pub type STSPHSRCVDMSK_R = crate::BitReader;
#[doc = "Field `STSPHSRCVDMSK` writer - Status Phase Received Mask (StsPhsRcvdMsk) Applies to control OUT endpoints only."]
pub type STSPHSRCVDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets Received Mask (Back2BackSETup) Applies to control OUT endpoints only."]
pub type BACK2BACKSETUP_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets Received Mask (Back2BackSETup) Applies to control OUT endpoints only."]
pub type BACK2BACKSETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERRMSK` reader - OUT Packet Error Mask (OutPktErrMsk)"]
pub type OUTPKTERRMSK_R = crate::BitReader;
#[doc = "Field `OUTPKTERRMSK` writer - OUT Packet Error Mask (OutPktErrMsk)"]
pub type OUTPKTERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAOUTINTRMSK` reader - BNA interrupt Mask (BnaOutIntrMsk)"]
pub type BNAOUTINTRMSK_R = crate::BitReader;
#[doc = "Field `BNAOUTINTRMSK` writer - BNA interrupt Mask (BnaOutIntrMsk)"]
pub type BNAOUTINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERRMSK` reader - Babble Error interrupt Mask (BbleErrMsk)"]
pub type BBLEERRMSK_R = crate::BitReader;
#[doc = "Field `BBLEERRMSK` writer - Babble Error interrupt Mask (BbleErrMsk)"]
pub type BBLEERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask (NAKMsk)"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask (NAKMsk)"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - NYET interrupt Mask (NYETMsk)"]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - NYET interrupt Mask (NYETMsk)"]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error (AHBErrMsk)"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask (SetUPMsk) Applies to control endpoints only."]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask (OUTTknEPdisMsk) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn outtknepdismsk(&self) -> OUTTKNEPDISMSK_R {
        OUTTKNEPDISMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received Mask (StsPhsRcvdMsk) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn stsphsrcvdmsk(&self) -> STSPHSRCVDMSK_R {
        STSPHSRCVDMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask (Back2BackSETup) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask (OutPktErrMsk)"]
    #[inline(always)]
    pub fn outpkterrmsk(&self) -> OUTPKTERRMSK_R {
        OUTPKTERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt Mask (BnaOutIntrMsk)"]
    #[inline(always)]
    pub fn bnaoutintrmsk(&self) -> BNAOUTINTRMSK_R {
        BNAOUTINTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error interrupt Mask (BbleErrMsk)"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask (NAKMsk)"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt Mask (NYETMsk)"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPEACHMSK15")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("epdisbldmsk", &self.epdisbldmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("setupmsk", &self.setupmsk())
            .field("outtknepdismsk", &self.outtknepdismsk())
            .field("stsphsrcvdmsk", &self.stsphsrcvdmsk())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterrmsk", &self.outpkterrmsk())
            .field("bnaoutintrmsk", &self.bnaoutintrmsk())
            .field("bbleerrmsk", &self.bbleerrmsk())
            .field("nakmsk", &self.nakmsk())
            .field("nyetmsk", &self.nyetmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<'_, DOEPEACHMSK15_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
    #[inline(always)]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<'_, DOEPEACHMSK15_SPEC> {
        EPDISBLDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error (AHBErrMsk)"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<'_, DOEPEACHMSK15_SPEC> {
        AHBERRMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask (SetUPMsk) Applies to control endpoints only."]
    #[inline(always)]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<'_, DOEPEACHMSK15_SPEC> {
        SETUPMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask (OUTTknEPdisMsk) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn outtknepdismsk(&mut self) -> OUTTKNEPDISMSK_W<'_, DOEPEACHMSK15_SPEC> {
        OUTTKNEPDISMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Status Phase Received Mask (StsPhsRcvdMsk) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn stsphsrcvdmsk(&mut self) -> STSPHSRCVDMSK_W<'_, DOEPEACHMSK15_SPEC> {
        STSPHSRCVDMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask (Back2BackSETup) Applies to control OUT endpoints only."]
    #[inline(always)]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<'_, DOEPEACHMSK15_SPEC> {
        BACK2BACKSETUP_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask (OutPktErrMsk)"]
    #[inline(always)]
    pub fn outpkterrmsk(&mut self) -> OUTPKTERRMSK_W<'_, DOEPEACHMSK15_SPEC> {
        OUTPKTERRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt Mask (BnaOutIntrMsk)"]
    #[inline(always)]
    pub fn bnaoutintrmsk(&mut self) -> BNAOUTINTRMSK_W<'_, DOEPEACHMSK15_SPEC> {
        BNAOUTINTRMSK_W::new(self, 9)
    }
    #[doc = "Bit 12 - Babble Error interrupt Mask (BbleErrMsk)"]
    #[inline(always)]
    pub fn bbleerrmsk(&mut self) -> BBLEERRMSK_W<'_, DOEPEACHMSK15_SPEC> {
        BBLEERRMSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt Mask (NAKMsk)"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<'_, DOEPEACHMSK15_SPEC> {
        NAKMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt Mask (NYETMsk)"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<'_, DOEPEACHMSK15_SPEC> {
        NYETMSK_W::new(self, 14)
    }
}
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPEACHMSK15_SPEC;
impl crate::RegisterSpec for DOEPEACHMSK15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepeachmsk15::R`](R) reader structure"]
impl crate::Readable for DOEPEACHMSK15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepeachmsk15::W`](W) writer structure"]
impl crate::Writable for DOEPEACHMSK15_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPEACHMSK15 to value 0"]
impl crate::Resettable for DOEPEACHMSK15_SPEC {}
