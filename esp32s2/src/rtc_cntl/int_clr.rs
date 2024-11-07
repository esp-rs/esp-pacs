#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLP_WAKEUP` writer - Clears the interrupt triggered when the chip wakes up from sleep."]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLP_REJECT` writer - Clears the interrupt triggered when the chip rejects to go to sleep."]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SDIO_IDLE` writer - Clears the interrupt triggered when the SDIO idles."]
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WDT` writer - Enables the RTC watchdog interrupt."]
pub type WDT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_SCAN_DONE` writer - Clears the interrupt triggered upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ULP_CP` writer - Enables the ULP co-processor interrupt."]
pub type ULP_CP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_DONE` writer - Clears the interrupt triggered upon the completion of a single touch."]
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_ACTIVE` writer - Clears the interrupt triggered when a touch is detected."]
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE` writer - Clears the interrupt triggered when a touch is released."]
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` writer - Clears the brown out interrupt."]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - Clears the RTC main timer interrupt."]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SARADC1` writer - Clears the SAR ADC 1 interrupt."]
pub type SARADC1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSENS` writer - Clears the touch sensor interrupt."]
pub type TSENS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COCPU` writer - Clears the ULP-RISCV interrupt."]
pub type COCPU_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SARADC2` writer - Clears the SAR ADC 2 interrupt."]
pub type SARADC2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SWD` writer - Clears the super watchdog interrupt."]
pub type SWD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XTAL32K_DEAD` writer - Clears the interrupt triggered when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COCPU_TRAP` writer - Clears the interrupt triggered when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH_TIMEOUT` writer - Clears the interrupt triggered when touch sensor times out."]
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GLITCH_DET` writer - Clears the interrupt triggered when a glitch is detected."]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clears the interrupt triggered when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_CLR_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the interrupt triggered when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_CLR_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the interrupt triggered when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_CLR_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<INT_CLR_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clears the interrupt triggered upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<INT_CLR_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_CLR_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clears the interrupt triggered upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<INT_CLR_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clears the interrupt triggered when a touch is detected."]
    #[inline(always)]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<INT_CLR_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clears the interrupt triggered when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<INT_CLR_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clears the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_CLR_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clears the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_CLR_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clears the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1(&mut self) -> SARADC1_W<INT_CLR_SPEC> {
        SARADC1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clears the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens(&mut self) -> TSENS_W<INT_CLR_SPEC> {
        TSENS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clears the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu(&mut self) -> COCPU_W<INT_CLR_SPEC> {
        COCPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clears the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2(&mut self) -> SARADC2_W<INT_CLR_SPEC> {
        SARADC2_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clears the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<INT_CLR_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clears the interrupt triggered when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_CLR_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clears the interrupt triggered when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<INT_CLR_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clears the interrupt triggered when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<INT_CLR_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clears the interrupt triggered when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_CLR_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
}
#[doc = "RTC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
