#[doc = "Register `CAM_RGB_YUV` reader"]
pub struct R(crate::R<CAM_RGB_YUV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_RGB_YUV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_RGB_YUV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_RGB_YUV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_RGB_YUV` writer"]
pub struct W(crate::W<CAM_RGB_YUV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_RGB_YUV_SPEC>;
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
impl From<crate::W<CAM_RGB_YUV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_RGB_YUV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type CAM_CONV_8BITS_DATA_INV_R = crate::BitReader;
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type CAM_CONV_8BITS_DATA_INV_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CAM_CONV_YUV2YUV_MODE_R = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CAM_CONV_YUV2YUV_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_RGB_YUV_SPEC, 2, O>;
#[doc = "Field `CAM_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CAM_CONV_YUV_MODE_R = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CAM_CONV_YUV_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_RGB_YUV_SPEC, 2, O>;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type CAM_CONV_PROTOCOL_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type CAM_CONV_PROTOCOL_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CAM_CONV_DATA_OUT_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CAM_CONV_DATA_OUT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CAM_CONV_DATA_IN_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CAM_CONV_DATA_IN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type CAM_CONV_MODE_8BITS_ON_R = crate::BitReader;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type CAM_CONV_MODE_8BITS_ON_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type CAM_CONV_TRANS_MODE_R = crate::BitReader;
#[doc = "Field `CAM_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type CAM_CONV_TRANS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
#[doc = "Field `CAM_CONV_BYPASS` reader - 0: Bypass converter. 1: Enable converter."]
pub type CAM_CONV_BYPASS_R = crate::BitReader;
#[doc = "Field `CAM_CONV_BYPASS` writer - 0: Bypass converter. 1: Enable converter."]
pub type CAM_CONV_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, CAM_RGB_YUV_SPEC, O>;
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
    pub fn cam_conv_bypass(&self) -> CAM_CONV_BYPASS_R {
        CAM_CONV_BYPASS_R::new(((self.bits >> 31) & 1) != 0)
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
                "cam_conv_bypass",
                &format_args!("{}", self.cam_conv_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_RGB_YUV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_8bits_data_inv(&mut self) -> CAM_CONV_8BITS_DATA_INV_W<21> {
        CAM_CONV_8BITS_DATA_INV_W::new(self)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv2yuv_mode(&mut self) -> CAM_CONV_YUV2YUV_MODE_W<22> {
        CAM_CONV_YUV2YUV_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_yuv_mode(&mut self) -> CAM_CONV_YUV_MODE_W<24> {
        CAM_CONV_YUV_MODE_W::new(self)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_protocol_mode(&mut self) -> CAM_CONV_PROTOCOL_MODE_W<26> {
        CAM_CONV_PROTOCOL_MODE_W::new(self)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_out_mode(&mut self) -> CAM_CONV_DATA_OUT_MODE_W<27> {
        CAM_CONV_DATA_OUT_MODE_W::new(self)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_data_in_mode(&mut self) -> CAM_CONV_DATA_IN_MODE_W<28> {
        CAM_CONV_DATA_IN_MODE_W::new(self)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_mode_8bits_on(&mut self) -> CAM_CONV_MODE_8BITS_ON_W<29> {
        CAM_CONV_MODE_8BITS_ON_W::new(self)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_trans_mode(&mut self) -> CAM_CONV_TRANS_MODE_W<30> {
        CAM_CONV_TRANS_MODE_W::new(self)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    #[must_use]
    pub fn cam_conv_bypass(&mut self) -> CAM_CONV_BYPASS_W<31> {
        CAM_CONV_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Camera configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_rgb_yuv](index.html) module"]
pub struct CAM_RGB_YUV_SPEC;
impl crate::RegisterSpec for CAM_RGB_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_rgb_yuv::R](R) reader structure"]
impl crate::Readable for CAM_RGB_YUV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_rgb_yuv::W](W) writer structure"]
impl crate::Writable for CAM_RGB_YUV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for CAM_RGB_YUV_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c0_0000;
}
