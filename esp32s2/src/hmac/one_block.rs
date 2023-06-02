#[doc = "Register `ONE_BLOCK` writer"]
pub struct W(crate::W<ONE_BLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONE_BLOCK_SPEC>;
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
impl From<crate::W<ONE_BLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONE_BLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_ONE_BLOCK` writer - Set this bit to show no padding is required."]
pub type SET_ONE_BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, ONE_BLOCK_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ONE_BLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to show no padding is required."]
    #[inline(always)]
    #[must_use]
    pub fn set_one_block(&mut self) -> SET_ONE_BLOCK_W<0> {
        SET_ONE_BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "One block message register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [one_block](index.html) module"]
pub struct ONE_BLOCK_SPEC;
impl crate::RegisterSpec for ONE_BLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [one_block::W](W) writer structure"]
impl crate::Writable for ONE_BLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ONE_BLOCK to value 0"]
impl crate::Resettable for ONE_BLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
