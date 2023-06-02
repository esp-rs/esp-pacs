#[doc = "Register `SAR_POWER_XPD_SAR` reader"]
pub struct R(crate::R<SAR_POWER_XPD_SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_POWER_XPD_SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_POWER_XPD_SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_POWER_XPD_SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_POWER_XPD_SAR` writer"]
pub struct W(crate::W<SAR_POWER_XPD_SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_POWER_XPD_SAR_SPEC>;
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
impl From<crate::W<SAR_POWER_XPD_SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_POWER_XPD_SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_XPD_SAR` reader - force power on/off saradc"]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - force power on/off saradc"]
pub type FORCE_XPD_SAR_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_POWER_XPD_SAR_SPEC, 2, O>;
#[doc = "Field `SARCLK_EN` reader - no public"]
pub type SARCLK_EN_R = crate::BitReader;
#[doc = "Field `SARCLK_EN` writer - no public"]
pub type SARCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_POWER_XPD_SAR_SPEC, O>;
impl R {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_POWER_XPD_SAR")
            .field(
                "force_xpd_sar",
                &format_args!("{}", self.force_xpd_sar().bits()),
            )
            .field("sarclk_en", &format_args!("{}", self.sarclk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_POWER_XPD_SAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<29> {
        FORCE_XPD_SAR_W::new(self)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W<31> {
        SARCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure power of saradc\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_power_xpd_sar](index.html) module"]
pub struct SAR_POWER_XPD_SAR_SPEC;
impl crate::RegisterSpec for SAR_POWER_XPD_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_power_xpd_sar::R](R) reader structure"]
impl crate::Readable for SAR_POWER_XPD_SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_power_xpd_sar::W](W) writer structure"]
impl crate::Writable for SAR_POWER_XPD_SAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_POWER_XPD_SAR to value 0"]
impl crate::Resettable for SAR_POWER_XPD_SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
