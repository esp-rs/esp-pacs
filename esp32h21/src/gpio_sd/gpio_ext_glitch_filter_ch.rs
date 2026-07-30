#[doc = "Register `GPIO_EXT_GLITCH_FILTER_CH%s` reader"]
pub type R = crate::R<GPIO_EXT_GLITCH_FILTER_CH_SPEC>;
#[doc = "Register `GPIO_EXT_GLITCH_FILTER_CH%s` writer"]
pub type W = crate::W<GPIO_EXT_GLITCH_FILTER_CH_SPEC>;
#[doc = "Field `GPIO_EXT_FILTER_CH0_EN` reader - Glitch Filter channel enable bit."]
pub type GPIO_EXT_FILTER_CH0_EN_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_FILTER_CH0_EN` writer - Glitch Filter channel enable bit."]
pub type GPIO_EXT_FILTER_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_FILTER_CH0_INPUT_IO_NUM` reader - Glitch Filter input io number."]
pub type GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_FILTER_CH0_INPUT_IO_NUM` writer - Glitch Filter input io number."]
pub type GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPIO_EXT_FILTER_CH0_WINDOW_THRES` reader - Glitch Filter window threshold."]
pub type GPIO_EXT_FILTER_CH0_WINDOW_THRES_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_FILTER_CH0_WINDOW_THRES` writer - Glitch Filter window threshold."]
pub type GPIO_EXT_FILTER_CH0_WINDOW_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GPIO_EXT_FILTER_CH0_WINDOW_WIDTH` reader - Glitch Filter window width."]
pub type GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_FILTER_CH0_WINDOW_WIDTH` writer - Glitch Filter window width."]
pub type GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_en(&self) -> GPIO_EXT_FILTER_CH0_EN_R {
        GPIO_EXT_FILTER_CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Glitch Filter input io number."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_input_io_num(&self) -> GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_R {
        GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Glitch Filter window threshold."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_window_thres(&self) -> GPIO_EXT_FILTER_CH0_WINDOW_THRES_R {
        GPIO_EXT_FILTER_CH0_WINDOW_THRES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - Glitch Filter window width."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_window_width(&self) -> GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_R {
        GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_GLITCH_FILTER_CH")
            .field("gpio_ext_filter_ch0_en", &self.gpio_ext_filter_ch0_en())
            .field(
                "gpio_ext_filter_ch0_input_io_num",
                &self.gpio_ext_filter_ch0_input_io_num(),
            )
            .field(
                "gpio_ext_filter_ch0_window_thres",
                &self.gpio_ext_filter_ch0_window_thres(),
            )
            .field(
                "gpio_ext_filter_ch0_window_width",
                &self.gpio_ext_filter_ch0_window_width(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_en(
        &mut self,
    ) -> GPIO_EXT_FILTER_CH0_EN_W<'_, GPIO_EXT_GLITCH_FILTER_CH_SPEC> {
        GPIO_EXT_FILTER_CH0_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - Glitch Filter input io number."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_input_io_num(
        &mut self,
    ) -> GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_W<'_, GPIO_EXT_GLITCH_FILTER_CH_SPEC> {
        GPIO_EXT_FILTER_CH0_INPUT_IO_NUM_W::new(self, 1)
    }
    #[doc = "Bits 8:13 - Glitch Filter window threshold."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_window_thres(
        &mut self,
    ) -> GPIO_EXT_FILTER_CH0_WINDOW_THRES_W<'_, GPIO_EXT_GLITCH_FILTER_CH_SPEC> {
        GPIO_EXT_FILTER_CH0_WINDOW_THRES_W::new(self, 8)
    }
    #[doc = "Bits 14:19 - Glitch Filter window width."]
    #[inline(always)]
    pub fn gpio_ext_filter_ch0_window_width(
        &mut self,
    ) -> GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_W<'_, GPIO_EXT_GLITCH_FILTER_CH_SPEC> {
        GPIO_EXT_FILTER_CH0_WINDOW_WIDTH_W::new(self, 14)
    }
}
#[doc = "Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_glitch_filter_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_glitch_filter_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_GLITCH_FILTER_CH_SPEC;
impl crate::RegisterSpec for GPIO_EXT_GLITCH_FILTER_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_glitch_filter_ch::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_GLITCH_FILTER_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_glitch_filter_ch::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_GLITCH_FILTER_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_GLITCH_FILTER_CH%s to value 0"]
impl crate::Resettable for GPIO_EXT_GLITCH_FILTER_CH_SPEC {}
