#[doc = "Register `TRACE_LOCKUP_PRIV_1` reader"]
pub type R = crate::R<TRACE_LOCKUP_PRIV_1_SPEC>;
#[doc = "Field `TRACE_LOCKUP_RECORDING_PRIV_1` reader - Represents the last lockup priv"]
pub type TRACE_LOCKUP_RECORDING_PRIV_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the last lockup priv"]
    #[inline(always)]
    pub fn trace_lockup_recording_priv_1(&self) -> TRACE_LOCKUP_RECORDING_PRIV_1_R {
        TRACE_LOCKUP_RECORDING_PRIV_1_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_LOCKUP_PRIV_1")
            .field(
                "trace_lockup_recording_priv_1",
                &self.trace_lockup_recording_priv_1(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup priv logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_lockup_priv_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_LOCKUP_PRIV_1_SPEC;
impl crate::RegisterSpec for TRACE_LOCKUP_PRIV_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_lockup_priv_1::R`](R) reader structure"]
impl crate::Readable for TRACE_LOCKUP_PRIV_1_SPEC {}
#[doc = "`reset()` method sets TRACE_LOCKUP_PRIV_1 to value 0"]
impl crate::Resettable for TRACE_LOCKUP_PRIV_1_SPEC {}
