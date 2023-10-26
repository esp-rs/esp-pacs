#[doc = "Register `MAIN_OVERFLOW` writer"]
pub type W = crate::W<MAIN_OVERFLOW_SPEC>;
#[doc = "Field `MAIN_TIMER_ALARM_LOAD` writer - need_des"]
pub type MAIN_TIMER_ALARM_LOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn main_timer_alarm_load(&mut self) -> MAIN_TIMER_ALARM_LOAD_W<MAIN_OVERFLOW_SPEC, 31> {
        MAIN_TIMER_ALARM_LOAD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_overflow::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAIN_OVERFLOW_SPEC;
impl crate::RegisterSpec for MAIN_OVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`main_overflow::W`](W) writer structure"]
impl crate::Writable for MAIN_OVERFLOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAIN_OVERFLOW to value 0"]
impl crate::Resettable for MAIN_OVERFLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
