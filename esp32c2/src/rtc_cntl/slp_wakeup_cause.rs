#[doc = "Register `SLP_WAKEUP_CAUSE` reader"]
pub type R = crate::R<SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "Register `SLP_WAKEUP_CAUSE` writer"]
pub type W = crate::W<SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "Field `WAKEUP_CAUSE` reader - sleep wakeup cause"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u32>;
#[doc = "Field `WAKEUP_CAUSE` writer - sleep wakeup cause"]
pub type WAKEUP_CAUSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - sleep wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CAUSE")
            .field(
                "wakeup_cause",
                &format_args!("{}", self.wakeup_cause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CAUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:16 - sleep wakeup cause"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_cause(&mut self) -> WAKEUP_CAUSE_W<SLP_WAKEUP_CAUSE_SPEC, 0> {
        WAKEUP_CAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cause::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cause::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CAUSE to value 0"]
impl crate::Resettable for SLP_WAKEUP_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
