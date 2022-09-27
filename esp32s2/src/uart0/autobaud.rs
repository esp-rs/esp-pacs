#[doc = "Register `AUTOBAUD` reader"]
pub struct R(crate::R<AUTOBAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOBAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOBAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOBAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOBAUD` writer"]
pub struct W(crate::W<AUTOBAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOBAUD_SPEC>;
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
impl From<crate::W<AUTOBAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOBAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - This is the enable bit for baud rate detection."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - This is the enable bit for baud rate detection."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOBAUD_SPEC, bool, O>;
#[doc = "Field `GLITCH_FILT` reader - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
pub type GLITCH_FILT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GLITCH_FILT` writer - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
pub type GLITCH_FILT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUTOBAUD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for baud rate detection."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for baud rate detection."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:15 - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
    #[inline(always)]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<8> {
        GLITCH_FILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autobaud configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autobaud](index.html) module"]
pub struct AUTOBAUD_SPEC;
impl crate::RegisterSpec for AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autobaud::R](R) reader structure"]
impl crate::Readable for AUTOBAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autobaud::W](W) writer structure"]
impl crate::Writable for AUTOBAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOBAUD to value 0x1000"]
impl crate::Resettable for AUTOBAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
