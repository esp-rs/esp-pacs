#[doc = "Register `LACTLOAD` writer"]
pub type W = crate::W<LACTLOAD_SPEC>;
#[doc = "Field `LOAD` writer - "]
pub type LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<LACTLOAD_SPEC> {
        LOAD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactload::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLOAD_SPEC;
impl crate::RegisterSpec for LACTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lactload::W`](W) writer structure"]
impl crate::Writable for LACTLOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LACTLOAD to value 0"]
impl crate::Resettable for LACTLOAD_SPEC {}
