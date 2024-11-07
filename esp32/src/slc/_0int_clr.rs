#[doc = "Register `_0INT_CLR` writer"]
pub type W = crate::W<_0INT_CLR_SPEC>;
#[doc = "Field `FRHOST_BIT0_INT_CLR` writer - "]
pub type FRHOST_BIT0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT1_INT_CLR` writer - "]
pub type FRHOST_BIT1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT2_INT_CLR` writer - "]
pub type FRHOST_BIT2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT3_INT_CLR` writer - "]
pub type FRHOST_BIT3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT4_INT_CLR` writer - "]
pub type FRHOST_BIT4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT5_INT_CLR` writer - "]
pub type FRHOST_BIT5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT6_INT_CLR` writer - "]
pub type FRHOST_BIT6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT7_INT_CLR` writer - "]
pub type FRHOST_BIT7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_START_INT_CLR` writer - "]
pub type SLC0_RX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_START_INT_CLR` writer - "]
pub type SLC0_TX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_UDF_INT_CLR` writer - "]
pub type SLC0_RX_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_OVF_INT_CLR` writer - "]
pub type SLC0_TX_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN0_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN1_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_DONE_INT_CLR` writer - "]
pub type SLC0_TX_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_SUC_EOF_INT_CLR` writer - "]
pub type SLC0_TX_SUC_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_DONE_INT_CLR` writer - "]
pub type SLC0_RX_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_EOF_INT_CLR` writer - "]
pub type SLC0_RX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TOHOST_INT_CLR` writer - "]
pub type SLC0_TOHOST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_RX_DSCR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_EMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_HOST_RD_ACK_INT_CLR` writer - "]
pub type SLC0_HOST_RD_ACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_WR_RETRY_DONE_INT_CLR` writer - "]
pub type SLC0_WR_RETRY_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_TX_ERR_EOF_INT_CLR` writer - "]
pub type SLC0_TX_ERR_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_DTC_INT_CLR` writer - "]
pub type CMD_DTC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC0_RX_QUICK_EOF_INT_CLR` writer - "]
pub type SLC0_RX_QUICK_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit0_int_clr(&mut self) -> FRHOST_BIT0_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit1_int_clr(&mut self) -> FRHOST_BIT1_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit2_int_clr(&mut self) -> FRHOST_BIT2_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit3_int_clr(&mut self) -> FRHOST_BIT3_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT3_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit4_int_clr(&mut self) -> FRHOST_BIT4_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT4_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit5_int_clr(&mut self) -> FRHOST_BIT5_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT5_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit6_int_clr(&mut self) -> FRHOST_BIT6_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT6_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit7_int_clr(&mut self) -> FRHOST_BIT7_INT_CLR_W<_0INT_CLR_SPEC> {
        FRHOST_BIT7_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rx_start_int_clr(&mut self) -> SLC0_RX_START_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_START_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_tx_start_int_clr(&mut self) -> SLC0_TX_START_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_START_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rx_udf_int_clr(&mut self) -> SLC0_RX_UDF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_UDF_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_tx_ovf_int_clr(&mut self) -> SLC0_TX_OVF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_OVF_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_token0_1to0_int_clr(&mut self) -> SLC0_TOKEN0_1TO0_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TOKEN0_1TO0_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_token1_1to0_int_clr(&mut self) -> SLC0_TOKEN1_1TO0_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TOKEN1_1TO0_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_tx_done_int_clr(&mut self) -> SLC0_TX_DONE_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_DONE_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_tx_suc_eof_int_clr(&mut self) -> SLC0_TX_SUC_EOF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_SUC_EOF_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rx_done_int_clr(&mut self) -> SLC0_RX_DONE_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_DONE_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc0_rx_eof_int_clr(&mut self) -> SLC0_RX_EOF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_EOF_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_tohost_int_clr(&mut self) -> SLC0_TOHOST_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TOHOST_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc0_tx_dscr_err_int_clr(&mut self) -> SLC0_TX_DSCR_ERR_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_DSCR_ERR_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_rx_dscr_err_int_clr(&mut self) -> SLC0_RX_DSCR_ERR_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_DSCR_ERR_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_tx_dscr_empty_int_clr(&mut self) -> SLC0_TX_DSCR_EMPTY_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_DSCR_EMPTY_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_host_rd_ack_int_clr(&mut self) -> SLC0_HOST_RD_ACK_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_HOST_RD_ACK_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_wr_retry_done_int_clr(&mut self) -> SLC0_WR_RETRY_DONE_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_WR_RETRY_DONE_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_err_eof_int_clr(&mut self) -> SLC0_TX_ERR_EOF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_TX_ERR_EOF_INT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmd_dtc_int_clr(&mut self) -> CMD_DTC_INT_CLR_W<_0INT_CLR_SPEC> {
        CMD_DTC_INT_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_rx_quick_eof_int_clr(&mut self) -> SLC0_RX_QUICK_EOF_INT_CLR_W<_0INT_CLR_SPEC> {
        SLC0_RX_QUICK_EOF_INT_CLR_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0INT_CLR_SPEC;
impl crate::RegisterSpec for _0INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_0int_clr::W`](W) writer structure"]
impl crate::Writable for _0INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _0INT_CLR to value 0"]
impl crate::Resettable for _0INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
