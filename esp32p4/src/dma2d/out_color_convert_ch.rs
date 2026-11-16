#[doc = "Register `OUT_COLOR_CONVERT_CH%s` reader"]
pub type R = crate::R<OUT_COLOR_CONVERT_CH_SPEC>;
#[doc = "Register `OUT_COLOR_CONVERT_CH%s` writer"]
pub type W = crate::W<OUT_COLOR_CONVERT_CH_SPEC>;
#[doc = "Field `OUT_COLOR_OUTPUT_SEL_CH` reader - Set final color convert process and output type 0: RGB888 to RGB565 1: YUV444 to YUV422 2: output directly"]
pub type OUT_COLOR_OUTPUT_SEL_CH_R = crate::FieldReader;
#[doc = "Field `OUT_COLOR_OUTPUT_SEL_CH` writer - Set final color convert process and output type 0: RGB888 to RGB565 1: YUV444 to YUV422 2: output directly"]
pub type OUT_COLOR_OUTPUT_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT_COLOR_3B_PROC_EN_CH` reader - Enable generic color convert modlue between color input & color output, need to configure parameter."]
pub type OUT_COLOR_3B_PROC_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_COLOR_3B_PROC_EN_CH` writer - Enable generic color convert modlue between color input & color output, need to configure parameter."]
pub type OUT_COLOR_3B_PROC_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_COLOR_INPUT_SEL_CH` reader - Set first color convert process and input color type 0: RGB565 to RGB888 1: YUV422 to YUV444 2: Other 2byte/pixel type 3: Other 3byte/pixel type 7: disable color space convert"]
pub type OUT_COLOR_INPUT_SEL_CH_R = crate::FieldReader;
#[doc = "Field `OUT_COLOR_INPUT_SEL_CH` writer - Set first color convert process and input color type 0: RGB565 to RGB888 1: YUV422 to YUV444 2: Other 2byte/pixel type 3: Other 3byte/pixel type 7: disable color space convert"]
pub type OUT_COLOR_INPUT_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Set final color convert process and output type 0: RGB888 to RGB565 1: YUV444 to YUV422 2: output directly"]
    #[inline(always)]
    pub fn out_color_output_sel_ch(&self) -> OUT_COLOR_OUTPUT_SEL_CH_R {
        OUT_COLOR_OUTPUT_SEL_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable generic color convert modlue between color input & color output, need to configure parameter."]
    #[inline(always)]
    pub fn out_color_3b_proc_en_ch(&self) -> OUT_COLOR_3B_PROC_EN_CH_R {
        OUT_COLOR_3B_PROC_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Set first color convert process and input color type 0: RGB565 to RGB888 1: YUV422 to YUV444 2: Other 2byte/pixel type 3: Other 3byte/pixel type 7: disable color space convert"]
    #[inline(always)]
    pub fn out_color_input_sel_ch(&self) -> OUT_COLOR_INPUT_SEL_CH_R {
        OUT_COLOR_INPUT_SEL_CH_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_COLOR_CONVERT_CH")
            .field("out_color_output_sel_ch", &self.out_color_output_sel_ch())
            .field("out_color_3b_proc_en_ch", &self.out_color_3b_proc_en_ch())
            .field("out_color_input_sel_ch", &self.out_color_input_sel_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Set final color convert process and output type 0: RGB888 to RGB565 1: YUV444 to YUV422 2: output directly"]
    #[inline(always)]
    pub fn out_color_output_sel_ch(
        &mut self,
    ) -> OUT_COLOR_OUTPUT_SEL_CH_W<'_, OUT_COLOR_CONVERT_CH_SPEC> {
        OUT_COLOR_OUTPUT_SEL_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable generic color convert modlue between color input & color output, need to configure parameter."]
    #[inline(always)]
    pub fn out_color_3b_proc_en_ch(
        &mut self,
    ) -> OUT_COLOR_3B_PROC_EN_CH_W<'_, OUT_COLOR_CONVERT_CH_SPEC> {
        OUT_COLOR_3B_PROC_EN_CH_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Set first color convert process and input color type 0: RGB565 to RGB888 1: YUV422 to YUV444 2: Other 2byte/pixel type 3: Other 3byte/pixel type 7: disable color space convert"]
    #[inline(always)]
    pub fn out_color_input_sel_ch(
        &mut self,
    ) -> OUT_COLOR_INPUT_SEL_CH_W<'_, OUT_COLOR_CONVERT_CH_SPEC> {
        OUT_COLOR_INPUT_SEL_CH_W::new(self, 3)
    }
}
#[doc = "Configures the tx color convert of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_convert_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_convert_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_COLOR_CONVERT_CH_SPEC;
impl crate::RegisterSpec for OUT_COLOR_CONVERT_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_color_convert_ch::R`](R) reader structure"]
impl crate::Readable for OUT_COLOR_CONVERT_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_color_convert_ch::W`](W) writer structure"]
impl crate::Writable for OUT_COLOR_CONVERT_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_COLOR_CONVERT_CH%s to value 0x38"]
impl crate::Resettable for OUT_COLOR_CONVERT_CH_SPEC {
    const RESET_VALUE: u32 = 0x38;
}
