#[doc = "Register `AHB_DMA_OUT_DONE_DES_ADDR_CH%s` reader"]
pub type R = crate::R<AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC>;
#[doc = "Field `AHB_DMA_OUT_DONE_DES_ADDR_CH` reader - Represents the address of the outlink descriptor when this descriptor is completed."]
pub type AHB_DMA_OUT_DONE_DES_ADDR_CH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the outlink descriptor when this descriptor is completed."]
    #[inline(always)]
    pub fn ahb_dma_out_done_des_addr_ch(&self) -> AHB_DMA_OUT_DONE_DES_ADDR_CH_R {
        AHB_DMA_OUT_DONE_DES_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_OUT_DONE_DES_ADDR_CH")
            .field(
                "ahb_dma_out_done_des_addr_ch",
                &self.ahb_dma_out_done_des_addr_ch(),
            )
            .finish()
    }
}
#[doc = "TX done outlink descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_done_des_addr_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_out_done_des_addr_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC {}
#[doc = "`reset()` method sets AHB_DMA_OUT_DONE_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC {}
