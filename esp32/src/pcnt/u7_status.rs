#[doc = "Register `U7_STATUS` reader"]
pub struct R(crate::R<U7_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U7_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U7_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U7_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_STATUS_U7` reader - "]
pub type CORE_STATUS_U7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u7(&self) -> CORE_STATUS_U7_R {
        CORE_STATUS_U7_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u7_status](index.html) module"]
pub struct U7_STATUS_SPEC;
impl crate::RegisterSpec for U7_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u7_status::R](R) reader structure"]
impl crate::Readable for U7_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U7_STATUS to value 0"]
impl crate::Resettable for U7_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
