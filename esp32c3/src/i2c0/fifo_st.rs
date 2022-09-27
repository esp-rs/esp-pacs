#[doc = "Register `FIFO_ST` reader"]
pub struct R(crate::R<FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_RADDR` reader - reg_rxfifo_raddr"]
pub type RXFIFO_RADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO_WADDR` reader - reg_rxfifo_waddr"]
pub type RXFIFO_WADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_RADDR` reader - reg_txfifo_raddr"]
pub type TXFIFO_RADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_WADDR` reader - reg_txfifo_waddr"]
pub type TXFIFO_WADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_RW_POINT` reader - reg_slave_rw_point"]
pub type SLAVE_RW_POINT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - reg_rxfifo_raddr"]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RXFIFO_RADDR_R {
        RXFIFO_RADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - reg_rxfifo_waddr"]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RXFIFO_WADDR_R {
        RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - reg_txfifo_raddr"]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TXFIFO_RADDR_R {
        TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - reg_txfifo_waddr"]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TXFIFO_WADDR_R {
        TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 22:29 - reg_slave_rw_point"]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[doc = "I2C_FIFO_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_st](index.html) module"]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_st::R](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FIFO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
