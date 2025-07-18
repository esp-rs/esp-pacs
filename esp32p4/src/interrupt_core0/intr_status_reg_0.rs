#[doc = "Register `INTR_STATUS_REG_0` reader"]
pub type R = crate::R<INTR_STATUS_REG_0_SPEC>;
#[doc = "Field `CORE0_INTR_STATUS_0` reader - NA"]
pub type CORE0_INTR_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_status_0(&self) -> CORE0_INTR_STATUS_0_R {
        CORE0_INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_0")
            .field("core0_intr_status_0", &self.core0_intr_status_0())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_reg_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_STATUS_REG_0_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_reg_0::R`](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_0_SPEC {}
#[doc = "`reset()` method sets INTR_STATUS_REG_0 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_0_SPEC {}
