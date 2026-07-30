#[doc = "Register `GPIO_EXT_SIGMADELTA_MISC` reader"]
pub type R = crate::R<GPIO_EXT_SIGMADELTA_MISC_SPEC>;
#[doc = "Register `GPIO_EXT_SIGMADELTA_MISC` writer"]
pub type W = crate::W<GPIO_EXT_SIGMADELTA_MISC_SPEC>;
#[doc = "Field `GPIO_EXT_SIGMADELTA_CLK_EN` reader - Clock enable bit of sigma delta modulation."]
pub type GPIO_EXT_SIGMADELTA_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_SIGMADELTA_CLK_EN` writer - Clock enable bit of sigma delta modulation."]
pub type GPIO_EXT_SIGMADELTA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    pub fn gpio_ext_sigmadelta_clk_en(&self) -> GPIO_EXT_SIGMADELTA_CLK_EN_R {
        GPIO_EXT_SIGMADELTA_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_SIGMADELTA_MISC")
            .field(
                "gpio_ext_sigmadelta_clk_en",
                &self.gpio_ext_sigmadelta_clk_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    pub fn gpio_ext_sigmadelta_clk_en(
        &mut self,
    ) -> GPIO_EXT_SIGMADELTA_CLK_EN_W<'_, GPIO_EXT_SIGMADELTA_MISC_SPEC> {
        GPIO_EXT_SIGMADELTA_CLK_EN_W::new(self, 0)
    }
}
#[doc = "MISC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_sigmadelta_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_sigmadelta_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_SIGMADELTA_MISC_SPEC;
impl crate::RegisterSpec for GPIO_EXT_SIGMADELTA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_sigmadelta_misc::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_SIGMADELTA_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_sigmadelta_misc::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_SIGMADELTA_MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_SIGMADELTA_MISC to value 0"]
impl crate::Resettable for GPIO_EXT_SIGMADELTA_MISC_SPEC {}
