#[doc = "Register `INT_ENA_RTC_W1TC` reader"]
pub type R = crate::R<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_ENA_W1TC` reader - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA_W1TC` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TC` reader - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ENA_W1TC` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TC` reader - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TC` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD_INT_ENA_W1TC` reader - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `SWD_INT_ENA_W1TC` writer - enable super watch dog interrupt"]
pub type SWD_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL_INT_ENA_W1TC` reader - Need add desc"]
pub type BBPLL_CAL_INT_ENA_W1TC_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_INT_ENA_W1TC` writer - Need add desc"]
pub type BBPLL_CAL_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1tc(&self) -> SLP_WAKEUP_INT_ENA_W1TC_R {
        SLP_WAKEUP_INT_ENA_W1TC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1tc(&self) -> SLP_REJECT_INT_ENA_W1TC_R {
        SLP_REJECT_INT_ENA_W1TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt_int_ena_w1tc(&self) -> WDT_INT_ENA_W1TC_R {
        WDT_INT_ENA_W1TC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out_int_ena_w1tc(&self) -> BROWN_OUT_INT_ENA_W1TC_R {
        BROWN_OUT_INT_ENA_W1TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer_int_ena_w1tc(&self) -> MAIN_TIMER_INT_ENA_W1TC_R {
        MAIN_TIMER_INT_ENA_W1TC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd_int_ena_w1tc(&self) -> SWD_INT_ENA_W1TC_R {
        SWD_INT_ENA_W1TC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal_int_ena_w1tc(&self) -> BBPLL_CAL_INT_ENA_W1TC_R {
        BBPLL_CAL_INT_ENA_W1TC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_RTC_W1TC")
            .field(
                "slp_wakeup_int_ena_w1tc",
                &format_args!("{}", self.slp_wakeup_int_ena_w1tc().bit()),
            )
            .field(
                "slp_reject_int_ena_w1tc",
                &format_args!("{}", self.slp_reject_int_ena_w1tc().bit()),
            )
            .field(
                "wdt_int_ena_w1tc",
                &format_args!("{}", self.wdt_int_ena_w1tc().bit()),
            )
            .field(
                "brown_out_int_ena_w1tc",
                &format_args!("{}", self.brown_out_int_ena_w1tc().bit()),
            )
            .field(
                "main_timer_int_ena_w1tc",
                &format_args!("{}", self.main_timer_int_ena_w1tc().bit()),
            )
            .field(
                "swd_int_ena_w1tc",
                &format_args!("{}", self.swd_int_ena_w1tc().bit()),
            )
            .field(
                "bbpll_cal_int_ena_w1tc",
                &format_args!("{}", self.bbpll_cal_int_ena_w1tc().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_REJECT_INT_ENA_W1TC_W::new(self, 1)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena_w1tc(&mut self) -> WDT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        WDT_INT_ENA_W1TC_W::new(self, 3)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena_w1tc(&mut self) -> BROWN_OUT_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        BROWN_OUT_INT_ENA_W1TC_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena_w1tc(&mut self) -> MAIN_TIMER_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        MAIN_TIMER_INT_ENA_W1TC_W::new(self, 10)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena_w1tc(&mut self) -> SWD_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        SWD_INT_ENA_W1TC_W::new(self, 15)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_int_ena_w1tc(&mut self) -> BBPLL_CAL_INT_ENA_W1TC_W<INT_ENA_RTC_W1TC_SPEC> {
        BBPLL_CAL_INT_ENA_W1TC_W::new(self, 20)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_rtc_w1tc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_rtc_w1tc::R`](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_W1TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1tc::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
