#[doc = "Register `INT_ST_RTC` reader"]
pub struct R(crate::R<INT_ST_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ST` reader - sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_ST_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ST` reader - sleep reject interrupt state"]
pub type SLP_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_ST` reader - SDIO idle interrupt state"]
pub type SDIO_IDLE_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - RTC WDT interrupt state"]
pub type WDT_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ST` reader - enable touch scan done interrupt raw"]
pub type TOUCH_SCAN_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ULP_CP_INT_ST` reader - ULP-coprocessor interrupt state"]
pub type ULP_CP_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE_INT_ST` reader - touch done interrupt state"]
pub type TOUCH_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE_INT_ST` reader - touch active interrupt state"]
pub type TOUCH_ACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE_INT_ST` reader - touch inactive interrupt state"]
pub type TOUCH_INACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ST` reader - brown out interrupt state"]
pub type BROWN_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ST` reader - RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_ST_R = crate::BitReader;
#[doc = "Field `SARADC1_INT_ST` reader - saradc1 interrupt state"]
pub type SARADC1_INT_ST_R = crate::BitReader;
#[doc = "Field `TSENS_INT_ST` reader - tsens interrupt state"]
pub type TSENS_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_INT_ST` reader - riscV cocpu interrupt state"]
pub type COCPU_INT_ST_R = crate::BitReader;
#[doc = "Field `SARADC2_INT_ST` reader - saradc2 interrupt state"]
pub type SARADC2_INT_ST_R = crate::BitReader;
#[doc = "Field `SWD_INT_ST` reader - super watch dog interrupt state"]
pub type SWD_INT_ST_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD_INT_ST` reader - xtal32k dead detection interrupt state"]
pub type XTAL32K_DEAD_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP_INT_ST` reader - cocpu trap interrupt state"]
pub type COCPU_TRAP_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_INT_ST` reader - Touch timeout interrupt state"]
pub type TOUCH_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_ST` reader - glitch_det_interrupt state"]
pub type GLITCH_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_LOOP_DONE_INT_ST` reader - touch approach mode loop interrupt state"]
pub type TOUCH_APPROACH_LOOP_DONE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&self) -> SLP_WAKEUP_INT_ST_R {
        SLP_WAKEUP_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_st(&self) -> SLP_REJECT_INT_ST_R {
        SLP_REJECT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt state"]
    #[inline(always)]
    pub fn sdio_idle_int_st(&self) -> SDIO_IDLE_INT_ST_R {
        SDIO_IDLE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt raw"]
    #[inline(always)]
    pub fn touch_scan_done_int_st(&self) -> TOUCH_SCAN_DONE_INT_ST_R {
        TOUCH_SCAN_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn ulp_cp_int_st(&self) -> ULP_CP_INT_ST_R {
        ULP_CP_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch done interrupt state"]
    #[inline(always)]
    pub fn touch_done_int_st(&self) -> TOUCH_DONE_INT_ST_R {
        TOUCH_DONE_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - touch active interrupt state"]
    #[inline(always)]
    pub fn touch_active_int_st(&self) -> TOUCH_ACTIVE_INT_ST_R {
        TOUCH_ACTIVE_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - touch inactive interrupt state"]
    #[inline(always)]
    pub fn touch_inactive_int_st(&self) -> TOUCH_INACTIVE_INT_ST_R {
        TOUCH_INACTIVE_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_st(&self) -> MAIN_TIMER_INT_ST_R {
        MAIN_TIMER_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - saradc1 interrupt state"]
    #[inline(always)]
    pub fn saradc1_int_st(&self) -> SARADC1_INT_ST_R {
        SARADC1_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - tsens interrupt state"]
    #[inline(always)]
    pub fn tsens_int_st(&self) -> TSENS_INT_ST_R {
        TSENS_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - riscV cocpu interrupt state"]
    #[inline(always)]
    pub fn cocpu_int_st(&self) -> COCPU_INT_ST_R {
        COCPU_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - saradc2 interrupt state"]
    #[inline(always)]
    pub fn saradc2_int_st(&self) -> SARADC2_INT_ST_R {
        SARADC2_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt state"]
    #[inline(always)]
    pub fn swd_int_st(&self) -> SWD_INT_ST_R {
        SWD_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt state"]
    #[inline(always)]
    pub fn xtal32k_dead_int_st(&self) -> XTAL32K_DEAD_INT_ST_R {
        XTAL32K_DEAD_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - cocpu trap interrupt state"]
    #[inline(always)]
    pub fn cocpu_trap_int_st(&self) -> COCPU_TRAP_INT_ST_R {
        COCPU_TRAP_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Touch timeout interrupt state"]
    #[inline(always)]
    pub fn touch_timeout_int_st(&self) -> TOUCH_TIMEOUT_INT_ST_R {
        TOUCH_TIMEOUT_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt state"]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt state"]
    #[inline(always)]
    pub fn touch_approach_loop_done_int_st(&self) -> TOUCH_APPROACH_LOOP_DONE_INT_ST_R {
        TOUCH_APPROACH_LOOP_DONE_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_RTC")
            .field(
                "slp_wakeup_int_st",
                &format_args!("{}", self.slp_wakeup_int_st().bit()),
            )
            .field(
                "slp_reject_int_st",
                &format_args!("{}", self.slp_reject_int_st().bit()),
            )
            .field(
                "sdio_idle_int_st",
                &format_args!("{}", self.sdio_idle_int_st().bit()),
            )
            .field("wdt_int_st", &format_args!("{}", self.wdt_int_st().bit()))
            .field(
                "touch_scan_done_int_st",
                &format_args!("{}", self.touch_scan_done_int_st().bit()),
            )
            .field(
                "ulp_cp_int_st",
                &format_args!("{}", self.ulp_cp_int_st().bit()),
            )
            .field(
                "touch_done_int_st",
                &format_args!("{}", self.touch_done_int_st().bit()),
            )
            .field(
                "touch_active_int_st",
                &format_args!("{}", self.touch_active_int_st().bit()),
            )
            .field(
                "touch_inactive_int_st",
                &format_args!("{}", self.touch_inactive_int_st().bit()),
            )
            .field(
                "brown_out_int_st",
                &format_args!("{}", self.brown_out_int_st().bit()),
            )
            .field(
                "main_timer_int_st",
                &format_args!("{}", self.main_timer_int_st().bit()),
            )
            .field(
                "saradc1_int_st",
                &format_args!("{}", self.saradc1_int_st().bit()),
            )
            .field(
                "tsens_int_st",
                &format_args!("{}", self.tsens_int_st().bit()),
            )
            .field(
                "cocpu_int_st",
                &format_args!("{}", self.cocpu_int_st().bit()),
            )
            .field(
                "saradc2_int_st",
                &format_args!("{}", self.saradc2_int_st().bit()),
            )
            .field("swd_int_st", &format_args!("{}", self.swd_int_st().bit()))
            .field(
                "xtal32k_dead_int_st",
                &format_args!("{}", self.xtal32k_dead_int_st().bit()),
            )
            .field(
                "cocpu_trap_int_st",
                &format_args!("{}", self.cocpu_trap_int_st().bit()),
            )
            .field(
                "touch_timeout_int_st",
                &format_args!("{}", self.touch_timeout_int_st().bit()),
            )
            .field(
                "glitch_det_int_st",
                &format_args!("{}", self.glitch_det_int_st().bit()),
            )
            .field(
                "touch_approach_loop_done_int_st",
                &format_args!("{}", self.touch_approach_loop_done_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "rtc interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_rtc](index.html) module"]
pub struct INT_ST_RTC_SPEC;
impl crate::RegisterSpec for INT_ST_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_rtc::R](R) reader structure"]
impl crate::Readable for INT_ST_RTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_RTC to value 0"]
impl crate::Resettable for INT_ST_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
