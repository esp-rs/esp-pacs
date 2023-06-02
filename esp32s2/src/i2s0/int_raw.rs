#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_TAKE_DATA_INT_RAW` reader - The raw interrupt status bit for I2S_RX_TAKE_DATA_INT interrupt."]
pub type RX_TAKE_DATA_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_PUT_DATA_INT_RAW` reader - The raw interrupt status bit for I2S_TX_PUT_DATA_INT interrupt."]
pub type TX_PUT_DATA_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_WFULL_INT_RAW` reader - The raw interrupt status bit for I2S_RX_WFULL_INT interrupt."]
pub type RX_WFULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_REMPTY_INT_RAW` reader - The raw interrupt status bit for I2S_RX_REMPTY_INT interrupt."]
pub type RX_REMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_WFULL_INT_RAW` reader - The raw interrupt status bit for I2S_TX_WFULL_INT interrupt."]
pub type TX_WFULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_REMPTY_INT_RAW` reader - The raw interrupt status bit for I2S_TX_REMPTY_INT interrupt."]
pub type TX_REMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` reader - The raw interrupt status bit for I2S_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` reader - The raw interrupt status bit for I2S_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_RAW` reader - The raw interrupt status bit for I2S_IN_DONE_INT interrupt."]
pub type IN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_RAW` reader - The raw interrupt status bit for I2S_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_RAW` reader - Reserved."]
pub type IN_ERR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_RAW` reader - The raw interrupt status bit for I2S_OUT_DONE_INT interrupt."]
pub type OUT_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` reader - The raw interrupt status bit for I2S_OUT_EOF_INT interrupt."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_INT_RAW` reader - The raw interrupt status bit for I2S_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_INT_RAW` reader - The raw interrupt status bit for I2S_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_INT_RAW` reader - The raw interrupt status bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_RAW` reader - The raw interrupt status bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `V_SYNC_INT_RAW` reader - The raw interrupt status bit for I2S_V_SYNC_INT interrupt."]
pub type V_SYNC_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for I2S_RX_TAKE_DATA_INT interrupt."]
    #[inline(always)]
    pub fn rx_take_data_int_raw(&self) -> RX_TAKE_DATA_INT_RAW_R {
        RX_TAKE_DATA_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for I2S_TX_PUT_DATA_INT interrupt."]
    #[inline(always)]
    pub fn tx_put_data_int_raw(&self) -> TX_PUT_DATA_INT_RAW_R {
        TX_PUT_DATA_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for I2S_RX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn rx_wfull_int_raw(&self) -> RX_WFULL_INT_RAW_R {
        RX_WFULL_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for I2S_RX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn rx_rempty_int_raw(&self) -> RX_REMPTY_INT_RAW_R {
        RX_REMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for I2S_TX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn tx_wfull_int_raw(&self) -> TX_WFULL_INT_RAW_R {
        TX_WFULL_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for I2S_TX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn tx_rempty_int_raw(&self) -> TX_REMPTY_INT_RAW_R {
        TX_REMPTY_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for I2S_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for I2S_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for I2S_IN_DONE_INT interrupt."]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for I2S_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for I2S_OUT_DONE_INT interrupt."]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for I2S_OUT_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for I2S_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_int_raw(&self) -> IN_DSCR_ERR_INT_RAW_R {
        IN_DSCR_ERR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for I2S_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_int_raw(&self) -> OUT_DSCR_ERR_INT_RAW_R {
        OUT_DSCR_ERR_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_int_raw(&self) -> IN_DSCR_EMPTY_INT_RAW_R {
        IN_DSCR_EMPTY_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt status bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw interrupt status bit for I2S_V_SYNC_INT interrupt."]
    #[inline(always)]
    pub fn v_sync_int_raw(&self) -> V_SYNC_INT_RAW_R {
        V_SYNC_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rx_take_data_int_raw",
                &format_args!("{}", self.rx_take_data_int_raw().bit()),
            )
            .field(
                "tx_put_data_int_raw",
                &format_args!("{}", self.tx_put_data_int_raw().bit()),
            )
            .field(
                "rx_wfull_int_raw",
                &format_args!("{}", self.rx_wfull_int_raw().bit()),
            )
            .field(
                "rx_rempty_int_raw",
                &format_args!("{}", self.rx_rempty_int_raw().bit()),
            )
            .field(
                "tx_wfull_int_raw",
                &format_args!("{}", self.tx_wfull_int_raw().bit()),
            )
            .field(
                "tx_rempty_int_raw",
                &format_args!("{}", self.tx_rempty_int_raw().bit()),
            )
            .field(
                "rx_hung_int_raw",
                &format_args!("{}", self.rx_hung_int_raw().bit()),
            )
            .field(
                "tx_hung_int_raw",
                &format_args!("{}", self.tx_hung_int_raw().bit()),
            )
            .field(
                "in_done_int_raw",
                &format_args!("{}", self.in_done_int_raw().bit()),
            )
            .field(
                "in_suc_eof_int_raw",
                &format_args!("{}", self.in_suc_eof_int_raw().bit()),
            )
            .field(
                "in_err_eof_int_raw",
                &format_args!("{}", self.in_err_eof_int_raw().bit()),
            )
            .field(
                "out_done_int_raw",
                &format_args!("{}", self.out_done_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "in_dscr_err_int_raw",
                &format_args!("{}", self.in_dscr_err_int_raw().bit()),
            )
            .field(
                "out_dscr_err_int_raw",
                &format_args!("{}", self.out_dscr_err_int_raw().bit()),
            )
            .field(
                "in_dscr_empty_int_raw",
                &format_args!("{}", self.in_dscr_empty_int_raw().bit()),
            )
            .field(
                "out_total_eof_int_raw",
                &format_args!("{}", self.out_total_eof_int_raw().bit()),
            )
            .field(
                "v_sync_int_raw",
                &format_args!("{}", self.v_sync_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
