#[doc = "Register `SAR_TSENS_CTRL2` reader"]
pub struct R(crate::R<SAR_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TSENS_CTRL2` writer"]
pub struct W(crate::W<SAR_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TSENS_CTRL2_SPEC>;
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
impl From<crate::W<SAR_TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TSENS_XPD_WAIT` reader - no public"]
pub type SAR_TSENS_XPD_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_TSENS_XPD_WAIT` writer - no public"]
pub type SAR_TSENS_XPD_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, u16, u16, 12, O>;
#[doc = "Field `SAR_TSENS_XPD_FORCE` reader - no public"]
pub type SAR_TSENS_XPD_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_TSENS_XPD_FORCE` writer - no public"]
pub type SAR_TSENS_XPD_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAR_TSENS_CLK_INV` reader - no public"]
pub type SAR_TSENS_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `SAR_TSENS_CLK_INV` writer - no public"]
pub type SAR_TSENS_CLK_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_wait(&self) -> SAR_TSENS_XPD_WAIT_R {
        SAR_TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_force(&self) -> SAR_TSENS_XPD_FORCE_R {
        SAR_TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - no public"]
    #[inline(always)]
    pub fn sar_tsens_clk_inv(&self) -> SAR_TSENS_CLK_INV_R {
        SAR_TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_wait(&mut self) -> SAR_TSENS_XPD_WAIT_W<0> {
        SAR_TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 12:13 - no public"]
    #[inline(always)]
    pub fn sar_tsens_xpd_force(&mut self) -> SAR_TSENS_XPD_FORCE_W<12> {
        SAR_TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 14 - no public"]
    #[inline(always)]
    pub fn sar_tsens_clk_inv(&mut self) -> SAR_TSENS_CLK_INV_W<14> {
        SAR_TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure tsens controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl2](index.html) module"]
pub struct SAR_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tsens_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for SAR_TSENS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4002
    }
}
