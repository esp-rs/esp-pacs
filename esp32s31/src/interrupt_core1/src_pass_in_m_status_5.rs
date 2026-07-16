#[doc = "Register `SRC_PASS_IN_M_STATUS_5` reader"]
pub type R = crate::R<SRC_PASS_IN_M_STATUS_5_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_M_STATUS_5` reader - "]
pub type INT_SRC_PASS_IN_M_STATUS_5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn int_src_pass_in_m_status_5(&self) -> INT_SRC_PASS_IN_M_STATUS_5_R {
        INT_SRC_PASS_IN_M_STATUS_5_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_PASS_IN_M_STATUS_5")
            .field(
                "int_src_pass_in_m_status_5",
                &self.int_src_pass_in_m_status_5(),
            )
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_m_status_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_PASS_IN_M_STATUS_5_SPEC;
impl crate::RegisterSpec for SRC_PASS_IN_M_STATUS_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_pass_in_m_status_5::R`](R) reader structure"]
impl crate::Readable for SRC_PASS_IN_M_STATUS_5_SPEC {}
#[doc = "`reset()` method sets SRC_PASS_IN_M_STATUS_5 to value 0"]
impl crate::Resettable for SRC_PASS_IN_M_STATUS_5_SPEC {}
