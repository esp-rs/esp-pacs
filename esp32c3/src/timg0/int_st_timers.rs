#[doc = "Register `INT_ST_TIMERS` reader"]
pub type R = crate::R<INT_ST_TIMERS_SPEC>;
#[doc = "Field `T0_INT_ST` reader - t0_int_st"]
pub type T0_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - wdt_int_st"]
pub type WDT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - t0_int_st"]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt_int_st"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_TIMERS")
            .field("t0_int_st", &format_args!("{}", self.t0_int_st().bit()))
            .field("wdt_int_st", &format_args!("{}", self.wdt_int_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "INT_ST_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ST_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_timers::R`](R) reader structure"]
impl crate::Readable for INT_ST_TIMERS_SPEC {}
#[doc = "`reset()` method sets INT_ST_TIMERS to value 0"]
impl crate::Resettable for INT_ST_TIMERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
