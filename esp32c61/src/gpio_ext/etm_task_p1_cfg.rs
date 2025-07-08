#[doc = "Register `ETM_TASK_P1_CFG` reader"]
pub type R = crate::R<ETM_TASK_P1_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P1_CFG` writer"]
pub type W = crate::W<ETM_TASK_P1_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO5_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO5_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO5_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO5_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO5_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO5_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO5_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO5_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO6_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO6_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO6_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO6_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO6_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO6_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO6_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO6_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO7_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO7_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO7_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO7_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO7_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO7_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO7_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO7_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO8_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO8_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO8_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO8_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO8_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO8_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO8_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO8_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_GPIO9_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO9_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO9_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO9_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETM_TASK_GPIO9_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO9_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO9_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO9_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio5_sel(&self) -> ETM_TASK_GPIO5_SEL_R {
        ETM_TASK_GPIO5_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio5_en(&self) -> ETM_TASK_GPIO5_EN_R {
        ETM_TASK_GPIO5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio6_sel(&self) -> ETM_TASK_GPIO6_SEL_R {
        ETM_TASK_GPIO6_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio6_en(&self) -> ETM_TASK_GPIO6_EN_R {
        ETM_TASK_GPIO6_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio7_sel(&self) -> ETM_TASK_GPIO7_SEL_R {
        ETM_TASK_GPIO7_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio7_en(&self) -> ETM_TASK_GPIO7_EN_R {
        ETM_TASK_GPIO7_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio8_sel(&self) -> ETM_TASK_GPIO8_SEL_R {
        ETM_TASK_GPIO8_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio8_en(&self) -> ETM_TASK_GPIO8_EN_R {
        ETM_TASK_GPIO8_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio9_sel(&self) -> ETM_TASK_GPIO9_SEL_R {
        ETM_TASK_GPIO9_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio9_en(&self) -> ETM_TASK_GPIO9_EN_R {
        ETM_TASK_GPIO9_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P1_CFG")
            .field("etm_task_gpio5_sel", &self.etm_task_gpio5_sel())
            .field("etm_task_gpio5_en", &self.etm_task_gpio5_en())
            .field("etm_task_gpio6_sel", &self.etm_task_gpio6_sel())
            .field("etm_task_gpio6_en", &self.etm_task_gpio6_en())
            .field("etm_task_gpio7_sel", &self.etm_task_gpio7_sel())
            .field("etm_task_gpio7_en", &self.etm_task_gpio7_en())
            .field("etm_task_gpio8_sel", &self.etm_task_gpio8_sel())
            .field("etm_task_gpio8_en", &self.etm_task_gpio8_en())
            .field("etm_task_gpio9_sel", &self.etm_task_gpio9_sel())
            .field("etm_task_gpio9_en", &self.etm_task_gpio9_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio5_sel(&mut self) -> ETM_TASK_GPIO5_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO5_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio5_en(&mut self) -> ETM_TASK_GPIO5_EN_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO5_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio6_sel(&mut self) -> ETM_TASK_GPIO6_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO6_SEL_W::new(self, 6)
    }
    #[doc = "Bit 11 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio6_en(&mut self) -> ETM_TASK_GPIO6_EN_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO6_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio7_sel(&mut self) -> ETM_TASK_GPIO7_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO7_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio7_en(&mut self) -> ETM_TASK_GPIO7_EN_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO7_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio8_sel(&mut self) -> ETM_TASK_GPIO8_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO8_SEL_W::new(self, 18)
    }
    #[doc = "Bit 23 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio8_en(&mut self) -> ETM_TASK_GPIO8_EN_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO8_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio9_sel(&mut self) -> ETM_TASK_GPIO9_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO9_SEL_W::new(self, 24)
    }
    #[doc = "Bit 29 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio9_en(&mut self) -> ETM_TASK_GPIO9_EN_W<ETM_TASK_P1_CFG_SPEC> {
        ETM_TASK_GPIO9_EN_W::new(self, 29)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P1_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p1_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P1_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p1_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P1_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P1_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P1_CFG_SPEC {}
