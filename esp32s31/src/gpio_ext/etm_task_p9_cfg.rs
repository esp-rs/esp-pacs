#[doc = "Register `ETM_TASK_P9_CFG` reader"]
pub type R = crate::R<ETM_TASK_P9_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P9_CFG` writer"]
pub type W = crate::W<ETM_TASK_P9_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(45-49)` reader - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(45-49)` writer - Configures to select an ETM task channel for GPIO%s.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(45-49)` reader - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(45-49)` writer - Configures whether or not to enable GPIO%s to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Configures to select an ETM task channel for GPIO(45-49).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO45_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures to select an ETM task channel for GPIO(45-49).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..5).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio45_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio46_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio47_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio48_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio49_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Configures whether or not to enable GPIO(45-49) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO45_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures whether or not to enable GPIO(45-49) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..5).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio45_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio46_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio47_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio48_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio49_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P9_CFG")
            .field("gpio45_sel", &self.gpio45_sel())
            .field("gpio46_sel", &self.gpio46_sel())
            .field("gpio47_sel", &self.gpio47_sel())
            .field("gpio48_sel", &self.gpio48_sel())
            .field("gpio49_sel", &self.gpio49_sel())
            .field("gpio45_en", &self.gpio45_en())
            .field("gpio46_en", &self.gpio46_en())
            .field("gpio47_en", &self.gpio47_en())
            .field("gpio48_en", &self.gpio48_en())
            .field("gpio49_en", &self.gpio49_en())
            .finish()
    }
}
impl W {
    #[doc = "Configures to select an ETM task channel for GPIO(45-49).\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO45_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_W::new(self, n * 6)
    }
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio45_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio46_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio47_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio48_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn gpio49_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_SEL_W::new(self, 24)
    }
    #[doc = "Configures whether or not to enable GPIO(45-49) to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO45_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_W::new(self, n * 6 + 5)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio45_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio46_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio47_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_EN_W::new(self, 17)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio48_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn gpio49_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        GPIO_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 9 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p9_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p9_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P9_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P9_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p9_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P9_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p9_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P9_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P9_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P9_CFG_SPEC {}
