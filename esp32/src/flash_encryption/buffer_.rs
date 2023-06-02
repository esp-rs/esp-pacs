#[doc = "Register `BUFFER_%s` writer"]
pub struct W(crate::W<BUFFER__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFFER__SPEC>;
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
impl From<crate::W<BUFFER__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFFER__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFER` writer - Data buffers for encryption."]
pub type BUFFER_W<'a, const O: u8> = crate::FieldWriter<'a, BUFFER__SPEC, 8, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFFER__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data buffers for encryption."]
    #[inline(always)]
    #[must_use]
    pub fn buffer(&mut self) -> BUFFER_W<0> {
        BUFFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buffer_](index.html) module"]
pub struct BUFFER__SPEC;
impl crate::RegisterSpec for BUFFER__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [buffer_::W](W) writer structure"]
impl crate::Writable for BUFFER__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFFER_%s to value 0"]
impl crate::Resettable for BUFFER__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
