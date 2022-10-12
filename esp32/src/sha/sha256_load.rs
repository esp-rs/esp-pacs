#[doc = "Register `SHA256_LOAD` writer"]
pub struct W(crate::W<SHA256_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA256_LOAD_SPEC>;
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
impl From<crate::W<SHA256_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA256_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA256_LOAD` writer - Write 1 to finish the SHA-256 operation to calculate the final message hash."]
pub type SHA256_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHA256_LOAD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-256 operation to calculate the final message hash."]
    #[inline(always)]
    pub fn sha256_load(&mut self) -> SHA256_LOAD_W<0> {
        SHA256_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha256_load](index.html) module"]
pub struct SHA256_LOAD_SPEC;
impl crate::RegisterSpec for SHA256_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sha256_load::W](W) writer structure"]
impl crate::Writable for SHA256_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHA256_LOAD to value 0"]
impl crate::Resettable for SHA256_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}