#[doc = "Register `T0LOAD` writer"]
pub type W = crate::W<T0LOAD_SPEC>;
#[doc = "Field `LOAD` writer - t0_load"]
pub type LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - t0_load"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<T0LOAD_SPEC> {
        LOAD_W::new(self, 0)
    }
}
#[doc = "TIMG_T0LOAD_REG.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LOAD_SPEC;
impl crate::RegisterSpec for T0LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t0load::W`](W) writer structure"]
impl crate::Writable for T0LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0LOAD to value 0"]
impl crate::Resettable for T0LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
