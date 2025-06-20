#[doc = "Register `GLITCH_FILTER_CH%s` reader"]
pub type R = crate::R<GLITCH_FILTER_CH_SPEC>;
#[doc = "Register `GLITCH_FILTER_CH%s` writer"]
pub type W = crate::W<GLITCH_FILTER_CH_SPEC>;
#[doc = "Field `FILTER_CH0_EN` reader - Configures whether or not to enable channel %s of Glitch Filter.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type FILTER_CH0_EN_R = crate::BitReader;
#[doc = "Field `FILTER_CH0_EN` writer - Configures whether or not to enable channel %s of Glitch Filter.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
pub type FILTER_CH0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER_CH0_INPUT_IO_NUM` reader - Configures to select the input GPIO for Glitch Filter. \\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
pub type FILTER_CH0_INPUT_IO_NUM_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_INPUT_IO_NUM` writer - Configures to select the input GPIO for Glitch Filter. \\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
pub type FILTER_CH0_INPUT_IO_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FILTER_CH0_WINDOW_THRES` reader - Configures the window threshold for Glitch Filter. The window threshold should be less than or equal to GPIOSD_FILTER_CH%s_WINDOW_WIDTH.\\\\ %see DOC-4768\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type FILTER_CH0_WINDOW_THRES_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_WINDOW_THRES` writer - Configures the window threshold for Glitch Filter. The window threshold should be less than or equal to GPIOSD_FILTER_CH%s_WINDOW_WIDTH.\\\\ %see DOC-4768\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type FILTER_CH0_WINDOW_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FILTER_CH0_WINDOW_WIDTH` reader - Configures the window width for Glitch Filter. The effective value of window width is 0 ~ 63. \\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type FILTER_CH0_WINDOW_WIDTH_R = crate::FieldReader;
#[doc = "Field `FILTER_CH0_WINDOW_WIDTH` writer - Configures the window width for Glitch Filter. The effective value of window width is 0 ~ 63. \\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
pub type FILTER_CH0_WINDOW_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable channel %s of Glitch Filter.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> FILTER_CH0_EN_R {
        FILTER_CH0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Configures to select the input GPIO for Glitch Filter. \\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> FILTER_CH0_INPUT_IO_NUM_R {
        FILTER_CH0_INPUT_IO_NUM_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Configures the window threshold for Glitch Filter. The window threshold should be less than or equal to GPIOSD_FILTER_CH%s_WINDOW_WIDTH.\\\\ %see DOC-4768\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> FILTER_CH0_WINDOW_THRES_R {
        FILTER_CH0_WINDOW_THRES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - Configures the window width for Glitch Filter. The effective value of window width is 0 ~ 63. \\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> FILTER_CH0_WINDOW_WIDTH_R {
        FILTER_CH0_WINDOW_WIDTH_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GLITCH_FILTER_CH")
            .field("filter_ch0_en", &self.filter_ch0_en())
            .field("filter_ch0_input_io_num", &self.filter_ch0_input_io_num())
            .field("filter_ch0_window_thres", &self.filter_ch0_window_thres())
            .field("filter_ch0_window_width", &self.filter_ch0_window_width())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable channel %s of Glitch Filter.\\\\ 0: Not enable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn filter_ch0_en(&mut self) -> FILTER_CH0_EN_W<GLITCH_FILTER_CH_SPEC> {
        FILTER_CH0_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:6 - Configures to select the input GPIO for Glitch Filter. \\\\ 0: Select GPIO0\\\\ 1: Select GPIO1\\\\ ......\\\\ 27: Select GPIO27\\\\ 28: Select GPIO28\\\\ 29 ~ 63: Reserved\\\\"]
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&mut self) -> FILTER_CH0_INPUT_IO_NUM_W<GLITCH_FILTER_CH_SPEC> {
        FILTER_CH0_INPUT_IO_NUM_W::new(self, 1)
    }
    #[doc = "Bits 8:13 - Configures the window threshold for Glitch Filter. The window threshold should be less than or equal to GPIOSD_FILTER_CH%s_WINDOW_WIDTH.\\\\ %see DOC-4768\\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn filter_ch0_window_thres(&mut self) -> FILTER_CH0_WINDOW_THRES_W<GLITCH_FILTER_CH_SPEC> {
        FILTER_CH0_WINDOW_THRES_W::new(self, 8)
    }
    #[doc = "Bits 14:19 - Configures the window width for Glitch Filter. The effective value of window width is 0 ~ 63. \\\\ Measurement unit: IO MUX operating clock cycle\\\\"]
    #[inline(always)]
    pub fn filter_ch0_window_width(&mut self) -> FILTER_CH0_WINDOW_WIDTH_W<GLITCH_FILTER_CH_SPEC> {
        FILTER_CH0_WINDOW_WIDTH_W::new(self, 14)
    }
}
#[doc = "Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`glitch_filter_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glitch_filter_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLITCH_FILTER_CH_SPEC;
impl crate::RegisterSpec for GLITCH_FILTER_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glitch_filter_ch::R`](R) reader structure"]
impl crate::Readable for GLITCH_FILTER_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glitch_filter_ch::W`](W) writer structure"]
impl crate::Writable for GLITCH_FILTER_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLITCH_FILTER_CH%s to value 0"]
impl crate::Resettable for GLITCH_FILTER_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
