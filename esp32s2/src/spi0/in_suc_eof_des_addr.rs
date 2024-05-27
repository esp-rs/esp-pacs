#[doc = "Register `IN_SUC_EOF_DES_ADDR` reader"]
pub type R = crate::R<IN_SUC_EOF_DES_ADDR_SPEC>;
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
            .field("dma_in_suc_eof_des_addr", &self.dma_in_suc_eof_des_addr())
            .finish()
    }
}
#[doc = "The latest SPI DMA eof RX descriptor address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SUC_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for IN_SUC_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_suc_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR to value 0"]
impl crate::Resettable for IN_SUC_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
