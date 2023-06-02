#[doc = "Register `IV_%s` writer"]
pub struct W(crate::W<IV__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV__SPEC>;
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
impl From<crate::W<IV__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV_0` writer - IV block data."]
pub type IV_0_W<'a, const O: u8> = crate::FieldWriter<'a, IV__SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IV__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - IV block data."]
    #[inline(always)]
    #[must_use]
    pub fn iv_0(&mut self) -> IV_0_W<0> {
        IV_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IV block data.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_](index.html) module"]
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iv_::W](W) writer structure"]
impl crate::Writable for IV__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IV_%s to value 0"]
impl crate::Resettable for IV__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
