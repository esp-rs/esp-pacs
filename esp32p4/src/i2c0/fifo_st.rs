#[doc = "Register `FIFO_ST` reader"]
pub type R = crate::R<FIFO_ST_SPEC>;
#[doc = "Field `RXFIFO_RADDR` reader - Represents the offset address of the APB reading from RXFIFO."]
pub type RXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WADDR` reader - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
pub type RXFIFO_WADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_RADDR` reader - Represents the offset address of i2c module reading from TXFIFO."]
pub type TXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WADDR` reader - Represents the offset address of APB bus writing to TXFIFO."]
pub type TXFIFO_WADDR_R = crate::FieldReader;
#[doc = "Field `SLAVE_RW_POINT` reader - Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode."]
pub type SLAVE_RW_POINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Represents the offset address of the APB reading from RXFIFO."]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RXFIFO_RADDR_R {
        RXFIFO_RADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RXFIFO_WADDR_R {
        RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Represents the offset address of i2c module reading from TXFIFO."]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TXFIFO_RADDR_R {
        TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Represents the offset address of APB bus writing to TXFIFO."]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TXFIFO_WADDR_R {
        TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 22:29 - Represents the offset address in the I2C Slave RAM addressed by I2C Master when in I2C slave mode."]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_ST")
            .field("rxfifo_raddr", &self.rxfifo_raddr())
            .field("rxfifo_waddr", &self.rxfifo_waddr())
            .field("txfifo_raddr", &self.txfifo_raddr())
            .field("txfifo_waddr", &self.txfifo_waddr())
            .field("slave_rw_point", &self.slave_rw_point())
            .finish()
    }
}
#[doc = "FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_st::R`](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FIFO_ST_SPEC {}
