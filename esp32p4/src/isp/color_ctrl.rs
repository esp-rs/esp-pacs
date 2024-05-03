#[doc = "Register `COLOR_CTRL` reader"]
pub type R = crate::R<COLOR_CTRL_SPEC>;
#[doc = "Register `COLOR_CTRL` writer"]
pub type W = crate::W<COLOR_CTRL_SPEC>;
#[doc = "Field `COLOR_SATURATION` reader - this field configures the color saturation value"]
pub type COLOR_SATURATION_R = crate::FieldReader;
#[doc = "Field `COLOR_SATURATION` writer - this field configures the color saturation value"]
pub type COLOR_SATURATION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_HUE` reader - this field configures the color hue angle"]
pub type COLOR_HUE_R = crate::FieldReader;
#[doc = "Field `COLOR_HUE` writer - this field configures the color hue angle"]
pub type COLOR_HUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_CONTRAST` reader - this field configures the color contrast value"]
pub type COLOR_CONTRAST_R = crate::FieldReader;
#[doc = "Field `COLOR_CONTRAST` writer - this field configures the color contrast value"]
pub type COLOR_CONTRAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BRIGHTNESS` reader - this field configures the color brightness value, signed 2's complement"]
pub type COLOR_BRIGHTNESS_R = crate::FieldReader;
#[doc = "Field `COLOR_BRIGHTNESS` writer - this field configures the color brightness value, signed 2's complement"]
pub type COLOR_BRIGHTNESS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the color saturation value"]
    #[inline(always)]
    pub fn color_saturation(&self) -> COLOR_SATURATION_R {
        COLOR_SATURATION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the color hue angle"]
    #[inline(always)]
    pub fn color_hue(&self) -> COLOR_HUE_R {
        COLOR_HUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the color contrast value"]
    #[inline(always)]
    pub fn color_contrast(&self) -> COLOR_CONTRAST_R {
        COLOR_CONTRAST_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the color brightness value, signed 2's complement"]
    #[inline(always)]
    pub fn color_brightness(&self) -> COLOR_BRIGHTNESS_R {
        COLOR_BRIGHTNESS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COLOR_CTRL")
            .field("color_saturation", &self.color_saturation().bits())
            .field("color_hue", &self.color_hue().bits())
            .field("color_contrast", &self.color_contrast().bits())
            .field("color_brightness", &self.color_brightness().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COLOR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the color saturation value"]
    #[inline(always)]
    #[must_use]
    pub fn color_saturation(&mut self) -> COLOR_SATURATION_W<COLOR_CTRL_SPEC> {
        COLOR_SATURATION_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the color hue angle"]
    #[inline(always)]
    #[must_use]
    pub fn color_hue(&mut self) -> COLOR_HUE_W<COLOR_CTRL_SPEC> {
        COLOR_HUE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the color contrast value"]
    #[inline(always)]
    #[must_use]
    pub fn color_contrast(&mut self) -> COLOR_CONTRAST_W<COLOR_CTRL_SPEC> {
        COLOR_CONTRAST_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the color brightness value, signed 2's complement"]
    #[inline(always)]
    #[must_use]
    pub fn color_brightness(&mut self) -> COLOR_BRIGHTNESS_W<COLOR_CTRL_SPEC> {
        COLOR_BRIGHTNESS_W::new(self, 24)
    }
}
#[doc = "color control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`color_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`color_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COLOR_CTRL_SPEC;
impl crate::RegisterSpec for COLOR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`color_ctrl::R`](R) reader structure"]
impl crate::Readable for COLOR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`color_ctrl::W`](W) writer structure"]
impl crate::Writable for COLOR_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COLOR_CTRL to value 0x0080_0080"]
impl crate::Resettable for COLOR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0080_0080;
}
