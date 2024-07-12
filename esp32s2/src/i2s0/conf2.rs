#[doc = "Register `CONF2` reader"]
pub type R = crate::R<CONF2_SPEC>;
#[doc = "Register `CONF2` writer"]
pub type W = crate::W<CONF2_SPEC>;
#[doc = "Field `CAMERA_EN` reader - Set this bit to enable camera mode."]
pub type CAMERA_EN_R = crate::BitReader;
#[doc = "Field `CAMERA_EN` writer - Set this bit to enable camera mode."]
pub type CAMERA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TX_WRX2_EN` reader - LCD WR double for one datum."]
pub type LCD_TX_WRX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_WRX2_EN` writer - LCD WR double for one datum."]
pub type LCD_TX_WRX2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TX_SDX2_EN` reader - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
pub type LCD_TX_SDX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_SDX2_EN` writer - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
pub type LCD_TX_SDX2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENABLE_TEST_EN` reader - for debug camera mode enable"]
pub type DATA_ENABLE_TEST_EN_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE_TEST_EN` writer - for debug camera mode enable"]
pub type DATA_ENABLE_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENABLE` reader - for debug camera mode enable"]
pub type DATA_ENABLE_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE` writer - for debug camera mode enable"]
pub type DATA_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_EN` reader - Set this bit to enable LCD mode."]
pub type LCD_EN_R = crate::BitReader;
#[doc = "Field `LCD_EN` writer - Set this bit to enable LCD mode."]
pub type LCD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_ADC_START_EN` reader - Set this bit to enable the function that ADC mode is triggered by external signal."]
pub type EXT_ADC_START_EN_R = crate::BitReader;
#[doc = "Field `EXT_ADC_START_EN` writer - Set this bit to enable the function that ADC mode is triggered by external signal."]
pub type EXT_ADC_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTER_VALID_EN` reader - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
pub type INTER_VALID_EN_R = crate::BitReader;
#[doc = "Field `INTER_VALID_EN` writer - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
pub type INTER_VALID_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_SYNC_FIFO_RESET` reader - Set this bit to reset FIFO in camera mode."]
pub type CAM_SYNC_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `CAM_SYNC_FIFO_RESET` writer - Set this bit to reset FIFO in camera mode."]
pub type CAM_SYNC_FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CLK_LOOPBACK` reader - Set this bit to loopback PCLK from I2S0I_WS_out."]
pub type CAM_CLK_LOOPBACK_R = crate::BitReader;
#[doc = "Field `CAM_CLK_LOOPBACK` writer - Set this bit to loopback PCLK from I2S0I_WS_out."]
pub type CAM_CLK_LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_FILTER_EN` reader - Set this bit to enable I2S VSYNC filter function."]
pub type VSYNC_FILTER_EN_R = crate::BitReader;
#[doc = "Field `VSYNC_FILTER_EN` writer - Set this bit to enable I2S VSYNC filter function."]
pub type VSYNC_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_FILTER_THRES` reader - Configure the I2S VSYNC filter threshold value."]
pub type VSYNC_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `VSYNC_FILTER_THRES` writer - Configure the I2S VSYNC filter threshold value."]
pub type VSYNC_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable camera mode."]
    #[inline(always)]
    pub fn camera_en(&self) -> CAMERA_EN_R {
        CAMERA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD WR double for one datum."]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&self) -> LCD_TX_WRX2_EN_R {
        LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&self) -> LCD_TX_SDX2_EN_R {
        LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable_test_en(&self) -> DATA_ENABLE_TEST_EN_R {
        DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - for debug camera mode enable"]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable LCD mode."]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable the function that ADC mode is triggered by external signal."]
    #[inline(always)]
    pub fn ext_adc_start_en(&self) -> EXT_ADC_START_EN_R {
        EXT_ADC_START_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
    #[inline(always)]
    pub fn inter_valid_en(&self) -> INTER_VALID_EN_R {
        INTER_VALID_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to reset FIFO in camera mode."]
    #[inline(always)]
    pub fn cam_sync_fifo_reset(&self) -> CAM_SYNC_FIFO_RESET_R {
        CAM_SYNC_FIFO_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to loopback PCLK from I2S0I_WS_out."]
    #[inline(always)]
    pub fn cam_clk_loopback(&self) -> CAM_CLK_LOOPBACK_R {
        CAM_CLK_LOOPBACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable I2S VSYNC filter function."]
    #[inline(always)]
    pub fn vsync_filter_en(&self) -> VSYNC_FILTER_EN_R {
        VSYNC_FILTER_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Configure the I2S VSYNC filter threshold value."]
    #[inline(always)]
    pub fn vsync_filter_thres(&self) -> VSYNC_FILTER_THRES_R {
        VSYNC_FILTER_THRES_R::new(((self.bits >> 11) & 7) as u8)
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
            .field("cam_sync_fifo_reset", &self.cam_sync_fifo_reset())
            .field("cam_clk_loopback", &self.cam_clk_loopback())
            .field("vsync_filter_en", &self.vsync_filter_en())
            .field("vsync_filter_thres", &self.vsync_filter_thres())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable camera mode."]
    #[inline(always)]
    #[must_use]
    pub fn camera_en(&mut self) -> CAMERA_EN_W<CONF2_SPEC> {
        CAMERA_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LCD WR double for one datum."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W<CONF2_SPEC> {
        LCD_TX_WRX2_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to duplicate data pairs (Frame Form 2) in LCD mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W<CONF2_SPEC> {
        LCD_TX_SDX2_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - for debug camera mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W<CONF2_SPEC> {
        DATA_ENABLE_TEST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - for debug camera mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W<CONF2_SPEC> {
        DATA_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable LCD mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_en(&mut self) -> LCD_EN_W<CONF2_SPEC> {
        LCD_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable the function that ADC mode is triggered by external signal."]
    #[inline(always)]
    #[must_use]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W<CONF2_SPEC> {
        EXT_ADC_START_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable camera VGA reducing-resolution mode: only receive two consecutive cycle data in four consecutive clocks."]
    #[inline(always)]
    #[must_use]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W<CONF2_SPEC> {
        INTER_VALID_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to reset FIFO in camera mode."]
    #[inline(always)]
    #[must_use]
    pub fn cam_sync_fifo_reset(&mut self) -> CAM_SYNC_FIFO_RESET_W<CONF2_SPEC> {
        CAM_SYNC_FIFO_RESET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to loopback PCLK from I2S0I_WS_out."]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_loopback(&mut self) -> CAM_CLK_LOOPBACK_W<CONF2_SPEC> {
        CAM_CLK_LOOPBACK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable I2S VSYNC filter function."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_filter_en(&mut self) -> VSYNC_FILTER_EN_W<CONF2_SPEC> {
        VSYNC_FILTER_EN_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - Configure the I2S VSYNC filter threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_filter_thres(&mut self) -> VSYNC_FILTER_THRES_W<CONF2_SPEC> {
        VSYNC_FILTER_THRES_W::new(self, 11)
    }
}
#[doc = "I2S configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
