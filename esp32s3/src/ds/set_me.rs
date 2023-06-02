#[doc = "Register `SET_ME` writer"]
pub struct W(crate::W<SET_ME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_ME_SPEC>;
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
impl From<crate::W<SET_ME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_ME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_ME` writer - Write 1 to this register to start DS operation."]
pub type SET_ME_W<'a, const O: u8> = crate::BitWriter<'a, SET_ME_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_ME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register to start DS operation."]
    #[inline(always)]
    #[must_use]
    pub fn set_me(&mut self) -> SET_ME_W<0> {
        SET_ME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Starts DS operation\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_me](index.html) module"]
pub struct SET_ME_SPEC;
impl crate::RegisterSpec for SET_ME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_me::W](W) writer structure"]
impl crate::Writable for SET_ME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_ME to value 0"]
impl crate::Resettable for SET_ME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
