#[doc = "Register `INT_ENA_RTC_W1TC` reader"]
pub type R = crate::R<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TC_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLP_REJECT` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` reader - enable RTC WDT interrupt"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - enable RTC WDT interrupt"]
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - enable brown out interrupt"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - enable brown out interrupt"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` reader - enable RTC main timer interrupt"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SWD` reader - enable super watch dog interrupt"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `SWD` writer - enable super watch dog interrupt"]
pub type SWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BBPLL_CAL` reader - Need add desc"]
pub type BBPLL_CAL_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL` writer - Need add desc"]
pub type BBPLL_CAL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal(&self) -> BBPLL_CAL_R {
        BBPLL_CAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_RTC_W1TC")
            .field("slp_wakeup", &self.slp_wakeup())
            .field("slp_reject", &self.slp_reject())
            .field("wdt", &self.wdt())
            .field("brown_out", &self.brown_out())
            .field("main_timer", &self.main_timer())
            .field("swd", &self.swd())
            .field("bbpll_cal", &self.bbpll_cal())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal(&mut self) -> BBPLL_CAL_W<'_, INT_ENA_RTC_W1TC_SPEC> {
        BBPLL_CAL_W::new(self, 20)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena_rtc_w1tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_rtc_w1tc::R`](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_W1TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1tc::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_860b;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {}
