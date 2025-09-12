#[doc = "Register `FILTER_CONTROL_FILTER_STATUS` reader"]
pub type R = crate::R<FILTER_CONTROL_FILTER_STATUS_SPEC>;
#[doc = "Register `FILTER_CONTROL_FILTER_STATUS` writer"]
pub type W = crate::W<FILTER_CONTROL_FILTER_STATUS_SPEC>;
#[doc = "Field `FANB` reader - CAN Basic Frame is accepted by filter A."]
pub type FANB_R = crate::BitReader;
#[doc = "Field `FANB` writer - CAN Basic Frame is accepted by filter A."]
pub type FANB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FANE` reader - CAN Extended Frame is accepted by Filter A."]
pub type FANE_R = crate::BitReader;
#[doc = "Field `FANE` writer - CAN Extended Frame is accepted by Filter A."]
pub type FANE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAFB` reader - CAN FD Basic Frame is accepted by filter A."]
pub type FAFB_R = crate::BitReader;
#[doc = "Field `FAFB` writer - CAN FD Basic Frame is accepted by filter A."]
pub type FAFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAFE` reader - CAN FD Extended Frame is accepted by filter A."]
pub type FAFE_R = crate::BitReader;
#[doc = "Field `FAFE` writer - CAN FD Extended Frame is accepted by filter A."]
pub type FAFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBNB` reader - CAN Basic Frame is accepted by filter B."]
pub type FBNB_R = crate::BitReader;
#[doc = "Field `FBNB` writer - CAN Basic Frame is accepted by filter B."]
pub type FBNB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBNE` reader - CAN Extended Frame is accepted by Filter B."]
pub type FBNE_R = crate::BitReader;
#[doc = "Field `FBNE` writer - CAN Extended Frame is accepted by Filter B."]
pub type FBNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBFB` reader - CAN FD Basic Frame is accepted by filter B."]
pub type FBFB_R = crate::BitReader;
#[doc = "Field `FBFB` writer - CAN FD Basic Frame is accepted by filter B."]
pub type FBFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBFE` reader - CAN FD Extended Frame is accepted by filter B."]
pub type FBFE_R = crate::BitReader;
#[doc = "Field `FBFE` writer - CAN FD Extended Frame is accepted by filter B."]
pub type FBFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCNB` reader - CAN Basic Frame is accepted by filter C."]
pub type FCNB_R = crate::BitReader;
#[doc = "Field `FCNB` writer - CAN Basic Frame is accepted by filter C."]
pub type FCNB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCNE` reader - CAN Extended Frame is accepted by Filter C."]
pub type FCNE_R = crate::BitReader;
#[doc = "Field `FCNE` writer - CAN Extended Frame is accepted by Filter C."]
pub type FCNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCFB` reader - CAN FD Basic Frame is accepted by filter C."]
pub type FCFB_R = crate::BitReader;
#[doc = "Field `FCFB` writer - CAN FD Basic Frame is accepted by filter C."]
pub type FCFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCFE` reader - CAN FD Extended Frame is accepted by filter C."]
pub type FCFE_R = crate::BitReader;
#[doc = "Field `FCFE` writer - CAN FD Extended Frame is accepted by filter C."]
pub type FCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRNB` reader - CAN Basic Frame is accepted by Range filter."]
pub type FRNB_R = crate::BitReader;
#[doc = "Field `FRNB` writer - CAN Basic Frame is accepted by Range filter."]
pub type FRNB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRNE` reader - CAN Extended Frame is accepted by Range filter."]
pub type FRNE_R = crate::BitReader;
#[doc = "Field `FRNE` writer - CAN Extended Frame is accepted by Range filter."]
pub type FRNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFB` reader - CAN FD Basic Frame is accepted by Range filter."]
pub type FRFB_R = crate::BitReader;
#[doc = "Field `FRFB` writer - CAN FD Basic Frame is accepted by Range filter."]
pub type FRFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFE` reader - CAN FD Extended Frame is accepted by Range filter."]
pub type FRFE_R = crate::BitReader;
#[doc = "Field `FRFE` writer - CAN FD Extended Frame is accepted by Range filter."]
pub type FRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFA` reader - Logic 1 when Filter A is available. Otherwise logic 0."]
pub type SFA_R = crate::BitReader;
#[doc = "Field `SFB` reader - Logic 1 when Filter B is available. Otherwise logic 0."]
pub type SFB_R = crate::BitReader;
#[doc = "Field `SFC` reader - Logic 1 when Filter C is available. Otherwise logic 0."]
pub type SFC_R = crate::BitReader;
#[doc = "Field `SFR` reader - Logic 1 when Range Filter is available. Otherwise logic 0."]
pub type SFR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CAN Basic Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fanb(&self) -> FANB_R {
        FANB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN Extended Frame is accepted by Filter A."]
    #[inline(always)]
    pub fn fane(&self) -> FANE_R {
        FANE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN FD Basic Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fafb(&self) -> FAFB_R {
        FAFB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CAN FD Extended Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fafe(&self) -> FAFE_R {
        FAFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAN Basic Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbnb(&self) -> FBNB_R {
        FBNB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CAN Extended Frame is accepted by Filter B."]
    #[inline(always)]
    pub fn fbne(&self) -> FBNE_R {
        FBNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CAN FD Basic Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbfb(&self) -> FBFB_R {
        FBFB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN FD Extended Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbfe(&self) -> FBFE_R {
        FBFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN Basic Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcnb(&self) -> FCNB_R {
        FCNB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAN Extended Frame is accepted by Filter C."]
    #[inline(always)]
    pub fn fcne(&self) -> FCNE_R {
        FCNE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CAN FD Basic Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcfb(&self) -> FCFB_R {
        FCFB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CAN FD Extended Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcfe(&self) -> FCFE_R {
        FCFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CAN Basic Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frnb(&self) -> FRNB_R {
        FRNB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN Extended Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frne(&self) -> FRNE_R {
        FRNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN FD Basic Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frfb(&self) -> FRFB_R {
        FRFB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CAN FD Extended Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frfe(&self) -> FRFE_R {
        FRFE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Logic 1 when Filter A is available. Otherwise logic 0."]
    #[inline(always)]
    pub fn sfa(&self) -> SFA_R {
        SFA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logic 1 when Filter B is available. Otherwise logic 0."]
    #[inline(always)]
    pub fn sfb(&self) -> SFB_R {
        SFB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Logic 1 when Filter C is available. Otherwise logic 0."]
    #[inline(always)]
    pub fn sfc(&self) -> SFC_R {
        SFC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logic 1 when Range Filter is available. Otherwise logic 0."]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CONTROL_FILTER_STATUS")
            .field("fanb", &self.fanb())
            .field("fane", &self.fane())
            .field("fafb", &self.fafb())
            .field("fafe", &self.fafe())
            .field("fbnb", &self.fbnb())
            .field("fbne", &self.fbne())
            .field("fbfb", &self.fbfb())
            .field("fbfe", &self.fbfe())
            .field("fcnb", &self.fcnb())
            .field("fcne", &self.fcne())
            .field("fcfb", &self.fcfb())
            .field("fcfe", &self.fcfe())
            .field("frnb", &self.frnb())
            .field("frne", &self.frne())
            .field("frfb", &self.frfb())
            .field("frfe", &self.frfe())
            .field("sfa", &self.sfa())
            .field("sfb", &self.sfb())
            .field("sfc", &self.sfc())
            .field("sfr", &self.sfr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CAN Basic Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fanb(&mut self) -> FANB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FANB_W::new(self, 0)
    }
    #[doc = "Bit 1 - CAN Extended Frame is accepted by Filter A."]
    #[inline(always)]
    pub fn fane(&mut self) -> FANE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FANE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CAN FD Basic Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fafb(&mut self) -> FAFB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FAFB_W::new(self, 2)
    }
    #[doc = "Bit 3 - CAN FD Extended Frame is accepted by filter A."]
    #[inline(always)]
    pub fn fafe(&mut self) -> FAFE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FAFE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CAN Basic Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbnb(&mut self) -> FBNB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FBNB_W::new(self, 4)
    }
    #[doc = "Bit 5 - CAN Extended Frame is accepted by Filter B."]
    #[inline(always)]
    pub fn fbne(&mut self) -> FBNE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FBNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - CAN FD Basic Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbfb(&mut self) -> FBFB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FBFB_W::new(self, 6)
    }
    #[doc = "Bit 7 - CAN FD Extended Frame is accepted by filter B."]
    #[inline(always)]
    pub fn fbfe(&mut self) -> FBFE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FBFE_W::new(self, 7)
    }
    #[doc = "Bit 8 - CAN Basic Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcnb(&mut self) -> FCNB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FCNB_W::new(self, 8)
    }
    #[doc = "Bit 9 - CAN Extended Frame is accepted by Filter C."]
    #[inline(always)]
    pub fn fcne(&mut self) -> FCNE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FCNE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CAN FD Basic Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcfb(&mut self) -> FCFB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FCFB_W::new(self, 10)
    }
    #[doc = "Bit 11 - CAN FD Extended Frame is accepted by filter C."]
    #[inline(always)]
    pub fn fcfe(&mut self) -> FCFE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FCFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - CAN Basic Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frnb(&mut self) -> FRNB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FRNB_W::new(self, 12)
    }
    #[doc = "Bit 13 - CAN Extended Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frne(&mut self) -> FRNE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FRNE_W::new(self, 13)
    }
    #[doc = "Bit 14 - CAN FD Basic Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frfb(&mut self) -> FRFB_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FRFB_W::new(self, 14)
    }
    #[doc = "Bit 15 - CAN FD Extended Frame is accepted by Range filter."]
    #[inline(always)]
    pub fn frfe(&mut self) -> FRFE_W<'_, FILTER_CONTROL_FILTER_STATUS_SPEC> {
        FRFE_W::new(self, 15)
    }
}
#[doc = "TWAI FD filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control_filter_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control_filter_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CONTROL_FILTER_STATUS_SPEC;
impl crate::RegisterSpec for FILTER_CONTROL_FILTER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_control_filter_status::R`](R) reader structure"]
impl crate::Readable for FILTER_CONTROL_FILTER_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_control_filter_status::W`](W) writer structure"]
impl crate::Writable for FILTER_CONTROL_FILTER_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_CONTROL_FILTER_STATUS to value 0x000f_000f"]
impl crate::Resettable for FILTER_CONTROL_FILTER_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x000f_000f;
}
