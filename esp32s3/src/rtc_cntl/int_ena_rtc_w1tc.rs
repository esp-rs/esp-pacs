///Register `INT_ENA_RTC_W1TC` writer
pub type W = crate::W<INT_ENA_RTC_W1TC_SPEC>;
///Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SLP_REJECT` writer - enable sleep reject interrupt
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SDIO_IDLE` writer - enable SDIO idle interrupt
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `WDT` writer - enable RTC WDT interrupt
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_SCAN_DONE` writer - enable touch scan done interrupt
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ULP_CP` writer - enable ULP-coprocessor interrupt
pub type ULP_CP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_DONE` writer - enable touch done interrupt
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_ACTIVE` writer - enable touch active interrupt
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_INACTIVE` writer - enable touch inactive interrupt
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `BROWN_OUT` writer - enable brown out interrupt
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `MAIN_TIMER` writer - enable RTC main timer interrupt
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SARADC1` writer - enable saradc1 interrupt
pub type SARADC1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TSENS` writer - enable tsens interrupt
pub type TSENS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `COCPU` writer - enable riscV cocpu interrupt
pub type COCPU_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SARADC2` writer - enable saradc2 interrupt
pub type SARADC2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SWD` writer - enable super watch dog interrupt
pub type SWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `XTAL32K_DEAD` writer - enable xtal32k_dead interrupt
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `COCPU_TRAP` writer - enable cocpu trap interrupt
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_TIMEOUT` writer - enable touch timeout interrupt
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `GLITCH_DET` writer - enbale gitch det interrupt
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TOUCH_APPROACH_LOOP_DONE` writer - enbale touch approach_loop done interrupt
pub type TOUCH_APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - enable sleep wakeup interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    ///Bit 1 - enable sleep reject interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_RTC_W1TC_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    ///Bit 2 - enable SDIO idle interrupt
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_ENA_RTC_W1TC_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    ///Bit 3 - enable RTC WDT interrupt
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_RTC_W1TC_SPEC> {
        WDT_W::new(self, 3)
    }
    ///Bit 4 - enable touch scan done interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    ///Bit 5 - enable ULP-coprocessor interrupt
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_ENA_RTC_W1TC_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    ///Bit 6 - enable touch done interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    ///Bit 7 - enable touch active interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    ///Bit 8 - enable touch inactive interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    ///Bit 9 - enable brown out interrupt
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_RTC_W1TC_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    ///Bit 10 - enable RTC main timer interrupt
    #[inline(always)]
    #[must_use]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_RTC_W1TC_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    ///Bit 11 - enable saradc1 interrupt
    #[inline(always)]
    #[must_use]
    pub fn saradc1(&mut self) -> SARADC1_W<INT_ENA_RTC_W1TC_SPEC> {
        SARADC1_W::new(self, 11)
    }
    ///Bit 12 - enable tsens interrupt
    #[inline(always)]
    #[must_use]
    pub fn tsens(&mut self) -> TSENS_W<INT_ENA_RTC_W1TC_SPEC> {
        TSENS_W::new(self, 12)
    }
    ///Bit 13 - enable riscV cocpu interrupt
    #[inline(always)]
    #[must_use]
    pub fn cocpu(&mut self) -> COCPU_W<INT_ENA_RTC_W1TC_SPEC> {
        COCPU_W::new(self, 13)
    }
    ///Bit 14 - enable saradc2 interrupt
    #[inline(always)]
    #[must_use]
    pub fn saradc2(&mut self) -> SARADC2_W<INT_ENA_RTC_W1TC_SPEC> {
        SARADC2_W::new(self, 14)
    }
    ///Bit 15 - enable super watch dog interrupt
    #[inline(always)]
    #[must_use]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_RTC_W1TC_SPEC> {
        SWD_W::new(self, 15)
    }
    ///Bit 16 - enable xtal32k_dead interrupt
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_RTC_W1TC_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    ///Bit 17 - enable cocpu trap interrupt
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<INT_ENA_RTC_W1TC_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    ///Bit 18 - enable touch timeout interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    ///Bit 19 - enbale gitch det interrupt
    #[inline(always)]
    #[must_use]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_RTC_W1TC_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    ///Bit 20 - enbale touch approach_loop done interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done(
        &mut self,
    ) -> TOUCH_APPROACH_LOOP_DONE_W<INT_ENA_RTC_W1TC_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_W::new(self, 20)
    }
}
/**oneset clr rtc interrupt enable

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_ena_rtc_w1tc::W`](W) writer structure
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_ffff;
}
///`reset()` method sets INT_ENA_RTC_W1TC to value 0
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
