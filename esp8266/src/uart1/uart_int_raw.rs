#[doc = "Register `UART_INT_RAW` reader"]
pub struct R(crate::R<UART_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rxfifo_full_int_raw` reader - The interrupt raw bit for rx fifo full interrupt(depands onUART_RXFIFO_FULL_THRHD bits)"]
pub type RXFIFO_FULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `txfifo_empty_int_raw` reader - The interrupt raw bit for tx fifo empty interrupt(depands onUART_TXFIFO_EMPTY_THRHD bits)"]
pub type TXFIFO_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `parity_err_int_raw` reader - The interrupt raw bit for parity check error"]
pub type PARITY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `frm_err_int_raw` reader - The interrupt raw bit for other rx error"]
pub type FRM_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `rxfifo_ovf_int_raw` reader - The interrupt raw bit for rx fifo overflow"]
pub type RXFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `dsr_chg_int_raw` reader - The interrupt raw bit for DSR changing level"]
pub type DSR_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `cts_chg_int_raw` reader - The interrupt raw bit for CTS changing level"]
pub type CTS_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `brk_det_int_raw` reader - The interrupt raw bit for Rx byte start error"]
pub type BRK_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `rxfifo_tout_int_raw` reader - The interrupt raw bit for Rx time-out interrupt(depands on theUART_RX_TOUT_THRHD)"]
pub type RXFIFO_TOUT_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for rx fifo full interrupt(depands onUART_RXFIFO_FULL_THRHD bits)"]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for tx fifo empty interrupt(depands onUART_TXFIFO_EMPTY_THRHD bits)"]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for parity check error"]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for other rx error"]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for rx fifo overflow"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for DSR changing level"]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CTS changing level"]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for Rx byte start error"]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for Rx time-out interrupt(depands on theUART_RX_TOUT_THRHD)"]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_INT_RAW")
            .field(
                "rxfifo_tout_int_raw",
                &format_args!("{}", self.rxfifo_tout_int_raw().bit()),
            )
            .field(
                "brk_det_int_raw",
                &format_args!("{}", self.brk_det_int_raw().bit()),
            )
            .field(
                "cts_chg_int_raw",
                &format_args!("{}", self.cts_chg_int_raw().bit()),
            )
            .field(
                "dsr_chg_int_raw",
                &format_args!("{}", self.dsr_chg_int_raw().bit()),
            )
            .field(
                "rxfifo_ovf_int_raw",
                &format_args!("{}", self.rxfifo_ovf_int_raw().bit()),
            )
            .field(
                "frm_err_int_raw",
                &format_args!("{}", self.frm_err_int_raw().bit()),
            )
            .field(
                "parity_err_int_raw",
                &format_args!("{}", self.parity_err_int_raw().bit()),
            )
            .field(
                "txfifo_empty_int_raw",
                &format_args!("{}", self.txfifo_empty_int_raw().bit()),
            )
            .field(
                "rxfifo_full_int_raw",
                &format_args!("{}", self.rxfifo_full_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UART INTERRUPT RAW STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_raw](index.html) module"]
pub struct UART_INT_RAW_SPEC;
impl crate::RegisterSpec for UART_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_raw::R](R) reader structure"]
impl crate::Readable for UART_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_INT_RAW to value 0"]
impl crate::Resettable for UART_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
