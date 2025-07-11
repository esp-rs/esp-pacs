#[doc = "Register `SRC_PASS_IN_SEC_STATUS_1` reader"]
pub type R = crate::R<SRC_PASS_IN_SEC_STATUS_1_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_SEC_STATUS_1` reader - Represents the PASS_IN_SEC status of the interrupt sources within interrupt-index-range 32 ~ 63. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
pub type INT_SRC_PASS_IN_SEC_STATUS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the PASS_IN_SEC status of the interrupt sources within interrupt-index-range 32 ~ 63. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
    #[inline(always)]
    pub fn int_src_pass_in_sec_status_1(&self) -> INT_SRC_PASS_IN_SEC_STATUS_1_R {
        INT_SRC_PASS_IN_SEC_STATUS_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_PASS_IN_SEC_STATUS_1")
            .field(
                "int_src_pass_in_sec_status_1",
                &self.int_src_pass_in_sec_status_1(),
            )
            .finish()
    }
}
#[doc = "PASS_IN_SEC status register for interrupt sources 32 ~ 63\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_PASS_IN_SEC_STATUS_1_SPEC;
impl crate::RegisterSpec for SRC_PASS_IN_SEC_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_pass_in_sec_status_1::R`](R) reader structure"]
impl crate::Readable for SRC_PASS_IN_SEC_STATUS_1_SPEC {}
#[doc = "`reset()` method sets SRC_PASS_IN_SEC_STATUS_1 to value 0"]
impl crate::Resettable for SRC_PASS_IN_SEC_STATUS_1_SPEC {}
