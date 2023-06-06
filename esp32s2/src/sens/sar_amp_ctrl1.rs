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
#[doc = "Field `SAR_AMP_WAIT1` reader - "]
pub type SAR_AMP_WAIT1_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT1` writer - "]
pub type SAR_AMP_WAIT1_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL1_SPEC, 16, O, u16>;
#[doc = "Field `SAR_AMP_WAIT2` reader - "]
pub type SAR_AMP_WAIT2_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT2` writer - "]
pub type SAR_AMP_WAIT2_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_AMP_CTRL1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SAR_AMP_WAIT1_R {
        SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SAR_AMP_WAIT2_R {
        SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_AMP_CTRL1")
            .field(
                "sar_amp_wait1",
                &format_args!("{}", self.sar_amp_wait1().bits()),
            )
            .field(
                "sar_amp_wait2",
                &format_args!("{}", self.sar_amp_wait2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_AMP_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sar_amp_wait1(&mut self) -> SAR_AMP_WAIT1_W<0> {
        SAR_AMP_WAIT1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
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
#[doc = "AMP control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_amp_ctrl1](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_AMP_CTRL1 to value 0x000a_000a"]
impl crate::Resettable for SAR_AMP_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_000a;
}
