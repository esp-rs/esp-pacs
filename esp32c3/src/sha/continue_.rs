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
#[doc = "Field `CONTINUE` reader - Reserved."]
pub type CONTINUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 1:31 - Reserved."]
    #[inline(always)]
    pub fn continue_(&self) -> CONTINUE_R {
        CONTINUE_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
