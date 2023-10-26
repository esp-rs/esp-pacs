#[doc = "Register `SHA256_START` writer"]
pub type W = crate::W<SHA256_START_SPEC>;
#[doc = "Field `SHA256_START` writer - Write 1 to start an SHA-256 operation on the first message block."]
pub type SHA256_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA256_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start an SHA-256 operation on the first message block."]
    #[inline(always)]
    #[must_use]
    pub fn sha256_start(&mut self) -> SHA256_START_W<SHA256_START_SPEC, 0> {
        SHA256_START_W::new(self)
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
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha256_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA256_START_SPEC;
impl crate::RegisterSpec for SHA256_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha256_start::W`](W) writer structure"]
impl crate::Writable for SHA256_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA256_START to value 0"]
impl crate::Resettable for SHA256_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
