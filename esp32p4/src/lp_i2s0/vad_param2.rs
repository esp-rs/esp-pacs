#[doc = "Register `VAD_PARAM2` reader"]
pub type R = crate::R<VAD_PARAM2_SPEC>;
#[doc = "Register `VAD_PARAM2` writer"]
pub type W = crate::W<VAD_PARAM2_SPEC>;
#[doc = "Field `PARAM_NOISE_AMP_DOWN` reader - VAD parameter"]
pub type PARAM_NOISE_AMP_DOWN_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_AMP_DOWN` writer - VAD parameter"]
pub type PARAM_NOISE_AMP_DOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_NOISE_AMP_UP` reader - VAD parameter"]
pub type PARAM_NOISE_AMP_UP_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_NOISE_AMP_UP` writer - VAD parameter"]
pub type PARAM_NOISE_AMP_UP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_amp_down(&self) -> PARAM_NOISE_AMP_DOWN_R {
        PARAM_NOISE_AMP_DOWN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_noise_amp_up(&self) -> PARAM_NOISE_AMP_UP_R {
        PARAM_NOISE_AMP_UP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM2")
            .field("param_noise_amp_down", &self.param_noise_amp_down().bits())
            .field("param_noise_amp_up", &self.param_noise_amp_up().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_PARAM2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_amp_down(&mut self) -> PARAM_NOISE_AMP_DOWN_W<VAD_PARAM2_SPEC> {
        PARAM_NOISE_AMP_DOWN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_noise_amp_up(&mut self) -> PARAM_NOISE_AMP_UP_W<VAD_PARAM2_SPEC> {
        PARAM_NOISE_AMP_UP_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_param2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM2_SPEC;
impl crate::RegisterSpec for VAD_PARAM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param2::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param2::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM2 to value 0x7eb8_6666"]
impl crate::Resettable for VAD_PARAM2_SPEC {
    const RESET_VALUE: u32 = 0x7eb8_6666;
}
