#[doc = "Register `CORE_0_RCD_PDEBUGSP` reader"]
pub type R = crate::R<CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGSP` reader - reg_core_0_rcd_pdebugsp"]
pub type CORE_0_RCD_PDEBUGSP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_rcd_pdebugsp"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugsp(&self) -> CORE_0_RCD_PDEBUGSP_R {
        CORE_0_RCD_PDEBUGSP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGSP")
            .field(
                "core_0_rcd_pdebugsp",
                &format_args!("{}", self.core_0_rcd_pdebugsp().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_PDEBUGSP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugsp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_PDEBUGSP_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_pdebugsp::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGSP_SPEC {}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGSP to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGSP_SPEC {
    const RESET_VALUE: u32 = 0;
}
