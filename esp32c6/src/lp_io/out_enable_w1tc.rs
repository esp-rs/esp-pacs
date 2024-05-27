///Register `OUT_ENABLE_W1TC` writer
pub type W = crate::W<OUT_ENABLE_W1TC_SPEC>;
///Field `ENABLE_W1TC` writer - clear one time output data
pub type ENABLE_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - clear one time output data
    #[inline(always)]
    #[must_use]
    pub fn enable_w1tc(&mut self) -> ENABLE_W1TC_W<OUT_ENABLE_W1TC_SPEC> {
        ENABLE_W1TC_W::new(self, 0)
    }
}
/**need des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`out_enable_w1tc::W`](W) writer structure
impl crate::Writable for OUT_ENABLE_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_ENABLE_W1TC to value 0
impl crate::Resettable for OUT_ENABLE_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
