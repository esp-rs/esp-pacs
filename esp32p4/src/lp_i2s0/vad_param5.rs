#[doc = "Register `VAD_PARAM5` reader"]
pub type R = crate::R<VAD_PARAM5_SPEC>;
#[doc = "Register `VAD_PARAM5` writer"]
pub type W = crate::W<VAD_PARAM5_SPEC>;
#[doc = "Field `PARAM_NOISE_MEAN_UP0` reader - VAD parameter"]
pub type PARAM_NOISE_MEAN_UP0_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_MEAN_UP0` writer - VAD parameter"]
pub type PARAM_NOISE_MEAN_UP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_MEAN_UP1` reader - VAD parameter"]
pub type PARAM_NOISE_MEAN_UP1_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_MEAN_UP1` writer - VAD parameter"]
pub type PARAM_NOISE_MEAN_UP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_mean_up0(&self) -> PARAM_NOISE_MEAN_UP0_R {
        PARAM_NOISE_MEAN_UP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_mean_up1(&self) -> PARAM_NOISE_MEAN_UP1_R {
        PARAM_NOISE_MEAN_UP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM5")
            .field(
                "param_noise_mean_up0",
                &format_args!("{}", self.param_noise_mean_up0().bits()),
            )
            .field(
                "param_noise_mean_up1",
                &format_args!("{}", self.param_noise_mean_up1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_PARAM5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_mean_up0(&mut self) -> PARAM_NOISE_MEAN_UP0_W<VAD_PARAM5_SPEC> {
        PARAM_NOISE_MEAN_UP0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_mean_up1(&mut self) -> PARAM_NOISE_MEAN_UP1_W<VAD_PARAM5_SPEC> {
        PARAM_NOISE_MEAN_UP1_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_param5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM5_SPEC;
impl crate::RegisterSpec for VAD_PARAM5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param5::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param5::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM5 to value 0x7c28_7d71"]
impl crate::Resettable for VAD_PARAM5_SPEC {
    const RESET_VALUE: u32 = 0x7c28_7d71;
}
