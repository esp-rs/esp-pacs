#[doc = "Register `ESC_CONF3` reader"]
pub struct R(crate::R<ESC_CONF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF3` writer"]
pub struct W(crate::W<ESC_CONF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF3_SPEC>;
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
impl From<crate::W<ESC_CONF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC_SEQ2` reader - This register stores the flow_control char to turn off the flow_control"]
pub type ESC_SEQ2_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2` writer - This register stores the flow_control char to turn off the flow_control"]
pub type ESC_SEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF3_SPEC, 8, O>;
#[doc = "Field `ESC_SEQ2_CHAR0` reader - This register stores the first char used to replace the reg_esc_seq2 in data."]
pub type ESC_SEQ2_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2_CHAR0` writer - This register stores the first char used to replace the reg_esc_seq2 in data."]
pub type ESC_SEQ2_CHAR0_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF3_SPEC, 8, O>;
#[doc = "Field `ESC_SEQ2_CHAR1` reader - This register stores the second char used to replace the reg_esc_seq2 in data."]
pub type ESC_SEQ2_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ2_CHAR1` writer - This register stores the second char used to replace the reg_esc_seq2 in data."]
pub type ESC_SEQ2_CHAR1_W<'a, const O: u8> = crate::FieldWriter<'a, ESC_CONF3_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores the flow_control char to turn off the flow_control"]
    #[inline(always)]
    pub fn esc_seq2(&self) -> ESC_SEQ2_R {
        ESC_SEQ2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char0(&self) -> ESC_SEQ2_CHAR0_R {
        ESC_SEQ2_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char1(&self) -> ESC_SEQ2_CHAR1_R {
        ESC_SEQ2_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF3")
            .field("esc_seq2", &format_args!("{}", self.esc_seq2().bits()))
            .field(
                "esc_seq2_char0",
                &format_args!("{}", self.esc_seq2_char0().bits()),
            )
            .field(
                "esc_seq2_char1",
                &format_args!("{}", self.esc_seq2_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the flow_control char to turn off the flow_control"]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2(&mut self) -> ESC_SEQ2_W<0> {
        ESC_SEQ2_W::new(self)
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2_char0(&mut self) -> ESC_SEQ2_CHAR0_W<8> {
        ESC_SEQ2_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq2_char1(&mut self) -> ESC_SEQ2_CHAR1_W<16> {
        ESC_SEQ2_CHAR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf3](index.html) module"]
pub struct ESC_CONF3_SPEC;
impl crate::RegisterSpec for ESC_CONF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf3::R](R) reader structure"]
impl crate::Readable for ESC_CONF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf3::W](W) writer structure"]
impl crate::Writable for ESC_CONF3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESC_CONF3 to value 0x00df_db13"]
impl crate::Resettable for ESC_CONF3_SPEC {
    const RESET_VALUE: Self::Ux = 0x00df_db13;
}
