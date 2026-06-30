#[doc = "Register `AHB_DMA_AHBINF_RESP_ERR_STATUS1` reader"]
pub type R = crate::R<AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC>;
#[doc = "Field `AHB_DMA_AHBINF_RESP_ERR_WR` reader - Represents the AHB response error is write request."]
pub type AHB_DMA_AHBINF_RESP_ERR_WR_R = crate::BitReader;
#[doc = "Field `AHB_DMA_AHBINF_RESP_ERR_ID` reader - Represents the AHB response error request id."]
pub type AHB_DMA_AHBINF_RESP_ERR_ID_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_AHBINF_RESP_ERR_CH_ID` reader - Represents the AHB response error request channel id.bit\\[1\\]=1:TX channel.\\\\bit\\[1\\]=0:RX channel.\\\\"]
pub type AHB_DMA_AHBINF_RESP_ERR_CH_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represents the AHB response error is write request."]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_resp_err_wr(&self) -> AHB_DMA_AHBINF_RESP_ERR_WR_R {
        AHB_DMA_AHBINF_RESP_ERR_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Represents the AHB response error request id."]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_resp_err_id(&self) -> AHB_DMA_AHBINF_RESP_ERR_ID_R {
        AHB_DMA_AHBINF_RESP_ERR_ID_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Represents the AHB response error request channel id.bit\\[1\\]=1:TX channel.\\\\bit\\[1\\]=0:RX channel.\\\\"]
    #[inline(always)]
    pub fn ahb_dma_ahbinf_resp_err_ch_id(&self) -> AHB_DMA_AHBINF_RESP_ERR_CH_ID_R {
        AHB_DMA_AHBINF_RESP_ERR_CH_ID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_AHBINF_RESP_ERR_STATUS1")
            .field(
                "ahb_dma_ahbinf_resp_err_wr",
                &self.ahb_dma_ahbinf_resp_err_wr(),
            )
            .field(
                "ahb_dma_ahbinf_resp_err_id",
                &self.ahb_dma_ahbinf_resp_err_id(),
            )
            .field(
                "ahb_dma_ahbinf_resp_err_ch_id",
                &self.ahb_dma_ahbinf_resp_err_ch_id(),
            )
            .finish()
    }
}
#[doc = "AHB response error status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ahbinf_resp_err_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC;
impl crate::RegisterSpec for AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_ahbinf_resp_err_status1::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC {}
#[doc = "`reset()` method sets AHB_DMA_AHBINF_RESP_ERR_STATUS1 to value 0x3e"]
impl crate::Resettable for AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0x3e;
}
