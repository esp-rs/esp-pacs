#[doc = "Register `OUT2_W1TS` writer"]
pub type W = crate::W<OUT2_W1TS_SPEC>;
#[doc = "Field `OUT2_W1TS` writer - Configures whether or not to set the output register GPIO_OUT2_REG of GPIO64 ~ GPIO66.\\\\ 0: Not set\\\\ 1: The corresponding bit in GPIO_OUT2_REG will be set to 1\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. \\\\ Recommended operation: use this register to set GPIO_OUT2_REG. \\\\"]
pub type OUT2_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT2_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether or not to set the output register GPIO_OUT2_REG of GPIO64 ~ GPIO66.\\\\ 0: Not set\\\\ 1: The corresponding bit in GPIO_OUT2_REG will be set to 1\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. \\\\ Recommended operation: use this register to set GPIO_OUT2_REG. \\\\"]
    #[inline(always)]
    pub fn out2_w1ts(&mut self) -> OUT2_W1TS_W<'_, OUT2_W1TS_SPEC> {
        OUT2_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out2_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT2_W1TS_SPEC;
impl crate::RegisterSpec for OUT2_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out2_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT2_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT2_W1TS to value 0"]
impl crate::Resettable for OUT2_W1TS_SPEC {}
