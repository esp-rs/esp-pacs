#[doc = "Register `CONTINUE` reader"]
pub struct R(crate::R<CONTINUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTINUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTINUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTINUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONTINUE` reader - reserved."]
pub type CONTINUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 1:31 - reserved."]
    #[inline(always)]
    pub fn continue_(&self) -> CONTINUE_R {
        CONTINUE_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTINUE")
            .field("continue_", &format_args!("{}", self.continue_().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Typical SHA configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continue_](index.html) module"]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [continue_::R](R) reader structure"]
impl crate::Readable for CONTINUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
