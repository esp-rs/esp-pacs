#[doc = "Register `BLK0_WDATA5` reader"]
pub type R = crate::R<BLK0_WDATA5_SPEC>;
#[doc = "Register `BLK0_WDATA5` writer"]
pub type W = crate::W<BLK0_WDATA5_SPEC>;
#[doc = "Field `SPI_PAD_CONFIG_CLK` reader - "]
pub type SPI_PAD_CONFIG_CLK_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_Q` reader - "]
pub type SPI_PAD_CONFIG_Q_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D` reader - "]
pub type SPI_PAD_CONFIG_D_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_CS0` reader - "]
pub type SPI_PAD_CONFIG_CS0_R = crate::FieldReader;
#[doc = "Field `CHIP_VER_REV2` reader - "]
pub type CHIP_VER_REV2_R = crate::BitReader;
#[doc = "Field `RESERVE_0_181` reader - "]
pub type RESERVE_0_181_R = crate::BitReader;
#[doc = "Field `RESERVE_0_181` writer - "]
pub type RESERVE_0_181_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOL_LEVEL_HP_INV` reader - "]
pub type VOL_LEVEL_HP_INV_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MINOR` reader - "]
pub type WAFER_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `RESERVE_0_186` reader - "]
pub type RESERVE_0_186_R = crate::FieldReader;
#[doc = "Field `RESERVE_0_186` writer - "]
pub type RESERVE_0_186_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_CRYPT_CONFIG` reader - "]
pub type FLASH_CRYPT_CONFIG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_pad_config_clk(&self) -> SPI_PAD_CONFIG_CLK_R {
        SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn spi_pad_config_q(&self) -> SPI_PAD_CONFIG_Q_R {
        SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn spi_pad_config_d(&self) -> SPI_PAD_CONFIG_D_R {
        SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn spi_pad_config_cs0(&self) -> SPI_PAD_CONFIG_CS0_R {
        SPI_PAD_CONFIG_CS0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chip_ver_rev2(&self) -> CHIP_VER_REV2_R {
        CHIP_VER_REV2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reserve_0_181(&self) -> RESERVE_0_181_R {
        RESERVE_0_181_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn vol_level_hp_inv(&self) -> VOL_LEVEL_HP_INV_R {
        VOL_LEVEL_HP_INV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn wafer_version_minor(&self) -> WAFER_VERSION_MINOR_R {
        WAFER_VERSION_MINOR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserve_0_186(&self) -> RESERVE_0_186_R {
        RESERVE_0_186_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_crypt_config(&self) -> FLASH_CRYPT_CONFIG_R {
        FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA5")
            .field("spi_pad_config_clk", &self.spi_pad_config_clk())
            .field("spi_pad_config_q", &self.spi_pad_config_q())
            .field("spi_pad_config_d", &self.spi_pad_config_d())
            .field("spi_pad_config_cs0", &self.spi_pad_config_cs0())
            .field("chip_ver_rev2", &self.chip_ver_rev2())
            .field("reserve_0_181", &self.reserve_0_181())
            .field("vol_level_hp_inv", &self.vol_level_hp_inv())
            .field("wafer_version_minor", &self.wafer_version_minor())
            .field("reserve_0_186", &self.reserve_0_186())
            .field("flash_crypt_config", &self.flash_crypt_config())
            .finish()
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reserve_0_181(&mut self) -> RESERVE_0_181_W<BLK0_WDATA5_SPEC> {
        RESERVE_0_181_W::new(self, 21)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn reserve_0_186(&mut self) -> RESERVE_0_186_W<BLK0_WDATA5_SPEC> {
        RESERVE_0_186_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA5_SPEC;
impl crate::RegisterSpec for BLK0_WDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata5::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata5::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA5 to value 0"]
impl crate::Resettable for BLK0_WDATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
