#[doc = "Register `INT_CLR_RTC` writer"]
pub type W = crate::W<INT_CLR_RTC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub type SWD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type XTAL32K_DEAD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Clear glitch det interrupt state"]
pub type GLITCH_DET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_CLR` writer - clear bbpll cal end interrupt state"]
pub type BBPLL_CAL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W<INT_CLR_RTC_SPEC> {
        SLP_WAKEUP_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W<INT_CLR_RTC_SPEC> {
        SLP_REJECT_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_RTC_SPEC> {
        WDT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<INT_CLR_RTC_SPEC> {
        BROWN_OUT_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W<INT_CLR_RTC_SPEC> {
        MAIN_TIMER_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W<INT_CLR_RTC_SPEC> {
        SWD_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_clr(&mut self) -> XTAL32K_DEAD_INT_CLR_W<INT_CLR_RTC_SPEC> {
        XTAL32K_DEAD_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W<INT_CLR_RTC_SPEC> {
        GLITCH_DET_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - clear bbpll cal end interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_clr(&mut self) -> BBPLL_CAL_INT_CLR_W<INT_CLR_RTC_SPEC> {
        BBPLL_CAL_INT_CLR_W::new(self, 20)
    }
}
#[doc = "rtc configure register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_rtc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr_rtc::W`](W) writer structure"]
impl crate::Writable for INT_CLR_RTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
