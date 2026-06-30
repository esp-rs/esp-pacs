#[doc = "Register `ETM_TASK_P11_CFG` reader"]
pub type R = crate::R<ETM_TASK_P11_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P11_CFG` writer"]
pub type W = crate::W<ETM_TASK_P11_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO55_SEL` reader - Configures to select an ETM task channel for GPIO55.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO55_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO55_SEL` writer - Configures to select an ETM task channel for GPIO55.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO55_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO55_EN` reader - Configures whether or not to enable GPIO55 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO55_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO55_EN` writer - Configures whether or not to enable GPIO55 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO55_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO56_SEL` reader - Configures to select an ETM task channel for GPIO56.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO56_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO56_SEL` writer - Configures to select an ETM task channel for GPIO56.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO56_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO56_EN` reader - Configures whether or not to enable GPIO56 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO56_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO56_EN` writer - Configures whether or not to enable GPIO56 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO56_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO57_SEL` reader - Configures to select an ETM task channel for GPIO57.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO57_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO57_SEL` writer - Configures to select an ETM task channel for GPIO57.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO57_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO57_EN` reader - Configures whether or not to enable GPIO57 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO57_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO57_EN` writer - Configures whether or not to enable GPIO57 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO57_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO58_SEL` reader - Configures to select an ETM task channel for GPIO58.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO58_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO58_SEL` writer - Configures to select an ETM task channel for GPIO58.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO58_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO58_EN` reader - Configures whether or not to enable GPIO58 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO58_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO58_EN` writer - Configures whether or not to enable GPIO58 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO58_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO59_SEL` reader - Configures to select an ETM task channel for GPIO59.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO59_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO59_SEL` writer - Configures to select an ETM task channel for GPIO59.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO59_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO59_EN` reader - Configures whether or not to enable GPIO59 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO59_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO59_EN` writer - Configures whether or not to enable GPIO59 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO59_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO55.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio55_sel(&self) -> ETM_TASK_GPIO55_SEL_R {
        ETM_TASK_GPIO55_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO55 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio55_en(&self) -> ETM_TASK_GPIO55_EN_R {
        ETM_TASK_GPIO55_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO56.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio56_sel(&self) -> ETM_TASK_GPIO56_SEL_R {
        ETM_TASK_GPIO56_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO56 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio56_en(&self) -> ETM_TASK_GPIO56_EN_R {
        ETM_TASK_GPIO56_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO57.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio57_sel(&self) -> ETM_TASK_GPIO57_SEL_R {
        ETM_TASK_GPIO57_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO57 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio57_en(&self) -> ETM_TASK_GPIO57_EN_R {
        ETM_TASK_GPIO57_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO58.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio58_sel(&self) -> ETM_TASK_GPIO58_SEL_R {
        ETM_TASK_GPIO58_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO58 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio58_en(&self) -> ETM_TASK_GPIO58_EN_R {
        ETM_TASK_GPIO58_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO59.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio59_sel(&self) -> ETM_TASK_GPIO59_SEL_R {
        ETM_TASK_GPIO59_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO59 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio59_en(&self) -> ETM_TASK_GPIO59_EN_R {
        ETM_TASK_GPIO59_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P11_CFG")
            .field("etm_task_gpio55_sel", &self.etm_task_gpio55_sel())
            .field("etm_task_gpio55_en", &self.etm_task_gpio55_en())
            .field("etm_task_gpio56_sel", &self.etm_task_gpio56_sel())
            .field("etm_task_gpio56_en", &self.etm_task_gpio56_en())
            .field("etm_task_gpio57_sel", &self.etm_task_gpio57_sel())
            .field("etm_task_gpio57_en", &self.etm_task_gpio57_en())
            .field("etm_task_gpio58_sel", &self.etm_task_gpio58_sel())
            .field("etm_task_gpio58_en", &self.etm_task_gpio58_en())
            .field("etm_task_gpio59_sel", &self.etm_task_gpio59_sel())
            .field("etm_task_gpio59_en", &self.etm_task_gpio59_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO55.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio55_sel(&mut self) -> ETM_TASK_GPIO55_SEL_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO55_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO55 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio55_en(&mut self) -> ETM_TASK_GPIO55_EN_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO55_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO56.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio56_sel(&mut self) -> ETM_TASK_GPIO56_SEL_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO56_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO56 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio56_en(&mut self) -> ETM_TASK_GPIO56_EN_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO56_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO57.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio57_sel(&mut self) -> ETM_TASK_GPIO57_SEL_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO57_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO57 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio57_en(&mut self) -> ETM_TASK_GPIO57_EN_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO57_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO58.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio58_sel(&mut self) -> ETM_TASK_GPIO58_SEL_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO58_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO58 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio58_en(&mut self) -> ETM_TASK_GPIO58_EN_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO58_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO59.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio59_sel(&mut self) -> ETM_TASK_GPIO59_SEL_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO59_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO59 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio59_en(&mut self) -> ETM_TASK_GPIO59_EN_W<'_, ETM_TASK_P11_CFG_SPEC> {
        ETM_TASK_GPIO59_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 11 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p11_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p11_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P11_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P11_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p11_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P11_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p11_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P11_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P11_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P11_CFG_SPEC {}
