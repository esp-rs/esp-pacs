#[doc = "Register `IN_COLOR_CONVERT_CH0` reader"]
pub type R = crate::R<IN_COLOR_CONVERT_CH0_SPEC>;
#[doc = "Register `IN_COLOR_CONVERT_CH0` writer"]
pub type W = crate::W<IN_COLOR_CONVERT_CH0_SPEC>;
#[doc = "Field `IN_COLOR_OUTPUT_SEL_CH` reader - Set final color convert process and output type 0: RGB888 to RGB565 1: output directly 2: YUV444 to YUV422 3: YUV444 to YUV420"]
pub type IN_COLOR_OUTPUT_SEL_CH_R = crate::FieldReader;
#[doc = "Field `IN_COLOR_OUTPUT_SEL_CH` writer - Set final color convert process and output type 0: RGB888 to RGB565 1: output directly 2: YUV444 to YUV422 3: YUV444 to YUV420"]
pub type IN_COLOR_OUTPUT_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN_COLOR_3B_PROC_EN_CH` reader - Enable generic color convert modlue between color input & color output, need to configure parameter."]
pub type IN_COLOR_3B_PROC_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_COLOR_3B_PROC_EN_CH` writer - Enable generic color convert modlue between color input & color output, need to configure parameter."]
pub type IN_COLOR_3B_PROC_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_COLOR_INPUT_SEL_CH` reader - Set first color convert process and input color type 0: YUV422/420 to YUV444 1: YUV422 2: YUV444/420 7: disable color space convert"]
pub type IN_COLOR_INPUT_SEL_CH_R = crate::FieldReader;
#[doc = "Field `IN_COLOR_INPUT_SEL_CH` writer - Set first color convert process and input color type 0: YUV422/420 to YUV444 1: YUV422 2: YUV444/420 7: disable color space convert"]
pub type IN_COLOR_INPUT_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Set final color convert process and output type 0: RGB888 to RGB565 1: output directly 2: YUV444 to YUV422 3: YUV444 to YUV420"]
    #[inline(always)]
    pub fn in_color_output_sel_ch(&self) -> IN_COLOR_OUTPUT_SEL_CH_R {
        IN_COLOR_OUTPUT_SEL_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable generic color convert modlue between color input & color output, need to configure parameter."]
    #[inline(always)]
    pub fn in_color_3b_proc_en_ch(&self) -> IN_COLOR_3B_PROC_EN_CH_R {
        IN_COLOR_3B_PROC_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Set first color convert process and input color type 0: YUV422/420 to YUV444 1: YUV422 2: YUV444/420 7: disable color space convert"]
    #[inline(always)]
    pub fn in_color_input_sel_ch(&self) -> IN_COLOR_INPUT_SEL_CH_R {
        IN_COLOR_INPUT_SEL_CH_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_COLOR_CONVERT_CH0")
            .field("in_color_output_sel_ch", &self.in_color_output_sel_ch())
            .field("in_color_3b_proc_en_ch", &self.in_color_3b_proc_en_ch())
            .field("in_color_input_sel_ch", &self.in_color_input_sel_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Set final color convert process and output type 0: RGB888 to RGB565 1: output directly 2: YUV444 to YUV422 3: YUV444 to YUV420"]
    #[inline(always)]
    pub fn in_color_output_sel_ch(
        &mut self,
    ) -> IN_COLOR_OUTPUT_SEL_CH_W<'_, IN_COLOR_CONVERT_CH0_SPEC> {
        IN_COLOR_OUTPUT_SEL_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable generic color convert modlue between color input & color output, need to configure parameter."]
    #[inline(always)]
    pub fn in_color_3b_proc_en_ch(
        &mut self,
    ) -> IN_COLOR_3B_PROC_EN_CH_W<'_, IN_COLOR_CONVERT_CH0_SPEC> {
        IN_COLOR_3B_PROC_EN_CH_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Set first color convert process and input color type 0: YUV422/420 to YUV444 1: YUV422 2: YUV444/420 7: disable color space convert"]
    #[inline(always)]
    pub fn in_color_input_sel_ch(
        &mut self,
    ) -> IN_COLOR_INPUT_SEL_CH_W<'_, IN_COLOR_CONVERT_CH0_SPEC> {
        IN_COLOR_INPUT_SEL_CH_W::new(self, 3)
    }
}
#[doc = "Configures the Rx color convert of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_convert_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_convert_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_COLOR_CONVERT_CH0_SPEC;
impl crate::RegisterSpec for IN_COLOR_CONVERT_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_color_convert_ch0::R`](R) reader structure"]
impl crate::Readable for IN_COLOR_CONVERT_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_color_convert_ch0::W`](W) writer structure"]
impl crate::Writable for IN_COLOR_CONVERT_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_COLOR_CONVERT_CH0 to value 0x38"]
impl crate::Resettable for IN_COLOR_CONVERT_CH0_SPEC {
    const RESET_VALUE: u32 = 0x38;
}
