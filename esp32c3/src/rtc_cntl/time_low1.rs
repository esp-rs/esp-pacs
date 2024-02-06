#[doc = "Register `TIME_LOW1` reader"]
pub type R = crate::R<TIME_LOW1_SPEC>;
#[doc = "Field `TIMER_VALUE1_LOW` reader - RTC timer low 32 bits"]
pub type TIMER_VALUE1_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RTC timer low 32 bits"]
    #[inline(always)]
    pub fn timer_value1_low(&self) -> TIMER_VALUE1_LOW_R {
        TIMER_VALUE1_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_LOW1")
            .field(
                "timer_value1_low",
                &format_args!("{}", self.timer_value1_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_LOW1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_LOW1_SPEC;
impl crate::RegisterSpec for TIME_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_low1::R`](R) reader structure"]
impl crate::Readable for TIME_LOW1_SPEC {}
#[doc = "`reset()` method sets TIME_LOW1 to value 0"]
impl crate::Resettable for TIME_LOW1_SPEC {
    const RESET_VALUE: u32 = 0;
}
