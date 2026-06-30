#[doc = "Register `OUT_COLOR_PARAM2_CH%s` reader"]
pub type R = crate::R<OUT_COLOR_PARAM2_CH_SPEC>;
#[doc = "Register `OUT_COLOR_PARAM2_CH%s` writer"]
pub type W = crate::W<OUT_COLOR_PARAM2_CH_SPEC>;
#[doc = "Field `OUT_COLOR_PARAM_M0_CH` reader - Set first 2 parameter of midium significant byte of pending 3 bytes"]
pub type OUT_COLOR_PARAM_M0_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_COLOR_PARAM_M0_CH` writer - Set first 2 parameter of midium significant byte of pending 3 bytes"]
pub type OUT_COLOR_PARAM_M0_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - Set first 2 parameter of midium significant byte of pending 3 bytes"]
    #[inline(always)]
    pub fn out_color_param_m0_ch(&self) -> OUT_COLOR_PARAM_M0_CH_R {
        OUT_COLOR_PARAM_M0_CH_R::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_COLOR_PARAM2_CH")
            .field("out_color_param_m0_ch", &self.out_color_param_m0_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:20 - Set first 2 parameter of midium significant byte of pending 3 bytes"]
    #[inline(always)]
    pub fn out_color_param_m0_ch(
        &mut self,
    ) -> OUT_COLOR_PARAM_M0_CH_W<'_, OUT_COLOR_PARAM2_CH_SPEC> {
        OUT_COLOR_PARAM_M0_CH_W::new(self, 0)
    }
}
#[doc = "Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param2_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param2_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_COLOR_PARAM2_CH_SPEC;
impl crate::RegisterSpec for OUT_COLOR_PARAM2_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_color_param2_ch::R`](R) reader structure"]
impl crate::Readable for OUT_COLOR_PARAM2_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_color_param2_ch::W`](W) writer structure"]
impl crate::Writable for OUT_COLOR_PARAM2_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_COLOR_PARAM2_CH%s to value 0x001e_712a"]
impl crate::Resettable for OUT_COLOR_PARAM2_CH_SPEC {
    const RESET_VALUE: u32 = 0x001e_712a;
}
