#[doc = "Register `CORE_0_TRACE_LOCKUP_PRIV_1` reader"]
pub type R = crate::R<CORE_0_TRACE_LOCKUP_PRIV_1_SPEC>;
#[doc = "Field `CORE_0_TRACE_LOCKUP_RECORDING_PRIV_1` reader - Represents the last lockup priv"]
pub type CORE_0_TRACE_LOCKUP_RECORDING_PRIV_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the last lockup priv"]
    #[inline(always)]
    pub fn core_0_trace_lockup_recording_priv_1(&self) -> CORE_0_TRACE_LOCKUP_RECORDING_PRIV_1_R {
        CORE_0_TRACE_LOCKUP_RECORDING_PRIV_1_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_TRACE_LOCKUP_PRIV_1")
            .field(
                "core_0_trace_lockup_recording_priv_1",
                &self.core_0_trace_lockup_recording_priv_1(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup priv logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_trace_lockup_priv_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_TRACE_LOCKUP_PRIV_1_SPEC;
impl crate::RegisterSpec for CORE_0_TRACE_LOCKUP_PRIV_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_trace_lockup_priv_1::R`](R) reader structure"]
impl crate::Readable for CORE_0_TRACE_LOCKUP_PRIV_1_SPEC {}
#[doc = "`reset()` method sets CORE_0_TRACE_LOCKUP_PRIV_1 to value 0"]
impl crate::Resettable for CORE_0_TRACE_LOCKUP_PRIV_1_SPEC {}
