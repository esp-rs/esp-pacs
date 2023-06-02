#[doc = "Register `APP_CPU_RECORD_PDEBUGSTATUS` reader"]
pub struct R(crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECORD_APP_PDEBUGSTATUS` reader - "]
pub type RECORD_APP_PDEBUGSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_app_pdebugstatus(&self) -> RECORD_APP_PDEBUGSTATUS_R {
        RECORD_APP_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_PDEBUGSTATUS")
            .field(
                "record_app_pdebugstatus",
                &format_args!("{}", self.record_app_pdebugstatus().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_CPU_RECORD_PDEBUGSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_pdebugstatus](index.html) module"]
pub struct APP_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_pdebugstatus::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
