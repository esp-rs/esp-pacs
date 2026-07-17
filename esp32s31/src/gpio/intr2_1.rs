#[doc = "Register `INTR2_1` reader"]
pub type R = crate::R<INTR2_1_SPEC>;
#[doc = "Field `INT_12` reader - Represents the GPIO_INT_1 interrupt status of GPIO64 ~ GPIO66. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_1 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_1 interrupt is enabled.\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS2_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INT_12_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Represents the GPIO_INT_1 interrupt status of GPIO64 ~ GPIO66. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INT_1 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INT_1 interrupt is enabled.\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS2_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn int_12(&self) -> INT_12_R {
        INT_12_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR2_1")
            .field("int_12", &self.int_12())
            .finish()
    }
}
#[doc = "GPIO_INT_1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr2_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR2_1_SPEC;
impl crate::RegisterSpec for INTR2_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr2_1::R`](R) reader structure"]
impl crate::Readable for INTR2_1_SPEC {}
#[doc = "`reset()` method sets INTR2_1 to value 0"]
impl crate::Resettable for INTR2_1_SPEC {}
