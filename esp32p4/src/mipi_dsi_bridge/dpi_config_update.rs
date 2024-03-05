#[doc = "Register `DPI_CONFIG_UPDATE` writer"]
pub type W = crate::W<DPI_CONFIG_UPDATE_SPEC>;
#[doc = "Field `DPI_CONFIG_UPDATE` writer - write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*"]
pub type DPI_CONFIG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPI_CONFIG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*"]
    #[inline(always)]
    #[must_use]
    pub fn dpi_config_update(&mut self) -> DPI_CONFIG_UPDATE_W<DPI_CONFIG_UPDATE_SPEC> {
        DPI_CONFIG_UPDATE_W::new(self, 0)
    }
}
#[doc = "dsi_bridge dpi config update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_config_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_CONFIG_UPDATE_SPEC;
impl crate::RegisterSpec for DPI_CONFIG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dpi_config_update::W`](W) writer structure"]
impl crate::Writable for DPI_CONFIG_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPI_CONFIG_UPDATE to value 0"]
impl crate::Resettable for DPI_CONFIG_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
