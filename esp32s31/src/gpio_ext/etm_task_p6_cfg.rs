#[doc = "Register `ETM_TASK_P6_CFG` reader"]
pub type R = crate::R<ETM_TASK_P6_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P6_CFG` writer"]
pub type W = crate::W<ETM_TASK_P6_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO30_SEL` reader - Configures to select an ETM task channel for GPIO30.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO30_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO30_SEL` writer - Configures to select an ETM task channel for GPIO30.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO30_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO30_EN` reader - Configures whether or not to enable GPIO30 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO30_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO30_EN` writer - Configures whether or not to enable GPIO30 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO30_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO31_SEL` reader - Configures to select an ETM task channel for GPIO31.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO31_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO31_SEL` writer - Configures to select an ETM task channel for GPIO31.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO31_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO31_EN` reader - Configures whether or not to enable GPIO31 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO31_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO31_EN` writer - Configures whether or not to enable GPIO31 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO31_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO32_SEL` reader - Configures to select an ETM task channel for GPIO32.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO32_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO32_SEL` writer - Configures to select an ETM task channel for GPIO32.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO32_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO32_EN` reader - Configures whether or not to enable GPIO32 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO32_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO32_EN` writer - Configures whether or not to enable GPIO32 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO32_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO33_SEL` reader - Configures to select an ETM task channel for GPIO33.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO33_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO33_SEL` writer - Configures to select an ETM task channel for GPIO33.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO33_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO33_EN` reader - Configures whether or not to enable GPIO33 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO33_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO33_EN` writer - Configures whether or not to enable GPIO33 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO33_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO34_SEL` reader - Configures to select an ETM task channel for GPIO34.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO34_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO34_SEL` writer - Configures to select an ETM task channel for GPIO34.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO34_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO34_EN` reader - Configures whether or not to enable GPIO34 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO34_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO34_EN` writer - Configures whether or not to enable GPIO34 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO34_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO30.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio30_sel(&self) -> ETM_TASK_GPIO30_SEL_R {
        ETM_TASK_GPIO30_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO30 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio30_en(&self) -> ETM_TASK_GPIO30_EN_R {
        ETM_TASK_GPIO30_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO31.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio31_sel(&self) -> ETM_TASK_GPIO31_SEL_R {
        ETM_TASK_GPIO31_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO31 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio31_en(&self) -> ETM_TASK_GPIO31_EN_R {
        ETM_TASK_GPIO31_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO32.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio32_sel(&self) -> ETM_TASK_GPIO32_SEL_R {
        ETM_TASK_GPIO32_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO32 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio32_en(&self) -> ETM_TASK_GPIO32_EN_R {
        ETM_TASK_GPIO32_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO33.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio33_sel(&self) -> ETM_TASK_GPIO33_SEL_R {
        ETM_TASK_GPIO33_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO33 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio33_en(&self) -> ETM_TASK_GPIO33_EN_R {
        ETM_TASK_GPIO33_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO34.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio34_sel(&self) -> ETM_TASK_GPIO34_SEL_R {
        ETM_TASK_GPIO34_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO34 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio34_en(&self) -> ETM_TASK_GPIO34_EN_R {
        ETM_TASK_GPIO34_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P6_CFG")
            .field("etm_task_gpio30_sel", &self.etm_task_gpio30_sel())
            .field("etm_task_gpio30_en", &self.etm_task_gpio30_en())
            .field("etm_task_gpio31_sel", &self.etm_task_gpio31_sel())
            .field("etm_task_gpio31_en", &self.etm_task_gpio31_en())
            .field("etm_task_gpio32_sel", &self.etm_task_gpio32_sel())
            .field("etm_task_gpio32_en", &self.etm_task_gpio32_en())
            .field("etm_task_gpio33_sel", &self.etm_task_gpio33_sel())
            .field("etm_task_gpio33_en", &self.etm_task_gpio33_en())
            .field("etm_task_gpio34_sel", &self.etm_task_gpio34_sel())
            .field("etm_task_gpio34_en", &self.etm_task_gpio34_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO30.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio30_sel(&mut self) -> ETM_TASK_GPIO30_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO30_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO30 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio30_en(&mut self) -> ETM_TASK_GPIO30_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO30_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO31.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio31_sel(&mut self) -> ETM_TASK_GPIO31_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO31_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO31 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio31_en(&mut self) -> ETM_TASK_GPIO31_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO31_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO32.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio32_sel(&mut self) -> ETM_TASK_GPIO32_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO32_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO32 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio32_en(&mut self) -> ETM_TASK_GPIO32_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO32_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO33.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio33_sel(&mut self) -> ETM_TASK_GPIO33_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO33_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO33 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio33_en(&mut self) -> ETM_TASK_GPIO33_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO33_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO34.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio34_sel(&mut self) -> ETM_TASK_GPIO34_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO34_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO34 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio34_en(&mut self) -> ETM_TASK_GPIO34_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        ETM_TASK_GPIO34_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 6 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p6_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p6_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P6_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P6_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p6_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P6_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p6_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P6_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P6_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P6_CFG_SPEC {}
