#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RXFIFO_FULL` reader - The enable bit for rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_R = crate::BitReader;
#[doc = "Field `RXFIFO_FULL` writer - The enable bit for rxfifo_full_int interrupt."]
pub type RXFIFO_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY` reader - The enable bit for txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` writer - The enable bit for txfifo_empty_int interrupt."]
pub type TXFIFO_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` reader - The enable bit for rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` writer - The enable bit for rxfifo_ovf_int interrupt."]
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_DETECT` reader - The enable bit for end_detect_int interrupt."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `END_DETECT` writer - The enable bit for end_detect_int interrupt."]
pub type END_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_TRAN_COMP` reader - The enable bit for slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `SLAVE_TRAN_COMP` writer - The enable bit for slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` reader - The enable bit for arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` writer - The enable bit for arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_TRAN_COMP` reader - The enable bit for master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP` writer - The enable bit for master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - The enable bit for trans_complete_int interrupt."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - The enable bit for trans_complete_int interrupt."]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT` reader - The enable bit for time_out_int interrupt."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TIME_OUT` writer - The enable bit for time_out_int interrupt."]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` reader - The enable bit for trans_start_int interrupt."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - The enable bit for trans_start_int interrupt."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_ERR` reader - The enable bit for ack_err_int interrupt."]
pub type ACK_ERR_R = crate::BitReader;
#[doc = "Field `ACK_ERR` writer - The enable bit for ack_err_int interrupt."]
pub type ACK_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REC_FULL` reader - The enable bit for rx_rec_full_int interrupt."]
pub type RX_REC_FULL_R = crate::BitReader;
#[doc = "Field `RX_REC_FULL` writer - The enable bit for rx_rec_full_int interrupt."]
pub type RX_REC_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SEND_EMPTY` reader - The enable bit for tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_SEND_EMPTY` writer - The enable bit for tx_send_empty_int interrupt."]
pub type TX_SEND_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for rxfifo_full_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RXFIFO_FULL_R {
        RXFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for txfifo_empty_int interrupt."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for ack_err_int interrupt."]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full(&self) -> RX_REC_FULL_R {
        RX_REC_FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for tx_send_empty_int interrupt."]
    #[inline(always)]
    pub fn tx_send_empty(&self) -> TX_SEND_EMPTY_R {
        TX_SEND_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rxfifo_full", &self.rxfifo_full())
            .field("txfifo_empty", &self.txfifo_empty())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("end_detect", &self.end_detect())
            .field("slave_tran_comp", &self.slave_tran_comp())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("master_tran_comp", &self.master_tran_comp())
            .field("trans_complete", &self.trans_complete())
            .field("time_out", &self.time_out())
            .field("trans_start", &self.trans_start())
            .field("ack_err", &self.ack_err())
            .field("rx_rec_full", &self.rx_rec_full())
            .field("tx_send_empty", &self.tx_send_empty())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for rxfifo_full_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full(&mut self) -> RXFIFO_FULL_W<INT_ENA_SPEC> {
        RXFIFO_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for txfifo_empty_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty(&mut self) -> TXFIFO_EMPTY_W<INT_ENA_SPEC> {
        TXFIFO_EMPTY_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for rxfifo_ovf_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_ENA_SPEC> {
        RXFIFO_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for end_detect_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn end_detect(&mut self) -> END_DETECT_W<INT_ENA_SPEC> {
        END_DETECT_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for slave_tran_comp_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp(&mut self) -> SLAVE_TRAN_COMP_W<INT_ENA_SPEC> {
        SLAVE_TRAN_COMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - The enable bit for arbitration_lost_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_W::new(self, 5)
    }
    #[doc = "Bit 6 - The enable bit for master_tran_comp_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp(&mut self) -> MASTER_TRAN_COMP_W<INT_ENA_SPEC> {
        MASTER_TRAN_COMP_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for trans_complete_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - The enable bit for time_out_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_ENA_SPEC> {
        TIME_OUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - The enable bit for trans_start_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<INT_ENA_SPEC> {
        TRANS_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - The enable bit for ack_err_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ack_err(&mut self) -> ACK_ERR_W<INT_ENA_SPEC> {
        ACK_ERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - The enable bit for rx_rec_full_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rec_full(&mut self) -> RX_REC_FULL_W<INT_ENA_SPEC> {
        RX_REC_FULL_W::new(self, 11)
    }
    #[doc = "Bit 12 - The enable bit for tx_send_empty_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_send_empty(&mut self) -> TX_SEND_EMPTY_W<INT_ENA_SPEC> {
        TX_SEND_EMPTY_W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
