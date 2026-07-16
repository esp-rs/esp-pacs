#[doc = "Register `TRACE_LOCKUP_PRIV_0` reader"]
pub type R = crate::R<TRACE_LOCKUP_PRIV_0_SPEC>;
#[doc = "Field `TRACE_LOCKUP_RECORDING_PRIV_0` reader - Represents the latest lockup priv"]
pub type TRACE_LOCKUP_RECORDING_PRIV_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the latest lockup priv"]
    #[inline(always)]
    pub fn trace_lockup_recording_priv_0(&self) -> TRACE_LOCKUP_RECORDING_PRIV_0_R {
        TRACE_LOCKUP_RECORDING_PRIV_0_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_LOCKUP_PRIV_0")
            .field(
                "trace_lockup_recording_priv_0",
                &self.trace_lockup_recording_priv_0(),
            )
            .finish()
    }
}
#[doc = "TRACE lockup priv logging register\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_lockup_priv_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_LOCKUP_PRIV_0_SPEC;
impl crate::RegisterSpec for TRACE_LOCKUP_PRIV_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_lockup_priv_0::R`](R) reader structure"]
impl crate::Readable for TRACE_LOCKUP_PRIV_0_SPEC {}
#[doc = "`reset()` method sets TRACE_LOCKUP_PRIV_0 to value 0"]
impl crate::Resettable for TRACE_LOCKUP_PRIV_0_SPEC {}
