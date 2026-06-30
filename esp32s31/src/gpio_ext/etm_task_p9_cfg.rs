#[doc = "Register `ETM_TASK_P9_CFG` reader"]
pub type R = crate::R<ETM_TASK_P9_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P9_CFG` writer"]
pub type W = crate::W<ETM_TASK_P9_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO45_SEL` reader - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO45_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO45_SEL` writer - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO45_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO45_EN` reader - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO45_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO45_EN` writer - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO45_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO46_SEL` reader - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO46_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO46_SEL` writer - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO46_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO46_EN` reader - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO46_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO46_EN` writer - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO46_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO47_SEL` reader - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO47_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO47_SEL` writer - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO47_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO47_EN` reader - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO47_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO47_EN` writer - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO47_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO48_SEL` reader - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO48_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO48_SEL` writer - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO48_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO48_EN` reader - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO48_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO48_EN` writer - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO48_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO49_SEL` reader - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO49_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO49_SEL` writer - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO49_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO49_EN` reader - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO49_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO49_EN` writer - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO49_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio45_sel(&self) -> ETM_TASK_GPIO45_SEL_R {
        ETM_TASK_GPIO45_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio45_en(&self) -> ETM_TASK_GPIO45_EN_R {
        ETM_TASK_GPIO45_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio46_sel(&self) -> ETM_TASK_GPIO46_SEL_R {
        ETM_TASK_GPIO46_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio46_en(&self) -> ETM_TASK_GPIO46_EN_R {
        ETM_TASK_GPIO46_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio47_sel(&self) -> ETM_TASK_GPIO47_SEL_R {
        ETM_TASK_GPIO47_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio47_en(&self) -> ETM_TASK_GPIO47_EN_R {
        ETM_TASK_GPIO47_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio48_sel(&self) -> ETM_TASK_GPIO48_SEL_R {
        ETM_TASK_GPIO48_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio48_en(&self) -> ETM_TASK_GPIO48_EN_R {
        ETM_TASK_GPIO48_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio49_sel(&self) -> ETM_TASK_GPIO49_SEL_R {
        ETM_TASK_GPIO49_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio49_en(&self) -> ETM_TASK_GPIO49_EN_R {
        ETM_TASK_GPIO49_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P9_CFG")
            .field("etm_task_gpio45_sel", &self.etm_task_gpio45_sel())
            .field("etm_task_gpio45_en", &self.etm_task_gpio45_en())
            .field("etm_task_gpio46_sel", &self.etm_task_gpio46_sel())
            .field("etm_task_gpio46_en", &self.etm_task_gpio46_en())
            .field("etm_task_gpio47_sel", &self.etm_task_gpio47_sel())
            .field("etm_task_gpio47_en", &self.etm_task_gpio47_en())
            .field("etm_task_gpio48_sel", &self.etm_task_gpio48_sel())
            .field("etm_task_gpio48_en", &self.etm_task_gpio48_en())
            .field("etm_task_gpio49_sel", &self.etm_task_gpio49_sel())
            .field("etm_task_gpio49_en", &self.etm_task_gpio49_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO45.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio45_sel(&mut self) -> ETM_TASK_GPIO45_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO45_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO45 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio45_en(&mut self) -> ETM_TASK_GPIO45_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO45_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO46.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio46_sel(&mut self) -> ETM_TASK_GPIO46_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO46_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO46 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio46_en(&mut self) -> ETM_TASK_GPIO46_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO46_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO47.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio47_sel(&mut self) -> ETM_TASK_GPIO47_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO47_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO47 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio47_en(&mut self) -> ETM_TASK_GPIO47_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO47_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO48.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio48_sel(&mut self) -> ETM_TASK_GPIO48_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO48_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO48 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio48_en(&mut self) -> ETM_TASK_GPIO48_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO48_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO49.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio49_sel(&mut self) -> ETM_TASK_GPIO49_SEL_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO49_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO49 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio49_en(&mut self) -> ETM_TASK_GPIO49_EN_W<'_, ETM_TASK_P9_CFG_SPEC> {
        ETM_TASK_GPIO49_EN_W::new(self, 29)
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
