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
#[doc = "Field `AAD_BLOCK_NUM` reader - Stores the ADD Block Number for the GCM operation."]
pub type AAD_BLOCK_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AAD_BLOCK_NUM` writer - Stores the ADD Block Number for the GCM operation."]
pub type AAD_BLOCK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, AAD_BLOCK_NUM_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the ADD Block Number for the GCM operation."]
    #[inline(always)]
    pub fn aad_block_num(&self) -> AAD_BLOCK_NUM_R {
        AAD_BLOCK_NUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AAD_BLOCK_NUM")
            .field(
                "aad_block_num",
                &format_args!("{}", self.aad_block_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AAD_BLOCK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the ADD Block Number for the GCM operation."]
    #[inline(always)]
    #[must_use]
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
#[doc = "AAD block number configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aad_block_num](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AAD_BLOCK_NUM to value 0"]
impl crate::Resettable for AAD_BLOCK_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
