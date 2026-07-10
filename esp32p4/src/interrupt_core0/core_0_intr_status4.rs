#[doc = "Register `CORE_0_INTR_STATUS4` reader"]
pub type R = crate::R<CORE_0_INTR_STATUS4_SPEC>;
#[doc = "Field `CORE0_INTR_STATUS_4` reader - NA"]
pub type CORE0_INTR_STATUS_4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_status_4(&self) -> CORE0_INTR_STATUS_4_R {
        CORE0_INTR_STATUS_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_STATUS4")
            .field("core0_intr_status_4", &self.core0_intr_status_4())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_STATUS4_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_STATUS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_status4::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_STATUS4_SPEC {}
#[doc = "`reset()` method sets CORE_0_INTR_STATUS4 to value 0"]
impl crate::Resettable for CORE_0_INTR_STATUS4_SPEC {}
