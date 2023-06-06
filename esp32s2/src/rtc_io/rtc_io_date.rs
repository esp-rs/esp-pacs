#[doc = "Register `RTC_IO_DATE` reader"]
pub struct R(crate::R<RTC_IO_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_IO_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_IO_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_IO_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_IO_DATE` writer"]
pub struct W(crate::W<RTC_IO_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_IO_DATE_SPEC>;
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
impl From<crate::W<RTC_IO_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_IO_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_DATE` reader - Version control register"]
pub type IO_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `IO_DATE` writer - Version control register"]
pub type IO_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_IO_DATE_SPEC, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    pub fn io_date(&self) -> IO_DATE_R {
        IO_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO_DATE")
            .field("io_date", &format_args!("{}", self.io_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_IO_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    #[must_use]
    pub fn io_date(&mut self) -> IO_DATE_W<0> {
        IO_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Version control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_io_date](index.html) module"]
pub struct RTC_IO_DATE_SPEC;
impl crate::RegisterSpec for RTC_IO_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_io_date::R](R) reader structure"]
impl crate::Readable for RTC_IO_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_io_date::W](W) writer structure"]
impl crate::Writable for RTC_IO_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_IO_DATE to value 0x0190_3170"]
impl crate::Resettable for RTC_IO_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0190_3170;
}
