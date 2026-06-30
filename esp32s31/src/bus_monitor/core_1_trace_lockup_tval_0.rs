#[doc = "Register `CORE_1_TRACE_LOCKUP_TVAL_0` reader"]
pub type R = crate::R<CORE_1_TRACE_LOCKUP_TVAL_0_SPEC>;
#[doc = "Field `CORE_1_TRACE_LOCKUP_RECORDING_TVAL_0` reader - Represents the latest lockup tval"]
pub type CORE_1_TRACE_LOCKUP_RECORDING_TVAL_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the latest lockup tval"]
    #[inline(always)]
    pub fn core_1_trace_lockup_recording_tval_0(&self) -> CORE_1_TRACE_LOCKUP_RECORDING_TVAL_0_R {
        CORE_1_TRACE_LOCKUP_RECORDING_TVAL_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_TRACE_LOCKUP_TVAL_0")
            .field(
                "core_1_trace_lockup_recording_tval_0",
                &self.core_1_trace_lockup_recording_tval_0(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup tval logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_trace_lockup_tval_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_TRACE_LOCKUP_TVAL_0_SPEC;
impl crate::RegisterSpec for CORE_1_TRACE_LOCKUP_TVAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_trace_lockup_tval_0::R`](R) reader structure"]
impl crate::Readable for CORE_1_TRACE_LOCKUP_TVAL_0_SPEC {}
#[doc = "`reset()` method sets CORE_1_TRACE_LOCKUP_TVAL_0 to value 0"]
impl crate::Resettable for CORE_1_TRACE_LOCKUP_TVAL_0_SPEC {}
