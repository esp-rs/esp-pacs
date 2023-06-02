#[doc = "Register `BLK0_WDATA5` reader"]
pub struct R(crate::R<BLK0_WDATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA5` writer"]
pub struct W(crate::W<BLK0_WDATA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BLK0_WDATA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_PAD_CONFIG_CLK` reader - program for SPI_pad_config_clk"]
pub type SPI_PAD_CONFIG_CLK_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_CLK` writer - program for SPI_pad_config_clk"]
pub type SPI_PAD_CONFIG_CLK_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 5, O>;
#[doc = "Field `SPI_PAD_CONFIG_Q` reader - program for SPI_pad_config_q"]
pub type SPI_PAD_CONFIG_Q_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_Q` writer - program for SPI_pad_config_q"]
pub type SPI_PAD_CONFIG_Q_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 5, O>;
#[doc = "Field `SPI_PAD_CONFIG_D` reader - program for SPI_pad_config_d"]
pub type SPI_PAD_CONFIG_D_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_D` writer - program for SPI_pad_config_d"]
pub type SPI_PAD_CONFIG_D_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 5, O>;
#[doc = "Field `SPI_PAD_CONFIG_CS0` reader - program for SPI_pad_config_cs0"]
pub type SPI_PAD_CONFIG_CS0_R = crate::FieldReader;
#[doc = "Field `SPI_PAD_CONFIG_CS0` writer - program for SPI_pad_config_cs0"]
pub type SPI_PAD_CONFIG_CS0_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 5, O>;
#[doc = "Field `INST_CONFIG` reader - "]
pub type INST_CONFIG_R = crate::FieldReader;
#[doc = "Field `INST_CONFIG` writer - "]
pub type INST_CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 8, O>;
#[doc = "Field `VOL_LEVEL_HP_INV` reader - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (R/W)"]
pub type VOL_LEVEL_HP_INV_R = crate::FieldReader;
#[doc = "Field `VOL_LEVEL_HP_INV` writer - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (R/W)"]
pub type VOL_LEVEL_HP_INV_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 2, O>;
#[doc = "Field `DIG_VOL_L6` reader - "]
pub type DIG_VOL_L6_R = crate::FieldReader;
#[doc = "Field `DIG_VOL_L6` writer - "]
pub type DIG_VOL_L6_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 4, O>;
#[doc = "Field `FLASH_CRYPT_CONFIG` reader - program for flash_crypt_config"]
pub type FLASH_CRYPT_CONFIG_R = crate::FieldReader;
#[doc = "Field `FLASH_CRYPT_CONFIG` writer - program for flash_crypt_config"]
pub type FLASH_CRYPT_CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA5_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn spi_pad_config_clk(&self) -> SPI_PAD_CONFIG_CLK_R {
        SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    pub fn spi_pad_config_q(&self) -> SPI_PAD_CONFIG_Q_R {
        SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    pub fn spi_pad_config_d(&self) -> SPI_PAD_CONFIG_D_R {
        SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn spi_pad_config_cs0(&self) -> SPI_PAD_CONFIG_CS0_R {
        SPI_PAD_CONFIG_CS0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn inst_config(&self) -> INST_CONFIG_R {
        INST_CONFIG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (R/W)"]
    #[inline(always)]
    pub fn vol_level_hp_inv(&self) -> VOL_LEVEL_HP_INV_R {
        VOL_LEVEL_HP_INV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dig_vol_l6(&self) -> DIG_VOL_L6_R {
        DIG_VOL_L6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - program for flash_crypt_config"]
    #[inline(always)]
    pub fn flash_crypt_config(&self) -> FLASH_CRYPT_CONFIG_R {
        FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA5")
            .field(
                "spi_pad_config_clk",
                &format_args!("{}", self.spi_pad_config_clk().bits()),
            )
            .field(
                "spi_pad_config_q",
                &format_args!("{}", self.spi_pad_config_q().bits()),
            )
            .field(
                "spi_pad_config_d",
                &format_args!("{}", self.spi_pad_config_d().bits()),
            )
            .field(
                "spi_pad_config_cs0",
                &format_args!("{}", self.spi_pad_config_cs0().bits()),
            )
            .field(
                "inst_config",
                &format_args!("{}", self.inst_config().bits()),
            )
            .field(
                "vol_level_hp_inv",
                &format_args!("{}", self.vol_level_hp_inv().bits()),
            )
            .field("dig_vol_l6", &format_args!("{}", self.dig_vol_l6().bits()))
            .field(
                "flash_crypt_config",
                &format_args!("{}", self.flash_crypt_config().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    #[must_use]
    pub fn spi_pad_config_clk(&mut self) -> SPI_PAD_CONFIG_CLK_W<0> {
        SPI_PAD_CONFIG_CLK_W::new(self)
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    #[must_use]
    pub fn spi_pad_config_q(&mut self) -> SPI_PAD_CONFIG_Q_W<5> {
        SPI_PAD_CONFIG_Q_W::new(self)
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    #[must_use]
    pub fn spi_pad_config_d(&mut self) -> SPI_PAD_CONFIG_D_W<10> {
        SPI_PAD_CONFIG_D_W::new(self)
    }
    #[doc = "Bits 15:19 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    #[must_use]
    pub fn spi_pad_config_cs0(&mut self) -> SPI_PAD_CONFIG_CS0_W<15> {
        SPI_PAD_CONFIG_CS0_W::new(self)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    #[must_use]
    pub fn inst_config(&mut self) -> INST_CONFIG_W<20> {
        INST_CONFIG_W::new(self)
    }
    #[doc = "Bits 22:23 - This field stores the voltage level for CPU to run at 240 MHz, or for flash/PSRAM to run at 80 MHz.0x0: level 7; 0x1: level 6; 0x2: level 5; 0x3: level 4. (R/W)"]
    #[inline(always)]
    #[must_use]
    pub fn vol_level_hp_inv(&mut self) -> VOL_LEVEL_HP_INV_W<22> {
        VOL_LEVEL_HP_INV_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dig_vol_l6(&mut self) -> DIG_VOL_L6_W<24> {
        DIG_VOL_L6_W::new(self)
    }
    #[doc = "Bits 28:31 - program for flash_crypt_config"]
    #[inline(always)]
    #[must_use]
    pub fn flash_crypt_config(&mut self) -> FLASH_CRYPT_CONFIG_W<28> {
        FLASH_CRYPT_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata5](index.html) module"]
pub struct BLK0_WDATA5_SPEC;
impl crate::RegisterSpec for BLK0_WDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata5::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata5::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA5 to value 0"]
impl crate::Resettable for BLK0_WDATA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
