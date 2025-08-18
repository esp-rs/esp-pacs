#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLAVE_TRAN_COMP` reader - enable slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `SLAVE_TRAN_COMP` writer - enable slave transit complete interrupt"]
pub type SLAVE_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` reader - enable arbitration lost interrupt"]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` writer - enable arbitration lost interrupt"]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_TRAN_COMP` reader - enable master transit complete interrupt"]
pub type MASTER_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP` writer - enable master transit complete interrupt"]
pub type MASTER_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - enable transit complete interrupt"]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - enable transit complete interrupt"]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT` reader - enable time out interrupt"]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TIME_OUT` writer - enable time out interrupt"]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_ERR` reader - enable eack error interrupt"]
pub type ACK_ERR_R = crate::BitReader;
#[doc = "Field `ACK_ERR` writer - enable eack error interrupt"]
pub type ACK_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA` reader - enable receive data interrupt"]
pub type RX_DATA_R = crate::BitReader;
#[doc = "Field `RX_DATA` writer - enable receive data interrupt"]
pub type RX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA` reader - enable transit data interrupt"]
pub type TX_DATA_R = crate::BitReader;
#[doc = "Field `TX_DATA` writer - enable transit data interrupt"]
pub type TX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START` reader - enable detect start interrupt"]
pub type DETECT_START_R = crate::BitReader;
#[doc = "Field `DETECT_START` writer - enable detect start interrupt"]
pub type DETECT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable slave transit complete interrupt"]
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable arbitration lost interrupt"]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable master transit complete interrupt"]
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable transit complete interrupt"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable time out interrupt"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable eack error interrupt"]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable receive data interrupt"]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable transit data interrupt"]
    #[inline(always)]
    pub fn tx_data(&self) -> TX_DATA_R {
        TX_DATA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable detect start interrupt"]
    #[inline(always)]
    pub fn detect_start(&self) -> DETECT_START_R {
        DETECT_START_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("slave_tran_comp", &self.slave_tran_comp())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("master_tran_comp", &self.master_tran_comp())
            .field("trans_complete", &self.trans_complete())
            .field("time_out", &self.time_out())
            .field("ack_err", &self.ack_err())
            .field("rx_data", &self.rx_data())
            .field("tx_data", &self.tx_data())
            .field("detect_start", &self.detect_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable slave transit complete interrupt"]
    #[inline(always)]
    pub fn slave_tran_comp(&mut self) -> SLAVE_TRAN_COMP_W<'_, INT_ENA_SPEC> {
        SLAVE_TRAN_COMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable arbitration lost interrupt"]
    #[inline(always)]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<'_, INT_ENA_SPEC> {
        ARBITRATION_LOST_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable master transit complete interrupt"]
    #[inline(always)]
    pub fn master_tran_comp(&mut self) -> MASTER_TRAN_COMP_W<'_, INT_ENA_SPEC> {
        MASTER_TRAN_COMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable transit complete interrupt"]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<'_, INT_ENA_SPEC> {
        TRANS_COMPLETE_W::new(self, 3)
    }
    #[doc = "Bit 4 - enable time out interrupt"]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<'_, INT_ENA_SPEC> {
        TIME_OUT_W::new(self, 4)
    }
    #[doc = "Bit 5 - enable eack error interrupt"]
    #[inline(always)]
    pub fn ack_err(&mut self) -> ACK_ERR_W<'_, INT_ENA_SPEC> {
        ACK_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - enable receive data interrupt"]
    #[inline(always)]
    pub fn rx_data(&mut self) -> RX_DATA_W<'_, INT_ENA_SPEC> {
        RX_DATA_W::new(self, 6)
    }
    #[doc = "Bit 7 - enable transit data interrupt"]
    #[inline(always)]
    pub fn tx_data(&mut self) -> TX_DATA_W<'_, INT_ENA_SPEC> {
        TX_DATA_W::new(self, 7)
    }
    #[doc = "Bit 8 - enable detect start interrupt"]
    #[inline(always)]
    pub fn detect_start(&mut self) -> DETECT_START_W<'_, INT_ENA_SPEC> {
        DETECT_START_W::new(self, 8)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
