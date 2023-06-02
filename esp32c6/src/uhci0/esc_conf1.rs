#[doc = "Register `ESC_CONF1` reader"]
pub struct R(crate::R<ESC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF1` writer"]
pub struct W(crate::W<ESC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF1_SPEC>;
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
impl From<crate::W<ESC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC_SEQ0` reader - Configures the char needing encoding, which is 0xDB as flow control char by default."]
pub type ESC_SEQ0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0` writer - Configures the char needing encoding, which is 0xDB as flow control char by default."]
pub type ESC_SEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF1_SPEC, 8, O>;
#[doc = "Field `ESC_SEQ0_CHAR0` reader - Configures the first char of SLIP escape character, default value is 0xDB."]
pub type ESC_SEQ0_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR0` writer - Configures the first char of SLIP escape character, default value is 0xDB."]
pub type ESC_SEQ0_CHAR0_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF1_SPEC, 8, O>;
#[doc = "Field `ESC_SEQ0_CHAR1` reader - Configures the second char of SLIP escape character, default value is 0xDD."]
pub type ESC_SEQ0_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR1` writer - Configures the second char of SLIP escape character, default value is 0xDD."]
pub type ESC_SEQ0_CHAR1_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Configures the char needing encoding, which is 0xDB as flow control char by default."]
    #[inline(always)]
    pub fn esc_seq0(&self) -> ESC_SEQ0_R {
        ESC_SEQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the first char of SLIP escape character, default value is 0xDB."]
    #[inline(always)]
    pub fn esc_seq0_char0(&self) -> ESC_SEQ0_CHAR0_R {
        ESC_SEQ0_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the second char of SLIP escape character, default value is 0xDD."]
    #[inline(always)]
    pub fn esc_seq0_char1(&self) -> ESC_SEQ0_CHAR1_R {
        ESC_SEQ0_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF1")
            .field("esc_seq0", &format_args!("{}", self.esc_seq0().bits()))
            .field(
                "esc_seq0_char0",
                &format_args!("{}", self.esc_seq0_char0().bits()),
            )
            .field(
                "esc_seq0_char1",
                &format_args!("{}", self.esc_seq0_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the char needing encoding, which is 0xDB as flow control char by default."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0(&mut self) -> ESC_SEQ0_W<0> {
        ESC_SEQ0_W::new(self)
    }
    #[doc = "Bits 8:15 - Configures the first char of SLIP escape character, default value is 0xDB."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0_char0(&mut self) -> ESC_SEQ0_CHAR0_W<8> {
        ESC_SEQ0_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - Configures the second char of SLIP escape character, default value is 0xDD."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0_char1(&mut self) -> ESC_SEQ0_CHAR1_W<16> {
        ESC_SEQ0_CHAR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI Escapes Sequence Configuration Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf1](index.html) module"]
pub struct ESC_CONF1_SPEC;
impl crate::RegisterSpec for ESC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf1::R](R) reader structure"]
impl crate::Readable for ESC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf1::W](W) writer structure"]
impl crate::Writable for ESC_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESC_CONF1 to value 0x00dd_dbdb"]
impl crate::Resettable for ESC_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00dd_dbdb;
}
