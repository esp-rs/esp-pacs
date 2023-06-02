#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TX_RST` reader - "]
pub type SLC0_TX_RST_R = crate::BitReader;
#[doc = "Field `SLC0_TX_RST` writer - "]
pub type SLC0_TX_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RX_RST` reader - "]
pub type SLC0_RX_RST_R = crate::BitReader;
#[doc = "Field `SLC0_RX_RST` writer - "]
pub type SLC0_RX_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `AHBM_FIFO_RST` reader - "]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - "]
pub type AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `AHBM_RST` reader - "]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - "]
pub type AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TX_LOOP_TEST` reader - "]
pub type SLC0_TX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC0_TX_LOOP_TEST` writer - "]
pub type SLC0_TX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RX_LOOP_TEST` reader - "]
pub type SLC0_RX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC0_RX_LOOP_TEST` writer - "]
pub type SLC0_RX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RX_AUTO_WRBACK` reader - "]
pub type SLC0_RX_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `SLC0_RX_AUTO_WRBACK` writer - "]
pub type SLC0_RX_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RX_NO_RESTART_CLR` reader - "]
pub type SLC0_RX_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `SLC0_RX_NO_RESTART_CLR` writer - "]
pub type SLC0_RX_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RXDSCR_BURST_EN` reader - "]
pub type SLC0_RXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RXDSCR_BURST_EN` writer - "]
pub type SLC0_RXDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RXDATA_BURST_EN` reader - "]
pub type SLC0_RXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC0_RXDATA_BURST_EN` writer - "]
pub type SLC0_RXDATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_RXLINK_AUTO_RET` reader - "]
pub type SLC0_RXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC0_RXLINK_AUTO_RET` writer - "]
pub type SLC0_RXLINK_AUTO_RET_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TXLINK_AUTO_RET` reader - "]
pub type SLC0_TXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC0_TXLINK_AUTO_RET` writer - "]
pub type SLC0_TXLINK_AUTO_RET_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TXDSCR_BURST_EN` reader - "]
pub type SLC0_TXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC0_TXDSCR_BURST_EN` writer - "]
pub type SLC0_TXDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TXDATA_BURST_EN` reader - "]
pub type SLC0_TXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC0_TXDATA_BURST_EN` writer - "]
pub type SLC0_TXDATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TOKEN_AUTO_CLR` reader - "]
pub type SLC0_TOKEN_AUTO_CLR_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN_AUTO_CLR` writer - "]
pub type SLC0_TOKEN_AUTO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_TOKEN_SEL` reader - "]
pub type SLC0_TOKEN_SEL_R = crate::BitReader;
#[doc = "Field `SLC0_TOKEN_SEL` writer - "]
pub type SLC0_TOKEN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TX_RST` reader - "]
pub type SLC1_TX_RST_R = crate::BitReader;
#[doc = "Field `SLC1_TX_RST` writer - "]
pub type SLC1_TX_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RX_RST` reader - "]
pub type SLC1_RX_RST_R = crate::BitReader;
#[doc = "Field `SLC1_RX_RST` writer - "]
pub type SLC1_RX_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC0_WR_RETRY_MASK_EN` reader - "]
pub type SLC0_WR_RETRY_MASK_EN_R = crate::BitReader;
#[doc = "Field `SLC0_WR_RETRY_MASK_EN` writer - "]
pub type SLC0_WR_RETRY_MASK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_WR_RETRY_MASK_EN` reader - "]
pub type SLC1_WR_RETRY_MASK_EN_R = crate::BitReader;
#[doc = "Field `SLC1_WR_RETRY_MASK_EN` writer - "]
pub type SLC1_WR_RETRY_MASK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TX_LOOP_TEST` reader - "]
pub type SLC1_TX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC1_TX_LOOP_TEST` writer - "]
pub type SLC1_TX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RX_LOOP_TEST` reader - "]
pub type SLC1_RX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC1_RX_LOOP_TEST` writer - "]
pub type SLC1_RX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RX_AUTO_WRBACK` reader - "]
pub type SLC1_RX_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `SLC1_RX_AUTO_WRBACK` writer - "]
pub type SLC1_RX_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RX_NO_RESTART_CLR` reader - "]
pub type SLC1_RX_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `SLC1_RX_NO_RESTART_CLR` writer - "]
pub type SLC1_RX_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RXDSCR_BURST_EN` reader - "]
pub type SLC1_RXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RXDSCR_BURST_EN` writer - "]
pub type SLC1_RXDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RXDATA_BURST_EN` reader - "]
pub type SLC1_RXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC1_RXDATA_BURST_EN` writer - "]
pub type SLC1_RXDATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_RXLINK_AUTO_RET` reader - "]
pub type SLC1_RXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC1_RXLINK_AUTO_RET` writer - "]
pub type SLC1_RXLINK_AUTO_RET_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TXLINK_AUTO_RET` reader - "]
pub type SLC1_TXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC1_TXLINK_AUTO_RET` writer - "]
pub type SLC1_TXLINK_AUTO_RET_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TXDSCR_BURST_EN` reader - "]
pub type SLC1_TXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC1_TXDSCR_BURST_EN` writer - "]
pub type SLC1_TXDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TXDATA_BURST_EN` reader - "]
pub type SLC1_TXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC1_TXDATA_BURST_EN` writer - "]
pub type SLC1_TXDATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TOKEN_AUTO_CLR` reader - "]
pub type SLC1_TOKEN_AUTO_CLR_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN_AUTO_CLR` writer - "]
pub type SLC1_TOKEN_AUTO_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SLC1_TOKEN_SEL` reader - "]
pub type SLC1_TOKEN_SEL_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN_SEL` writer - "]
pub type SLC1_TOKEN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_rst(&self) -> SLC0_TX_RST_R {
        SLC0_TX_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_rx_rst(&self) -> SLC0_RX_RST_R {
        SLC0_RX_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_tx_loop_test(&self) -> SLC0_TX_LOOP_TEST_R {
        SLC0_TX_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_rx_loop_test(&self) -> SLC0_RX_LOOP_TEST_R {
        SLC0_RX_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_auto_wrback(&self) -> SLC0_RX_AUTO_WRBACK_R {
        SLC0_RX_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc0_rx_no_restart_clr(&self) -> SLC0_RX_NO_RESTART_CLR_R {
        SLC0_RX_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rxdscr_burst_en(&self) -> SLC0_RXDSCR_BURST_EN_R {
        SLC0_RXDSCR_BURST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_rxdata_burst_en(&self) -> SLC0_RXDATA_BURST_EN_R {
        SLC0_RXDATA_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rxlink_auto_ret(&self) -> SLC0_RXLINK_AUTO_RET_R {
        SLC0_RXLINK_AUTO_RET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_txlink_auto_ret(&self) -> SLC0_TXLINK_AUTO_RET_R {
        SLC0_TXLINK_AUTO_RET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_txdscr_burst_en(&self) -> SLC0_TXDSCR_BURST_EN_R {
        SLC0_TXDSCR_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_txdata_burst_en(&self) -> SLC0_TXDATA_BURST_EN_R {
        SLC0_TXDATA_BURST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_token_auto_clr(&self) -> SLC0_TOKEN_AUTO_CLR_R {
        SLC0_TOKEN_AUTO_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_token_sel(&self) -> SLC0_TOKEN_SEL_R {
        SLC0_TOKEN_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_rst(&self) -> SLC1_TX_RST_R {
        SLC1_TX_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_rst(&self) -> SLC1_RX_RST_R {
        SLC1_RX_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_wr_retry_mask_en(&self) -> SLC0_WR_RETRY_MASK_EN_R {
        SLC0_WR_RETRY_MASK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_wr_retry_mask_en(&self) -> SLC1_WR_RETRY_MASK_EN_R {
        SLC1_WR_RETRY_MASK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_loop_test(&self) -> SLC1_TX_LOOP_TEST_R {
        SLC1_TX_LOOP_TEST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_loop_test(&self) -> SLC1_RX_LOOP_TEST_R {
        SLC1_RX_LOOP_TEST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_rx_auto_wrback(&self) -> SLC1_RX_AUTO_WRBACK_R {
        SLC1_RX_AUTO_WRBACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_rx_no_restart_clr(&self) -> SLC1_RX_NO_RESTART_CLR_R {
        SLC1_RX_NO_RESTART_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_rxdscr_burst_en(&self) -> SLC1_RXDSCR_BURST_EN_R {
        SLC1_RXDSCR_BURST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc1_rxdata_burst_en(&self) -> SLC1_RXDATA_BURST_EN_R {
        SLC1_RXDATA_BURST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc1_rxlink_auto_ret(&self) -> SLC1_RXLINK_AUTO_RET_R {
        SLC1_RXLINK_AUTO_RET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc1_txlink_auto_ret(&self) -> SLC1_TXLINK_AUTO_RET_R {
        SLC1_TXLINK_AUTO_RET_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_txdscr_burst_en(&self) -> SLC1_TXDSCR_BURST_EN_R {
        SLC1_TXDSCR_BURST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_txdata_burst_en(&self) -> SLC1_TXDATA_BURST_EN_R {
        SLC1_TXDATA_BURST_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_token_auto_clr(&self) -> SLC1_TOKEN_AUTO_CLR_R {
        SLC1_TOKEN_AUTO_CLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc1_token_sel(&self) -> SLC1_TOKEN_SEL_R {
        SLC1_TOKEN_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("slc0_tx_rst", &format_args!("{}", self.slc0_tx_rst().bit()))
            .field("slc0_rx_rst", &format_args!("{}", self.slc0_rx_rst().bit()))
            .field(
                "ahbm_fifo_rst",
                &format_args!("{}", self.ahbm_fifo_rst().bit()),
            )
            .field("ahbm_rst", &format_args!("{}", self.ahbm_rst().bit()))
            .field(
                "slc0_tx_loop_test",
                &format_args!("{}", self.slc0_tx_loop_test().bit()),
            )
            .field(
                "slc0_rx_loop_test",
                &format_args!("{}", self.slc0_rx_loop_test().bit()),
            )
            .field(
                "slc0_rx_auto_wrback",
                &format_args!("{}", self.slc0_rx_auto_wrback().bit()),
            )
            .field(
                "slc0_rx_no_restart_clr",
                &format_args!("{}", self.slc0_rx_no_restart_clr().bit()),
            )
            .field(
                "slc0_rxdscr_burst_en",
                &format_args!("{}", self.slc0_rxdscr_burst_en().bit()),
            )
            .field(
                "slc0_rxdata_burst_en",
                &format_args!("{}", self.slc0_rxdata_burst_en().bit()),
            )
            .field(
                "slc0_rxlink_auto_ret",
                &format_args!("{}", self.slc0_rxlink_auto_ret().bit()),
            )
            .field(
                "slc0_txlink_auto_ret",
                &format_args!("{}", self.slc0_txlink_auto_ret().bit()),
            )
            .field(
                "slc0_txdscr_burst_en",
                &format_args!("{}", self.slc0_txdscr_burst_en().bit()),
            )
            .field(
                "slc0_txdata_burst_en",
                &format_args!("{}", self.slc0_txdata_burst_en().bit()),
            )
            .field(
                "slc0_token_auto_clr",
                &format_args!("{}", self.slc0_token_auto_clr().bit()),
            )
            .field(
                "slc0_token_sel",
                &format_args!("{}", self.slc0_token_sel().bit()),
            )
            .field("slc1_tx_rst", &format_args!("{}", self.slc1_tx_rst().bit()))
            .field("slc1_rx_rst", &format_args!("{}", self.slc1_rx_rst().bit()))
            .field(
                "slc0_wr_retry_mask_en",
                &format_args!("{}", self.slc0_wr_retry_mask_en().bit()),
            )
            .field(
                "slc1_wr_retry_mask_en",
                &format_args!("{}", self.slc1_wr_retry_mask_en().bit()),
            )
            .field(
                "slc1_tx_loop_test",
                &format_args!("{}", self.slc1_tx_loop_test().bit()),
            )
            .field(
                "slc1_rx_loop_test",
                &format_args!("{}", self.slc1_rx_loop_test().bit()),
            )
            .field(
                "slc1_rx_auto_wrback",
                &format_args!("{}", self.slc1_rx_auto_wrback().bit()),
            )
            .field(
                "slc1_rx_no_restart_clr",
                &format_args!("{}", self.slc1_rx_no_restart_clr().bit()),
            )
            .field(
                "slc1_rxdscr_burst_en",
                &format_args!("{}", self.slc1_rxdscr_burst_en().bit()),
            )
            .field(
                "slc1_rxdata_burst_en",
                &format_args!("{}", self.slc1_rxdata_burst_en().bit()),
            )
            .field(
                "slc1_rxlink_auto_ret",
                &format_args!("{}", self.slc1_rxlink_auto_ret().bit()),
            )
            .field(
                "slc1_txlink_auto_ret",
                &format_args!("{}", self.slc1_txlink_auto_ret().bit()),
            )
            .field(
                "slc1_txdscr_burst_en",
                &format_args!("{}", self.slc1_txdscr_burst_en().bit()),
            )
            .field(
                "slc1_txdata_burst_en",
                &format_args!("{}", self.slc1_txdata_burst_en().bit()),
            )
            .field(
                "slc1_token_auto_clr",
                &format_args!("{}", self.slc1_token_auto_clr().bit()),
            )
            .field(
                "slc1_token_sel",
                &format_args!("{}", self.slc1_token_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_rst(&mut self) -> SLC0_TX_RST_W<0> {
        SLC0_TX_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_rst(&mut self) -> SLC0_RX_RST_W<1> {
        SLC0_RX_RST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<2> {
        AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<3> {
        AHBM_RST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_loop_test(&mut self) -> SLC0_TX_LOOP_TEST_W<4> {
        SLC0_TX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_loop_test(&mut self) -> SLC0_RX_LOOP_TEST_W<5> {
        SLC0_RX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_auto_wrback(&mut self) -> SLC0_RX_AUTO_WRBACK_W<6> {
        SLC0_RX_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_no_restart_clr(&mut self) -> SLC0_RX_NO_RESTART_CLR_W<7> {
        SLC0_RX_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxdscr_burst_en(&mut self) -> SLC0_RXDSCR_BURST_EN_W<8> {
        SLC0_RXDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxdata_burst_en(&mut self) -> SLC0_RXDATA_BURST_EN_W<9> {
        SLC0_RXDATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_auto_ret(&mut self) -> SLC0_RXLINK_AUTO_RET_W<10> {
        SLC0_RXLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txlink_auto_ret(&mut self) -> SLC0_TXLINK_AUTO_RET_W<11> {
        SLC0_TXLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txdscr_burst_en(&mut self) -> SLC0_TXDSCR_BURST_EN_W<12> {
        SLC0_TXDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txdata_burst_en(&mut self) -> SLC0_TXDATA_BURST_EN_W<13> {
        SLC0_TXDATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_auto_clr(&mut self) -> SLC0_TOKEN_AUTO_CLR_W<14> {
        SLC0_TOKEN_AUTO_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_sel(&mut self) -> SLC0_TOKEN_SEL_W<15> {
        SLC0_TOKEN_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_rst(&mut self) -> SLC1_TX_RST_W<16> {
        SLC1_TX_RST_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_rst(&mut self) -> SLC1_RX_RST_W<17> {
        SLC1_RX_RST_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_wr_retry_mask_en(&mut self) -> SLC0_WR_RETRY_MASK_EN_W<18> {
        SLC0_WR_RETRY_MASK_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_wr_retry_mask_en(&mut self) -> SLC1_WR_RETRY_MASK_EN_W<19> {
        SLC1_WR_RETRY_MASK_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_loop_test(&mut self) -> SLC1_TX_LOOP_TEST_W<20> {
        SLC1_TX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_loop_test(&mut self) -> SLC1_RX_LOOP_TEST_W<21> {
        SLC1_RX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_auto_wrback(&mut self) -> SLC1_RX_AUTO_WRBACK_W<22> {
        SLC1_RX_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_no_restart_clr(&mut self) -> SLC1_RX_NO_RESTART_CLR_W<23> {
        SLC1_RX_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxdscr_burst_en(&mut self) -> SLC1_RXDSCR_BURST_EN_W<24> {
        SLC1_RXDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxdata_burst_en(&mut self) -> SLC1_RXDATA_BURST_EN_W<25> {
        SLC1_RXDATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_auto_ret(&mut self) -> SLC1_RXLINK_AUTO_RET_W<26> {
        SLC1_RXLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_auto_ret(&mut self) -> SLC1_TXLINK_AUTO_RET_W<27> {
        SLC1_TXLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txdscr_burst_en(&mut self) -> SLC1_TXDSCR_BURST_EN_W<28> {
        SLC1_TXDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txdata_burst_en(&mut self) -> SLC1_TXDATA_BURST_EN_W<29> {
        SLC1_TXDATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_auto_clr(&mut self) -> SLC1_TOKEN_AUTO_CLR_W<30> {
        SLC1_TOKEN_AUTO_CLR_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_sel(&mut self) -> SLC1_TOKEN_SEL_W<31> {
        SLC1_TOKEN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0xff3c_ff30"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff3c_ff30;
}
