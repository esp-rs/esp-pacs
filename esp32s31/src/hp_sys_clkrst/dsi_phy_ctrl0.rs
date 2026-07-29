#[doc = "Register `DSI_PHY_CTRL0` reader"]
pub type R = crate::R<DSI_PHY_CTRL0_SPEC>;
#[doc = "Register `DSI_PHY_CTRL0` writer"]
pub type W = crate::W<DSI_PHY_CTRL0_SPEC>;
#[doc = "Field `DPHY_CLK_SRC_SEL` reader - need_des"]
pub type DPHY_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `DPHY_CLK_SRC_SEL` writer - need_des"]
pub type DPHY_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DPHY_CFG_CLK_EN` reader - need_des"]
pub type DPHY_CFG_CLK_EN_R = crate::BitReader;
#[doc = "Field `DPHY_CFG_CLK_EN` writer - need_des"]
pub type DPHY_CFG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_PLL_REFCLK_EN` reader - need_des"]
pub type DPHY_PLL_REFCLK_EN_R = crate::BitReader;
#[doc = "Field `DPHY_PLL_REFCLK_EN` writer - need_des"]
pub type DPHY_PLL_REFCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn dphy_clk_src_sel(&self) -> DPHY_CLK_SRC_SEL_R {
        DPHY_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn dphy_cfg_clk_en(&self) -> DPHY_CFG_CLK_EN_R {
        DPHY_CFG_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn dphy_pll_refclk_en(&self) -> DPHY_PLL_REFCLK_EN_R {
        DPHY_PLL_REFCLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_PHY_CTRL0")
            .field("dphy_clk_src_sel", &self.dphy_clk_src_sel())
            .field("dphy_cfg_clk_en", &self.dphy_cfg_clk_en())
            .field("dphy_pll_refclk_en", &self.dphy_pll_refclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn dphy_clk_src_sel(&mut self) -> DPHY_CLK_SRC_SEL_W<'_, DSI_PHY_CTRL0_SPEC> {
        DPHY_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn dphy_cfg_clk_en(&mut self) -> DPHY_CFG_CLK_EN_W<'_, DSI_PHY_CTRL0_SPEC> {
        DPHY_CFG_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn dphy_pll_refclk_en(&mut self) -> DPHY_PLL_REFCLK_EN_W<'_, DSI_PHY_CTRL0_SPEC> {
        DPHY_PLL_REFCLK_EN_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_phy_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_phy_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PHY_CTRL0_SPEC;
impl crate::RegisterSpec for DSI_PHY_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_phy_ctrl0::R`](R) reader structure"]
impl crate::Readable for DSI_PHY_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_phy_ctrl0::W`](W) writer structure"]
impl crate::Writable for DSI_PHY_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PHY_CTRL0 to value 0"]
impl crate::Resettable for DSI_PHY_CTRL0_SPEC {}
