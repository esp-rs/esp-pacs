#[doc = "Register `PROCPU_INT` reader"]
pub type R = crate::R<PROCPU_INT_SPEC>;
#[doc = "Field `PROCPU_INT` reader - Represents the CPU interrupt status of GPIO0 ~ GPIO31. Each bit represents:\\\\ 0: Represents CPU interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the CPU interrupt is enabled.\\\\ This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type PROCPU_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the CPU interrupt status of GPIO0 ~ GPIO31. Each bit represents:\\\\ 0: Represents CPU interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the CPU interrupt is enabled.\\\\ This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn procpu_int(&self) -> PROCPU_INT_R {
        PROCPU_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROCPU_INT")
            .field("procpu_int", &self.procpu_int())
            .finish()
    }
}
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCPU_INT_SPEC;
impl crate::RegisterSpec for PROCPU_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procpu_int::R`](R) reader structure"]
impl crate::Readable for PROCPU_INT_SPEC {}
#[doc = "`reset()` method sets PROCPU_INT to value 0"]
impl crate::Resettable for PROCPU_INT_SPEC {}
