#[doc = "Register `SR_COLOR_MODE` reader"]
pub type R = crate::R<SR_COLOR_MODE_SPEC>;
#[doc = "Register `SR_COLOR_MODE` writer"]
pub type W = crate::W<SR_COLOR_MODE_SPEC>;
#[doc = "Field `SR_RX_CM` reader - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
pub type SR_RX_CM_R = crate::FieldReader;
#[doc = "Field `SR_RX_CM` writer - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
pub type SR_RX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SR_TX_CM` reader - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
pub type SR_TX_CM_R = crate::FieldReader;
#[doc = "Field `SR_TX_CM` writer - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
pub type SR_TX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YUV_RX_RANGE` reader - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YUV_RX_RANGE_R = crate::BitReader;
#[doc = "Field `YUV_RX_RANGE` writer - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YUV_RX_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV_TX_RANGE` reader - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YUV_TX_RANGE_R = crate::BitReader;
#[doc = "Field `YUV_TX_RANGE` writer - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YUV_TX_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV2RGB_PROTOCAL` reader - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type YUV2RGB_PROTOCAL_R = crate::BitReader;
#[doc = "Field `YUV2RGB_PROTOCAL` writer - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type YUV2RGB_PROTOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB2YUV_PROTOCAL` reader - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type RGB2YUV_PROTOCAL_R = crate::BitReader;
#[doc = "Field `RGB2YUV_PROTOCAL` writer - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type RGB2YUV_PROTOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
    #[inline(always)]
    pub fn sr_rx_cm(&self) -> SR_RX_CM_R {
        SR_RX_CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
    #[inline(always)]
    pub fn sr_tx_cm(&self) -> SR_TX_CM_R {
        SR_TX_CM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_rx_range(&self) -> YUV_RX_RANGE_R {
        YUV_RX_RANGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_tx_range(&self) -> YUV_TX_RANGE_R {
        YUV_TX_RANGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn yuv2rgb_protocal(&self) -> YUV2RGB_PROTOCAL_R {
        YUV2RGB_PROTOCAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn rgb2yuv_protocal(&self) -> RGB2YUV_PROTOCAL_R {
        RGB2YUV_PROTOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_COLOR_MODE")
            .field("sr_rx_cm", &format_args!("{}", self.sr_rx_cm().bits()))
            .field("sr_tx_cm", &format_args!("{}", self.sr_tx_cm().bits()))
            .field(
                "yuv_rx_range",
                &format_args!("{}", self.yuv_rx_range().bit()),
            )
            .field(
                "yuv_tx_range",
                &format_args!("{}", self.yuv_tx_range().bit()),
            )
            .field(
                "yuv2rgb_protocal",
                &format_args!("{}", self.yuv2rgb_protocal().bit()),
            )
            .field(
                "rgb2yuv_protocal",
                &format_args!("{}", self.rgb2yuv_protocal().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_COLOR_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_cm(&mut self) -> SR_RX_CM_W<SR_COLOR_MODE_SPEC> {
        SR_RX_CM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn sr_tx_cm(&mut self) -> SR_TX_CM_W<SR_COLOR_MODE_SPEC> {
        SR_TX_CM_W::new(self, 4)
    }
    #[doc = "Bit 8 - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_rx_range(&mut self) -> YUV_RX_RANGE_W<SR_COLOR_MODE_SPEC> {
        YUV_RX_RANGE_W::new(self, 8)
    }
    #[doc = "Bit 9 - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_tx_range(&mut self) -> YUV_TX_RANGE_W<SR_COLOR_MODE_SPEC> {
        YUV_TX_RANGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_protocal(&mut self) -> YUV2RGB_PROTOCAL_W<SR_COLOR_MODE_SPEC> {
        YUV2RGB_PROTOCAL_W::new(self, 10)
    }
    #[doc = "Bit 11 - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_protocal(&mut self) -> RGB2YUV_PROTOCAL_W<SR_COLOR_MODE_SPEC> {
        RGB2YUV_PROTOCAL_W::new(self, 11)
    }
}
#[doc = "Scaling and rotating engine color mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_color_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_color_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_COLOR_MODE_SPEC;
impl crate::RegisterSpec for SR_COLOR_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_color_mode::R`](R) reader structure"]
impl crate::Readable for SR_COLOR_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr_color_mode::W`](W) writer structure"]
impl crate::Writable for SR_COLOR_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_COLOR_MODE to value 0"]
impl crate::Resettable for SR_COLOR_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
