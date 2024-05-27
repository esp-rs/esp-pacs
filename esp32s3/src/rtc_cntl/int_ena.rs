///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `SLP_WAKEUP` reader - enable sleep wakeup interrupt
pub type SLP_WAKEUP_R = crate::BitReader;
///Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_REJECT` reader - enable sleep reject interrupt
pub type SLP_REJECT_R = crate::BitReader;
///Field `SLP_REJECT` writer - enable sleep reject interrupt
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_IDLE` reader - enable SDIO idle interrupt
pub type SDIO_IDLE_R = crate::BitReader;
///Field `SDIO_IDLE` writer - enable SDIO idle interrupt
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT` reader - enable RTC WDT interrupt
pub type WDT_R = crate::BitReader;
///Field `WDT` writer - enable RTC WDT interrupt
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_SCAN_DONE` reader - enable touch scan done interrupt
pub type TOUCH_SCAN_DONE_R = crate::BitReader;
///Field `TOUCH_SCAN_DONE` writer - enable touch scan done interrupt
pub type TOUCH_SCAN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_CP` reader - enable ULP-coprocessor interrupt
pub type ULP_CP_R = crate::BitReader;
///Field `ULP_CP` writer - enable ULP-coprocessor interrupt
pub type ULP_CP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_DONE` reader - enable touch done interrupt
pub type TOUCH_DONE_R = crate::BitReader;
///Field `TOUCH_DONE` writer - enable touch done interrupt
pub type TOUCH_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_ACTIVE` reader - enable touch active interrupt
pub type TOUCH_ACTIVE_R = crate::BitReader;
///Field `TOUCH_ACTIVE` writer - enable touch active interrupt
pub type TOUCH_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_INACTIVE` reader - enable touch inactive interrupt
pub type TOUCH_INACTIVE_R = crate::BitReader;
///Field `TOUCH_INACTIVE` writer - enable touch inactive interrupt
pub type TOUCH_INACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BROWN_OUT` reader - enable brown out interrupt
pub type BROWN_OUT_R = crate::BitReader;
///Field `BROWN_OUT` writer - enable brown out interrupt
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAIN_TIMER` reader - enable RTC main timer interrupt
pub type MAIN_TIMER_R = crate::BitReader;
///Field `MAIN_TIMER` writer - enable RTC main timer interrupt
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC1` reader - enable saradc1 interrupt
pub type SARADC1_R = crate::BitReader;
///Field `SARADC1` writer - enable saradc1 interrupt
pub type SARADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSENS` reader - enable tsens interrupt
pub type TSENS_R = crate::BitReader;
///Field `TSENS` writer - enable tsens interrupt
pub type TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU` reader - enable riscV cocpu interrupt
pub type COCPU_R = crate::BitReader;
///Field `COCPU` writer - enable riscV cocpu interrupt
pub type COCPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC2` reader - enable saradc2 interrupt
pub type SARADC2_R = crate::BitReader;
///Field `SARADC2` writer - enable saradc2 interrupt
pub type SARADC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWD` reader - enable super watch dog interrupt
pub type SWD_R = crate::BitReader;
///Field `SWD` writer - enable super watch dog interrupt
pub type SWD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL32K_DEAD` reader - enable xtal32k_dead interrupt
pub type XTAL32K_DEAD_R = crate::BitReader;
///Field `XTAL32K_DEAD` writer - enable xtal32k_dead interrupt
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_TRAP` reader - enable cocpu trap interrupt
pub type COCPU_TRAP_R = crate::BitReader;
///Field `COCPU_TRAP` writer - enable cocpu trap interrupt
pub type COCPU_TRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_TIMEOUT` reader - enable touch timeout interrupt
pub type TOUCH_TIMEOUT_R = crate::BitReader;
///Field `TOUCH_TIMEOUT` writer - enable touch timeout interrupt
pub type TOUCH_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GLITCH_DET` reader - enbale gitch det interrupt
pub type GLITCH_DET_R = crate::BitReader;
///Field `GLITCH_DET` writer - enbale gitch det interrupt
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_APPROACH_LOOP_DONE` reader - touch approach mode loop interrupt
pub type TOUCH_APPROACH_LOOP_DONE_R = crate::BitReader;
///Field `TOUCH_APPROACH_LOOP_DONE` writer - touch approach mode loop interrupt
pub type TOUCH_APPROACH_LOOP_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable sleep wakeup interrupt
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable sleep reject interrupt
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable SDIO idle interrupt
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable RTC WDT interrupt
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable touch scan done interrupt
    #[inline(always)]
    pub fn touch_scan_done(&self) -> TOUCH_SCAN_DONE_R {
        TOUCH_SCAN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable ULP-coprocessor interrupt
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable touch done interrupt
    #[inline(always)]
    pub fn touch_done(&self) -> TOUCH_DONE_R {
        TOUCH_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable touch active interrupt
    #[inline(always)]
    pub fn touch_active(&self) -> TOUCH_ACTIVE_R {
        TOUCH_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - enable touch inactive interrupt
    #[inline(always)]
    pub fn touch_inactive(&self) -> TOUCH_INACTIVE_R {
        TOUCH_INACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - enable brown out interrupt
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - enable RTC main timer interrupt
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - enable saradc1 interrupt
    #[inline(always)]
    pub fn saradc1(&self) -> SARADC1_R {
        SARADC1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - enable tsens interrupt
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - enable riscV cocpu interrupt
    #[inline(always)]
    pub fn cocpu(&self) -> COCPU_R {
        COCPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - enable saradc2 interrupt
    #[inline(always)]
    pub fn saradc2(&self) -> SARADC2_R {
        SARADC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - enable super watch dog interrupt
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - enable xtal32k_dead interrupt
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - enable cocpu trap interrupt
    #[inline(always)]
    pub fn cocpu_trap(&self) -> COCPU_TRAP_R {
        COCPU_TRAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - enable touch timeout interrupt
    #[inline(always)]
    pub fn touch_timeout(&self) -> TOUCH_TIMEOUT_R {
        TOUCH_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - enbale gitch det interrupt
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - touch approach mode loop interrupt
    #[inline(always)]
    pub fn touch_approach_loop_done(&self) -> TOUCH_APPROACH_LOOP_DONE_R {
        TOUCH_APPROACH_LOOP_DONE_R::new(((self.bits >> 20) & 1) != 0)
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
            .field("touch_approach_loop_done", &self.touch_approach_loop_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable sleep wakeup interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    ///Bit 1 - enable sleep reject interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    ///Bit 2 - enable SDIO idle interrupt
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_ENA_SPEC> {
        SDIO_IDLE_W::new(self, 2)
    }
    ///Bit 3 - enable RTC WDT interrupt
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_SPEC> {
        WDT_W::new(self, 3)
    }
    ///Bit 4 - enable touch scan done interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done(&mut self) -> TOUCH_SCAN_DONE_W<INT_ENA_SPEC> {
        TOUCH_SCAN_DONE_W::new(self, 4)
    }
    ///Bit 5 - enable ULP-coprocessor interrupt
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_ENA_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    ///Bit 6 - enable touch done interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_done(&mut self) -> TOUCH_DONE_W<INT_ENA_SPEC> {
        TOUCH_DONE_W::new(self, 6)
    }
    ///Bit 7 - enable touch active interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_active(&mut self) -> TOUCH_ACTIVE_W<INT_ENA_SPEC> {
        TOUCH_ACTIVE_W::new(self, 7)
    }
    ///Bit 8 - enable touch inactive interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive(&mut self) -> TOUCH_INACTIVE_W<INT_ENA_SPEC> {
        TOUCH_INACTIVE_W::new(self, 8)
    }
    ///Bit 9 - enable brown out interrupt
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    ///Bit 10 - enable RTC main timer interrupt
    #[inline(always)]
    #[must_use]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    ///Bit 11 - enable saradc1 interrupt
    #[inline(always)]
    #[must_use]
    pub fn saradc1(&mut self) -> SARADC1_W<INT_ENA_SPEC> {
        SARADC1_W::new(self, 11)
    }
    ///Bit 12 - enable tsens interrupt
    #[inline(always)]
    #[must_use]
    pub fn tsens(&mut self) -> TSENS_W<INT_ENA_SPEC> {
        TSENS_W::new(self, 12)
    }
    ///Bit 13 - enable riscV cocpu interrupt
    #[inline(always)]
    #[must_use]
    pub fn cocpu(&mut self) -> COCPU_W<INT_ENA_SPEC> {
        COCPU_W::new(self, 13)
    }
    ///Bit 14 - enable saradc2 interrupt
    #[inline(always)]
    #[must_use]
    pub fn saradc2(&mut self) -> SARADC2_W<INT_ENA_SPEC> {
        SARADC2_W::new(self, 14)
    }
    ///Bit 15 - enable super watch dog interrupt
    #[inline(always)]
    #[must_use]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_SPEC> {
        SWD_W::new(self, 15)
    }
    ///Bit 16 - enable xtal32k_dead interrupt
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    ///Bit 17 - enable cocpu trap interrupt
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap(&mut self) -> COCPU_TRAP_W<INT_ENA_SPEC> {
        COCPU_TRAP_W::new(self, 17)
    }
    ///Bit 18 - enable touch timeout interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout(&mut self) -> TOUCH_TIMEOUT_W<INT_ENA_SPEC> {
        TOUCH_TIMEOUT_W::new(self, 18)
    }
    ///Bit 19 - enbale gitch det interrupt
    #[inline(always)]
    #[must_use]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    ///Bit 20 - touch approach mode loop interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done(&mut self) -> TOUCH_APPROACH_LOOP_DONE_W<INT_ENA_SPEC> {
        TOUCH_APPROACH_LOOP_DONE_W::new(self, 20)
    }
}
/**configure rtc interrupt register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
