///Register `CAM_RGB_YUV` reader
pub type R = crate::R<CAM_RGB_YUV_SPEC>;
///Register `CAM_RGB_YUV` writer
pub type W = crate::W<CAM_RGB_YUV_SPEC>;
///Field `CAM_CONV_8BITS_DATA_INV` reader - Swap every two 8-bit input data. 1: Enabled. 0: Disabled.
pub type CAM_CONV_8BITS_DATA_INV_R = crate::BitReader;
///Field `CAM_CONV_8BITS_DATA_INV` writer - Swap every two 8-bit input data. 1: Enabled. 0: Disabled.
pub type CAM_CONV_8BITS_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_YUV2YUV_MODE` reader - In YUV-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format. 3: disabled. To enable YUV-to-YUV mode, LCD_CAM_CAM_CONV_TRANS_MODE must be set to 1.
pub type CAM_CONV_YUV2YUV_MODE_R = crate::FieldReader;
///Field `CAM_CONV_YUV2YUV_MODE` writer - In YUV-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format. 3: disabled. To enable YUV-to-YUV mode, LCD_CAM_CAM_CONV_TRANS_MODE must be set to 1.
pub type CAM_CONV_YUV2YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CAM_CONV_YUV_MODE` reader - In YUV-to-YUV mode and YUV-to-RGB mode, LCD_CAM_CAM_CONV_YUV_MODE decides the YUV mode of input data. 0: input data is in YUV422 format. 1: input data is in YUV420 format. 2: input data is in YUV411 format. In RGB-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format.
pub type CAM_CONV_YUV_MODE_R = crate::FieldReader;
///Field `CAM_CONV_YUV_MODE` writer - In YUV-to-YUV mode and YUV-to-RGB mode, LCD_CAM_CAM_CONV_YUV_MODE decides the YUV mode of input data. 0: input data is in YUV422 format. 1: input data is in YUV420 format. 2: input data is in YUV411 format. In RGB-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format.
pub type CAM_CONV_YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CAM_CONV_PROTOCOL_MODE` reader - 0: BT601. 1: BT709.
pub type CAM_CONV_PROTOCOL_MODE_R = crate::BitReader;
///Field `CAM_CONV_PROTOCOL_MODE` writer - 0: BT601. 1: BT709.
pub type CAM_CONV_PROTOCOL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_DATA_OUT_MODE` reader - Configure color range for output data. 0: limited color range. 1: full color range.
pub type CAM_CONV_DATA_OUT_MODE_R = crate::BitReader;
///Field `CAM_CONV_DATA_OUT_MODE` writer - Configure color range for output data. 0: limited color range. 1: full color range.
pub type CAM_CONV_DATA_OUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_DATA_IN_MODE` reader - Configure color range for input data. 0: limited color range. 1: full color range.
pub type CAM_CONV_DATA_IN_MODE_R = crate::BitReader;
///Field `CAM_CONV_DATA_IN_MODE` writer - Configure color range for input data. 0: limited color range. 1: full color range.
pub type CAM_CONV_DATA_IN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_MODE_8BITS_ON` reader - 0: 16-bit mode. 1: 8-bit mode.
pub type CAM_CONV_MODE_8BITS_ON_R = crate::BitReader;
///Field `CAM_CONV_MODE_8BITS_ON` writer - 0: 16-bit mode. 1: 8-bit mode.
pub type CAM_CONV_MODE_8BITS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_TRANS_MODE` reader - 0: converted to RGB format. 1: converted to YUV format.
pub type CAM_CONV_TRANS_MODE_R = crate::BitReader;
///Field `CAM_CONV_TRANS_MODE` writer - 0: converted to RGB format. 1: converted to YUV format.
pub type CAM_CONV_TRANS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAM_CONV_BYPASS` reader - 0: Bypass converter. 1: Enable converter.
pub type CAM_CONV_BYPASS_R = crate::BitReader;
///Field `CAM_CONV_BYPASS` writer - 0: Bypass converter. 1: Enable converter.
pub type CAM_CONV_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 21 - Swap every two 8-bit input data. 1: Enabled. 0: Disabled.
    #[inline(always)]
    pub fn cam_conv_8bits_data_inv(&self) -> CAM_CONV_8BITS_DATA_INV_R {
        CAM_CONV_8BITS_DATA_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - In YUV-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format. 3: disabled. To enable YUV-to-YUV mode, LCD_CAM_CAM_CONV_TRANS_MODE must be set to 1.
    #[inline(always)]
    pub fn cam_conv_yuv2yuv_mode(&self) -> CAM_CONV_YUV2YUV_MODE_R {
        CAM_CONV_YUV2YUV_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - In YUV-to-YUV mode and YUV-to-RGB mode, LCD_CAM_CAM_CONV_YUV_MODE decides the YUV mode of input data. 0: input data is in YUV422 format. 1: input data is in YUV420 format. 2: input data is in YUV411 format. In RGB-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format.
    #[inline(always)]
    pub fn cam_conv_yuv_mode(&self) -> CAM_CONV_YUV_MODE_R {
        CAM_CONV_YUV_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - 0: BT601. 1: BT709.
    #[inline(always)]
    pub fn cam_conv_protocol_mode(&self) -> CAM_CONV_PROTOCOL_MODE_R {
        CAM_CONV_PROTOCOL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Configure color range for output data. 0: limited color range. 1: full color range.
    #[inline(always)]
    pub fn cam_conv_data_out_mode(&self) -> CAM_CONV_DATA_OUT_MODE_R {
        CAM_CONV_DATA_OUT_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Configure color range for input data. 0: limited color range. 1: full color range.
    #[inline(always)]
    pub fn cam_conv_data_in_mode(&self) -> CAM_CONV_DATA_IN_MODE_R {
        CAM_CONV_DATA_IN_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - 0: 16-bit mode. 1: 8-bit mode.
    #[inline(always)]
    pub fn cam_conv_mode_8bits_on(&self) -> CAM_CONV_MODE_8BITS_ON_R {
        CAM_CONV_MODE_8BITS_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - 0: converted to RGB format. 1: converted to YUV format.
    #[inline(always)]
    pub fn cam_conv_trans_mode(&self) -> CAM_CONV_TRANS_MODE_R {
        CAM_CONV_TRANS_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    pub fn cam_conv_bypass(&self) -> CAM_CONV_BYPASS_R {
        CAM_CONV_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_RGB_YUV")
            .field("cam_conv_8bits_data_inv", &self.cam_conv_8bits_data_inv())
            .field("cam_conv_yuv2yuv_mode", &self.cam_conv_yuv2yuv_mode())
            .field("cam_conv_yuv_mode", &self.cam_conv_yuv_mode())
            .field("cam_conv_protocol_mode", &self.cam_conv_protocol_mode())
            .field("cam_conv_data_out_mode", &self.cam_conv_data_out_mode())
            .field("cam_conv_data_in_mode", &self.cam_conv_data_in_mode())
            .field("cam_conv_mode_8bits_on", &self.cam_conv_mode_8bits_on())
            .field("cam_conv_trans_mode", &self.cam_conv_trans_mode())
            .field("cam_conv_bypass", &self.cam_conv_bypass())
            .finish()
    }
}
impl W {
    ///Bit 21 - Swap every two 8-bit input data. 1: Enabled. 0: Disabled.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_8bits_data_inv(&mut self) -> CAM_CONV_8BITS_DATA_INV_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_8BITS_DATA_INV_W::new(self, 21)
    }
    ///Bits 22:23 - In YUV-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format. 3: disabled. To enable YUV-to-YUV mode, LCD_CAM_CAM_CONV_TRANS_MODE must be set to 1.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv2yuv_mode(&mut self) -> CAM_CONV_YUV2YUV_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_YUV2YUV_MODE_W::new(self, 22)
    }
    ///Bits 24:25 - In YUV-to-YUV mode and YUV-to-RGB mode, LCD_CAM_CAM_CONV_YUV_MODE decides the YUV mode of input data. 0: input data is in YUV422 format. 1: input data is in YUV420 format. 2: input data is in YUV411 format. In RGB-to-YUV mode, 0: data is converted to YUV422 format. 1: data is converted to YUV420 format. 2: data is converted to YUV411 format.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv_mode(&mut self) -> CAM_CONV_YUV_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_YUV_MODE_W::new(self, 24)
    }
    ///Bit 26 - 0: BT601. 1: BT709.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_protocol_mode(&mut self) -> CAM_CONV_PROTOCOL_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_PROTOCOL_MODE_W::new(self, 26)
    }
    ///Bit 27 - Configure color range for output data. 0: limited color range. 1: full color range.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_out_mode(&mut self) -> CAM_CONV_DATA_OUT_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_DATA_OUT_MODE_W::new(self, 27)
    }
    ///Bit 28 - Configure color range for input data. 0: limited color range. 1: full color range.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_in_mode(&mut self) -> CAM_CONV_DATA_IN_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_DATA_IN_MODE_W::new(self, 28)
    }
    ///Bit 29 - 0: 16-bit mode. 1: 8-bit mode.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_mode_8bits_on(&mut self) -> CAM_CONV_MODE_8BITS_ON_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_MODE_8BITS_ON_W::new(self, 29)
    }
    ///Bit 30 - 0: converted to RGB format. 1: converted to YUV format.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_trans_mode(&mut self) -> CAM_CONV_TRANS_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_TRANS_MODE_W::new(self, 30)
    }
    ///Bit 31 - 0: Bypass converter. 1: Enable converter.
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_bypass(&mut self) -> CAM_CONV_BYPASS_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_BYPASS_W::new(self, 31)
    }
}
/**Camera data format conversion register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_rgb_yuv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CAM_RGB_YUV_SPEC;
impl crate::RegisterSpec for CAM_RGB_YUV_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cam_rgb_yuv::R`](R) reader structure
impl crate::Readable for CAM_RGB_YUV_SPEC {}
///`write(|w| ..)` method takes [`cam_rgb_yuv::W`](W) writer structure
impl crate::Writable for CAM_RGB_YUV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAM_RGB_YUV to value 0
impl crate::Resettable for CAM_RGB_YUV_SPEC {
    const RESET_VALUE: u32 = 0;
}
