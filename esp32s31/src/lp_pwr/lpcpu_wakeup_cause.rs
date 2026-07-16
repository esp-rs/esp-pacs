#[doc = "Register `LPCPU_WAKEUP_CAUSE` reader"]
pub type R = crate::R<LPCPU_WAKEUP_CAUSE_SPEC>;
#[doc = "Field `LPCPU_WAKEUP_CAUSE` reader - wakeup cause for lpcpu"]
pub type LPCPU_WAKEUP_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup cause for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_wakeup_cause(&self) -> LPCPU_WAKEUP_CAUSE_R {
        LPCPU_WAKEUP_CAUSE_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_WAKEUP_CAUSE")
            .field("lpcpu_wakeup_cause", &self.lpcpu_wakeup_cause())
            .finish()
    }
}
#[doc = "wakeup cause register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_wakeup_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_WAKEUP_CAUSE_SPEC;
impl crate::RegisterSpec for LPCPU_WAKEUP_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_wakeup_cause::R`](R) reader structure"]
impl crate::Readable for LPCPU_WAKEUP_CAUSE_SPEC {}
#[doc = "`reset()` method sets LPCPU_WAKEUP_CAUSE to value 0"]
impl crate::Resettable for LPCPU_WAKEUP_CAUSE_SPEC {}
