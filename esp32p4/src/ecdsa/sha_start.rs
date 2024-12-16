#[doc = "Register `SHA_START` writer"]
pub type W = crate::W<SHA_START_SPEC>;
#[doc = "Field `SHA_START` writer - Write 1 to start the first caculation of SHA Calculator in ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type SHA_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start the first caculation of SHA Calculator in ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    pub fn sha_start(&mut self) -> SHA_START_W<SHA_START_SPEC> {
        SHA_START_W::new(self, 0)
    }
}
#[doc = "ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA_START_SPEC;
impl crate::RegisterSpec for SHA_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha_start::W`](W) writer structure"]
impl crate::Writable for SHA_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA_START to value 0"]
impl crate::Resettable for SHA_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
