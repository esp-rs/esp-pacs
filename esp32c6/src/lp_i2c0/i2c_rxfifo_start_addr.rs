#[doc = "Register `I2C_RXFIFO_START_ADDR` reader"]
pub struct R(crate::R<I2C_RXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_RXFIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_RXFIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_RXFIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_RXFIFO_START_ADDR` reader - This is the I2C rxfifo first address."]
pub type I2C_RXFIFO_START_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is the I2C rxfifo first address."]
    #[inline(always)]
    pub fn i2c_rxfifo_start_addr(&self) -> I2C_RXFIFO_START_ADDR_R {
        I2C_RXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[doc = "I2C RXFIFO base address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_rxfifo_start_addr](index.html) module"]
pub struct I2C_RXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for I2C_RXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_rxfifo_start_addr::R](R) reader structure"]
impl crate::Readable for I2C_RXFIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_RXFIFO_START_ADDR to value 0"]
impl crate::Resettable for I2C_RXFIFO_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
