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
#[doc = "Field `RXFIFO_CNT` reader - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `ST_URX_OUT` reader - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
pub type ST_URX_OUT_R = crate::FieldReader;
#[doc = "Field `DSRN` reader - This register stores the level value of the internal uart dsr signal."]
pub type DSRN_R = crate::BitReader;
#[doc = "Field `CTSN` reader - This register stores the level value of the internal uart cts signal."]
pub type CTSN_R = crate::BitReader;
#[doc = "Field `RXD` reader - This register stores the level value of the internal uart rxd signal."]
pub type RXD_R = crate::BitReader;
#[doc = "Field `TXFIFO_CNT` reader - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `ST_UTX_OUT` reader - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
pub type ST_UTX_OUT_R = crate::FieldReader;
#[doc = "Field `DTRN` reader - The register represent the level value of the internal uart dsr signal."]
pub type DTRN_R = crate::BitReader;
#[doc = "Field `RTSN` reader - This register represent the level value of the internal uart cts signal."]
pub type RTSN_R = crate::BitReader;
#[doc = "Field `TXD` reader - This register represent the level value of the internal uart rxd signal."]
pub type TXD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - (rx_mem_cnt rxfifo_cnt) stores the byte num of valid datas in receiver's fifo. rx_mem_cnt register stores the 3 most significant bits rxfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - This register stores the value of receiver's finite state machine. 0:RX_IDLE 1:RX_STRT 2:RX_DAT0 3:RX_DAT1 4:RX_DAT2 5:RX_DAT3 6:RX_DAT4 7:RX_DAT5 8:RX_DAT6 9:RX_DAT7 10:RX_PRTY 11:RX_STP1 12:RX_STP2 13:RX_DL1"]
    #[inline(always)]
    pub fn st_urx_out(&self) -> ST_URX_OUT_R {
        ST_URX_OUT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - This register stores the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This register stores the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This register stores the level value of the internal uart rxd signal."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - (tx_mem_cnt txfifo_cnt) stores the byte num of valid datas in transmitter's fifo.tx_mem_cnt stores the 3 most significant bits txfifo_cnt stores the 8 least significant bits."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - This register stores the value of transmitter's finite state machine. 0:TX_IDLE 1:TX_STRT 2:TX_DAT0 3:TX_DAT1 4:TX_DAT2 5:TX_DAT3 6:TX_DAT4 7:TX_DAT5 8:TX_DAT6 9:TX_DAT7 10:TX_PRTY 11:TX_STP1 12:TX_STP2 13:TX_DL0 14:TX_DL1"]
    #[inline(always)]
    pub fn st_utx_out(&self) -> ST_UTX_OUT_R {
        ST_UTX_OUT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - The register represent the level value of the internal uart dsr signal."]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This register represent the level value of the internal uart cts signal."]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register represent the level value of the internal uart rxd signal."]
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
            .field("st_urx_out", &format_args!("{}", self.st_urx_out().bits()))
            .field("dsrn", &format_args!("{}", self.dsrn().bit()))
            .field("ctsn", &format_args!("{}", self.ctsn().bit()))
            .field("rxd", &format_args!("{}", self.rxd().bit()))
            .field("txfifo_cnt", &format_args!("{}", self.txfifo_cnt().bits()))
            .field("st_utx_out", &format_args!("{}", self.st_utx_out().bits()))
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
    const RESET_VALUE: Self::Ux = 0;
}
