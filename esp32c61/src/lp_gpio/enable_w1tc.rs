#[doc = "Register `ENABLE_W1TC` writer"]
pub type W = crate::W<ENABLE_W1TC_SPEC>;
#[doc = "Field `ENABLE_W1TC` writer - Configures whether or not to clear the output enable register LP_GPIO_ENABLE_REG of LP_GPIO0 ~ LP_GPIO6. \\\\ 0: Not clear\\\\ 1: The corresponding bit in LP_GPIO_ENABLE_REG will be cleared\\\\ Bit0 ~ bit6 are corresponding to LP_GPIO0 ~ LP_GPIO6. Bit7 ~ bit31 are invalid. \\\\ Recommended operation: use this register to clear LP_GPIO_ENABLE_REG.\\\\"]
pub type ENABLE_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures whether or not to clear the output enable register LP_GPIO_ENABLE_REG of LP_GPIO0 ~ LP_GPIO6. \\\\ 0: Not clear\\\\ 1: The corresponding bit in LP_GPIO_ENABLE_REG will be cleared\\\\ Bit0 ~ bit6 are corresponding to LP_GPIO0 ~ LP_GPIO6. Bit7 ~ bit31 are invalid. \\\\ Recommended operation: use this register to clear LP_GPIO_ENABLE_REG.\\\\"]
    #[inline(always)]
    pub fn enable_w1tc(&mut self) -> ENABLE_W1TC_W<'_, ENABLE_W1TC_SPEC> {
        ENABLE_W1TC_W::new(self, 0)
    }
}
#[doc = "LP_GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_W1TC to value 0"]
impl crate::Resettable for ENABLE_W1TC_SPEC {}
