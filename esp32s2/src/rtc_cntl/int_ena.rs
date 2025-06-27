#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - Enables interruption when the chip wakes up from sleep."]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - Enables interruption when the chip wakes up from sleep."]
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT` reader - Enables interruption when the chip rejects to go to sleep."]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - Enables interruption when the chip rejects to go to sleep."]
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IDLE` reader - Enables interruption when the SDIO idles."]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` writer - Enables interruption when the SDIO idles."]
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT` reader - Enables the RTC watchdog interrupt."]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - Enables the RTC watchdog interrupt."]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SCAN_DONE` reader - Enables interruption upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE` writer - Enables interruption upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP` reader - Enables the ULP co-processor interrupt."]
pub type ULP_CP_R = crate::BitReader;
#[doc = "Field `ULP_CP` writer - Enables the ULP co-processor interrupt."]
pub type ULP_CP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_DONE` reader - Enables interruption upon the completion of a single touch."]
pub type TOUCH_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE` writer - Enables interruption upon the completion of a single touch."]
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_ACTIVE` reader - Enables interruption when a touch is detected."]
pub type TOUCH_ACTIVE_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE` writer - Enables interruption when a touch is detected."]
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE` reader - Enables interruption when a touch is released."]
pub type TOUCH_INACTIVE_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE` writer - Enables interruption when a touch is released."]
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - Enables the brown out interrupt."]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - Enables the brown out interrupt."]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER` reader - Enables the RTC main timer interrupt."]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` writer - Enables the RTC main timer interrupt."]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC1` reader - Enables the SAR ADC 1 interrupt."]
pub type SARADC1_R = crate::BitReader;
#[doc = "Field `SARADC1` writer - Enables the SAR ADC 1 interrupt."]
pub type SARADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS` reader - Enables the touch sensor interrupt."]
pub type TSENS_R = crate::BitReader;
#[doc = "Field `TSENS` writer - Enables the touch sensor interrupt."]
pub type TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU` reader - Enables the ULP-RISCV interrupt."]
pub type COCPU_R = crate::BitReader;
#[doc = "Field `COCPU` writer - Enables the ULP-RISCV interrupt."]
pub type COCPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC2` reader - Enables the SAR ADC 2 interrupt."]
pub type SARADC2_R = crate::BitReader;
#[doc = "Field `SARADC2` writer - Enables the SAR ADC 2 interrupt."]
pub type SARADC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD` reader - Enables the super watchdog interrupt."]
pub type SWD_R = crate::BitReader;
#[doc = "Field `SWD` writer - Enables the super watchdog interrupt."]
pub type SWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL32K_DEAD` reader - Enables interruption when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` writer - Enables interruption when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_TRAP` reader - Enables interruption when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP` writer - Enables interruption when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_TIMEOUT` reader - Enables interruption when touch sensor times out."]
pub type TOUCH_TIMEOUT_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT` writer - Enables interruption when touch sensor times out."]
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET` reader - Enables interruption when a glitch is detected."]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` writer - Enables interruption when a glitch is detected."]
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done(&self) -> TOUCH_SCAN_DONE_R {
        TOUCH_SCAN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done(&self) -> TOUCH_DONE_R {
        TOUCH_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    pub fn touch_active(&self) -> TOUCH_ACTIVE_R {
        TOUCH_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive(&self) -> TOUCH_INACTIVE_R {
        TOUCH_INACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1(&self) -> SARADC1_R {
        SARADC1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu(&self) -> COCPU_R {
        COCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2(&self) -> SARADC2_R {
        SARADC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout(&self) -> TOUCH_TIMEOUT_R {
        TOUCH_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
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
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_ENA_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_SPEC> {
        WDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<INT_ENA_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_ENA_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<INT_ENA_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<INT_ENA_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<INT_ENA_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1(&mut self) -> SARADC1_W<INT_ENA_SPEC> {
        SARADC1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens(&mut self) -> TSENS_W<INT_ENA_SPEC> {
        TSENS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu(&mut self) -> COCPU_W<INT_ENA_SPEC> {
        COCPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2(&mut self) -> SARADC2_W<INT_ENA_SPEC> {
        SARADC2_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_SPEC> {
        SWD_W::new(self, 15)
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<INT_ENA_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<INT_ENA_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
}
#[doc = "RTC interrupt enabling register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
