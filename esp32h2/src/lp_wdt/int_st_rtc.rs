#[doc = "Register `INT_ST_RTC` reader"]
pub type R = crate::R<INT_ST_RTC_SPEC>;
#[doc = "Field `SUPER_WDT_INT_ST` reader - need_des"]
pub type SUPER_WDT_INT_ST_R = crate::BitReader;
#[doc = "Field `WDT_INT_ST` reader - need_des"]
pub type WDT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn super_wdt_int_st(&self) -> SUPER_WDT_INT_ST_R {
        SUPER_WDT_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_RTC")
            .field(
                "super_wdt_int_st",
                &format_args!("{}", self.super_wdt_int_st().bit()),
            )
            .field("wdt_int_st", &format_args!("{}", self.wdt_int_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_rtc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_RTC_SPEC;
impl crate::RegisterSpec for INT_ST_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_rtc::R`](R) reader structure"]
impl crate::Readable for INT_ST_RTC_SPEC {}
#[doc = "`reset()` method sets INT_ST_RTC to value 0"]
impl crate::Resettable for INT_ST_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
