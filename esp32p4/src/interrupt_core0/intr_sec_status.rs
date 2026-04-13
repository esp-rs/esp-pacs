#[doc = "Register `INTR_SEC_STATUS` reader"]
pub type R = crate::R<INTR_SEC_STATUS_SPEC>;
#[doc = "Field `INTR_SEC_STATUS` reader - NA"]
pub type INTR_SEC_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn intr_sec_status(&self) -> INTR_SEC_STATUS_R {
        INTR_SEC_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_SEC_STATUS")
            .field("intr_sec_status", &self.intr_sec_status())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sec_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SEC_STATUS_SPEC;
impl crate::RegisterSpec for INTR_SEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sec_status::R`](R) reader structure"]
impl crate::Readable for INTR_SEC_STATUS_SPEC {}
#[doc = "`reset()` method sets INTR_SEC_STATUS to value 0"]
impl crate::Resettable for INTR_SEC_STATUS_SPEC {}
