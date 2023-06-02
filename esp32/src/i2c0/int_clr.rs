#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_CLR` writer - Set this bit to clear the rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TXFIFO_EMPTY_INT_CLR` writer - Set this bit to clear the txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - Set this bit to clear the rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `END_DETECT_INT_CLR` writer - Set this bit to clear the end_detect_int interrupt."]
pub type END_DETECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_CLR` writer - Set this bit to clear the slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - Set this bit to clear the arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `MASTER_TRAN_COMP_INT_CLR` writer - Set this bit to clear the master_tran_comp interrupt."]
pub type MASTER_TRAN_COMP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - Set this bit to clear the trans_complete_int interrupt."]
pub type TRANS_COMPLETE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - Set this bit to clear the time_out_int interrupt."]
pub type TIME_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TRANS_START_INT_CLR` writer - Set this bit to clear the trans_start_int interrupt."]
pub type TRANS_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `ACK_ERR_INT_CLR` writer - Set this bit to clear the ack_err_int interrupt."]
pub type ACK_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RX_REC_FULL_INT_CLR` writer - Set this bit to clear the rx_rec_full_int interrupt."]
pub type RX_REC_FULL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TX_SEND_EMPTY_INT_CLR` writer - Set this bit to clear the tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W<0> {
        RXFIFO_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the txfifo_empty_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W<1> {
        TXFIFO_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the rxfifo_ovf_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<2> {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the end_detect_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn end_detect_int_clr(&mut self) -> END_DETECT_INT_CLR_W<3> {
        END_DETECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the slave_tran_comp_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp_int_clr(&mut self) -> SLAVE_TRAN_COMP_INT_CLR_W<4> {
        SLAVE_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the arbitration_lost_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W<5> {
        ARBITRATION_LOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the master_tran_comp interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp_int_clr(&mut self) -> MASTER_TRAN_COMP_INT_CLR_W<6> {
        MASTER_TRAN_COMP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the trans_complete_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W<7> {
        TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the time_out_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W<8> {
        TIME_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the trans_start_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_int_clr(&mut self) -> TRANS_START_INT_CLR_W<9> {
        TRANS_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the ack_err_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ack_err_int_clr(&mut self) -> ACK_ERR_INT_CLR_W<10> {
        ACK_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the rx_rec_full_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rec_full_int_clr(&mut self) -> RX_REC_FULL_INT_CLR_W<11> {
        RX_REC_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the tx_send_empty_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_send_empty_int_clr(&mut self) -> TX_SEND_EMPTY_INT_CLR_W<12> {
        TX_SEND_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
