#[doc = "Register `BLEAESPTR` reader"]
pub type R = crate::R<BLEAESPTR_SPEC>;
#[doc = "Register `BLEAESPTR` writer"]
pub type W = crate::W<BLEAESPTR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to the block to encrypt/decrypt with AES\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaesptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaesptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEAESPTR_SPEC;
impl crate::RegisterSpec for BLEAESPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleaesptr::R`](R) reader structure"]
impl crate::Readable for BLEAESPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleaesptr::W`](W) writer structure"]
impl crate::Writable for BLEAESPTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEAESPTR to value 0"]
impl crate::Resettable for BLEAESPTR_SPEC {}
