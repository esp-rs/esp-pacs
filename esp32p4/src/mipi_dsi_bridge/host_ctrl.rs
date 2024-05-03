#[doc = "Register `HOST_CTRL` reader"]
pub type R = crate::R<HOST_CTRL_SPEC>;
#[doc = "Register `HOST_CTRL` writer"]
pub type W = crate::W<HOST_CTRL_SPEC>;
#[doc = "Field `DSI_CFG_REF_CLK_EN` reader - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
pub type DSI_CFG_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `DSI_CFG_REF_CLK_EN` writer - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
pub type DSI_CFG_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_cfg_ref_clk_en(&self) -> DSI_CFG_REF_CLK_EN_R {
        DSI_CFG_REF_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_CTRL")
            .field("dsi_cfg_ref_clk_en", &self.dsi_cfg_ref_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_cfg_ref_clk_en(&mut self) -> DSI_CFG_REF_CLK_EN_W<HOST_CTRL_SPEC> {
        DSI_CFG_REF_CLK_EN_W::new(self, 0)
    }
}
#[doc = "dsi_bridge host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTRL_SPEC;
impl crate::RegisterSpec for HOST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctrl::R`](R) reader structure"]
impl crate::Readable for HOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl::W`](W) writer structure"]
impl crate::Writable for HOST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0x01"]
impl crate::Resettable for HOST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
