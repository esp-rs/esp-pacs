#[doc = "Register `STATUS_W1TC` writer"]
pub type W = crate::W<STATUS_W1TC_SPEC>;
#[doc = "Field `STATUS_W1TC` writer - Configures whether or not to clear the interrupt status register GPIO_STATUS_INTERRUPT of GPIO0 ~ GPIO31. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS_INTERRUPT will be cleared. \\item Recommended operation: use this register to clear GPIO_STATUS_INTERRUPT."]
pub type STATUS_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures whether or not to clear the interrupt status register GPIO_STATUS_INTERRUPT of GPIO0 ~ GPIO31. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS_INTERRUPT will be cleared. \\item Recommended operation: use this register to clear GPIO_STATUS_INTERRUPT."]
    #[inline(always)]
    pub fn status_w1tc(&mut self) -> STATUS_W1TC_W<STATUS_W1TC_SPEC> {
        STATUS_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TC_SPEC;
impl crate::RegisterSpec for STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1tc::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_W1TC to value 0"]
impl crate::Resettable for STATUS_W1TC_SPEC {}
