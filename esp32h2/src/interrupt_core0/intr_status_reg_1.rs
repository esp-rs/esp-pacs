#[doc = "Register `INTR_STATUS_REG_1` reader"]
pub type R = crate::R<INTR_STATUS_REG_1_SPEC>;
#[doc = "Field `INTR_STATUS_1` reader - Status register for interrupt sources 32~63 mapping register"]
pub type INTR_STATUS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status register for interrupt sources 32~63 mapping register"]
    #[inline(always)]
    pub fn intr_status_1(&self) -> INTR_STATUS_1_R {
        INTR_STATUS_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_1")
            .field("intr_status_1", &self.intr_status_1())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_status_reg_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_STATUS_REG_1_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_reg_1::R`](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_1_SPEC {}
#[doc = "`reset()` method sets INTR_STATUS_REG_1 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_1_SPEC {}
