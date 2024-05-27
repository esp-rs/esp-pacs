///Register `OUT_EOF_BFR_DES_ADDR` reader
pub type R = crate::R<OUT_EOF_BFR_DES_ADDR_SPEC>;
///Field `DMA_OUT_EOF_BFR_DES_ADDR` reader - The address of buffer relative to the outlink descriptor that produce eof.
pub type DMA_OUT_EOF_BFR_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The address of buffer relative to the outlink descriptor that produce eof.
    #[inline(always)]
    pub fn dma_out_eof_bfr_des_addr(&self) -> DMA_OUT_EOF_BFR_DES_ADDR_R {
        DMA_OUT_EOF_BFR_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_BFR_DES_ADDR")
            .field("dma_out_eof_bfr_des_addr", &self.dma_out_eof_bfr_des_addr())
            .finish()
    }
}
/**The latest SPI DMA eof TX buffer address

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_EOF_BFR_DES_ADDR_SPEC;
impl crate::RegisterSpec for OUT_EOF_BFR_DES_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_eof_bfr_des_addr::R`](R) reader structure
impl crate::Readable for OUT_EOF_BFR_DES_ADDR_SPEC {}
///`reset()` method sets OUT_EOF_BFR_DES_ADDR to value 0
impl crate::Resettable for OUT_EOF_BFR_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
