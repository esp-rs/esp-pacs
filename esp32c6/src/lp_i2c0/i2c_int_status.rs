#[doc = "Register `I2C_INT_STATUS` reader"]
pub struct R(crate::R<I2C_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_RXFIFO_WM_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_WM_INT interrupt."]
pub type I2C_RXFIFO_WM_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_TXFIFO_WM_INT_ST` reader - The masked interrupt status bit for I2C_TXFIFO_WM_INT interrupt."]
pub type I2C_TXFIFO_WM_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_OVF_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type I2C_RXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_END_DETECT_INT_ST` reader - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_END_DETECT_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_BYTE_TRANS_DONE_INT_ST` reader - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_BYTE_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_ARBITRATION_LOST_INT_ST` reader - The masked interrupt status bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type I2C_ARBITRATION_LOST_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_MST_TXFIFO_UDF_INT_ST` reader - The masked interrupt status bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_MST_TXFIFO_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_TRANS_COMPLETE_INT_ST` reader - The masked interrupt status bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_TRANS_COMPLETE_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_TIME_OUT_INT_ST` reader - The masked interrupt status bit for the I2C_TIME_OUT_INT interrupt."]
pub type I2C_TIME_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_TRANS_START_INT_ST` reader - The masked interrupt status bit for the I2C_TRANS_START_INT interrupt."]
pub type I2C_TRANS_START_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_NACK_INT_ST` reader - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type I2C_NACK_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_TXFIFO_OVF_INT_ST` reader - The masked interrupt status bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type I2C_TXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_UDF_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type I2C_RXFIFO_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_SCL_ST_TO_INT_ST` reader - The masked interrupt status bit for I2C_SCL_ST_TO_INT interrupt."]
pub type I2C_SCL_ST_TO_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_INT_ST` reader - The masked interrupt status bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type I2C_SCL_MAIN_ST_TO_INT_ST_R = crate::BitReader;
#[doc = "Field `I2C_DET_START_INT_ST` reader - The masked interrupt status bit for I2C_DET_START_INT interrupt."]
pub type I2C_DET_START_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_wm_int_st(&self) -> I2C_RXFIFO_WM_INT_ST_R {
        I2C_RXFIFO_WM_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_wm_int_st(&self) -> I2C_TXFIFO_WM_INT_ST_R {
        I2C_TXFIFO_WM_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_st(&self) -> I2C_RXFIFO_OVF_INT_ST_R {
        I2C_RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_end_detect_int_st(&self) -> I2C_END_DETECT_INT_ST_R {
        I2C_END_DETECT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_byte_trans_done_int_st(&self) -> I2C_BYTE_TRANS_DONE_INT_ST_R {
        I2C_BYTE_TRANS_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_st(&self) -> I2C_ARBITRATION_LOST_INT_ST_R {
        I2C_ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn i2c_mst_txfifo_udf_int_st(&self) -> I2C_MST_TXFIFO_UDF_INT_ST_R {
        I2C_MST_TXFIFO_UDF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn i2c_trans_complete_int_st(&self) -> I2C_TRANS_COMPLETE_INT_ST_R {
        I2C_TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_time_out_int_st(&self) -> I2C_TIME_OUT_INT_ST_R {
        I2C_TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn i2c_trans_start_int_st(&self) -> I2C_TRANS_START_INT_ST_R {
        I2C_TRANS_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn i2c_nack_int_st(&self) -> I2C_NACK_INT_ST_R {
        I2C_NACK_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_ovf_int_st(&self) -> I2C_TXFIFO_OVF_INT_ST_R {
        I2C_TXFIFO_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_udf_int_st(&self) -> I2C_RXFIFO_UDF_INT_ST_R {
        I2C_RXFIFO_UDF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn i2c_scl_st_to_int_st(&self) -> I2C_SCL_ST_TO_INT_ST_R {
        I2C_SCL_ST_TO_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_int_st(&self) -> I2C_SCL_MAIN_ST_TO_INT_ST_R {
        I2C_SCL_MAIN_ST_TO_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn i2c_det_start_int_st(&self) -> I2C_DET_START_INT_ST_R {
        I2C_DET_START_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_INT_STATUS")
            .field(
                "i2c_rxfifo_wm_int_st",
                &format_args!("{}", self.i2c_rxfifo_wm_int_st().bit()),
            )
            .field(
                "i2c_txfifo_wm_int_st",
                &format_args!("{}", self.i2c_txfifo_wm_int_st().bit()),
            )
            .field(
                "i2c_rxfifo_ovf_int_st",
                &format_args!("{}", self.i2c_rxfifo_ovf_int_st().bit()),
            )
            .field(
                "i2c_end_detect_int_st",
                &format_args!("{}", self.i2c_end_detect_int_st().bit()),
            )
            .field(
                "i2c_byte_trans_done_int_st",
                &format_args!("{}", self.i2c_byte_trans_done_int_st().bit()),
            )
            .field(
                "i2c_arbitration_lost_int_st",
                &format_args!("{}", self.i2c_arbitration_lost_int_st().bit()),
            )
            .field(
                "i2c_mst_txfifo_udf_int_st",
                &format_args!("{}", self.i2c_mst_txfifo_udf_int_st().bit()),
            )
            .field(
                "i2c_trans_complete_int_st",
                &format_args!("{}", self.i2c_trans_complete_int_st().bit()),
            )
            .field(
                "i2c_time_out_int_st",
                &format_args!("{}", self.i2c_time_out_int_st().bit()),
            )
            .field(
                "i2c_trans_start_int_st",
                &format_args!("{}", self.i2c_trans_start_int_st().bit()),
            )
            .field(
                "i2c_nack_int_st",
                &format_args!("{}", self.i2c_nack_int_st().bit()),
            )
            .field(
                "i2c_txfifo_ovf_int_st",
                &format_args!("{}", self.i2c_txfifo_ovf_int_st().bit()),
            )
            .field(
                "i2c_rxfifo_udf_int_st",
                &format_args!("{}", self.i2c_rxfifo_udf_int_st().bit()),
            )
            .field(
                "i2c_scl_st_to_int_st",
                &format_args!("{}", self.i2c_scl_st_to_int_st().bit()),
            )
            .field(
                "i2c_scl_main_st_to_int_st",
                &format_args!("{}", self.i2c_scl_main_st_to_int_st().bit()),
            )
            .field(
                "i2c_det_start_int_st",
                &format_args!("{}", self.i2c_det_start_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_INT_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status of captured I2C communication events\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_status](index.html) module"]
pub struct I2C_INT_STATUS_SPEC;
impl crate::RegisterSpec for I2C_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_int_status::R](R) reader structure"]
impl crate::Readable for I2C_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_INT_STATUS to value 0"]
impl crate::Resettable for I2C_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
