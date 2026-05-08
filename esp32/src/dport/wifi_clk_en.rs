#[doc = "Register `WIFI_CLK_EN` reader"]
pub type R = crate::R<WIFI_CLK_EN_SPEC>;
#[doc = "Register `WIFI_CLK_EN` writer"]
pub type W = crate::W<WIFI_CLK_EN_SPEC>;
#[doc = "Field `WIFI_EN` reader - "]
pub type WIFI_EN_R = crate::FieldReader;
#[doc = "Field `WIFI_EN` writer - "]
pub type WIFI_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WIFI_BT_COMMON` reader - "]
pub type WIFI_BT_COMMON_R = crate::FieldReader;
#[doc = "Field `WIFI_BT_COMMON` writer - "]
pub type WIFI_BT_COMMON_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SDIO_CLK_EN` reader - Set the bit to enable the clock of SDIO module. Clear the bit to disable the clock of SDIO module."]
pub type SDIO_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_CLK_EN` writer - Set the bit to enable the clock of SDIO module. Clear the bit to disable the clock of SDIO module."]
pub type SDIO_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_EN` reader - "]
pub type BT_EN_R = crate::FieldReader;
#[doc = "Field `BT_EN` writer - "]
pub type BT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDIO_HOST_CLK_EN` reader - Set the bit to enable the clock of SD/MMC module. Clear the bit to disable the clock of SD/MMC module."]
pub type SDIO_HOST_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_CLK_EN` writer - Set the bit to enable the clock of SD/MMC module. Clear the bit to disable the clock of SD/MMC module."]
pub type SDIO_HOST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_CLK_EN` reader - Set the bit to enable the clock of Ethernet MAC module. Clear the bit to disable the clock of Ethernet MAC module."]
pub type EMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_CLK_EN` writer - Set the bit to enable the clock of Ethernet MAC module. Clear the bit to disable the clock of Ethernet MAC module."]
pub type EMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CLK_EN` reader - Set the bit to enable the clock of RNG module. Clear the bit to disable the clock of RNG module."]
pub type RNG_CLK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CLK_EN` writer - Set the bit to enable the clock of RNG module. Clear the bit to disable the clock of RNG module."]
pub type RNG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifi_en(&self) -> WIFI_EN_R {
        WIFI_EN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn wifi_bt_common(&self) -> WIFI_BT_COMMON_R {
        WIFI_BT_COMMON_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 4 - Set the bit to enable the clock of SDIO module. Clear the bit to disable the clock of SDIO module."]
    #[inline(always)]
    pub fn sdio_clk_en(&self) -> SDIO_CLK_EN_R {
        SDIO_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn bt_en(&self) -> BT_EN_R {
        BT_EN_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 13 - Set the bit to enable the clock of SD/MMC module. Clear the bit to disable the clock of SD/MMC module."]
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set the bit to enable the clock of Ethernet MAC module. Clear the bit to disable the clock of Ethernet MAC module."]
    #[inline(always)]
    pub fn emac_clk_en(&self) -> EMAC_CLK_EN_R {
        EMAC_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set the bit to enable the clock of RNG module. Clear the bit to disable the clock of RNG module."]
    #[inline(always)]
    pub fn rng_clk_en(&self) -> RNG_CLK_EN_R {
        RNG_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field("wifi_en", &self.wifi_en())
            .field("wifi_bt_common", &self.wifi_bt_common())
            .field("bt_en", &self.bt_en())
            .field("sdio_clk_en", &self.sdio_clk_en())
            .field("sdio_host_clk_en", &self.sdio_host_clk_en())
            .field("emac_clk_en", &self.emac_clk_en())
            .field("rng_clk_en", &self.rng_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifi_en(&mut self) -> WIFI_EN_W<'_, WIFI_CLK_EN_SPEC> {
        WIFI_EN_W::new(self, 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn wifi_bt_common(&mut self) -> WIFI_BT_COMMON_W<'_, WIFI_CLK_EN_SPEC> {
        WIFI_BT_COMMON_W::new(self, 0)
    }
    #[doc = "Bit 4 - Set the bit to enable the clock of SDIO module. Clear the bit to disable the clock of SDIO module."]
    #[inline(always)]
    pub fn sdio_clk_en(&mut self) -> SDIO_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        SDIO_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn bt_en(&mut self) -> BT_EN_W<'_, WIFI_CLK_EN_SPEC> {
        BT_EN_W::new(self, 11)
    }
    #[doc = "Bit 13 - Set the bit to enable the clock of SD/MMC module. Clear the bit to disable the clock of SD/MMC module."]
    #[inline(always)]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        SDIO_HOST_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set the bit to enable the clock of Ethernet MAC module. Clear the bit to disable the clock of Ethernet MAC module."]
    #[inline(always)]
    pub fn emac_clk_en(&mut self) -> EMAC_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        EMAC_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set the bit to enable the clock of RNG module. Clear the bit to disable the clock of RNG module."]
    #[inline(always)]
    pub fn rng_clk_en(&mut self) -> RNG_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        RNG_CLK_EN_W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_clk_en::R`](R) reader structure"]
impl crate::Readable for WIFI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_clk_en::W`](W) writer structure"]
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030"]
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffc_e030;
}
