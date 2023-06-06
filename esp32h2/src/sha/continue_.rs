#[doc = "Register `CONTINUE` writer"]
pub struct W(crate::W<CONTINUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTINUE_SPEC>;
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
impl From<crate::W<CONTINUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTINUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUE` writer - Reserved."]
pub type CONTINUE_W<'a, const O: u8> = crate::FieldWriter<'a, CONTINUE_SPEC, 31, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 1:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn continue_(&mut self) -> CONTINUE_W<1> {
        CONTINUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Typical SHA configuration register 1.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continue_](index.html) module"]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [continue_::W](W) writer structure"]
impl crate::Writable for CONTINUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
