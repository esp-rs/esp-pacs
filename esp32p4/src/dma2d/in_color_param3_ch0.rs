#[doc = "Register `IN_COLOR_PARAM3_CH0` reader"]
pub type R = crate::R<IN_COLOR_PARAM3_CH0_SPEC>;
#[doc = "Register `IN_COLOR_PARAM3_CH0` writer"]
pub type W = crate::W<IN_COLOR_PARAM3_CH0_SPEC>;
#[doc = "Field `IN_COLOR_PARAM_M1_CH` reader - Set last 2 parameter of midium significant byte of pending 3 bytes"]
pub type IN_COLOR_PARAM_M1_CH_R = crate::FieldReader<u32>;
#[doc = "Field `IN_COLOR_PARAM_M1_CH` writer - Set last 2 parameter of midium significant byte of pending 3 bytes"]
pub type IN_COLOR_PARAM_M1_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Set last 2 parameter of midium significant byte of pending 3 bytes"]
    #[inline(always)]
    pub fn in_color_param_m1_ch(&self) -> IN_COLOR_PARAM_M1_CH_R {
        IN_COLOR_PARAM_M1_CH_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_COLOR_PARAM3_CH0")
            .field("in_color_param_m1_ch", &self.in_color_param_m1_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Set last 2 parameter of midium significant byte of pending 3 bytes"]
    #[inline(always)]
    pub fn in_color_param_m1_ch(&mut self) -> IN_COLOR_PARAM_M1_CH_W<'_, IN_COLOR_PARAM3_CH0_SPEC> {
        IN_COLOR_PARAM_M1_CH_W::new(self, 0)
    }
}
#[doc = "Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param3_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param3_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_COLOR_PARAM3_CH0_SPEC;
impl crate::RegisterSpec for IN_COLOR_PARAM3_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_color_param3_ch0::R`](R) reader structure"]
impl crate::Readable for IN_COLOR_PARAM3_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_color_param3_ch0::W`](W) writer structure"]
impl crate::Writable for IN_COLOR_PARAM3_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_COLOR_PARAM3_CH0 to value 0x021e_4f30"]
impl crate::Resettable for IN_COLOR_PARAM3_CH0_SPEC {
    const RESET_VALUE: u32 = 0x021e_4f30;
}
