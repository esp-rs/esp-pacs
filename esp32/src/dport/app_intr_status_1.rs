#[doc = "Register `APP_INTR_STATUS_1` reader"]
pub struct R(crate::R<APP_INTR_STATUS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_INTR_STATUS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_INTR_STATUS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_INTR_STATUS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APP_INTR_STATUS_1` reader - "]
pub type APP_INTR_STATUS_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_intr_status_1(&self) -> APP_INTR_STATUS_1_R {
        APP_INTR_STATUS_1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_intr_status_1](index.html) module"]
pub struct APP_INTR_STATUS_1_SPEC;
impl crate::RegisterSpec for APP_INTR_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_intr_status_1::R](R) reader structure"]
impl crate::Readable for APP_INTR_STATUS_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_INTR_STATUS_1 to value 0"]
impl crate::Resettable for APP_INTR_STATUS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
