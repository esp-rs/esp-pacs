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
#[doc = "Field `TIME_VALID` writer - Clear RTC time valid interrupt state"]
pub type TIME_VALID_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SAR` writer - Clear ULP-coprocessor interrupt state"]
pub type SAR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TOUCH` writer - Clear touch interrupt state"]
pub type TOUCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MAIN_TIMER` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
    #[doc = "Bit 4 - Clear RTC time valid interrupt state"]
    #[inline(always)]
    pub fn time_valid(&mut self) -> TIME_VALID_W<'_, INT_CLR_SPEC> {
        TIME_VALID_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W<'_, INT_CLR_SPEC> {
        SAR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear touch interrupt state"]
    #[inline(always)]
    pub fn touch(&mut self) -> TOUCH_W<'_, INT_CLR_SPEC> {
        TOUCH_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<'_, INT_CLR_SPEC> {
        BROWN_OUT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<'_, INT_CLR_SPEC> {
        MAIN_TIMER_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01ff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
