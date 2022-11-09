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
#[doc = "Field `OUT_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L1` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L1_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L1` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L1_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_L3` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L3_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_L3` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OUTFIFO_OVF_L1_R {
        OUTFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OUTFIFO_UDF_L1_R {
        OUTFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&self) -> OUTFIFO_OVF_L3_R {
        OUTFIFO_OVF_L3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&self) -> OUTFIFO_UDF_L3_R {
        OUTFIFO_UDF_L3_R::new(((self.bits >> 7) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
