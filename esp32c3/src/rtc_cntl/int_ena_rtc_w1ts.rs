#[doc = "Register `INT_ENA_RTC_W1TS` writer"]
pub struct W(crate::W<INT_ENA_RTC_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENA_RTC_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TS` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TS` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_WDT_INT_ENA_W1TS` writer - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA_W1TS` writer - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA_W1TS` writer - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_SWD_INT_ENA_W1TS` writer - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA_W1TS` writer - enable xtal32k_dead interrupt"]
pub type RTC_XTAL32K_DEAD_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_GLITCH_DET_INT_ENA_W1TS` writer - enbale gitch det interrupt"]
pub type RTC_GLITCH_DET_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
#[doc = "Field `RTC_BBPLL_CAL_INT_ENA_W1TS` writer - enbale bbpll cal interrupt"]
pub type RTC_BBPLL_CAL_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1ts(&mut self) -> SLP_WAKEUP_INT_ENA_W1TS_W<0> {
        SLP_WAKEUP_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1ts(&mut self) -> SLP_REJECT_INT_ENA_W1TS_W<1> {
        SLP_REJECT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena_w1ts(&mut self) -> RTC_WDT_INT_ENA_W1TS_W<3> {
        RTC_WDT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena_w1ts(&mut self) -> RTC_BROWN_OUT_INT_ENA_W1TS_W<9> {
        RTC_BROWN_OUT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena_w1ts(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W1TS_W<10> {
        RTC_MAIN_TIMER_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena_w1ts(&mut self) -> RTC_SWD_INT_ENA_W1TS_W<15> {
        RTC_SWD_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena_w1ts(&mut self) -> RTC_XTAL32K_DEAD_INT_ENA_W1TS_W<16> {
        RTC_XTAL32K_DEAD_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena_w1ts(&mut self) -> RTC_GLITCH_DET_INT_ENA_W1TS_W<19> {
        RTC_GLITCH_DET_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 20 - enbale bbpll cal interrupt"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_ena_w1ts(&mut self) -> RTC_BBPLL_CAL_INT_ENA_W1TS_W<20> {
        RTC_BBPLL_CAL_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc_w1ts](index.html) module"]
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc_w1ts::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TS to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
