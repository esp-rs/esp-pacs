#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `RESP_REC` reader - The received ACK value in master mode or slave mode. 0: ACK. 1: NACK."]
pub type RESP_REC_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` reader - When in slave mode, 1: master reads from slave. 0: master writes to slave."]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - When the I2C controller takes more than I2C_TIME_OUT clocks to receive a data bit, this field changes to 1."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - When the I2C controller loses control of SCL line, this register changes to 1."]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - 1: the I2C bus is busy transferring data. 0: the I2C bus is in idle state."]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDRESSED` reader - When configured as an I2C Slave, and the address sent by the master is equal to the address of the slave, then this bit will be of high level."]
pub type SLAVE_ADDRESSED_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS` reader - This field changes to 1 when one byte is transferred."]
pub type BYTE_TRANS_R = crate::BitReader;
#[doc = "Field `RXFIFO_CNT` reader - This field represents the amount of data needed to be sent."]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `STRETCH_CAUSE` reader - The cause of stretching SCL low in slave mode. 0: stretching SCL low at the beginning of I2C read data state. 1: stretching SCL low when I2C TX FIFO is empty in slave mode. 2: stretching SCL low when I2C RX FIFO is full in slave mode."]
pub type STRETCH_CAUSE_R = crate::FieldReader;
#[doc = "Field `TXFIFO_CNT` reader - This field stores the amount of received data in RAM."]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - This field indicates the states of the I2C module state machine. 0: Idle. 1: Address shift. 2: ACK address. 3: RX data. 4: TX data. 5: Send ACK. 6: Wait ACK"]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - This field indicates the states of the state machine used to produce SCL. 0: Idle. 1: Start. 2: Negative edge. 3: Low. 4: Positive edge. 5: High. 6: Stop"]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The received ACK value in master mode or slave mode. 0: ACK. 1: NACK."]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When in slave mode, 1: master reads from slave. 0: master writes to slave."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When the I2C controller takes more than I2C_TIME_OUT clocks to receive a data bit, this field changes to 1."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When the I2C controller loses control of SCL line, this register changes to 1."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the I2C bus is busy transferring data. 0: the I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When configured as an I2C Slave, and the address sent by the master is equal to the address of the slave, then this bit will be of high level."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This field changes to 1 when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:13 - This field represents the amount of data needed to be sent."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - The cause of stretching SCL low in slave mode. 0: stretching SCL low at the beginning of I2C read data state. 1: stretching SCL low when I2C TX FIFO is empty in slave mode. 2: stretching SCL low when I2C RX FIFO is full in slave mode."]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:23 - This field stores the amount of received data in RAM."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - This field indicates the states of the I2C module state machine. 0: Idle. 1: Address shift. 2: ACK address. 3: RX data. 4: TX data. 5: Send ACK. 6: Wait ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - This field indicates the states of the state machine used to produce SCL. 0: Idle. 1: Start. 2: Negative edge. 3: Low. 4: Positive edge. 5: High. 6: Stop"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("resp_rec", &self.resp_rec())
            .field("slave_rw", &self.slave_rw())
            .field("time_out", &self.time_out())
            .field("arb_lost", &self.arb_lost())
            .field("bus_busy", &self.bus_busy())
            .field("slave_addressed", &self.slave_addressed())
            .field("byte_trans", &self.byte_trans())
            .field("rxfifo_cnt", &self.rxfifo_cnt())
            .field("stretch_cause", &self.stretch_cause())
            .field("txfifo_cnt", &self.txfifo_cnt())
            .field("scl_main_state_last", &self.scl_main_state_last())
            .field("scl_state_last", &self.scl_state_last())
            .finish()
    }
}
#[doc = "Describe I2C work status\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
