#[doc = "Register `INTR_0` reader"]
pub type R = crate::R<INTR_0_SPEC>;
#[doc = "Field `INT_0` reader - Represents the GPIO_INT_0 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_0 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_0 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the GPIO_INT_0 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_0 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_0 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_0(&self) -> INT_0_R {
        INT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_0")
            .field("int_0", &self.int_0())
            .finish()
    }
}
#[doc = "GPIO_INT_0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_0_SPEC;
impl crate::RegisterSpec for INTR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_0::R`](R) reader structure"]
impl crate::Readable for INTR_0_SPEC {}
#[doc = "`reset()` method sets INTR_0 to value 0"]
impl crate::Resettable for INTR_0_SPEC {}
