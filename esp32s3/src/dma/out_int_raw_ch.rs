#[doc = "Register `OUT_INT_RAW_CH%s` reader"]
pub struct R(crate::R<OUT_INT_RAW_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_INT_RAW_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_INT_RAW_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_INT_RAW_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_DONE_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_RAW` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L1_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L1_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L1_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L1_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L3_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L3_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L3_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L3_CH_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done_ch_int_raw(&self) -> OUT_DONE_CH_INT_RAW_R {
        OUT_DONE_CH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof_ch_int_raw(&self) -> OUT_EOF_CH_INT_RAW_R {
        OUT_EOF_CH_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_raw(&self) -> OUT_DSCR_ERR_CH_INT_RAW_R {
        OUT_DSCR_ERR_CH_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_raw(&self) -> OUT_TOTAL_EOF_CH_INT_RAW_R {
        OUT_TOTAL_EOF_CH_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1_ch_int_raw(&self) -> OUTFIFO_OVF_L1_CH_INT_RAW_R {
        OUTFIFO_OVF_L1_CH_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1_ch_int_raw(&self) -> OUTFIFO_UDF_L1_CH_INT_RAW_R {
        OUTFIFO_UDF_L1_CH_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l3_ch_int_raw(&self) -> OUTFIFO_OVF_L3_CH_INT_RAW_R {
        OUTFIFO_OVF_L3_CH_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l3_ch_int_raw(&self) -> OUTFIFO_UDF_L3_CH_INT_RAW_R {
        OUTFIFO_UDF_L3_CH_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Raw status interrupt of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int_raw_ch](index.html) module"]
pub struct OUT_INT_RAW_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_RAW_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_int_raw_ch::R](R) reader structure"]
impl crate::Readable for OUT_INT_RAW_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_INT_RAW_CH%s to value 0"]
impl crate::Resettable for OUT_INT_RAW_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
