#[doc = "Register `CORE_0_RCD_PDEBUGLS0STAT` reader"]
pub struct R(crate::R<CORE_0_RCD_PDEBUGLS0STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_PDEBUGLS0STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_PDEBUGLS0STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_PDEBUGLS0STAT_SPEC>) -> Self {
        R(reader)
    }
}
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
        f.debug_struct("CORE_0_RCD_PDEBUGLS0STAT")
            .field(
                "core_0_rcd_pdebugls0stat",
                &format_args!("{}", self.core_0_rcd_pdebugls0stat().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_PDEBUGLS0STAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "core0 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_pdebugls0stat](index.html) module"]
pub struct CORE_0_RCD_PDEBUGLS0STAT_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGLS0STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_pdebugls0stat::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGLS0STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGLS0STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
