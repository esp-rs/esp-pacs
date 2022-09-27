#[doc = "Register `SET_MESSAGE_ING` writer"]
pub struct W(crate::W<SET_MESSAGE_ING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_MESSAGE_ING_SPEC>;
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
impl From<crate::W<SET_MESSAGE_ING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_MESSAGE_ING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_TEXT_ING` writer - Continue typical hmac."]
pub type SET_TEXT_ING_W<'a, const O: u8> = crate::BitWriter<'a, u32, SET_MESSAGE_ING_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Continue typical hmac."]
    #[inline(always)]
    pub fn set_text_ing(&mut self) -> SET_TEXT_ING_W<0> {
        SET_TEXT_ING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process control register 2.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_message_ing](index.html) module"]
pub struct SET_MESSAGE_ING_SPEC;
impl crate::RegisterSpec for SET_MESSAGE_ING_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_message_ing::W](W) writer structure"]
impl crate::Writable for SET_MESSAGE_ING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_MESSAGE_ING to value 0"]
impl crate::Resettable for SET_MESSAGE_ING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
