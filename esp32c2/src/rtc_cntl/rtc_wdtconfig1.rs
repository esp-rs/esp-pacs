#[doc = "Register `RTC_WDTCONFIG1` reader"]
pub struct R(crate::R<RTC_WDTCONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WDTCONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WDTCONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WDTCONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WDTCONFIG1` writer"]
pub struct W(crate::W<RTC_WDTCONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WDTCONFIG1_SPEC>;
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
impl From<crate::W<RTC_WDTCONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WDTCONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG0_HOLD` reader - Need add desc"]
pub type WDT_STG0_HOLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_STG0_HOLD` writer - Need add desc"]
pub type WDT_STG0_HOLD_W<'a> = crate::FieldWriter<'a, u32, RTC_WDTCONFIG1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> WDT_STG0_HOLD_R {
        WDT_STG0_HOLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_stg0_hold(&mut self) -> WDT_STG0_HOLD_W {
        WDT_STG0_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wdtconfig1](index.html) module"]
pub struct RTC_WDTCONFIG1_SPEC;
impl crate::RegisterSpec for RTC_WDTCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wdtconfig1::R](R) reader structure"]
impl crate::Readable for RTC_WDTCONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wdtconfig1::W](W) writer structure"]
impl crate::Writable for RTC_WDTCONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WDTCONFIG1 to value 0x0003_0d40"]
impl crate::Resettable for RTC_WDTCONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0d40
    }
}
