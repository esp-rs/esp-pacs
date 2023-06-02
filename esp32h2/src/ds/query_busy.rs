#[doc = "Register `QUERY_BUSY` reader"]
pub struct R(crate::R<QUERY_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUERY_BUSY` reader - digital signature state. 1'b0: idle, 1'b1: busy"]
pub type QUERY_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - digital signature state. 1'b0: idle, 1'b1: busy"]
    #[inline(always)]
    pub fn query_busy(&self) -> QUERY_BUSY_R {
        QUERY_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_BUSY")
            .field("query_busy", &format_args!("{}", self.query_busy().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_BUSY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DS query busy register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_busy](index.html) module"]
pub struct QUERY_BUSY_SPEC;
impl crate::RegisterSpec for QUERY_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_busy::R](R) reader structure"]
impl crate::Readable for QUERY_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_BUSY to value 0"]
impl crate::Resettable for QUERY_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
