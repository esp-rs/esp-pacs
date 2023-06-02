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
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower than this value the pulse is ignored."]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower than this value the pulse is ignored."]
pub type GLITCH_FILT_W<'a, const O: u8> = crate::FieldWriter<'a, RX_FILT_SPEC, 8, O>;
#[doc = "Field `GLITCH_FILT_EN` reader - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_R = crate::BitReader;
#[doc = "Field `GLITCH_FILT_EN` writer - Set this bit to enable Rx signal filter."]
pub type GLITCH_FILT_EN_W<'a, const O: u8> = crate::BitWriter<'a, RX_FILT_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value the pulse is ignored."]
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_FILT")
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .field(
                "glitch_filt_en",
                &format_args!("{}", self.glitch_filt_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_FILT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - when input pulse width is lower than this value the pulse is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<0> {
        GLITCH_FILT_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to enable Rx signal filter."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt_en(&mut self) -> GLITCH_FILT_EN_W<8> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_FILT to value 0x08"]
impl crate::Resettable for RX_FILT_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
