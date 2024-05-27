#[doc = "Register `APP_CPU_RECORD_PDEBUGLS0DATA` reader"]
pub type R = crate::R<APP_CPU_RECORD_PDEBUGLS0DATA_SPEC>;
#[doc = "Field `RECORD_APP_PDEBUGLS0DATA` reader - "]
pub type RECORD_APP_PDEBUGLS0DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_app_pdebugls0data(&self) -> RECORD_APP_PDEBUGLS0DATA_R {
        RECORD_APP_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_PDEBUGLS0DATA")
            .field("record_app_pdebugls0data", &self.record_app_pdebugls0data())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugls0data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_pdebugls0data::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGLS0DATA_SPEC {}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGLS0DATA to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGLS0DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
