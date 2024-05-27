///Register `IV_%s` writer
pub type W = crate::W<IV__SPEC>;
///Field `IV` writer - IV block data.
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IV__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - IV block data.
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IV__SPEC> {
        IV_W::new(self, 0)
    }
}
/**IV block data.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`iv_::W`](W) writer structure
impl crate::Writable for IV__SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IV_%s to value 0
impl crate::Resettable for IV__SPEC {
    const RESET_VALUE: u32 = 0;
}
