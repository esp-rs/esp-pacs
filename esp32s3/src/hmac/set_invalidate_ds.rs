#[doc = "Register `SET_INVALIDATE_DS` writer"]
pub struct W(crate::W<SET_INVALIDATE_DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_INVALIDATE_DS_SPEC>;
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
impl From<crate::W<SET_INVALIDATE_DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_INVALIDATE_DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_INVALIDATE_DS` writer - Clear result from hmac downstream DS."]
pub type SET_INVALIDATE_DS_W<'a, const O: u8> = crate::BitWriter<'a, SET_INVALIDATE_DS_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_INVALIDATE_DS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear result from hmac downstream DS."]
    #[inline(always)]
    #[must_use]
    pub fn set_invalidate_ds(&mut self) -> SET_INVALIDATE_DS_W<0> {
        SET_INVALIDATE_DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Invalidate register 1.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_invalidate_ds](index.html) module"]
pub struct SET_INVALIDATE_DS_SPEC;
impl crate::RegisterSpec for SET_INVALIDATE_DS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_invalidate_ds::W](W) writer structure"]
impl crate::Writable for SET_INVALIDATE_DS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_INVALIDATE_DS to value 0"]
impl crate::Resettable for SET_INVALIDATE_DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
