#[doc = "Register `CONF2` reader"]
pub type R = crate::R<CONF2_SPEC>;
#[doc = "Register `CONF2` writer"]
pub type W = crate::W<CONF2_SPEC>;
#[doc = "Field `CAMERA_EN` reader - "]
pub type CAMERA_EN_R = crate::BitReader;
#[doc = "Field `CAMERA_EN` writer - "]
pub type CAMERA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TX_WRX2_EN` reader - "]
pub type LCD_TX_WRX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_WRX2_EN` writer - "]
pub type LCD_TX_WRX2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TX_SDX2_EN` reader - "]
pub type LCD_TX_SDX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_SDX2_EN` writer - "]
pub type LCD_TX_SDX2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENABLE_TEST_EN` reader - "]
pub type DATA_ENABLE_TEST_EN_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE_TEST_EN` writer - "]
pub type DATA_ENABLE_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENABLE` reader - "]
pub type DATA_ENABLE_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE` writer - "]
pub type DATA_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_EN` reader - "]
pub type LCD_EN_R = crate::BitReader;
#[doc = "Field `LCD_EN` writer - "]
pub type LCD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_ADC_START_EN` reader - "]
pub type EXT_ADC_START_EN_R = crate::BitReader;
#[doc = "Field `EXT_ADC_START_EN` writer - "]
pub type EXT_ADC_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTER_VALID_EN` reader - "]
pub type INTER_VALID_EN_R = crate::BitReader;
#[doc = "Field `INTER_VALID_EN` writer - "]
pub type INTER_VALID_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn camera_en(&self) -> CAMERA_EN_R {
        CAMERA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&self) -> LCD_TX_WRX2_EN_R {
        LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&self) -> LCD_TX_SDX2_EN_R {
        LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_enable_test_en(&self) -> DATA_ENABLE_TEST_EN_R {
        DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ext_adc_start_en(&self) -> EXT_ADC_START_EN_R {
        EXT_ADC_START_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inter_valid_en(&self) -> INTER_VALID_EN_R {
        INTER_VALID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF2")
            .field("camera_en", &self.camera_en())
            .field("lcd_tx_wrx2_en", &self.lcd_tx_wrx2_en())
            .field("lcd_tx_sdx2_en", &self.lcd_tx_sdx2_en())
            .field("data_enable_test_en", &self.data_enable_test_en())
            .field("data_enable", &self.data_enable())
            .field("lcd_en", &self.lcd_en())
            .field("ext_adc_start_en", &self.ext_adc_start_en())
            .field("inter_valid_en", &self.inter_valid_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_en(&mut self) -> CAMERA_EN_W<CONF2_SPEC> {
        CAMERA_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W<CONF2_SPEC> {
        LCD_TX_WRX2_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W<CONF2_SPEC> {
        LCD_TX_SDX2_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W<CONF2_SPEC> {
        DATA_ENABLE_TEST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W<CONF2_SPEC> {
        DATA_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_en(&mut self) -> LCD_EN_W<CONF2_SPEC> {
        LCD_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W<CONF2_SPEC> {
        EXT_ADC_START_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W<CONF2_SPEC> {
        INTER_VALID_EN_W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF2_SPEC;
impl crate::RegisterSpec for CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf2::R`](R) reader structure"]
impl crate::Readable for CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf2::W`](W) writer structure"]
impl crate::Writable for CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF2 to value 0"]
impl crate::Resettable for CONF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
