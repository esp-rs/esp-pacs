#[doc = "Register `AAD_BLOCK_NUM` reader"]
pub struct R(crate::R<AAD_BLOCK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AAD_BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AAD_BLOCK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AAD_BLOCK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AAD_BLOCK_NUM` writer"]
pub struct W(crate::W<AAD_BLOCK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AAD_BLOCK_NUM_SPEC>;
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
impl From<crate::W<AAD_BLOCK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AAD_BLOCK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAD_BLOCK_NUM` reader - Those bits stores the number of AAD block."]
pub type AAD_BLOCK_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AAD_BLOCK_NUM` writer - Those bits stores the number of AAD block."]
pub type AAD_BLOCK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AAD_BLOCK_NUM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the number of AAD block."]
    #[inline(always)]
    pub fn aad_block_num(&self) -> AAD_BLOCK_NUM_R {
        AAD_BLOCK_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the number of AAD block."]
    #[inline(always)]
    pub fn aad_block_num(&mut self) -> AAD_BLOCK_NUM_W<0> {
        AAD_BLOCK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional Authential Data block number register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aad_block_num](index.html) module"]
pub struct AAD_BLOCK_NUM_SPEC;
impl crate::RegisterSpec for AAD_BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aad_block_num::R](R) reader structure"]
impl crate::Readable for AAD_BLOCK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aad_block_num::W](W) writer structure"]
impl crate::Writable for AAD_BLOCK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AAD_BLOCK_NUM to value 0"]
impl crate::Resettable for AAD_BLOCK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}