#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` reader - SDIO idle interrupt raw"]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `WDT` reader - RTC WDT interrupt raw"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `TIME_VALID` reader - RTC time valid interrupt raw"]
pub type TIME_VALID_R = crate::BitReader;
#[doc = "Field `ULP_CP` reader - ULP-coprocessor interrupt raw"]
pub type ULP_CP_R = crate::BitReader;
#[doc = "Field `TOUCH` reader - touch interrupt raw"]
pub type TOUCH_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - brown out interrupt raw"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO idle interrupt raw"]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC time valid interrupt raw"]
    #[inline(always)]
    pub fn time_valid(&self) -> TIME_VALID_R {
        TIME_VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULP-coprocessor interrupt raw"]
    #[inline(always)]
    pub fn ulp_cp(&self) -> ULP_CP_R {
        ULP_CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - touch interrupt raw"]
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
