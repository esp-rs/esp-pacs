#[doc = "Register `HP_GPIO_O_HOLD_CTRL1` reader"]
pub type R = crate::R<HP_GPIO_O_HOLD_CTRL1_SPEC>;
#[doc = "Register `HP_GPIO_O_HOLD_CTRL1` writer"]
pub type W = crate::W<HP_GPIO_O_HOLD_CTRL1_SPEC>;
#[doc = "Field `HP_GPIO_0_HOLD_CTRL1` reader - "]
pub type HP_GPIO_0_HOLD_CTRL1_R = crate::FieldReader<u32>;
#[doc = "Field `HP_GPIO_0_HOLD_CTRL1` writer - "]
pub type HP_GPIO_0_HOLD_CTRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn hp_gpio_0_hold_ctrl1(&self) -> HP_GPIO_0_HOLD_CTRL1_R {
        HP_GPIO_0_HOLD_CTRL1_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_GPIO_O_HOLD_CTRL1")
            .field("hp_gpio_0_hold_ctrl1", &self.hp_gpio_0_hold_ctrl1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn hp_gpio_0_hold_ctrl1(
        &mut self,
    ) -> HP_GPIO_0_HOLD_CTRL1_W<'_, HP_GPIO_O_HOLD_CTRL1_SPEC> {
        HP_GPIO_0_HOLD_CTRL1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_o_hold_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_o_hold_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_GPIO_O_HOLD_CTRL1_SPEC;
impl crate::RegisterSpec for HP_GPIO_O_HOLD_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_gpio_o_hold_ctrl1::R`](R) reader structure"]
impl crate::Readable for HP_GPIO_O_HOLD_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_gpio_o_hold_ctrl1::W`](W) writer structure"]
impl crate::Writable for HP_GPIO_O_HOLD_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_GPIO_O_HOLD_CTRL1 to value 0"]
impl crate::Resettable for HP_GPIO_O_HOLD_CTRL1_SPEC {}
