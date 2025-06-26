#[doc = "Register `STATUS_NEXT` reader"]
pub type R = crate::R<STATUS_NEXT_SPEC>;
#[doc = "Field `STATUS_INTERRUPT_NEXT` reader - Interrupt source signal of GPIO0 ~ 31, could be rising edge interrupt, falling edge interrupt, level sensitive interrupt and any edge interrupt."]
pub type STATUS_INTERRUPT_NEXT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt source signal of GPIO0 ~ 31, could be rising edge interrupt, falling edge interrupt, level sensitive interrupt and any edge interrupt."]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> STATUS_INTERRUPT_NEXT_R {
        STATUS_INTERRUPT_NEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT")
            .field("status_interrupt_next", &self.status_interrupt_next())
            .finish()
    }
}
#[doc = "GPIO0 ~ 31 interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_NEXT_SPEC;
impl crate::RegisterSpec for STATUS_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next::R`](R) reader structure"]
impl crate::Readable for STATUS_NEXT_SPEC {}
#[doc = "`reset()` method sets STATUS_NEXT to value 0"]
impl crate::Resettable for STATUS_NEXT_SPEC {}
