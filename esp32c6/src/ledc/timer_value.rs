#[doc = "Register `TIMER%s_VALUE` reader"]
pub struct R(crate::R<TIMER_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_CNT` reader - This register stores the current counter value of timer %s."]
pub type TIMER_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores the current counter value of timer %s."]
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
        self.read().fmt(f)
    }
}
#[doc = "Timer %s current counter value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_value](index.html) module"]
pub struct TIMER_VALUE_SPEC;
impl crate::RegisterSpec for TIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_value::R](R) reader structure"]
impl crate::Readable for TIMER_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER%s_VALUE to value 0"]
impl crate::Resettable for TIMER_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
