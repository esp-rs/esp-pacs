#[doc = "Register `ETM_TASK_P0_CFG` reader"]
pub type R = crate::R<ETM_TASK_P0_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P0_CFG` writer"]
pub type W = crate::W<ETM_TASK_P0_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO0_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO0_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO0_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO0_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO0_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO0_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO1_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO1_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO1_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO1_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO1_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO1_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO2_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO2_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO2_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO2_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO2_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO2_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO3_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO3_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO3_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO3_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO3_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO3_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO4_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO4_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO4_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO4_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO4_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO4_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO4_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio0_sel(&self) -> ETM_TASK_GPIO0_SEL_R {
        ETM_TASK_GPIO0_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio0_en(&self) -> ETM_TASK_GPIO0_EN_R {
        ETM_TASK_GPIO0_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio1_sel(&self) -> ETM_TASK_GPIO1_SEL_R {
        ETM_TASK_GPIO1_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio1_en(&self) -> ETM_TASK_GPIO1_EN_R {
        ETM_TASK_GPIO1_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio2_sel(&self) -> ETM_TASK_GPIO2_SEL_R {
        ETM_TASK_GPIO2_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio2_en(&self) -> ETM_TASK_GPIO2_EN_R {
        ETM_TASK_GPIO2_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio3_sel(&self) -> ETM_TASK_GPIO3_SEL_R {
        ETM_TASK_GPIO3_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio3_en(&self) -> ETM_TASK_GPIO3_EN_R {
        ETM_TASK_GPIO3_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio4_sel(&self) -> ETM_TASK_GPIO4_SEL_R {
        ETM_TASK_GPIO4_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio4_en(&self) -> ETM_TASK_GPIO4_EN_R {
        ETM_TASK_GPIO4_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P0_CFG")
            .field("etm_task_gpio0_sel", &self.etm_task_gpio0_sel())
            .field("etm_task_gpio0_en", &self.etm_task_gpio0_en())
            .field("etm_task_gpio1_sel", &self.etm_task_gpio1_sel())
            .field("etm_task_gpio1_en", &self.etm_task_gpio1_en())
            .field("etm_task_gpio2_sel", &self.etm_task_gpio2_sel())
            .field("etm_task_gpio2_en", &self.etm_task_gpio2_en())
            .field("etm_task_gpio3_sel", &self.etm_task_gpio3_sel())
            .field("etm_task_gpio3_en", &self.etm_task_gpio3_en())
            .field("etm_task_gpio4_sel", &self.etm_task_gpio4_sel())
            .field("etm_task_gpio4_en", &self.etm_task_gpio4_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio0_sel(&mut self) -> ETM_TASK_GPIO0_SEL_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO0_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio0_en(&mut self) -> ETM_TASK_GPIO0_EN_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO0_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio1_sel(&mut self) -> ETM_TASK_GPIO1_SEL_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO1_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio1_en(&mut self) -> ETM_TASK_GPIO1_EN_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO1_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio2_sel(&mut self) -> ETM_TASK_GPIO2_SEL_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO2_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio2_en(&mut self) -> ETM_TASK_GPIO2_EN_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO2_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio3_sel(&mut self) -> ETM_TASK_GPIO3_SEL_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO3_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio3_en(&mut self) -> ETM_TASK_GPIO3_EN_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO3_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio4_sel(&mut self) -> ETM_TASK_GPIO4_SEL_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO4_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio4_en(&mut self) -> ETM_TASK_GPIO4_EN_W<ETM_TASK_P0_CFG_SPEC> {
        ETM_TASK_GPIO4_EN_W::new(self, 29)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
