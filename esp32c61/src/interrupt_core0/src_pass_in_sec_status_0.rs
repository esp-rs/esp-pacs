#[doc = "Register `SRC_PASS_IN_SEC_STATUS_0` reader"]
pub type R = crate::R<SRC_PASS_IN_SEC_STATUS_0_SPEC>;
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
        f.debug_struct("SRC_PASS_IN_SEC_STATUS_0")
            .field(
                "int_src_pass_in_sec_status_0",
                &self.int_src_pass_in_sec_status_0(),
            )
            .finish()
    }
}
#[doc = "PASS_IN_SEC status register for interrupt sources 0 ~ 31\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_sec_status_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_PASS_IN_SEC_STATUS_0_SPEC;
impl crate::RegisterSpec for SRC_PASS_IN_SEC_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_pass_in_sec_status_0::R`](R) reader structure"]
impl crate::Readable for SRC_PASS_IN_SEC_STATUS_0_SPEC {}
#[doc = "`reset()` method sets SRC_PASS_IN_SEC_STATUS_0 to value 0"]
impl crate::Resettable for SRC_PASS_IN_SEC_STATUS_0_SPEC {}
