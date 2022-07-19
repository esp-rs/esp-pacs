#[doc = "Register `RTC_TIMER5` reader"]
pub struct R(crate::R<RTC_TIMER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER5` writer"]
pub struct W(crate::W<RTC_TIMER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER5_SPEC>;
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
impl From<crate::W<RTC_TIMER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER5_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W {
        MIN_SLP_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer5](index.html) module"]
pub struct RTC_TIMER5_SPEC;
impl crate::RegisterSpec for RTC_TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer5::R](R) reader structure"]
impl crate::Readable for RTC_TIMER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer5::W](W) writer structure"]
impl crate::Writable for RTC_TIMER5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER5 to value 0x8000"]
impl crate::Resettable for RTC_TIMER5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
