#[doc = "Register `SLC_CONF0` reader"]
pub struct R(crate::R<SLC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_CONF0` writer"]
pub struct W(crate::W<SLC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_CONF0_SPEC>;
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
impl From<crate::W<SLC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TXLINK_RST` reader - "]
pub type SLC_TXLINK_RST_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_RST` writer - "]
pub type SLC_TXLINK_RST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_RXLINK_RST` reader - "]
pub type SLC_RXLINK_RST_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_RST` writer - "]
pub type SLC_RXLINK_RST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_AHBM_FIFO_RST` reader - "]
pub type SLC_AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `SLC_AHBM_FIFO_RST` writer - "]
pub type SLC_AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_AHBM_RST` reader - "]
pub type SLC_AHBM_RST_R = crate::BitReader;
#[doc = "Field `SLC_AHBM_RST` writer - "]
pub type SLC_AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_TX_LOOP_TEST` reader - "]
pub type SLC_TX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC_TX_LOOP_TEST` writer - "]
pub type SLC_TX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_RX_LOOP_TEST` reader - "]
pub type SLC_RX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC_RX_LOOP_TEST` writer - "]
pub type SLC_RX_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_RX_AUTO_WRBACK` reader - "]
pub type SLC_RX_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `SLC_RX_AUTO_WRBACK` writer - "]
pub type SLC_RX_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_RX_NO_RESTART_CLR` reader - "]
pub type SLC_RX_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `SLC_RX_NO_RESTART_CLR` writer - "]
pub type SLC_RX_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_DSCR_BURST_EN` reader - "]
pub type SLC_DSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_DSCR_BURST_EN` writer - "]
pub type SLC_DSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_DATA_BURST_EN` reader - "]
pub type SLC_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_DATA_BURST_EN` writer - "]
pub type SLC_DATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLC_CONF0_SPEC, O>;
#[doc = "Field `SLC_MODE` reader - "]
pub type SLC_MODE_R = crate::FieldReader;
#[doc = "Field `SLC_MODE` writer - "]
pub type SLC_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_CONF0_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_txlink_rst(&self) -> SLC_TXLINK_RST_R {
        SLC_TXLINK_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_rxlink_rst(&self) -> SLC_RXLINK_RST_R {
        SLC_RXLINK_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc_ahbm_fifo_rst(&self) -> SLC_AHBM_FIFO_RST_R {
        SLC_AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc_ahbm_rst(&self) -> SLC_AHBM_RST_R {
        SLC_AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc_tx_loop_test(&self) -> SLC_TX_LOOP_TEST_R {
        SLC_TX_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc_rx_loop_test(&self) -> SLC_RX_LOOP_TEST_R {
        SLC_RX_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc_rx_auto_wrback(&self) -> SLC_RX_AUTO_WRBACK_R {
        SLC_RX_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc_rx_no_restart_clr(&self) -> SLC_RX_NO_RESTART_CLR_R {
        SLC_RX_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_dscr_burst_en(&self) -> SLC_DSCR_BURST_EN_R {
        SLC_DSCR_BURST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_data_burst_en(&self) -> SLC_DATA_BURST_EN_R {
        SLC_DATA_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn slc_mode(&self) -> SLC_MODE_R {
        SLC_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_CONF0")
            .field("slc_mode", &format_args!("{}", self.slc_mode().bits()))
            .field(
                "slc_data_burst_en",
                &format_args!("{}", self.slc_data_burst_en().bit()),
            )
            .field(
                "slc_dscr_burst_en",
                &format_args!("{}", self.slc_dscr_burst_en().bit()),
            )
            .field(
                "slc_rx_no_restart_clr",
                &format_args!("{}", self.slc_rx_no_restart_clr().bit()),
            )
            .field(
                "slc_rx_auto_wrback",
                &format_args!("{}", self.slc_rx_auto_wrback().bit()),
            )
            .field(
                "slc_rx_loop_test",
                &format_args!("{}", self.slc_rx_loop_test().bit()),
            )
            .field(
                "slc_tx_loop_test",
                &format_args!("{}", self.slc_tx_loop_test().bit()),
            )
            .field(
                "slc_ahbm_rst",
                &format_args!("{}", self.slc_ahbm_rst().bit()),
            )
            .field(
                "slc_ahbm_fifo_rst",
                &format_args!("{}", self.slc_ahbm_fifo_rst().bit()),
            )
            .field(
                "slc_rxlink_rst",
                &format_args!("{}", self.slc_rxlink_rst().bit()),
            )
            .field(
                "slc_txlink_rst",
                &format_args!("{}", self.slc_txlink_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_rst(&mut self) -> SLC_TXLINK_RST_W<0> {
        SLC_TXLINK_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_rst(&mut self) -> SLC_RXLINK_RST_W<1> {
        SLC_RXLINK_RST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahbm_fifo_rst(&mut self) -> SLC_AHBM_FIFO_RST_W<2> {
        SLC_AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahbm_rst(&mut self) -> SLC_AHBM_RST_W<3> {
        SLC_AHBM_RST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_loop_test(&mut self) -> SLC_TX_LOOP_TEST_W<4> {
        SLC_TX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_loop_test(&mut self) -> SLC_RX_LOOP_TEST_W<5> {
        SLC_RX_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_auto_wrback(&mut self) -> SLC_RX_AUTO_WRBACK_W<6> {
        SLC_RX_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_no_restart_clr(&mut self) -> SLC_RX_NO_RESTART_CLR_W<7> {
        SLC_RX_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc_dscr_burst_en(&mut self) -> SLC_DSCR_BURST_EN_W<8> {
        SLC_DSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc_data_burst_en(&mut self) -> SLC_DATA_BURST_EN_W<9> {
        SLC_DATA_BURST_EN_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn slc_mode(&mut self) -> SLC_MODE_W<12> {
        SLC_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_conf0](index.html) module"]
pub struct SLC_CONF0_SPEC;
impl crate::RegisterSpec for SLC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_conf0::R](R) reader structure"]
impl crate::Readable for SLC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_conf0::W](W) writer structure"]
impl crate::Writable for SLC_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_CONF0 to value 0"]
impl crate::Resettable for SLC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
