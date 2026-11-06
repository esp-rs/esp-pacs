#[doc = "Register `WIFI_CLK_EN` reader"]
pub type R = crate::R<WIFI_CLK_EN_SPEC>;
#[doc = "Register `WIFI_CLK_EN` writer"]
pub type W = crate::W<WIFI_CLK_EN_SPEC>;
#[doc = "Field `SDIOSLAVE_CLK_EN` reader - Set this bit to enable the SDIO slave clock. Clear the bit to disable it."]
pub type SDIOSLAVE_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIOSLAVE_CLK_EN` writer - Set this bit to enable the SDIO slave clock. Clear the bit to disable it."]
pub type SDIOSLAVE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNUSED_BIT5` reader - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT5_R = crate::BitReader;
#[doc = "Field `UNUSED_BIT5` writer - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_BASEBAND_CLK_EN` reader - Set this bit to enable the Bluetooth baseband clock. Clear the bit to disable it."]
pub type BT_BASEBAND_CLK_EN_R = crate::BitReader;
#[doc = "Field `BT_BASEBAND_CLK_EN` writer - Set this bit to enable the Bluetooth baseband clock. Clear the bit to disable it."]
pub type BT_BASEBAND_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNUSED_BIT12` reader - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT12_R = crate::BitReader;
#[doc = "Field `UNUSED_BIT12` writer - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_CLK_EN` reader - Set this bit to enable the SD/MMC host clock. Clear the bit to disable it."]
pub type SDIO_HOST_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_CLK_EN` writer - Set this bit to enable the SD/MMC host clock. Clear the bit to disable it."]
pub type SDIO_HOST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_CLK_EN` reader - Set this bit to enable the Ethernet MAC clock. Clear the bit to disable it."]
pub type EMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_CLK_EN` writer - Set this bit to enable the Ethernet MAC clock. Clear the bit to disable it."]
pub type EMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CLK_EN` reader - Set this bit to enable the RNG clock. Clear the bit to disable it."]
pub type RNG_CLK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CLK_EN` writer - Set this bit to enable the RNG clock. Clear the bit to disable it."]
pub type RNG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_LC_CLK_EN` reader - Set these bits to enable the Bluetooth link controller clocks. Clear the bits to disable them."]
pub type BT_LC_CLK_EN_R = crate::FieldReader;
#[doc = "Field `BT_LC_CLK_EN` writer - Set these bits to enable the Bluetooth link controller clocks. Clear the bits to disable them."]
pub type BT_LC_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAC_CLK_EN` reader - Set this bit to enable MAC module clock. Clear the bit to disable MAC module clock."]
pub type MAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `MAC_CLK_EN` writer - Set this bit to enable MAC module clock. Clear the bit to disable MAC module clock."]
pub type MAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set this bit to enable the SDIO slave clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn sdioslave_clk_en(&self) -> SDIOSLAVE_CLK_EN_R {
        SDIOSLAVE_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit5(&self) -> UNUSED_BIT5_R {
        UNUSED_BIT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable the Bluetooth baseband clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn bt_baseband_clk_en(&self) -> BT_BASEBAND_CLK_EN_R {
        BT_BASEBAND_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit12(&self) -> UNUSED_BIT12_R {
        UNUSED_BIT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable the SD/MMC host clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable the Ethernet MAC clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn emac_clk_en(&self) -> EMAC_CLK_EN_R {
        EMAC_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable the RNG clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn rng_clk_en(&self) -> RNG_CLK_EN_R {
        RNG_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Set these bits to enable the Bluetooth link controller clocks. Clear the bits to disable them."]
    #[inline(always)]
    pub fn bt_lc_clk_en(&self) -> BT_LC_CLK_EN_R {
        BT_LC_CLK_EN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Set this bit to enable MAC module clock. Clear the bit to disable MAC module clock."]
    #[inline(always)]
    pub fn mac_clk_en(&self) -> MAC_CLK_EN_R {
        MAC_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field("sdioslave_clk_en", &self.sdioslave_clk_en())
            .field("unused_bit5", &self.unused_bit5())
            .field("bt_baseband_clk_en", &self.bt_baseband_clk_en())
            .field("unused_bit12", &self.unused_bit12())
            .field("sdio_host_clk_en", &self.sdio_host_clk_en())
            .field("emac_clk_en", &self.emac_clk_en())
            .field("rng_clk_en", &self.rng_clk_en())
            .field("bt_lc_clk_en", &self.bt_lc_clk_en())
            .field("mac_clk_en", &self.mac_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit to enable the SDIO slave clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn sdioslave_clk_en(&mut self) -> SDIOSLAVE_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        SDIOSLAVE_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit5(&mut self) -> UNUSED_BIT5_W<'_, WIFI_CLK_EN_SPEC> {
        UNUSED_BIT5_W::new(self, 5)
    }
    #[doc = "Bit 11 - Set this bit to enable the Bluetooth baseband clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn bt_baseband_clk_en(&mut self) -> BT_BASEBAND_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        BT_BASEBAND_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit12(&mut self) -> UNUSED_BIT12_W<'_, WIFI_CLK_EN_SPEC> {
        UNUSED_BIT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to enable the SD/MMC host clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        SDIO_HOST_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable the Ethernet MAC clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn emac_clk_en(&mut self) -> EMAC_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        EMAC_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to enable the RNG clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn rng_clk_en(&mut self) -> RNG_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        RNG_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Set these bits to enable the Bluetooth link controller clocks. Clear the bits to disable them."]
    #[inline(always)]
    pub fn bt_lc_clk_en(&mut self) -> BT_LC_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        BT_LC_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Set this bit to enable MAC module clock. Clear the bit to disable MAC module clock."]
    #[inline(always)]
    pub fn mac_clk_en(&mut self) -> MAC_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        MAC_CLK_EN_W::new(self, 18)
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
