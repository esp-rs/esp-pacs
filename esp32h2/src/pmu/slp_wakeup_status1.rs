#[doc = "Register `SLP_WAKEUP_STATUS1` reader"]
pub type R = crate::R<SLP_WAKEUP_STATUS1_SPEC>;
#[doc = "Field `REJECT_CAUSE` reader - need_des"]
pub type REJECT_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_STATUS1")
            .field(
                "reject_cause",
                &format_args!("{}", self.reject_cause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_STATUS1_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_status1::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_STATUS1_SPEC {}
#[doc = "`reset()` method sets SLP_WAKEUP_STATUS1 to value 0"]
impl crate::Resettable for SLP_WAKEUP_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
