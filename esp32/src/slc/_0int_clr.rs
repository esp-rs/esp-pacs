#[doc = "Register `_0INT_CLR` writer"]
pub type W = crate::W<_0INT_CLR_SPEC>;
#[doc = "Field `FRHOST_BIT0_INT_CLR` writer - "]
pub type FRHOST_BIT0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT1_INT_CLR` writer - "]
pub type FRHOST_BIT1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT2_INT_CLR` writer - "]
pub type FRHOST_BIT2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT3_INT_CLR` writer - "]
pub type FRHOST_BIT3_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT4_INT_CLR` writer - "]
pub type FRHOST_BIT4_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT5_INT_CLR` writer - "]
pub type FRHOST_BIT5_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT6_INT_CLR` writer - "]
pub type FRHOST_BIT6_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRHOST_BIT7_INT_CLR` writer - "]
pub type FRHOST_BIT7_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_START_INT_CLR` writer - "]
pub type SLC0_RX_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_START_INT_CLR` writer - "]
pub type SLC0_TX_START_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_UDF_INT_CLR` writer - "]
pub type SLC0_RX_UDF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_OVF_INT_CLR` writer - "]
pub type SLC0_TX_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN0_1TO0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN1_1TO0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_DONE_INT_CLR` writer - "]
pub type SLC0_TX_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_SUC_EOF_INT_CLR` writer - "]
pub type SLC0_TX_SUC_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_DONE_INT_CLR` writer - "]
pub type SLC0_RX_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_EOF_INT_CLR` writer - "]
pub type SLC0_RX_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TOHOST_INT_CLR` writer - "]
pub type SLC0_TOHOST_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_RX_DSCR_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_EMPTY_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_HOST_RD_ACK_INT_CLR` writer - "]
pub type SLC0_HOST_RD_ACK_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_WR_RETRY_DONE_INT_CLR` writer - "]
pub type SLC0_WR_RETRY_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_TX_ERR_EOF_INT_CLR` writer - "]
pub type SLC0_TX_ERR_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMD_DTC_INT_CLR` writer - "]
pub type CMD_DTC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC0_RX_QUICK_EOF_INT_CLR` writer - "]
pub type SLC0_RX_QUICK_EOF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit0_int_clr(&mut self) -> FRHOST_BIT0_INT_CLR_W<_0INT_CLR_SPEC, 0> {
        FRHOST_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit1_int_clr(&mut self) -> FRHOST_BIT1_INT_CLR_W<_0INT_CLR_SPEC, 1> {
        FRHOST_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit2_int_clr(&mut self) -> FRHOST_BIT2_INT_CLR_W<_0INT_CLR_SPEC, 2> {
        FRHOST_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit3_int_clr(&mut self) -> FRHOST_BIT3_INT_CLR_W<_0INT_CLR_SPEC, 3> {
        FRHOST_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit4_int_clr(&mut self) -> FRHOST_BIT4_INT_CLR_W<_0INT_CLR_SPEC, 4> {
        FRHOST_BIT4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit5_int_clr(&mut self) -> FRHOST_BIT5_INT_CLR_W<_0INT_CLR_SPEC, 5> {
        FRHOST_BIT5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit6_int_clr(&mut self) -> FRHOST_BIT6_INT_CLR_W<_0INT_CLR_SPEC, 6> {
        FRHOST_BIT6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit7_int_clr(&mut self) -> FRHOST_BIT7_INT_CLR_W<_0INT_CLR_SPEC, 7> {
        FRHOST_BIT7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_start_int_clr(&mut self) -> SLC0_RX_START_INT_CLR_W<_0INT_CLR_SPEC, 8> {
        SLC0_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_start_int_clr(&mut self) -> SLC0_TX_START_INT_CLR_W<_0INT_CLR_SPEC, 9> {
        SLC0_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_udf_int_clr(&mut self) -> SLC0_RX_UDF_INT_CLR_W<_0INT_CLR_SPEC, 10> {
        SLC0_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_ovf_int_clr(&mut self) -> SLC0_TX_OVF_INT_CLR_W<_0INT_CLR_SPEC, 11> {
        SLC0_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token0_1to0_int_clr(&mut self) -> SLC0_TOKEN0_1TO0_INT_CLR_W<_0INT_CLR_SPEC, 12> {
        SLC0_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token1_1to0_int_clr(&mut self) -> SLC0_TOKEN1_1TO0_INT_CLR_W<_0INT_CLR_SPEC, 13> {
        SLC0_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_done_int_clr(&mut self) -> SLC0_TX_DONE_INT_CLR_W<_0INT_CLR_SPEC, 14> {
        SLC0_TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_suc_eof_int_clr(&mut self) -> SLC0_TX_SUC_EOF_INT_CLR_W<_0INT_CLR_SPEC, 15> {
        SLC0_TX_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_done_int_clr(&mut self) -> SLC0_RX_DONE_INT_CLR_W<_0INT_CLR_SPEC, 16> {
        SLC0_RX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_eof_int_clr(&mut self) -> SLC0_RX_EOF_INT_CLR_W<_0INT_CLR_SPEC, 17> {
        SLC0_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_int_clr(&mut self) -> SLC0_TOHOST_INT_CLR_W<_0INT_CLR_SPEC, 18> {
        SLC0_TOHOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_dscr_err_int_clr(&mut self) -> SLC0_TX_DSCR_ERR_INT_CLR_W<_0INT_CLR_SPEC, 19> {
        SLC0_TX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_dscr_err_int_clr(&mut self) -> SLC0_RX_DSCR_ERR_INT_CLR_W<_0INT_CLR_SPEC, 20> {
        SLC0_RX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_dscr_empty_int_clr(
        &mut self,
    ) -> SLC0_TX_DSCR_EMPTY_INT_CLR_W<_0INT_CLR_SPEC, 21> {
        SLC0_TX_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_host_rd_ack_int_clr(&mut self) -> SLC0_HOST_RD_ACK_INT_CLR_W<_0INT_CLR_SPEC, 22> {
        SLC0_HOST_RD_ACK_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_wr_retry_done_int_clr(
        &mut self,
    ) -> SLC0_WR_RETRY_DONE_INT_CLR_W<_0INT_CLR_SPEC, 23> {
        SLC0_WR_RETRY_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_err_eof_int_clr(&mut self) -> SLC0_TX_ERR_EOF_INT_CLR_W<_0INT_CLR_SPEC, 24> {
        SLC0_TX_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_dtc_int_clr(&mut self) -> CMD_DTC_INT_CLR_W<_0INT_CLR_SPEC, 25> {
        CMD_DTC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_quick_eof_int_clr(&mut self) -> SLC0_RX_QUICK_EOF_INT_CLR_W<_0INT_CLR_SPEC, 26> {
        SLC0_RX_QUICK_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0INT_CLR_SPEC;
impl crate::RegisterSpec for _0INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`_0int_clr::W`](W) writer structure"]
impl crate::Writable for _0INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0INT_CLR to value 0"]
impl crate::Resettable for _0INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
