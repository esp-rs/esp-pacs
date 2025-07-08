#[doc = "Register `I2C_TXFIFO_START_ADDR` reader"]
pub type R = crate::R<I2C_TXFIFO_START_ADDR_SPEC>;
#[doc = "Field `I2C_TXFIFO_START_ADDR` reader - This is the I2C txfifo first address."]
pub type I2C_TXFIFO_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This is the I2C txfifo first address."]
    #[inline(always)]
    pub fn i2c_txfifo_start_addr(&self) -> I2C_TXFIFO_START_ADDR_R {
        I2C_TXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TXFIFO_START_ADDR")
            .field("i2c_txfifo_start_addr", &self.i2c_txfifo_start_addr())
            .finish()
    }
}
#[doc = "I2C TXFIFO base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_txfifo_start_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_TXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for I2C_TXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_txfifo_start_addr::R`](R) reader structure"]
impl crate::Readable for I2C_TXFIFO_START_ADDR_SPEC {}
#[doc = "`reset()` method sets I2C_TXFIFO_START_ADDR to value 0"]
impl crate::Resettable for I2C_TXFIFO_START_ADDR_SPEC {}
