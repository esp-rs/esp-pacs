#[doc = "Register `SAR_MEAS_WAIT2` reader"]
pub struct R(crate::R<SAR_MEAS_WAIT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_WAIT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_WAIT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_WAIT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_WAIT2` writer"]
pub struct W(crate::W<SAR_MEAS_WAIT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_WAIT2_SPEC>;
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
impl From<crate::W<SAR_MEAS_WAIT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_WAIT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_XPD_SAR_SW` reader - "]
pub type FORCE_XPD_SAR_SW_R = crate::BitReader;
#[doc = "Field `FORCE_XPD_SAR_SW` writer - "]
pub type FORCE_XPD_SAR_SW_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEAS_WAIT2_SPEC, O>;
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub type SAR_AMP_WAIT3_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_MEAS_WAIT2_SPEC, 16, O, u16, u16>;
#[doc = "Field `FORCE_XPD_AMP` reader - "]
pub type FORCE_XPD_AMP_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_AMP` writer - "]
pub type FORCE_XPD_AMP_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_WAIT2_SPEC, 2, O>;
#[doc = "Field `FORCE_XPD_SAR` reader - "]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - "]
pub type FORCE_XPD_SAR_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_WAIT2_SPEC, 2, O>;
#[doc = "Field `SAR2_RSTB_WAIT` reader - "]
pub type SAR2_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_WAIT` writer - "]
pub type SAR2_RSTB_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS_WAIT2_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_xpd_sar_sw(&self) -> FORCE_XPD_SAR_SW_R {
        FORCE_XPD_SAR_SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS_WAIT2")
            .field(
                "force_xpd_sar_sw",
                &format_args!("{}", self.force_xpd_sar_sw().bit()),
            )
            .field(
                "sar_amp_wait3",
                &format_args!("{}", self.sar_amp_wait3().bits()),
            )
            .field(
                "force_xpd_amp",
                &format_args!("{}", self.force_xpd_amp().bits()),
            )
            .field(
                "force_xpd_sar",
                &format_args!("{}", self.force_xpd_sar().bits()),
            )
            .field(
                "sar2_rstb_wait",
                &format_args!("{}", self.sar2_rstb_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS_WAIT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar_sw(&mut self) -> FORCE_XPD_SAR_SW_W<0> {
        FORCE_XPD_SAR_SW_W::new(self)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<0> {
        SAR_AMP_WAIT3_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W<16> {
        FORCE_XPD_AMP_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<18> {
        FORCE_XPD_SAR_W::new(self)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W<20> {
        SAR2_RSTB_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_wait2](index.html) module"]
pub struct SAR_MEAS_WAIT2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_WAIT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_wait2::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_wait2::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS_WAIT2 to value 0x0020_000a"]
impl crate::Resettable for SAR_MEAS_WAIT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_000a;
}
