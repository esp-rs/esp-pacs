#[doc = "Register `INT_RAW_RTC` reader"]
pub struct R(crate::R<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW_RTC` writer"]
pub struct W(crate::W<INT_RAW_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_RTC_SPEC>;
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
impl From<crate::W<INT_RAW_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_RAW` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_RAW` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_RAW` reader - SDIO idle interrupt raw"]
pub type SDIO_IDLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `WDT_INT_RAW` reader - RTC WDT interrupt raw"]
pub type WDT_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE_INT_RAW` reader - enable touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ULP_CP_INT_RAW` reader - ULP-coprocessor interrupt raw"]
pub type ULP_CP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE_INT_RAW` reader - touch interrupt raw"]
pub type TOUCH_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE_INT_RAW` reader - touch active interrupt raw"]
pub type TOUCH_ACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE_INT_RAW` reader - touch inactive interrupt raw"]
pub type TOUCH_INACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_RAW` reader - brown out interrupt raw"]
pub type BROWN_OUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_RAW` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_INT_RAW_R = crate::BitReader;
#[doc = "Field `SARADC1_INT_RAW` reader - saradc1 interrupt raw"]
pub type SARADC1_INT_RAW_R = crate::BitReader;
#[doc = "Field `TSENS_INT_RAW` reader - tsens interrupt raw"]
pub type TSENS_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_INT_RAW` reader - riscV cocpu interrupt raw"]
pub type COCPU_INT_RAW_R = crate::BitReader;
#[doc = "Field `SARADC2_INT_RAW` reader - saradc2 interrupt raw"]
pub type SARADC2_INT_RAW_R = crate::BitReader;
#[doc = "Field `SWD_INT_RAW` reader - super watch dog interrupt raw"]
pub type SWD_INT_RAW_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD_INT_RAW` reader - xtal32k dead detection interrupt raw"]
pub type XTAL32K_DEAD_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP_INT_RAW` reader - cocpu trap interrupt raw"]
pub type COCPU_TRAP_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_INT_RAW` reader - touch timeout interrupt raw"]
pub type TOUCH_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_RAW` reader - glitch_det_interrupt_raw"]
pub type GLITCH_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_RAW` reader - touch approach mode loop interrupt raw"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_RAW` writer - touch approach mode loop interrupt raw"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, INT_RAW_RTC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup_int_raw(&self) -> SLP_WAKEUP_INT_RAW_R {
        SLP_WAKEUP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject_int_raw(&self) -> SLP_REJECT_INT_RAW_R {
        SLP_REJECT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&self) -> SDIO_IDLE_INT_RAW_R {
        SDIO_IDLE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn touch_scan_done_int_raw(&self) -> TOUCH_SCAN_DONE_INT_RAW_R {
        TOUCH_SCAN_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn ulp_cp_int_raw(&self) -> ULP_CP_INT_RAW_R {
        ULP_CP_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn touch_done_int_raw(&self) -> TOUCH_DONE_INT_RAW_R {
        TOUCH_DONE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt raw"]
    #[inline(always)]
    pub fn touch_active_int_raw(&self) -> TOUCH_ACTIVE_INT_RAW_R {
        TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt raw"]
    #[inline(always)]
    pub fn touch_inactive_int_raw(&self) -> TOUCH_INACTIVE_INT_RAW_R {
        TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out_int_raw(&self) -> BROWN_OUT_INT_RAW_R {
        BROWN_OUT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer_int_raw(&self) -> MAIN_TIMER_INT_RAW_R {
        MAIN_TIMER_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt raw"]
    #[inline(always)]
    pub fn saradc1_int_raw(&self) -> SARADC1_INT_RAW_R {
        SARADC1_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt raw"]
    #[inline(always)]
    pub fn tsens_int_raw(&self) -> TSENS_INT_RAW_R {
        TSENS_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt raw"]
    #[inline(always)]
    pub fn cocpu_int_raw(&self) -> COCPU_INT_RAW_R {
        COCPU_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt raw"]
    #[inline(always)]
    pub fn saradc2_int_raw(&self) -> SARADC2_INT_RAW_R {
        SARADC2_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn swd_int_raw(&self) -> SWD_INT_RAW_R {
        SWD_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt raw"]
    #[inline(always)]
    pub fn xtal32k_dead_int_raw(&self) -> XTAL32K_DEAD_INT_RAW_R {
        XTAL32K_DEAD_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt raw"]
    #[inline(always)]
    pub fn cocpu_trap_int_raw(&self) -> COCPU_TRAP_INT_RAW_R {
        COCPU_TRAP_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - touch timeout interrupt raw"]
    #[inline(always)]
    pub fn touch_timeout_int_raw(&self) -> TOUCH_TIMEOUT_INT_RAW_R {
        TOUCH_TIMEOUT_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt_raw"]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    pub fn touch_approach_loop_done_int_raw(&self) -> TOUCH_APPROACH_LOOP_DONE_INT_RAW_R {
        TOUCH_APPROACH_LOOP_DONE_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW_RTC")
            .field(
                "slp_wakeup_int_raw",
                &format_args!("{}", self.slp_wakeup_int_raw().bit()),
            )
            .field(
                "slp_reject_int_raw",
                &format_args!("{}", self.slp_reject_int_raw().bit()),
            )
            .field(
                "sdio_idle_int_raw",
                &format_args!("{}", self.sdio_idle_int_raw().bit()),
            )
            .field("wdt_int_raw", &format_args!("{}", self.wdt_int_raw().bit()))
            .field(
                "touch_scan_done_int_raw",
                &format_args!("{}", self.touch_scan_done_int_raw().bit()),
            )
            .field(
                "ulp_cp_int_raw",
                &format_args!("{}", self.ulp_cp_int_raw().bit()),
            )
            .field(
                "touch_done_int_raw",
                &format_args!("{}", self.touch_done_int_raw().bit()),
            )
            .field(
                "touch_active_int_raw",
                &format_args!("{}", self.touch_active_int_raw().bit()),
            )
            .field(
                "touch_inactive_int_raw",
                &format_args!("{}", self.touch_inactive_int_raw().bit()),
            )
            .field(
                "brown_out_int_raw",
                &format_args!("{}", self.brown_out_int_raw().bit()),
            )
            .field(
                "main_timer_int_raw",
                &format_args!("{}", self.main_timer_int_raw().bit()),
            )
            .field(
                "saradc1_int_raw",
                &format_args!("{}", self.saradc1_int_raw().bit()),
            )
            .field(
                "tsens_int_raw",
                &format_args!("{}", self.tsens_int_raw().bit()),
            )
            .field(
                "cocpu_int_raw",
                &format_args!("{}", self.cocpu_int_raw().bit()),
            )
            .field(
                "saradc2_int_raw",
                &format_args!("{}", self.saradc2_int_raw().bit()),
            )
            .field("swd_int_raw", &format_args!("{}", self.swd_int_raw().bit()))
            .field(
                "xtal32k_dead_int_raw",
                &format_args!("{}", self.xtal32k_dead_int_raw().bit()),
            )
            .field(
                "cocpu_trap_int_raw",
                &format_args!("{}", self.cocpu_trap_int_raw().bit()),
            )
            .field(
                "touch_timeout_int_raw",
                &format_args!("{}", self.touch_timeout_int_raw().bit()),
            )
            .field(
                "glitch_det_int_raw",
                &format_args!("{}", self.glitch_det_int_raw().bit()),
            )
            .field(
                "touch_approach_loop_done_int_raw",
                &format_args!("{}", self.touch_approach_loop_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - touch approach mode loop interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_loop_done_int_raw(&mut self) -> TOUCH_APPROACH_LOOP_DONE_INT_RAW_W<20> {
        TOUCH_APPROACH_LOOP_DONE_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_rtc](index.html) module"]
pub struct INT_RAW_RTC_SPEC;
impl crate::RegisterSpec for INT_RAW_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_rtc::R](R) reader structure"]
impl crate::Readable for INT_RAW_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw_rtc::W](W) writer structure"]
impl crate::Writable for INT_RAW_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW_RTC to value 0"]
impl crate::Resettable for INT_RAW_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
