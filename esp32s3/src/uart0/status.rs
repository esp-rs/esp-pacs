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
pub type RXFIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSRN` reader - The register represent the level value of the internal uart dsr signal."]
pub type DSRN_R = crate::BitReader;
#[doc = "Field `CTSN` reader - This register represent the level value of the internal uart cts signal."]
pub type CTSN_R = crate::BitReader;
#[doc = "Field `RXD` reader - This register represent the level value of the internal uart rxd signal."]
pub type RXD_R = crate::BitReader;
#[doc = "Field `TXFIFO_CNT` reader - Stores the byte number of data in Tx-FIFO."]
pub type TXFIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTRN` reader - This bit represents the level of the internal uart dtr signal."]
pub type DTRN_R = crate::BitReader;
#[doc = "Field `RTSN` reader - This bit represents the level of the internal uart rts signal."]
pub type RTSN_R = crate::BitReader;
#[doc = "Field `TXD` reader - This bit represents the level of the internal uart txd signal."]
pub type TXD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Stores the byte number of valid data in Rx-FIFO."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
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
    #[doc = "Bits 16:25 - Stores the byte number of data in Tx-FIFO."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rxfifo_cnt", &format_args!("{}", self.rxfifo_cnt().bits()))
            .field("dsrn", &format_args!("{}", self.dsrn().bit()))
            .field("ctsn", &format_args!("{}", self.ctsn().bit()))
            .field("rxd", &format_args!("{}", self.rxd().bit()))
            .field("txfifo_cnt", &format_args!("{}", self.txfifo_cnt().bits()))
            .field("dtrn", &format_args!("{}", self.dtrn().bit()))
            .field("rtsn", &format_args!("{}", self.rtsn().bit()))
            .field("txd", &format_args!("{}", self.txd().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
