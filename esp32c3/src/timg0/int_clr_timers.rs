///Register `INT_CLR_TIMERS` writer
pub type W = crate::W<INT_CLR_TIMERS_SPEC>;
///Field `T(0-0)` writer - t%s_int_clr
pub type T_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `WDT` writer - wdt_int_clr
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///t(0-0)_int_clr
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self, n: u8) -> T_W<INT_CLR_TIMERS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        T_W::new(self, n * 0)
    }
    ///Bit 0 - t0_int_clr
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T_W<INT_CLR_TIMERS_SPEC> {
        T_W::new(self, 0)
    }
    ///Bit 1 - wdt_int_clr
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_CLR_TIMERS_SPEC> {
        WDT_W::new(self, 1)
    }
}
/**INT_CLR_TIMG_REG

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_TIMERS_SPEC;
impl crate::RegisterSpec for INT_CLR_TIMERS_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr_timers::W`](W) writer structure
impl crate::Writable for INT_CLR_TIMERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
///`reset()` method sets INT_CLR_TIMERS to value 0
impl crate::Resettable for INT_CLR_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
