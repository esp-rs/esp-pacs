#[doc = "Register `DMA_OUT_EOF_BFR_DES_ADDR` reader"]
pub type R = crate::R<DMA_OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "Field `OUT_EOF_BFR_DES_ADDR` reader - This register stores the address of the transmit descriptor before the last transmit descriptor."]
pub type OUT_EOF_BFR_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the transmit descriptor before the last transmit descriptor."]
    #[inline(always)]
    pub fn out_eof_bfr_des_addr(&self) -> OUT_EOF_BFR_DES_ADDR_R {
        OUT_EOF_BFR_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_EOF_BFR_DES_ADDR")
            .field("out_eof_bfr_des_addr", &self.out_eof_bfr_des_addr())
            .finish()
    }
}
#[doc = "Outlink descriptor address before the last transmit descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_eof_bfr_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_EOF_BFR_DES_ADDR_SPEC;
impl crate::RegisterSpec for DMA_OUT_EOF_BFR_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_eof_bfr_des_addr::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_EOF_BFR_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets DMA_OUT_EOF_BFR_DES_ADDR to value 0"]
impl crate::Resettable for DMA_OUT_EOF_BFR_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
