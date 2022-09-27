#[doc = "Register `OUT_INT_ST_CH%s` reader"]
pub struct R(crate::R<OUT_INT_ST_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_INT_ST_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_INT_ST_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_INT_ST_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_DONE_CH_INT_ST` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH_INT_ST` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ST` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ST` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L1_CH_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L1_CH_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L3_CH_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_CH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L3_CH_INT_ST` reader - The raw interrupt status bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_CH_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_st(&self) -> OUT_DONE_CH_INT_ST_R {
        OUT_DONE_CH_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_st(&self) -> OUT_EOF_CH_INT_ST_R {
        OUT_EOF_CH_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_st(&self) -> OUT_DSCR_ERR_CH_INT_ST_R {
        OUT_DSCR_ERR_CH_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_st(&self) -> OUT_TOTAL_EOF_CH_INT_ST_R {
        OUT_TOTAL_EOF_CH_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch_int_st(&self) -> OUTFIFO_OVF_L1_CH_INT_ST_R {
        OUTFIFO_OVF_L1_CH_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch_int_st(&self) -> OUTFIFO_UDF_L1_CH_INT_ST_R {
        OUTFIFO_UDF_L1_CH_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3_ch_int_st(&self) -> OUTFIFO_OVF_L3_CH_INT_ST_R {
        OUTFIFO_OVF_L3_CH_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3_ch_int_st(&self) -> OUTFIFO_UDF_L3_CH_INT_ST_R {
        OUTFIFO_UDF_L3_CH_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Masked interrupt of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int_st_ch](index.html) module"]
pub struct OUT_INT_ST_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_ST_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_int_st_ch::R](R) reader structure"]
impl crate::Readable for OUT_INT_ST_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_INT_ST_CH%s to value 0"]
impl crate::Resettable for OUT_INT_ST_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
