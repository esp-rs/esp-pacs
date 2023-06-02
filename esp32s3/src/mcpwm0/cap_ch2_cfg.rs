#[doc = "Register `CAP_CH2_CFG` reader"]
pub struct R(crate::R<CAP_CH2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH2_CFG` writer"]
pub struct W(crate::W<CAP_CH2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH2_CFG_SPEC>;
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
impl From<crate::W<CAP_CH2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP2_EN` reader - When set, capture on channel 2 is enabled"]
pub type CAP2_EN_R = crate::BitReader;
#[doc = "Field `CAP2_EN` writer - When set, capture on channel 2 is enabled"]
pub type CAP2_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH2_CFG_SPEC, O>;
#[doc = "Field `CAP2_MODE` reader - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP2_MODE_R = crate::FieldReader;
#[doc = "Field `CAP2_MODE` writer - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP2_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CAP_CH2_CFG_SPEC, 2, O>;
#[doc = "Field `CAP2_PRESCALE` reader - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub type CAP2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CAP2_PRESCALE` writer - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub type CAP2_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, CAP_CH2_CFG_SPEC, 8, O>;
#[doc = "Field `CAP2_IN_INVERT` reader - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub type CAP2_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CAP2_IN_INVERT` writer - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub type CAP2_IN_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH2_CFG_SPEC, O>;
#[doc = "Field `CAP2_SW` writer - Write 1 will trigger a software forced capture on channel 2"]
pub type CAP2_SW_W<'a, const O: u8> = crate::BitWriter<'a, CAP_CH2_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP2_EN_R {
        CAP2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap2_mode(&self) -> CAP2_MODE_R {
        CAP2_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&self) -> CAP2_PRESCALE_R {
        CAP2_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&self) -> CAP2_IN_INVERT_R {
        CAP2_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH2_CFG")
            .field("cap2_en", &format_args!("{}", self.cap2_en().bit()))
            .field("cap2_mode", &format_args!("{}", self.cap2_mode().bits()))
            .field(
                "cap2_prescale",
                &format_args!("{}", self.cap2_prescale().bits()),
            )
            .field(
                "cap2_in_invert",
                &format_args!("{}", self.cap2_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_en(&mut self) -> CAP2_EN_W<0> {
        CAP2_EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_mode(&mut self) -> CAP2_MODE_W<1> {
        CAP2_MODE_W::new(self)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_prescale(&mut self) -> CAP2_PRESCALE_W<3> {
        CAP2_PRESCALE_W::new(self)
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_in_invert(&mut self) -> CAP2_IN_INVERT_W<11> {
        CAP2_IN_INVERT_W::new(self)
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_sw(&mut self) -> CAP2_SW_W<12> {
        CAP2_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture channel 2 configuration and enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch2_cfg](index.html) module"]
pub struct CAP_CH2_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch2_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch2_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH2_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_CH2_CFG to value 0"]
impl crate::Resettable for CAP_CH2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
