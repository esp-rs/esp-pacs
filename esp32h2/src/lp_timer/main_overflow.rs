#[doc = "Register `MAIN_OVERFLOW` writer"]
pub type W = crate::W<MAIN_OVERFLOW_SPEC>;
#[doc = "Field `MAIN_TIMER_ALARM_LOAD` writer - need_des"]
pub type MAIN_TIMER_ALARM_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAIN_OVERFLOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_alarm_load(&mut self) -> MAIN_TIMER_ALARM_LOAD_W<MAIN_OVERFLOW_SPEC> {
        MAIN_TIMER_ALARM_LOAD_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_overflow::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAIN_OVERFLOW_SPEC;
impl crate::RegisterSpec for MAIN_OVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`main_overflow::W`](W) writer structure"]
impl crate::Writable for MAIN_OVERFLOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAIN_OVERFLOW to value 0"]
impl crate::Resettable for MAIN_OVERFLOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
