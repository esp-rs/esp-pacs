#[doc = "Register `RTC_SLP_TIMER1` reader"]
pub struct R(crate::R<RTC_SLP_TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SLP_TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SLP_TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SLP_TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SLP_TIMER1` writer"]
pub struct W(crate::W<RTC_SLP_TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SLP_TIMER1_SPEC>;
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
impl From<crate::W<RTC_SLP_TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SLP_TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_HI` reader - RTC sleep timer high 16 bits"]
pub type SLP_VAL_HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLP_VAL_HI` writer - RTC sleep timer high 16 bits"]
pub type SLP_VAL_HI_W<'a> = crate::FieldWriter<'a, u32, RTC_SLP_TIMER1_SPEC, u16, u16, 16, 0>;
#[doc = "Field `RTC_MAIN_TIMER_ALARM_EN` reader - timer alarm enable bit"]
pub type RTC_MAIN_TIMER_ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_TIMER_ALARM_EN` writer - timer alarm enable bit"]
pub type RTC_MAIN_TIMER_ALARM_EN_W<'a> = crate::BitWriter<'a, u32, RTC_SLP_TIMER1_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn rtc_main_timer_alarm_en(&self) -> RTC_MAIN_TIMER_ALARM_EN_R {
        RTC_MAIN_TIMER_ALARM_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W {
        SLP_VAL_HI_W::new(self)
    }
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn rtc_main_timer_alarm_en(&mut self) -> RTC_MAIN_TIMER_ALARM_EN_W {
        RTC_MAIN_TIMER_ALARM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_slp_timer1](index.html) module"]
pub struct RTC_SLP_TIMER1_SPEC;
impl crate::RegisterSpec for RTC_SLP_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_slp_timer1::R](R) reader structure"]
impl crate::Readable for RTC_SLP_TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_slp_timer1::W](W) writer structure"]
impl crate::Writable for RTC_SLP_TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SLP_TIMER1 to value 0"]
impl crate::Resettable for RTC_SLP_TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
