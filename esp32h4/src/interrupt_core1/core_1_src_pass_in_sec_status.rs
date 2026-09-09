#[doc = "Register `CORE_1_SRC_PASS_IN_SEC_STATUS%s` reader"]
pub type R = crate::R<CORE_1_SRC_PASS_IN_SEC_STATUS_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_SEC_STATUS` reader - "]
pub type INT_SRC_PASS_IN_SEC_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_src_pass_in_sec_status(&self) -> INT_SRC_PASS_IN_SEC_STATUS_R {
        INT_SRC_PASS_IN_SEC_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_SRC_PASS_IN_SEC_STATUS")
            .field(
                "int_src_pass_in_sec_status",
                &self.int_src_pass_in_sec_status(),
            )
            .finish()
    }
}
#[doc = "PASS_IN_SEC status for interrupt sources\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_src_pass_in_sec_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_SRC_PASS_IN_SEC_STATUS_SPEC;
impl crate::RegisterSpec for CORE_1_SRC_PASS_IN_SEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_src_pass_in_sec_status::R`](R) reader structure"]
impl crate::Readable for CORE_1_SRC_PASS_IN_SEC_STATUS_SPEC {}
#[doc = "`reset()` method sets CORE_1_SRC_PASS_IN_SEC_STATUS%s to value 0"]
impl crate::Resettable for CORE_1_SRC_PASS_IN_SEC_STATUS_SPEC {}
