#[doc = "Register `LCD_RGB_YUV` reader"]
pub type R = crate::R<LCD_RGB_YUV_SPEC>;
#[doc = "Register `LCD_RGB_YUV` writer"]
pub type W = crate::W<LCD_RGB_YUV_SPEC>;
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type LCD_CONV_8BITS_DATA_INV_R = crate::BitReader;
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type LCD_CONV_8BITS_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_TXTORX` reader - 0: txtorx mode off. 1: txtorx mode on."]
pub type LCD_CONV_TXTORX_R = crate::BitReader;
#[doc = "Field `LCD_CONV_TXTORX` writer - 0: txtorx mode off. 1: txtorx mode on."]
pub type LCD_CONV_TXTORX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LCD_CONV_YUV2YUV_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LCD_CONV_YUV2YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LCD_CONV_YUV_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LCD_CONV_YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type LCD_CONV_PROTOCOL_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type LCD_CONV_PROTOCOL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LCD_CONV_DATA_OUT_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LCD_CONV_DATA_OUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LCD_CONV_DATA_IN_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LCD_CONV_DATA_IN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type LCD_CONV_MODE_8BITS_ON_R = crate::BitReader;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type LCD_CONV_MODE_8BITS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type LCD_CONV_TRANS_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type LCD_CONV_TRANS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_ENABLE` reader - 0: Bypass converter. 1: Enable converter."]
pub type LCD_CONV_ENABLE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_ENABLE` writer - 0: Bypass converter. 1: Enable converter."]
pub type LCD_CONV_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&self) -> LCD_CONV_8BITS_DATA_INV_R {
        LCD_CONV_8BITS_DATA_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&self) -> LCD_CONV_TXTORX_R {
        LCD_CONV_TXTORX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&self) -> LCD_CONV_YUV2YUV_MODE_R {
        LCD_CONV_YUV2YUV_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&self) -> LCD_CONV_YUV_MODE_R {
        LCD_CONV_YUV_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&self) -> LCD_CONV_PROTOCOL_MODE_R {
        LCD_CONV_PROTOCOL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&self) -> LCD_CONV_DATA_OUT_MODE_R {
        LCD_CONV_DATA_OUT_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&self) -> LCD_CONV_DATA_IN_MODE_R {
        LCD_CONV_DATA_IN_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&self) -> LCD_CONV_MODE_8BITS_ON_R {
        LCD_CONV_MODE_8BITS_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&self) -> LCD_CONV_TRANS_MODE_R {
        LCD_CONV_TRANS_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_enable(&self) -> LCD_CONV_ENABLE_R {
        LCD_CONV_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RGB_YUV")
            .field("lcd_conv_8bits_data_inv", &self.lcd_conv_8bits_data_inv())
            .field("lcd_conv_txtorx", &self.lcd_conv_txtorx())
            .field("lcd_conv_yuv2yuv_mode", &self.lcd_conv_yuv2yuv_mode())
            .field("lcd_conv_yuv_mode", &self.lcd_conv_yuv_mode())
            .field("lcd_conv_protocol_mode", &self.lcd_conv_protocol_mode())
            .field("lcd_conv_data_out_mode", &self.lcd_conv_data_out_mode())
            .field("lcd_conv_data_in_mode", &self.lcd_conv_data_in_mode())
            .field("lcd_conv_mode_8bits_on", &self.lcd_conv_mode_8bits_on())
            .field("lcd_conv_trans_mode", &self.lcd_conv_trans_mode())
            .field("lcd_conv_enable", &self.lcd_conv_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&mut self) -> LCD_CONV_8BITS_DATA_INV_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_8BITS_DATA_INV_W::new(self, 20)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&mut self) -> LCD_CONV_TXTORX_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_TXTORX_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&mut self) -> LCD_CONV_YUV2YUV_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_YUV2YUV_MODE_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&mut self) -> LCD_CONV_YUV_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_YUV_MODE_W::new(self, 24)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&mut self) -> LCD_CONV_PROTOCOL_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_PROTOCOL_MODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&mut self) -> LCD_CONV_DATA_OUT_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_DATA_OUT_MODE_W::new(self, 27)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&mut self) -> LCD_CONV_DATA_IN_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_DATA_IN_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&mut self) -> LCD_CONV_MODE_8BITS_ON_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_MODE_8BITS_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&mut self) -> LCD_CONV_TRANS_MODE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_TRANS_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_enable(&mut self) -> LCD_CONV_ENABLE_W<LCD_RGB_YUV_SPEC> {
        LCD_CONV_ENABLE_W::new(self, 31)
    }
}
#[doc = "LCD YUV/RGB converter configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_rgb_yuv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rgb_yuv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_RGB_YUV_SPEC;
impl crate::RegisterSpec for LCD_RGB_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_rgb_yuv::R`](R) reader structure"]
impl crate::Readable for LCD_RGB_YUV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_rgb_yuv::W`](W) writer structure"]
impl crate::Writable for LCD_RGB_YUV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for LCD_RGB_YUV_SPEC {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
