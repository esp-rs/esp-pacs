#[doc = "Register `SHA1_LOAD` writer"]
pub type W = crate::W<SHA1_LOAD_SPEC>;
#[doc = "Field `SHA1_LOAD` writer - Write 1 to finish the SHA-1 operation to calculate the final message hash."]
pub type SHA1_LOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA1_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-1 operation to calculate the final message hash."]
    #[inline(always)]
    #[must_use]
    pub fn sha1_load(&mut self) -> SHA1_LOAD_W<SHA1_LOAD_SPEC, 0> {
        SHA1_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha1_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA1_LOAD_SPEC;
impl crate::RegisterSpec for SHA1_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha1_load::W`](W) writer structure"]
impl crate::Writable for SHA1_LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA1_LOAD to value 0"]
impl crate::Resettable for SHA1_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
