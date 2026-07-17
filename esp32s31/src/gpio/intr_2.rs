#[doc = "Register `INTR_2` reader"]
pub type R = crate::R<INTR_2_SPEC>;
#[doc = "Field `INT_2` reader - Represents the GPIO_INT_2 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_2 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_2 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the GPIO_INT_2 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_2 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_2 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_2(&self) -> INT_2_R {
        INT_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_2")
            .field("int_2", &self.int_2())
            .finish()
    }
}
#[doc = "GPIO_INT_2 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_2_SPEC;
impl crate::RegisterSpec for INTR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_2::R`](R) reader structure"]
impl crate::Readable for INTR_2_SPEC {}
#[doc = "`reset()` method sets INTR_2 to value 0"]
impl crate::Resettable for INTR_2_SPEC {}
