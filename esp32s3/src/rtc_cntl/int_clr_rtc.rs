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
pub type SLP_WAKEUP_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_CLR` writer - Clear sleep reject interrupt state"]
pub type SLP_REJECT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - Clear SDIO idle interrupt state"]
pub type SDIO_IDLE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_WDT_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type RTC_WDT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_CLR` writer - clear touch scan done interrupt raw"]
pub type RTC_TOUCH_SCAN_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_ULP_CP_INT_CLR` writer - Clear ULP-coprocessor interrupt state"]
pub type RTC_ULP_CP_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_DONE_INT_CLR` writer - Clear touch done interrupt state"]
pub type RTC_TOUCH_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_CLR` writer - Clear touch active interrupt state"]
pub type RTC_TOUCH_ACTIVE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_CLR` writer - Clear touch inactive interrupt state"]
pub type RTC_TOUCH_INACTIVE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_BROWN_OUT_INT_CLR` writer - Clear brown out interrupt state"]
pub type RTC_BROWN_OUT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_MAIN_TIMER_INT_CLR` writer - Clear RTC main timer interrupt state"]
pub type RTC_MAIN_TIMER_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC1_INT_CLR` writer - Clear saradc1 interrupt state"]
pub type RTC_SARADC1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TSENS_INT_CLR` writer - Clear tsens interrupt state"]
pub type RTC_TSENS_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_INT_CLR` writer - Clear riscV cocpu interrupt state"]
pub type RTC_COCPU_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC2_INT_CLR` writer - Clear saradc2 interrupt state"]
pub type RTC_SARADC2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SWD_INT_CLR` writer - Clear super watch dog interrupt state"]
pub type RTC_SWD_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_CLR` writer - Clear RTC WDT interrupt state"]
pub type RTC_XTAL32K_DEAD_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_TRAP_INT_CLR` writer - Clear cocpu trap interrupt state"]
pub type RTC_COCPU_TRAP_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_CLR` writer - Clear touch timeout interrupt state"]
pub type RTC_TOUCH_TIMEOUT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_GLITCH_DET_INT_CLR` writer - Clear glitch det interrupt state"]
pub type RTC_GLITCH_DET_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR` writer - cleartouch approach mode loop interrupt state"]
pub type RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_RTC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W<0> {
        SLP_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Clear sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W<1> {
        SLP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Clear SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W<2> {
        SDIO_IDLE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_wdt_int_clr(&mut self) -> RTC_WDT_INT_CLR_W<3> {
        RTC_WDT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - clear touch scan done interrupt raw"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_clr(&mut self) -> RTC_TOUCH_SCAN_DONE_INT_CLR_W<4> {
        RTC_TOUCH_SCAN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Clear ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_clr(&mut self) -> RTC_ULP_CP_INT_CLR_W<5> {
        RTC_ULP_CP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Clear touch done interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_done_int_clr(&mut self) -> RTC_TOUCH_DONE_INT_CLR_W<6> {
        RTC_TOUCH_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Clear touch active interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_active_int_clr(&mut self) -> RTC_TOUCH_ACTIVE_INT_CLR_W<7> {
        RTC_TOUCH_ACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Clear touch inactive interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_clr(&mut self) -> RTC_TOUCH_INACTIVE_INT_CLR_W<8> {
        RTC_TOUCH_INACTIVE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Clear brown out interrupt state"]
    #[inline(always)]
    pub fn rtc_brown_out_int_clr(&mut self) -> RTC_BROWN_OUT_INT_CLR_W<9> {
        RTC_BROWN_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Clear RTC main timer interrupt state"]
    #[inline(always)]
    pub fn rtc_main_timer_int_clr(&mut self) -> RTC_MAIN_TIMER_INT_CLR_W<10> {
        RTC_MAIN_TIMER_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Clear saradc1 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc1_int_clr(&mut self) -> RTC_SARADC1_INT_CLR_W<11> {
        RTC_SARADC1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Clear tsens interrupt state"]
    #[inline(always)]
    pub fn rtc_tsens_int_clr(&mut self) -> RTC_TSENS_INT_CLR_W<12> {
        RTC_TSENS_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Clear riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_int_clr(&mut self) -> RTC_COCPU_INT_CLR_W<13> {
        RTC_COCPU_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Clear saradc2 interrupt state"]
    #[inline(always)]
    pub fn rtc_saradc2_int_clr(&mut self) -> RTC_SARADC2_INT_CLR_W<14> {
        RTC_SARADC2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Clear super watch dog interrupt state"]
    #[inline(always)]
    pub fn rtc_swd_int_clr(&mut self) -> RTC_SWD_INT_CLR_W<15> {
        RTC_SWD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Clear RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_clr(&mut self) -> RTC_XTAL32K_DEAD_INT_CLR_W<16> {
        RTC_XTAL32K_DEAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Clear cocpu trap interrupt state"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_clr(&mut self) -> RTC_COCPU_TRAP_INT_CLR_W<17> {
        RTC_COCPU_TRAP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Clear touch timeout interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_clr(&mut self) -> RTC_TOUCH_TIMEOUT_INT_CLR_W<18> {
        RTC_TOUCH_TIMEOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Clear glitch det interrupt state"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_clr(&mut self) -> RTC_GLITCH_DET_INT_CLR_W<19> {
        RTC_GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - cleartouch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_clr(
        &mut self,
    ) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W<20> {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_CLR_W::new(self)
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
}
#[doc = "`reset()` method sets INT_CLR_RTC to value 0"]
impl crate::Resettable for INT_CLR_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
