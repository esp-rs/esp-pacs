#[doc = "Register `SAR_ATTEN1` reader"]
pub struct R(crate::R<SAR_ATTEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_ATTEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_ATTEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_ATTEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_ATTEN1` writer"]
pub struct W(crate::W<SAR_ATTEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_ATTEN1_SPEC>;
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
impl From<crate::W<SAR_ATTEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_ATTEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_ATTEN` reader - 2-bit attenuation for each pad"]
pub type SAR1_ATTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SAR1_ATTEN` writer - 2-bit attenuation for each pad"]
pub type SAR1_ATTEN_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_ATTEN1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad"]
    #[inline(always)]
    pub fn sar1_atten(&self) -> SAR1_ATTEN_R {
        SAR1_ATTEN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_ATTEN1")
            .field("sar1_atten", &format_args!("{}", self.sar1_atten().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_ATTEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_atten(&mut self) -> SAR1_ATTEN_W<0> {
        SAR1_ATTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc1 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_atten1](index.html) module"]
pub struct SAR_ATTEN1_SPEC;
impl crate::RegisterSpec for SAR_ATTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_atten1::R](R) reader structure"]
impl crate::Readable for SAR_ATTEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_atten1::W](W) writer structure"]
impl crate::Writable for SAR_ATTEN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_ATTEN1 to value 0xffff_ffff"]
impl crate::Resettable for SAR_ATTEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
