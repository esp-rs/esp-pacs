#[doc = "Register `SHA1_START` writer"]
pub type W = crate::W<SHA1_START_SPEC>;
#[doc = "Field `SHA1_START` writer - Write 1 to start an SHA-1 operation on the first message block."]
pub type SHA1_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA1_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start an SHA-1 operation on the first message block."]
    #[inline(always)]
    #[must_use]
    pub fn sha1_start(&mut self) -> SHA1_START_W<SHA1_START_SPEC> {
        SHA1_START_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha1_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA1_START_SPEC;
impl crate::RegisterSpec for SHA1_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha1_start::W`](W) writer structure"]
impl crate::Writable for SHA1_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA1_START to value 0"]
impl crate::Resettable for SHA1_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
