#[doc = "Register `ONETIME_SAMPLE` reader"]
pub struct R(crate::R<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONETIME_SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONETIME_SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONETIME_SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONETIME_SAMPLE` writer"]
pub struct W(crate::W<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONETIME_SAMPLE_SPEC>;
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
impl From<crate::W<ONETIME_SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONETIME_SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_ONETIME_ATTEN` reader - Need add description"]
pub type SARADC_ONETIME_ATTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARADC_ONETIME_ATTEN` writer - Need add description"]
pub type SARADC_ONETIME_ATTEN_W<'a> =
    crate::FieldWriter<'a, u32, ONETIME_SAMPLE_SPEC, u8, u8, 2, 23>;
#[doc = "Field `SARADC_ONETIME_CHANNEL` reader - Need add description"]
pub type SARADC_ONETIME_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARADC_ONETIME_CHANNEL` writer - Need add description"]
pub type SARADC_ONETIME_CHANNEL_W<'a> =
    crate::FieldWriter<'a, u32, ONETIME_SAMPLE_SPEC, u8, u8, 4, 25>;
#[doc = "Field `SARADC_ONETIME_START` reader - Need add description"]
pub type SARADC_ONETIME_START_R = crate::BitReader<bool>;
#[doc = "Field `SARADC_ONETIME_START` writer - Need add description"]
pub type SARADC_ONETIME_START_W<'a> = crate::BitWriter<'a, u32, ONETIME_SAMPLE_SPEC, bool, 29>;
#[doc = "Field `SARADC2_ONETIME_SAMPLE` reader - Need add description"]
pub type SARADC2_ONETIME_SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `SARADC2_ONETIME_SAMPLE` writer - Need add description"]
pub type SARADC2_ONETIME_SAMPLE_W<'a> = crate::BitWriter<'a, u32, ONETIME_SAMPLE_SPEC, bool, 30>;
#[doc = "Field `SARADC1_ONETIME_SAMPLE` reader - Need add description"]
pub type SARADC1_ONETIME_SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `SARADC1_ONETIME_SAMPLE` writer - Need add description"]
pub type SARADC1_ONETIME_SAMPLE_W<'a> = crate::BitWriter<'a, u32, ONETIME_SAMPLE_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 23:24 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_atten(&self) -> SARADC_ONETIME_ATTEN_R {
        SARADC_ONETIME_ATTEN_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:28 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_channel(&self) -> SARADC_ONETIME_CHANNEL_R {
        SARADC_ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_start(&self) -> SARADC_ONETIME_START_R {
        SARADC_ONETIME_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn saradc2_onetime_sample(&self) -> SARADC2_ONETIME_SAMPLE_R {
        SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn saradc1_onetime_sample(&self) -> SARADC1_ONETIME_SAMPLE_R {
        SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 23:24 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_atten(&mut self) -> SARADC_ONETIME_ATTEN_W {
        SARADC_ONETIME_ATTEN_W::new(self)
    }
    #[doc = "Bits 25:28 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_channel(&mut self) -> SARADC_ONETIME_CHANNEL_W {
        SARADC_ONETIME_CHANNEL_W::new(self)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn saradc_onetime_start(&mut self) -> SARADC_ONETIME_START_W {
        SARADC_ONETIME_START_W::new(self)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn saradc2_onetime_sample(&mut self) -> SARADC2_ONETIME_SAMPLE_W {
        SARADC2_ONETIME_SAMPLE_W::new(self)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn saradc1_onetime_sample(&mut self) -> SARADC1_ONETIME_SAMPLE_W {
        SARADC1_ONETIME_SAMPLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [onetime_sample](index.html) module"]
pub struct ONETIME_SAMPLE_SPEC;
impl crate::RegisterSpec for ONETIME_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [onetime_sample::R](R) reader structure"]
impl crate::Readable for ONETIME_SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [onetime_sample::W](W) writer structure"]
impl crate::Writable for ONETIME_SAMPLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ONETIME_SAMPLE to value 0x1a00_0000"]
impl crate::Resettable for ONETIME_SAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a00_0000
    }
}
