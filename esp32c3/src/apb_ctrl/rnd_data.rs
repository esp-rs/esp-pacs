#[doc = "Register `RND_DATA` reader"]
pub struct R(crate::R<RND_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RND_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RND_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RND_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RND_DATA` reader - reg_rnd_data"]
pub type RND_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_rnd_data"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_DATA")
            .field("rnd_data", &format_args!("{}", self.rnd_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RND_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "APB_CTRL_RND_DATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_data](index.html) module"]
pub struct RND_DATA_SPEC;
impl crate::RegisterSpec for RND_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnd_data::R](R) reader structure"]
impl crate::Readable for RND_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RND_DATA to value 0"]
impl crate::Resettable for RND_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
