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
#[doc = "Field `RXFIFO_CNT` reader - Stores the byte number of valid data in Rx-FIFO."]
pub type RXFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSRN` reader - The register represent the level value of the internal uart dsr signal."]
pub type DSRN_R = crate::BitReader<bool>;
#[doc = "Field `CTSN` reader - This register represent the level value of the internal uart cts signal."]
pub type CTSN_R = crate::BitReader<bool>;
#[doc = "Field `RXD` reader - This register represent the level value of the internal uart rxd signal."]
pub type RXD_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_CNT` reader - Stores the byte number of data in Tx-FIFO."]
pub type TXFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTRN` reader - This bit represents the level of the internal uart dtr signal."]
pub type DTRN_R = crate::BitReader<bool>;
#[doc = "Field `RTSN` reader - This bit represents the level of the internal uart rts signal."]
pub type RTSN_R = crate::BitReader<bool>;
#[doc = "Field `TXD` reader - This bit represents the level of the internal uart txd signal."]
pub type TXD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 3:7 - Stores the byte number of valid data in Rx-FIFO."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This register represent the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Stores the byte number of data in Tx-FIFO."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - This bit represents the level of the internal uart dtr signal."]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit represents the level of the internal uart rts signal."]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit represents the level of the internal uart txd signal."]
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
#[doc = "`reset()` method sets STATUS to value 0xe000_c000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0xe000_c000;
}
