#[doc = "Register `APP_CPU_RECORD_PDEBUGPC` reader"]
pub struct R(crate::R<APP_CPU_RECORD_PDEBUGPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_PDEBUGPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_PDEBUGPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_PDEBUGPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECORD_APP_PDEBUGPC` reader - "]
pub type RECORD_APP_PDEBUGPC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_app_pdebugpc(&self) -> RECORD_APP_PDEBUGPC_R {
        RECORD_APP_PDEBUGPC_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_pdebugpc](index.html) module"]
pub struct APP_CPU_RECORD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_pdebugpc::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGPC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGPC to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
