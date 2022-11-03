#[doc = "Register `RNG_DATA` reader"]
pub struct R(crate::R<RNG_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RND_DATA` reader - need_des"]
pub type RND_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new(self.bits)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_data](index.html) module"]
pub struct RNG_DATA_SPEC;
impl crate::RegisterSpec for RNG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_data::R](R) reader structure"]
impl crate::Readable for RNG_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RNG_DATA to value 0"]
impl crate::Resettable for RNG_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
