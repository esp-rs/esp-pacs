#[doc = "Register `BBPD_CTRL` reader"]
pub struct R(crate::R<BBPD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBPD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBPD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBPD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBPD_CTRL` writer"]
pub struct W(crate::W<BBPD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBPD_CTRL_SPEC>;
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
impl From<crate::W<BBPD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBPD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC_EST_FORCE_PD` reader - "]
pub type DC_EST_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DC_EST_FORCE_PD` writer - "]
pub type DC_EST_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPD_CTRL_SPEC, bool, O>;
#[doc = "Field `DC_EST_FORCE_PU` reader - "]
pub type DC_EST_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DC_EST_FORCE_PU` writer - "]
pub type DC_EST_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPD_CTRL_SPEC, bool, O>;
#[doc = "Field `FFT_FORCE_PD` reader - "]
pub type FFT_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `FFT_FORCE_PD` writer - "]
pub type FFT_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPD_CTRL_SPEC, bool, O>;
#[doc = "Field `FFT_FORCE_PU` reader - "]
pub type FFT_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `FFT_FORCE_PU` writer - "]
pub type FFT_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BBPD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dc_est_force_pd(&self) -> DC_EST_FORCE_PD_R {
        DC_EST_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dc_est_force_pu(&self) -> DC_EST_FORCE_PU_R {
        DC_EST_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fft_force_pd(&self) -> FFT_FORCE_PD_R {
        FFT_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fft_force_pu(&self) -> FFT_FORCE_PU_R {
        FFT_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dc_est_force_pd(&mut self) -> DC_EST_FORCE_PD_W<0> {
        DC_EST_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dc_est_force_pu(&mut self) -> DC_EST_FORCE_PU_W<1> {
        DC_EST_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fft_force_pd(&mut self) -> FFT_FORCE_PD_W<2> {
        FFT_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fft_force_pu(&mut self) -> FFT_FORCE_PU_W<3> {
        FFT_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baseband control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbpd_ctrl](index.html) module"]
pub struct BBPD_CTRL_SPEC;
impl crate::RegisterSpec for BBPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbpd_ctrl::R](R) reader structure"]
impl crate::Readable for BBPD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbpd_ctrl::W](W) writer structure"]
impl crate::Writable for BBPD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBPD_CTRL to value 0"]
impl crate::Resettable for BBPD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
