#[doc = "Register `APP_CPU_RECORD_PID` reader"]
pub struct R(crate::R<APP_CPU_RECORD_PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECORD_APP_PID` reader - "]
pub type RECORD_APP_PID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn record_app_pid(&self) -> RECORD_APP_PID_R {
        RECORD_APP_PID_R::new((self.bits & 7) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_pid](index.html) module"]
pub struct APP_CPU_RECORD_PID_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_pid::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_PID to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
