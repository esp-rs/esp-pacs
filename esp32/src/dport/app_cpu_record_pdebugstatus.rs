#[doc = "Register `APP_CPU_RECORD_PDEBUGSTATUS` reader"]
pub type R = crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>;
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
            .field("record_app_pdebugstatus", &self.record_app_pdebugstatus())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cpu_record_pdebugstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_pdebugstatus::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {}
