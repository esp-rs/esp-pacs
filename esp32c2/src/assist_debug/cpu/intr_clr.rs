#[doc = "Register `INTR_CLR` writer"]
pub type W = crate::W<INTR_CLR_SPEC>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - clr sp underlow monitor interrupt"]
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - clr sp overflow monitor interrupt"]
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - clr sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - clr sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self, 1)
    }
}
#[doc = "core0 monitor interrupt clr register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {}
