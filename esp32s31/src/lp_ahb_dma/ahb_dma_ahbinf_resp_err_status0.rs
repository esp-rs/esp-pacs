#[doc = "Register `AHB_DMA_AHBINF_RESP_ERR_STATUS0` reader"]
pub type R = crate::R<AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC>;
#[doc = "Field `AHB_DMA_AHBINF_RESP_ERR_ADDR` reader - Represents the address of the AHB response error."]
pub type AHB_DMA_AHBINF_RESP_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the AHB response error."]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_resp_err_addr(&self) -> AHB_DMA_AHBINF_RESP_ERR_ADDR_R {
        AHB_DMA_AHBINF_RESP_ERR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_AHBINF_RESP_ERR_STATUS0")
            .field(
                "ahb_dma_ahbinf_resp_err_addr",
                &self.ahb_dma_ahbinf_resp_err_addr(),
            )
            .finish()
    }
}
#[doc = "AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ahbinf_resp_err_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC;
impl crate::RegisterSpec for AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_ahbinf_resp_err_status0::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC {}
#[doc = "`reset()` method sets AHB_DMA_AHBINF_RESP_ERR_STATUS0 to value 0"]
impl crate::Resettable for AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC {}
