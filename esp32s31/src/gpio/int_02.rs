#[doc = "Register `INT_02` reader"]
pub type R = crate::R<INT_02_SPEC>;
#[doc = "Field `INT_02` reader - Represents the GPIO_INT_0 interrupt status of GPIO64 ~ GPIO66. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_0 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_0 interrupt is enabled.\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS2_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_02_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Represents the GPIO_INT_0 interrupt status of GPIO64 ~ GPIO66. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_0 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_0 interrupt is enabled.\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS2_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_02(&self) -> INT_02_R {
        INT_02_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_02")
            .field("int_02", &self.int_02())
            .finish()
    }
}
#[doc = "GPIO_INT_0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_02::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_02_SPEC;
impl crate::RegisterSpec for INT_02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_02::R`](R) reader structure"]
impl crate::Readable for INT_02_SPEC {}
#[doc = "`reset()` method sets INT_02 to value 0"]
impl crate::Resettable for INT_02_SPEC {}
