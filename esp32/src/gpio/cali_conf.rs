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
pub type CALI_RTC_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CALI_CONF_SPEC, u16, u16, 10, O>;
#[doc = "Field `CALI_START` reader - "]
pub type CALI_START_R = crate::BitReader<bool>;
#[doc = "Field `CALI_START` writer - "]
pub type CALI_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALI_CONF_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cali_rtc_max(&mut self) -> CALI_RTC_MAX_W<0> {
        CALI_RTC_MAX_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
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
}
#[doc = "`reset()` method sets cali_conf to value 0"]
impl crate::Resettable for CALI_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
