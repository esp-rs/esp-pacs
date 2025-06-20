#[doc = "Register `_3_CONTINUE` writer"]
pub type W = crate::W<_3_CONTINUE_SPEC>;
#[doc = "Field `_3_CONTINUE` writer - Continue typical sha3."]
pub type _3_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Continue typical sha3."]
    #[inline(always)]
    pub fn _3_continue(&mut self) -> _3_CONTINUE_W<_3_CONTINUE_SPEC> {
        _3_CONTINUE_W::new(self, 0)
    }
}
#[doc = "Typical SHA3 configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_CONTINUE_SPEC;
impl crate::RegisterSpec for _3_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_continue::W`](W) writer structure"]
impl crate::Writable for _3_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _3_CONTINUE to value 0"]
impl crate::Resettable for _3_CONTINUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
