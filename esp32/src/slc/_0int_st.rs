#[doc = "Register `_0INT_ST` reader"]
pub struct R(crate::R<_0INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRHOST_BIT0_INT_ST` reader - "]
pub type FRHOST_BIT0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT1_INT_ST` reader - "]
pub type FRHOST_BIT1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT2_INT_ST` reader - "]
pub type FRHOST_BIT2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT3_INT_ST` reader - "]
pub type FRHOST_BIT3_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT4_INT_ST` reader - "]
pub type FRHOST_BIT4_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT5_INT_ST` reader - "]
pub type FRHOST_BIT5_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT6_INT_ST` reader - "]
pub type FRHOST_BIT6_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `FRHOST_BIT7_INT_ST` reader - "]
pub type FRHOST_BIT7_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_START_INT_ST` reader - "]
pub type SLC0_RX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_START_INT_ST` reader - "]
pub type SLC0_TX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_UDF_INT_ST` reader - "]
pub type SLC0_RX_UDF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_OVF_INT_ST` reader - "]
pub type SLC0_TX_OVF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_ST` reader - "]
pub type SLC0_TOKEN0_1TO0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_ST` reader - "]
pub type SLC0_TOKEN1_1TO0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_DONE_INT_ST` reader - "]
pub type SLC0_TX_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_SUC_EOF_INT_ST` reader - "]
pub type SLC0_TX_SUC_EOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_DONE_INT_ST` reader - "]
pub type SLC0_RX_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_EOF_INT_ST` reader - "]
pub type SLC0_RX_EOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TOHOST_INT_ST` reader - "]
pub type SLC0_TOHOST_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_DSCR_ERR_INT_ST` reader - "]
pub type SLC0_TX_DSCR_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_DSCR_ERR_INT_ST` reader - "]
pub type SLC0_RX_DSCR_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_DSCR_EMPTY_INT_ST` reader - "]
pub type SLC0_TX_DSCR_EMPTY_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_HOST_RD_ACK_INT_ST` reader - "]
pub type SLC0_HOST_RD_ACK_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_WR_RETRY_DONE_INT_ST` reader - "]
pub type SLC0_WR_RETRY_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_ERR_EOF_INT_ST` reader - "]
pub type SLC0_TX_ERR_EOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `CMD_DTC_INT_ST` reader - "]
pub type CMD_DTC_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_RX_QUICK_EOF_INT_ST` reader - "]
pub type SLC0_RX_QUICK_EOF_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit0_int_st(&self) -> FRHOST_BIT0_INT_ST_R {
        FRHOST_BIT0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit1_int_st(&self) -> FRHOST_BIT1_INT_ST_R {
        FRHOST_BIT1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit2_int_st(&self) -> FRHOST_BIT2_INT_ST_R {
        FRHOST_BIT2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit3_int_st(&self) -> FRHOST_BIT3_INT_ST_R {
        FRHOST_BIT3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit4_int_st(&self) -> FRHOST_BIT4_INT_ST_R {
        FRHOST_BIT4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit5_int_st(&self) -> FRHOST_BIT5_INT_ST_R {
        FRHOST_BIT5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit6_int_st(&self) -> FRHOST_BIT6_INT_ST_R {
        FRHOST_BIT6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit7_int_st(&self) -> FRHOST_BIT7_INT_ST_R {
        FRHOST_BIT7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rx_start_int_st(&self) -> SLC0_RX_START_INT_ST_R {
        SLC0_RX_START_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_tx_start_int_st(&self) -> SLC0_TX_START_INT_ST_R {
        SLC0_TX_START_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rx_udf_int_st(&self) -> SLC0_RX_UDF_INT_ST_R {
        SLC0_RX_UDF_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_tx_ovf_int_st(&self) -> SLC0_TX_OVF_INT_ST_R {
        SLC0_TX_OVF_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_token0_1to0_int_st(&self) -> SLC0_TOKEN0_1TO0_INT_ST_R {
        SLC0_TOKEN0_1TO0_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_token1_1to0_int_st(&self) -> SLC0_TOKEN1_1TO0_INT_ST_R {
        SLC0_TOKEN1_1TO0_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_tx_done_int_st(&self) -> SLC0_TX_DONE_INT_ST_R {
        SLC0_TX_DONE_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_tx_suc_eof_int_st(&self) -> SLC0_TX_SUC_EOF_INT_ST_R {
        SLC0_TX_SUC_EOF_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rx_done_int_st(&self) -> SLC0_RX_DONE_INT_ST_R {
        SLC0_RX_DONE_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc0_rx_eof_int_st(&self) -> SLC0_RX_EOF_INT_ST_R {
        SLC0_RX_EOF_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_tohost_int_st(&self) -> SLC0_TOHOST_INT_ST_R {
        SLC0_TOHOST_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc0_tx_dscr_err_int_st(&self) -> SLC0_TX_DSCR_ERR_INT_ST_R {
        SLC0_TX_DSCR_ERR_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_rx_dscr_err_int_st(&self) -> SLC0_RX_DSCR_ERR_INT_ST_R {
        SLC0_RX_DSCR_ERR_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_tx_dscr_empty_int_st(&self) -> SLC0_TX_DSCR_EMPTY_INT_ST_R {
        SLC0_TX_DSCR_EMPTY_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_host_rd_ack_int_st(&self) -> SLC0_HOST_RD_ACK_INT_ST_R {
        SLC0_HOST_RD_ACK_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_wr_retry_done_int_st(&self) -> SLC0_WR_RETRY_DONE_INT_ST_R {
        SLC0_WR_RETRY_DONE_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_err_eof_int_st(&self) -> SLC0_TX_ERR_EOF_INT_ST_R {
        SLC0_TX_ERR_EOF_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmd_dtc_int_st(&self) -> CMD_DTC_INT_ST_R {
        CMD_DTC_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_rx_quick_eof_int_st(&self) -> SLC0_RX_QUICK_EOF_INT_ST_R {
        SLC0_RX_QUICK_EOF_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0int_st](index.html) module"]
pub struct _0INT_ST_SPEC;
impl crate::RegisterSpec for _0INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0int_st::R](R) reader structure"]
impl crate::Readable for _0INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0INT_ST to value 0"]
impl crate::Resettable for _0INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
