#[doc = "Register `ETM_TASK_P8_CFG` reader"]
pub type R = crate::R<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P8_CFG` writer"]
pub type W = crate::W<ETM_TASK_P8_CFG_SPEC>;
#[doc = "Field `GPIO_EN(32-35)` reader - Enable bit of GPIO response etm task."]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(32-35)` writer - Enable bit of GPIO response etm task."]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_SEL(32-35)` reader - GPIO choose a etm task channel."]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(32-35)` writer - GPIO choose a etm task channel."]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO32_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..4).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio32_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio33_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio34_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio35_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO32_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 8 + 1)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..4).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 8 + 1)) & 7) as u8))
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio32_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio33_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio34_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio35_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P8_CFG")
            .field("gpio32_en", &self.gpio32_en())
            .field("gpio33_en", &self.gpio33_en())
            .field("gpio34_en", &self.gpio34_en())
            .field("gpio35_en", &self.gpio35_en())
            .field("gpio32_sel", &self.gpio32_sel())
            .field("gpio33_sel", &self.gpio33_sel())
            .field("gpio34_sel", &self.gpio34_sel())
            .field("gpio35_sel", &self.gpio35_sel())
            .finish()
    }
}
impl W {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO32_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<ETM_TASK_P8_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GPIO_EN_W::new(self, n * 8)
    }
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio32_en(&mut self) -> GPIO_EN_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio33_en(&mut self) -> GPIO_EN_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio34_en(&mut self) -> GPIO_EN_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio35_en(&mut self) -> GPIO_EN_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_EN_W::new(self, 24)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO32_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<ETM_TASK_P8_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GPIO_SEL_W::new(self, n * 8 + 1)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio32_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 1)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio33_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 9)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio34_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 17)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio35_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P8_CFG_SPEC> {
        GPIO_SEL_W::new(self, 25)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p8_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p8_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P8_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P8_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p8_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P8_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p8_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P8_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P8_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P8_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
