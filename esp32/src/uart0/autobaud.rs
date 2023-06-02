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
#[doc = "Field `EN` reader - This is the enable bit for detecting baudrate."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - This is the enable bit for detecting baudrate."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, AUTOBAUD_SPEC, O>;
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub type GLITCH_FILT_W<'a, const O: u8> = crate::FieldWriter<'a, AUTOBAUD_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOBAUD")
            .field("en", &format_args!("{}", self.en().bit()))
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AUTOBAUD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autobaud](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOBAUD to value 0x1000"]
impl crate::Resettable for AUTOBAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
