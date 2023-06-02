#[doc = "Register `TIME_HIGH1` reader"]
pub struct R(crate::R<TIME_HIGH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_HIGH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_HIGH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_HIGH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE1_HIGH` reader - Stores the higher 16 bits of RTC timer."]
pub type TIMER_VALUE1_HIGH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Stores the higher 16 bits of RTC timer."]
    #[inline(always)]
    pub fn timer_value1_high(&self) -> TIMER_VALUE1_HIGH_R {
        TIMER_VALUE1_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_HIGH1")
            .field(
                "timer_value1_high",
                &format_args!("{}", self.timer_value1_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_HIGH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Stores the higher 16 bits of RTC timer 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_high1](index.html) module"]
pub struct TIME_HIGH1_SPEC;
impl crate::RegisterSpec for TIME_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_high1::R](R) reader structure"]
impl crate::Readable for TIME_HIGH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME_HIGH1 to value 0"]
impl crate::Resettable for TIME_HIGH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
