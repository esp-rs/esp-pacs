#[doc = "Register `SHA512_CONTINUE` writer"]
pub type W = crate::W<SHA512_CONTINUE_SPEC>;
#[doc = "Field `SHA512_CONTINUE` writer - Write 1 to continue the SHA-512 operation with subsequent blocks."]
pub type SHA512_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA512_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue the SHA-512 operation with subsequent blocks."]
    #[inline(always)]
    pub fn sha512_continue(&mut self) -> SHA512_CONTINUE_W<'_, SHA512_CONTINUE_SPEC> {
        SHA512_CONTINUE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha512_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA512_CONTINUE_SPEC;
impl crate::RegisterSpec for SHA512_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha512_continue::W`](W) writer structure"]
impl crate::Writable for SHA512_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA512_CONTINUE to value 0"]
impl crate::Resettable for SHA512_CONTINUE_SPEC {}
