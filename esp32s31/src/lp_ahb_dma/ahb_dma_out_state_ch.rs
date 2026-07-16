#[doc = "Register `AHB_DMA_OUT_STATE_CH%s` reader"]
pub type R = crate::R<AHB_DMA_OUT_STATE_CH_SPEC>;
#[doc = "Field `AHB_DMA_OUTLINK_DSCR_ADDR_CH` reader - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
pub type AHB_DMA_OUTLINK_DSCR_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `AHB_DMA_OUT_DSCR_STATE_CH` reader - reserved"]
pub type AHB_DMA_OUT_DSCR_STATE_CH_R = crate::FieldReader;
#[doc = "Field `AHB_DMA_OUT_STATE_CH` reader - reserved"]
pub type AHB_DMA_OUT_STATE_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - Represents the lower 18 bits of the address of the next transmit descriptor to be processed."]
    #[inline(always)]
    pub fn ahb_dma_outlink_dscr_addr_ch(&self) -> AHB_DMA_OUTLINK_DSCR_ADDR_CH_R {
        AHB_DMA_OUTLINK_DSCR_ADDR_CH_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_state_ch(&self) -> AHB_DMA_OUT_DSCR_STATE_CH_R {
        AHB_DMA_OUT_DSCR_STATE_CH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn ahb_dma_out_state_ch(&self) -> AHB_DMA_OUT_STATE_CH_R {
        AHB_DMA_OUT_STATE_CH_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_OUT_STATE_CH")
            .field(
                "ahb_dma_outlink_dscr_addr_ch",
                &self.ahb_dma_outlink_dscr_addr_ch(),
            )
            .field(
                "ahb_dma_out_dscr_state_ch",
                &self.ahb_dma_out_dscr_state_ch(),
            )
            .field("ahb_dma_out_state_ch", &self.ahb_dma_out_state_ch())
            .finish()
    }
}
#[doc = "Transmit status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_state_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_OUT_STATE_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_OUT_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_out_state_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_OUT_STATE_CH_SPEC {}
#[doc = "`reset()` method sets AHB_DMA_OUT_STATE_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_OUT_STATE_CH_SPEC {}
