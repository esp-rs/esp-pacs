#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` reader - SDIO idle interrupt raw"]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `WDT` reader - RTC WDT interrupt raw"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE` reader - enable touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_R = crate::BitReader;
#[doc = "Field `ULP_CP` reader - ULP-coprocessor interrupt raw"]
pub type ULP_CP_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE` reader - touch interrupt raw"]
pub type TOUCH_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE` reader - touch active interrupt raw"]
pub type TOUCH_ACTIVE_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE` reader - touch inactive interrupt raw"]
pub type TOUCH_INACTIVE_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - brown out interrupt raw"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `SARADC1` reader - saradc1 interrupt raw"]
pub type SARADC1_R = crate::BitReader;
#[doc = "Field `TSENS` reader - tsens interrupt raw"]
pub type TSENS_R = crate::BitReader;
#[doc = "Field `COCPU` reader - riscV cocpu interrupt raw"]
pub type COCPU_R = crate::BitReader;
#[doc = "Field `SARADC2` reader - saradc2 interrupt raw"]
pub type SARADC2_R = crate::BitReader;
#[doc = "Field `SWD` reader - super watch dog interrupt raw"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` reader - xtal32k dead detection interrupt raw"]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP` reader - cocpu trap interrupt raw"]
pub type COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT` reader - touch timeout interrupt raw"]
pub type TOUCH_TIMEOUT_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - glitch_det_interrupt_raw"]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE` reader - touch approach mode loop interrupt raw"]
pub type TOUCH_APPROACH_LOOP_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE` writer - touch approach mode loop interrupt raw"]
pub type TOUCH_APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn touch_scan_done(&self) -> TOUCH_SCAN_DONE_R {
        TOUCH_SCAN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn touch_done(&self) -> TOUCH_DONE_R {
        TOUCH_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt raw"]
    #[inline(always)]
    pub fn touch_active(&self) -> TOUCH_ACTIVE_R {
        TOUCH_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt raw"]
    #[inline(always)]
    pub fn touch_inactive(&self) -> TOUCH_INACTIVE_R {
        TOUCH_INACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt raw"]
    #[inline(always)]
    pub fn saradc1(&self) -> SARADC1_R {
        SARADC1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt raw"]
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt raw"]
    #[inline(always)]
    pub fn cocpu(&self) -> COCPU_R {
        COCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt raw"]
    #[inline(always)]
    pub fn saradc2(&self) -> SARADC2_R {
        SARADC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt raw"]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt raw"]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - touch timeout interrupt raw"]
    #[inline(always)]
    pub fn touch_timeout(&self) -> TOUCH_TIMEOUT_R {
        TOUCH_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt_raw"]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    pub fn touch_approach_loop_done(&self) -> TOUCH_APPROACH_LOOP_DONE_R {
        TOUCH_APPROACH_LOOP_DONE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("slp_wakeup", &self.slp_wakeup())
            .field("slp_reject", &self.slp_reject())
            .field("sdio_idle", &self.sdio_idle())
            .field("wdt", &self.wdt())
            .field("touch_scan_done", &self.touch_scan_done())
            .field("ulp_cp", &self.ulp_cp())
            .field("touch_done", &self.touch_done())
            .field("touch_active", &self.touch_active())
            .field("touch_inactive", &self.touch_inactive())
            .field("brown_out", &self.brown_out())
            .field("main_timer", &self.main_timer())
            .field("saradc1", &self.saradc1())
            .field("tsens", &self.tsens())
            .field("cocpu", &self.cocpu())
            .field("saradc2", &self.saradc2())
            .field("swd", &self.swd())
            .field("xtal32k_dead", &self.xtal32k_dead())
            .field("cocpu_trap", &self.cocpu_trap())
            .field("touch_timeout", &self.touch_timeout())
            .field("glitch_det", &self.glitch_det())
            .field("touch_approach_loop_done", &self.touch_approach_loop_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done(&mut self) -> TOUCH_APPROACH_LOOP_DONE_W<INT_RAW_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_W::new(self, 20)
    }
}
#[doc = "rtc interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
