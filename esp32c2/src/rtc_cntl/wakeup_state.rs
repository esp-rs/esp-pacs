#[doc = "Register `WAKEUP_STATE` reader"]
pub struct R(crate::R<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_STATE` writer"]
pub struct W(crate::W<WAKEUP_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_STATE_SPEC>;
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
impl From<crate::W<WAKEUP_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WAKEUP_ENA` reader - wakeup enable bitmap"]
pub type RTC_WAKEUP_ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_WAKEUP_ENA` writer - wakeup enable bitmap"]
pub type RTC_WAKEUP_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEUP_STATE_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 15:31 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn rtc_wakeup_ena(&self) -> RTC_WAKEUP_ENA_R {
        RTC_WAKEUP_ENA_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 15:31 - wakeup enable bitmap"]
    #[inline(always)]
    pub fn rtc_wakeup_ena(&mut self) -> RTC_WAKEUP_ENA_W<15> {
        RTC_WAKEUP_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_state](index.html) module"]
pub struct WAKEUP_STATE_SPEC;
impl crate::RegisterSpec for WAKEUP_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_state::R](R) reader structure"]
impl crate::Readable for WAKEUP_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_state::W](W) writer structure"]
impl crate::Writable for WAKEUP_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_STATE to value 0x0006_0000"]
impl crate::Resettable for WAKEUP_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0006_0000
    }
}
