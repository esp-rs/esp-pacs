#[doc = "Register `ST` reader"]
pub type R = crate::R<ST_SPEC>;
#[doc = "Field `IN_DONE_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
pub type IN_DONE_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type IN_SUC_EOF_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type IN_ERR_EOF_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type IN_DSCR_ERR_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type IN_DSCR_EMPTY_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type INFIFO_OVF_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type INFIFO_UDF_CH0_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_ST` reader - The masked interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type IN_AHBINF_RESP_ERR_CH0_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_st(&self) -> IN_DONE_CH0_INT_ST_R {
        IN_DONE_CH0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_st(&self) -> IN_SUC_EOF_CH0_INT_ST_R {
        IN_SUC_EOF_CH0_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_st(&self) -> IN_ERR_EOF_CH0_INT_ST_R {
        IN_ERR_EOF_CH0_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_st(&self) -> IN_DSCR_ERR_CH0_INT_ST_R {
        IN_DSCR_ERR_CH0_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_st(&self) -> IN_DSCR_EMPTY_CH0_INT_ST_R {
        IN_DSCR_EMPTY_CH0_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_st(&self) -> INFIFO_OVF_CH0_INT_ST_R {
        INFIFO_OVF_CH0_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_st(&self) -> INFIFO_UDF_CH0_INT_ST_R {
        INFIFO_UDF_CH0_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_st(&self) -> IN_AHBINF_RESP_ERR_CH0_INT_ST_R {
        IN_AHBINF_RESP_ERR_CH0_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ST")
            .field("in_done_ch0_int_st", &self.in_done_ch0_int_st())
            .field("in_suc_eof_ch0_int_st", &self.in_suc_eof_ch0_int_st())
            .field("in_err_eof_ch0_int_st", &self.in_err_eof_ch0_int_st())
            .field("in_dscr_err_ch0_int_st", &self.in_dscr_err_ch0_int_st())
            .field("in_dscr_empty_ch0_int_st", &self.in_dscr_empty_ch0_int_st())
            .field("infifo_ovf_ch0_int_st", &self.infifo_ovf_ch0_int_st())
            .field("infifo_udf_ch0_int_st", &self.infifo_udf_ch0_int_st())
            .field(
                "in_ahbinf_resp_err_ch0_int_st",
                &self.in_ahbinf_resp_err_ch0_int_st(),
            )
            .finish()
    }
}
#[doc = "Masked interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for ST_SPEC {}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {}
