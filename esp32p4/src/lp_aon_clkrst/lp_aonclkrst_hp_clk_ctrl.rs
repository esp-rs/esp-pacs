#[doc = "Register `LP_AONCLKRST_HP_CLK_CTRL` reader"]
pub type R = crate::R<LP_AONCLKRST_HP_CLK_CTRL_SPEC>;
#[doc = "Register `LP_AONCLKRST_HP_CLK_CTRL` writer"]
pub type W = crate::W<LP_AONCLKRST_HP_CLK_CTRL_SPEC>;
#[doc = "Field `LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL` reader - HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m."]
pub type LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL` writer - HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m."]
pub type LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_AONCLKRST_HP_ROOT_CLK_EN` reader - HP SoC Root Clock Enable."]
pub type LP_AONCLKRST_HP_ROOT_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_ROOT_CLK_EN` writer - HP SoC Root Clock Enable."]
pub type LP_AONCLKRST_HP_ROOT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN` reader - PARLIO TX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN` writer - PARLIO TX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN` reader - PARLIO RX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN` writer - PARLIO RX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN` reader - UART4 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN` writer - UART4 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN` reader - UART3 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN` writer - UART3 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN` reader - UART2 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN` writer - UART2 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN` reader - UART1 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN` writer - UART1 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN` reader - UART0 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN` writer - UART0 SLP Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN` reader - I2S2 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN` writer - I2S2 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN` reader - I2S1 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN` writer - I2S1 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN` reader - I2S0 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN` writer - I2S0 MCLK Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN` reader - EMAC RX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN` writer - EMAC RX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN` reader - EMAC TX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN` writer - EMAC TX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN` reader - EMAC TXRX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN` writer - EMAC TXRX Clock From Pad Enable."]
pub type LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_XTAL_32K_CLK_EN` reader - XTAL 32K Clock Enable."]
pub type LP_AONCLKRST_HP_XTAL_32K_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_XTAL_32K_CLK_EN` writer - XTAL 32K Clock Enable."]
pub type LP_AONCLKRST_HP_XTAL_32K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_RC_32K_CLK_EN` reader - RC 32K Clock Enable."]
pub type LP_AONCLKRST_HP_RC_32K_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_RC_32K_CLK_EN` writer - RC 32K Clock Enable."]
pub type LP_AONCLKRST_HP_RC_32K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_SOSC_150K_CLK_EN` reader - SOSC 150K Clock Enable."]
pub type LP_AONCLKRST_HP_SOSC_150K_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_SOSC_150K_CLK_EN` writer - SOSC 150K Clock Enable."]
pub type LP_AONCLKRST_HP_SOSC_150K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_PLL_8M_CLK_EN` reader - PLL 8M Clock Enable."]
pub type LP_AONCLKRST_HP_PLL_8M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_PLL_8M_CLK_EN` writer - PLL 8M Clock Enable."]
pub type LP_AONCLKRST_HP_PLL_8M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN` reader - AUDIO PLL Clock Enable."]
pub type LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN` writer - AUDIO PLL Clock Enable."]
pub type LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN` reader - SDIO PLL2 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN` writer - SDIO PLL2 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN` reader - SDIO PLL1 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN` writer - SDIO PLL1 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN` reader - SDIO PLL0 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN` writer - SDIO PLL0 Clock Enable."]
pub type LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_FOSC_20M_CLK_EN` reader - FOSC 20M Clock Enable."]
pub type LP_AONCLKRST_HP_FOSC_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_FOSC_20M_CLK_EN` writer - FOSC 20M Clock Enable."]
pub type LP_AONCLKRST_HP_FOSC_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_XTAL_40M_CLK_EN` reader - XTAL 40M Clock Enalbe."]
pub type LP_AONCLKRST_HP_XTAL_40M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_XTAL_40M_CLK_EN` writer - XTAL 40M Clock Enalbe."]
pub type LP_AONCLKRST_HP_XTAL_40M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_CPLL_400M_CLK_EN` reader - CPLL 400M Clock Enable."]
pub type LP_AONCLKRST_HP_CPLL_400M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_CPLL_400M_CLK_EN` writer - CPLL 400M Clock Enable."]
pub type LP_AONCLKRST_HP_CPLL_400M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_SPLL_480M_CLK_EN` reader - SPLL 480M Clock Enable."]
pub type LP_AONCLKRST_HP_SPLL_480M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_SPLL_480M_CLK_EN` writer - SPLL 480M Clock Enable."]
pub type LP_AONCLKRST_HP_SPLL_480M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HP_MPLL_500M_CLK_EN` reader - MPLL 500M Clock Enable."]
pub type LP_AONCLKRST_HP_MPLL_500M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HP_MPLL_500M_CLK_EN` writer - MPLL 500M Clock Enable."]
pub type LP_AONCLKRST_HP_MPLL_500M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_root_clk_src_sel(&self) -> LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_R {
        LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - HP SoC Root Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_root_clk_en(&self) -> LP_AONCLKRST_HP_ROOT_CLK_EN_R {
        LP_AONCLKRST_HP_ROOT_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PARLIO TX Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_parlio_tx_clk_en(&self) -> LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PARLIO RX Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_parlio_rx_clk_en(&self) -> LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART4 SLP Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart4_slp_clk_en(&self) -> LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART3 SLP Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart3_slp_clk_en(&self) -> LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART2 SLP Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart2_slp_clk_en(&self) -> LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART1 SLP Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart1_slp_clk_en(&self) -> LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART0 SLP Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart0_slp_clk_en(&self) -> LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S2 MCLK Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s2_mclk_en(&self) -> LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_R {
        LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S1 MCLK Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s1_mclk_en(&self) -> LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_R {
        LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2S0 MCLK Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s0_mclk_en(&self) -> LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_R {
        LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EMAC RX Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_tx_clk_en(&self) -> LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EMAC TX Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_rx_clk_en(&self) -> LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EMAC TXRX Clock From Pad Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_txrx_clk_en(&self) -> LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_R {
        LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XTAL 32K Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_xtal_32k_clk_en(&self) -> LP_AONCLKRST_HP_XTAL_32K_CLK_EN_R {
        LP_AONCLKRST_HP_XTAL_32K_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RC 32K Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_rc_32k_clk_en(&self) -> LP_AONCLKRST_HP_RC_32K_CLK_EN_R {
        LP_AONCLKRST_HP_RC_32K_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SOSC 150K Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sosc_150k_clk_en(&self) -> LP_AONCLKRST_HP_SOSC_150K_CLK_EN_R {
        LP_AONCLKRST_HP_SOSC_150K_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL 8M Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pll_8m_clk_en(&self) -> LP_AONCLKRST_HP_PLL_8M_CLK_EN_R {
        LP_AONCLKRST_HP_PLL_8M_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AUDIO PLL Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_audio_pll_clk_en(&self) -> LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_R {
        LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDIO PLL2 Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll2_clk_en(&self) -> LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_R {
        LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO PLL1 Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll1_clk_en(&self) -> LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_R {
        LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDIO PLL0 Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll0_clk_en(&self) -> LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_R {
        LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FOSC 20M Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_fosc_20m_clk_en(&self) -> LP_AONCLKRST_HP_FOSC_20M_CLK_EN_R {
        LP_AONCLKRST_HP_FOSC_20M_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - XTAL 40M Clock Enalbe."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_xtal_40m_clk_en(&self) -> LP_AONCLKRST_HP_XTAL_40M_CLK_EN_R {
        LP_AONCLKRST_HP_XTAL_40M_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPLL 400M Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_cpll_400m_clk_en(&self) -> LP_AONCLKRST_HP_CPLL_400M_CLK_EN_R {
        LP_AONCLKRST_HP_CPLL_400M_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SPLL 480M Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_spll_480m_clk_en(&self) -> LP_AONCLKRST_HP_SPLL_480M_CLK_EN_R {
        LP_AONCLKRST_HP_SPLL_480M_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MPLL 500M Clock Enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_hp_mpll_500m_clk_en(&self) -> LP_AONCLKRST_HP_MPLL_500M_CLK_EN_R {
        LP_AONCLKRST_HP_MPLL_500M_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HP_CLK_CTRL")
            .field(
                "lp_aonclkrst_hp_root_clk_src_sel",
                &self.lp_aonclkrst_hp_root_clk_src_sel(),
            )
            .field(
                "lp_aonclkrst_hp_root_clk_en",
                &self.lp_aonclkrst_hp_root_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_parlio_tx_clk_en",
                &self.lp_aonclkrst_hp_pad_parlio_tx_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_parlio_rx_clk_en",
                &self.lp_aonclkrst_hp_pad_parlio_rx_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_uart4_slp_clk_en",
                &self.lp_aonclkrst_hp_pad_uart4_slp_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_uart3_slp_clk_en",
                &self.lp_aonclkrst_hp_pad_uart3_slp_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_uart2_slp_clk_en",
                &self.lp_aonclkrst_hp_pad_uart2_slp_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_uart1_slp_clk_en",
                &self.lp_aonclkrst_hp_pad_uart1_slp_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_uart0_slp_clk_en",
                &self.lp_aonclkrst_hp_pad_uart0_slp_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_i2s2_mclk_en",
                &self.lp_aonclkrst_hp_pad_i2s2_mclk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_i2s1_mclk_en",
                &self.lp_aonclkrst_hp_pad_i2s1_mclk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_i2s0_mclk_en",
                &self.lp_aonclkrst_hp_pad_i2s0_mclk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_emac_tx_clk_en",
                &self.lp_aonclkrst_hp_pad_emac_tx_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_emac_rx_clk_en",
                &self.lp_aonclkrst_hp_pad_emac_rx_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pad_emac_txrx_clk_en",
                &self.lp_aonclkrst_hp_pad_emac_txrx_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_xtal_32k_clk_en",
                &self.lp_aonclkrst_hp_xtal_32k_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_rc_32k_clk_en",
                &self.lp_aonclkrst_hp_rc_32k_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_sosc_150k_clk_en",
                &self.lp_aonclkrst_hp_sosc_150k_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_pll_8m_clk_en",
                &self.lp_aonclkrst_hp_pll_8m_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_audio_pll_clk_en",
                &self.lp_aonclkrst_hp_audio_pll_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_sdio_pll2_clk_en",
                &self.lp_aonclkrst_hp_sdio_pll2_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_sdio_pll1_clk_en",
                &self.lp_aonclkrst_hp_sdio_pll1_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_sdio_pll0_clk_en",
                &self.lp_aonclkrst_hp_sdio_pll0_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_fosc_20m_clk_en",
                &self.lp_aonclkrst_hp_fosc_20m_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_xtal_40m_clk_en",
                &self.lp_aonclkrst_hp_xtal_40m_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_cpll_400m_clk_en",
                &self.lp_aonclkrst_hp_cpll_400m_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_spll_480m_clk_en",
                &self.lp_aonclkrst_hp_spll_480m_clk_en(),
            )
            .field(
                "lp_aonclkrst_hp_mpll_500m_clk_en",
                &self.lp_aonclkrst_hp_mpll_500m_clk_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_root_clk_src_sel(
        &mut self,
    ) -> LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - HP SoC Root Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_root_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_ROOT_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_ROOT_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - PARLIO TX Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_parlio_tx_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - PARLIO RX Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_parlio_rx_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - UART4 SLP Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_uart4_slp_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - UART3 SLP Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_uart3_slp_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - UART2 SLP Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_uart2_slp_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - UART1 SLP Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_uart1_slp_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - UART0 SLP Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_uart0_slp_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - I2S2 MCLK Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_i2s2_mclk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S1 MCLK Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_i2s1_mclk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - I2S0 MCLK Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_i2s0_mclk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - EMAC RX Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_emac_tx_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - EMAC TX Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_emac_rx_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - EMAC TXRX Clock From Pad Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pad_emac_txrx_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - XTAL 32K Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_xtal_32k_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_XTAL_32K_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_XTAL_32K_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - RC 32K Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_rc_32k_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_RC_32K_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_RC_32K_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - SOSC 150K Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_sosc_150k_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_SOSC_150K_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_SOSC_150K_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - PLL 8M Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_pll_8m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_PLL_8M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_PLL_8M_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - AUDIO PLL Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_audio_pll_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SDIO PLL2 Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_sdio_pll2_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIO PLL1 Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_sdio_pll1_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SDIO PLL0 Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_sdio_pll0_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - FOSC 20M Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_fosc_20m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_FOSC_20M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_FOSC_20M_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - XTAL 40M Clock Enalbe."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_xtal_40m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_XTAL_40M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_XTAL_40M_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - CPLL 400M Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_cpll_400m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_CPLL_400M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_CPLL_400M_CLK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - SPLL 480M Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_spll_480m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_SPLL_480M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_SPLL_480M_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - MPLL 500M Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_hp_mpll_500m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_HP_MPLL_500M_CLK_EN_W<LP_AONCLKRST_HP_CLK_CTRL_SPEC> {
        LP_AONCLKRST_HP_MPLL_500M_CLK_EN_W::new(self, 28)
    }
}
#[doc = "HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HP_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HP_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HP_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_CLK_CTRL to value 0x1fff_fffc"]
impl crate::Resettable for LP_AONCLKRST_HP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1fff_fffc;
}
