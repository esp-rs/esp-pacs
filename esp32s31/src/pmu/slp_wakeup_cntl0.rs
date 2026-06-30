#[doc = "Register `SLP_WAKEUP_CNTL0` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL0_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL0` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL0_SPEC>;
#[doc = "Field `SLP_REJECT_EN` reader - need_des"]
pub type SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_EN` writer - need_des"]
pub type SLP_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_REQ` writer - need_des"]
pub type SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn slp_reject_en(&self) -> SLP_REJECT_EN_R {
        SLP_REJECT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL0")
            .field("slp_reject_en", &self.slp_reject_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn slp_reject_en(&mut self) -> SLP_REJECT_EN_W<'_, SLP_WAKEUP_CNTL0_SPEC> {
        SLP_REJECT_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep_req(&mut self) -> SLEEP_REQ_W<'_, SLP_WAKEUP_CNTL0_SPEC> {
        SLEEP_REQ_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL0_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl0::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl0::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL0 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL0_SPEC {}
