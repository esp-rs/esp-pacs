#[doc = "Register `HP_GPIO_O_HOLD_CTRL0` reader"]
pub type R = crate::R<HP_GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "Register `HP_GPIO_O_HOLD_CTRL0` writer"]
pub type W = crate::W<HP_GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "Field `HP_GPIO_0_HOLD_CTRL0` reader - "]
pub type HP_GPIO_0_HOLD_CTRL0_R = crate::FieldReader<u32>;
#[doc = "Field `HP_GPIO_0_HOLD_CTRL0` writer - "]
pub type HP_GPIO_0_HOLD_CTRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hp_gpio_0_hold_ctrl0(&self) -> HP_GPIO_0_HOLD_CTRL0_R {
        HP_GPIO_0_HOLD_CTRL0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_GPIO_O_HOLD_CTRL0")
            .field("hp_gpio_0_hold_ctrl0", &self.hp_gpio_0_hold_ctrl0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hp_gpio_0_hold_ctrl0(
        &mut self,
    ) -> HP_GPIO_0_HOLD_CTRL0_W<'_, HP_GPIO_O_HOLD_CTRL0_SPEC> {
        HP_GPIO_0_HOLD_CTRL0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_o_hold_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_GPIO_O_HOLD_CTRL0_SPEC;
impl crate::RegisterSpec for HP_GPIO_O_HOLD_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_gpio_o_hold_ctrl0::R`](R) reader structure"]
impl crate::Readable for HP_GPIO_O_HOLD_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_gpio_o_hold_ctrl0::W`](W) writer structure"]
impl crate::Writable for HP_GPIO_O_HOLD_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_GPIO_O_HOLD_CTRL0 to value 0"]
impl crate::Resettable for HP_GPIO_O_HOLD_CTRL0_SPEC {}
