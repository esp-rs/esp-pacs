#[doc = "Register `INTR_SEC_STATUS` reader"]
pub type R = crate::R<INTR_SEC_STATUS_SPEC>;
#[doc = "Field `CORE0_INTR_SEC_STATUS` reader - "]
pub type CORE0_INTR_SEC_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core0_intr_sec_status(&self) -> CORE0_INTR_SEC_STATUS_R {
        CORE0_INTR_SEC_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_SEC_STATUS")
            .field("core0_intr_sec_status", &self.core0_intr_sec_status())
            .finish()
    }
}
#[doc = "Interrupt security status\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sec_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SEC_STATUS_SPEC;
impl crate::RegisterSpec for INTR_SEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sec_status::R`](R) reader structure"]
impl crate::Readable for INTR_SEC_STATUS_SPEC {}
#[doc = "`reset()` method sets INTR_SEC_STATUS to value 0"]
impl crate::Resettable for INTR_SEC_STATUS_SPEC {}
