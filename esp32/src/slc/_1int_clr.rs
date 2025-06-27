#[doc = "Register `_1INT_CLR` writer"]
pub type W = crate::W<_1INT_CLR_SPEC>;
#[doc = "Field `FRHOST_BIT8_INT_CLR` writer - "]
pub type FRHOST_BIT8_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT9_INT_CLR` writer - "]
pub type FRHOST_BIT9_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT10_INT_CLR` writer - "]
pub type FRHOST_BIT10_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT11_INT_CLR` writer - "]
pub type FRHOST_BIT11_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT12_INT_CLR` writer - "]
pub type FRHOST_BIT12_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT13_INT_CLR` writer - "]
pub type FRHOST_BIT13_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT14_INT_CLR` writer - "]
pub type FRHOST_BIT14_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRHOST_BIT15_INT_CLR` writer - "]
pub type FRHOST_BIT15_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_START_INT_CLR` writer - "]
pub type SLC1_RX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_START_INT_CLR` writer - "]
pub type SLC1_TX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_UDF_INT_CLR` writer - "]
pub type SLC1_RX_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_OVF_INT_CLR` writer - "]
pub type SLC1_TX_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC1_TOKEN0_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC1_TOKEN1_1TO0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_DONE_INT_CLR` writer - "]
pub type SLC1_TX_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_SUC_EOF_INT_CLR` writer - "]
pub type SLC1_TX_SUC_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_DONE_INT_CLR` writer - "]
pub type SLC1_RX_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_EOF_INT_CLR` writer - "]
pub type SLC1_RX_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TOHOST_INT_CLR` writer - "]
pub type SLC1_TOHOST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC1_TX_DSCR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC1_RX_DSCR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC1_TX_DSCR_EMPTY_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_HOST_RD_ACK_INT_CLR` writer - "]
pub type SLC1_HOST_RD_ACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_WR_RETRY_DONE_INT_CLR` writer - "]
pub type SLC1_WR_RETRY_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC1_TX_ERR_EOF_INT_CLR` writer - "]
pub type SLC1_TX_ERR_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_1INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit8_int_clr(&mut self) -> FRHOST_BIT8_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT8_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit9_int_clr(&mut self) -> FRHOST_BIT9_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT9_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit10_int_clr(&mut self) -> FRHOST_BIT10_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT10_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit11_int_clr(&mut self) -> FRHOST_BIT11_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT11_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit12_int_clr(&mut self) -> FRHOST_BIT12_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT12_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit13_int_clr(&mut self) -> FRHOST_BIT13_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT13_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit14_int_clr(&mut self) -> FRHOST_BIT14_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT14_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit15_int_clr(&mut self) -> FRHOST_BIT15_INT_CLR_W<_1INT_CLR_SPEC> {
        FRHOST_BIT15_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc1_rx_start_int_clr(&mut self) -> SLC1_RX_START_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_RX_START_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc1_tx_start_int_clr(&mut self) -> SLC1_TX_START_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_START_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_clr(&mut self) -> SLC1_RX_UDF_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_RX_UDF_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_clr(&mut self) -> SLC1_TX_OVF_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_OVF_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_clr(&mut self) -> SLC1_TOKEN0_1TO0_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TOKEN0_1TO0_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_clr(&mut self) -> SLC1_TOKEN1_1TO0_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TOKEN1_1TO0_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_done_int_clr(&mut self) -> SLC1_TX_DONE_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_DONE_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc1_tx_suc_eof_int_clr(&mut self) -> SLC1_TX_SUC_EOF_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_SUC_EOF_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_done_int_clr(&mut self) -> SLC1_RX_DONE_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_RX_DONE_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_eof_int_clr(&mut self) -> SLC1_RX_EOF_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_RX_EOF_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_tohost_int_clr(&mut self) -> SLC1_TOHOST_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TOHOST_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_tx_dscr_err_int_clr(&mut self) -> SLC1_TX_DSCR_ERR_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_DSCR_ERR_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_dscr_err_int_clr(&mut self) -> SLC1_RX_DSCR_ERR_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_RX_DSCR_ERR_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_tx_dscr_empty_int_clr(&mut self) -> SLC1_TX_DSCR_EMPTY_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_DSCR_EMPTY_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_host_rd_ack_int_clr(&mut self) -> SLC1_HOST_RD_ACK_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_HOST_RD_ACK_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_wr_retry_done_int_clr(&mut self) -> SLC1_WR_RETRY_DONE_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_WR_RETRY_DONE_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_tx_err_eof_int_clr(&mut self) -> SLC1_TX_ERR_EOF_INT_CLR_W<_1INT_CLR_SPEC> {
        SLC1_TX_ERR_EOF_INT_CLR_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_1int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1INT_CLR_SPEC;
impl crate::RegisterSpec for _1INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_1int_clr::W`](W) writer structure"]
impl crate::Writable for _1INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _1INT_CLR to value 0"]
impl crate::Resettable for _1INT_CLR_SPEC {}
