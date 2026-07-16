#[doc = "Register `STATUS2_W1TS` writer"]
pub type W = crate::W<STATUS2_W1TS_SPEC>;
#[doc = "Field `STATUS2_W1TS` writer - Configures whether or not to set the interrupt status register GPIO_STATUS2_INTERRUPT of GPIO64 ~ GPIO66. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS2_INTERRUPT will be set to 1. \\item Recommended operation: use this register to set GPIO_STATUS2_INTERRUPT."]
pub type STATUS2_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS2_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether or not to set the interrupt status register GPIO_STATUS2_INTERRUPT of GPIO64 ~ GPIO66. - Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. - If the value 1 is written to a bit here, the corresponding bit in GPIO_STATUS2_INTERRUPT will be set to 1. \\item Recommended operation: use this register to set GPIO_STATUS2_INTERRUPT."]
    #[inline(always)]
    pub fn status2_w1ts(&mut self) -> STATUS2_W1TS_W<'_, STATUS2_W1TS_SPEC> {
        STATUS2_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status2_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS2_W1TS_SPEC;
impl crate::RegisterSpec for STATUS2_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status2_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS2_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS2_W1TS to value 0"]
impl crate::Resettable for STATUS2_W1TS_SPEC {}
