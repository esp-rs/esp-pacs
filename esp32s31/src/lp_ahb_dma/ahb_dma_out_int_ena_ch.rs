#[doc = "Register `AHB_DMA_OUT_INT_ENA_CH%s` reader"]
pub type R = crate::R<AHB_DMA_OUT_INT_ENA_CH_SPEC>;
#[doc = "Register `AHB_DMA_OUT_INT_ENA_CH%s` writer"]
pub type W = crate::W<AHB_DMA_OUT_INT_ENA_CH_SPEC>;
#[doc = "Field `AHB_DMA_OUT_DONE_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
pub type AHB_DMA_OUT_DONE_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_DONE_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
pub type AHB_DMA_OUT_DONE_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_EOF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_EOF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUTFIFO_OVF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUTFIFO_OVF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUTFIFO_UDF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUTFIFO_UDF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
pub type AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_done_ch_int_ena(&self) -> AHB_DMA_OUT_DONE_CH_INT_ENA_R {
        AHB_DMA_OUT_DONE_CH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_eof_ch_int_ena(&self) -> AHB_DMA_OUT_EOF_CH_INT_ENA_R {
        AHB_DMA_OUT_EOF_CH_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_err_ch_int_ena(&self) -> AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_R {
        AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_total_eof_ch_int_ena(&self) -> AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_R {
        AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_ovf_ch_int_ena(&self) -> AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_R {
        AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_udf_ch_int_ena(&self) -> AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_R {
        AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_ahbinf_resp_err_ch_int_ena(
        &self,
    ) -> AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_R {
        AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_OUT_INT_ENA_CH")
            .field(
                "ahb_dma_out_done_ch_int_ena",
                &self.ahb_dma_out_done_ch_int_ena(),
            )
            .field(
                "ahb_dma_out_eof_ch_int_ena",
                &self.ahb_dma_out_eof_ch_int_ena(),
            )
            .field(
                "ahb_dma_out_dscr_err_ch_int_ena",
                &self.ahb_dma_out_dscr_err_ch_int_ena(),
            )
            .field(
                "ahb_dma_out_total_eof_ch_int_ena",
                &self.ahb_dma_out_total_eof_ch_int_ena(),
            )
            .field(
                "ahb_dma_outfifo_ovf_ch_int_ena",
                &self.ahb_dma_outfifo_ovf_ch_int_ena(),
            )
            .field(
                "ahb_dma_outfifo_udf_ch_int_ena",
                &self.ahb_dma_outfifo_udf_ch_int_ena(),
            )
            .field(
                "ahb_dma_out_ahbinf_resp_err_ch_int_ena",
                &self.ahb_dma_out_ahbinf_resp_err_ch_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_done_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUT_DONE_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUT_DONE_CH_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_eof_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUT_EOF_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUT_EOF_CH_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_err_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUT_DSCR_ERR_CH_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_total_eof_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUT_TOTAL_EOF_CH_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_ovf_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUTFIFO_OVF_CH_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_outfifo_udf_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUTFIFO_UDF_CH_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_out_ahbinf_resp_err_ch_int_ena(
        &mut self,
    ) -> AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_W<'_, AHB_DMA_OUT_INT_ENA_CH_SPEC> {
        AHB_DMA_OUT_AHBINF_RESP_ERR_CH_INT_ENA_W::new(self, 6)
    }
}
#[doc = "Interrupt enable bits of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_int_ena_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_int_ena_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_OUT_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_OUT_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_out_int_ena_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_OUT_INT_ENA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_out_int_ena_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_OUT_INT_ENA_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_OUT_INT_ENA_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_OUT_INT_ENA_CH_SPEC {}
