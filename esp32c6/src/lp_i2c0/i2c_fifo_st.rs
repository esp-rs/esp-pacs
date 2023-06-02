#[doc = "Register `I2C_FIFO_ST` reader"]
pub struct R(crate::R<I2C_FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_RXFIFO_RADDR` reader - This is the offset address of the APB reading from rxfifo"]
pub type I2C_RXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `I2C_RXFIFO_WADDR` reader - This is the offset address of i2c module receiving data and writing to rxfifo."]
pub type I2C_RXFIFO_WADDR_R = crate::FieldReader;
#[doc = "Field `I2C_TXFIFO_RADDR` reader - This is the offset address of i2c module reading from txfifo."]
pub type I2C_TXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `I2C_TXFIFO_WADDR` reader - This is the offset address of APB bus writing to txfifo."]
pub type I2C_TXFIFO_WADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - This is the offset address of the APB reading from rxfifo"]
    #[inline(always)]
    pub fn i2c_rxfifo_raddr(&self) -> I2C_RXFIFO_RADDR_R {
        I2C_RXFIFO_RADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - This is the offset address of i2c module receiving data and writing to rxfifo."]
    #[inline(always)]
    pub fn i2c_rxfifo_waddr(&self) -> I2C_RXFIFO_WADDR_R {
        I2C_RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - This is the offset address of i2c module reading from txfifo."]
    #[inline(always)]
    pub fn i2c_txfifo_raddr(&self) -> I2C_TXFIFO_RADDR_R {
        I2C_TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - This is the offset address of APB bus writing to txfifo."]
    #[inline(always)]
    pub fn i2c_txfifo_waddr(&self) -> I2C_TXFIFO_WADDR_R {
        I2C_TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_FIFO_ST")
            .field(
                "i2c_rxfifo_raddr",
                &format_args!("{}", self.i2c_rxfifo_raddr().bits()),
            )
            .field(
                "i2c_rxfifo_waddr",
                &format_args!("{}", self.i2c_rxfifo_waddr().bits()),
            )
            .field(
                "i2c_txfifo_raddr",
                &format_args!("{}", self.i2c_txfifo_raddr().bits()),
            )
            .field(
                "i2c_txfifo_waddr",
                &format_args!("{}", self.i2c_txfifo_waddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_FIFO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_st](index.html) module"]
pub struct I2C_FIFO_ST_SPEC;
impl crate::RegisterSpec for I2C_FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_fifo_st::R](R) reader structure"]
impl crate::Readable for I2C_FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_FIFO_ST to value 0"]
impl crate::Resettable for I2C_FIFO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
