#[doc = "Register `APP_CPU_RECORD_PDEBUGLS0STAT` reader"]
pub type R = crate::R<APP_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
#[doc = "Field `RECORD_APP_PDEBUGLS0STAT` reader - "]
pub type RECORD_APP_PDEBUGLS0STAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_app_pdebugls0stat(&self) -> RECORD_APP_PDEBUGLS0STAT_R {
        RECORD_APP_PDEBUGLS0STAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_PDEBUGLS0STAT")
            .field("record_app_pdebugls0stat", &self.record_app_pdebugls0stat())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cpu_record_pdebugls0stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_PDEBUGLS0STAT_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_pdebugls0stat::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGLS0STAT_SPEC {}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGLS0STAT_SPEC {}
