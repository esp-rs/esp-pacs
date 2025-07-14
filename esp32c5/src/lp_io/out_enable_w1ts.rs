#[doc = "Register `OUT_ENABLE_W1TS` writer"]
pub type W = crate::W<OUT_ENABLE_W1TS_SPEC>;
#[doc = "Field `ENABLE_W1TS` writer - Configures whether or not to set the output enable register LP_IO_ENABLE_REG of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_ENABLE_REG will be set to 1. - Recommended operation: use this register to set LP_IO_ENABLE_REG."]
pub type ENABLE_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ENABLE_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures whether or not to set the output enable register LP_IO_ENABLE_REG of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_ENABLE_REG will be set to 1. - Recommended operation: use this register to set LP_IO_ENABLE_REG."]
    #[inline(always)]
    pub fn enable_w1ts(&mut self) -> ENABLE_W1TS_W<OUT_ENABLE_W1TS_SPEC> {
        ENABLE_W1TS_W::new(self, 0)
    }
}
#[doc = "LP GPIO output enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_enable_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT_ENABLE_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_ENABLE_W1TS to value 0"]
impl crate::Resettable for OUT_ENABLE_W1TS_SPEC {}
