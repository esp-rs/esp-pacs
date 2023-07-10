#[doc = "Register `MAIN_OVERFLOW` writer"]
pub struct W(crate::W<MAIN_OVERFLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAIN_OVERFLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MAIN_OVERFLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAIN_OVERFLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_TIMER_ALARM_LOAD` writer - need_des"]
pub type MAIN_TIMER_ALARM_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, MAIN_OVERFLOW_SPEC, O>;
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
    pub fn main_timer_alarm_load(&mut self) -> MAIN_TIMER_ALARM_LOAD_W<31> {
        MAIN_TIMER_ALARM_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_overflow](index.html) module"]
pub struct MAIN_OVERFLOW_SPEC;
impl crate::RegisterSpec for MAIN_OVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [main_overflow::W](W) writer structure"]
impl crate::Writable for MAIN_OVERFLOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAIN_OVERFLOW to value 0"]
impl crate::Resettable for MAIN_OVERFLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
