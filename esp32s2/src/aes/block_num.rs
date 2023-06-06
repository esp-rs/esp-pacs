#[doc = "Register `BLOCK_NUM` reader"]
pub struct R(crate::R<BLOCK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_NUM` writer"]
pub struct W(crate::W<BLOCK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_NUM_SPEC>;
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
impl From<crate::W<BLOCK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_NUM` reader - Stores the Block Number of plaintext or cipertext when the AES Accelerator operates under the DMA-AES working mode. For details, see Section 1.5.4."]
pub type BLOCK_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCK_NUM` writer - Stores the Block Number of plaintext or cipertext when the AES Accelerator operates under the DMA-AES working mode. For details, see Section 1.5.4."]
pub type BLOCK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, BLOCK_NUM_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the Block Number of plaintext or cipertext when the AES Accelerator operates under the DMA-AES working mode. For details, see Section 1.5.4."]
    #[inline(always)]
    pub fn block_num(&self) -> BLOCK_NUM_R {
        BLOCK_NUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_NUM")
            .field("block_num", &format_args!("{}", self.block_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLOCK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the Block Number of plaintext or cipertext when the AES Accelerator operates under the DMA-AES working mode. For details, see Section 1.5.4."]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<0> {
        BLOCK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block number configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_num](index.html) module"]
pub struct BLOCK_NUM_SPEC;
impl crate::RegisterSpec for BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [block_num::R](R) reader structure"]
impl crate::Readable for BLOCK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_num::W](W) writer structure"]
impl crate::Writable for BLOCK_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCK_NUM to value 0"]
impl crate::Resettable for BLOCK_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
