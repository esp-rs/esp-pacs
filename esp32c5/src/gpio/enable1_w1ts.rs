#[doc = "Register `ENABLE1_W1TS` writer"]
pub type W = crate::W<ENABLE1_W1TS_SPEC>;
#[doc = "Field `ENABLE1_W1TS` writer - Configures whether or not to set the output enable register GPIO_ENABLE_REG of GPIO32 ~ GPIO32.\\\\ 0: Not set\\\\ 1: The corresponding bit in GPIO_ENABLE_REG will be set to 1\\\\ Recommended operation: use this register to set GPIO_ENABLE1_REG.\\\\"]
pub type ENABLE1_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to set the output enable register GPIO_ENABLE_REG of GPIO32 ~ GPIO32.\\\\ 0: Not set\\\\ 1: The corresponding bit in GPIO_ENABLE_REG will be set to 1\\\\ Recommended operation: use this register to set GPIO_ENABLE1_REG.\\\\"]
    #[inline(always)]
    pub fn enable1_w1ts(&mut self) -> ENABLE1_W1TS_W<'_, ENABLE1_W1TS_SPEC> {
        ENABLE1_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO output enable set register for GPIO32-32\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable1_w1ts::W`](W) writer structure"]
impl crate::Writable for ENABLE1_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE1_W1TS to value 0"]
impl crate::Resettable for ENABLE1_W1TS_SPEC {}
