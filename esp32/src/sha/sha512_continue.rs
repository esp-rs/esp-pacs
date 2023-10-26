#[doc = "Register `SHA512_CONTINUE` writer"]
pub type W = crate::W<SHA512_CONTINUE_SPEC>;
#[doc = "Field `SHA512_CONTINUE` writer - Write 1 to continue the SHA-512 operation with subsequent blocks."]
pub type SHA512_CONTINUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA512_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue the SHA-512 operation with subsequent blocks."]
    #[inline(always)]
    #[must_use]
    pub fn sha512_continue(&mut self) -> SHA512_CONTINUE_W<SHA512_CONTINUE_SPEC, 0> {
        SHA512_CONTINUE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha512_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA512_CONTINUE_SPEC;
impl crate::RegisterSpec for SHA512_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha512_continue::W`](W) writer structure"]
impl crate::Writable for SHA512_CONTINUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA512_CONTINUE to value 0"]
impl crate::Resettable for SHA512_CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
