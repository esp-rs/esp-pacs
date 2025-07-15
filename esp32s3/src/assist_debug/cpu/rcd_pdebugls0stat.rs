#[doc = "Register `RCD_PDEBUGLS0STAT` reader"]
pub type R = crate::R<RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGLS0STAT` reader - core0_pdebug_s0stat"]
pub type CORE_0_RCD_PDEBUGLS0STAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0_pdebug_s0stat"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugls0stat(&self) -> CORE_0_RCD_PDEBUGLS0STAT_R {
        CORE_0_RCD_PDEBUGLS0STAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGLS0STAT")
            .field("core_0_rcd_pdebugls0stat", &self.core_0_rcd_pdebugls0stat())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugls0stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGLS0STAT_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGLS0STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebugls0stat::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGLS0STAT_SPEC {}
#[doc = "`reset()` method sets RCD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for RCD_PDEBUGLS0STAT_SPEC {}
