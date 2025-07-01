#[doc = "Register `VAD_PARAM3` reader"]
pub type R = crate::R<VAD_PARAM3_SPEC>;
#[doc = "Register `VAD_PARAM3` writer"]
pub type W = crate::W<VAD_PARAM3_SPEC>;
#[doc = "Field `PARAM_NOISE_SPE_UP0` reader - VAD parameter"]
pub type PARAM_NOISE_SPE_UP0_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP0` writer - VAD parameter"]
pub type PARAM_NOISE_SPE_UP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP1` reader - VAD parameter"]
pub type PARAM_NOISE_SPE_UP1_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_UP1` writer - VAD parameter"]
pub type PARAM_NOISE_SPE_UP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up0(&self) -> PARAM_NOISE_SPE_UP0_R {
        PARAM_NOISE_SPE_UP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up1(&self) -> PARAM_NOISE_SPE_UP1_R {
        PARAM_NOISE_SPE_UP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM3")
            .field("param_noise_spe_up0", &self.param_noise_spe_up0())
            .field("param_noise_spe_up1", &self.param_noise_spe_up1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up0(&mut self) -> PARAM_NOISE_SPE_UP0_W<VAD_PARAM3_SPEC> {
        PARAM_NOISE_SPE_UP0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_up1(&mut self) -> PARAM_NOISE_SPE_UP1_W<VAD_PARAM3_SPEC> {
        PARAM_NOISE_SPE_UP1_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM3_SPEC;
impl crate::RegisterSpec for VAD_PARAM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param3::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param3::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAD_PARAM3 to value 0x7d71_7fdf"]
impl crate::Resettable for VAD_PARAM3_SPEC {
    const RESET_VALUE: u32 = 0x7d71_7fdf;
}
