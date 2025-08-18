#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLP_WAKEUP` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLP_REJECT` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SDIO_IDLE` writer - Clear SDIO idle interrupt state"]
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` writer - Clear RTC WDT interrupt state"]
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_SCAN_DONE` writer - clear touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ULP_CP` writer - Clear ULP-coprocessor interrupt state"]
pub type ULP_CP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_DONE` writer - Clear touch done interrupt state"]
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_ACTIVE` writer - Clear touch active interrupt state"]
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE` writer - Clear touch inactive interrupt state"]
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SARADC1` writer - Clear saradc1 interrupt state"]
pub type SARADC1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSENS` writer - Clear tsens interrupt state"]
pub type TSENS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COCPU` writer - Clear riscV cocpu interrupt state"]
pub type COCPU_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SARADC2` writer - Clear saradc2 interrupt state"]
pub type SARADC2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SWD` writer - Clear super watch dog interrupt state"]
pub type SWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XTAL32K_DEAD` writer - Clear RTC WDT interrupt state"]
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COCPU_TRAP` writer - Clear cocpu trap interrupt state"]
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_TIMEOUT` writer - Clear touch timeout interrupt state"]
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - Clear glitch det interrupt state"]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE` writer - cleartouch approach mode loop interrupt state"]
pub type TOUCH_APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<'_, INT_CLR_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<'_, INT_CLR_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<'_, INT_CLR_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<'_, INT_CLR_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear touch scan done interrupt raw"]
    #[inline(always)]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<'_, INT_CLR_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<'_, INT_CLR_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear touch done interrupt state"]
    #[inline(always)]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<'_, INT_CLR_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear touch active interrupt state"]
    #[inline(always)]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<'_, INT_CLR_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear touch inactive interrupt state"]
    #[inline(always)]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<'_, INT_CLR_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<'_, INT_CLR_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<'_, INT_CLR_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear saradc1 interrupt state"]
    #[inline(always)]
    pub fn saradc1(&mut self) -> SARADC1_W<'_, INT_CLR_SPEC> {
        SARADC1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear tsens interrupt state"]
    #[inline(always)]
    pub fn tsens(&mut self) -> TSENS_W<'_, INT_CLR_SPEC> {
        TSENS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn cocpu(&mut self) -> COCPU_W<'_, INT_CLR_SPEC> {
        COCPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear saradc2 interrupt state"]
    #[inline(always)]
    pub fn saradc2(&mut self) -> SARADC2_W<'_, INT_CLR_SPEC> {
        SARADC2_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<'_, INT_CLR_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<'_, INT_CLR_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear cocpu trap interrupt state"]
    #[inline(always)]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<'_, INT_CLR_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear touch timeout interrupt state"]
    #[inline(always)]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<'_, INT_CLR_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<'_, INT_CLR_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    #[doc = "Bit 20 - cleartouch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn touch_approach_loop_done(&mut self) -> TOUCH_APPROACH_LOOP_DONE_W<'_, INT_CLR_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_W::new(self, 20)
    }
}
#[doc = "rtc interrupt register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
