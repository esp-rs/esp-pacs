#[doc = "Register `PRO_CPU_RECORD_PDEBUGPC` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PDEBUGPC_SPEC>;
#[doc = "Field `RECORD_PRO_PDEBUGPC` reader - "]
pub type RECORD_PRO_PDEBUGPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugpc(&self) -> RECORD_PRO_PDEBUGPC_R {
        RECORD_PRO_PDEBUGPC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGPC")
            .field("record_pro_pdebugpc", &self.record_pro_pdebugpc())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pdebugpc::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGPC_SPEC {}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGPC to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
