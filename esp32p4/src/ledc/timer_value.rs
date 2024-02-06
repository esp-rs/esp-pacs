#[doc = "Register `TIMER%s_VALUE` reader"]
pub type R = crate::R<TIMER_VALUE_SPEC>;
#[doc = "Field `TIMER_CNT` reader - Represents the current counter value of timer %s."]
pub type TIMER_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Represents the current counter value of timer %s."]
    #[inline(always)]
    pub fn timer_cnt(&self) -> TIMER_CNT_R {
        TIMER_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_VALUE")
            .field("timer_cnt", &format_args!("{}", self.timer_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Timer %s current counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_VALUE_SPEC;
impl crate::RegisterSpec for TIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_value::R`](R) reader structure"]
impl crate::Readable for TIMER_VALUE_SPEC {}
#[doc = "`reset()` method sets TIMER%s_VALUE to value 0"]
impl crate::Resettable for TIMER_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
