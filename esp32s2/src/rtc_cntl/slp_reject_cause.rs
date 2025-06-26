#[doc = "Register `SLP_REJECT_CAUSE` reader"]
pub type R = crate::R<SLP_REJECT_CAUSE_SPEC>;
#[doc = "Field `REJECT_CAUSE` reader - Stores the reject-to-sleep cause."]
pub type REJECT_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Stores the reject-to-sleep cause."]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CAUSE")
            .field("reject_cause", &self.reject_cause())
            .finish()
    }
}
#[doc = "Stores the reject-to-sleep cause.\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_reject_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_reject_cause::R`](R) reader structure"]
impl crate::Readable for SLP_REJECT_CAUSE_SPEC {}
#[doc = "`reset()` method sets SLP_REJECT_CAUSE to value 0"]
impl crate::Resettable for SLP_REJECT_CAUSE_SPEC {}
