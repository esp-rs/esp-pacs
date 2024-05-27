///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `SLAVE_TRAN_COMP` reader - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit
pub type SLAVE_TRAN_COMP_R = crate::BitReader;
///Field `SLAVE_TRAN_COMP` writer - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit
pub type SLAVE_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBITRATION_LOST` reader - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit
pub type ARBITRATION_LOST_R = crate::BitReader;
///Field `ARBITRATION_LOST` writer - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASTER_TRAN_COMP` reader - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit
pub type MASTER_TRAN_COMP_R = crate::BitReader;
///Field `MASTER_TRAN_COMP` writer - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit
pub type MASTER_TRAN_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRANS_COMPLETE` reader - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit
pub type TRANS_COMPLETE_R = crate::BitReader;
///Field `TRANS_COMPLETE` writer - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIME_OUT` reader - RTC_I2C_TIME_OUT_INT interrupt enable bit
pub type TIME_OUT_R = crate::BitReader;
///Field `TIME_OUT` writer - RTC_I2C_TIME_OUT_INT interrupt enable bit
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK_ERR` reader - RTC_I2C_ACK_ERR_INT interrupt enable bit
pub type ACK_ERR_R = crate::BitReader;
///Field `ACK_ERR` writer - RTC_I2C_ACK_ERR_INT interrupt enable bit
pub type ACK_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_DATA` reader - RTC_I2C_RX_DATA_INT interrupt enable bit
pub type RX_DATA_R = crate::BitReader;
///Field `RX_DATA` writer - RTC_I2C_RX_DATA_INT interrupt enable bit
pub type RX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DATA` reader - RTC_I2C_TX_DATA_INT interrupt enable bit
pub type TX_DATA_R = crate::BitReader;
///Field `TX_DATA` writer - RTC_I2C_TX_DATA_INT interrupt enable bit
pub type TX_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DETECT_START` reader - RTC_I2C_DETECT_START_INT interrupt enable bit
pub type DETECT_START_R = crate::BitReader;
///Field `DETECT_START` writer - RTC_I2C_DETECT_START_INT interrupt enable bit
pub type DETECT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit
    #[inline(always)]
    pub fn slave_tran_comp(&self) -> SLAVE_TRAN_COMP_R {
        SLAVE_TRAN_COMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit
    #[inline(always)]
    pub fn master_tran_comp(&self) -> MASTER_TRAN_COMP_R {
        MASTER_TRAN_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit
    #[inline(always)]
    pub fn tx_data(&self) -> TX_DATA_R {
        TX_DATA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit
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
    ///Bit 0 - RTC_I2C_SLAVE_TRAN_COMP_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp(&mut self) -> SLAVE_TRAN_COMP_W<INT_ENA_SPEC> {
        SLAVE_TRAN_COMP_W::new(self, 0)
    }
    ///Bit 1 - RTC_I2C_ARBITRATION_LOST_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_W::new(self, 1)
    }
    ///Bit 2 - RTC_I2C_MASTER_TRAN_COMP_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp(&mut self) -> MASTER_TRAN_COMP_W<INT_ENA_SPEC> {
        MASTER_TRAN_COMP_W::new(self, 2)
    }
    ///Bit 3 - RTC_I2C_TRANS_COMPLETE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_W::new(self, 3)
    }
    ///Bit 4 - RTC_I2C_TIME_OUT_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_ENA_SPEC> {
        TIME_OUT_W::new(self, 4)
    }
    ///Bit 5 - RTC_I2C_ACK_ERR_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn ack_err(&mut self) -> ACK_ERR_W<INT_ENA_SPEC> {
        ACK_ERR_W::new(self, 5)
    }
    ///Bit 6 - RTC_I2C_RX_DATA_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn rx_data(&mut self) -> RX_DATA_W<INT_ENA_SPEC> {
        RX_DATA_W::new(self, 6)
    }
    ///Bit 7 - RTC_I2C_TX_DATA_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<INT_ENA_SPEC> {
        TX_DATA_W::new(self, 7)
    }
    ///Bit 8 - RTC_I2C_DETECT_START_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn detect_start(&mut self) -> DETECT_START_W<INT_ENA_SPEC> {
        DETECT_START_W::new(self, 8)
    }
}
/**Enable RTC I2C interrupt

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
