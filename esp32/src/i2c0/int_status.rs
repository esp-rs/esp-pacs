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
#[doc = "Field `RXFIFO_FULL_INT_ST` reader - The masked interrupt status for rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_INT_ST_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY_INT_ST` reader - The masked interrupt status for txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_ST` reader - The masked interrupt status for rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `END_DETECT_INT_ST` reader - The masked interrupt status for end_detect_int interrupt."]
pub type END_DETECT_INT_ST_R = crate::BitReader;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ST` reader - The masked interrupt status for slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_INT_ST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - The masked interrupt status for arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_INT_ST_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - The masked interrupt status for master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - The masked interrupt status for trans_complete_int interrupt."]
pub type TRANS_COMPLETE_INT_ST_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ST` reader - The masked interrupt status for time_out_int interrupt."]
pub type TIME_OUT_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_START_INT_ST` reader - The masked interrupt status for trans_start_int interrupt."]
pub type TRANS_START_INT_ST_R = crate::BitReader;
#[doc = "Field `ACK_ERR_INT_ST` reader - The masked interrupt status for ack_err_int interrupt."]
pub type ACK_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_REC_FULL_INT_ST` reader - The masked interrupt status for rx_rec_full_int interrupt."]
pub type RX_REC_FULL_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_SEND_EMPTY_INT_ST` reader - The masked interrupt status for tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_st(&self) -> RX_REC_FULL_INT_ST_R {
        RX_REC_FULL_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_st(&self) -> TX_SEND_EMPTY_INT_ST_R {
        TX_SEND_EMPTY_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS")
            .field(
                "rxfifo_full_int_st",
                &format_args!("{}", self.rxfifo_full_int_st().bit()),
            )
            .field(
                "txfifo_empty_int_st",
                &format_args!("{}", self.txfifo_empty_int_st().bit()),
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
                "trans_start_int_st",
                &format_args!("{}", self.trans_start_int_st().bit()),
            )
            .field(
                "ack_err_int_st",
                &format_args!("{}", self.ack_err_int_st().bit()),
            )
            .field(
                "rx_rec_full_int_st",
                &format_args!("{}", self.rx_rec_full_int_st().bit()),
            )
            .field(
                "tx_send_empty_int_st",
                &format_args!("{}", self.tx_send_empty_int_st().bit()),
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
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
