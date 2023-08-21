#[doc = "Register `I2C_DATA` reader"]
pub type R = crate::R<I2C_DATA_SPEC>;
#[doc = "Field `I2C_FIFO_RDATA` reader - The value of rx FIFO read data."]
pub type I2C_FIFO_RDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The value of rx FIFO read data."]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_DATA")
            .field(
                "i2c_fifo_rdata",
                &format_args!("{}", self.i2c_fifo_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Rx FIFO read data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_DATA_SPEC;
impl crate::RegisterSpec for I2C_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_data::R`](R) reader structure"]
impl crate::Readable for I2C_DATA_SPEC {}
#[doc = "`reset()` method sets I2C_DATA to value 0"]
impl crate::Resettable for I2C_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
