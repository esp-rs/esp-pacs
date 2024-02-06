#[doc = "Register `INT_ST_RTC` reader"]
pub type R = crate::R<INT_ST_RTC_SPEC>;
#[doc = "Field `SLP_WAKEUP_INT_ST` reader - sleep wakeup interrupt state"]
pub type SLP_WAKEUP_INT_ST_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ST` reader - sleep reject interrupt state"]
pub type SLP_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - RTC WDT interrupt state"]
pub type WDT_INT_ST_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ST` reader - brown out interrupt state"]
pub type BROWN_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ST` reader - RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_ST_R = crate::BitReader;
#[doc = "Field `SWD_INT_ST` reader - super watch dog interrupt state"]
pub type SWD_INT_ST_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD_INT_ST` reader - xtal32k dead detection interrupt state"]
pub type XTAL32K_DEAD_INT_ST_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_ST` reader - glitch_det_interrupt state"]
pub type GLITCH_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL_INT_ST` reader - bbpll cal end interrupt state"]
pub type BBPLL_CAL_INT_ST_R = crate::BitReader;
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
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 19 - glitch_det_interrupt state"]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - bbpll cal end interrupt state"]
    #[inline(always)]
    pub fn bbpll_cal_int_st(&self) -> BBPLL_CAL_INT_ST_R {
        BBPLL_CAL_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
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
            .field("wdt_int_st", &format_args!("{}", self.wdt_int_st().bit()))
            .field(
                "brown_out_int_st",
                &format_args!("{}", self.brown_out_int_st().bit()),
            )
            .field(
                "main_timer_int_st",
                &format_args!("{}", self.main_timer_int_st().bit()),
            )
            .field("swd_int_st", &format_args!("{}", self.swd_int_st().bit()))
            .field(
                "xtal32k_dead_int_st",
                &format_args!("{}", self.xtal32k_dead_int_st().bit()),
            )
            .field(
                "glitch_det_int_st",
                &format_args!("{}", self.glitch_det_int_st().bit()),
            )
            .field(
                "bbpll_cal_int_st",
                &format_args!("{}", self.bbpll_cal_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_rtc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_RTC_SPEC;
impl crate::RegisterSpec for INT_ST_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_rtc::R`](R) reader structure"]
impl crate::Readable for INT_ST_RTC_SPEC {}
#[doc = "`reset()` method sets INT_ST_RTC to value 0"]
impl crate::Resettable for INT_ST_RTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
