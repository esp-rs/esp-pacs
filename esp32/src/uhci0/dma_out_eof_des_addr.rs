#[doc = "Register `DMA_OUT_EOF_DES_ADDR` reader"]
pub struct R(crate::R<DMA_OUT_EOF_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_EOF_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_EOF_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_EOF_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_EOF_DES_ADDR` reader - This register stores the address of out link descriptoir when eof bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of out link descriptoir when eof bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr(&self) -> OUT_EOF_DES_ADDR_R {
        OUT_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_eof_des_addr](index.html) module"]
pub struct DMA_OUT_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for DMA_OUT_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_eof_des_addr::R](R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_EOF_DES_ADDR to value 0"]
impl crate::Resettable for DMA_OUT_EOF_DES_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
