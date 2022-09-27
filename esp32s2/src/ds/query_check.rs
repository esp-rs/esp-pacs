#[doc = "Register `QUERY_CHECK` reader"]
pub struct R(crate::R<QUERY_CHECK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_CHECK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_CHECK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_CHECK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MD_ERROR` reader - 1: MD check fails. 0: MD check passes."]
pub type MD_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `PADDING_BAD` reader - 1: The padding check fails. 0: The padding check passes."]
pub type PADDING_BAD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1: MD check fails. 0: MD check passes."]
    #[inline(always)]
    pub fn md_error(&self) -> MD_ERROR_R {
        MD_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: The padding check fails. 0: The padding check passes."]
    #[inline(always)]
    pub fn padding_bad(&self) -> PADDING_BAD_R {
        PADDING_BAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Queries DS check result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_check](index.html) module"]
pub struct QUERY_CHECK_SPEC;
impl crate::RegisterSpec for QUERY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_check::R](R) reader structure"]
impl crate::Readable for QUERY_CHECK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_CHECK to value 0"]
impl crate::Resettable for QUERY_CHECK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
