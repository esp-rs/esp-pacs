#[doc = "Register `SHA256_START` writer"]
pub type W = crate::W<SHA256_START_SPEC>;
#[doc = "Field `SHA256_START` writer - Write 1 to start an SHA-256 operation on the first message block."]
pub type SHA256_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA256_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start an SHA-256 operation on the first message block."]
    #[inline(always)]
    pub fn sha256_start(&mut self) -> SHA256_START_W<SHA256_START_SPEC> {
        SHA256_START_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA256_START_SPEC;
impl crate::RegisterSpec for SHA256_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha256_start::W`](W) writer structure"]
impl crate::Writable for SHA256_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA256_START to value 0"]
impl crate::Resettable for SHA256_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
