#[doc = "Register `INTR_SRC_PASS_IN_SEC_STATUS_%s` reader"]
pub type R = crate::R<INTR_SRC_PASS_IN_SEC_STATUS__SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_SRC_PASS_IN_SEC_STATUS_")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "Interrupt source pass-in-secure status word %s\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SRC_PASS_IN_SEC_STATUS__SPEC;
impl crate::RegisterSpec for INTR_SRC_PASS_IN_SEC_STATUS__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_src_pass_in_sec_status_::R`](R) reader structure"]
impl crate::Readable for INTR_SRC_PASS_IN_SEC_STATUS__SPEC {}
#[doc = "`reset()` method sets INTR_SRC_PASS_IN_SEC_STATUS_%s to value 0"]
impl crate::Resettable for INTR_SRC_PASS_IN_SEC_STATUS__SPEC {}
