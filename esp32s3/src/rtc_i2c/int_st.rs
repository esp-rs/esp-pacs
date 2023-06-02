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
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - arbitration lost interrupt state"]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - master transit complete interrupt state"]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - transit complete interrupt state"]
pub type TRANS_COMPLETE_INT_ST_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ST` reader - time out interrupt state"]
pub type TIME_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `ACK_ERR_INT_ST` reader - ack error interrupt state"]
pub type ACK_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_DATA_INT_ST` reader - receive data interrupt state"]
pub type RX_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_DATA_INT_ST` reader - transit data interrupt state"]
pub type TX_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `DETECT_START_INT_ST` reader - detect start interrupt state"]
pub type DETECT_START_INT_ST_R = crate::BitReader;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "slave_tran_comp_int_st",
                &format_args!("{}", self.slave_tran_comp_int_st().bit()),
            )
            .field(
                "arbitration_lost_int_st",
                &format_args!("{}", self.arbitration_lost_int_st().bit()),
            )
            .field(
                "master_tran_comp_int_st",
                &format_args!("{}", self.master_tran_comp_int_st().bit()),
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
                "ack_err_int_st",
                &format_args!("{}", self.ack_err_int_st().bit()),
            )
            .field(
                "rx_data_int_st",
                &format_args!("{}", self.rx_data_int_st().bit()),
            )
            .field(
                "tx_data_int_st",
                &format_args!("{}", self.tx_data_int_st().bit()),
            )
            .field(
                "detect_start_int_st",
                &format_args!("{}", self.detect_start_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
    const RESET_VALUE: Self::Ux = 0;
}
