#[doc = "Register `GPIO_EXT_ETM_TASK_P5_CFG` reader"]
pub type R = crate::R<GPIO_EXT_ETM_TASK_P5_CFG_SPEC>;
#[doc = "Register `GPIO_EXT_ETM_TASK_P5_CFG` writer"]
pub type W = crate::W<GPIO_EXT_ETM_TASK_P5_CFG_SPEC>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO25_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO25_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO25_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO25_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO25_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO25_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO25_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO25_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO25_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO25_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_en(&self) -> GPIO_EXT_ETM_TASK_GPIO25_EN_R {
        GPIO_EXT_ETM_TASK_GPIO25_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_ETM_TASK_P5_CFG")
            .field(
                "gpio_ext_etm_task_gpio25_sel",
                &self.gpio_ext_etm_task_gpio25_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio25_en",
                &self.gpio_ext_etm_task_gpio25_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO25_SEL_W<'_, GPIO_EXT_ETM_TASK_P5_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO25_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO25_EN_W<'_, GPIO_EXT_ETM_TASK_P5_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO25_EN_W::new(self, 5)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p5_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p5_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_ETM_TASK_P5_CFG_SPEC;
impl crate::RegisterSpec for GPIO_EXT_ETM_TASK_P5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_etm_task_p5_cfg::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_ETM_TASK_P5_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_etm_task_p5_cfg::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_ETM_TASK_P5_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_ETM_TASK_P5_CFG to value 0"]
impl crate::Resettable for GPIO_EXT_ETM_TASK_P5_CFG_SPEC {}
