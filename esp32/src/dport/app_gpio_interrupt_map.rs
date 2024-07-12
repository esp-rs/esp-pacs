#[doc = "Register `APP_GPIO_INTERRUPT_MAP` reader"]
pub type R = crate::R<APP_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = "Register `APP_GPIO_INTERRUPT_MAP` writer"]
pub type W = crate::W<APP_GPIO_INTERRUPT_MAP_SPEC>;
#[doc = "Field `APP_GPIO_INTERRUPT_APP_MAP` reader - "]
pub type APP_GPIO_INTERRUPT_APP_MAP_R = crate::FieldReader;
#[doc = "Field `APP_GPIO_INTERRUPT_APP_MAP` writer - "]
pub type APP_GPIO_INTERRUPT_APP_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_gpio_interrupt_app_map(&self) -> APP_GPIO_INTERRUPT_APP_MAP_R {
        APP_GPIO_INTERRUPT_APP_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_GPIO_INTERRUPT_MAP")
            .field(
                "app_gpio_interrupt_app_map",
                &self.app_gpio_interrupt_app_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn app_gpio_interrupt_app_map(
        &mut self,
    ) -> APP_GPIO_INTERRUPT_APP_MAP_W<APP_GPIO_INTERRUPT_MAP_SPEC> {
        APP_GPIO_INTERRUPT_APP_MAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_gpio_interrupt_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_gpio_interrupt_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_GPIO_INTERRUPT_MAP_SPEC;
impl crate::RegisterSpec for APP_GPIO_INTERRUPT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_gpio_interrupt_map::R`](R) reader structure"]
impl crate::Readable for APP_GPIO_INTERRUPT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_gpio_interrupt_map::W`](W) writer structure"]
impl crate::Writable for APP_GPIO_INTERRUPT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_GPIO_INTERRUPT_MAP to value 0x10"]
impl crate::Resettable for APP_GPIO_INTERRUPT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
