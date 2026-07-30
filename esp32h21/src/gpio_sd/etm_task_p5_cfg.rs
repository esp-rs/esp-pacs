#[doc = "Register `ETM_TASK_P5_CFG` reader"]
pub type R = crate::R<ETM_TASK_P5_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P5_CFG` writer"]
pub type W = crate::W<ETM_TASK_P5_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(_EXT_ETM_TASK_GPIO25)` reader - GPIO choose a etm task channel."]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(_EXT_ETM_TASK_GPIO25)` writer - GPIO choose a etm task channel."]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(_EXT_ETM_TASK_GPIO25)` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(_EXT_ETM_TASK_GPIO25)` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO25_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 0)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..1).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 0)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO25_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 0 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..1).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 0 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P5_CFG")
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
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO25_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<'_, ETM_TASK_P5_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        GPIO_SEL_W::new(self, n * 0)
    }
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P5_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO25_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<'_, ETM_TASK_P5_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        GPIO_EN_W::new(self, n * 0 + 5)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio25_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P5_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p5_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p5_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P5_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P5_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p5_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P5_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p5_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P5_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P5_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P5_CFG_SPEC {}
