#[doc = "Register `GPIO_EXT_ETM_TASK_P4_CFG` reader"]
pub type R = crate::R<GPIO_EXT_ETM_TASK_P4_CFG_SPEC>;
#[doc = "Register `GPIO_EXT_ETM_TASK_P4_CFG` writer"]
pub type W = crate::W<GPIO_EXT_ETM_TASK_P4_CFG_SPEC>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO20_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO20_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO20_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO20_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO20_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO20_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO20_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO20_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO21_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO21_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO21_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO21_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO21_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO21_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO21_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO21_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO22_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO22_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO22_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO22_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO22_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO22_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO22_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO22_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO23_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO23_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO23_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO23_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO23_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO23_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO23_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO23_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO24_SEL` reader - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO24_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO24_SEL` writer - GPIO choose a etm task channel."]
pub type GPIO_EXT_ETM_TASK_GPIO24_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO24_EN` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO24_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_ETM_TASK_GPIO24_EN` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EXT_ETM_TASK_GPIO24_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio20_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO20_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO20_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio20_en(&self) -> GPIO_EXT_ETM_TASK_GPIO20_EN_R {
        GPIO_EXT_ETM_TASK_GPIO20_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio21_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO21_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO21_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio21_en(&self) -> GPIO_EXT_ETM_TASK_GPIO21_EN_R {
        GPIO_EXT_ETM_TASK_GPIO21_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio22_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO22_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO22_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio22_en(&self) -> GPIO_EXT_ETM_TASK_GPIO22_EN_R {
        GPIO_EXT_ETM_TASK_GPIO22_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio23_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO23_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO23_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio23_en(&self) -> GPIO_EXT_ETM_TASK_GPIO23_EN_R {
        GPIO_EXT_ETM_TASK_GPIO23_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio24_sel(&self) -> GPIO_EXT_ETM_TASK_GPIO24_SEL_R {
        GPIO_EXT_ETM_TASK_GPIO24_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio24_en(&self) -> GPIO_EXT_ETM_TASK_GPIO24_EN_R {
        GPIO_EXT_ETM_TASK_GPIO24_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_ETM_TASK_P4_CFG")
            .field(
                "gpio_ext_etm_task_gpio20_sel",
                &self.gpio_ext_etm_task_gpio20_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio20_en",
                &self.gpio_ext_etm_task_gpio20_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio21_sel",
                &self.gpio_ext_etm_task_gpio21_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio21_en",
                &self.gpio_ext_etm_task_gpio21_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio22_sel",
                &self.gpio_ext_etm_task_gpio22_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio22_en",
                &self.gpio_ext_etm_task_gpio22_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio23_sel",
                &self.gpio_ext_etm_task_gpio23_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio23_en",
                &self.gpio_ext_etm_task_gpio23_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio24_sel",
                &self.gpio_ext_etm_task_gpio24_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio24_en",
                &self.gpio_ext_etm_task_gpio24_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio20_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO20_SEL_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO20_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio20_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO20_EN_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO20_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio21_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO21_SEL_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO21_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio21_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO21_EN_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO21_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio22_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO22_SEL_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO22_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio22_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO22_EN_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO22_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio23_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO23_SEL_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO23_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio23_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO23_EN_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO23_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio24_sel(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO24_SEL_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO24_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio24_en(
        &mut self,
    ) -> GPIO_EXT_ETM_TASK_GPIO24_EN_W<'_, GPIO_EXT_ETM_TASK_P4_CFG_SPEC> {
        GPIO_EXT_ETM_TASK_GPIO24_EN_W::new(self, 29)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_etm_task_p4_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_etm_task_p4_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_ETM_TASK_P4_CFG_SPEC;
impl crate::RegisterSpec for GPIO_EXT_ETM_TASK_P4_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_etm_task_p4_cfg::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_ETM_TASK_P4_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_etm_task_p4_cfg::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_ETM_TASK_P4_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_ETM_TASK_P4_CFG to value 0"]
impl crate::Resettable for GPIO_EXT_ETM_TASK_P4_CFG_SPEC {}
