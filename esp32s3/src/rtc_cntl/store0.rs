#[doc = "Register `STORE0` reader"]
pub struct R(crate::R<STORE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE0` writer"]
pub struct W(crate::W<STORE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE0_SPEC>;
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
impl From<crate::W<STORE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SCRATCH0` reader - Reserved register"]
pub type RTC_SCRATCH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_SCRATCH0` writer - Reserved register"]
pub type RTC_SCRATCH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn rtc_scratch0(&self) -> RTC_SCRATCH0_R {
        RTC_SCRATCH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved register"]
    #[inline(always)]
    pub fn rtc_scratch0(&mut self) -> RTC_SCRATCH0_W<0> {
        RTC_SCRATCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reserved register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0](index.html) module"]
pub struct STORE0_SPEC;
impl crate::RegisterSpec for STORE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store0::R](R) reader structure"]
impl crate::Readable for STORE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store0::W](W) writer structure"]
impl crate::Writable for STORE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE0 to value 0"]
impl crate::Resettable for STORE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
