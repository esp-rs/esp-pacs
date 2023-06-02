#[doc = "Register `I2C_SR` reader"]
pub struct R(crate::R<I2C_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_RESP_REC` reader - The received ACK value in master mode or slave mode. 0: ACK, 1: NACK."]
pub type I2C_RESP_REC_R = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` reader - When the I2C controller loses control of SCL line, this register changes to 1."]
pub type I2C_ARB_LOST_R = crate::BitReader;
#[doc = "Field `I2C_BUS_BUSY` reader - 1: the I2C bus is busy transferring data, 0: the I2C bus is in idle state."]
pub type I2C_BUS_BUSY_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_CNT` reader - This field represents the amount of data needed to be sent."]
pub type I2C_RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `I2C_TXFIFO_CNT` reader - This field stores the amount of received data in RAM."]
pub type I2C_TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_MAIN_STATE_LAST` reader - This field indicates the states of the I2C module state machine. 0: Idle, 1: Address shift, 2: ACK address, 3: Rx data, 4: Tx data, 5: Send ACK, 6: Wait ACK"]
pub type I2C_SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_STATE_LAST` reader - This field indicates the states of the state machine used to produce SCL. 0: Idle, 1: Start, 2: Negative edge, 3: Low, 4: Positive edge, 5: High, 6: Stop"]
pub type I2C_SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The received ACK value in master mode or slave mode. 0: ACK, 1: NACK."]
    #[inline(always)]
    pub fn i2c_resp_rec(&self) -> I2C_RESP_REC_R {
        I2C_RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - When the I2C controller loses control of SCL line, this register changes to 1."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the I2C bus is busy transferring data, 0: the I2C bus is in idle state."]
    #[inline(always)]
    pub fn i2c_bus_busy(&self) -> I2C_BUS_BUSY_R {
        I2C_BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - This field represents the amount of data needed to be sent."]
    #[inline(always)]
    pub fn i2c_rxfifo_cnt(&self) -> I2C_RXFIFO_CNT_R {
        I2C_RXFIFO_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - This field stores the amount of received data in RAM."]
    #[inline(always)]
    pub fn i2c_txfifo_cnt(&self) -> I2C_TXFIFO_CNT_R {
        I2C_TXFIFO_CNT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - This field indicates the states of the I2C module state machine. 0: Idle, 1: Address shift, 2: ACK address, 3: Rx data, 4: Tx data, 5: Send ACK, 6: Wait ACK"]
    #[inline(always)]
    pub fn i2c_scl_main_state_last(&self) -> I2C_SCL_MAIN_STATE_LAST_R {
        I2C_SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - This field indicates the states of the state machine used to produce SCL. 0: Idle, 1: Start, 2: Negative edge, 3: Low, 4: Positive edge, 5: High, 6: Stop"]
    #[inline(always)]
    pub fn i2c_scl_state_last(&self) -> I2C_SCL_STATE_LAST_R {
        I2C_SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SR")
            .field(
                "i2c_resp_rec",
                &format_args!("{}", self.i2c_resp_rec().bit()),
            )
            .field(
                "i2c_arb_lost",
                &format_args!("{}", self.i2c_arb_lost().bit()),
            )
            .field(
                "i2c_bus_busy",
                &format_args!("{}", self.i2c_bus_busy().bit()),
            )
            .field(
                "i2c_rxfifo_cnt",
                &format_args!("{}", self.i2c_rxfifo_cnt().bits()),
            )
            .field(
                "i2c_txfifo_cnt",
                &format_args!("{}", self.i2c_txfifo_cnt().bits()),
            )
            .field(
                "i2c_scl_main_state_last",
                &format_args!("{}", self.i2c_scl_main_state_last().bits()),
            )
            .field(
                "i2c_scl_state_last",
                &format_args!("{}", self.i2c_scl_state_last().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Describe I2C work status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sr](index.html) module"]
pub struct I2C_SR_SPEC;
impl crate::RegisterSpec for I2C_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_sr::R](R) reader structure"]
impl crate::Readable for I2C_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_SR to value 0"]
impl crate::Resettable for I2C_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
