#[doc = "Register `CORE_0_TRACE_LOCKUP_PRIV_0` reader"]
pub type R = crate::R<CORE_0_TRACE_LOCKUP_PRIV_0_SPEC>;
#[doc = "Field `CORE_0_TRACE_LOCKUP_RECORDING_PRIV_0` reader - Represents the latest lockup priv"]
pub type CORE_0_TRACE_LOCKUP_RECORDING_PRIV_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the latest lockup priv"]
    #[inline(always)]
    pub fn core_0_trace_lockup_recording_priv_0(&self) -> CORE_0_TRACE_LOCKUP_RECORDING_PRIV_0_R {
        CORE_0_TRACE_LOCKUP_RECORDING_PRIV_0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_TRACE_LOCKUP_PRIV_0")
            .field(
                "core_0_trace_lockup_recording_priv_0",
                &self.core_0_trace_lockup_recording_priv_0(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup priv logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_trace_lockup_priv_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_TRACE_LOCKUP_PRIV_0_SPEC;
impl crate::RegisterSpec for CORE_0_TRACE_LOCKUP_PRIV_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_trace_lockup_priv_0::R`](R) reader structure"]
impl crate::Readable for CORE_0_TRACE_LOCKUP_PRIV_0_SPEC {}
#[doc = "`reset()` method sets CORE_0_TRACE_LOCKUP_PRIV_0 to value 0"]
impl crate::Resettable for CORE_0_TRACE_LOCKUP_PRIV_0_SPEC {}
