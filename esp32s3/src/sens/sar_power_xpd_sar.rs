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
#[doc = "Field `SAR_FORCE_XPD_SAR` reader - force power on/off saradc"]
pub type SAR_FORCE_XPD_SAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_FORCE_XPD_SAR` writer - force power on/off saradc"]
pub type SAR_FORCE_XPD_SAR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_POWER_XPD_SAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAR_SARCLK_EN` reader - no public"]
pub type SAR_SARCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SAR_SARCLK_EN` writer - no public"]
pub type SAR_SARCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_POWER_XPD_SAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    pub fn sar_force_xpd_sar(&self) -> SAR_FORCE_XPD_SAR_R {
        SAR_FORCE_XPD_SAR_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    pub fn sar_sarclk_en(&self) -> SAR_SARCLK_EN_R {
        SAR_SARCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 29:30 - force power on/off saradc"]
    #[inline(always)]
    pub fn sar_force_xpd_sar(&mut self) -> SAR_FORCE_XPD_SAR_W<29> {
        SAR_FORCE_XPD_SAR_W::new(self)
    }
    #[doc = "Bit 31 - no public"]
    #[inline(always)]
    pub fn sar_sarclk_en(&mut self) -> SAR_SARCLK_EN_W<31> {
        SAR_SARCLK_EN_W::new(self)
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
}
#[doc = "`reset()` method sets SAR_POWER_XPD_SAR to value 0"]
impl crate::Resettable for SAR_POWER_XPD_SAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
