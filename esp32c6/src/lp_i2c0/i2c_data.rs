#[doc = "Register `I2C_DATA` reader"]
pub struct R(crate::R<I2C_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_FIFO_RDATA` reader - The value of rx FIFO read data."]
pub type I2C_FIFO_RDATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value of rx FIFO read data."]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx FIFO read data.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_data](index.html) module"]
pub struct I2C_DATA_SPEC;
impl crate::RegisterSpec for I2C_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_data::R](R) reader structure"]
impl crate::Readable for I2C_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_DATA to value 0"]
impl crate::Resettable for I2C_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
