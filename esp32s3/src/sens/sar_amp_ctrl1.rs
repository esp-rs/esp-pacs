#[doc = "Register `SAR_AMP_CTRL1` reader"]
pub struct R(crate::R<SAR_AMP_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_AMP_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_AMP_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_AMP_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_AMP_CTRL1` writer"]
pub struct W(crate::W<SAR_AMP_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_AMP_CTRL1_SPEC>;
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
impl From<crate::W<SAR_AMP_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_AMP_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_AMP_WAIT1` reader - no public"]
pub type SAR_AMP_WAIT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_AMP_WAIT1` writer - no public"]
pub type SAR_AMP_WAIT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `SAR_AMP_WAIT2` reader - no public"]
pub type SAR_AMP_WAIT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_AMP_WAIT2` writer - no public"]
pub type SAR_AMP_WAIT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_AMP_CTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SAR_AMP_WAIT1_R {
        SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SAR_AMP_WAIT2_R {
        SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait1(&mut self) -> SAR_AMP_WAIT1_W<0> {
        SAR_AMP_WAIT1_W::new(self)
    }
    #[doc = "Bits 16:31 - no public"]
    #[inline(always)]
    pub fn sar_amp_wait2(&mut self) -> SAR_AMP_WAIT2_W<16> {
        SAR_AMP_WAIT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_amp_ctrl1](index.html) module"]
pub struct SAR_AMP_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_AMP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_amp_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_AMP_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_amp_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_AMP_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL1 to value 0x000a_000a"]
impl crate::Resettable for SAR_AMP_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_000a
    }
}