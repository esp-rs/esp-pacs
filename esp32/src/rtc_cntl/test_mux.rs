#[doc = "Register `TEST_MUX` reader"]
pub struct R(crate::R<TEST_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_MUX` writer"]
pub struct W(crate::W<TEST_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_MUX_SPEC>;
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
impl From<crate::W<TEST_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENT_RTC` reader - ENT_RTC"]
pub type ENT_RTC_R = crate::BitReader;
#[doc = "Field `ENT_RTC` writer - ENT_RTC"]
pub type ENT_RTC_W<'a, const O: u8> = crate::BitWriter<'a, TEST_MUX_SPEC, O>;
#[doc = "Field `DTEST_RTC` reader - DTEST_RTC"]
pub type DTEST_RTC_R = crate::FieldReader;
#[doc = "Field `DTEST_RTC` writer - DTEST_RTC"]
pub type DTEST_RTC_W<'a, const O: u8> = crate::FieldWriter<'a, TEST_MUX_SPEC, 2, O>;
impl R {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&self) -> ENT_RTC_R {
        ENT_RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&self) -> DTEST_RTC_R {
        DTEST_RTC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_MUX")
            .field("ent_rtc", &format_args!("{}", self.ent_rtc().bit()))
            .field("dtest_rtc", &format_args!("{}", self.dtest_rtc().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    #[must_use]
    pub fn ent_rtc(&mut self) -> ENT_RTC_W<29> {
        ENT_RTC_W::new(self)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_rtc(&mut self) -> DTEST_RTC_W<30> {
        DTEST_RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_mux](index.html) module"]
pub struct TEST_MUX_SPEC;
impl crate::RegisterSpec for TEST_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_mux::R](R) reader structure"]
impl crate::Readable for TEST_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_mux::W](W) writer structure"]
impl crate::Writable for TEST_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST_MUX to value 0"]
impl crate::Resettable for TEST_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
