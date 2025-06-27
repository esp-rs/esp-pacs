#[doc = "Register `SHA1_CONTINUE` writer"]
pub type W = crate::W<SHA1_CONTINUE_SPEC>;
#[doc = "Field `SHA1_CONTINUE` writer - Write 1 to continue the SHA-1 operation with subsequent blocks."]
pub type SHA1_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA1_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue the SHA-1 operation with subsequent blocks."]
    #[inline(always)]
    pub fn sha1_continue(&mut self) -> SHA1_CONTINUE_W<SHA1_CONTINUE_SPEC> {
        SHA1_CONTINUE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha1_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA1_CONTINUE_SPEC;
impl crate::RegisterSpec for SHA1_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha1_continue::W`](W) writer structure"]
impl crate::Writable for SHA1_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA1_CONTINUE to value 0"]
impl crate::Resettable for SHA1_CONTINUE_SPEC {}
