#[doc = "Register `ETM_EVENT_CH%s_CFG` reader"]
pub struct R(crate::R<ETM_EVENT_CH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_EVENT_CH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_EVENT_CH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_EVENT_CH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_EVENT_CH%s_CFG` writer"]
pub struct W(crate::W<ETM_EVENT_CH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_EVENT_CH_CFG_SPEC>;
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
impl From<crate::W<ETM_EVENT_CH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_EVENT_CH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_CH0_EVENT_SEL` reader - Etm event channel select gpio."]
pub type ETM_CH0_EVENT_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_CH0_EVENT_SEL` writer - Etm event channel select gpio."]
pub type ETM_CH0_EVENT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, ETM_EVENT_CH_CFG_SPEC, 5, O>;
#[doc = "Field `ETM_CH0_EVENT_EN` reader - Etm event send enable bit."]
pub type ETM_CH0_EVENT_EN_R = crate::BitReader;
#[doc = "Field `ETM_CH0_EVENT_EN` writer - Etm event send enable bit."]
pub type ETM_CH0_EVENT_EN_W<'a, const O: u8> = crate::BitWriter<'a, ETM_EVENT_CH_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - Etm event channel select gpio."]
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> ETM_CH0_EVENT_SEL_R {
        ETM_CH0_EVENT_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> ETM_CH0_EVENT_EN_R {
        ETM_CH0_EVENT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_EVENT_CH_CFG")
            .field(
                "etm_ch0_event_sel",
                &format_args!("{}", self.etm_ch0_event_sel().bits()),
            )
            .field(
                "etm_ch0_event_en",
                &format_args!("{}", self.etm_ch0_event_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_EVENT_CH_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Etm event channel select gpio."]
    #[inline(always)]
    #[must_use]
    pub fn etm_ch0_event_sel(&mut self) -> ETM_CH0_EVENT_SEL_W<0> {
        ETM_CH0_EVENT_SEL_W::new(self)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn etm_ch0_event_en(&mut self) -> ETM_CH0_EVENT_EN_W<7> {
        ETM_CH0_EVENT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Etm Config register of Channel%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_event_ch_cfg](index.html) module"]
pub struct ETM_EVENT_CH_CFG_SPEC;
impl crate::RegisterSpec for ETM_EVENT_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_event_ch_cfg::R](R) reader structure"]
impl crate::Readable for ETM_EVENT_CH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_event_ch_cfg::W](W) writer structure"]
impl crate::Writable for ETM_EVENT_CH_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_EVENT_CH%s_CFG to value 0"]
impl crate::Resettable for ETM_EVENT_CH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
