#[doc = "Register `rng` reader"]
pub type R = crate::R<RNG_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RNG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RNG register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_SPEC;
impl crate::RegisterSpec for RNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng::R`](R) reader structure"]
impl crate::Readable for RNG_SPEC {}
#[doc = "`reset()` method sets rng to value 0"]
impl crate::Resettable for RNG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
