#[doc = "Register `ENABLE1_W1TS` writer"]
pub type W = crate::W<ENABLE1_W1TS_SPEC>;
#[doc = "Field `ENABLE1_W1TS` writer - GPIO32 ~ 53 output enable set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be set to 1. Recommended operation: use this register to set GPIO_ENABLE1_REG."]
pub type ENABLE1_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output enable set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be set to 1. Recommended operation: use this register to set GPIO_ENABLE1_REG."]
    #[inline(always)]
    #[must_use]
    pub fn enable1_w1ts(&mut self) -> ENABLE1_W1TS_W<ENABLE1_W1TS_SPEC> {
        ENABLE1_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO32 ~ 53 output enable bit set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable1_w1ts::W`](W) writer structure"]
impl crate::Writable for ENABLE1_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE1_W1TS to value 0"]
impl crate::Resettable for ENABLE1_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
