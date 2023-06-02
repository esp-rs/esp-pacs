#[doc = "Register `CORE_0_RCD_PDEBUGSP` reader"]
pub struct R(crate::R<CORE_0_RCD_PDEBUGSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_PDEBUGSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_PDEBUGSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_PDEBUGSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_RCD_PDEBUGSP` reader - recorded sp"]
pub type CORE_0_RCD_PDEBUGSP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - recorded sp"]
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
        self.read().fmt(f)
    }
}
#[doc = "record status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_pdebugsp](index.html) module"]
pub struct CORE_0_RCD_PDEBUGSP_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_pdebugsp::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGSP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGSP to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
