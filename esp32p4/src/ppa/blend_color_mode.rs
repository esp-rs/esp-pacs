#[doc = "Register `BLEND_COLOR_MODE` reader"]
pub type R = crate::R<BLEND_COLOR_MODE_SPEC>;
#[doc = "Register `BLEND_COLOR_MODE` writer"]
pub type W = crate::W<BLEND_COLOR_MODE_SPEC>;
#[doc = "Field `BLEND0_RX_CM` reader - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4."]
pub type BLEND0_RX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND0_RX_CM` writer - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4."]
pub type BLEND0_RX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND1_RX_CM` reader - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type BLEND1_RX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND1_RX_CM` writer - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type BLEND1_RX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND_TX_CM` reader - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved.."]
pub type BLEND_TX_CM_R = crate::FieldReader;
#[doc = "Field `BLEND_TX_CM` writer - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved.."]
pub type BLEND_TX_CM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4."]
    #[inline(always)]
    pub fn blend0_rx_cm(&self) -> BLEND0_RX_CM_R {
        BLEND0_RX_CM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&self) -> BLEND1_RX_CM_R {
        BLEND1_RX_CM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved.."]
    #[inline(always)]
    pub fn blend_tx_cm(&self) -> BLEND_TX_CM_R {
        BLEND_TX_CM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_COLOR_MODE")
            .field("blend0_rx_cm", &self.blend0_rx_cm())
            .field("blend1_rx_cm", &self.blend1_rx_cm())
            .field("blend_tx_cm", &self.blend_tx_cm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4."]
    #[inline(always)]
    pub fn blend0_rx_cm(&mut self) -> BLEND0_RX_CM_W<BLEND_COLOR_MODE_SPEC> {
        BLEND0_RX_CM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&mut self) -> BLEND1_RX_CM_W<BLEND_COLOR_MODE_SPEC> {
        BLEND1_RX_CM_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved.."]
    #[inline(always)]
    pub fn blend_tx_cm(&mut self) -> BLEND_TX_CM_W<BLEND_COLOR_MODE_SPEC> {
        BLEND_TX_CM_W::new(self, 8)
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
