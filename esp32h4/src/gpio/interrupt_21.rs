#[doc = "Register `INTERRUPT_21` reader"]
pub type R = crate::R<INTERRUPT_21_SPEC>;
#[doc = "Field `INTERRUPT_21` reader - Represents the GPIO_INTERRUPT_2 interrupt status of GPIO32 ~ GPIO43. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INTERRUPT_2 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INTERRUPT_2 interrupt is enabled.\\\\ Bit32 ~ bit43 are corresponding to GPIO32 ~ GPIO43. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INTERRUPT_21_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Represents the GPIO_INTERRUPT_2 interrupt status of GPIO32 ~ GPIO43. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_INTERRUPT_2 interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_INTERRUPT_2 interrupt is enabled.\\\\ Bit32 ~ bit43 are corresponding to GPIO32 ~ GPIO43. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn interrupt_21(&self) -> INTERRUPT_21_R {
        INTERRUPT_21_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_21")
            .field("interrupt_21", &self.interrupt_21())
            .finish()
    }
}
#[doc = "GPIO_INTERRUPT_2 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_21_SPEC;
impl crate::RegisterSpec for INTERRUPT_21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_21::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_21_SPEC {}
#[doc = "`reset()` method sets INTERRUPT_21 to value 0"]
impl crate::Resettable for INTERRUPT_21_SPEC {}
