#[doc = "Register `SET_MESSAGE_END` writer"]
pub type W = crate::W<SET_MESSAGE_END_SPEC>;
#[doc = "Field `SET_TEXT_END` writer - Start hardware padding."]
pub type SET_TEXT_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_MESSAGE_END_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start hardware padding."]
    #[inline(always)]
    #[must_use]
    pub fn set_text_end(&mut self) -> SET_TEXT_END_W<SET_MESSAGE_END_SPEC> {
        SET_TEXT_END_W::new(self, 0)
    }
}
#[doc = "Process control register 3.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_message_end::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_MESSAGE_END_SPEC;
impl crate::RegisterSpec for SET_MESSAGE_END_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_message_end::W`](W) writer structure"]
impl crate::Writable for SET_MESSAGE_END_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_MESSAGE_END to value 0"]
impl crate::Resettable for SET_MESSAGE_END_SPEC {
    const RESET_VALUE: u32 = 0;
}
