#[doc = "Register `SET_RESULT_FINISH` writer"]
pub type W = crate::W<SET_RESULT_FINISH_SPEC>;
#[doc = "Field `SET_RESULT_END` writer - After read result from upstream, then let hmac back to idle."]
pub type SET_RESULT_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_RESULT_FINISH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - After read result from upstream, then let hmac back to idle."]
    #[inline(always)]
    #[must_use]
    pub fn set_result_end(&mut self) -> SET_RESULT_END_W<SET_RESULT_FINISH_SPEC> {
        SET_RESULT_END_W::new(self, 0)
    }
}
#[doc = "Process control register 4.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_result_finish::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_RESULT_FINISH_SPEC;
impl crate::RegisterSpec for SET_RESULT_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_result_finish::W`](W) writer structure"]
impl crate::Writable for SET_RESULT_FINISH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_RESULT_FINISH to value 0"]
impl crate::Resettable for SET_RESULT_FINISH_SPEC {
    const RESET_VALUE: u32 = 0;
}
