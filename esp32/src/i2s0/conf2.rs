#[doc = "Register `CONF2` reader"]
pub struct R(crate::R<CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF2` writer"]
pub struct W(crate::W<CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF2_SPEC>;
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
impl From<crate::W<CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAMERA_EN` reader - "]
pub type CAMERA_EN_R = crate::BitReader;
#[doc = "Field `CAMERA_EN` writer - "]
pub type CAMERA_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `LCD_TX_WRX2_EN` reader - "]
pub type LCD_TX_WRX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_WRX2_EN` writer - "]
pub type LCD_TX_WRX2_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `LCD_TX_SDX2_EN` reader - "]
pub type LCD_TX_SDX2_EN_R = crate::BitReader;
#[doc = "Field `LCD_TX_SDX2_EN` writer - "]
pub type LCD_TX_SDX2_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `DATA_ENABLE_TEST_EN` reader - "]
pub type DATA_ENABLE_TEST_EN_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE_TEST_EN` writer - "]
pub type DATA_ENABLE_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `DATA_ENABLE` reader - "]
pub type DATA_ENABLE_R = crate::BitReader;
#[doc = "Field `DATA_ENABLE` writer - "]
pub type DATA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `LCD_EN` reader - "]
pub type LCD_EN_R = crate::BitReader;
#[doc = "Field `LCD_EN` writer - "]
pub type LCD_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `EXT_ADC_START_EN` reader - "]
pub type EXT_ADC_START_EN_R = crate::BitReader;
#[doc = "Field `EXT_ADC_START_EN` writer - "]
pub type EXT_ADC_START_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
#[doc = "Field `INTER_VALID_EN` reader - "]
pub type INTER_VALID_EN_R = crate::BitReader;
#[doc = "Field `INTER_VALID_EN` writer - "]
pub type INTER_VALID_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF2_SPEC, O>;
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
            .field("camera_en", &format_args!("{}", self.camera_en().bit()))
            .field(
                "lcd_tx_wrx2_en",
                &format_args!("{}", self.lcd_tx_wrx2_en().bit()),
            )
            .field(
                "lcd_tx_sdx2_en",
                &format_args!("{}", self.lcd_tx_sdx2_en().bit()),
            )
            .field(
                "data_enable_test_en",
                &format_args!("{}", self.data_enable_test_en().bit()),
            )
            .field("data_enable", &format_args!("{}", self.data_enable().bit()))
            .field("lcd_en", &format_args!("{}", self.lcd_en().bit()))
            .field(
                "ext_adc_start_en",
                &format_args!("{}", self.ext_adc_start_en().bit()),
            )
            .field(
                "inter_valid_en",
                &format_args!("{}", self.inter_valid_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn camera_en(&mut self) -> CAMERA_EN_W<0> {
        CAMERA_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W<1> {
        LCD_TX_WRX2_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W<2> {
        LCD_TX_SDX2_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W<3> {
        DATA_ENABLE_TEST_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W<4> {
        DATA_ENABLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_en(&mut self) -> LCD_EN_W<5> {
        LCD_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W<6> {
        EXT_ADC_START_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W<7> {
        INTER_VALID_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2](index.html) module"]
pub struct CONF2_SPEC;
impl crate::RegisterSpec for CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf2::R](R) reader structure"]
impl crate::Readable for CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf2::W](W) writer structure"]
impl crate::Writable for CONF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF2 to value 0"]
impl crate::Resettable for CONF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
