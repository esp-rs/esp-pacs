#[doc = "Register `CORE_1_RCD_PDEBUGLS0ADDR` reader"]
pub type R = crate::R<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "Field `CORE_1_RCD_PDEBUGLS0ADDR` reader - Core1_pdebug_s0addr"]
pub type CORE_1_RCD_PDEBUGLS0ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Core1_pdebug_s0addr"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugls0addr(&self) -> CORE_1_RCD_PDEBUGLS0ADDR_R {
        CORE_1_RCD_PDEBUGLS0ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGLS0ADDR")
            .field("core_1_rcd_pdebugls0addr", &self.core_1_rcd_pdebugls0addr())
            .finish()
    }
}
#[doc = "Core1 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_rcd_pdebugls0addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_RCD_PDEBUGLS0ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_rcd_pdebugls0addr::R`](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGLS0ADDR to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {}
