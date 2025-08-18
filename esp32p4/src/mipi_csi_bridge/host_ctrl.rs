#[doc = "Register `HOST_CTRL` reader"]
pub type R = crate::R<HOST_CTRL_SPEC>;
#[doc = "Register `HOST_CTRL` writer"]
pub type W = crate::W<HOST_CTRL_SPEC>;
#[doc = "Field `CSI_ENABLECLK` reader - enable clock lane module of csi phy."]
pub type CSI_ENABLECLK_R = crate::BitReader;
#[doc = "Field `CSI_ENABLECLK` writer - enable clock lane module of csi phy."]
pub type CSI_ENABLECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_CFG_CLK_EN` reader - enable cfg_clk of csi host module."]
pub type CSI_CFG_CLK_EN_R = crate::BitReader;
#[doc = "Field `CSI_CFG_CLK_EN` writer - enable cfg_clk of csi host module."]
pub type CSI_CFG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBK_TEST_EN` reader - for phy test by loopback dsi phy to csi phy."]
pub type LOOPBK_TEST_EN_R = crate::BitReader;
#[doc = "Field `LOOPBK_TEST_EN` writer - for phy test by loopback dsi phy to csi phy."]
pub type LOOPBK_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable clock lane module of csi phy."]
    #[inline(always)]
    pub fn csi_enableclk(&self) -> CSI_ENABLECLK_R {
        CSI_ENABLECLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable cfg_clk of csi host module."]
    #[inline(always)]
    pub fn csi_cfg_clk_en(&self) -> CSI_CFG_CLK_EN_R {
        CSI_CFG_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - for phy test by loopback dsi phy to csi phy."]
    #[inline(always)]
    pub fn loopbk_test_en(&self) -> LOOPBK_TEST_EN_R {
        LOOPBK_TEST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_CTRL")
            .field("csi_enableclk", &self.csi_enableclk())
            .field("csi_cfg_clk_en", &self.csi_cfg_clk_en())
            .field("loopbk_test_en", &self.loopbk_test_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable clock lane module of csi phy."]
    #[inline(always)]
    pub fn csi_enableclk(&mut self) -> CSI_ENABLECLK_W<'_, HOST_CTRL_SPEC> {
        CSI_ENABLECLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - enable cfg_clk of csi host module."]
    #[inline(always)]
    pub fn csi_cfg_clk_en(&mut self) -> CSI_CFG_CLK_EN_W<'_, HOST_CTRL_SPEC> {
        CSI_CFG_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - for phy test by loopback dsi phy to csi phy."]
    #[inline(always)]
    pub fn loopbk_test_en(&mut self) -> LOOPBK_TEST_EN_W<'_, HOST_CTRL_SPEC> {
        LOOPBK_TEST_EN_W::new(self, 2)
    }
}
#[doc = "csi host control by csi bridge.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTRL_SPEC;
impl crate::RegisterSpec for HOST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctrl::R`](R) reader structure"]
impl crate::Readable for HOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl::W`](W) writer structure"]
impl crate::Writable for HOST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CTRL to value 0x03"]
impl crate::Resettable for HOST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
