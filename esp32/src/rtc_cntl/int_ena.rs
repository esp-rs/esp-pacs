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
///Field `TIME_VALID` reader - enable RTC time valid interrupt
pub type TIME_VALID_R = crate::BitReader;
///Field `TIME_VALID` writer - enable RTC time valid interrupt
pub type TIME_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_CP` reader - enable ULP-coprocessor interrupt
pub type ULP_CP_R = crate::BitReader;
///Field `ULP_CP` writer - enable ULP-coprocessor interrupt
pub type ULP_CP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH` reader - enable touch interrupt
pub type TOUCH_R = crate::BitReader;
///Field `TOUCH` writer - enable touch interrupt
pub type TOUCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BROWN_OUT` reader - enable brown out interrupt
pub type BROWN_OUT_R = crate::BitReader;
///Field `BROWN_OUT` writer - enable brown out interrupt
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MAIN_TIMER` reader - enable RTC main timer interrupt
pub type MAIN_TIMER_R = crate::BitReader;
///Field `MAIN_TIMER` writer - enable RTC main timer interrupt
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - enable RTC time valid interrupt
    #[inline(always)]
    pub fn time_valid(&self) -> TIME_VALID_R {
        TIME_VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable ULP-coprocessor interrupt
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable touch interrupt
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable brown out interrupt
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - enable RTC main timer interrupt
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("time_valid", &self.time_valid())
            .field("ulp_cp", &self.ulp_cp())
            .field("touch", &self.touch())
            .field("brown_out", &self.brown_out())
            .field("main_timer", &self.main_timer())
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
    ///Bit 4 - enable RTC time valid interrupt
    #[inline(always)]
    #[must_use]
    pub fn time_valid(&mut self) -> TIME_VALID_W<INT_ENA_SPEC> {
        TIME_VALID_W::new(self, 4)
    }
    ///Bit 5 - enable ULP-coprocessor interrupt
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp(&mut self) -> ULP_CP_W<INT_ENA_SPEC> {
        ULP_CP_W::new(self, 5)
    }
    ///Bit 6 - enable touch interrupt
    #[inline(always)]
    #[must_use]
    pub fn touch(&mut self) -> TOUCH_W<INT_ENA_SPEC> {
        TOUCH_W::new(self, 6)
    }
    ///Bit 7 - enable brown out interrupt
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_SPEC> {
        BROWN_OUT_W::new(self, 7)
    }
    ///Bit 8 - enable RTC main timer interrupt
    #[inline(always)]
    #[must_use]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_SPEC> {
        MAIN_TIMER_W::new(self, 8)
    }
}
/**

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
