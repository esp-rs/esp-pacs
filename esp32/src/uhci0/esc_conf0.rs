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
#[doc = "Field `SEPER_CHAR` reader - This register stores the seperator char seperator char is used to seperate the data frame."]
pub type SEPER_CHAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEPER_CHAR` writer - This register stores the seperator char seperator char is used to seperate the data frame."]
pub type SEPER_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESC_CONF0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEPER_ESC_CHAR0` reader - This register stores thee first char used to replace seperator char in data."]
pub type SEPER_ESC_CHAR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEPER_ESC_CHAR0` writer - This register stores thee first char used to replace seperator char in data."]
pub type SEPER_ESC_CHAR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESC_CONF0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEPER_ESC_CHAR1` reader - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
pub type SEPER_ESC_CHAR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEPER_ESC_CHAR1` writer - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
pub type SEPER_ESC_CHAR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESC_CONF0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn seper_char(&self) -> SEPER_CHAR_R {
        SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> SEPER_ESC_CHAR0_R {
        SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> SEPER_ESC_CHAR1_R {
        SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn seper_char(&mut self) -> SEPER_CHAR_W<0> {
        SEPER_CHAR_W::new(self)
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn seper_esc_char0(&mut self) -> SEPER_ESC_CHAR0_W<8> {
        SEPER_ESC_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf0](index.html) module"]
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
}
#[doc = "`reset()` method sets ESC_CONF0 to value 0x00dc_dbc0"]
impl crate::Resettable for ESC_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00dc_dbc0
    }
}
