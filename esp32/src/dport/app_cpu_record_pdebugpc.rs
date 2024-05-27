#[doc = "Register `APP_CPU_RECORD_PDEBUGPC` reader"]
pub type R = crate::R<APP_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = "Field `RECORD_APP_PDEBUGPC` reader - "]
pub type RECORD_APP_PDEBUGPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_app_pdebugpc(&self) -> RECORD_APP_PDEBUGPC_R {
        RECORD_APP_PDEBUGPC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_PDEBUGPC")
            .field("record_app_pdebugpc", &self.record_app_pdebugpc())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pdebugpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CPU_RECORD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cpu_record_pdebugpc::R`](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGPC_SPEC {}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGPC to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
