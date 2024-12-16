#[doc = "Register `SHA1_LOAD` writer"]
pub type W = crate::W<SHA1_LOAD_SPEC>;
#[doc = "Field `SHA1_LOAD` writer - Write 1 to finish the SHA-1 operation to calculate the final message hash."]
pub type SHA1_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-1 operation to calculate the final message hash."]
    #[inline(always)]
    pub fn sha1_load(&mut self) -> SHA1_LOAD_W<SHA1_LOAD_SPEC> {
        SHA1_LOAD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha1_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA1_LOAD_SPEC;
impl crate::RegisterSpec for SHA1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha1_load::W`](W) writer structure"]
impl crate::Writable for SHA1_LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA1_LOAD to value 0"]
impl crate::Resettable for SHA1_LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
