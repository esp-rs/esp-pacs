#[doc = "Register `STATUS1_W1TC` writer"]
pub type W = crate::W<STATUS1_W1TC_SPEC>;
#[doc = "Field `STATUS1_W1TC` writer - Configures whether or not to clear the interrupt status register GPIO_STATUS1_INTERRUPT of GPIO32 ~ GPIO32. - Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_INTERRUPT will be cleared. \\item Recommended operation: use this register to clear GPIO_STATUS1_INTERRUPT."]
pub type STATUS1_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear the interrupt status register GPIO_STATUS1_INTERRUPT of GPIO32 ~ GPIO32. - Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_INTERRUPT will be cleared. \\item Recommended operation: use this register to clear GPIO_STATUS1_INTERRUPT."]
    #[inline(always)]
    pub fn status1_w1tc(&mut self) -> STATUS1_W1TC_W<'_, STATUS1_W1TC_SPEC> {
        STATUS1_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status1_w1tc::W`](W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {}
