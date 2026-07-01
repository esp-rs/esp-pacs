#[doc = "Register `ETM_TASK_P8_CFG` reader"]
pub type R = crate::R<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P8_CFG` writer"]
pub type W = crate::W<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(40-44)` reader - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(40-44)` writer - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(40-44)` reader - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(40-44)` writer - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Configures to select an ETM task channel for GPIO(40-44).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO40_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures to select an ETM task channel for GPIO(40-44).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..5).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio40_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio41_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio42_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio43_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio44_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Configures whether or not to enable GPIO(40-44) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO40_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures whether or not to enable GPIO(40-44) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..5).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio40_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio41_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio42_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio43_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio44_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P8_CFG")
            .field("gpio40_sel", &self.gpio40_sel())
            .field("gpio41_sel", &self.gpio41_sel())
            .field("gpio42_sel", &self.gpio42_sel())
            .field("gpio43_sel", &self.gpio43_sel())
            .field("gpio44_sel", &self.gpio44_sel())
            .field("gpio40_en", &self.gpio40_en())
            .field("gpio41_en", &self.gpio41_en())
            .field("gpio42_en", &self.gpio42_en())
            .field("gpio43_en", &self.gpio43_en())
            .field("gpio44_en", &self.gpio44_en())
            .finish()
    }
}
impl W {
    #[doc = "Configures to select an ETM task channel for GPIO(40-44).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO40_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_W::new(self, n * 6)
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio40_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio41_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio42_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio43_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio44_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 24)
    }
    #[doc = "Configures whether or not to enable GPIO(40-44) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO40_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_W::new(self, n * 6 + 5)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio40_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio41_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio42_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 17)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio43_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio44_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 8 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p8_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p8_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P8_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P8_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p8_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P8_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p8_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P8_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P8_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P8_CFG_SPEC {}
