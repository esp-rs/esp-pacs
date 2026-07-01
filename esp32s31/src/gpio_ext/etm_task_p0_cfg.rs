#[doc = "Register `ETM_TASK_P0_CFG` reader"]
pub type R = crate::R<ETM_TASK_P0_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P0_CFG` writer"]
pub type W = crate::W<ETM_TASK_P0_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(0-4)` reader - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(0-4)` writer - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(0-4)` reader - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(0-4)` writer - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Configures to select an ETM task channel for GPIO(0-4).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO0_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures to select an ETM task channel for GPIO(0-4).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..5).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio0_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO1.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio1_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO2.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio2_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO3.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio3_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO4.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio4_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Configures whether or not to enable GPIO(0-4) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO0_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures whether or not to enable GPIO(0-4) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..5).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO0 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio0_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO1 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio1_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO2 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio2_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO3 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio3_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO4 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio4_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P0_CFG")
            .field("gpio0_sel", &self.gpio0_sel())
            .field("gpio1_sel", &self.gpio1_sel())
            .field("gpio2_sel", &self.gpio2_sel())
            .field("gpio3_sel", &self.gpio3_sel())
            .field("gpio4_sel", &self.gpio4_sel())
            .field("gpio0_en", &self.gpio0_en())
            .field("gpio1_en", &self.gpio1_en())
            .field("gpio2_en", &self.gpio2_en())
            .field("gpio3_en", &self.gpio3_en())
            .field("gpio4_en", &self.gpio4_en())
            .finish()
    }
}
impl W {
    #[doc = "Configures to select an ETM task channel for GPIO(0-4).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO0_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_W::new(self, n * 6)
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO0.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio0_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO1.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio1_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO2.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio2_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO3.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio3_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO4.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio4_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_SEL_W::new(self, 24)
    }
    #[doc = "Configures whether or not to enable GPIO(0-4) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO0_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_W::new(self, n * 6 + 5)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO0 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio0_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO1 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio1_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO2 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio2_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_EN_W::new(self, 17)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO3 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio3_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO4 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio4_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P0_CFG_SPEC> {
        GPIO_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 0 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P0_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p0_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P0_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p0_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P0_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P0_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P0_CFG_SPEC {}
