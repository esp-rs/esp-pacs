#[doc = "Register `T0LOAD` writer"]
pub type W = crate::W<T0LOAD_SPEC>;
#[doc = "Field `T_LOAD` writer - Write any value to trigger a timer 0 time-base counter reload."]
pub type T_LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to trigger a timer 0 time-base counter reload."]
    #[inline(always)]
    pub fn t_load(&mut self) -> T_LOAD_W<'_, T0LOAD_SPEC> {
        T_LOAD_W::new(self, 0)
    }
}
#[doc = "Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LOAD_SPEC;
impl crate::RegisterSpec for T0LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t0load::W`](W) writer structure"]
impl crate::Writable for T0LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0LOAD to value 0"]
impl crate::Resettable for T0LOAD_SPEC {}
