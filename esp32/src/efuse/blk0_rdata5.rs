#[doc = "Register `BLK0_RDATA5` reader"]
pub struct R(crate::R<BLK0_RDATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_SPI_PAD_CONFIG_CLK` reader - read for SPI_pad_config_clk"]
pub type RD_SPI_PAD_CONFIG_CLK_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_Q` reader - read for SPI_pad_config_q"]
pub type RD_SPI_PAD_CONFIG_Q_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_D` reader - read for SPI_pad_config_d"]
pub type RD_SPI_PAD_CONFIG_D_R = crate::FieldReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_CS0` reader - read for SPI_pad_config_cs0"]
pub type RD_SPI_PAD_CONFIG_CS0_R = crate::FieldReader;
#[doc = "Field `RD_CHIP_VER_REV2` reader - "]
pub type RD_CHIP_VER_REV2_R = crate::BitReader;
#[doc = "Field `RD_VOL_LEVEL_HP_INV` reader - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (RO)"]
pub type RD_VOL_LEVEL_HP_INV_R = crate::FieldReader;
#[doc = "Field `RD_WAFER_VERSION_MINOR` reader - "]
pub type RD_WAFER_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `RD_FLASH_CRYPT_CONFIG` reader - read for flash_crypt_config"]
pub type RD_FLASH_CRYPT_CONFIG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - read for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn rd_spi_pad_config_clk(&self) -> RD_SPI_PAD_CONFIG_CLK_R {
        RD_SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - read for SPI_pad_config_q"]
    #[inline(always)]
    pub fn rd_spi_pad_config_q(&self) -> RD_SPI_PAD_CONFIG_Q_R {
        RD_SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - read for SPI_pad_config_d"]
    #[inline(always)]
    pub fn rd_spi_pad_config_d(&self) -> RD_SPI_PAD_CONFIG_D_R {
        RD_SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - read for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn rd_spi_pad_config_cs0(&self) -> RD_SPI_PAD_CONFIG_CS0_R {
        RD_SPI_PAD_CONFIG_CS0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rd_chip_ver_rev2(&self) -> RD_CHIP_VER_REV2_R {
        RD_CHIP_VER_REV2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (RO)"]
    #[inline(always)]
    pub fn rd_vol_level_hp_inv(&self) -> RD_VOL_LEVEL_HP_INV_R {
        RD_VOL_LEVEL_HP_INV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rd_wafer_version_minor(&self) -> RD_WAFER_VERSION_MINOR_R {
        RD_WAFER_VERSION_MINOR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - read for flash_crypt_config"]
    #[inline(always)]
    pub fn rd_flash_crypt_config(&self) -> RD_FLASH_CRYPT_CONFIG_R {
        RD_FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA5")
            .field(
                "rd_spi_pad_config_clk",
                &format_args!("{}", self.rd_spi_pad_config_clk().bits()),
            )
            .field(
                "rd_spi_pad_config_q",
                &format_args!("{}", self.rd_spi_pad_config_q().bits()),
            )
            .field(
                "rd_spi_pad_config_d",
                &format_args!("{}", self.rd_spi_pad_config_d().bits()),
            )
            .field(
                "rd_spi_pad_config_cs0",
                &format_args!("{}", self.rd_spi_pad_config_cs0().bits()),
            )
            .field(
                "rd_chip_ver_rev2",
                &format_args!("{}", self.rd_chip_ver_rev2().bit()),
            )
            .field(
                "rd_vol_level_hp_inv",
                &format_args!("{}", self.rd_vol_level_hp_inv().bits()),
            )
            .field(
                "rd_wafer_version_minor",
                &format_args!("{}", self.rd_wafer_version_minor().bits()),
            )
            .field(
                "rd_flash_crypt_config",
                &format_args!("{}", self.rd_flash_crypt_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata5](index.html) module"]
pub struct BLK0_RDATA5_SPEC;
impl crate::RegisterSpec for BLK0_RDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata5::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA5 to value 0"]
impl crate::Resettable for BLK0_RDATA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
