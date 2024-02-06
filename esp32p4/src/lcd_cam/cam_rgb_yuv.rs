#[doc = "Register `CAM_RGB_YUV` reader"]
pub type R = crate::R<CAM_RGB_YUV_SPEC>;
#[doc = "Register `CAM_RGB_YUV` writer"]
pub type W = crate::W<CAM_RGB_YUV_SPEC>;
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type CAM_CONV_8BITS_DATA_INV_R = crate::BitReader;
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type CAM_CONV_8BITS_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CAM_CONV_YUV2YUV_MODE_R = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CAM_CONV_YUV2YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAM_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CAM_CONV_YUV_MODE_R = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CAM_CONV_YUV_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type CAM_CONV_PROTOCOL_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type CAM_CONV_PROTOCOL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CAM_CONV_DATA_OUT_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CAM_CONV_DATA_OUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CAM_CONV_DATA_IN_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CAM_CONV_DATA_IN_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type CAM_CONV_MODE_8BITS_ON_R = crate::BitReader;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type CAM_CONV_MODE_8BITS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type CAM_CONV_TRANS_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type CAM_CONV_TRANS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_ENABLE` reader - 0: Bypass converter. 1: Enable converter."]
pub type CAM_CONV_ENABLE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_ENABLE` writer - 0: Bypass converter. 1: Enable converter."]
pub type CAM_CONV_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn cam_conv_8bits_data_inv(&self) -> CAM_CONV_8BITS_DATA_INV_R {
        CAM_CONV_8BITS_DATA_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn cam_conv_yuv2yuv_mode(&self) -> CAM_CONV_YUV2YUV_MODE_R {
        CAM_CONV_YUV2YUV_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn cam_conv_yuv_mode(&self) -> CAM_CONV_YUV_MODE_R {
        CAM_CONV_YUV_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn cam_conv_protocol_mode(&self) -> CAM_CONV_PROTOCOL_MODE_R {
        CAM_CONV_PROTOCOL_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_out_mode(&self) -> CAM_CONV_DATA_OUT_MODE_R {
        CAM_CONV_DATA_OUT_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_in_mode(&self) -> CAM_CONV_DATA_IN_MODE_R {
        CAM_CONV_DATA_IN_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn cam_conv_mode_8bits_on(&self) -> CAM_CONV_MODE_8BITS_ON_R {
        CAM_CONV_MODE_8BITS_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn cam_conv_trans_mode(&self) -> CAM_CONV_TRANS_MODE_R {
        CAM_CONV_TRANS_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn cam_conv_enable(&self) -> CAM_CONV_ENABLE_R {
        CAM_CONV_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_RGB_YUV")
            .field(
                "cam_conv_8bits_data_inv",
                &format_args!("{}", self.cam_conv_8bits_data_inv().bit()),
            )
            .field(
                "cam_conv_yuv2yuv_mode",
                &format_args!("{}", self.cam_conv_yuv2yuv_mode().bits()),
            )
            .field(
                "cam_conv_yuv_mode",
                &format_args!("{}", self.cam_conv_yuv_mode().bits()),
            )
            .field(
                "cam_conv_protocol_mode",
                &format_args!("{}", self.cam_conv_protocol_mode().bit()),
            )
            .field(
                "cam_conv_data_out_mode",
                &format_args!("{}", self.cam_conv_data_out_mode().bit()),
            )
            .field(
                "cam_conv_data_in_mode",
                &format_args!("{}", self.cam_conv_data_in_mode().bit()),
            )
            .field(
                "cam_conv_mode_8bits_on",
                &format_args!("{}", self.cam_conv_mode_8bits_on().bit()),
            )
            .field(
                "cam_conv_trans_mode",
                &format_args!("{}", self.cam_conv_trans_mode().bit()),
            )
            .field(
                "cam_conv_enable",
                &format_args!("{}", self.cam_conv_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_RGB_YUV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 21 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_8bits_data_inv(&mut self) -> CAM_CONV_8BITS_DATA_INV_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_8BITS_DATA_INV_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv2yuv_mode(&mut self) -> CAM_CONV_YUV2YUV_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_YUV2YUV_MODE_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv_mode(&mut self) -> CAM_CONV_YUV_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_YUV_MODE_W::new(self, 24)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_protocol_mode(&mut self) -> CAM_CONV_PROTOCOL_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_PROTOCOL_MODE_W::new(self, 26)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_out_mode(&mut self) -> CAM_CONV_DATA_OUT_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_DATA_OUT_MODE_W::new(self, 27)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_in_mode(&mut self) -> CAM_CONV_DATA_IN_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_DATA_IN_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_mode_8bits_on(&mut self) -> CAM_CONV_MODE_8BITS_ON_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_MODE_8BITS_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_trans_mode(&mut self) -> CAM_CONV_TRANS_MODE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_TRANS_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_enable(&mut self) -> CAM_CONV_ENABLE_W<CAM_RGB_YUV_SPEC> {
        CAM_CONV_ENABLE_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAM YUV/RGB converter configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_rgb_yuv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_RGB_YUV_SPEC;
impl crate::RegisterSpec for CAM_RGB_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_rgb_yuv::R`](R) reader structure"]
impl crate::Readable for CAM_RGB_YUV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_rgb_yuv::W`](W) writer structure"]
impl crate::Writable for CAM_RGB_YUV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAM_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for CAM_RGB_YUV_SPEC {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
