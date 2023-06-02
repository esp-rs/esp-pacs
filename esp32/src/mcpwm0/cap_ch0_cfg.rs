#[doc = "Register `CAP_CH0_CFG` reader"]
pub struct R(crate::R<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH0_CFG` writer"]
pub struct W(crate::W<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH0_CFG_SPEC>;
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
impl From<crate::W<CAP_CH0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0_EN` reader - "]
pub type CAP0_EN_R = crate::BitReader;
#[doc = "Field `CAP0_EN` writer - "]
pub type CAP0_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH0_CFG_SPEC, O>;
#[doc = "Field `CAP0_MODE` reader - "]
pub type CAP0_MODE_R = crate::FieldReader;
#[doc = "Field `CAP0_MODE` writer - "]
pub type CAP0_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CAP_CH0_CFG_SPEC, 2, O>;
#[doc = "Field `CAP0_PRESCALE` reader - "]
pub type CAP0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CAP0_PRESCALE` writer - "]
pub type CAP0_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, CAP_CH0_CFG_SPEC, 8, O>;
#[doc = "Field `CAP0_IN_INVERT` reader - "]
pub type CAP0_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CAP0_IN_INVERT` writer - "]
pub type CAP0_IN_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH0_CFG_SPEC, O>;
#[doc = "Field `CAP0_SW` writer - "]
pub type CAP0_SW_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH0_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP0_EN_R {
        CAP0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn cap0_mode(&self) -> CAP0_MODE_R {
        CAP0_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10"]
    #[inline(always)]
    pub fn cap0_prescale(&self) -> CAP0_PRESCALE_R {
        CAP0_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cap0_in_invert(&self) -> CAP0_IN_INVERT_R {
        CAP0_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH0_CFG")
            .field("cap0_en", &format_args!("{}", self.cap0_en().bit()))
            .field("cap0_mode", &format_args!("{}", self.cap0_mode().bits()))
            .field(
                "cap0_prescale",
                &format_args!("{}", self.cap0_prescale().bits()),
            )
            .field(
                "cap0_in_invert",
                &format_args!("{}", self.cap0_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_en(&mut self) -> CAP0_EN_W<0> {
        CAP0_EN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_mode(&mut self) -> CAP0_MODE_W<1> {
        CAP0_MODE_W::new(self)
    }
    #[doc = "Bits 3:10"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_prescale(&mut self) -> CAP0_PRESCALE_W<3> {
        CAP0_PRESCALE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_in_invert(&mut self) -> CAP0_IN_INVERT_W<11> {
        CAP0_IN_INVERT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_sw(&mut self) -> CAP0_SW_W<12> {
        CAP0_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch0_cfg](index.html) module"]
pub struct CAP_CH0_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch0_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch0_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH0_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_CH0_CFG to value 0"]
impl crate::Resettable for CAP_CH0_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
