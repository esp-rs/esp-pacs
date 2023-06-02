#[doc = "Register `QUERY_IDLE` reader"]
pub struct R(crate::R<QUERY_IDLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_IDLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_IDLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_IDLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUERY_IDLE` reader - Represents the RSA status. 0: Busy 1: Idle"]
pub type QUERY_IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the RSA status. 0: Busy 1: Idle"]
    #[inline(always)]
    pub fn query_idle(&self) -> QUERY_IDLE_R {
        QUERY_IDLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_IDLE")
            .field("query_idle", &format_args!("{}", self.query_idle().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_IDLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Represents the RSA status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_idle](index.html) module"]
pub struct QUERY_IDLE_SPEC;
impl crate::RegisterSpec for QUERY_IDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_idle::R](R) reader structure"]
impl crate::Readable for QUERY_IDLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_IDLE to value 0"]
impl crate::Resettable for QUERY_IDLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
