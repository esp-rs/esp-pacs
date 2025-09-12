#[doc = "Register `PERI_CLK_CTRL03` reader"]
pub type R = crate::R<PERI_CLK_CTRL03_SPEC>;
#[doc = "Register `PERI_CLK_CTRL03` writer"]
pub type W = crate::W<PERI_CLK_CTRL03_SPEC>;
#[doc = "Field `MIPI_DSI_DPHY_CFG_CLK_EN` reader - Reserved"]
pub type MIPI_DSI_DPHY_CFG_CLK_EN_R = crate::BitReader;
#[doc = "Field `MIPI_DSI_DPHY_CFG_CLK_EN` writer - Reserved"]
pub type MIPI_DSI_DPHY_CFG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPHY_PLL_REFCLK_EN` reader - Reserved"]
pub type MIPI_DSI_DPHY_PLL_REFCLK_EN_R = crate::BitReader;
#[doc = "Field `MIPI_DSI_DPHY_PLL_REFCLK_EN` writer - Reserved"]
pub type MIPI_DSI_DPHY_PLL_REFCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_CSI_DPHY_CLK_SRC_SEL` reader - Reserved"]
pub type MIPI_CSI_DPHY_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `MIPI_CSI_DPHY_CLK_SRC_SEL` writer - Reserved"]
pub type MIPI_CSI_DPHY_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MIPI_CSI_DPHY_CFG_CLK_EN` reader - Reserved"]
pub type MIPI_CSI_DPHY_CFG_CLK_EN_R = crate::BitReader;
#[doc = "Field `MIPI_CSI_DPHY_CFG_CLK_EN` writer - Reserved"]
pub type MIPI_CSI_DPHY_CFG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPICLK_SRC_SEL` reader - Reserved"]
pub type MIPI_DSI_DPICLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPICLK_SRC_SEL` writer - Reserved"]
pub type MIPI_DSI_DPICLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MIPI_DSI_DPICLK_EN` reader - Reserved"]
pub type MIPI_DSI_DPICLK_EN_R = crate::BitReader;
#[doc = "Field `MIPI_DSI_DPICLK_EN` writer - Reserved"]
pub type MIPI_DSI_DPICLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPICLK_DIV_NUM` reader - Reserved"]
pub type MIPI_DSI_DPICLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPICLK_DIV_NUM` writer - Reserved"]
pub type MIPI_DSI_DPICLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_cfg_clk_en(&self) -> MIPI_DSI_DPHY_CFG_CLK_EN_R {
        MIPI_DSI_DPHY_CFG_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_pll_refclk_en(&self) -> MIPI_DSI_DPHY_PLL_REFCLK_EN_R {
        MIPI_DSI_DPHY_PLL_REFCLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn mipi_csi_dphy_clk_src_sel(&self) -> MIPI_CSI_DPHY_CLK_SRC_SEL_R {
        MIPI_CSI_DPHY_CLK_SRC_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn mipi_csi_dphy_cfg_clk_en(&self) -> MIPI_CSI_DPHY_CFG_CLK_EN_R {
        MIPI_CSI_DPHY_CFG_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_src_sel(&self) -> MIPI_DSI_DPICLK_SRC_SEL_R {
        MIPI_DSI_DPICLK_SRC_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_en(&self) -> MIPI_DSI_DPICLK_EN_R {
        MIPI_DSI_DPICLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_div_num(&self) -> MIPI_DSI_DPICLK_DIV_NUM_R {
        MIPI_DSI_DPICLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL03")
            .field("mipi_dsi_dphy_cfg_clk_en", &self.mipi_dsi_dphy_cfg_clk_en())
            .field(
                "mipi_dsi_dphy_pll_refclk_en",
                &self.mipi_dsi_dphy_pll_refclk_en(),
            )
            .field(
                "mipi_csi_dphy_clk_src_sel",
                &self.mipi_csi_dphy_clk_src_sel(),
            )
            .field("mipi_csi_dphy_cfg_clk_en", &self.mipi_csi_dphy_cfg_clk_en())
            .field("mipi_dsi_dpiclk_src_sel", &self.mipi_dsi_dpiclk_src_sel())
            .field("mipi_dsi_dpiclk_en", &self.mipi_dsi_dpiclk_en())
            .field("mipi_dsi_dpiclk_div_num", &self.mipi_dsi_dpiclk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_cfg_clk_en(
        &mut self,
    ) -> MIPI_DSI_DPHY_CFG_CLK_EN_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_DSI_DPHY_CFG_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_pll_refclk_en(
        &mut self,
    ) -> MIPI_DSI_DPHY_PLL_REFCLK_EN_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_DSI_DPHY_PLL_REFCLK_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn mipi_csi_dphy_clk_src_sel(
        &mut self,
    ) -> MIPI_CSI_DPHY_CLK_SRC_SEL_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_CSI_DPHY_CLK_SRC_SEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn mipi_csi_dphy_cfg_clk_en(
        &mut self,
    ) -> MIPI_CSI_DPHY_CFG_CLK_EN_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_CSI_DPHY_CFG_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_src_sel(
        &mut self,
    ) -> MIPI_DSI_DPICLK_SRC_SEL_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_DSI_DPICLK_SRC_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_en(&mut self) -> MIPI_DSI_DPICLK_EN_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_DSI_DPICLK_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_div_num(
        &mut self,
    ) -> MIPI_DSI_DPICLK_DIV_NUM_W<'_, PERI_CLK_CTRL03_SPEC> {
        MIPI_DSI_DPICLK_DIV_NUM_W::new(self, 8)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL03_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL03_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl03::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL03_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl03::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL03_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL03 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL03_SPEC {}
