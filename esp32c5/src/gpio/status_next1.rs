#[doc = "Register `STATUS_NEXT1` reader"]
pub type R = crate::R<STATUS_NEXT1_SPEC>;
#[doc = "Field `STATUS_INTERRUPT_NEXT1` reader - Represents the interrupt source signal of GPIO32 ~ GPIO32.\\\\ Bit0 ~ bit24 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. Each bit represents:\\\\ 0: The GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: The GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ The interrupt could be rising edge interrupt, falling edge interrupt, level sensitive interrupt and any edge interrupt.\\\\"]
pub type STATUS_INTERRUPT_NEXT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the interrupt source signal of GPIO32 ~ GPIO32.\\\\ Bit0 ~ bit24 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. Each bit represents:\\\\ 0: The GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: The GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ The interrupt could be rising edge interrupt, falling edge interrupt, level sensitive interrupt and any edge interrupt.\\\\"]
    #[inline(always)]
    pub fn status_interrupt_next1(&self) -> STATUS_INTERRUPT_NEXT1_R {
        STATUS_INTERRUPT_NEXT1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT1")
            .field("status_interrupt_next1", &self.status_interrupt_next1())
            .finish()
    }
}
#[doc = "GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_NEXT1_SPEC;
impl crate::RegisterSpec for STATUS_NEXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next1::R`](R) reader structure"]
impl crate::Readable for STATUS_NEXT1_SPEC {}
#[doc = "`reset()` method sets STATUS_NEXT1 to value 0"]
impl crate::Resettable for STATUS_NEXT1_SPEC {}
