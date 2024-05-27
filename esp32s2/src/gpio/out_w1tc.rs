///Register `OUT_W1TC` writer
pub type W = crate::W<OUT_W1TC_SPEC>;
///Field `OUT_W1TC` writer - GPIO0 ~ 31 output clear register. If the value 1 is written to a bit here, the cor- responding bit in GPIO_OUT_REG will be cleared. Recommended operation: use this register to clear GPIO_OUT_REG.
pub type OUT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - GPIO0 ~ 31 output clear register. If the value 1 is written to a bit here, the cor- responding bit in GPIO_OUT_REG will be cleared. Recommended operation: use this register to clear GPIO_OUT_REG.
    #[inline(always)]
    #[must_use]
    pub fn out_w1tc(&mut self) -> OUT_W1TC_W<OUT_W1TC_SPEC> {
        OUT_W1TC_W::new(self, 0)
    }
}
/**GPIO0 ~ 31 output bit clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure
impl crate::Writable for OUT_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_W1TC to value 0
impl crate::Resettable for OUT_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
