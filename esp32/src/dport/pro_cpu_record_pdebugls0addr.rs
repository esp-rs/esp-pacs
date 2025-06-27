#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0ADDR` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC>;
#[doc = "Field `RECORD_PRO_PDEBUGLS0ADDR` reader - "]
pub type RECORD_PRO_PDEBUGLS0ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugls0addr(&self) -> RECORD_PRO_PDEBUGLS0ADDR_R {
        RECORD_PRO_PDEBUGLS0ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGLS0ADDR")
            .field("record_pro_pdebugls0addr", &self.record_pro_pdebugls0addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cpu_record_pdebugls0addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pdebugls0addr::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC {}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGLS0ADDR to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGLS0ADDR_SPEC {}
