#[doc = "Register `CORE_1_TRACE_LOCKUP_CAUSE_0` reader"]
pub type R = crate::R<CORE_1_TRACE_LOCKUP_CAUSE_0_SPEC>;
#[doc = "Field `CORE_1_TRACE_LOCKUP_RECORDING_CAUSE_0` reader - Represents the latest lockup cause"]
pub type CORE_1_TRACE_LOCKUP_RECORDING_CAUSE_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Represents the latest lockup cause"]
    #[inline(always)]
    pub fn core_1_trace_lockup_recording_cause_0(&self) -> CORE_1_TRACE_LOCKUP_RECORDING_CAUSE_0_R {
        CORE_1_TRACE_LOCKUP_RECORDING_CAUSE_0_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_TRACE_LOCKUP_CAUSE_0")
            .field(
                "core_1_trace_lockup_recording_cause_0",
                &self.core_1_trace_lockup_recording_cause_0(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup cause logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_trace_lockup_cause_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_TRACE_LOCKUP_CAUSE_0_SPEC;
impl crate::RegisterSpec for CORE_1_TRACE_LOCKUP_CAUSE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_trace_lockup_cause_0::R`](R) reader structure"]
impl crate::Readable for CORE_1_TRACE_LOCKUP_CAUSE_0_SPEC {}
#[doc = "`reset()` method sets CORE_1_TRACE_LOCKUP_CAUSE_0 to value 0"]
impl crate::Resettable for CORE_1_TRACE_LOCKUP_CAUSE_0_SPEC {}
