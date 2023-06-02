#[doc = "Register `cali_conf` reader"]
pub struct R(crate::R<CALI_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALI_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALI_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALI_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cali_conf` writer"]
pub struct W(crate::W<CALI_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALI_CONF_SPEC>;
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
impl From<crate::W<CALI_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALI_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALI_RTC_MAX` reader - "]
pub type CALI_RTC_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALI_RTC_MAX` writer - "]
pub type CALI_RTC_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, CALI_CONF_SPEC, 10, O, u16, u16>;
#[doc = "Field `CALI_START` reader - "]
pub type CALI_START_R = crate::BitReader;
#[doc = "Field `CALI_START` writer - "]
pub type CALI_START_W<'a, const O: u8> = crate::BitWriter<'a, CALI_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cali_rtc_max(&self) -> CALI_RTC_MAX_R {
        CALI_RTC_MAX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_start(&self) -> CALI_START_R {
        CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cali_conf")
            .field(
                "cali_rtc_max",
                &format_args!("{}", self.cali_rtc_max().bits()),
            )
            .field("cali_start", &format_args!("{}", self.cali_start().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CALI_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn cali_rtc_max(&mut self) -> CALI_RTC_MAX_W<0> {
        CALI_RTC_MAX_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cali_start(&mut self) -> CALI_START_W<31> {
        CALI_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cali_conf](index.html) module"]
pub struct CALI_CONF_SPEC;
impl crate::RegisterSpec for CALI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cali_conf::R](R) reader structure"]
impl crate::Readable for CALI_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cali_conf::W](W) writer structure"]
impl crate::Writable for CALI_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cali_conf to value 0"]
impl crate::Resettable for CALI_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
