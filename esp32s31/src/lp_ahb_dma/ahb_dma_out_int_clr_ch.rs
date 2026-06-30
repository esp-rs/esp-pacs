#[doc = "Register `AHB_DMA_OUT_INT_CLR_CH%s` writer"]
pub type W = crate::W<AHB_DMA_OUT_INT_CLR_CH_SPEC>;
#[doc = "Field `AHB_DMA_OUT_DONE_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DONE_CH%s_INT."]
pub type AHB_DMA_OUT_DONE_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_EOF_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_EOF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_DSCR_ERR_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_DSCR_ERR_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_TOTAL_EOF_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_TOTAL_EOF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUTFIFO_OVF_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_OVF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUTFIFO_UDF_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_UDF_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_DMA_OUT_INT_CLR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_done_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUT_DONE_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUT_DONE_CH_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_eof_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUT_EOF_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUT_EOF_CH_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_err_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUT_DSCR_ERR_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUT_DSCR_ERR_CH_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_total_eof_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUT_TOTAL_EOF_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUT_TOTAL_EOF_CH_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_ovf_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUTFIFO_OVF_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUTFIFO_OVF_CH_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_udf_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUTFIFO_UDF_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUTFIFO_UDF_CH_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_ahbinf_resp_err_ch_int_clr(
        &mut self,
    ) -> AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_CLR_W<'_, AHB_DMA_OUT_INT_CLR_CH_SPEC> {
        AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_CLR_W::new(self, 6)
    }
}
#[doc = "Interrupt clear bits of TX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_int_clr_ch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_OUT_INT_CLR_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_OUT_INT_CLR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_out_int_clr_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_OUT_INT_CLR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_OUT_INT_CLR_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_OUT_INT_CLR_CH_SPEC {}
