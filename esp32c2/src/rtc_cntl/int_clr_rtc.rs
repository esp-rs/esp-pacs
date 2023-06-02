#[doc = "Register `INT_CLR_RTC` reader"]
pub struct R(crate::R<INT_CLR_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CLR_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CLR_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CLR_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_CLR_RTC` writer"]
pub struct W(crate::W<INT_CLR_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_RTC_SPEC>;
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
impl From<crate::W<INT_CLR_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_CLR` reader - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SLP_REJECT_INT_CLR` reader - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `WDT_INT_CLR` reader - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_R = crate::BitReader;
#[doc = "Field `WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `BROWN_OUT_INT_CLR` reader - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `MAIN_TIMER_INT_CLR` reader - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SWD_INT_CLR` reader - Clear super watch dog interrupt state"]
pub type SWD_INT_CLR_R = crate::BitReader;
#[doc = "Field `SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub type SWD_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `BBPLL_CAL_INT_CLR` reader - Need add desc"]
pub type BBPLL_CAL_INT_CLR_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_INT_CLR` writer - Need add desc"]
pub type BBPLL_CAL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&self) -> SLP_WAKEUP_INT_CLR_R {
        SLP_WAKEUP_INT_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&self) -> SLP_REJECT_INT_CLR_R {
        SLP_REJECT_INT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_clr(&self) -> WDT_INT_CLR_R {
        WDT_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_clr(&self) -> BROWN_OUT_INT_CLR_R {
        BROWN_OUT_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_clr(&self) -> MAIN_TIMER_INT_CLR_R {
        MAIN_TIMER_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn swd_int_clr(&self) -> SWD_INT_CLR_R {
        SWD_INT_CLR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal_int_clr(&self) -> BBPLL_CAL_INT_CLR_R {
        BBPLL_CAL_INT_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR_RTC")
            .field(
                "slp_wakeup_int_clr",
                &format_args!("{}", self.slp_wakeup_int_clr().bit()),
            )
            .field(
                "slp_reject_int_clr",
                &format_args!("{}", self.slp_reject_int_clr().bit()),
            )
            .field("wdt_int_clr", &format_args!("{}", self.wdt_int_clr().bit()))
            .field(
                "brown_out_int_clr",
                &format_args!("{}", self.brown_out_int_clr().bit()),
            )
            .field(
                "main_timer_int_clr",
                &format_args!("{}", self.main_timer_int_clr().bit()),
            )
            .field("swd_int_clr", &format_args!("{}", self.swd_int_clr().bit()))
            .field(
                "bbpll_cal_int_clr",
                &format_args!("{}", self.bbpll_cal_int_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W<0> {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W<1> {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<3> {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<9> {
        BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W<10> {
        MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W<15> {
        SWD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_clr(&mut self) -> BBPLL_CAL_INT_CLR_W<20> {
        BBPLL_CAL_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_rtc](index.html) module"]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_clr_rtc::R](R) reader structure"]
impl crate::Readable for INT_CLR_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_clr_rtc::W](W) writer structure"]
impl crate::Writable for INT_CLR_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
