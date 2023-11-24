#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clear SDIO idle interrupt state"]
pub type SDIO_IDLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_VALID_INT_CLR` writer - Clear RTC time valid interrupt state"]
pub type TIME_VALID_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_INT_CLR` writer - Clear ULP-coprocessor interrupt state"]
pub type SAR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_INT_CLR` writer - Clear touch interrupt state"]
pub type TOUCH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W<INT_CLR_SPEC> {
        SLP_WAKEUP_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W<INT_CLR_SPEC> {
        SLP_REJECT_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W<INT_CLR_SPEC> {
        SDIO_IDLE_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<INT_CLR_SPEC> {
        WDT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear RTC time valid interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn time_valid_int_clr(&mut self) -> TIME_VALID_INT_CLR_W<INT_CLR_SPEC> {
        TIME_VALID_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn sar_int_clr(&mut self) -> SAR_INT_CLR_W<INT_CLR_SPEC> {
        SAR_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear touch interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_int_clr(&mut self) -> TOUCH_INT_CLR_W<INT_CLR_SPEC> {
        TOUCH_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear brown out interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<INT_CLR_SPEC> {
        BROWN_OUT_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W<INT_CLR_SPEC> {
        MAIN_TIMER_INT_CLR_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
