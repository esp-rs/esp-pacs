#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_WM_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ST_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM_INT_ST` reader - The masked interrupt status bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `END_DETECT_INT_ST` reader - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ST_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE_INT_ST` reader - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - The masked interrupt status bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF_INT_ST` reader - The masked interrupt status bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - The masked interrupt status bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ST_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ST` reader - The masked interrupt status bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_START_INT_ST` reader - The masked interrupt status bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ST_R = crate::BitReader;
#[doc = "Field `NACK_INT_ST` reader - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ST_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF_INT_ST` reader - The masked interrupt status bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF_INT_ST` reader - The masked interrupt status bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO_INT_ST` reader - The masked interrupt status bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ST_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ST` reader - The masked interrupt status bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ST_R = crate::BitReader;
#[doc = "Field `DET_START_INT_ST` reader - The masked interrupt status bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH_INT_ST` reader - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_INT_ST_R = crate::BitReader;
#[doc = "Field `GENERAL_CALL_INT_ST` reader - The masked interrupt status bit for I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_st(&self) -> RXFIFO_WM_INT_ST_R {
        RXFIFO_WM_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_st(&self) -> TXFIFO_WM_INT_ST_R {
        TXFIFO_WM_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_st(&self) -> BYTE_TRANS_DONE_INT_ST_R {
        BYTE_TRANS_DONE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_st(&self) -> MST_TXFIFO_UDF_INT_ST_R {
        MST_TXFIFO_UDF_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_st(&self) -> NACK_INT_ST_R {
        NACK_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_st(&self) -> TXFIFO_OVF_INT_ST_R {
        TXFIFO_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_st(&self) -> RXFIFO_UDF_INT_ST_R {
        RXFIFO_UDF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_st(&self) -> SCL_ST_TO_INT_ST_R {
        SCL_ST_TO_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_st(&self) -> SCL_MAIN_ST_TO_INT_ST_R {
        SCL_MAIN_ST_TO_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_st(&self) -> DET_START_INT_ST_R {
        DET_START_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch_int_st(&self) -> SLAVE_STRETCH_INT_ST_R {
        SLAVE_STRETCH_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status bit for I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call_int_st(&self) -> GENERAL_CALL_INT_ST_R {
        GENERAL_CALL_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field(
                "rxfifo_wm_int_st",
                &format_args!("{}", self.rxfifo_wm_int_st().bit()),
            )
            .field(
                "txfifo_wm_int_st",
                &format_args!("{}", self.txfifo_wm_int_st().bit()),
            )
            .field(
                "rxfifo_ovf_int_st",
                &format_args!("{}", self.rxfifo_ovf_int_st().bit()),
            )
            .field(
                "end_detect_int_st",
                &format_args!("{}", self.end_detect_int_st().bit()),
            )
            .field(
                "byte_trans_done_int_st",
                &format_args!("{}", self.byte_trans_done_int_st().bit()),
            )
            .field(
                "arbitration_lost_int_st",
                &format_args!("{}", self.arbitration_lost_int_st().bit()),
            )
            .field(
                "mst_txfifo_udf_int_st",
                &format_args!("{}", self.mst_txfifo_udf_int_st().bit()),
            )
            .field(
                "trans_complete_int_st",
                &format_args!("{}", self.trans_complete_int_st().bit()),
            )
            .field(
                "time_out_int_st",
                &format_args!("{}", self.time_out_int_st().bit()),
            )
            .field(
                "trans_start_int_st",
                &format_args!("{}", self.trans_start_int_st().bit()),
            )
            .field("nack_int_st", &format_args!("{}", self.nack_int_st().bit()))
            .field(
                "txfifo_ovf_int_st",
                &format_args!("{}", self.txfifo_ovf_int_st().bit()),
            )
            .field(
                "rxfifo_udf_int_st",
                &format_args!("{}", self.rxfifo_udf_int_st().bit()),
            )
            .field(
                "scl_st_to_int_st",
                &format_args!("{}", self.scl_st_to_int_st().bit()),
            )
            .field(
                "scl_main_st_to_int_st",
                &format_args!("{}", self.scl_main_st_to_int_st().bit()),
            )
            .field(
                "det_start_int_st",
                &format_args!("{}", self.det_start_int_st().bit()),
            )
            .field(
                "slave_stretch_int_st",
                &format_args!("{}", self.slave_stretch_int_st().bit()),
            )
            .field(
                "general_call_int_st",
                &format_args!("{}", self.general_call_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status of captured I2C communication events\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
