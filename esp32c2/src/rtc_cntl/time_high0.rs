#[doc = "Register `TIME_HIGH0` reader"]
pub struct R(crate::R<TIME_HIGH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_HIGH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_HIGH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_HIGH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME_HIGH0` writer"]
pub struct W(crate::W<TIME_HIGH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_HIGH0_SPEC>;
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
impl From<crate::W<TIME_HIGH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_HIGH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_VALUE0_HIGH` reader - RTC timer high 16 bits"]
pub type TIMER_VALUE0_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_VALUE0_HIGH` writer - RTC timer high 16 bits"]
pub type TIMER_VALUE0_HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, TIME_HIGH0_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn timer_value0_high(&self) -> TIMER_VALUE0_HIGH_R {
        TIMER_VALUE0_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_HIGH0")
            .field(
                "timer_value0_high",
                &format_args!("{}", self.timer_value0_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_HIGH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_value0_high(&mut self) -> TIMER_VALUE0_HIGH_W<0> {
        TIMER_VALUE0_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_high0](index.html) module"]
pub struct TIME_HIGH0_SPEC;
impl crate::RegisterSpec for TIME_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_high0::R](R) reader structure"]
impl crate::Readable for TIME_HIGH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_high0::W](W) writer structure"]
impl crate::Writable for TIME_HIGH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME_HIGH0 to value 0"]
impl crate::Resettable for TIME_HIGH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
