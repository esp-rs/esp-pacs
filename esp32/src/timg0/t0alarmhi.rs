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
#[doc = "Field `ALARM_HI` reader - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALARM_HI` writer - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_W<'a> = crate::FieldWriter<'a, u32, T0ALARMHI_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 0 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W {
        ALARM_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0alarmhi](index.html) module"]
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
}
#[doc = "`reset()` method sets T0ALARMHI to value 0"]
impl crate::Resettable for T0ALARMHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
