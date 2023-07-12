#[doc = "Register `T0ALARMHI` reader"]
pub struct R(crate::R<T0ALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0ALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0ALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0ALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0ALARMHI` writer"]
pub struct W(crate::W<T0ALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0ALARMHI_SPEC>;
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
impl From<crate::W<T0ALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0ALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_HI` reader - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type ALARM_HI_W<'a, const O: u8> = crate::FieldWriter<'a, T0ALARMHI_SPEC, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0ALARMHI")
            .field("alarm_hi", &format_args!("{}", self.alarm_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0ALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<0> {
        ALARM_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s alarm value, high bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0alarmhi](index.html) module"]
pub struct T0ALARMHI_SPEC;
impl crate::RegisterSpec for T0ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0alarmhi::R](R) reader structure"]
impl crate::Readable for T0ALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0alarmhi::W](W) writer structure"]
impl crate::Writable for T0ALARMHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0ALARMHI to value 0"]
impl crate::Resettable for T0ALARMHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
