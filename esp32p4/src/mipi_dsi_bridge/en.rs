#[doc = "Register `EN` reader"]
pub type R = crate::R<EN_SPEC>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EN_SPEC>;
#[doc = "Field `DSI_EN` reader - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DSI_EN_R = crate::BitReader;
#[doc = "Field `DSI_EN` writer - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DSI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_en(&self) -> DSI_EN_R {
        DSI_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EN")
            .field("dsi_en", &self.dsi_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_en(&mut self) -> DSI_EN_W<EN_SPEC> {
        DSI_EN_W::new(self, 0)
    }
}
#[doc = "dsi bridge en register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN_SPEC;
impl crate::RegisterSpec for EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
