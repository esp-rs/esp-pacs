#[doc = "Register `ETM_TASK_P6_CFG` reader"]
pub type R = crate::R<ETM_TASK_P6_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P6_CFG` writer"]
pub type W = crate::W<ETM_TASK_P6_CFG_SPEC>;
#[doc = "Field `GPIO_SEL(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34)` reader - Select GPIO%s for ETM task"]
pub type GPIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SEL(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34)` writer - Select GPIO%s for ETM task"]
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO_EN(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34)` reader - Enable GPIO%s for ETM task"]
pub type GPIO_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EN(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34)` writer - Enable GPIO%s for ETM task"]
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Select GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO30_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Select GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..5).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 6)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Select GPIO_EXT_ETM_TASK_GPIO30 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio30_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - Select GPIO_EXT_ETM_TASK_GPIO31 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio31_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select GPIO_EXT_ETM_TASK_GPIO32 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio32_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Select GPIO_EXT_ETM_TASK_GPIO33 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio33_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Select GPIO_EXT_ETM_TASK_GPIO34 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio34_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Enable GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO30_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..5).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Enable GPIO_EXT_ETM_TASK_GPIO30 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio30_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable GPIO_EXT_ETM_TASK_GPIO31 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio31_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO_EXT_ETM_TASK_GPIO32 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio32_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable GPIO_EXT_ETM_TASK_GPIO33 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio33_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable GPIO_EXT_ETM_TASK_GPIO34 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio34_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P6_CFG")
            .field(
                "gpio_ext_etm_task_gpio30_sel",
                &self.gpio_ext_etm_task_gpio30_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio31_sel",
                &self.gpio_ext_etm_task_gpio31_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio32_sel",
                &self.gpio_ext_etm_task_gpio32_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio33_sel",
                &self.gpio_ext_etm_task_gpio33_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio34_sel",
                &self.gpio_ext_etm_task_gpio34_sel(),
            )
            .field(
                "gpio_ext_etm_task_gpio30_en",
                &self.gpio_ext_etm_task_gpio30_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio31_en",
                &self.gpio_ext_etm_task_gpio31_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio32_en",
                &self.gpio_ext_etm_task_gpio32_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio33_en",
                &self.gpio_ext_etm_task_gpio33_en(),
            )
            .field(
                "gpio_ext_etm_task_gpio34_en",
                &self.gpio_ext_etm_task_gpio34_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Select GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO30_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_SEL_W::new(self, n * 6)
    }
    #[doc = "Bits 0:2 - Select GPIO_EXT_ETM_TASK_GPIO30 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio30_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_SEL_W::new(self, 0)
    }
    #[doc = "Bits 6:8 - Select GPIO_EXT_ETM_TASK_GPIO31 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio31_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_SEL_W::new(self, 6)
    }
    #[doc = "Bits 12:14 - Select GPIO_EXT_ETM_TASK_GPIO32 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio32_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:20 - Select GPIO_EXT_ETM_TASK_GPIO33 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio33_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Select GPIO_EXT_ETM_TASK_GPIO34 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio34_sel(&mut self) -> GPIO_SEL_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_SEL_W::new(self, 24)
    }
    #[doc = "Enable GPIO(_EXT_ETM_TASK_GPIO30,_EXT_ETM_TASK_GPIO31,_EXT_ETM_TASK_GPIO32,_EXT_ETM_TASK_GPIO33,_EXT_ETM_TASK_GPIO34) for ETM task"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO_EXT_ETM_TASK_GPIO30_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        GPIO_EN_W::new(self, n * 6 + 5)
    }
    #[doc = "Bit 5 - Enable GPIO_EXT_ETM_TASK_GPIO30 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio30_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Enable GPIO_EXT_ETM_TASK_GPIO31 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio31_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_EN_W::new(self, 11)
    }
    #[doc = "Bit 17 - Enable GPIO_EXT_ETM_TASK_GPIO32 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio32_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_EN_W::new(self, 17)
    }
    #[doc = "Bit 23 - Enable GPIO_EXT_ETM_TASK_GPIO33 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio33_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - Enable GPIO_EXT_ETM_TASK_GPIO34 for ETM task"]
    #[inline(always)]
    pub fn gpio_ext_etm_task_gpio34_en(&mut self) -> GPIO_EN_W<'_, ETM_TASK_P6_CFG_SPEC> {
        GPIO_EN_W::new(self, 29)
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
