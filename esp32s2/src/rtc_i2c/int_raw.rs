#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_RAW` reader - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt raw bit"]
pub type SLAVE_TRAN_COMP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - RTC_I2C_ARBITRATION_LOST_INT interrupt raw bit"]
pub type ARBITRATION_LOST_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRAN_COMP_INT_RAW` reader - RTC_I2C_MASTER_TRAN_COMP_INT interrupt raw bit"]
pub type MASTER_TRAN_COMP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - RTC_I2C_TRANS_COMPLETE_INT interrupt raw bit"]
pub type TRANS_COMPLETE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_RAW` reader - RTC_I2C_TIME_OUT_INT interrupt raw bit"]
pub type TIME_OUT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR_INT_RAW` reader - RTC_I2C_ACK_ERR_INT interrupt raw bit"]
pub type ACK_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_INT_RAW` reader - RTC_I2C_RX_DATA_INT interrupt raw bit"]
pub type RX_DATA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_INT_RAW` reader - RTC_I2C_TX_DATA_INT interrupt raw bit"]
pub type TX_DATA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `DETECT_START_INT_RAW` reader - RTC_I2C_DETECT_START_INT interrupt raw bit"]
pub type DETECT_START_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt raw bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_raw(&self) -> SLAVE_TRAN_COMP_INT_RAW_R {
        SLAVE_TRAN_COMP_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt raw bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt raw bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_raw(&self) -> MASTER_TRAN_COMP_INT_RAW_R {
        MASTER_TRAN_COMP_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt raw bit"]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt raw bit"]
    #[inline(always)]
    pub fn ack_err_int_raw(&self) -> ACK_ERR_INT_RAW_R {
        ACK_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt raw bit"]
    #[inline(always)]
    pub fn rx_data_int_raw(&self) -> RX_DATA_INT_RAW_R {
        RX_DATA_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt raw bit"]
    #[inline(always)]
    pub fn tx_data_int_raw(&self) -> TX_DATA_INT_RAW_R {
        TX_DATA_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt raw bit"]
    #[inline(always)]
    pub fn detect_start_int_raw(&self) -> DETECT_START_INT_RAW_R {
        DETECT_START_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC I2C raw interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
