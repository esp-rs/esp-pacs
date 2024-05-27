///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `SLAVE_TRAN_COMP` reader - slave transit complete interrupt state
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
///Field `ARBITRATION_LOST` reader - arbitration lost interrupt state
pub type ARBITRATION_LOST_R = crate::BitReader;
///Field `MASTER_TRAN_COMP` reader - master transit complete interrupt state
pub type MASTER_TRAN_COMP_R = crate::BitReader;
///Field `TRANS_COMPLETE` reader - transit complete interrupt state
pub type TRANS_COMPLETE_R = crate::BitReader;
///Field `TIME_OUT` reader - time out interrupt state
pub type TIME_OUT_R = crate::BitReader;
///Field `ACK_ERR` reader - ack error interrupt state
pub type ACK_ERR_R = crate::BitReader;
///Field `RX_DATA` reader - receive data interrupt state
pub type RX_DATA_R = crate::BitReader;
///Field `TX_DATA` reader - transit data interrupt state
pub type TX_DATA_R = crate::BitReader;
///Field `DETECT_START` reader - detect start interrupt state
pub type DETECT_START_R = crate::BitReader;
impl R {
    ///Bit 0 - slave transit complete interrupt state
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - arbitration lost interrupt state
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - master transit complete interrupt state
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - transit complete interrupt state
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - time out interrupt state
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ack error interrupt state
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - receive data interrupt state
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - transit data interrupt state
    #[inline(always)]
    pub fn tx_data(&self) -> TX_DATA_R {
        TX_DATA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - detect start interrupt state
    #[inline(always)]
    pub fn detect_start(&self) -> DETECT_START_R {
        DETECT_START_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
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
/**interrupt state register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
