#[doc = "Register `CORE_0_INTR_STATUS%s` reader"]
pub type R = crate::R<CORE_0_INTR_STATUS_SPEC>;
#[doc = "Field `PRO_INTR_STATUS_0` reader - This register stores the status of the first 32 input interrupt sources."]
pub type PRO_INTR_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the status of the first 32 input interrupt sources."]
    #[inline(always)]
    pub fn pro_intr_status_0(&self) -> PRO_INTR_STATUS_0_R {
        PRO_INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_STATUS")
            .field("pro_intr_status_0", &self.pro_intr_status_0())
            .finish()
    }
}
#[doc = "Interrupt status register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_STATUS_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_status::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_STATUS_SPEC {}
#[doc = "`reset()` method sets CORE_0_INTR_STATUS%s to value 0"]
impl crate::Resettable for CORE_0_INTR_STATUS_SPEC {}
