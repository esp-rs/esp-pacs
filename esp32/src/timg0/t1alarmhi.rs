#[doc = "Register `T1ALARMHI` reader"]
pub struct R(crate::R<T1ALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1ALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1ALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1ALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1ALARMHI` writer"]
pub struct W(crate::W<T1ALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1ALARMHI_SPEC>;
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
impl From<crate::W<T1ALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1ALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_HI` reader - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALARM_HI` writer - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub type ALARM_HI_W<'a> = crate::FieldWriter<'a, u32, T1ALARMHI_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1alarmhi](index.html) module"]
pub struct T1ALARMHI_SPEC;
impl crate::RegisterSpec for T1ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1alarmhi::R](R) reader structure"]
impl crate::Readable for T1ALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1alarmhi::W](W) writer structure"]
impl crate::Writable for T1ALARMHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1ALARMHI to value 0"]
impl crate::Resettable for T1ALARMHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
