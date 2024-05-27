///Register `ENABLE_W1TS` writer
pub type W = crate::W<ENABLE_W1TS_SPEC>;
///Field `ENABLE_W1TS` writer - GPIO0~17 output enable write 1 to set
pub type ENABLE_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 14:31 - GPIO0~17 output enable write 1 to set
    #[inline(always)]
    #[must_use]
    pub fn enable_w1ts(&mut self) -> ENABLE_W1TS_W<ENABLE_W1TS_SPEC> {
        ENABLE_W1TS_W::new(self, 14)
    }
}
/**

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE_W1TS_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`enable_w1ts::W`](W) writer structure
impl crate::Writable for ENABLE_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENABLE_W1TS to value 0
impl crate::Resettable for ENABLE_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
