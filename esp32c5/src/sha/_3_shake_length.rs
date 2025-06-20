#[doc = "Register `_3_SHAKE_LENGTH` writer"]
pub type W = crate::W<_3_SHAKE_LENGTH_SPEC>;
#[doc = "Field `_3_SHAKE_LENGTH` writer - SHAKE output hash word length"]
pub type _3_SHAKE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_SHAKE_LENGTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:10 - SHAKE output hash word length"]
    #[inline(always)]
    pub fn _3_shake_length(&mut self) -> _3_SHAKE_LENGTH_W<_3_SHAKE_LENGTH_SPEC> {
        _3_SHAKE_LENGTH_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 3.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_shake_length::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_SHAKE_LENGTH_SPEC;
impl crate::RegisterSpec for _3_SHAKE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_shake_length::W`](W) writer structure"]
impl crate::Writable for _3_SHAKE_LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _3_SHAKE_LENGTH to value 0x32"]
impl crate::Resettable for _3_SHAKE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0x32;
}
