#[doc = "Register `SHA384_LOAD` writer"]
pub struct W(crate::W<SHA384_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA384_LOAD_SPEC>;
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
impl From<crate::W<SHA384_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA384_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA384_LOAD` writer - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
pub type SHA384_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, SHA384_LOAD_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA384_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
    #[inline(always)]
    #[must_use]
    pub fn sha384_load(&mut self) -> SHA384_LOAD_W<0> {
        SHA384_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha384_load](index.html) module"]
pub struct SHA384_LOAD_SPEC;
impl crate::RegisterSpec for SHA384_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sha384_load::W](W) writer structure"]
impl crate::Writable for SHA384_LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA384_LOAD to value 0"]
impl crate::Resettable for SHA384_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
