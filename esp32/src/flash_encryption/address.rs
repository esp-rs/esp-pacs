#[doc = "Register `ADDRESS` writer"]
pub type W = crate::W<ADDRESS_SPEC>;
#[doc = "Field `ADDRESS` writer - The physical address on the off-chip flash must be 8-word boundary aligned."]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADDRESS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The physical address on the off-chip flash must be 8-word boundary aligned."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<ADDRESS_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRESS_SPEC;
impl crate::RegisterSpec for ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for ADDRESS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for ADDRESS_SPEC {
    const RESET_VALUE: u32 = 0;
}
