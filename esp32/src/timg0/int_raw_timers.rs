#[doc = "Register `INT_RAW_TIMERS` reader"]
pub struct R(crate::R<INT_RAW_TIMERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_TIMERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_TIMERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_TIMERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_INT_RAW` reader - interrupt when timer0 alarm"]
pub type T0_INT_RAW_R = crate::BitReader;
#[doc = "Field `T1_INT_RAW` reader - interrupt when timer1 alarm"]
pub type T1_INT_RAW_R = crate::BitReader;
#[doc = "Field `WDT_INT_RAW` reader - Interrupt when an interrupt stage timeout"]
pub type WDT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LACT_INT_RAW` reader - "]
pub type LACT_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_raw(&self) -> T0_INT_RAW_R {
        T0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_raw(&self) -> T1_INT_RAW_R {
        T1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_raw(&self) -> LACT_INT_RAW_R {
        LACT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW_TIMERS")
            .field("t0_int_raw", &format_args!("{}", self.t0_int_raw().bit()))
            .field("t1_int_raw", &format_args!("{}", self.t1_int_raw().bit()))
            .field("wdt_int_raw", &format_args!("{}", self.wdt_int_raw().bit()))
            .field(
                "lact_int_raw",
                &format_args!("{}", self.lact_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_timers](index.html) module"]
pub struct INT_RAW_TIMERS_SPEC;
impl crate::RegisterSpec for INT_RAW_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_timers::R](R) reader structure"]
impl crate::Readable for INT_RAW_TIMERS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW_TIMERS to value 0"]
impl crate::Resettable for INT_RAW_TIMERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
