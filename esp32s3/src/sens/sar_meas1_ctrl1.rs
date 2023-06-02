#[doc = "Register `SAR_MEAS1_CTRL1` reader"]
pub struct R(crate::R<SAR_MEAS1_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS1_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS1_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS1_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS1_CTRL1` writer"]
pub struct W(crate::W<SAR_MEAS1_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS1_CTRL1_SPEC>;
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
impl From<crate::W<SAR_MEAS1_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS1_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_XPD_AMP` reader - no public"]
pub type FORCE_XPD_AMP_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_AMP` writer - no public"]
pub type FORCE_XPD_AMP_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS1_CTRL1_SPEC, 2, O>;
#[doc = "Field `AMP_RST_FB_FORCE` reader - no public"]
pub type AMP_RST_FB_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_RST_FB_FORCE` writer - no public"]
pub type AMP_RST_FB_FORCE_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS1_CTRL1_SPEC, 2, O>;
#[doc = "Field `AMP_SHORT_REF_FORCE` reader - no public"]
pub type AMP_SHORT_REF_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_FORCE` writer - no public"]
pub type AMP_SHORT_REF_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEAS1_CTRL1_SPEC, 2, O>;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` reader - no public"]
pub type AMP_SHORT_REF_GND_FORCE_R = crate::FieldReader;
#[doc = "Field `AMP_SHORT_REF_GND_FORCE` writer - no public"]
pub type AMP_SHORT_REF_GND_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEAS1_CTRL1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 24:25 - no public"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - no public"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - no public"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS1_CTRL1")
            .field(
                "force_xpd_amp",
                &format_args!("{}", self.force_xpd_amp().bits()),
            )
            .field(
                "amp_rst_fb_force",
                &format_args!("{}", self.amp_rst_fb_force().bits()),
            )
            .field(
                "amp_short_ref_force",
                &format_args!("{}", self.amp_short_ref_force().bits()),
            )
            .field(
                "amp_short_ref_gnd_force",
                &format_args!("{}", self.amp_short_ref_gnd_force().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS1_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 24:25 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W<24> {
        FORCE_XPD_AMP_W::new(self)
    }
    #[doc = "Bits 26:27 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W<26> {
        AMP_RST_FB_FORCE_W::new(self)
    }
    #[doc = "Bits 28:29 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W<28> {
        AMP_SHORT_REF_FORCE_W::new(self)
    }
    #[doc = "Bits 30:31 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W<30> {
        AMP_SHORT_REF_GND_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas1_ctrl1](index.html) module"]
pub struct SAR_MEAS1_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas1_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_MEAS1_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas1_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_MEAS1_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS1_CTRL1 to value 0"]
impl crate::Resettable for SAR_MEAS1_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
