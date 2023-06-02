#[doc = "Register `QUERY_KEY_WRONG` reader"]
pub struct R(crate::R<QUERY_KEY_WRONG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUERY_KEY_WRONG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUERY_KEY_WRONG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUERY_KEY_WRONG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `QUERY_KEY_WRONG` reader - digital signature key wrong counter"]
pub type QUERY_KEY_WRONG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - digital signature key wrong counter"]
    #[inline(always)]
    pub fn query_key_wrong(&self) -> QUERY_KEY_WRONG_R {
        QUERY_KEY_WRONG_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_KEY_WRONG")
            .field(
                "query_key_wrong",
                &format_args!("{}", self.query_key_wrong().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_KEY_WRONG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DS query key-wrong counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [query_key_wrong](index.html) module"]
pub struct QUERY_KEY_WRONG_SPEC;
impl crate::RegisterSpec for QUERY_KEY_WRONG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [query_key_wrong::R](R) reader structure"]
impl crate::Readable for QUERY_KEY_WRONG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QUERY_KEY_WRONG to value 0"]
impl crate::Resettable for QUERY_KEY_WRONG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
