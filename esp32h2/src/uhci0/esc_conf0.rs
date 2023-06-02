#[doc = "Register `ESC_CONF0` reader"]
pub struct R(crate::R<ESC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF0` writer"]
pub struct W(crate::W<ESC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF0_SPEC>;
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
impl From<crate::W<ESC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEPER_CHAR` reader - a"]
pub type SEPER_CHAR_R = crate::FieldReader;
#[doc = "Field `SEPER_CHAR` writer - a"]
pub type SEPER_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF0_SPEC, 8, O>;
#[doc = "Field `SEPER_ESC_CHAR0` reader - a"]
pub type SEPER_ESC_CHAR0_R = crate::FieldReader;
#[doc = "Field `SEPER_ESC_CHAR0` writer - a"]
pub type SEPER_ESC_CHAR0_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF0_SPEC, 8, O>;
#[doc = "Field `SEPER_ESC_CHAR1` reader - a"]
pub type SEPER_ESC_CHAR1_R = crate::FieldReader;
#[doc = "Field `SEPER_ESC_CHAR1` writer - a"]
pub type SEPER_ESC_CHAR1_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF0_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    pub fn seper_char(&self) -> SEPER_CHAR_R {
        SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> SEPER_ESC_CHAR0_R {
        SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> SEPER_ESC_CHAR1_R {
        SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF0")
            .field("seper_char", &format_args!("{}", self.seper_char().bits()))
            .field(
                "seper_esc_char0",
                &format_args!("{}", self.seper_esc_char0().bits()),
            )
            .field(
                "seper_esc_char1",
                &format_args!("{}", self.seper_esc_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    #[must_use]
    pub fn seper_char(&mut self) -> SEPER_CHAR_W<0> {
        SEPER_CHAR_W::new(self)
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char0(&mut self) -> SEPER_ESC_CHAR0_W<8> {
        SEPER_ESC_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char1(&mut self) -> SEPER_ESC_CHAR1_W<16> {
        SEPER_ESC_CHAR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf0](index.html) module"]
pub struct ESC_CONF0_SPEC;
impl crate::RegisterSpec for ESC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf0::R](R) reader structure"]
impl crate::Readable for ESC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf0::W](W) writer structure"]
impl crate::Writable for ESC_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESC_CONF0 to value 0x00dc_dbc0"]
impl crate::Resettable for ESC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00dc_dbc0;
}
