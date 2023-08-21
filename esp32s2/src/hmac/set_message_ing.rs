#[doc = "Register `SET_MESSAGE_ING` writer"]
pub type W = crate::W<SET_MESSAGE_ING_SPEC>;
#[doc = "Field `SET_TEXT_ING` writer - Set this bit to show there are still some message blocks to be processed."]
pub type SET_TEXT_ING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_MESSAGE_ING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to show there are still some message blocks to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn set_text_ing(&mut self) -> SET_TEXT_ING_W<SET_MESSAGE_ING_SPEC, 0> {
        SET_TEXT_ING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HMAC message continue register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_ing::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_MESSAGE_ING_SPEC;
impl crate::RegisterSpec for SET_MESSAGE_ING_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_message_ing::W`](W) writer structure"]
impl crate::Writable for SET_MESSAGE_ING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_MESSAGE_ING to value 0"]
impl crate::Resettable for SET_MESSAGE_ING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
