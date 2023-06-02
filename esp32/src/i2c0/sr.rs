#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACK_REC` reader - This register stores the value of ACK bit."]
pub type ACK_REC_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` reader - when in slave mode 1: master read slave 0: master write slave."]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - when I2C lost control of SDA line this register changes to high level."]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDRESSED` reader - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
pub type SLAVE_ADDRESSED_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS` reader - This register changes to high level when one byte is transferred."]
pub type BYTE_TRANS_R = crate::BitReader;
#[doc = "Field `RXFIFO_CNT` reader - This register represent the amount of data need to send."]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_CNT` reader - This register stores the amount of received data in ram."]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This register stores the value of ACK bit."]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when in slave mode 1: master read slave 0: master write slave."]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when I2C takes more than time_out_reg clocks to receive a data then this register changes to high level."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when I2C lost control of SDA line this register changes to high level."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:I2C bus is busy transferring data. 0:I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when configured as i2c slave and the address send by master is equal to slave's address then this bit will be high level."]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This register changes to high level when one byte is transferred."]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:13 - This register represent the amount of data need to send."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - This register stores the amount of received data in ram."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - This register stores the value of state machine for i2c module. 3'h0: SCL_MAIN_IDLE 3'h1: SCL_ADDRESS_SHIFT 3'h2: SCL_ACK_ADDRESS 3'h3: SCL_RX_DATA 3'h4 SCL_TX_DATA 3'h5:SCL_SEND_ACK 3'h6:SCL_WAIT_ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - This register stores the value of state machine to produce SCL. 3'h0: SCL_IDLE 3'h1:SCL_START 3'h2:SCL_LOW_EDGE 3'h3: SCL_LOW 3'h4:SCL_HIGH_EDGE 3'h5:SCL_HIGH 3'h6:SCL_STOP"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ack_rec", &format_args!("{}", self.ack_rec().bit()))
            .field("slave_rw", &format_args!("{}", self.slave_rw().bit()))
            .field("time_out", &format_args!("{}", self.time_out().bit()))
            .field("arb_lost", &format_args!("{}", self.arb_lost().bit()))
            .field("bus_busy", &format_args!("{}", self.bus_busy().bit()))
            .field(
                "slave_addressed",
                &format_args!("{}", self.slave_addressed().bit()),
            )
            .field("byte_trans", &format_args!("{}", self.byte_trans().bit()))
            .field("rxfifo_cnt", &format_args!("{}", self.rxfifo_cnt().bits()))
            .field("txfifo_cnt", &format_args!("{}", self.txfifo_cnt().bits()))
            .field(
                "scl_main_state_last",
                &format_args!("{}", self.scl_main_state_last().bits()),
            )
            .field(
                "scl_state_last",
                &format_args!("{}", self.scl_state_last().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
