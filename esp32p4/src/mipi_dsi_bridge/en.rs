#[doc = "Register `EN` reader"]
pub type R = crate::R<EN_SPEC>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EN_SPEC>;
#[doc = "Field `DSI_EN` reader - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DSI_EN_R = crate::BitReader;
#[doc = "Field `DSI_EN` writer - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DSI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_BRIG_RST` reader - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
pub type DSI_BRIG_RST_R = crate::BitReader;
#[doc = "Field `DSI_BRIG_RST` writer - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
pub type DSI_BRIG_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_en(&self) -> DSI_EN_R {
        DSI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
    #[inline(always)]
    pub fn dsi_brig_rst(&self) -> DSI_BRIG_RST_R {
        DSI_BRIG_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EN")
            .field("dsi_en", &self.dsi_en())
            .field("dsi_brig_rst", &self.dsi_brig_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_en(&mut self) -> DSI_EN_W<'_, EN_SPEC> {
        DSI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
    #[inline(always)]
    pub fn dsi_brig_rst(&mut self) -> DSI_BRIG_RST_W<'_, EN_SPEC> {
        DSI_BRIG_RST_W::new(self, 1)
    }
}
#[doc = "dsi bridge en register\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN_SPEC;
impl crate::RegisterSpec for EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EN_SPEC {}
