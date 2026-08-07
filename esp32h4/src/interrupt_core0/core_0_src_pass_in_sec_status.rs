#[doc = "Register `CORE_0_SRC_PASS_IN_SEC_STATUS%s` reader"]
pub type R = crate::R<CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_SEC_STATUS_0` reader - Represents the PASS_IN_SEC status of the interrupt sources within interrupt-index-range 0 ~ 31. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
pub type INT_SRC_PASS_IN_SEC_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the PASS_IN_SEC status of the interrupt sources within interrupt-index-range 0 ~ 31. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
    #[inline(always)]
    pub fn int_src_pass_in_sec_status_0(&self) -> INT_SRC_PASS_IN_SEC_STATUS_0_R {
        INT_SRC_PASS_IN_SEC_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_SRC_PASS_IN_SEC_STATUS")
            .field(
                "int_src_pass_in_sec_status_0",
                &self.int_src_pass_in_sec_status_0(),
            )
            .finish()
    }
}
#[doc = "PASS_IN_SEC status for interrupt sources\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_src_pass_in_sec_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC;
impl crate::RegisterSpec for CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_src_pass_in_sec_status::R`](R) reader structure"]
impl crate::Readable for CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC {}
#[doc = "`reset()` method sets CORE_0_SRC_PASS_IN_SEC_STATUS%s to value 0"]
impl crate::Resettable for CORE_0_SRC_PASS_IN_SEC_STATUS_SPEC {}
