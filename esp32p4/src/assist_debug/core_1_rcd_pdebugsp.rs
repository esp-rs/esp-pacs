#[doc = "Register `CORE_1_RCD_PDEBUGSP` reader"]
pub type R = crate::R<CORE_1_RCD_PDEBUGSP_SPEC>;
#[doc = "Field `CORE_1_RCD_PDEBUGSP` reader - recorded sp"]
pub type CORE_1_RCD_PDEBUGSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - recorded sp"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugsp(&self) -> CORE_1_RCD_PDEBUGSP_R {
        CORE_1_RCD_PDEBUGSP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGSP")
            .field("core_1_rcd_pdebugsp", &self.core_1_rcd_pdebugsp())
            .finish()
    }
}
#[doc = "record status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_rcd_pdebugsp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_RCD_PDEBUGSP_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_rcd_pdebugsp::R`](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGSP_SPEC {}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGSP to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGSP_SPEC {
    const RESET_VALUE: u32 = 0;
}
