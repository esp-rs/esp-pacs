#[doc = "Register `INT_1` reader"]
pub type R = crate::R<INT_1_SPEC>;
#[doc = "Field `INT_1` reader - Represents the GPIO_INT_1 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_1 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_1 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the GPIO_INT_1 interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_1 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_1 interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_1(&self) -> INT_1_R {
        INT_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_1")
            .field("int_1", &self.int_1())
            .finish()
    }
}
#[doc = "GPIO_INT_1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_1_SPEC;
impl crate::RegisterSpec for INT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_1::R`](R) reader structure"]
impl crate::Readable for INT_1_SPEC {}
#[doc = "`reset()` method sets INT_1 to value 0"]
impl crate::Resettable for INT_1_SPEC {}
