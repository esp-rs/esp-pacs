#[doc = "Register `SLP_WAKEUP_CNTL1` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL1_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL1` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL1_SPEC>;
#[doc = "Field `SLEEP_REJECT_ENA` reader - need_des"]
pub type SLEEP_REJECT_ENA_R = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_REJECT_ENA` writer - need_des"]
pub type SLEEP_REJECT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL1")
            .field("sleep_reject_ena", &self.sleep_reject_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W<'_, SLP_WAKEUP_CNTL1_SPEC> {
        SLEEP_REJECT_ENA_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL1_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl1::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl1::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL1 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL1_SPEC {}
