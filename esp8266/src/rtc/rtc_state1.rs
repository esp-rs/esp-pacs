#[doc = "Register `RTC_STATE1` reader"]
pub struct R(crate::R<RTC_STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_STATE1` writer"]
pub struct W(crate::W<RTC_STATE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_STATE1_SPEC>;
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
impl From<crate::W<RTC_STATE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_STATE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_STATE1_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_STATE1")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_STATE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_STATE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_state1](index.html) module"]
pub struct RTC_STATE1_SPEC;
impl crate::RegisterSpec for RTC_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_state1::R](R) reader structure"]
impl crate::Readable for RTC_STATE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_state1::W](W) writer structure"]
impl crate::Writable for RTC_STATE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_STATE1 to value 0"]
impl crate::Resettable for RTC_STATE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
