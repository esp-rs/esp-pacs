#[doc = "Register `OUT_W1TC` writer"]
pub type W = crate::W<OUT_W1TC_SPEC>;
#[doc = "Field `OUT_W1TC` writer - Configures whether or not to clear the output register LP_IO_OUT_REG of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_OUT_REG will be cleared. - Recommended operation: use this register to clear LP_IO_OUT_REG."]
pub type OUT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures whether or not to clear the output register LP_IO_OUT_REG of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_OUT_REG will be cleared. - Recommended operation: use this register to clear LP_IO_OUT_REG."]
    #[inline(always)]
    pub fn out_w1tc(&mut self) -> OUT_W1TC_W<'_, OUT_W1TC_SPEC> {
        OUT_W1TC_W::new(self, 0)
    }
}
#[doc = "LP GPIO output clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure"]
impl crate::Writable for OUT_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OUT_W1TC_SPEC {}
