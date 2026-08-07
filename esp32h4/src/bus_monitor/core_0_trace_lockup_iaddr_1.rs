#[doc = "Register `CORE_0_TRACE_LOCKUP_IADDR_1` reader"]
pub type R = crate::R<CORE_0_TRACE_LOCKUP_IADDR_1_SPEC>;
#[doc = "Field `CORE_0_TRACE_LOCKUP_RECORDING_IADDR_1` reader - Represents the last lockup iaddr"]
pub type CORE_0_TRACE_LOCKUP_RECORDING_IADDR_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the last lockup iaddr"]
    #[inline(always)]
    pub fn core_0_trace_lockup_recording_iaddr_1(&self) -> CORE_0_TRACE_LOCKUP_RECORDING_IADDR_1_R {
        CORE_0_TRACE_LOCKUP_RECORDING_IADDR_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_TRACE_LOCKUP_IADDR_1")
            .field(
                "core_0_trace_lockup_recording_iaddr_1",
                &self.core_0_trace_lockup_recording_iaddr_1(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup iaddr logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_trace_lockup_iaddr_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_TRACE_LOCKUP_IADDR_1_SPEC;
impl crate::RegisterSpec for CORE_0_TRACE_LOCKUP_IADDR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_trace_lockup_iaddr_1::R`](R) reader structure"]
impl crate::Readable for CORE_0_TRACE_LOCKUP_IADDR_1_SPEC {}
#[doc = "`reset()` method sets CORE_0_TRACE_LOCKUP_IADDR_1 to value 0"]
impl crate::Resettable for CORE_0_TRACE_LOCKUP_IADDR_1_SPEC {}
