#[doc = "Register `COLOR_HUE_CTRL` reader"]
pub type R = crate::R<COLOR_HUE_CTRL_SPEC>;
#[doc = "Register `COLOR_HUE_CTRL` writer"]
pub type W = crate::W<COLOR_HUE_CTRL_SPEC>;
#[doc = "Field `COLOR_HUE_H` reader - Configures the color hue angle most bit"]
pub type COLOR_HUE_H_R = crate::BitReader;
#[doc = "Field `COLOR_HUE_H` writer - Configures the color hue angle most bit"]
pub type COLOR_HUE_H_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the color hue angle most bit"]
    #[inline(always)]
    pub fn color_hue_h(&self) -> COLOR_HUE_H_R {
        COLOR_HUE_H_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COLOR_HUE_CTRL")
            .field("color_hue_h", &self.color_hue_h())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the color hue angle most bit"]
    #[inline(always)]
    pub fn color_hue_h(&mut self) -> COLOR_HUE_H_W<'_, COLOR_HUE_CTRL_SPEC> {
        COLOR_HUE_H_W::new(self, 0)
    }
}
#[doc = "color control register\n\nYou can [`read`](crate::Reg::read) this register and get [`color_hue_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color_hue_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COLOR_HUE_CTRL_SPEC;
impl crate::RegisterSpec for COLOR_HUE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`color_hue_ctrl::R`](R) reader structure"]
impl crate::Readable for COLOR_HUE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`color_hue_ctrl::W`](W) writer structure"]
impl crate::Writable for COLOR_HUE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COLOR_HUE_CTRL to value 0"]
impl crate::Resettable for COLOR_HUE_CTRL_SPEC {}
