#[doc = "Register `INT_CLR` reader"]
pub type R = crate::R<INT_CLR_SPEC>;
#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLP_REJECT` reader - Clear sleep reject interrupt state"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` reader - Clear RTC WDT interrupt state"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - Clear RTC WDT interrupt state"]
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - Clear brown out interrupt state"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` reader - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SWD` reader - Clear super watch dog interrupt state"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `SWD` writer - Clear super watch dog interrupt state"]
pub type SWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BBPLL_CAL` reader - Need add desc"]
pub type BBPLL_CAL_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL` writer - Need add desc"]
pub type BBPLL_CAL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
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
        f.debug_struct("INT_CLR")
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
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_CLR_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_CLR_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<INT_CLR_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_CLR_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_CLR_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<INT_CLR_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 20 - Need add desc"]
    #[inline(always)]
    pub fn bbpll_cal(&mut self) -> BBPLL_CAL_W<INT_CLR_SPEC> {
        BBPLL_CAL_W::new(self, 20)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clr::R`](R) reader structure"]
impl crate::Readable for INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_860b;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
