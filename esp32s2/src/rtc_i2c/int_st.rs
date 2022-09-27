#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_ST` reader - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt status bit"]
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - RTC_I2C_ARBITRATION_LOST_INT interrupt status bit"]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - RTC_I2C_MASTER_TRAN_COMP_INT interrupt status bit"]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - RTC_I2C_TRANS_COMPLETE_INT interrupt status bit"]
pub type TRANS_COMPLETE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_ST` reader - RTC_I2C_TIME_OUT_INT interrupt status bit"]
pub type TIME_OUT_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR_INT_ST` reader - RTC_I2C_ACK_ERR_INT interrupt status bit"]
pub type ACK_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_INT_ST` reader - RTC_I2C_RX_DATA_INT interrupt status bit"]
pub type RX_DATA_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_INT_ST` reader - RTC_I2C_TX_DATA_INT interrupt status bit"]
pub type TX_DATA_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `DETECT_START_INT_ST` reader - RTC_I2C_DETECT_START_INT interrupt status bit"]
pub type DETECT_START_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt status bit"]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt status bit"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt status bit"]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt status bit"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_I2C_TIME_OUT_INT interrupt status bit"]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_I2C_ACK_ERR_INT interrupt status bit"]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC_I2C_RX_DATA_INT interrupt status bit"]
    #[inline(always)]
    pub fn rx_data_int_st(&self) -> RX_DATA_INT_ST_R {
        RX_DATA_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC_I2C_TX_DATA_INT interrupt status bit"]
    #[inline(always)]
    pub fn tx_data_int_st(&self) -> TX_DATA_INT_ST_R {
        TX_DATA_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC_I2C_DETECT_START_INT interrupt status bit"]
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> DETECT_START_INT_ST_R {
        DETECT_START_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RTC I2C interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
