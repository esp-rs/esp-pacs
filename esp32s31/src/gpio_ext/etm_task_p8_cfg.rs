#[doc = "Register `ETM_TASK_P8_CFG` reader"]
pub type R = crate::R<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P8_CFG` writer"]
pub type W = crate::W<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO40_SEL` reader - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO40_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO40_SEL` writer - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO40_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO40_EN` reader - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO40_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO40_EN` writer - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO40_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO41_SEL` reader - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO41_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO41_SEL` writer - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO41_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO41_EN` reader - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO41_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO41_EN` writer - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO41_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO42_SEL` reader - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO42_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO42_SEL` writer - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO42_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO42_EN` reader - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO42_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO42_EN` writer - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO42_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO43_SEL` reader - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO43_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO43_SEL` writer - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO43_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO43_EN` reader - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO43_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO43_EN` writer - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO43_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO44_SEL` reader - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO44_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO44_SEL` writer - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO44_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO44_EN` reader - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO44_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO44_EN` writer - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO44_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio40_sel(&self) -> ETM_TASK_GPIO40_SEL_R {
        ETM_TASK_GPIO40_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio40_en(&self) -> ETM_TASK_GPIO40_EN_R {
        ETM_TASK_GPIO40_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio41_sel(&self) -> ETM_TASK_GPIO41_SEL_R {
        ETM_TASK_GPIO41_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio41_en(&self) -> ETM_TASK_GPIO41_EN_R {
        ETM_TASK_GPIO41_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio42_sel(&self) -> ETM_TASK_GPIO42_SEL_R {
        ETM_TASK_GPIO42_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio42_en(&self) -> ETM_TASK_GPIO42_EN_R {
        ETM_TASK_GPIO42_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio43_sel(&self) -> ETM_TASK_GPIO43_SEL_R {
        ETM_TASK_GPIO43_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio43_en(&self) -> ETM_TASK_GPIO43_EN_R {
        ETM_TASK_GPIO43_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio44_sel(&self) -> ETM_TASK_GPIO44_SEL_R {
        ETM_TASK_GPIO44_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio44_en(&self) -> ETM_TASK_GPIO44_EN_R {
        ETM_TASK_GPIO44_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P8_CFG")
            .field("etm_task_gpio40_sel", &self.etm_task_gpio40_sel())
            .field("etm_task_gpio40_en", &self.etm_task_gpio40_en())
            .field("etm_task_gpio41_sel", &self.etm_task_gpio41_sel())
            .field("etm_task_gpio41_en", &self.etm_task_gpio41_en())
            .field("etm_task_gpio42_sel", &self.etm_task_gpio42_sel())
            .field("etm_task_gpio42_en", &self.etm_task_gpio42_en())
            .field("etm_task_gpio43_sel", &self.etm_task_gpio43_sel())
            .field("etm_task_gpio43_en", &self.etm_task_gpio43_en())
            .field("etm_task_gpio44_sel", &self.etm_task_gpio44_sel())
            .field("etm_task_gpio44_en", &self.etm_task_gpio44_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO40.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio40_sel(&mut self) -> ETM_TASK_GPIO40_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO40_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO40 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio40_en(&mut self) -> ETM_TASK_GPIO40_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO40_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO41.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio41_sel(&mut self) -> ETM_TASK_GPIO41_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO41_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO41 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio41_en(&mut self) -> ETM_TASK_GPIO41_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO41_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO42.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio42_sel(&mut self) -> ETM_TASK_GPIO42_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO42_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO42 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio42_en(&mut self) -> ETM_TASK_GPIO42_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO42_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO43.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio43_sel(&mut self) -> ETM_TASK_GPIO43_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO43_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO43 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio43_en(&mut self) -> ETM_TASK_GPIO43_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO43_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO44.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio44_sel(&mut self) -> ETM_TASK_GPIO44_SEL_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO44_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO44 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio44_en(&mut self) -> ETM_TASK_GPIO44_EN_W<'_, ETM_TASK_P8_CFG_SPEC> {
        ETM_TASK_GPIO44_EN_W::new(self, 29)
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
