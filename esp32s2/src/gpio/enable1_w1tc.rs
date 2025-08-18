#[doc = "Register `ENABLE1_W1TC` writer"]
pub type W = crate::W<ENABLE1_W1TC_SPEC>;
#[doc = "Field `ENABLE1_W1TC` writer - GPIO32 ~ 53 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be cleared. Recommended operation: use this register to clear GPIO_ENABLE1_REG."]
pub type ENABLE1_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_ENABLE1_REG will be cleared. Recommended operation: use this register to clear GPIO_ENABLE1_REG."]
    #[inline(always)]
    pub fn enable1_w1tc(&mut self) -> ENABLE1_W1TC_W<'_, ENABLE1_W1TC_SPEC> {
        ENABLE1_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO32 ~ 53 output enable bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable1_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE1_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE1_W1TC to value 0"]
impl crate::Resettable for ENABLE1_W1TC_SPEC {}
