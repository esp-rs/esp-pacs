#[doc = "Register `_3_CLEAR_INT` writer"]
pub type W = crate::W<_3_CLEAR_INT_SPEC>;
#[doc = "Field `_3_CLEAR_INT` writer - Clear sha3 interrupt."]
pub type _3_CLEAR_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_CLEAR_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sha3 interrupt."]
    #[inline(always)]
    pub fn _3_clear_int(&mut self) -> _3_CLEAR_INT_W<_3_CLEAR_INT_SPEC> {
        _3_CLEAR_INT_W::new(self, 0)
    }
}
#[doc = "Interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_clear_int::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_CLEAR_INT_SPEC;
impl crate::RegisterSpec for _3_CLEAR_INT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_clear_int::W`](W) writer structure"]
impl crate::Writable for _3_CLEAR_INT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_CLEAR_INT to value 0"]
impl crate::Resettable for _3_CLEAR_INT_SPEC {}
