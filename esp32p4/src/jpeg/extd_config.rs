#[doc = "Register `EXTD_CONFIG` reader"]
pub type R = crate::R<EXTD_CONFIG_SPEC>;
#[doc = "Register `EXTD_CONFIG` writer"]
pub type W = crate::W<EXTD_CONFIG_SPEC>;
#[doc = "Field `EXTD_COLOR_SPACE_EN` reader - Configure whether to extend picture's color space\\\\0:disable\\\\1:enable"]
pub type EXTD_COLOR_SPACE_EN_R = crate::BitReader;
#[doc = "Field `EXTD_COLOR_SPACE_EN` writer - Configure whether to extend picture's color space\\\\0:disable\\\\1:enable"]
pub type EXTD_COLOR_SPACE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTD_COLOR_SPACE` reader - Configure extended picture's color space. Valid when JPEG_EXTD_COLOR_SPACE_EN configured to 1\\\\0:yuv444\\\\1:yuv420"]
pub type EXTD_COLOR_SPACE_R = crate::BitReader;
#[doc = "Field `EXTD_COLOR_SPACE` writer - Configure extended picture's color space. Valid when JPEG_EXTD_COLOR_SPACE_EN configured to 1\\\\0:yuv444\\\\1:yuv420"]
pub type EXTD_COLOR_SPACE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether to extend picture's color space\\\\0:disable\\\\1:enable"]
    #[inline(always)]
    pub fn extd_color_space_en(&self) -> EXTD_COLOR_SPACE_EN_R {
        EXTD_COLOR_SPACE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure extended picture's color space. Valid when JPEG_EXTD_COLOR_SPACE_EN configured to 1\\\\0:yuv444\\\\1:yuv420"]
    #[inline(always)]
    pub fn extd_color_space(&self) -> EXTD_COLOR_SPACE_R {
        EXTD_COLOR_SPACE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTD_CONFIG")
            .field("extd_color_space_en", &self.extd_color_space_en())
            .field("extd_color_space", &self.extd_color_space())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether to extend picture's color space\\\\0:disable\\\\1:enable"]
    #[inline(always)]
    pub fn extd_color_space_en(&mut self) -> EXTD_COLOR_SPACE_EN_W<'_, EXTD_CONFIG_SPEC> {
        EXTD_COLOR_SPACE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure extended picture's color space. Valid when JPEG_EXTD_COLOR_SPACE_EN configured to 1\\\\0:yuv444\\\\1:yuv420"]
    #[inline(always)]
    pub fn extd_color_space(&mut self) -> EXTD_COLOR_SPACE_W<'_, EXTD_CONFIG_SPEC> {
        EXTD_COLOR_SPACE_W::new(self, 1)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`extd_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extd_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTD_CONFIG_SPEC;
impl crate::RegisterSpec for EXTD_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extd_config::R`](R) reader structure"]
impl crate::Readable for EXTD_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extd_config::W`](W) writer structure"]
impl crate::Writable for EXTD_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTD_CONFIG to value 0"]
impl crate::Resettable for EXTD_CONFIG_SPEC {}
