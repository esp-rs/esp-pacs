#[doc = "Register `LCD_RGB_YUV` reader"]
pub struct R(crate::R<LCD_RGB_YUV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_RGB_YUV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_RGB_YUV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_RGB_YUV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_RGB_YUV` writer"]
pub struct W(crate::W<LCD_RGB_YUV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_RGB_YUV_SPEC>;
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
impl From<crate::W<LCD_RGB_YUV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_RGB_YUV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type LCD_CONV_8BITS_DATA_INV_R = crate::BitReader;
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type LCD_CONV_8BITS_DATA_INV_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_TXTORX` reader - 0: txtorx mode off. 1: txtorx mode on."]
pub type LCD_CONV_TXTORX_R = crate::BitReader;
#[doc = "Field `LCD_CONV_TXTORX` writer - 0: txtorx mode off. 1: txtorx mode on."]
pub type LCD_CONV_TXTORX_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LCD_CONV_YUV2YUV_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LCD_CONV_YUV2YUV_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_RGB_YUV_SPEC, 2, O>;
#[doc = "Field `LCD_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LCD_CONV_YUV_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LCD_CONV_YUV_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_RGB_YUV_SPEC, 2, O>;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type LCD_CONV_PROTOCOL_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type LCD_CONV_PROTOCOL_MODE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LCD_CONV_DATA_OUT_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LCD_CONV_DATA_OUT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LCD_CONV_DATA_IN_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LCD_CONV_DATA_IN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type LCD_CONV_MODE_8BITS_ON_R = crate::BitReader;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type LCD_CONV_MODE_8BITS_ON_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type LCD_CONV_TRANS_MODE_R = crate::BitReader;
#[doc = "Field `LCD_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type LCD_CONV_TRANS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
#[doc = "Field `LCD_CONV_BYPASS` reader - 0: Bypass converter. 1: Enable converter."]
pub type LCD_CONV_BYPASS_R = crate::BitReader;
#[doc = "Field `LCD_CONV_BYPASS` writer - 0: Bypass converter. 1: Enable converter."]
pub type LCD_CONV_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, LCD_RGB_YUV_SPEC, O>;
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
    pub fn lcd_conv_bypass(&self) -> LCD_CONV_BYPASS_R {
        LCD_CONV_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_RGB_YUV")
            .field(
                "lcd_conv_8bits_data_inv",
                &format_args!("{}", self.lcd_conv_8bits_data_inv().bit()),
            )
            .field(
                "lcd_conv_txtorx",
                &format_args!("{}", self.lcd_conv_txtorx().bit()),
            )
            .field(
                "lcd_conv_yuv2yuv_mode",
                &format_args!("{}", self.lcd_conv_yuv2yuv_mode().bits()),
            )
            .field(
                "lcd_conv_yuv_mode",
                &format_args!("{}", self.lcd_conv_yuv_mode().bits()),
            )
            .field(
                "lcd_conv_protocol_mode",
                &format_args!("{}", self.lcd_conv_protocol_mode().bit()),
            )
            .field(
                "lcd_conv_data_out_mode",
                &format_args!("{}", self.lcd_conv_data_out_mode().bit()),
            )
            .field(
                "lcd_conv_data_in_mode",
                &format_args!("{}", self.lcd_conv_data_in_mode().bit()),
            )
            .field(
                "lcd_conv_mode_8bits_on",
                &format_args!("{}", self.lcd_conv_mode_8bits_on().bit()),
            )
            .field(
                "lcd_conv_trans_mode",
                &format_args!("{}", self.lcd_conv_trans_mode().bit()),
            )
            .field(
                "lcd_conv_bypass",
                &format_args!("{}", self.lcd_conv_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_RGB_YUV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_8bits_data_inv(&mut self) -> LCD_CONV_8BITS_DATA_INV_W<20> {
        LCD_CONV_8BITS_DATA_INV_W::new(self)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_txtorx(&mut self) -> LCD_CONV_TXTORX_W<21> {
        LCD_CONV_TXTORX_W::new(self)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_yuv2yuv_mode(&mut self) -> LCD_CONV_YUV2YUV_MODE_W<22> {
        LCD_CONV_YUV2YUV_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_yuv_mode(&mut self) -> LCD_CONV_YUV_MODE_W<24> {
        LCD_CONV_YUV_MODE_W::new(self)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_protocol_mode(&mut self) -> LCD_CONV_PROTOCOL_MODE_W<26> {
        LCD_CONV_PROTOCOL_MODE_W::new(self)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_data_out_mode(&mut self) -> LCD_CONV_DATA_OUT_MODE_W<27> {
        LCD_CONV_DATA_OUT_MODE_W::new(self)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_data_in_mode(&mut self) -> LCD_CONV_DATA_IN_MODE_W<28> {
        LCD_CONV_DATA_IN_MODE_W::new(self)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_mode_8bits_on(&mut self) -> LCD_CONV_MODE_8BITS_ON_W<29> {
        LCD_CONV_MODE_8BITS_ON_W::new(self)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_trans_mode(&mut self) -> LCD_CONV_TRANS_MODE_W<30> {
        LCD_CONV_TRANS_MODE_W::new(self)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_conv_bypass(&mut self) -> LCD_CONV_BYPASS_W<31> {
        LCD_CONV_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_rgb_yuv](index.html) module"]
pub struct LCD_RGB_YUV_SPEC;
impl crate::RegisterSpec for LCD_RGB_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_rgb_yuv::R](R) reader structure"]
impl crate::Readable for LCD_RGB_YUV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_rgb_yuv::W](W) writer structure"]
impl crate::Writable for LCD_RGB_YUV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for LCD_RGB_YUV_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c0_0000;
}
