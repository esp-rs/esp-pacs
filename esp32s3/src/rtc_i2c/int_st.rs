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
#[doc = "Field `SLAVE_TRAN_COMP_INT_ST` reader - slave transit complete interrupt state"]
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - arbitration lost interrupt state"]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - master transit complete interrupt state"]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - transit complete interrupt state"]
pub type TRANS_COMPLETE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_ST` reader - time out interrupt state"]
pub type TIME_OUT_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR_INT_ST` reader - ack error interrupt state"]
pub type ACK_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_INT_ST` reader - receive data interrupt state"]
pub type RX_DATA_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_INT_ST` reader - transit data interrupt state"]
pub type TX_DATA_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `DETECT_START_INT_ST` reader - detect start interrupt state"]
pub type DETECT_START_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - slave transit complete interrupt state"]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - arbitration lost interrupt state"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - master transit complete interrupt state"]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transit complete interrupt state"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - time out interrupt state"]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ack error interrupt state"]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive data interrupt state"]
    #[inline(always)]
    pub fn rx_data_int_st(&self) -> RX_DATA_INT_ST_R {
        RX_DATA_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transit data interrupt state"]
    #[inline(always)]
    pub fn tx_data_int_st(&self) -> TX_DATA_INT_ST_R {
        TX_DATA_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - detect start interrupt state"]
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> DETECT_START_INT_ST_R {
        DETECT_START_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "interrupt state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
