#[doc = "Register `BLEWLPRIVADDRPTR` reader"]
pub type R = crate::R<BLEWLPRIVADDRPTR_SPEC>;
#[doc = "Register `BLEWLPRIVADDRPTR` writer"]
pub type W = crate::W<BLEWLPRIVADDRPTR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Pointer to private devices whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlprivaddrptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlprivaddrptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEWLPRIVADDRPTR_SPEC;
impl crate::RegisterSpec for BLEWLPRIVADDRPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blewlprivaddrptr::R`](R) reader structure"]
impl crate::Readable for BLEWLPRIVADDRPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blewlprivaddrptr::W`](W) writer structure"]
impl crate::Writable for BLEWLPRIVADDRPTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEWLPRIVADDRPTR to value 0"]
impl crate::Resettable for BLEWLPRIVADDRPTR_SPEC {}
