#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT` reader - enable RTC WDT interrupt"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - enable RTC WDT interrupt"]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - enable brown out interrupt"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - enable brown out interrupt"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER` reader - enable RTC main timer interrupt"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD` reader - enable super watch dog interrupt"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `SWD` writer - enable super watch dog interrupt"]
pub type SWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD` reader - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET` reader - enbale gitch det interrupt"]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL_CAL` reader - enbale bbpll cal end interrupt"]
pub type BBPLL_CAL_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL` writer - enbale bbpll cal end interrupt"]
pub type BBPLL_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enbale bbpll cal end interrupt"]
    #[inline(always)]
    pub fn bbpll_cal(&self) -> BBPLL_CAL_R {
        BBPLL_CAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("slp_wakeup", &format_args!("{}", self.slp_wakeup().bit()))
            .field("slp_reject", &format_args!("{}", self.slp_reject().bit()))
            .field("wdt", &format_args!("{}", self.wdt().bit()))
            .field("brown_out", &format_args!("{}", self.brown_out().bit()))
            .field("main_timer", &format_args!("{}", self.main_timer().bit()))
            .field("swd", &format_args!("{}", self.swd().bit()))
            .field(
                "xtal32k_dead",
                &format_args!("{}", self.xtal32k_dead().bit()),
            )
            .field("glitch_det", &format_args!("{}", self.glitch_det().bit()))
            .field("bbpll_cal", &format_args!("{}", self.bbpll_cal().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    #[doc = "Bit 20 - enbale bbpll cal end interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal(&mut self) -> BBPLL_CAL_W<INT_ENA_SPEC> {
        BBPLL_CAL_W::new(self, 20)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
