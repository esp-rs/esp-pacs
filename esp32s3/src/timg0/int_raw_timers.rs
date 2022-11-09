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
#[doc = "Field `T0_INT_RAW` reader - The raw interrupt status bit for the TIMG_T0_INT interrupt."]
pub type T0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `T1_INT_RAW` reader - The raw interrupt status bit for the TIMG_T1_INT interrupt."]
pub type T1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT_RAW` reader - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
pub type WDT_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0_int_raw(&self) -> T0_INT_RAW_R {
        T0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1_int_raw(&self) -> T1_INT_RAW_R {
        T1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> WDT_INT_RAW_R {
        WDT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_timers](index.html) module"]
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
