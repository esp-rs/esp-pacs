#[doc = "Register `INTR_SRC_PASS_IN_SEC_STATUS_2` reader"]
pub type R = crate::R<INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC>;
#[doc = "Field `CORE1_INTR_SRC_PASS_IN_SEC_STATUS_2` reader - NA"]
pub type CORE1_INTR_SRC_PASS_IN_SEC_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core1_intr_src_pass_in_sec_status_2(&self) -> CORE1_INTR_SRC_PASS_IN_SEC_STATUS_2_R {
        CORE1_INTR_SRC_PASS_IN_SEC_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_SRC_PASS_IN_SEC_STATUS_2")
            .field(
                "core1_intr_src_pass_in_sec_status_2",
                &self.core1_intr_src_pass_in_sec_status_2(),
            )
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_src_pass_in_sec_status_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC;
impl crate::RegisterSpec for INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_src_pass_in_sec_status_2::R`](R) reader structure"]
impl crate::Readable for INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC {}
#[doc = "`reset()` method sets INTR_SRC_PASS_IN_SEC_STATUS_2 to value 0"]
impl crate::Resettable for INTR_SRC_PASS_IN_SEC_STATUS_2_SPEC {}
