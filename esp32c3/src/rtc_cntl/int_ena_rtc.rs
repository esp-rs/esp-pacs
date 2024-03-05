#[doc = "Register `INT_ENA_RTC` reader"]
pub type R = crate::R<INT_ENA_RTC_SPEC>;
#[doc = "Register `INT_ENA_RTC` writer"]
pub type W = crate::W<INT_ENA_RTC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_ENA` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ENA` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA` reader - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA` reader - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ENA` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_ENA` reader - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ENA` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_ENA` reader - enable super watch dog interrupt"]
pub type SWD_INT_ENA_R = crate::BitReader;
#[doc = "Field `SWD_INT_ENA` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA` reader - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD_INT_ENA` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_ENA` reader - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_ENA` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_ENA` reader - enbale bbpll cal end interrupt"]
pub type BBPLL_CAL_INT_ENA_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_INT_ENA` writer - enbale bbpll cal end interrupt"]
pub type BBPLL_CAL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&self) -> SLP_WAKEUP_INT_ENA_R {
        SLP_WAKEUP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena(&self) -> SLP_REJECT_INT_ENA_R {
        SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out_int_ena(&self) -> BROWN_OUT_INT_ENA_R {
        BROWN_OUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer_int_ena(&self) -> MAIN_TIMER_INT_ENA_R {
        MAIN_TIMER_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd_int_ena(&self) -> SWD_INT_ENA_R {
        SWD_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena(&self) -> XTAL32K_DEAD_INT_ENA_R {
        XTAL32K_DEAD_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enbale bbpll cal end interrupt"]
    #[inline(always)]
    pub fn bbpll_cal_int_ena(&self) -> BBPLL_CAL_INT_ENA_R {
        BBPLL_CAL_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_RTC")
            .field(
                "slp_wakeup_int_ena",
                &format_args!("{}", self.slp_wakeup_int_ena().bit()),
            )
            .field(
                "slp_reject_int_ena",
                &format_args!("{}", self.slp_reject_int_ena().bit()),
            )
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .field(
                "brown_out_int_ena",
                &format_args!("{}", self.brown_out_int_ena().bit()),
            )
            .field(
                "main_timer_int_ena",
                &format_args!("{}", self.main_timer_int_ena().bit()),
            )
            .field("swd_int_ena", &format_args!("{}", self.swd_int_ena().bit()))
            .field(
                "xtal32k_dead_int_ena",
                &format_args!("{}", self.xtal32k_dead_int_ena().bit()),
            )
            .field(
                "glitch_det_int_ena",
                &format_args!("{}", self.glitch_det_int_ena().bit()),
            )
            .field(
                "bbpll_cal_int_ena",
                &format_args!("{}", self.bbpll_cal_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W<INT_ENA_RTC_SPEC> {
        SLP_WAKEUP_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W<INT_ENA_RTC_SPEC> {
        SLP_REJECT_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<INT_ENA_RTC_SPEC> {
        WDT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena(&mut self) -> BROWN_OUT_INT_ENA_W<INT_ENA_RTC_SPEC> {
        BROWN_OUT_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena(&mut self) -> MAIN_TIMER_INT_ENA_W<INT_ENA_RTC_SPEC> {
        MAIN_TIMER_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena(&mut self) -> SWD_INT_ENA_W<INT_ENA_RTC_SPEC> {
        SWD_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena(&mut self) -> XTAL32K_DEAD_INT_ENA_W<INT_ENA_RTC_SPEC> {
        XTAL32K_DEAD_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W<INT_ENA_RTC_SPEC> {
        GLITCH_DET_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - enbale bbpll cal end interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_ena(&mut self) -> BBPLL_CAL_INT_ENA_W<INT_ENA_RTC_SPEC> {
        BBPLL_CAL_INT_ENA_W::new(self, 20)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_rtc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_rtc::R`](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
