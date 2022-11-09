#[doc = "Register `CORE_0_RCD_PDEBUGPC` reader"]
pub struct R(crate::R<CORE_0_RCD_PDEBUGPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_PDEBUGPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_PDEBUGPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_PDEBUGPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_RCD_PDEBUGPC` reader - reg_core_0_rcd_pdebugpc"]
pub type CORE_0_RCD_PDEBUGPC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_rcd_pdebugpc"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugpc(&self) -> CORE_0_RCD_PDEBUGPC_R {
        CORE_0_RCD_PDEBUGPC_R::new(self.bits)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_pdebugpc](index.html) module"]
pub struct CORE_0_RCD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_pdebugpc::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGPC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGPC to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
