#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
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
#[doc = "Field `TIME_VALID_INT_ST` reader - RTC time valid interrupt state"]
pub type TIME_VALID_INT_ST_R = crate::BitReader;
#[doc = "Field `SAR_INT_ST` reader - ULP-coprocessor interrupt state"]
pub type SAR_INT_ST_R = crate::BitReader;
#[doc = "Field `TOUCH_INT_ST` reader - touch interrupt state"]
pub type TOUCH_INT_ST_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ST` reader - brown out interrupt state"]
pub type BROWN_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ST` reader - RTC main timer interrupt state"]
pub type MAIN_TIMER_INT_ST_R = crate::BitReader;
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
    #[doc = "Bit 4 - RTC time valid interrupt state"]
    #[inline(always)]
    pub fn time_valid_int_st(&self) -> TIME_VALID_INT_ST_R {
        TIME_VALID_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt state"]
    #[inline(always)]
    pub fn sar_int_st(&self) -> SAR_INT_ST_R {
        SAR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch interrupt state"]
    #[inline(always)]
    pub fn touch_int_st(&self) -> TOUCH_INT_ST_R {
        TOUCH_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - brown out interrupt state"]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn main_timer_int_st(&self) -> MAIN_TIMER_INT_ST_R {
        MAIN_TIMER_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
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
                "time_valid_int_st",
                &format_args!("{}", self.time_valid_int_st().bit()),
            )
            .field("sar_int_st", &format_args!("{}", self.sar_int_st().bit()))
            .field(
                "touch_int_st",
                &format_args!("{}", self.touch_int_st().bit()),
            )
            .field(
                "brown_out_int_st",
                &format_args!("{}", self.brown_out_int_st().bit()),
            )
            .field(
                "main_timer_int_st",
                &format_args!("{}", self.main_timer_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
