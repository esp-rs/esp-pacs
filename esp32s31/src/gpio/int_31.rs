#[doc = "Register `INT_31` reader"]
pub type R = crate::R<INT_31_SPEC>;
#[doc = "Field `INT_31` reader - Represents the GPIO_INT_3 interrupt status of GPIO32 ~ GPIO63. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_3 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_3 interrupt is enabled.\\\\ Bit32 ~ bit63 are corresponding to GPIO32 ~ GPIO63. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_31_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the GPIO_INT_3 interrupt status of GPIO32 ~ GPIO63. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_3 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_3 interrupt is enabled.\\\\ Bit32 ~ bit63 are corresponding to GPIO32 ~ GPIO63. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_31(&self) -> INT_31_R {
        INT_31_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_31")
            .field("int_31", &self.int_31())
            .finish()
    }
}
#[doc = "GPIO_INT_3 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_31_SPEC;
impl crate::RegisterSpec for INT_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_31::R`](R) reader structure"]
impl crate::Readable for INT_31_SPEC {}
#[doc = "`reset()` method sets INT_31 to value 0"]
impl crate::Resettable for INT_31_SPEC {}
