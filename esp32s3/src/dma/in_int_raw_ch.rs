#[doc = "Register `IN_INT_RAW_CH%s` reader"]
pub struct R(crate::R<IN_INT_RAW_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_RAW_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_RAW_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_RAW_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_DONE_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_CH_INT_RAW` reader - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
pub type IN_ERR_EOF_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR_CH_INT_RAW` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_RAW` reader - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_FULL_WM_CH_INT_RAW` reader - The raw interrupt bit turns to high level when received data byte number is up to threshold configured by REG_DMA_INFIFO_FULL_THRS_CH0 in Rx FIFO of channel 0."]
pub type INFIFO_FULL_WM_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF_L1_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_L1_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF_L1_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_L1_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF_L3_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 3 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_L3_CH_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF_L3_CH_INT_RAW` reader - This raw interrupt bit turns to high level when level 3 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_L3_CH_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done_ch_int_raw(&self) -> IN_DONE_CH_INT_RAW_R {
        IN_DONE_CH_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof_ch_int_raw(&self) -> IN_SUC_EOF_CH_INT_RAW_R {
        IN_SUC_EOF_CH_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof_ch_int_raw(&self) -> IN_ERR_EOF_CH_INT_RAW_R {
        IN_ERR_EOF_CH_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err_ch_int_raw(&self) -> IN_DSCR_ERR_CH_INT_RAW_R {
        IN_DSCR_ERR_CH_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_raw(&self) -> IN_DSCR_EMPTY_CH_INT_RAW_R {
        IN_DSCR_EMPTY_CH_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when received data byte number is up to threshold configured by REG_DMA_INFIFO_FULL_THRS_CH0 in Rx FIFO of channel 0."]
    #[inline(always)]
    pub fn infifo_full_wm_ch_int_raw(&self) -> INFIFO_FULL_WM_CH_INT_RAW_R {
        INFIFO_FULL_WM_CH_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch_int_raw(&self) -> INFIFO_OVF_L1_CH_INT_RAW_R {
        INFIFO_OVF_L1_CH_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch_int_raw(&self) -> INFIFO_UDF_L1_CH_INT_RAW_R {
        INFIFO_UDF_L1_CH_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 3 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l3_ch_int_raw(&self) -> INFIFO_OVF_L3_CH_INT_RAW_R {
        INFIFO_OVF_L3_CH_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 3 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l3_ch_int_raw(&self) -> INFIFO_UDF_L3_CH_INT_RAW_R {
        INFIFO_UDF_L3_CH_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Raw status interrupt of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_raw_ch](index.html) module"]
pub struct IN_INT_RAW_CH_SPEC;
impl crate::RegisterSpec for IN_INT_RAW_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int_raw_ch::R](R) reader structure"]
impl crate::Readable for IN_INT_RAW_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_INT_RAW_CH%s to value 0"]
impl crate::Resettable for IN_INT_RAW_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
