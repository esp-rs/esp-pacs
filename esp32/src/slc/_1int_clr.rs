#[doc = "Register `_1INT_CLR` writer"]
pub struct W(crate::W<_1INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<_1INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRHOST_BIT8_INT_CLR` writer - "]
pub type FRHOST_BIT8_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT9_INT_CLR` writer - "]
pub type FRHOST_BIT9_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT10_INT_CLR` writer - "]
pub type FRHOST_BIT10_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT11_INT_CLR` writer - "]
pub type FRHOST_BIT11_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT12_INT_CLR` writer - "]
pub type FRHOST_BIT12_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT13_INT_CLR` writer - "]
pub type FRHOST_BIT13_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT14_INT_CLR` writer - "]
pub type FRHOST_BIT14_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `FRHOST_BIT15_INT_CLR` writer - "]
pub type FRHOST_BIT15_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_RX_START_INT_CLR` writer - "]
pub type SLC1_RX_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_START_INT_CLR` writer - "]
pub type SLC1_TX_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_RX_UDF_INT_CLR` writer - "]
pub type SLC1_RX_UDF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_OVF_INT_CLR` writer - "]
pub type SLC1_TX_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC1_TOKEN0_1TO0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC1_TOKEN1_1TO0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_DONE_INT_CLR` writer - "]
pub type SLC1_TX_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_SUC_EOF_INT_CLR` writer - "]
pub type SLC1_TX_SUC_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_RX_DONE_INT_CLR` writer - "]
pub type SLC1_RX_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_RX_EOF_INT_CLR` writer - "]
pub type SLC1_RX_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TOHOST_INT_CLR` writer - "]
pub type SLC1_TOHOST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC1_TX_DSCR_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC1_RX_DSCR_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC1_TX_DSCR_EMPTY_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_HOST_RD_ACK_INT_CLR` writer - "]
pub type SLC1_HOST_RD_ACK_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_WR_RETRY_DONE_INT_CLR` writer - "]
pub type SLC1_WR_RETRY_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[doc = "Field `SLC1_TX_ERR_EOF_INT_CLR` writer - "]
pub type SLC1_TX_ERR_EOF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, _1INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_1INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit8_int_clr(&mut self) -> FRHOST_BIT8_INT_CLR_W<0> {
        FRHOST_BIT8_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit9_int_clr(&mut self) -> FRHOST_BIT9_INT_CLR_W<1> {
        FRHOST_BIT9_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit10_int_clr(&mut self) -> FRHOST_BIT10_INT_CLR_W<2> {
        FRHOST_BIT10_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit11_int_clr(&mut self) -> FRHOST_BIT11_INT_CLR_W<3> {
        FRHOST_BIT11_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit12_int_clr(&mut self) -> FRHOST_BIT12_INT_CLR_W<4> {
        FRHOST_BIT12_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit13_int_clr(&mut self) -> FRHOST_BIT13_INT_CLR_W<5> {
        FRHOST_BIT13_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit14_int_clr(&mut self) -> FRHOST_BIT14_INT_CLR_W<6> {
        FRHOST_BIT14_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit15_int_clr(&mut self) -> FRHOST_BIT15_INT_CLR_W<7> {
        FRHOST_BIT15_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_start_int_clr(&mut self) -> SLC1_RX_START_INT_CLR_W<8> {
        SLC1_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_start_int_clr(&mut self) -> SLC1_TX_START_INT_CLR_W<9> {
        SLC1_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_udf_int_clr(&mut self) -> SLC1_RX_UDF_INT_CLR_W<10> {
        SLC1_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_ovf_int_clr(&mut self) -> SLC1_TX_OVF_INT_CLR_W<11> {
        SLC1_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token0_1to0_int_clr(&mut self) -> SLC1_TOKEN0_1TO0_INT_CLR_W<12> {
        SLC1_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token1_1to0_int_clr(&mut self) -> SLC1_TOKEN1_1TO0_INT_CLR_W<13> {
        SLC1_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_done_int_clr(&mut self) -> SLC1_TX_DONE_INT_CLR_W<14> {
        SLC1_TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_suc_eof_int_clr(&mut self) -> SLC1_TX_SUC_EOF_INT_CLR_W<15> {
        SLC1_TX_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_done_int_clr(&mut self) -> SLC1_RX_DONE_INT_CLR_W<16> {
        SLC1_RX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_eof_int_clr(&mut self) -> SLC1_RX_EOF_INT_CLR_W<17> {
        SLC1_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_int_clr(&mut self) -> SLC1_TOHOST_INT_CLR_W<18> {
        SLC1_TOHOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_dscr_err_int_clr(&mut self) -> SLC1_TX_DSCR_ERR_INT_CLR_W<19> {
        SLC1_TX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_dscr_err_int_clr(&mut self) -> SLC1_RX_DSCR_ERR_INT_CLR_W<20> {
        SLC1_RX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_dscr_empty_int_clr(&mut self) -> SLC1_TX_DSCR_EMPTY_INT_CLR_W<21> {
        SLC1_TX_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_host_rd_ack_int_clr(&mut self) -> SLC1_HOST_RD_ACK_INT_CLR_W<22> {
        SLC1_HOST_RD_ACK_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_wr_retry_done_int_clr(&mut self) -> SLC1_WR_RETRY_DONE_INT_CLR_W<23> {
        SLC1_WR_RETRY_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_err_eof_int_clr(&mut self) -> SLC1_TX_ERR_EOF_INT_CLR_W<24> {
        SLC1_TX_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1int_clr](index.html) module"]
pub struct _1INT_CLR_SPEC;
impl crate::RegisterSpec for _1INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [_1int_clr::W](W) writer structure"]
impl crate::Writable for _1INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _1INT_CLR to value 0"]
impl crate::Resettable for _1INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
