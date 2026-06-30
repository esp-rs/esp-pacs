#[doc = "Register `ETM_TASK_P12_CFG` reader"]
pub type R = crate::R<ETM_TASK_P12_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P12_CFG` writer"]
pub type W = crate::W<ETM_TASK_P12_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO60_SEL` reader - Configures to select an ETM task channel for GPIO60.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO60_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO60_SEL` writer - Configures to select an ETM task channel for GPIO60.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO60_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO60_EN` reader - Configures whether or not to enable GPIO60 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO60_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO60_EN` writer - Configures whether or not to enable GPIO60 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO60_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO61_SEL` reader - Configures to select an ETM task channel for GPIO61.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO61_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO61_SEL` writer - Configures to select an ETM task channel for GPIO61.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO61_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO61_EN` reader - Configures whether or not to enable GPIO61 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO61_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO61_EN` writer - Configures whether or not to enable GPIO61 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO61_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO62_SEL` reader - Configures to select an ETM task channel for GPIO62.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO62_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO62_SEL` writer - Configures to select an ETM task channel for GPIO62.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO62_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO62_EN` reader - Configures whether or not to enable GPIO62 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO62_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO62_EN` writer - Configures whether or not to enable GPIO62 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO62_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO60.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio60_sel(&self) -> ETM_TASK_GPIO60_SEL_R {
        ETM_TASK_GPIO60_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO60 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio60_en(&self) -> ETM_TASK_GPIO60_EN_R {
        ETM_TASK_GPIO60_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO61.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio61_sel(&self) -> ETM_TASK_GPIO61_SEL_R {
        ETM_TASK_GPIO61_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO61 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio61_en(&self) -> ETM_TASK_GPIO61_EN_R {
        ETM_TASK_GPIO61_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO62.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio62_sel(&self) -> ETM_TASK_GPIO62_SEL_R {
        ETM_TASK_GPIO62_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO62 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio62_en(&self) -> ETM_TASK_GPIO62_EN_R {
        ETM_TASK_GPIO62_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P12_CFG")
            .field("etm_task_gpio60_sel", &self.etm_task_gpio60_sel())
            .field("etm_task_gpio60_en", &self.etm_task_gpio60_en())
            .field("etm_task_gpio61_sel", &self.etm_task_gpio61_sel())
            .field("etm_task_gpio61_en", &self.etm_task_gpio61_en())
            .field("etm_task_gpio62_sel", &self.etm_task_gpio62_sel())
            .field("etm_task_gpio62_en", &self.etm_task_gpio62_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO60.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio60_sel(&mut self) -> ETM_TASK_GPIO60_SEL_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO60_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO60 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio60_en(&mut self) -> ETM_TASK_GPIO60_EN_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO60_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO61.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio61_sel(&mut self) -> ETM_TASK_GPIO61_SEL_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO61_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO61 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio61_en(&mut self) -> ETM_TASK_GPIO61_EN_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO61_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO62.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio62_sel(&mut self) -> ETM_TASK_GPIO62_SEL_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO62_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO62 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio62_en(&mut self) -> ETM_TASK_GPIO62_EN_W<'_, ETM_TASK_P12_CFG_SPEC> {
        ETM_TASK_GPIO62_EN_W::new(self, 17)
    }
}
#[doc = "GPIO selection register 12 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p12_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p12_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P12_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P12_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p12_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P12_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p12_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P12_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P12_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P12_CFG_SPEC {}
