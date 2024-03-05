#[doc = "Register `CLEAR_IRQ` writer"]
pub type W = crate::W<CLEAR_IRQ_SPEC>;
#[doc = "Field `CLEAR_INTERRUPT` writer - Clear sha interrupt."]
pub type CLEAR_INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLEAR_IRQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sha interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clear_interrupt(&mut self) -> CLEAR_INTERRUPT_W<CLEAR_IRQ_SPEC> {
        CLEAR_INTERRUPT_W::new(self, 0)
    }
}
#[doc = "Interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_irq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAR_IRQ_SPEC;
impl crate::RegisterSpec for CLEAR_IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear_irq::W`](W) writer structure"]
impl crate::Writable for CLEAR_IRQ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEAR_IRQ to value 0"]
impl crate::Resettable for CLEAR_IRQ_SPEC {
    const RESET_VALUE: u32 = 0;
}
