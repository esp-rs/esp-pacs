#[doc = "Register `BLEWLPUBADDRPTR` reader"]
pub type R = crate::R<BLEWLPUBADDRPTR_SPEC>;
#[doc = "Register `BLEWLPUBADDRPTR` writer"]
pub type W = crate::W<BLEWLPUBADDRPTR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to public devices whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlpubaddrptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlpubaddrptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEWLPUBADDRPTR_SPEC;
impl crate::RegisterSpec for BLEWLPUBADDRPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blewlpubaddrptr::R`](R) reader structure"]
impl crate::Readable for BLEWLPUBADDRPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blewlpubaddrptr::W`](W) writer structure"]
impl crate::Writable for BLEWLPUBADDRPTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEWLPUBADDRPTR to value 0"]
impl crate::Resettable for BLEWLPUBADDRPTR_SPEC {}
