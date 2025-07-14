#[doc = "Register `CORE_1_INTR_STATUS%s` reader"]
pub type R = crate::R<CORE_1_INTR_STATUS_SPEC>;
#[doc = "Field `APP_INTR_STATUS_0` reader - "]
pub type APP_INTR_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_intr_status_0(&self) -> APP_INTR_STATUS_0_R {
        APP_INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_INTR_STATUS")
            .field("app_intr_status_0", &self.app_intr_status_0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_INTR_STATUS_SPEC;
impl crate::RegisterSpec for CORE_1_INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_status::R`](R) reader structure"]
impl crate::Readable for CORE_1_INTR_STATUS_SPEC {}
#[doc = "`reset()` method sets CORE_1_INTR_STATUS%s to value 0"]
impl crate::Resettable for CORE_1_INTR_STATUS_SPEC {}
