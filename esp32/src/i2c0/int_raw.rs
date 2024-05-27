#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RXFIFO_FULL` reader - The raw interrupt status bit for rxfifo full when use apb fifo access."]
pub type RXFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY` reader - The raw interrupt status bit for txfifo empty when use apb fifo access."]
pub type TXFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `END_DETECT` reader - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `SLAVE_TRAN_COMP` reader - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `MASTER_TRAN_COMP` reader - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
pub type MASTER_TRAN_COMP_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TRANS_START` reader - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `ACK_ERR` reader - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
pub type ACK_ERR_R = crate::BitReader;
#[doc = "Field `RX_REC_FULL` reader - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
pub type RX_REC_FULL_R = crate::BitReader;
#[doc = "Field `TX_SEND_EMPTY` reader - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
pub type TX_SEND_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for rxfifo full when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_full(&self) -> RXFIFO_FULL_R {
        RXFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for txfifo empty when use apb fifo access."]
    #[inline(always)]
    pub fn txfifo_empty(&self) -> TXFIFO_EMPTY_R {
        TXFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for receiving data overflow when use apb fifo access."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for end_detect_int interrupt. when I2C deals with the END command it will produce end_detect_int interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for slave_tran_comp_int interrupt. when I2C Slave detectsthe STOP bit it will produce slave_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for arbitration_lost_int interrupt.when I2C lost the usage right of I2C BUS it will produce arbitration_lost_int interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for master_tra_comp_int interrupt. when I2C Master sends or receives a byte it will produce master_tran_comp_int interrupt."]
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for trans_complete_int interrupt. when I2C Master finished STOP command it will produce trans_complete_int interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for time_out_int interrupt. when I2C takes a lot of time to receive a data it will produce time_out_int interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for trans_start_int interrupt. when I2C sends the START bit it will produce trans_start_int interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for ack_err_int interrupt. when I2C receives a wrong ACK bit it will produce ack_err_int interrupt.."]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for rx_rec_full_int interrupt. when I2C receives more data than nonfifo_rx_thres it will produce rx_rec_full_int interrupt."]
    #[inline(always)]
    pub fn rx_rec_full(&self) -> RX_REC_FULL_R {
        RX_REC_FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for tx_send_empty_int interrupt.when I2C sends more data than nonfifo_tx_thres it will produce tx_send_empty_int interrupt.."]
    #[inline(always)]
    pub fn tx_send_empty(&self) -> TX_SEND_EMPTY_R {
        TX_SEND_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
