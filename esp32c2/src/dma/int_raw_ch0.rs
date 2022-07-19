#[doc = "Register `INT_RAW_CH0` reader"]
pub struct R(crate::R<INT_RAW_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_DONE_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
pub type IN_ERR_EOF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_RAW` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF_CH0_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF_CH0_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_CH0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_CH0_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done_ch0_int_raw(&self) -> IN_DONE_CH0_INT_RAW_R {
        IN_DONE_CH0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_raw(&self) -> IN_SUC_EOF_CH0_INT_RAW_R {
        IN_SUC_EOF_CH0_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_raw(&self) -> IN_ERR_EOF_CH0_INT_RAW_R {
        IN_ERR_EOF_CH0_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done_ch0_int_raw(&self) -> OUT_DONE_CH0_INT_RAW_R {
        OUT_DONE_CH0_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof_ch0_int_raw(&self) -> OUT_EOF_CH0_INT_RAW_R {
        OUT_EOF_CH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_raw(&self) -> IN_DSCR_ERR_CH0_INT_RAW_R {
        IN_DSCR_ERR_CH0_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_raw(&self) -> OUT_DSCR_ERR_CH0_INT_RAW_R {
        OUT_DSCR_ERR_CH0_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_raw(&self) -> IN_DSCR_EMPTY_CH0_INT_RAW_R {
        IN_DSCR_EMPTY_CH0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_raw(&self) -> OUT_TOTAL_EOF_CH0_INT_RAW_R {
        OUT_TOTAL_EOF_CH0_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_raw(&self) -> INFIFO_OVF_CH0_INT_RAW_R {
        INFIFO_OVF_CH0_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_raw(&self) -> INFIFO_UDF_CH0_INT_RAW_R {
        INFIFO_UDF_CH0_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_raw(&self) -> OUTFIFO_OVF_CH0_INT_RAW_R {
        OUTFIFO_OVF_CH0_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_raw(&self) -> OUTFIFO_UDF_CH0_INT_RAW_R {
        OUTFIFO_UDF_CH0_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DMA_INT_RAW_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_ch0](index.html) module"]
pub struct INT_RAW_CH0_SPEC;
impl crate::RegisterSpec for INT_RAW_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_ch0::R](R) reader structure"]
impl crate::Readable for INT_RAW_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW_CH0 to value 0"]
impl crate::Resettable for INT_RAW_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
