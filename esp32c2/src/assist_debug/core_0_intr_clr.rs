///Register `CORE_0_INTR_CLR` writer
pub type W = crate::W<CORE_0_INTR_CLR_SPEC>;
///Field `CORE_0_SP_SPILL_MIN_CLR` writer - clr sp underlow monitor interrupt
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_0_SP_SPILL_MAX_CLR` writer - clr sp overflow monitor interrupt
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clr sp underlow monitor interrupt
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self, 0)
    }
    ///Bit 1 - clr sp overflow monitor interrupt
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<CORE_0_INTR_CLR_SPEC> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self, 1)
    }
}
/**core0 monitor interrupt clr register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`core_0_intr_clr::W`](W) writer structure
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_INTR_CLR to value 0
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
