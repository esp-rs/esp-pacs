#[doc = "Register `SHA384_LOAD` writer"]
pub type W = crate::W<SHA384_LOAD_SPEC>;
#[doc = "Field `SHA384_LOAD` writer - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
pub type SHA384_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA384_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
    #[inline(always)]
    #[must_use]
    pub fn sha384_load(&mut self) -> SHA384_LOAD_W<SHA384_LOAD_SPEC> {
        SHA384_LOAD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha384_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA384_LOAD_SPEC;
impl crate::RegisterSpec for SHA384_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha384_load::W`](W) writer structure"]
impl crate::Writable for SHA384_LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA384_LOAD to value 0"]
impl crate::Resettable for SHA384_LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
