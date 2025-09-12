#[doc = "Register `_3_START` writer"]
pub type W = crate::W<_3_START_SPEC>;
#[doc = "Field `_3_START` writer - Start typical sha3."]
pub type _3_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_3_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start typical sha3."]
    #[inline(always)]
    pub fn _3_start(&mut self) -> _3_START_W<'_, _3_START_SPEC> {
        _3_START_W::new(self, 0)
    }
}
#[doc = "Typical SHA3 configuration register 0.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_START_SPEC;
impl crate::RegisterSpec for _3_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_3_start::W`](W) writer structure"]
impl crate::Writable for _3_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_START to value 0"]
impl crate::Resettable for _3_START_SPEC {}
