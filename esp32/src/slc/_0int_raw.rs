#[doc = "Register `_0INT_RAW` reader"]
pub type R = crate::R<_0INT_RAW_SPEC>;
#[doc = "Field `FRHOST_BIT0_INT_RAW` reader - "]
pub type FRHOST_BIT0_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT1_INT_RAW` reader - "]
pub type FRHOST_BIT1_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT2_INT_RAW` reader - "]
pub type FRHOST_BIT2_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT3_INT_RAW` reader - "]
pub type FRHOST_BIT3_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT4_INT_RAW` reader - "]
pub type FRHOST_BIT4_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT5_INT_RAW` reader - "]
pub type FRHOST_BIT5_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT6_INT_RAW` reader - "]
pub type FRHOST_BIT6_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT7_INT_RAW` reader - "]
pub type FRHOST_BIT7_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_START_INT_RAW` reader - "]
pub type SLC0_RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_START_INT_RAW` reader - "]
pub type SLC0_TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_UDF_INT_RAW` reader - "]
pub type SLC0_RX_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_OVF_INT_RAW` reader - "]
pub type SLC0_TX_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_RAW` reader - "]
pub type SLC0_TOKEN0_1TO0_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_RAW` reader - "]
pub type SLC0_TOKEN1_1TO0_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_DONE_INT_RAW` reader - "]
pub type SLC0_TX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_SUC_EOF_INT_RAW` reader - "]
pub type SLC0_TX_SUC_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_DONE_INT_RAW` reader - "]
pub type SLC0_RX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_EOF_INT_RAW` reader - "]
pub type SLC0_RX_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TOHOST_INT_RAW` reader - "]
pub type SLC0_TOHOST_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_DSCR_ERR_INT_RAW` reader - "]
pub type SLC0_TX_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_DSCR_ERR_INT_RAW` reader - "]
pub type SLC0_RX_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_DSCR_EMPTY_INT_RAW` reader - "]
pub type SLC0_TX_DSCR_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_HOST_RD_ACK_INT_RAW` reader - "]
pub type SLC0_HOST_RD_ACK_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_WR_RETRY_DONE_INT_RAW` reader - "]
pub type SLC0_WR_RETRY_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_TX_ERR_EOF_INT_RAW` reader - "]
pub type SLC0_TX_ERR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CMD_DTC_INT_RAW` reader - "]
pub type CMD_DTC_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLC0_RX_QUICK_EOF_INT_RAW` reader - "]
pub type SLC0_RX_QUICK_EOF_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit0_int_raw(&self) -> FRHOST_BIT0_INT_RAW_R {
        FRHOST_BIT0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit1_int_raw(&self) -> FRHOST_BIT1_INT_RAW_R {
        FRHOST_BIT1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit2_int_raw(&self) -> FRHOST_BIT2_INT_RAW_R {
        FRHOST_BIT2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit3_int_raw(&self) -> FRHOST_BIT3_INT_RAW_R {
        FRHOST_BIT3_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit4_int_raw(&self) -> FRHOST_BIT4_INT_RAW_R {
        FRHOST_BIT4_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit5_int_raw(&self) -> FRHOST_BIT5_INT_RAW_R {
        FRHOST_BIT5_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit6_int_raw(&self) -> FRHOST_BIT6_INT_RAW_R {
        FRHOST_BIT6_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit7_int_raw(&self) -> FRHOST_BIT7_INT_RAW_R {
        FRHOST_BIT7_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rx_start_int_raw(&self) -> SLC0_RX_START_INT_RAW_R {
        SLC0_RX_START_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_tx_start_int_raw(&self) -> SLC0_TX_START_INT_RAW_R {
        SLC0_TX_START_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rx_udf_int_raw(&self) -> SLC0_RX_UDF_INT_RAW_R {
        SLC0_RX_UDF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_tx_ovf_int_raw(&self) -> SLC0_TX_OVF_INT_RAW_R {
        SLC0_TX_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_token0_1to0_int_raw(&self) -> SLC0_TOKEN0_1TO0_INT_RAW_R {
        SLC0_TOKEN0_1TO0_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_token1_1to0_int_raw(&self) -> SLC0_TOKEN1_1TO0_INT_RAW_R {
        SLC0_TOKEN1_1TO0_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_tx_done_int_raw(&self) -> SLC0_TX_DONE_INT_RAW_R {
        SLC0_TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_tx_suc_eof_int_raw(&self) -> SLC0_TX_SUC_EOF_INT_RAW_R {
        SLC0_TX_SUC_EOF_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rx_done_int_raw(&self) -> SLC0_RX_DONE_INT_RAW_R {
        SLC0_RX_DONE_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc0_rx_eof_int_raw(&self) -> SLC0_RX_EOF_INT_RAW_R {
        SLC0_RX_EOF_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_tohost_int_raw(&self) -> SLC0_TOHOST_INT_RAW_R {
        SLC0_TOHOST_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc0_tx_dscr_err_int_raw(&self) -> SLC0_TX_DSCR_ERR_INT_RAW_R {
        SLC0_TX_DSCR_ERR_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_rx_dscr_err_int_raw(&self) -> SLC0_RX_DSCR_ERR_INT_RAW_R {
        SLC0_RX_DSCR_ERR_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_tx_dscr_empty_int_raw(&self) -> SLC0_TX_DSCR_EMPTY_INT_RAW_R {
        SLC0_TX_DSCR_EMPTY_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_host_rd_ack_int_raw(&self) -> SLC0_HOST_RD_ACK_INT_RAW_R {
        SLC0_HOST_RD_ACK_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_wr_retry_done_int_raw(&self) -> SLC0_WR_RETRY_DONE_INT_RAW_R {
        SLC0_WR_RETRY_DONE_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_err_eof_int_raw(&self) -> SLC0_TX_ERR_EOF_INT_RAW_R {
        SLC0_TX_ERR_EOF_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmd_dtc_int_raw(&self) -> CMD_DTC_INT_RAW_R {
        CMD_DTC_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_rx_quick_eof_int_raw(&self) -> SLC0_RX_QUICK_EOF_INT_RAW_R {
        SLC0_RX_QUICK_EOF_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0INT_RAW")
            .field("frhost_bit0_int_raw", &self.frhost_bit0_int_raw())
            .field("frhost_bit1_int_raw", &self.frhost_bit1_int_raw())
            .field("frhost_bit2_int_raw", &self.frhost_bit2_int_raw())
            .field("frhost_bit3_int_raw", &self.frhost_bit3_int_raw())
            .field("frhost_bit4_int_raw", &self.frhost_bit4_int_raw())
            .field("frhost_bit5_int_raw", &self.frhost_bit5_int_raw())
            .field("frhost_bit6_int_raw", &self.frhost_bit6_int_raw())
            .field("frhost_bit7_int_raw", &self.frhost_bit7_int_raw())
            .field("slc0_rx_start_int_raw", &self.slc0_rx_start_int_raw())
            .field("slc0_tx_start_int_raw", &self.slc0_tx_start_int_raw())
            .field("slc0_rx_udf_int_raw", &self.slc0_rx_udf_int_raw())
            .field("slc0_tx_ovf_int_raw", &self.slc0_tx_ovf_int_raw())
            .field("slc0_token0_1to0_int_raw", &self.slc0_token0_1to0_int_raw())
            .field("slc0_token1_1to0_int_raw", &self.slc0_token1_1to0_int_raw())
            .field("slc0_tx_done_int_raw", &self.slc0_tx_done_int_raw())
            .field("slc0_tx_suc_eof_int_raw", &self.slc0_tx_suc_eof_int_raw())
            .field("slc0_rx_done_int_raw", &self.slc0_rx_done_int_raw())
            .field("slc0_rx_eof_int_raw", &self.slc0_rx_eof_int_raw())
            .field("slc0_tohost_int_raw", &self.slc0_tohost_int_raw())
            .field("slc0_tx_dscr_err_int_raw", &self.slc0_tx_dscr_err_int_raw())
            .field("slc0_rx_dscr_err_int_raw", &self.slc0_rx_dscr_err_int_raw())
            .field(
                "slc0_tx_dscr_empty_int_raw",
                &self.slc0_tx_dscr_empty_int_raw(),
            )
            .field("slc0_host_rd_ack_int_raw", &self.slc0_host_rd_ack_int_raw())
            .field(
                "slc0_wr_retry_done_int_raw",
                &self.slc0_wr_retry_done_int_raw(),
            )
            .field("slc0_tx_err_eof_int_raw", &self.slc0_tx_err_eof_int_raw())
            .field("cmd_dtc_int_raw", &self.cmd_dtc_int_raw())
            .field(
                "slc0_rx_quick_eof_int_raw",
                &self.slc0_rx_quick_eof_int_raw(),
            )
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_0int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0INT_RAW_SPEC;
impl crate::RegisterSpec for _0INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0int_raw::R`](R) reader structure"]
impl crate::Readable for _0INT_RAW_SPEC {}
#[doc = "`reset()` method sets _0INT_RAW to value 0"]
impl crate::Resettable for _0INT_RAW_SPEC {}
