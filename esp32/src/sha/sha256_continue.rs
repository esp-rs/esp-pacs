#[doc = "Register `SHA256_CONTINUE` writer"]
pub type W = crate::W<SHA256_CONTINUE_SPEC>;
#[doc = "Field `SHA256_CONTINUE` writer - Write 1 to continue the SHA-256 operation with subsequent blocks."]
pub type SHA256_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA256_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue the SHA-256 operation with subsequent blocks."]
    #[inline(always)]
    #[must_use]
    pub fn sha256_continue(&mut self) -> SHA256_CONTINUE_W<SHA256_CONTINUE_SPEC> {
        SHA256_CONTINUE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha256_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA256_CONTINUE_SPEC;
impl crate::RegisterSpec for SHA256_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha256_continue::W`](W) writer structure"]
impl crate::Writable for SHA256_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA256_CONTINUE to value 0"]
impl crate::Resettable for SHA256_CONTINUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
