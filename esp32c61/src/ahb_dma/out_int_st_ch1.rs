#[doc = "Register `OUT_INT_ST_CH1` reader"]
pub type R = crate::R<OUT_INT_ST_CH1_SPEC>;
#[doc = "Field `OUT_DONE_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUT_DONE_CH1_INT"]
pub type OUT_DONE_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUT_EOF_CH1_INT"]
pub type OUT_EOF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUT_DSCR_ERR_CH1_INT"]
pub type OUT_DSCR_ERR_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH1_INT"]
pub type OUT_TOTAL_EOF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUTFIFO_OVF_CH1_INT"]
pub type OUTFIFO_OVF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUTFIFO_UDF_CH1_INT"]
pub type OUTFIFO_UDF_CH1_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH1_INT_ST` reader - The masked interrupt status of AHB_DMA_OUT_RESP_ERR_CH1_INT"]
pub type OUT_AHBINF_RESP_ERR_CH1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of AHB_DMA_OUT_DONE_CH1_INT"]
    #[inline(always)]
    pub fn out_done_ch1_int_st(&self) -> OUT_DONE_CH1_INT_ST_R {
        OUT_DONE_CH1_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of AHB_DMA_OUT_EOF_CH1_INT"]
    #[inline(always)]
    pub fn out_eof_ch1_int_st(&self) -> OUT_EOF_CH1_INT_ST_R {
        OUT_EOF_CH1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of AHB_DMA_OUT_DSCR_ERR_CH1_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch1_int_st(&self) -> OUT_DSCR_ERR_CH1_INT_ST_R {
        OUT_DSCR_ERR_CH1_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH1_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch1_int_st(&self) -> OUT_TOTAL_EOF_CH1_INT_ST_R {
        OUT_TOTAL_EOF_CH1_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of AHB_DMA_OUTFIFO_OVF_CH1_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch1_int_st(&self) -> OUTFIFO_OVF_CH1_INT_ST_R {
        OUTFIFO_OVF_CH1_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of AHB_DMA_OUTFIFO_UDF_CH1_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch1_int_st(&self) -> OUTFIFO_UDF_CH1_INT_ST_R {
        OUTFIFO_UDF_CH1_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of AHB_DMA_OUT_RESP_ERR_CH1_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch1_int_st(&self) -> OUT_AHBINF_RESP_ERR_CH1_INT_ST_R {
        OUT_AHBINF_RESP_ERR_CH1_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_ST_CH1")
            .field("out_done_ch1_int_st", &self.out_done_ch1_int_st())
            .field("out_eof_ch1_int_st", &self.out_eof_ch1_int_st())
            .field("out_dscr_err_ch1_int_st", &self.out_dscr_err_ch1_int_st())
            .field("out_total_eof_ch1_int_st", &self.out_total_eof_ch1_int_st())
            .field("outfifo_ovf_ch1_int_st", &self.outfifo_ovf_ch1_int_st())
            .field("outfifo_udf_ch1_int_st", &self.outfifo_udf_ch1_int_st())
            .field(
                "out_ahbinf_resp_err_ch1_int_st",
                &self.out_ahbinf_resp_err_ch1_int_st(),
            )
            .finish()
    }
}
#[doc = "Masked interrupt status of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_st_ch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_ST_CH1_SPEC;
impl crate::RegisterSpec for OUT_INT_ST_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_st_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_INT_ST_CH1_SPEC {}
#[doc = "`reset()` method sets OUT_INT_ST_CH1 to value 0"]
impl crate::Resettable for OUT_INT_ST_CH1_SPEC {}
