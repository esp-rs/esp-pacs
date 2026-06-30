#[doc = "Register `ETM_TASK_P7_CFG` reader"]
pub type R = crate::R<ETM_TASK_P7_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P7_CFG` writer"]
pub type W = crate::W<ETM_TASK_P7_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO35_SEL` reader - Configures to select an ETM task channel for GPIO35.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO35_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO35_SEL` writer - Configures to select an ETM task channel for GPIO35.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO35_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO35_EN` reader - Configures whether or not to enable GPIO35 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO35_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO35_EN` writer - Configures whether or not to enable GPIO35 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO35_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO36_SEL` reader - Configures to select an ETM task channel for GPIO36.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO36_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO36_SEL` writer - Configures to select an ETM task channel for GPIO36.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO36_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO36_EN` reader - Configures whether or not to enable GPIO36 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO36_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO36_EN` writer - Configures whether or not to enable GPIO36 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO36_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO37_SEL` reader - Configures to select an ETM task channel for GPIO37.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO37_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO37_SEL` writer - Configures to select an ETM task channel for GPIO37.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO37_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO37_EN` reader - Configures whether or not to enable GPIO37 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO37_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO37_EN` writer - Configures whether or not to enable GPIO37 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO37_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO38_SEL` reader - Configures to select an ETM task channel for GPIO38.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO38_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO38_SEL` writer - Configures to select an ETM task channel for GPIO38.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO38_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO38_EN` reader - Configures whether or not to enable GPIO38 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO38_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO38_EN` writer - Configures whether or not to enable GPIO38 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO38_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO39_SEL` reader - Configures to select an ETM task channel for GPIO39.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO39_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO39_SEL` writer - Configures to select an ETM task channel for GPIO39.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
pub type ETM_TASK_GPIO39_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO39_EN` reader - Configures whether or not to enable GPIO39 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO39_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO39_EN` writer - Configures whether or not to enable GPIO39 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type ETM_TASK_GPIO39_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO35.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio35_sel(&self) -> ETM_TASK_GPIO35_SEL_R {
        ETM_TASK_GPIO35_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO35 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio35_en(&self) -> ETM_TASK_GPIO35_EN_R {
        ETM_TASK_GPIO35_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO36.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio36_sel(&self) -> ETM_TASK_GPIO36_SEL_R {
        ETM_TASK_GPIO36_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO36 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio36_en(&self) -> ETM_TASK_GPIO36_EN_R {
        ETM_TASK_GPIO36_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO37.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio37_sel(&self) -> ETM_TASK_GPIO37_SEL_R {
        ETM_TASK_GPIO37_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO37 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio37_en(&self) -> ETM_TASK_GPIO37_EN_R {
        ETM_TASK_GPIO37_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO38.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio38_sel(&self) -> ETM_TASK_GPIO38_SEL_R {
        ETM_TASK_GPIO38_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO38 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio38_en(&self) -> ETM_TASK_GPIO38_EN_R {
        ETM_TASK_GPIO38_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO39.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio39_sel(&self) -> ETM_TASK_GPIO39_SEL_R {
        ETM_TASK_GPIO39_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO39 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio39_en(&self) -> ETM_TASK_GPIO39_EN_R {
        ETM_TASK_GPIO39_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P7_CFG")
            .field("etm_task_gpio35_sel", &self.etm_task_gpio35_sel())
            .field("etm_task_gpio35_en", &self.etm_task_gpio35_en())
            .field("etm_task_gpio36_sel", &self.etm_task_gpio36_sel())
            .field("etm_task_gpio36_en", &self.etm_task_gpio36_en())
            .field("etm_task_gpio37_sel", &self.etm_task_gpio37_sel())
            .field("etm_task_gpio37_en", &self.etm_task_gpio37_en())
            .field("etm_task_gpio38_sel", &self.etm_task_gpio38_sel())
            .field("etm_task_gpio38_en", &self.etm_task_gpio38_en())
            .field("etm_task_gpio39_sel", &self.etm_task_gpio39_sel())
            .field("etm_task_gpio39_en", &self.etm_task_gpio39_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures to select an ETM task channel for GPIO35.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio35_sel(&mut self) -> ETM_TASK_GPIO35_SEL_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO35_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable GPIO35 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio35_en(&mut self) -> ETM_TASK_GPIO35_EN_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO35_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - Configures to select an ETM task channel for GPIO36.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio36_sel(&mut self) -> ETM_TASK_GPIO36_SEL_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO36_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Configures whether or not to enable GPIO36 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio36_en(&mut self) -> ETM_TASK_GPIO36_EN_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO36_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Configures to select an ETM task channel for GPIO37.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio37_sel(&mut self) -> ETM_TASK_GPIO37_SEL_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO37_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Configures whether or not to enable GPIO37 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio37_en(&mut self) -> ETM_TASK_GPIO37_EN_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO37_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - Configures to select an ETM task channel for GPIO38.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio38_sel(&mut self) -> ETM_TASK_GPIO38_SEL_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO38_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Configures whether or not to enable GPIO38 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio38_en(&mut self) -> ETM_TASK_GPIO38_EN_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO38_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Configures to select an ETM task channel for GPIO39.\\\\ 0: Select channel 0\\\\ 1: Select channel 1\\\\ ......\\\\ 7: Select channel 7\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio39_sel(&mut self) -> ETM_TASK_GPIO39_SEL_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO39_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Configures whether or not to enable GPIO39 to response ETM task.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_task_gpio39_en(&mut self) -> ETM_TASK_GPIO39_EN_W<'_, ETM_TASK_P7_CFG_SPEC> {
        ETM_TASK_GPIO39_EN_W::new(self, 29)
    }
}
#[doc = "GPIO selection register 7 for ETM\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p7_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p7_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P7_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P7_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p7_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P7_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p7_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P7_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P7_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P7_CFG_SPEC {}
