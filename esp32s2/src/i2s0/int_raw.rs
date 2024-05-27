#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RX_TAKE_DATA` reader - The raw interrupt status bit for I2S_RX_TAKE_DATA_INT interrupt."]
pub type RX_TAKE_DATA_R = crate::BitReader;
#[doc = "Field `TX_PUT_DATA` reader - The raw interrupt status bit for I2S_TX_PUT_DATA_INT interrupt."]
pub type TX_PUT_DATA_R = crate::BitReader;
#[doc = "Field `RX_WFULL` reader - The raw interrupt status bit for I2S_RX_WFULL_INT interrupt."]
pub type RX_WFULL_R = crate::BitReader;
#[doc = "Field `RX_REMPTY` reader - The raw interrupt status bit for I2S_RX_REMPTY_INT interrupt."]
pub type RX_REMPTY_R = crate::BitReader;
#[doc = "Field `TX_WFULL` reader - The raw interrupt status bit for I2S_TX_WFULL_INT interrupt."]
pub type TX_WFULL_R = crate::BitReader;
#[doc = "Field `TX_REMPTY` reader - The raw interrupt status bit for I2S_TX_REMPTY_INT interrupt."]
pub type TX_REMPTY_R = crate::BitReader;
#[doc = "Field `RX_HUNG` reader - The raw interrupt status bit for I2S_RX_HUNG_INT interrupt."]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `TX_HUNG` reader - The raw interrupt status bit for I2S_TX_HUNG_INT interrupt."]
pub type TX_HUNG_R = crate::BitReader;
#[doc = "Field `IN_DONE` reader - The raw interrupt status bit for I2S_IN_DONE_INT interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status bit for I2S_IN_SUC_EOF_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` reader - Reserved."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DONE` reader - The raw interrupt status bit for I2S_OUT_DONE_INT interrupt."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The raw interrupt status bit for I2S_OUT_EOF_INT interrupt."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt status bit for I2S_IN_DSCR_ERR_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt status bit for I2S_OUT_DSCR_ERR_INT interrupt."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt status bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt status bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `V_SYNC` reader - The raw interrupt status bit for I2S_V_SYNC_INT interrupt."]
pub type V_SYNC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for I2S_RX_TAKE_DATA_INT interrupt."]
    #[inline(always)]
    pub fn rx_take_data(&self) -> RX_TAKE_DATA_R {
        RX_TAKE_DATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for I2S_TX_PUT_DATA_INT interrupt."]
    #[inline(always)]
    pub fn tx_put_data(&self) -> TX_PUT_DATA_R {
        TX_PUT_DATA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for I2S_RX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn rx_wfull(&self) -> RX_WFULL_R {
        RX_WFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for I2S_RX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn rx_rempty(&self) -> RX_REMPTY_R {
        RX_REMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for I2S_TX_WFULL_INT interrupt."]
    #[inline(always)]
    pub fn tx_wfull(&self) -> TX_WFULL_R {
        TX_WFULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for I2S_TX_REMPTY_INT interrupt."]
    #[inline(always)]
    pub fn tx_rempty(&self) -> TX_REMPTY_R {
        TX_REMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for I2S_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for I2S_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for I2S_IN_DONE_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for I2S_IN_SUC_EOF_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for I2S_OUT_DONE_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for I2S_OUT_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for I2S_IN_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for I2S_OUT_DSCR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for I2S_IN_DSCR_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt status bit for I2S_OUT_TOTAL_EOF_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw interrupt status bit for I2S_V_SYNC_INT interrupt."]
    #[inline(always)]
    pub fn v_sync(&self) -> V_SYNC_R {
        V_SYNC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_take_data", &self.rx_take_data())
            .field("tx_put_data", &self.tx_put_data())
            .field("rx_wfull", &self.rx_wfull())
            .field("rx_rempty", &self.rx_rempty())
            .field("tx_wfull", &self.tx_wfull())
            .field("tx_rempty", &self.tx_rempty())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("out_total_eof", &self.out_total_eof())
            .field("v_sync", &self.v_sync())
            .finish()
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
