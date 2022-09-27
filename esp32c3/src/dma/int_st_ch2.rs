#[doc = "Register `INT_ST_CH2` reader"]
pub struct R(crate::R<INT_ST_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_DONE_CH2_INT_ST` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_CH2_INT_ST` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_CH2_INT_ST` reader - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_CH2_INT_ST` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH2_INT_ST` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR_CH2_INT_ST` reader - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH2_INT_ST` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY_CH2_INT_ST` reader - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH2_INT_ST` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF_CH2_INT_ST` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF_CH2_INT_ST` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_CH2_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_CH2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_CH2_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_CH2_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch2_int_st(&self) -> IN_DONE_CH2_INT_ST_R {
        IN_DONE_CH2_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch2_int_st(&self) -> IN_SUC_EOF_CH2_INT_ST_R {
        IN_SUC_EOF_CH2_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch2_int_st(&self) -> IN_ERR_EOF_CH2_INT_ST_R {
        IN_ERR_EOF_CH2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch2_int_st(&self) -> OUT_DONE_CH2_INT_ST_R {
        OUT_DONE_CH2_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch2_int_st(&self) -> OUT_EOF_CH2_INT_ST_R {
        OUT_EOF_CH2_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch2_int_st(&self) -> IN_DSCR_ERR_CH2_INT_ST_R {
        IN_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch2_int_st(&self) -> OUT_DSCR_ERR_CH2_INT_ST_R {
        OUT_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch2_int_st(&self) -> IN_DSCR_EMPTY_CH2_INT_ST_R {
        IN_DSCR_EMPTY_CH2_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch2_int_st(&self) -> OUT_TOTAL_EOF_CH2_INT_ST_R {
        OUT_TOTAL_EOF_CH2_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_ch2_int_st(&self) -> INFIFO_OVF_CH2_INT_ST_R {
        INFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_ch2_int_st(&self) -> INFIFO_UDF_CH2_INT_ST_R {
        INFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch2_int_st(&self) -> OUTFIFO_OVF_CH2_INT_ST_R {
        OUTFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch2_int_st(&self) -> OUTFIFO_UDF_CH2_INT_ST_R {
        OUTFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DMA_INT_ST_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_ch2](index.html) module"]
pub struct INT_ST_CH2_SPEC;
impl crate::RegisterSpec for INT_ST_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_ch2::R](R) reader structure"]
impl crate::Readable for INT_ST_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_CH2 to value 0"]
impl crate::Resettable for INT_ST_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
