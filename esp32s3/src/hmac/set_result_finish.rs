#[doc = "Register `SET_RESULT_FINISH` writer"]
pub struct W(crate::W<SET_RESULT_FINISH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_RESULT_FINISH_SPEC>;
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
impl From<crate::W<SET_RESULT_FINISH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_RESULT_FINISH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_RESULT_END` writer - After read result from upstream, then let hmac back to idle."]
pub type SET_RESULT_END_W<'a, const O: u8> = crate::BitWriter<'a, SET_RESULT_FINISH_SPEC, O>;
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
    pub fn set_result_end(&mut self) -> SET_RESULT_END_W<0> {
        SET_RESULT_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process control register 4.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_result_finish](index.html) module"]
pub struct SET_RESULT_FINISH_SPEC;
impl crate::RegisterSpec for SET_RESULT_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_result_finish::W](W) writer structure"]
impl crate::Writable for SET_RESULT_FINISH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_RESULT_FINISH to value 0"]
impl crate::Resettable for SET_RESULT_FINISH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
