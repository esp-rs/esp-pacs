#[doc = "Register `INT_ENA_RTC_W1TC` writer"]
pub struct W(crate::W<INT_ENA_RTC_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_W1TC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TC` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_ENA_W1TC` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA_W1TC` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_WDT_INT_ENA_W1TC` writer - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_ENA_W1TC` writer - enable touch scan done interrupt"]
pub type RTC_TOUCH_SCAN_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_ULP_CP_INT_ENA_W1TC` writer - enable ULP-coprocessor interrupt"]
pub type RTC_ULP_CP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_DONE_INT_ENA_W1TC` writer - enable touch done interrupt"]
pub type RTC_TOUCH_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_ENA_W1TC` writer - enable touch active interrupt"]
pub type RTC_TOUCH_ACTIVE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_ENA_W1TC` writer - enable touch inactive interrupt"]
pub type RTC_TOUCH_INACTIVE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA_W1TC` writer - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA_W1TC` writer - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC1_INT_ENA_W1TC` writer - enable saradc1 interrupt"]
pub type RTC_SARADC1_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TSENS_INT_ENA_W1TC` writer - enable tsens interrupt"]
pub type RTC_TSENS_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_INT_ENA_W1TC` writer - enable riscV cocpu interrupt"]
pub type RTC_COCPU_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC2_INT_ENA_W1TC` writer - enable saradc2 interrupt"]
pub type RTC_SARADC2_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_SWD_INT_ENA_W1TC` writer - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA_W1TC` writer - enable xtal32k_dead interrupt"]
pub type RTC_XTAL32K_DEAD_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_TRAP_INT_ENA_W1TC` writer - enable cocpu trap interrupt"]
pub type RTC_COCPU_TRAP_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_ENA_W1TC` writer - enable touch timeout interrupt"]
pub type RTC_TOUCH_TIMEOUT_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_GLITCH_DET_INT_ENA_W1TC` writer - enbale gitch det interrupt"]
pub type RTC_GLITCH_DET_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC` writer - enbale touch approach_loop done interrupt"]
pub type RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_W1TC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1tc(&mut self) -> SLP_WAKEUP_INT_ENA_W1TC_W<0> {
        SLP_WAKEUP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1tc(&mut self) -> SLP_REJECT_INT_ENA_W1TC_W<1> {
        SLP_REJECT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle_int_ena_w1tc(&mut self) -> SDIO_IDLE_INT_ENA_W1TC_W<2> {
        SDIO_IDLE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena_w1tc(&mut self) -> RTC_WDT_INT_ENA_W1TC_W<3> {
        RTC_WDT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_ena_w1tc(&mut self) -> RTC_TOUCH_SCAN_DONE_INT_ENA_W1TC_W<4> {
        RTC_TOUCH_SCAN_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_ena_w1tc(&mut self) -> RTC_ULP_CP_INT_ENA_W1TC_W<5> {
        RTC_ULP_CP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_done_int_ena_w1tc(&mut self) -> RTC_TOUCH_DONE_INT_ENA_W1TC_W<6> {
        RTC_TOUCH_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    pub fn rtc_touch_active_int_ena_w1tc(&mut self) -> RTC_TOUCH_ACTIVE_INT_ENA_W1TC_W<7> {
        RTC_TOUCH_ACTIVE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_ena_w1tc(&mut self) -> RTC_TOUCH_INACTIVE_INT_ENA_W1TC_W<8> {
        RTC_TOUCH_INACTIVE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena_w1tc(&mut self) -> RTC_BROWN_OUT_INT_ENA_W1TC_W<9> {
        RTC_BROWN_OUT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena_w1tc(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W1TC_W<10> {
        RTC_MAIN_TIMER_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc1_int_ena_w1tc(&mut self) -> RTC_SARADC1_INT_ENA_W1TC_W<11> {
        RTC_SARADC1_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    pub fn rtc_tsens_int_ena_w1tc(&mut self) -> RTC_TSENS_INT_ENA_W1TC_W<12> {
        RTC_TSENS_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_int_ena_w1tc(&mut self) -> RTC_COCPU_INT_ENA_W1TC_W<13> {
        RTC_COCPU_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc2_int_ena_w1tc(&mut self) -> RTC_SARADC2_INT_ENA_W1TC_W<14> {
        RTC_SARADC2_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena_w1tc(&mut self) -> RTC_SWD_INT_ENA_W1TC_W<15> {
        RTC_SWD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena_w1tc(&mut self) -> RTC_XTAL32K_DEAD_INT_ENA_W1TC_W<16> {
        RTC_XTAL32K_DEAD_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_ena_w1tc(&mut self) -> RTC_COCPU_TRAP_INT_ENA_W1TC_W<17> {
        RTC_COCPU_TRAP_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_ena_w1tc(&mut self) -> RTC_TOUCH_TIMEOUT_INT_ENA_W1TC_W<18> {
        RTC_TOUCH_TIMEOUT_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena_w1tc(&mut self) -> RTC_GLITCH_DET_INT_ENA_W1TC_W<19> {
        RTC_GLITCH_DET_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Bit 20 - enbale touch approach_loop done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_ena_w1tc(
        &mut self,
    ) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W<20> {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "oneset clr rtc interrupt enable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc_w1tc](index.html) module"]
pub struct INT_ENA_RTC_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc_w1tc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC_W1TC to value 0"]
impl crate::Resettable for INT_ENA_RTC_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
