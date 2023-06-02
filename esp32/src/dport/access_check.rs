#[doc = "Register `ACCESS_CHECK` reader"]
pub struct R(crate::R<ACCESS_CHECK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCESS_CHECK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCESS_CHECK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCESS_CHECK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO` reader - "]
pub type PRO_R = crate::BitReader;
#[doc = "Field `APP` reader - "]
pub type APP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACCESS_CHECK")
            .field("pro", &format_args!("{}", self.pro().bit()))
            .field("app", &format_args!("{}", self.app().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACCESS_CHECK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [access_check](index.html) module"]
pub struct ACCESS_CHECK_SPEC;
impl crate::RegisterSpec for ACCESS_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [access_check::R](R) reader structure"]
impl crate::Readable for ACCESS_CHECK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCESS_CHECK to value 0"]
impl crate::Resettable for ACCESS_CHECK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
