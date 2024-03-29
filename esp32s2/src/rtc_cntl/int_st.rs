#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - Stores the status of the interrupt triggered when the chip wakes up from sleep."]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` reader - Stores the status of the interrupt triggered when the chip rejects to go to sleep."]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` reader - Stores the status of the interrupt triggered when the SDIO idles."]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `WDT` reader - Stores the status of the RTC watchdog interrupt."]
pub type WDT_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE` reader - Stores the status of the interrupt triggered upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_R = crate::BitReader;
#[doc = "Field `ULP_CP` reader - Stores the status of the ULP co-processor interrupt."]
pub type ULP_CP_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE` reader - Stores the status of the interrupt triggered upon the completion of a single touch."]
pub type TOUCH_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE` reader - Stores the status of the interrupt triggered when a touch is detected."]
pub type TOUCH_ACTIVE_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE` reader - Stores the status of the interrupt triggered when a touch is released."]
pub type TOUCH_INACTIVE_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - Stores the status of the brown out interrupt."]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - Stores the status of the RTC main timer interrupt."]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `SARADC1` reader - Stores the status of the SAR ADC 1 interrupt."]
pub type SARADC1_R = crate::BitReader;
#[doc = "Field `TSENS` reader - Stores the status of the touch sensor interrupt."]
pub type TSENS_R = crate::BitReader;
#[doc = "Field `COCPU` reader - Stores the status of the ULP-RISCV interrupt."]
pub type COCPU_R = crate::BitReader;
#[doc = "Field `SARADC2` reader - Stores the status of the SAR ADC 2 interrupt."]
pub type SARADC2_R = crate::BitReader;
#[doc = "Field `SWD` reader - Stores the status of the super watchdog interrupt."]
pub type SWD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` reader - Stores the status of the interrupt triggered when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP` reader - Stores the status of the interrupt triggered when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT` reader - Stores the status of the interrupt triggered when touch sensor times out."]
pub type TOUCH_TIMEOUT_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - Stores the status of the interrupt triggered when a glitch is detected."]
pub type GLITCH_DET_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stores the status of the interrupt triggered when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stores the status of the interrupt triggered when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stores the status of the interrupt triggered when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stores the status of the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stores the status of the interrupt triggered upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done(&self) -> TOUCH_SCAN_DONE_R {
        TOUCH_SCAN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stores the status of the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stores the status of the interrupt triggered upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done(&self) -> TOUCH_DONE_R {
        TOUCH_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stores the status of the interrupt triggered when a touch is detected."]
    #[inline(always)]
    pub fn touch_active(&self) -> TOUCH_ACTIVE_R {
        TOUCH_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stores the status of the interrupt triggered when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive(&self) -> TOUCH_INACTIVE_R {
        TOUCH_INACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stores the status of the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stores the status of the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stores the status of the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1(&self) -> SARADC1_R {
        SARADC1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stores the status of the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stores the status of the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu(&self) -> COCPU_R {
        COCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stores the status of the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2(&self) -> SARADC2_R {
        SARADC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Stores the status of the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Stores the status of the interrupt triggered when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stores the status of the interrupt triggered when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Stores the status of the interrupt triggered when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout(&self) -> TOUCH_TIMEOUT_R {
        TOUCH_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stores the status of the interrupt triggered when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("slp_wakeup", &format_args!("{}", self.slp_wakeup().bit()))
            .field("slp_reject", &format_args!("{}", self.slp_reject().bit()))
            .field("sdio_idle", &format_args!("{}", self.sdio_idle().bit()))
            .field("wdt", &format_args!("{}", self.wdt().bit()))
            .field(
                "touch_scan_done",
                &format_args!("{}", self.touch_scan_done().bit()),
            )
            .field("ulp_cp", &format_args!("{}", self.ulp_cp().bit()))
            .field("touch_done", &format_args!("{}", self.touch_done().bit()))
            .field(
                "touch_active",
                &format_args!("{}", self.touch_active().bit()),
            )
            .field(
                "touch_inactive",
                &format_args!("{}", self.touch_inactive().bit()),
            )
            .field("brown_out", &format_args!("{}", self.brown_out().bit()))
            .field("main_timer", &format_args!("{}", self.main_timer().bit()))
            .field("saradc1", &format_args!("{}", self.saradc1().bit()))
            .field("tsens", &format_args!("{}", self.tsens().bit()))
            .field("cocpu", &format_args!("{}", self.cocpu().bit()))
            .field("saradc2", &format_args!("{}", self.saradc2().bit()))
            .field("swd", &format_args!("{}", self.swd().bit()))
            .field(
                "xtal32k_dead",
                &format_args!("{}", self.xtal32k_dead().bit()),
            )
            .field("cocpu_trap", &format_args!("{}", self.cocpu_trap().bit()))
            .field(
                "touch_timeout",
                &format_args!("{}", self.touch_timeout().bit()),
            )
            .field("glitch_det", &format_args!("{}", self.glitch_det().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RTC interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
