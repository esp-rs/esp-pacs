#[doc = "Register `INTR_STATUS_REG_4` reader"]
pub type R = crate::R<INTR_STATUS_REG_4_SPEC>;
#[doc = "Field `CORE1_INTR_STATUS_4` reader - NA"]
pub type CORE1_INTR_STATUS_4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core1_intr_status_4(&self) -> CORE1_INTR_STATUS_4_R {
        CORE1_INTR_STATUS_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_4")
            .field("core1_intr_status_4", &self.core1_intr_status_4())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_reg_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_STATUS_REG_4_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_reg_4::R`](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_4_SPEC {}
#[doc = "`reset()` method sets INTR_STATUS_REG_4 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_4_SPEC {}
