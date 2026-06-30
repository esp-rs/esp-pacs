#[doc = "Register `AHB_DMA_IN_INT_ST_CH%s` reader"]
pub type R = crate::R<AHB_DMA_IN_INT_ST_CH_SPEC>;
#[doc = "Field `AHB_DMA_IN_DONE_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DONE_CH%s_INT."]
pub type AHB_DMA_IN_DONE_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_SUC_EOF_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type AHB_DMA_IN_SUC_EOF_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_ERR_EOF_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type AHB_DMA_IN_ERR_EOF_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_DSCR_ERR_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type AHB_DMA_IN_DSCR_ERR_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_DSCR_EMPTY_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type AHB_DMA_IN_DSCR_EMPTY_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_INFIFO_OVF_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type AHB_DMA_INFIFO_OVF_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_INFIFO_UDF_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type AHB_DMA_INFIFO_UDF_CH_INT_ST_R = crate::BitReader;
#[doc = "Field `AHB_DMA_IN_AHBINF_RESP_ERR_CH_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_RESP_ERR_CH%s_INT."]
pub type AHB_DMA_IN_AHBINF_RESP_ERR_CH_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_done_ch_int_st(&self) -> AHB_DMA_IN_DONE_CH_INT_ST_R {
        AHB_DMA_IN_DONE_CH_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_suc_eof_ch_int_st(&self) -> AHB_DMA_IN_SUC_EOF_CH_INT_ST_R {
        AHB_DMA_IN_SUC_EOF_CH_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_err_eof_ch_int_st(&self) -> AHB_DMA_IN_ERR_EOF_CH_INT_ST_R {
        AHB_DMA_IN_ERR_EOF_CH_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_err_ch_int_st(&self) -> AHB_DMA_IN_DSCR_ERR_CH_INT_ST_R {
        AHB_DMA_IN_DSCR_ERR_CH_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_empty_ch_int_st(&self) -> AHB_DMA_IN_DSCR_EMPTY_CH_INT_ST_R {
        AHB_DMA_IN_DSCR_EMPTY_CH_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_infifo_ovf_ch_int_st(&self) -> AHB_DMA_INFIFO_OVF_CH_INT_ST_R {
        AHB_DMA_INFIFO_OVF_CH_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_infifo_udf_ch_int_st(&self) -> AHB_DMA_INFIFO_UDF_CH_INT_ST_R {
        AHB_DMA_INFIFO_UDF_CH_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status of AHB_DMA_IN_RESP_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn ahb_dma_in_ahbinf_resp_err_ch_int_st(&self) -> AHB_DMA_IN_AHBINF_RESP_ERR_CH_INT_ST_R {
        AHB_DMA_IN_AHBINF_RESP_ERR_CH_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_IN_INT_ST_CH")
            .field(
                "ahb_dma_in_done_ch_int_st",
                &self.ahb_dma_in_done_ch_int_st(),
            )
            .field(
                "ahb_dma_in_suc_eof_ch_int_st",
                &self.ahb_dma_in_suc_eof_ch_int_st(),
            )
            .field(
                "ahb_dma_in_err_eof_ch_int_st",
                &self.ahb_dma_in_err_eof_ch_int_st(),
            )
            .field(
                "ahb_dma_in_dscr_err_ch_int_st",
                &self.ahb_dma_in_dscr_err_ch_int_st(),
            )
            .field(
                "ahb_dma_in_dscr_empty_ch_int_st",
                &self.ahb_dma_in_dscr_empty_ch_int_st(),
            )
            .field(
                "ahb_dma_infifo_ovf_ch_int_st",
                &self.ahb_dma_infifo_ovf_ch_int_st(),
            )
            .field(
                "ahb_dma_infifo_udf_ch_int_st",
                &self.ahb_dma_infifo_udf_ch_int_st(),
            )
            .field(
                "ahb_dma_in_ahbinf_resp_err_ch_int_st",
                &self.ahb_dma_in_ahbinf_resp_err_ch_int_st(),
            )
            .finish()
    }
}
#[doc = "Masked interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_int_st_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_IN_INT_ST_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_IN_INT_ST_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_in_int_st_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_IN_INT_ST_CH_SPEC {}
#[doc = "`reset()` method sets AHB_DMA_IN_INT_ST_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_IN_INT_ST_CH_SPEC {}
