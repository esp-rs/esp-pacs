#[doc = "Register `CORE_0_SRC_PASS_IN_SEC_STATUS3` reader"]
pub type R = crate::R<CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_SEC_STATUS_3` reader - Represents the PASS_IN_SEC status of the interrupt sources with interrupt-index-range 96 ~ 97. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
pub type INT_SRC_PASS_IN_SEC_STATUS_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the PASS_IN_SEC status of the interrupt sources with interrupt-index-range 96 ~ 97. Each bit corresponds to one interrupt source 0:The corresponding interrupt source is not PASS_IN_SEC. 1:The corresponding interrupt source is PASS_IN_SEC."]
    #[inline(always)]
    pub fn int_src_pass_in_sec_status_3(&self) -> INT_SRC_PASS_IN_SEC_STATUS_3_R {
        INT_SRC_PASS_IN_SEC_STATUS_3_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_SRC_PASS_IN_SEC_STATUS3")
            .field(
                "int_src_pass_in_sec_status_3",
                &self.int_src_pass_in_sec_status_3(),
            )
            .finish()
    }
}
#[doc = "PASS_IN_SEC status register for interrupt sources 96 ~ 97\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_src_pass_in_sec_status3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC;
impl crate::RegisterSpec for CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_src_pass_in_sec_status3::R`](R) reader structure"]
impl crate::Readable for CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC {}
#[doc = "`reset()` method sets CORE_0_SRC_PASS_IN_SEC_STATUS3 to value 0"]
impl crate::Resettable for CORE_0_SRC_PASS_IN_SEC_STATUS3_SPEC {}
