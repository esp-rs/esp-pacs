#[doc = "Register `CORE_0_RCD_PDEBUGLS0DATA` reader"]
pub type R = crate::R<CORE_0_RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGLS0DATA` reader - core0_pdebug_s0data"]
pub type CORE_0_RCD_PDEBUGLS0DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0_pdebug_s0data"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugls0data(&self) -> CORE_0_RCD_PDEBUGLS0DATA_R {
        CORE_0_RCD_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGLS0DATA")
            .field("core_0_rcd_pdebugls0data", &self.core_0_rcd_pdebugls0data())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_pdebugls0data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_pdebugls0data::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGLS0DATA_SPEC {}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGLS0DATA to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGLS0DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
