#[doc = "Register `STATUS1_W1TS` writer"]
pub type W = crate::W<STATUS1_W1TS_SPEC>;
#[doc = "Field `STATUS1_W1TS` writer - GPIO32 ~ 53 interrupt status set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be set to 1. Recommended operation: use this register to set GPIO_STATUS1_REG."]
pub type STATUS1_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS1_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 interrupt status set register. If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS1_REG will be set to 1. Recommended operation: use this register to set GPIO_STATUS1_REG."]
    #[inline(always)]
    pub fn status1_w1ts(&mut self) -> STATUS1_W1TS_W<STATUS1_W1TS_SPEC> {
        STATUS1_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO32 ~ 53 interrupt status bit set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_W1TS_SPEC;
impl crate::RegisterSpec for STATUS1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status1_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS1_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS1_W1TS to value 0"]
impl crate::Resettable for STATUS1_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
