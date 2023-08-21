#[doc = "Register `reset_stage` reader"]
pub type R = crate::R<RESET_STAGE_SPEC>;
#[doc = "Register `reset_stage` writer"]
pub type W = crate::W<RESET_STAGE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_STAGE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Watchdog stage reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_stage::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_stage::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_STAGE_SPEC;
impl crate::RegisterSpec for RESET_STAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_stage::R`](R) reader structure"]
impl crate::Readable for RESET_STAGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_stage::W`](W) writer structure"]
impl crate::Writable for RESET_STAGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reset_stage to value 0"]
impl crate::Resettable for RESET_STAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
