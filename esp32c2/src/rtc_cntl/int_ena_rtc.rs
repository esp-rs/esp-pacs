#[doc = "Register `INT_ENA_RTC` reader"]
pub struct R(crate::R<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_RTC` writer"]
pub struct W(crate::W<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 0>;
#[doc = "Field `SLP_REJECT_INT_ENA` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT_INT_ENA` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 1>;
#[doc = "Field `RTC_WDT_INT_ENA` reader - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_WDT_INT_ENA` writer - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 3>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA` reader - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA` writer - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 9>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA` reader - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA` writer - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 10>;
#[doc = "Field `RTC_SWD_INT_ENA` reader - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SWD_INT_ENA` writer - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 15>;
#[doc = "Field `RTC_BBPLL_CAL_INT_ENA` reader - Need add desc"]
pub type RTC_BBPLL_CAL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_BBPLL_CAL_INT_ENA` writer - Need add desc"]
pub type RTC_BBPLL_CAL_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, 20>;
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
    pub fn rtc_wdt_int_ena(&self) -> RTC_WDT_INT_ENA_R {
        RTC_WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena(&self) -> RTC_BROWN_OUT_INT_ENA_R {
        RTC_BROWN_OUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena(&self) -> RTC_MAIN_TIMER_INT_ENA_R {
        RTC_MAIN_TIMER_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena(&self) -> RTC_SWD_INT_ENA_R {
        RTC_SWD_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_ena(&self) -> RTC_BBPLL_CAL_INT_ENA_R {
        RTC_BBPLL_CAL_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W {
        SLP_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W {
        SLP_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena(&mut self) -> RTC_WDT_INT_ENA_W {
        RTC_WDT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena(&mut self) -> RTC_BROWN_OUT_INT_ENA_W {
        RTC_BROWN_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W {
        RTC_MAIN_TIMER_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena(&mut self) -> RTC_SWD_INT_ENA_W {
        RTC_SWD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_ena(&mut self) -> RTC_BBPLL_CAL_INT_ENA_W {
        RTC_BBPLL_CAL_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc](index.html) module"]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_rtc::R](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
