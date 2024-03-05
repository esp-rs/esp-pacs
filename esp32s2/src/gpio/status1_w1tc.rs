#[doc = "Register `STATUS1_W1TC` writer"]
pub type W = crate::W<STATUS1_W1TC_SPEC>;
#[doc = "Field `STATUS1_W1TC` writer - GPIO32 ~ 53 interrupt status clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be cleared. Recommended operation: use this register to clear GPIO_STATUS1_REG."]
pub type STATUS1_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 interrupt status clear register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be cleared. Recommended operation: use this register to clear GPIO_STATUS1_REG."]
    #[inline(always)]
    #[must_use]
    pub fn status1_w1tc(&mut self) -> STATUS1_W1TC_W<STATUS1_W1TC_SPEC> {
        STATUS1_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO32 ~ 53 interrupt status bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status1_w1tc::W`](W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
