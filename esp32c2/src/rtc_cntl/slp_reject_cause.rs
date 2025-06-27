#[doc = "Register `SLP_REJECT_CAUSE` reader"]
pub type R = crate::R<SLP_REJECT_CAUSE_SPEC>;
#[doc = "Register `SLP_REJECT_CAUSE` writer"]
pub type W = crate::W<SLP_REJECT_CAUSE_SPEC>;
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub type REJECT_CAUSE_R = crate::FieldReader<u32>;
#[doc = "Field `REJECT_CAUSE` writer - sleep reject cause"]
pub type REJECT_CAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CAUSE")
            .field("reject_cause", &self.reject_cause())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&mut self) -> REJECT_CAUSE_W<SLP_REJECT_CAUSE_SPEC> {
        REJECT_CAUSE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_reject_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_reject_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_reject_cause::R`](R) reader structure"]
impl crate::Readable for SLP_REJECT_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_reject_cause::W`](W) writer structure"]
impl crate::Writable for SLP_REJECT_CAUSE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_REJECT_CAUSE to value 0"]
impl crate::Resettable for SLP_REJECT_CAUSE_SPEC {}
