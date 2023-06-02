#[doc = "Register `INT_CLR_RTC` writer"]
pub struct W(crate::W<INT_CLR_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_CLR` writer - Clear sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clear SDIO idle interrupt state"]
pub type SDIO_IDLE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type WDT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_CLR` writer - clear touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `ULP_CP_INT_CLR` writer - Clear ULP-coprocessor interrupt state"]
pub type ULP_CP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_DONE_INT_CLR` writer - Clear touch done interrupt state"]
pub type TOUCH_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_ACTIVE_INT_CLR` writer - Clear touch active interrupt state"]
pub type TOUCH_ACTIVE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_INACTIVE_INT_CLR` writer - Clear touch inactive interrupt state"]
pub type TOUCH_INACTIVE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type BROWN_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SARADC1_INT_CLR` writer - Clear saradc1 interrupt state"]
pub type SARADC1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TSENS_INT_CLR` writer - Clear tsens interrupt state"]
pub type TSENS_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `COCPU_INT_CLR` writer - Clear riscV cocpu interrupt state"]
pub type COCPU_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SARADC2_INT_CLR` writer - Clear saradc2 interrupt state"]
pub type SARADC2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub type SWD_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `XTAL32K_DEAD_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type XTAL32K_DEAD_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `COCPU_TRAP_INT_CLR` writer - Clear cocpu trap interrupt state"]
pub type COCPU_TRAP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_TIMEOUT_INT_CLR` writer - Clear touch timeout interrupt state"]
pub type TOUCH_TIMEOUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Clear glitch det interrupt state"]
pub type GLITCH_DET_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_CLR` writer - cleartouch approach mode loop interrupt state"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, INT_CLR_RTC_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W<0> {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W<1> {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W<2> {
        SDIO_IDLE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W<3> {
        WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - clear touch scan done interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done_int_clr(&mut self) -> TOUCH_SCAN_DONE_INT_CLR_W<4> {
        TOUCH_SCAN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_int_clr(&mut self) -> ULP_CP_INT_CLR_W<5> {
        ULP_CP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Clear touch done interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_done_int_clr(&mut self) -> TOUCH_DONE_INT_CLR_W<6> {
        TOUCH_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Clear touch active interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_active_int_clr(&mut self) -> TOUCH_ACTIVE_INT_CLR_W<7> {
        TOUCH_ACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Clear touch inactive interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_int_clr(&mut self) -> TOUCH_INACTIVE_INT_CLR_W<8> {
        TOUCH_INACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<9> {
        BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W<10> {
        MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Clear saradc1 interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn saradc1_int_clr(&mut self) -> SARADC1_INT_CLR_W<11> {
        SARADC1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Clear tsens interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_int_clr(&mut self) -> TSENS_INT_CLR_W<12> {
        TSENS_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Clear riscV cocpu interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_int_clr(&mut self) -> COCPU_INT_CLR_W<13> {
        COCPU_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Clear saradc2 interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn saradc2_int_clr(&mut self) -> SARADC2_INT_CLR_W<14> {
        SARADC2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W<15> {
        SWD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_clr(&mut self) -> XTAL32K_DEAD_INT_CLR_W<16> {
        XTAL32K_DEAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Clear cocpu trap interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap_int_clr(&mut self) -> COCPU_TRAP_INT_CLR_W<17> {
        COCPU_TRAP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Clear touch timeout interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_int_clr(&mut self) -> TOUCH_TIMEOUT_INT_CLR_W<18> {
        TOUCH_TIMEOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W<19> {
        GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - cleartouch approach mode loop interrupt state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done_int_clr(&mut self) -> TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<20> {
        TOUCH_APPROACH_LOOP_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr_rtc](index.html) module"]
pub struct INT_CLR_RTC_SPEC;
impl crate::RegisterSpec for INT_CLR_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr_rtc::W](W) writer structure"]
impl crate::Writable for INT_CLR_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
