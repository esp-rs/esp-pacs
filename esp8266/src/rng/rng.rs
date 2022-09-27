#[doc = "Register `rng` reader"]
pub struct R(crate::R<RNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RNG register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng](index.html) module"]
pub struct RNG_SPEC;
impl crate::RegisterSpec for RNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng::R](R) reader structure"]
impl crate::Readable for RNG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rng to value 0"]
impl crate::Resettable for RNG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
