#[doc = "Register `INT_ENA_RTC_W1TS` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TS_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TS` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TS` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA_W1TS` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TS` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TS` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_ENA_W1TS` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TS` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_ENA_W1TS` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_ENA_W1TS` writer - enbale bbpll cal interrupt"]
pub type BBPLL_CAL_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena_w1ts(&mut self) -> SLP_WAKEUP_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_WAKEUP_INT_ENA_W1TS_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena_w1ts(&mut self) -> SLP_REJECT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_REJECT_INT_ENA_W1TS_W::new(self, 1)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena_w1ts(&mut self) -> WDT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        WDT_INT_ENA_W1TS_W::new(self, 3)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena_w1ts(&mut self) -> BROWN_OUT_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        BROWN_OUT_INT_ENA_W1TS_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena_w1ts(&mut self) -> MAIN_TIMER_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        MAIN_TIMER_INT_ENA_W1TS_W::new(self, 10)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena_w1ts(&mut self) -> SWD_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        SWD_INT_ENA_W1TS_W::new(self, 15)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena_w1ts(
        &mut self,
    ) -> XTAL32K_DEAD_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        XTAL32K_DEAD_INT_ENA_W1TS_W::new(self, 16)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena_w1ts(&mut self) -> GLITCH_DET_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        GLITCH_DET_INT_ENA_W1TS_W::new(self, 19)
    }
    #[doc = "Bit 20 - enbale bbpll cal interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_ena_w1ts(&mut self) -> BBPLL_CAL_INT_ENA_W1TS_W<INT_ENA_RTC_W1TS_SPEC> {
        BBPLL_CAL_INT_ENA_W1TS_W::new(self, 20)
    }
}
#[doc = "rtc configure register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1ts::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TS to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
