#[doc = "Register `STATUS_W1TC` writer"]
pub struct W(crate::W<STATUS_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_W1TC_SPEC>;
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
impl From<crate::W<STATUS_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS_INT_W1TC` writer - GPIO0~17 interrupt status write 1 to clear"]
pub type STATUS_INT_W1TC_W<'a, const O: u8> = crate::FieldWriter<'a, STATUS_W1TC_SPEC, 18, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 14:31 - GPIO0~17 interrupt status write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn status_int_w1tc(&mut self) -> STATUS_INT_W1TC_W<14> {
        STATUS_INT_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status_w1tc](index.html) module"]
pub struct STATUS_W1TC_SPEC;
impl crate::RegisterSpec for STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [status_w1tc::W](W) writer structure"]
impl crate::Writable for STATUS_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS_W1TC to value 0"]
impl crate::Resettable for STATUS_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
