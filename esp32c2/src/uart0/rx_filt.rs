#[doc = "Register `RX_FILT` reader"]
pub struct R(crate::R<RX_FILT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FILT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FILT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FILT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_FILT` writer"]
pub struct W(crate::W<RX_FILT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_FILT_SPEC>;
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
impl From<crate::W<RX_FILT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_FILT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower than this value, the pulse is ignored."]
pub type GLITCH_FILT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower than this value, the pulse is ignored."]
pub type GLITCH_FILT_W<'a> = crate::FieldWriter<'a, u32, RX_FILT_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GLITCH_FILT_EN` reader - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_R = crate::BitReader<bool>;
#[doc = "Field `GLITCH_FILT_EN` writer - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_W<'a> = crate::BitWriter<'a, u32, RX_FILT_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value, the pulse is ignored."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set this bit to enable Rx signal filter."]
    #[inline(always)]
    pub fn glitch_filt_en(&self) -> GLITCH_FILT_EN_R {
        GLITCH_FILT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value, the pulse is ignored."]
    #[inline(always)]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W {
        GLITCH_FILT_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to enable Rx signal filter."]
    #[inline(always)]
    pub fn glitch_filt_en(&mut self) -> GLITCH_FILT_EN_W {
        GLITCH_FILT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Filter configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_filt](index.html) module"]
pub struct RX_FILT_SPEC;
impl crate::RegisterSpec for RX_FILT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_filt::R](R) reader structure"]
impl crate::Readable for RX_FILT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_filt::W](W) writer structure"]
impl crate::Writable for RX_FILT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_FILT to value 0x08"]
impl crate::Resettable for RX_FILT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
