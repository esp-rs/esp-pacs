#[doc = "Register `BLEND_COLOR_MODE` reader"]
pub type R = crate::R<BLEND_COLOR_MODE_SPEC>;
#[doc = "Register `BLEND_COLOR_MODE` writer"]
pub type W = crate::W<BLEND_COLOR_MODE_SPEC>;
#[doc = "Field `BLEND0_RX_CM` reader - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BLEND0_RX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND0_RX_CM` writer - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BLEND0_RX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND1_RX_CM` reader - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type BLEND1_RX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_CM` writer - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type BLEND1_RX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND_TX_CM` reader - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BLEND_TX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND_TX_CM` writer - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BLEND_TX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND0_RX_YUV_RANGE` reader - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
pub type BLEND0_RX_YUV_RANGE_R = crate::BitReader;
#[doc = "Field `BLEND0_RX_YUV_RANGE` writer - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
pub type BLEND0_RX_YUV_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_TX_YUV_RANGE` reader - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
pub type BLEND_TX_YUV_RANGE_R = crate::BitReader;
#[doc = "Field `BLEND_TX_YUV_RANGE` writer - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
pub type BLEND_TX_YUV_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_RX_YUV2RGB_PROTOCAL` reader - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
pub type BLEND0_RX_YUV2RGB_PROTOCAL_R = crate::BitReader;
#[doc = "Field `BLEND0_RX_YUV2RGB_PROTOCAL` writer - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
pub type BLEND0_RX_YUV2RGB_PROTOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_TX_RGB2YUV_PROTOCAL` reader - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
pub type BLEND_TX_RGB2YUV_PROTOCAL_R = crate::BitReader;
#[doc = "Field `BLEND_TX_RGB2YUV_PROTOCAL` writer - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
pub type BLEND_TX_RGB2YUV_PROTOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_RX_YUV422_BYTE_ORDER` reader - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type BLEND0_RX_YUV422_BYTE_ORDER_R = crate::FieldReader;
#[doc = "Field `BLEND0_RX_YUV422_BYTE_ORDER` writer - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type BLEND0_RX_YUV422_BYTE_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend0_rx_cm(&self) -> BLEND0_RX_CM_R {
        BLEND0_RX_CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&self) -> BLEND1_RX_CM_R {
        BLEND1_RX_CM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend_tx_cm(&self) -> BLEND_TX_CM_R {
        BLEND_TX_CM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend0_rx_yuv_range(&self) -> BLEND0_RX_YUV_RANGE_R {
        BLEND0_RX_YUV_RANGE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend_tx_yuv_range(&self) -> BLEND_TX_YUV_RANGE_R {
        BLEND_TX_YUV_RANGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend0_rx_yuv2rgb_protocal(&self) -> BLEND0_RX_YUV2RGB_PROTOCAL_R {
        BLEND0_RX_YUV2RGB_PROTOCAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend_tx_rgb2yuv_protocal(&self) -> BLEND_TX_RGB2YUV_PROTOCAL_R {
        BLEND_TX_RGB2YUV_PROTOCAL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn blend0_rx_yuv422_byte_order(&self) -> BLEND0_RX_YUV422_BYTE_ORDER_R {
        BLEND0_RX_YUV422_BYTE_ORDER_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_COLOR_MODE")
            .field("blend0_rx_cm", &self.blend0_rx_cm())
            .field("blend1_rx_cm", &self.blend1_rx_cm())
            .field("blend_tx_cm", &self.blend_tx_cm())
            .field("blend0_rx_yuv_range", &self.blend0_rx_yuv_range())
            .field("blend_tx_yuv_range", &self.blend_tx_yuv_range())
            .field(
                "blend0_rx_yuv2rgb_protocal",
                &self.blend0_rx_yuv2rgb_protocal(),
            )
            .field(
                "blend_tx_rgb2yuv_protocal",
                &self.blend_tx_rgb2yuv_protocal(),
            )
            .field(
                "blend0_rx_yuv422_byte_order",
                &self.blend0_rx_yuv422_byte_order(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend0_rx_cm(&mut self) -> BLEND0_RX_CM_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND0_RX_CM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&mut self) -> BLEND1_RX_CM_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND1_RX_CM_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend_tx_cm(&mut self) -> BLEND_TX_CM_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND_TX_CM_W::new(self, 8)
    }
    #[doc = "Bit 12 - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend0_rx_yuv_range(&mut self) -> BLEND0_RX_YUV_RANGE_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND0_RX_YUV_RANGE_W::new(self, 12)
    }
    #[doc = "Bit 13 - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend_tx_yuv_range(&mut self) -> BLEND_TX_YUV_RANGE_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND_TX_YUV_RANGE_W::new(self, 13)
    }
    #[doc = "Bit 14 - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend0_rx_yuv2rgb_protocal(
        &mut self,
    ) -> BLEND0_RX_YUV2RGB_PROTOCAL_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND0_RX_YUV2RGB_PROTOCAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend_tx_rgb2yuv_protocal(
        &mut self,
    ) -> BLEND_TX_RGB2YUV_PROTOCAL_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND_TX_RGB2YUV_PROTOCAL_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn blend0_rx_yuv422_byte_order(
        &mut self,
    ) -> BLEND0_RX_YUV422_BYTE_ORDER_W<'_, BLEND_COLOR_MODE_SPEC> {
        BLEND0_RX_YUV422_BYTE_ORDER_W::new(self, 16)
    }
}
#[doc = "blending engine color mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_color_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_color_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_COLOR_MODE_SPEC;
impl crate::RegisterSpec for BLEND_COLOR_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_color_mode::R`](R) reader structure"]
impl crate::Readable for BLEND_COLOR_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blend_color_mode::W`](W) writer structure"]
impl crate::Writable for BLEND_COLOR_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_COLOR_MODE to value 0"]
impl crate::Resettable for BLEND_COLOR_MODE_SPEC {}
