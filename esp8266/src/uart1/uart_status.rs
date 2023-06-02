#[doc = "Register `UART_STATUS` reader"]
pub struct R(crate::R<UART_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rxfifo_cnt` reader - Number of data in uart rx fifo"]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `dsrn` reader - The level of uart dsr pin"]
pub type DSRN_R = crate::BitReader;
#[doc = "Field `ctsn` reader - The level of uart cts pin"]
pub type CTSN_R = crate::BitReader;
#[doc = "Field `rxd` reader - The level of uart rxd pin"]
pub type RXD_R = crate::BitReader;
#[doc = "Field `txfifo_cnt` reader - Number of data in UART TX fifo"]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `dtrn` reader - The level of uart dtr pin"]
pub type DTRN_R = crate::BitReader;
#[doc = "Field `rtsn` reader - The level of uart rts pin"]
pub type RTSN_R = crate::BitReader;
#[doc = "Field `txd` reader - The level of the uart txd pin"]
pub type TXD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Number of data in uart rx fifo"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - The level of uart dsr pin"]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The level of uart cts pin"]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The level of uart rxd pin"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of data in UART TX fifo"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 29 - The level of uart dtr pin"]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The level of uart rts pin"]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The level of the uart txd pin"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_STATUS")
            .field("txd", &format_args!("{}", self.txd().bit()))
            .field("rtsn", &format_args!("{}", self.rtsn().bit()))
            .field("dtrn", &format_args!("{}", self.dtrn().bit()))
            .field("txfifo_cnt", &format_args!("{}", self.txfifo_cnt().bits()))
            .field("rxd", &format_args!("{}", self.rxd().bit()))
            .field("ctsn", &format_args!("{}", self.ctsn().bit()))
            .field("dsrn", &format_args!("{}", self.dsrn().bit()))
            .field("rxfifo_cnt", &format_args!("{}", self.rxfifo_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UART STATUS REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](index.html) module"]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_status::R](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_STATUS to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
