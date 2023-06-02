#[doc = "Register `QUERY_CLEAN` reader"]
pub struct R(crate::R<QUERY_CLEAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_CLEAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_CLEAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_CLEAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUERY_CLEAN` reader - Represents whether or not the RSA memory completes initialization. 0: Not complete 1: Completed"]
pub type QUERY_CLEAN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not the RSA memory completes initialization. 0: Not complete 1: Completed"]
    #[inline(always)]
    pub fn query_clean(&self) -> QUERY_CLEAN_R {
        QUERY_CLEAN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_CLEAN")
            .field("query_clean", &format_args!("{}", self.query_clean().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_CLEAN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RSA clean register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_clean](index.html) module"]
pub struct QUERY_CLEAN_SPEC;
impl crate::RegisterSpec for QUERY_CLEAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_clean::R](R) reader structure"]
impl crate::Readable for QUERY_CLEAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_CLEAN to value 0"]
impl crate::Resettable for QUERY_CLEAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
