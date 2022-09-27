#[doc = "Register `QUERY_ERROR` reader"]
pub struct R(crate::R<QUERY_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUREY_CHECK` reader - Hmac configuration state. 0: key are agree with purpose. 1: error"]
pub type QUREY_CHECK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Hmac configuration state. 0: key are agree with purpose. 1: error"]
    #[inline(always)]
    pub fn qurey_check(&self) -> QUREY_CHECK_R {
        QUREY_CHECK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Error register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_error](index.html) module"]
pub struct QUERY_ERROR_SPEC;
impl crate::RegisterSpec for QUERY_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_error::R](R) reader structure"]
impl crate::Readable for QUERY_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_ERROR to value 0"]
impl crate::Resettable for QUERY_ERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
