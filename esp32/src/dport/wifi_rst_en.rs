#[doc = "Register `WIFI_RST_EN` reader"]
pub type R = crate::R<WIFI_RST_EN_SPEC>;
#[doc = "Register `WIFI_RST_EN` writer"]
pub type W = crate::W<WIFI_RST_EN_SPEC>;
#[doc = "Field `MAC_RST` reader - Set this bit to reset MAC module. Clear the bit to release MAC module."]
pub type MAC_RST_R = crate::BitReader;
#[doc = "Field `MAC_RST` writer - Set this bit to reset MAC module. Clear the bit to release MAC module."]
pub type MAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_RST` reader - Set this bit to reset SDIO module. Clear the bit to release SDIO module."]
pub type SDIO_RST_R = crate::BitReader;
#[doc = "Field `SDIO_RST` writer - Set this bit to reset SDIO module. Clear the bit to release SDIO module."]
pub type SDIO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_RST` reader - Set this bit to reset SD/MMC module. Clear the bit to release SD/MMC module."]
pub type SDIO_HOST_RST_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_RST` writer - Set this bit to reset SD/MMC module. Clear the bit to release SD/MMC module."]
pub type SDIO_HOST_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RST` reader - Set this bit to reset Ethernet MAC module. Clear the bit to release Ethernet MAC module."]
pub type EMAC_RST_R = crate::BitReader;
#[doc = "Field `EMAC_RST` writer - Set this bit to reset Ethernet MAC module. Clear the bit to release Ethernet MAC module."]
pub type EMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Set this bit to reset MAC module. Clear the bit to release MAC module."]
    #[inline(always)]
    pub fn mac_rst(&self) -> MAC_RST_R {
        MAC_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to reset SDIO module. Clear the bit to release SDIO module."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to reset SD/MMC module. Clear the bit to release SD/MMC module."]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to reset Ethernet MAC module. Clear the bit to release Ethernet MAC module."]
    #[inline(always)]
    pub fn emac_rst(&self) -> EMAC_RST_R {
        EMAC_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_RST_EN")
            .field("emac_rst", &self.emac_rst())
            .field("sdio_host_rst", &self.sdio_host_rst())
            .field("sdio_rst", &self.sdio_rst())
            .field("mac_rst", &self.mac_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Set this bit to reset MAC module. Clear the bit to release MAC module."]
    #[inline(always)]
    pub fn mac_rst(&mut self) -> MAC_RST_W<WIFI_RST_EN_SPEC> {
        MAC_RST_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to reset SDIO module. Clear the bit to release SDIO module."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<WIFI_RST_EN_SPEC> {
        SDIO_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to reset SD/MMC module. Clear the bit to release SD/MMC module."]
    #[inline(always)]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W<WIFI_RST_EN_SPEC> {
        SDIO_HOST_RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to reset Ethernet MAC module. Clear the bit to release Ethernet MAC module."]
    #[inline(always)]
    pub fn emac_rst(&mut self) -> EMAC_RST_W<WIFI_RST_EN_SPEC> {
        EMAC_RST_W::new(self, 7)
    }
}
#[doc = "Wifi peripheral reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_RST_EN_SPEC;
impl crate::RegisterSpec for WIFI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_rst_en::R`](R) reader structure"]
impl crate::Readable for WIFI_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_rst_en::W`](W) writer structure"]
impl crate::Writable for WIFI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_RST_EN to value 0"]
impl crate::Resettable for WIFI_RST_EN_SPEC {}
