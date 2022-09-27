#[doc = "Register `INTR_STATUS_3` reader"]
pub struct R(crate::R<INTR_STATUS_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_STATUS_3` reader - this register store the status of the first 32 interrupt source"]
pub type INTR_STATUS_3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - this register store the status of the first 32 interrupt source"]
    #[inline(always)]
    pub fn intr_status_3(&self) -> INTR_STATUS_3_R {
        INTR_STATUS_3_R::new(self.bits)
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status_3](index.html) module"]
pub struct INTR_STATUS_3_SPEC;
impl crate::RegisterSpec for INTR_STATUS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status_3::R](R) reader structure"]
impl crate::Readable for INTR_STATUS_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_STATUS_3 to value 0"]
impl crate::Resettable for INTR_STATUS_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
