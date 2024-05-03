#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup interrupt state"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` reader - sleep reject interrupt state"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` reader - SDIO idle interrupt state"]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `WDT` reader - RTC WDT interrupt state"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE` reader - enable touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_R = crate::BitReader;
#[doc = "Field `ULP_CP` reader - ULP-coprocessor interrupt state"]
pub type ULP_CP_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE` reader - touch done interrupt state"]
pub type TOUCH_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE` reader - touch active interrupt state"]
pub type TOUCH_ACTIVE_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE` reader - touch inactive interrupt state"]
pub type TOUCH_INACTIVE_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - brown out interrupt state"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - RTC main timer interrupt state"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `SARADC1` reader - saradc1 interrupt state"]
pub type SARADC1_R = crate::BitReader;
#[doc = "Field `TSENS` reader - tsens interrupt state"]
pub type TSENS_R = crate::BitReader;
#[doc = "Field `COCPU` reader - riscV cocpu interrupt state"]
pub type COCPU_R = crate::BitReader;
#[doc = "Field `SARADC2` reader - saradc2 interrupt state"]
pub type SARADC2_R = crate::BitReader;
#[doc = "Field `SWD` reader - super watch dog interrupt state"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` reader - xtal32k dead detection interrupt state"]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP` reader - cocpu trap interrupt state"]
pub type COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT` reader - Touch timeout interrupt state"]
pub type TOUCH_TIMEOUT_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - glitch_det_interrupt state"]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE` reader - touch approach mode loop interrupt state"]
pub type TOUCH_APPROACH_LOOP_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn touch_scan_done(&self) -> TOUCH_SCAN_DONE_R {
        TOUCH_SCAN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch done interrupt state"]
    #[inline(always)]
    pub fn touch_done(&self) -> TOUCH_DONE_R {
        TOUCH_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt state"]
    #[inline(always)]
    pub fn touch_active(&self) -> TOUCH_ACTIVE_R {
        TOUCH_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt state"]
    #[inline(always)]
    pub fn touch_inactive(&self) -> TOUCH_INACTIVE_R {
        TOUCH_INACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt state"]
    #[inline(always)]
    pub fn saradc1(&self) -> SARADC1_R {
        SARADC1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt state"]
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn cocpu(&self) -> COCPU_R {
        COCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt state"]
    #[inline(always)]
    pub fn saradc2(&self) -> SARADC2_R {
        SARADC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt state"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt state"]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt state"]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Touch timeout interrupt state"]
    #[inline(always)]
    pub fn touch_timeout(&self) -> TOUCH_TIMEOUT_R {
        TOUCH_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt state"]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn touch_approach_loop_done(&self) -> TOUCH_APPROACH_LOOP_DONE_R {
        TOUCH_APPROACH_LOOP_DONE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("slp_wakeup", &self.slp_wakeup().bit())
            .field("slp_reject", &self.slp_reject().bit())
            .field("sdio_idle", &self.sdio_idle().bit())
            .field("wdt", &self.wdt().bit())
            .field("touch_scan_done", &self.touch_scan_done().bit())
            .field("ulp_cp", &self.ulp_cp().bit())
            .field("touch_done", &self.touch_done().bit())
            .field("touch_active", &self.touch_active().bit())
            .field("touch_inactive", &self.touch_inactive().bit())
            .field("brown_out", &self.brown_out().bit())
            .field("main_timer", &self.main_timer().bit())
            .field("saradc1", &self.saradc1().bit())
            .field("tsens", &self.tsens().bit())
            .field("cocpu", &self.cocpu().bit())
            .field("saradc2", &self.saradc2().bit())
            .field("swd", &self.swd().bit())
            .field("xtal32k_dead", &self.xtal32k_dead().bit())
            .field("cocpu_trap", &self.cocpu_trap().bit())
            .field("touch_timeout", &self.touch_timeout().bit())
            .field("glitch_det", &self.glitch_det().bit())
            .field(
                "touch_approach_loop_done",
                &self.touch_approach_loop_done().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rtc interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
