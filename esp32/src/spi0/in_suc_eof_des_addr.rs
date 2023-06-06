#[doc = "Register `IN_SUC_EOF_DES_ADDR` reader"]
pub struct R(crate::R<IN_SUC_EOF_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SUC_EOF_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SUC_EOF_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SUC_EOF_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_IN_SUC_EOF_DES_ADDR` reader - The last inlink descriptor address when spi dma produce from_suc_eof."]
pub type DMA_IN_SUC_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The last inlink descriptor address when spi dma produce from_suc_eof."]
    #[inline(always)]
    pub fn dma_in_suc_eof_des_addr(&self) -> DMA_IN_SUC_EOF_DES_ADDR_R {
        DMA_IN_SUC_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SUC_EOF_DES_ADDR")
            .field(
                "dma_in_suc_eof_des_addr",
                &format_args!("{}", self.dma_in_suc_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_SUC_EOF_DES_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_suc_eof_des_addr](index.html) module"]
pub struct IN_SUC_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for IN_SUC_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_suc_eof_des_addr::R](R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR to value 0"]
impl crate::Resettable for IN_SUC_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
