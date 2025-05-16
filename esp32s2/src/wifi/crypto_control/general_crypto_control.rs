#[doc = "Register `GENERAL_CRYPTO_CONTROL` reader"]
pub type R = crate::R<GENERAL_CRYPTO_CONTROL_SPEC>;
#[doc = "Register `GENERAL_CRYPTO_CONTROL` writer"]
pub type W = crate::W<GENERAL_CRYPTO_CONTROL_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Control over the whole crypto unit\n\nYou can [`read`](crate::Reg::read) this register and get [`general_crypto_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`general_crypto_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GENERAL_CRYPTO_CONTROL_SPEC;
impl crate::RegisterSpec for GENERAL_CRYPTO_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`general_crypto_control::R`](R) reader structure"]
impl crate::Readable for GENERAL_CRYPTO_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`general_crypto_control::W`](W) writer structure"]
impl crate::Writable for GENERAL_CRYPTO_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GENERAL_CRYPTO_CONTROL to value 0"]
impl crate::Resettable for GENERAL_CRYPTO_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
