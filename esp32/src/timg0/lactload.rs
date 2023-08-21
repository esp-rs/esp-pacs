#[doc = "Register `LACTLOAD` writer"]
pub type W = crate::W<LACTLOAD_SPEC>;
#[doc = "Field `LACT_LOAD` writer - "]
pub type LACT_LOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_load(&mut self) -> LACT_LOAD_W<LACTLOAD_SPEC, 0> {
        LACT_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLOAD_SPEC;
impl crate::RegisterSpec for LACTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lactload::W`](W) writer structure"]
impl crate::Writable for LACTLOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTLOAD to value 0"]
impl crate::Resettable for LACTLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
