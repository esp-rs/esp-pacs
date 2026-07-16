#[doc = "Register `SRC_PASS_IN_S_STATUS%s` reader"]
pub type R = crate::R<SRC_PASS_IN_S_STATUS_SPEC>;
#[doc = "Field `INT_SRC_PASS_IN_S_STATUS_0` reader - "]
pub type INT_SRC_PASS_IN_S_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_src_pass_in_s_status_0(&self) -> INT_SRC_PASS_IN_S_STATUS_0_R {
        INT_SRC_PASS_IN_S_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_PASS_IN_S_STATUS")
            .field(
                "int_src_pass_in_s_status_0",
                &self.int_src_pass_in_s_status_0(),
            )
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`src_pass_in_s_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_PASS_IN_S_STATUS_SPEC;
impl crate::RegisterSpec for SRC_PASS_IN_S_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_pass_in_s_status::R`](R) reader structure"]
impl crate::Readable for SRC_PASS_IN_S_STATUS_SPEC {}
#[doc = "`reset()` method sets SRC_PASS_IN_S_STATUS%s to value 0"]
impl crate::Resettable for SRC_PASS_IN_S_STATUS_SPEC {}
