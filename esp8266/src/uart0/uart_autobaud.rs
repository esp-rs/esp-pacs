#[doc = "Register `UART_AUTOBAUD` reader"]
pub struct R(crate::R<UART_AUTOBAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_AUTOBAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_AUTOBAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_AUTOBAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_AUTOBAUD` writer"]
pub struct W(crate::W<UART_AUTOBAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_AUTOBAUD_SPEC>;
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
impl From<crate::W<UART_AUTOBAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_AUTOBAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `autobaud_en` reader - Set this bit to enable baudrate detect"]
pub type AUTOBAUD_EN_R = crate::BitReader;
#[doc = "Field `autobaud_en` writer - Set this bit to enable baudrate detect"]
pub type AUTOBAUD_EN_W<'a, const O: u8> = crate::BitWriter<'a, UART_AUTOBAUD_SPEC, O>;
#[doc = "Field `glitch_filt` reader - "]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `glitch_filt` writer - "]
pub type GLITCH_FILT_W<'a, const O: u8> = crate::FieldWriter<'a, UART_AUTOBAUD_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable baudrate detect"]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_AUTOBAUD")
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .field("autobaud_en", &format_args!("{}", self.autobaud_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_AUTOBAUD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable baudrate detect"]
    #[inline(always)]
    #[must_use]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W<0> {
        AUTOBAUD_EN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
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
#[doc = "UART BAUDRATE DETECT REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_autobaud](index.html) module"]
pub struct UART_AUTOBAUD_SPEC;
impl crate::RegisterSpec for UART_AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_autobaud::R](R) reader structure"]
impl crate::Readable for UART_AUTOBAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_autobaud::W](W) writer structure"]
impl crate::Writable for UART_AUTOBAUD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_AUTOBAUD to value 0"]
impl crate::Resettable for UART_AUTOBAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
