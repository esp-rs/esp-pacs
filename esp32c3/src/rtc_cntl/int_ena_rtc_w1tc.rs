#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - clear sleep wakeup interrupt enable"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - clear sleep reject interrupt enable"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA_W1TC` writer - clear RTC WDT interrupt enable"]
pub type WDT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TC` writer - clear brown out interrupt enable"]
pub type BROWN_OUT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TC` writer - Clear RTC main timer interrupt enable"]
pub type MAIN_TIMER_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_ENA_W1TC` writer - clear super watch dog interrupt enable"]
pub type SWD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TC` writer - clear xtal32k_dead interrupt enable"]
pub type XTAL32K_DEAD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_ENA_W1TC` writer - clear gitch det interrupt enable"]
pub type GLITCH_DET_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_ENA_W1TC` writer - clear bbpll cal interrupt enable"]
pub type BBPLL_CAL_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - clear sleep wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear sleep reject interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_REJECT_INT_ENA_W1TC_W::new(self, 1)
    }
    #[doc = "Bit 3 - clear RTC WDT interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena_w1tc(&mut self) -> WDT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        WDT_INT_ENA_W1TC_W::new(self, 3)
    }
    #[doc = "Bit 9 - clear brown out interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena_w1tc(&mut self) -> BROWN_OUT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        BROWN_OUT_INT_ENA_W1TC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena_w1tc(&mut self) -> MAIN_TIMER_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        MAIN_TIMER_INT_ENA_W1TC_W::new(self, 10)
    }
    #[doc = "Bit 15 - clear super watch dog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena_w1tc(&mut self) -> SWD_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SWD_INT_ENA_W1TC_W::new(self, 15)
    }
    #[doc = "Bit 16 - clear xtal32k_dead interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena_w1tc(
        &mut self,
    ) -> XTAL32K_DEAD_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        XTAL32K_DEAD_INT_ENA_W1TC_W::new(self, 16)
    }
    #[doc = "Bit 19 - clear gitch det interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena_w1tc(&mut self) -> GLITCH_DET_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        GLITCH_DET_INT_ENA_W1TC_W::new(self, 19)
    }
    #[doc = "Bit 20 - clear bbpll cal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_ena_w1tc(&mut self) -> BBPLL_CAL_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        BBPLL_CAL_INT_ENA_W1TC_W::new(self, 20)
    }
}
#[doc = "rtc configure register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1tc::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
