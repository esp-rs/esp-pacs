#[doc = "Register `VAD_PARAM4` reader"]
pub type R = crate::R<VAD_PARAM4_SPEC>;
#[doc = "Register `VAD_PARAM4` writer"]
pub type W = crate::W<VAD_PARAM4_SPEC>;
#[doc = "Field `PARAM_NOISE_SPE_DOWN` reader - VAD parameter"]
pub type PARAM_NOISE_SPE_DOWN_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_SPE_DOWN` writer - VAD parameter"]
pub type PARAM_NOISE_SPE_DOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_MEAN_DOWN` reader - VAD parameter"]
pub type PARAM_NOISE_MEAN_DOWN_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_MEAN_DOWN` writer - VAD parameter"]
pub type PARAM_NOISE_MEAN_DOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_spe_down(&self) -> PARAM_NOISE_SPE_DOWN_R {
        PARAM_NOISE_SPE_DOWN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_mean_down(&self) -> PARAM_NOISE_MEAN_DOWN_R {
        PARAM_NOISE_MEAN_DOWN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM4")
            .field("param_noise_spe_down", &self.param_noise_spe_down())
            .field("param_noise_mean_down", &self.param_noise_mean_down())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_spe_down(&mut self) -> PARAM_NOISE_SPE_DOWN_W<VAD_PARAM4_SPEC> {
        PARAM_NOISE_SPE_DOWN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_mean_down(&mut self) -> PARAM_NOISE_MEAN_DOWN_W<VAD_PARAM4_SPEC> {
        PARAM_NOISE_MEAN_DOWN_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_param4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM4_SPEC;
impl crate::RegisterSpec for VAD_PARAM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param4::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param4::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM4 to value 0x799a_6666"]
impl crate::Resettable for VAD_PARAM4_SPEC {
    const RESET_VALUE: u32 = 0x799a_6666;
}
