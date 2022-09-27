#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ENA` reader - The enable bit for rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_FULL_INT_ENA` writer - The enable bit for rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` reader - The enable bit for txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` writer - The enable bit for txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - The enable bit for rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - The enable bit for rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `END_DETECT_INT_ENA` reader - The enable bit for end_detect_int interrupt."]
pub type END_DETECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `END_DETECT_INT_ENA` writer - The enable bit for end_detect_int interrupt."]
pub type END_DETECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` reader - The enable bit for slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_TRAN_COMP_INT_ENA` writer - The enable bit for slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - The enable bit for arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - The enable bit for arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` reader - The enable bit for master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_TRAN_COMP_INT_ENA` writer - The enable bit for master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - The enable bit for trans_complete_int interrupt."]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - The enable bit for trans_complete_int interrupt."]
pub type TRANS_COMPLETE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - The enable bit for time_out_int interrupt."]
pub type TIME_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_ENA` writer - The enable bit for time_out_int interrupt."]
pub type TIME_OUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TRANS_START_INT_ENA` reader - The enable bit for trans_start_int interrupt."]
pub type TRANS_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_START_INT_ENA` writer - The enable bit for trans_start_int interrupt."]
pub type TRANS_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `ACK_ERR_INT_ENA` reader - The enable bit for ack_err_int interrupt."]
pub type ACK_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR_INT_ENA` writer - The enable bit for ack_err_int interrupt."]
pub type ACK_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `RX_REC_FULL_INT_ENA` reader - The enable bit for rx_rec_full_int interrupt."]
pub type RX_REC_FULL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_REC_FULL_INT_ENA` writer - The enable bit for rx_rec_full_int interrupt."]
pub type RX_REC_FULL_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TX_SEND_EMPTY_INT_ENA` reader - The enable bit for tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_SEND_EMPTY_INT_ENA` writer - The enable bit for tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The enable bit for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> RXFIFO_FULL_INT_ENA_R {
        RXFIFO_FULL_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> TXFIFO_EMPTY_INT_ENA_R {
        TXFIFO_EMPTY_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&self) -> END_DETECT_INT_ENA_R {
        END_DETECT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&self) -> SLAVE_TRAN_COMP_INT_ENA_R {
        SLAVE_TRAN_COMP_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&self) -> MASTER_TRAN_COMP_INT_ENA_R {
        MASTER_TRAN_COMP_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&self) -> TRANS_START_INT_ENA_R {
        TRANS_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_ena(&self) -> ACK_ERR_INT_ENA_R {
        ACK_ERR_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_ena(&self) -> RX_REC_FULL_INT_ENA_R {
        RX_REC_FULL_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_ena(&self) -> TX_SEND_EMPTY_INT_ENA_R {
        TX_SEND_EMPTY_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W<0> {
        RXFIFO_FULL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W<1> {
        TXFIFO_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W<2> {
        RXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W<3> {
        END_DETECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The enable bit for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp_int_ena(&mut self) -> SLAVE_TRAN_COMP_INT_ENA_W<4> {
        SLAVE_TRAN_COMP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The enable bit for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<5> {
        ARBITRATION_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The enable bit for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp_int_ena(&mut self) -> MASTER_TRAN_COMP_INT_ENA_W<6> {
        MASTER_TRAN_COMP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The enable bit for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W<7> {
        TRANS_COMPLETE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The enable bit for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W<8> {
        TIME_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The enable bit for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W<9> {
        TRANS_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The enable bit for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err_int_ena(&mut self) -> ACK_ERR_INT_ENA_W<10> {
        ACK_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full_int_ena(&mut self) -> RX_REC_FULL_INT_ENA_W<11> {
        RX_REC_FULL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The enable bit for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty_int_ena(&mut self) -> TX_SEND_EMPTY_INT_ENA_W<12> {
        TX_SEND_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
