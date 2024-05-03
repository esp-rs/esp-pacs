#[doc = "Register `APB_SYNC_POSTW_EN` reader"]
pub type R = crate::R<APB_SYNC_POSTW_EN_SPEC>;
#[doc = "Register `APB_SYNC_POSTW_EN` writer"]
pub type W = crate::W<APB_SYNC_POSTW_EN_SPEC>;
#[doc = "Field `GMAC_APB_POSTW_EN` reader - N/A"]
pub type GMAC_APB_POSTW_EN_R = crate::BitReader;
#[doc = "Field `GMAC_APB_POSTW_EN` writer - N/A"]
pub type GMAC_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_HOST_APB_POSTW_EN` reader - N/A"]
pub type DSI_HOST_APB_POSTW_EN_R = crate::BitReader;
#[doc = "Field `DSI_HOST_APB_POSTW_EN` writer - N/A"]
pub type DSI_HOST_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_APB_SYNC_POSTW_EN` reader - N/A"]
pub type CSI_HOST_APB_SYNC_POSTW_EN_R = crate::BitReader;
#[doc = "Field `CSI_HOST_APB_SYNC_POSTW_EN` writer - N/A"]
pub type CSI_HOST_APB_SYNC_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_APB_ASYNC_POSTW_EN` reader - N/A"]
pub type CSI_HOST_APB_ASYNC_POSTW_EN_R = crate::BitReader;
#[doc = "Field `CSI_HOST_APB_ASYNC_POSTW_EN` writer - N/A"]
pub type CSI_HOST_APB_ASYNC_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn gmac_apb_postw_en(&self) -> GMAC_APB_POSTW_EN_R {
        GMAC_APB_POSTW_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn dsi_host_apb_postw_en(&self) -> DSI_HOST_APB_POSTW_EN_R {
        DSI_HOST_APB_POSTW_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn csi_host_apb_sync_postw_en(&self) -> CSI_HOST_APB_SYNC_POSTW_EN_R {
        CSI_HOST_APB_SYNC_POSTW_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn csi_host_apb_async_postw_en(&self) -> CSI_HOST_APB_ASYNC_POSTW_EN_R {
        CSI_HOST_APB_ASYNC_POSTW_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SYNC_POSTW_EN")
            .field("gmac_apb_postw_en", &self.gmac_apb_postw_en().bit())
            .field("dsi_host_apb_postw_en", &self.dsi_host_apb_postw_en().bit())
            .field(
                "csi_host_apb_sync_postw_en",
                &self.csi_host_apb_sync_postw_en().bit(),
            )
            .field(
                "csi_host_apb_async_postw_en",
                &self.csi_host_apb_async_postw_en().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_SYNC_POSTW_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_apb_postw_en(&mut self) -> GMAC_APB_POSTW_EN_W<APB_SYNC_POSTW_EN_SPEC> {
        GMAC_APB_POSTW_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_host_apb_postw_en(&mut self) -> DSI_HOST_APB_POSTW_EN_W<APB_SYNC_POSTW_EN_SPEC> {
        DSI_HOST_APB_POSTW_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn csi_host_apb_sync_postw_en(
        &mut self,
    ) -> CSI_HOST_APB_SYNC_POSTW_EN_W<APB_SYNC_POSTW_EN_SPEC> {
        CSI_HOST_APB_SYNC_POSTW_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn csi_host_apb_async_postw_en(
        &mut self,
    ) -> CSI_HOST_APB_ASYNC_POSTW_EN_W<APB_SYNC_POSTW_EN_SPEC> {
        CSI_HOST_APB_ASYNC_POSTW_EN_W::new(self, 3)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_sync_postw_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_sync_postw_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SYNC_POSTW_EN_SPEC;
impl crate::RegisterSpec for APB_SYNC_POSTW_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_sync_postw_en::R`](R) reader structure"]
impl crate::Readable for APB_SYNC_POSTW_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_sync_postw_en::W`](W) writer structure"]
impl crate::Writable for APB_SYNC_POSTW_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_SYNC_POSTW_EN to value 0"]
impl crate::Resettable for APB_SYNC_POSTW_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
