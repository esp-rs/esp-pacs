#[doc = "Register `HP_CLK_CTRL` reader"]
pub type R = crate::R<HP_CLK_CTRL_SPEC>;
#[doc = "Register `HP_CLK_CTRL` writer"]
pub type W = crate::W<HP_CLK_CTRL_SPEC>;
#[doc = "Field `HP_XTALX2_80M_CLK_EN` reader - XTALx2 80M Clock Enalbe."]
pub type HP_XTALX2_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_XTALX2_80M_CLK_EN` writer - XTALx2 80M Clock Enalbe."]
pub type HP_XTALX2_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ROOT_CLK_EN` reader - HP SoC Root Clock Enable."]
pub type HP_ROOT_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_ROOT_CLK_EN` writer - HP SoC Root Clock Enable."]
pub type HP_ROOT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_XTAL_32K_CLK_EN` reader - XTAL 32K Clock Enable."]
pub type HP_XTAL_32K_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_XTAL_32K_CLK_EN` writer - XTAL 32K Clock Enable."]
pub type HP_XTAL_32K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_RC_32K_CLK_EN` reader - RC 32K Clock Enable."]
pub type HP_RC_32K_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_RC_32K_CLK_EN` writer - RC 32K Clock Enable."]
pub type HP_RC_32K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SOSC_150K_CLK_EN` reader - SOSC 150K Clock Enable."]
pub type HP_SOSC_150K_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_SOSC_150K_CLK_EN` writer - SOSC 150K Clock Enable."]
pub type HP_SOSC_150K_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_PLL_80M_CLK_EN` reader - PLL 8M Clock Enable."]
pub type HP_PLL_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_PLL_80M_CLK_EN` writer - PLL 8M Clock Enable."]
pub type HP_PLL_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_AUDIO_PLL_CLK_EN` reader - AUDIO PLL Clock Enable."]
pub type HP_AUDIO_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_AUDIO_PLL_CLK_EN` writer - AUDIO PLL Clock Enable."]
pub type HP_AUDIO_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SDIO_PLL2_CLK_EN` reader - reserved"]
pub type HP_SDIO_PLL2_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_SDIO_PLL2_CLK_EN` writer - reserved"]
pub type HP_SDIO_PLL2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SDIO_PLL1_CLK_EN` reader - reserved"]
pub type HP_SDIO_PLL1_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_SDIO_PLL1_CLK_EN` writer - reserved"]
pub type HP_SDIO_PLL1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SDIO_PLL0_CLK_EN` reader - reserved"]
pub type HP_SDIO_PLL0_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_SDIO_PLL0_CLK_EN` writer - reserved"]
pub type HP_SDIO_PLL0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_FOSC_20M_CLK_EN` reader - FOSC 20M Clock Enable."]
pub type HP_FOSC_20M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_FOSC_20M_CLK_EN` writer - FOSC 20M Clock Enable."]
pub type HP_FOSC_20M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_XTAL_40M_CLK_EN` reader - XTAL 40M Clock Enalbe."]
pub type HP_XTAL_40M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_XTAL_40M_CLK_EN` writer - XTAL 40M Clock Enalbe."]
pub type HP_XTAL_40M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPLL_300M_CLK_EN` reader - CPLL 300M Clock Enable."]
pub type HP_CPLL_300M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_CPLL_300M_CLK_EN` writer - CPLL 300M Clock Enable."]
pub type HP_CPLL_300M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SPLL_480M_CLK_EN` reader - SPLL 480M Clock Enable."]
pub type HP_SPLL_480M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_SPLL_480M_CLK_EN` writer - SPLL 480M Clock Enable."]
pub type HP_SPLL_480M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MPLL_500M_CLK_EN` reader - MPLL 500M Clock Enable."]
pub type HP_MPLL_500M_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_MPLL_500M_CLK_EN` writer - MPLL 500M Clock Enable."]
pub type HP_MPLL_500M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - XTALx2 80M Clock Enalbe."]
    #[inline(always)]
    pub fn hp_xtalx2_80m_clk_en(&self) -> HP_XTALX2_80M_CLK_EN_R {
        HP_XTALX2_80M_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HP SoC Root Clock Enable."]
    #[inline(always)]
    pub fn hp_root_clk_en(&self) -> HP_ROOT_CLK_EN_R {
        HP_ROOT_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XTAL 32K Clock Enable."]
    #[inline(always)]
    pub fn hp_xtal_32k_clk_en(&self) -> HP_XTAL_32K_CLK_EN_R {
        HP_XTAL_32K_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RC 32K Clock Enable."]
    #[inline(always)]
    pub fn hp_rc_32k_clk_en(&self) -> HP_RC_32K_CLK_EN_R {
        HP_RC_32K_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SOSC 150K Clock Enable."]
    #[inline(always)]
    pub fn hp_sosc_150k_clk_en(&self) -> HP_SOSC_150K_CLK_EN_R {
        HP_SOSC_150K_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PLL 8M Clock Enable."]
    #[inline(always)]
    pub fn hp_pll_80m_clk_en(&self) -> HP_PLL_80M_CLK_EN_R {
        HP_PLL_80M_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AUDIO PLL Clock Enable."]
    #[inline(always)]
    pub fn hp_audio_pll_clk_en(&self) -> HP_AUDIO_PLL_CLK_EN_R {
        HP_AUDIO_PLL_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll2_clk_en(&self) -> HP_SDIO_PLL2_CLK_EN_R {
        HP_SDIO_PLL2_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll1_clk_en(&self) -> HP_SDIO_PLL1_CLK_EN_R {
        HP_SDIO_PLL1_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll0_clk_en(&self) -> HP_SDIO_PLL0_CLK_EN_R {
        HP_SDIO_PLL0_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FOSC 20M Clock Enable."]
    #[inline(always)]
    pub fn hp_fosc_20m_clk_en(&self) -> HP_FOSC_20M_CLK_EN_R {
        HP_FOSC_20M_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - XTAL 40M Clock Enalbe."]
    #[inline(always)]
    pub fn hp_xtal_40m_clk_en(&self) -> HP_XTAL_40M_CLK_EN_R {
        HP_XTAL_40M_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPLL 300M Clock Enable."]
    #[inline(always)]
    pub fn hp_cpll_300m_clk_en(&self) -> HP_CPLL_300M_CLK_EN_R {
        HP_CPLL_300M_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SPLL 480M Clock Enable."]
    #[inline(always)]
    pub fn hp_spll_480m_clk_en(&self) -> HP_SPLL_480M_CLK_EN_R {
        HP_SPLL_480M_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MPLL 500M Clock Enable."]
    #[inline(always)]
    pub fn hp_mpll_500m_clk_en(&self) -> HP_MPLL_500M_CLK_EN_R {
        HP_MPLL_500M_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CLK_CTRL")
            .field("hp_xtalx2_80m_clk_en", &self.hp_xtalx2_80m_clk_en())
            .field("hp_root_clk_en", &self.hp_root_clk_en())
            .field("hp_xtal_32k_clk_en", &self.hp_xtal_32k_clk_en())
            .field("hp_rc_32k_clk_en", &self.hp_rc_32k_clk_en())
            .field("hp_sosc_150k_clk_en", &self.hp_sosc_150k_clk_en())
            .field("hp_pll_80m_clk_en", &self.hp_pll_80m_clk_en())
            .field("hp_audio_pll_clk_en", &self.hp_audio_pll_clk_en())
            .field("hp_sdio_pll2_clk_en", &self.hp_sdio_pll2_clk_en())
            .field("hp_sdio_pll1_clk_en", &self.hp_sdio_pll1_clk_en())
            .field("hp_sdio_pll0_clk_en", &self.hp_sdio_pll0_clk_en())
            .field("hp_fosc_20m_clk_en", &self.hp_fosc_20m_clk_en())
            .field("hp_xtal_40m_clk_en", &self.hp_xtal_40m_clk_en())
            .field("hp_cpll_300m_clk_en", &self.hp_cpll_300m_clk_en())
            .field("hp_spll_480m_clk_en", &self.hp_spll_480m_clk_en())
            .field("hp_mpll_500m_clk_en", &self.hp_mpll_500m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17 - XTALx2 80M Clock Enalbe."]
    #[inline(always)]
    pub fn hp_xtalx2_80m_clk_en(&mut self) -> HP_XTALX2_80M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_XTALX2_80M_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - HP SoC Root Clock Enable."]
    #[inline(always)]
    pub fn hp_root_clk_en(&mut self) -> HP_ROOT_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_ROOT_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - XTAL 32K Clock Enable."]
    #[inline(always)]
    pub fn hp_xtal_32k_clk_en(&mut self) -> HP_XTAL_32K_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_XTAL_32K_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - RC 32K Clock Enable."]
    #[inline(always)]
    pub fn hp_rc_32k_clk_en(&mut self) -> HP_RC_32K_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_RC_32K_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SOSC 150K Clock Enable."]
    #[inline(always)]
    pub fn hp_sosc_150k_clk_en(&mut self) -> HP_SOSC_150K_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_SOSC_150K_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - PLL 8M Clock Enable."]
    #[inline(always)]
    pub fn hp_pll_80m_clk_en(&mut self) -> HP_PLL_80M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_PLL_80M_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - AUDIO PLL Clock Enable."]
    #[inline(always)]
    pub fn hp_audio_pll_clk_en(&mut self) -> HP_AUDIO_PLL_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_AUDIO_PLL_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll2_clk_en(&mut self) -> HP_SDIO_PLL2_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_SDIO_PLL2_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll1_clk_en(&mut self) -> HP_SDIO_PLL1_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_SDIO_PLL1_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn hp_sdio_pll0_clk_en(&mut self) -> HP_SDIO_PLL0_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_SDIO_PLL0_CLK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - FOSC 20M Clock Enable."]
    #[inline(always)]
    pub fn hp_fosc_20m_clk_en(&mut self) -> HP_FOSC_20M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_FOSC_20M_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - XTAL 40M Clock Enalbe."]
    #[inline(always)]
    pub fn hp_xtal_40m_clk_en(&mut self) -> HP_XTAL_40M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_XTAL_40M_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - CPLL 300M Clock Enable."]
    #[inline(always)]
    pub fn hp_cpll_300m_clk_en(&mut self) -> HP_CPLL_300M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_CPLL_300M_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - SPLL 480M Clock Enable."]
    #[inline(always)]
    pub fn hp_spll_480m_clk_en(&mut self) -> HP_SPLL_480M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_SPLL_480M_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - MPLL 500M Clock Enable."]
    #[inline(always)]
    pub fn hp_mpll_500m_clk_en(&mut self) -> HP_MPLL_500M_CLK_EN_W<'_, HP_CLK_CTRL_SPEC> {
        HP_MPLL_500M_CLK_EN_W::new(self, 31)
    }
}
#[doc = "HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CLK_CTRL_SPEC;
impl crate::RegisterSpec for HP_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CLK_CTRL to value 0xfffe_0000"]
impl crate::Resettable for HP_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xfffe_0000;
}
