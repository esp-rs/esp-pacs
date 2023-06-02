#[doc = "Register `PLC_CONF1` reader"]
pub struct R(crate::R<PLC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC_CONF1` writer"]
pub struct W(crate::W<PLC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_CONF1_SPEC>;
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
impl From<crate::W<PLC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAD_CEF_ATTEN_PARA` reader - "]
pub type BAD_CEF_ATTEN_PARA_R = crate::FieldReader;
#[doc = "Field `BAD_CEF_ATTEN_PARA` writer - "]
pub type BAD_CEF_ATTEN_PARA_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF1_SPEC, 8, O>;
#[doc = "Field `BAD_CEF_ATTEN_PARA_SHIFT` reader - "]
pub type BAD_CEF_ATTEN_PARA_SHIFT_R = crate::FieldReader;
#[doc = "Field `BAD_CEF_ATTEN_PARA_SHIFT` writer - "]
pub type BAD_CEF_ATTEN_PARA_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF1_SPEC, 4, O>;
#[doc = "Field `BAD_OLA_WIN2_PARA_SHIFT` reader - "]
pub type BAD_OLA_WIN2_PARA_SHIFT_R = crate::FieldReader;
#[doc = "Field `BAD_OLA_WIN2_PARA_SHIFT` writer - "]
pub type BAD_OLA_WIN2_PARA_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF1_SPEC, 4, O>;
#[doc = "Field `BAD_OLA_WIN2_PARA` reader - "]
pub type BAD_OLA_WIN2_PARA_R = crate::FieldReader;
#[doc = "Field `BAD_OLA_WIN2_PARA` writer - "]
pub type BAD_OLA_WIN2_PARA_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF1_SPEC, 8, O>;
#[doc = "Field `SLIDE_WIN_LEN` reader - "]
pub type SLIDE_WIN_LEN_R = crate::FieldReader;
#[doc = "Field `SLIDE_WIN_LEN` writer - "]
pub type SLIDE_WIN_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, PLC_CONF1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bad_cef_atten_para(&self) -> BAD_CEF_ATTEN_PARA_R {
        BAD_CEF_ATTEN_PARA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bad_cef_atten_para_shift(&self) -> BAD_CEF_ATTEN_PARA_SHIFT_R {
        BAD_CEF_ATTEN_PARA_SHIFT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_ola_win2_para_shift(&self) -> BAD_OLA_WIN2_PARA_SHIFT_R {
        BAD_OLA_WIN2_PARA_SHIFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bad_ola_win2_para(&self) -> BAD_OLA_WIN2_PARA_R {
        BAD_OLA_WIN2_PARA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slide_win_len(&self) -> SLIDE_WIN_LEN_R {
        SLIDE_WIN_LEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLC_CONF1")
            .field(
                "bad_cef_atten_para",
                &format_args!("{}", self.bad_cef_atten_para().bits()),
            )
            .field(
                "bad_cef_atten_para_shift",
                &format_args!("{}", self.bad_cef_atten_para_shift().bits()),
            )
            .field(
                "bad_ola_win2_para_shift",
                &format_args!("{}", self.bad_ola_win2_para_shift().bits()),
            )
            .field(
                "bad_ola_win2_para",
                &format_args!("{}", self.bad_ola_win2_para().bits()),
            )
            .field(
                "slide_win_len",
                &format_args!("{}", self.slide_win_len().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLC_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn bad_cef_atten_para(&mut self) -> BAD_CEF_ATTEN_PARA_W<0> {
        BAD_CEF_ATTEN_PARA_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn bad_cef_atten_para_shift(&mut self) -> BAD_CEF_ATTEN_PARA_SHIFT_W<8> {
        BAD_CEF_ATTEN_PARA_SHIFT_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn bad_ola_win2_para_shift(&mut self) -> BAD_OLA_WIN2_PARA_SHIFT_W<12> {
        BAD_OLA_WIN2_PARA_SHIFT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn bad_ola_win2_para(&mut self) -> BAD_OLA_WIN2_PARA_W<16> {
        BAD_OLA_WIN2_PARA_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn slide_win_len(&mut self) -> SLIDE_WIN_LEN_W<24> {
        SLIDE_WIN_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc_conf1](index.html) module"]
pub struct PLC_CONF1_SPEC;
impl crate::RegisterSpec for PLC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc_conf1::R](R) reader structure"]
impl crate::Readable for PLC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc_conf1::W](W) writer structure"]
impl crate::Writable for PLC_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLC_CONF1 to value 0xa017_8a05"]
impl crate::Resettable for PLC_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0xa017_8a05;
}
