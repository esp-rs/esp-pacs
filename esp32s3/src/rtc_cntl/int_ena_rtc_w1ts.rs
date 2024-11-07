#[doc = "Register `INT_ENA_RTC_W1TS` writer"]
pub type W = crate::W<INT_ENA_RTC_W1TS_SPEC>;
#[doc = "Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SLP_REJECT` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SDIO_IDLE` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WDT` writer - enable RTC WDT interrupt"]
pub type WDT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_SCAN_DONE` writer - enable touch scan done interrupt"]
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ULP_CP` writer - enable ULP-coprocessor interrupt"]
pub type ULP_CP_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_DONE` writer - enable touch done interrupt"]
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_ACTIVE` writer - enable touch active interrupt"]
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE` writer - enable touch inactive interrupt"]
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BROWN_OUT` writer - enable brown out interrupt"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SARADC1` writer - enable saradc1 interrupt"]
pub type SARADC1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TSENS` writer - enable tsens interrupt"]
pub type TSENS_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `COCPU` writer - enable riscV cocpu interrupt"]
pub type COCPU_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SARADC2` writer - enable saradc2 interrupt"]
pub type SARADC2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWD` writer - enable super watch dog interrupt"]
pub type SWD_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `XTAL32K_DEAD` writer - enable xtal32k_dead interrupt"]
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `COCPU_TRAP` writer - enable cocpu trap interrupt"]
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_TIMEOUT` writer - enable touch timeout interrupt"]
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - enbale gitch det interrupt"]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE` writer - enbale touch approach_loop done interrupt"]
pub type TOUCH_APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_ENA_RTC_W1TS_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_RTC_W1TS_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_ENA_RTC_W1TS_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_RTC_W1TS_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_RTC_W1TS_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    pub fn saradc1(&mut self) -> SARADC1_W<INT_ENA_RTC_W1TS_SPEC> {
        SARADC1_W::new(self, 11)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    pub fn tsens(&mut self) -> TSENS_W<INT_ENA_RTC_W1TS_SPEC> {
        TSENS_W::new(self, 12)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    pub fn cocpu(&mut self) -> COCPU_W<INT_ENA_RTC_W1TS_SPEC> {
        COCPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    pub fn saradc2(&mut self) -> SARADC2_W<INT_ENA_RTC_W1TS_SPEC> {
        SARADC2_W::new(self, 14)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_RTC_W1TS_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_RTC_W1TS_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<INT_ENA_RTC_W1TS_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_RTC_W1TS_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    #[doc = "Bit 20 - enbale touch approach_loop done interrupt"]
    #[inline(always)]
    pub fn touch_approach_loop_done(
        &mut self,
    ) -> TOUCH_APPROACH_LOOP_DONE_W<INT_ENA_RTC_W1TS_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_W::new(self, 20)
    }
}
#[doc = "oneset rtc interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_rtc_w1ts::W`](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_ffff;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TS to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
