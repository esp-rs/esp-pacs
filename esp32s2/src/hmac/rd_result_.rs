#[doc = "Register `RD_RESULT_%s` reader"]
pub struct R(crate::R<RD_RESULT__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RESULT__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RESULT__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RESULT__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Read the %sth 32-bit of hash result."]
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read the %sth 32-bit of hash result."]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RESULT_")
            .field("rdata", &format_args!("{}", self.rdata().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_RESULT__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Hash result register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_result_](index.html) module"]
pub struct RD_RESULT__SPEC;
impl crate::RegisterSpec for RD_RESULT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_result_::R](R) reader structure"]
impl crate::Readable for RD_RESULT__SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RESULT_%s to value 0"]
impl crate::Resettable for RD_RESULT__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
