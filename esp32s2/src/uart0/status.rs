#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_CNT` reader - Stores the number of valid data bytes in RX FIFO."]
pub type RXFIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSRN` reader - This register represents the level of the internal UART DSR signal."]
pub type DSRN_R = crate::BitReader<bool>;
#[doc = "Field `CTSN` reader - This register represents the level of the internal UART CTS signal."]
pub type CTSN_R = crate::BitReader<bool>;
#[doc = "Field `RXD` reader - This register represents the level of the internal UART RXD signal."]
pub type RXD_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_CNT` reader - Stores the number of data bytes in TX FIFO."]
pub type TXFIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTRN` reader - This bit represents the level of the internal UART DTR signal."]
pub type DTRN_R = crate::BitReader<bool>;
#[doc = "Field `RTSN` reader - This bit represents the level of the internal UART RTS signal."]
pub type RTSN_R = crate::BitReader<bool>;
#[doc = "Field `TXD` reader - This bit represents the level of the internal UART TXD signal."]
pub type TXD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - Stores the number of valid data bytes in RX FIFO."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 13 - This register represents the level of the internal UART DSR signal."]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This register represents the level of the internal UART CTS signal."]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This register represents the level of the internal UART RXD signal."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Stores the number of data bytes in TX FIFO."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - This bit represents the level of the internal UART DTR signal."]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit represents the level of the internal UART RTS signal."]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit represents the level of the internal UART TXD signal."]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UART status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
