#[doc = "Register `LPCPU_REJECT_CAUSE` reader"]
pub type R = crate::R<LPCPU_REJECT_CAUSE_SPEC>;
#[doc = "Field `LPCPU_REJECT_CAUSE` reader - reject cause for lpcpu"]
pub type LPCPU_REJECT_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - reject cause for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_reject_cause(&self) -> LPCPU_REJECT_CAUSE_R {
        LPCPU_REJECT_CAUSE_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_REJECT_CAUSE")
            .field("lpcpu_reject_cause", &self.lpcpu_reject_cause())
            .finish()
    }
}
#[doc = "reject cause register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_reject_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for LPCPU_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_reject_cause::R`](R) reader structure"]
impl crate::Readable for LPCPU_REJECT_CAUSE_SPEC {}
#[doc = "`reset()` method sets LPCPU_REJECT_CAUSE to value 0"]
impl crate::Resettable for LPCPU_REJECT_CAUSE_SPEC {}
