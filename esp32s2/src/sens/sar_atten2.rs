#[doc = "Register `SAR_ATTEN2` reader"]
pub struct R(crate::R<SAR_ATTEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_ATTEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_ATTEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_ATTEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_ATTEN2` writer"]
pub struct W(crate::W<SAR_ATTEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_ATTEN2_SPEC>;
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
impl From<crate::W<SAR_ATTEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_ATTEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_ATTEN` reader - 2-bit attenuation for each pad. \\[1:0\\] is used for channel 0, \\[3:2\\] is used for channel 1, etc."]
pub type SAR2_ATTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_ATTEN` writer - 2-bit attenuation for each pad. \\[1:0\\] is used for channel 0, \\[3:2\\] is used for channel 1, etc."]
pub type SAR2_ATTEN_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_ATTEN2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad. \\[1:0\\] is used for channel 0, \\[3:2\\] is used for channel 1, etc."]
    #[inline(always)]
    pub fn sar2_atten(&self) -> SAR2_ATTEN_R {
        SAR2_ATTEN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_ATTEN2")
            .field("sar2_atten", &format_args!("{}", self.sar2_atten().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_ATTEN2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad. \\[1:0\\] is used for channel 0, \\[3:2\\] is used for channel 1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_atten(&mut self) -> SAR2_ATTEN_W<0> {
        SAR2_ATTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure SAR ADC2 attenuation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_atten2](index.html) module"]
pub struct SAR_ATTEN2_SPEC;
impl crate::RegisterSpec for SAR_ATTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_atten2::R](R) reader structure"]
impl crate::Readable for SAR_ATTEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_atten2::W](W) writer structure"]
impl crate::Writable for SAR_ATTEN2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_ATTEN2 to value 0xffff_ffff"]
impl crate::Resettable for SAR_ATTEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
