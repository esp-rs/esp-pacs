#[doc = "Register `VAD_PARAM0` reader"]
pub type R = crate::R<VAD_PARAM0_SPEC>;
#[doc = "Register `VAD_PARAM0` writer"]
pub type W = crate::W<VAD_PARAM0_SPEC>;
#[doc = "Field `PARAM_MIN_ENERGY` reader - VAD parameter"]
pub type PARAM_MIN_ENERGY_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_MIN_ENERGY` writer - VAD parameter"]
pub type PARAM_MIN_ENERGY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_INIT_FRAME_NUM` reader - VAD parameter"]
pub type PARAM_INIT_FRAME_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_INIT_FRAME_NUM` writer - VAD parameter"]
pub type PARAM_INIT_FRAME_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_min_energy(&self) -> PARAM_MIN_ENERGY_R {
        PARAM_MIN_ENERGY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:24 - VAD parameter"]
    #[inline(always)]
    pub fn param_init_frame_num(&self) -> PARAM_INIT_FRAME_NUM_R {
        PARAM_INIT_FRAME_NUM_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM0")
            .field("param_min_energy", &self.param_min_energy())
            .field("param_init_frame_num", &self.param_init_frame_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_min_energy(&mut self) -> PARAM_MIN_ENERGY_W<VAD_PARAM0_SPEC> {
        PARAM_MIN_ENERGY_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_init_frame_num(&mut self) -> PARAM_INIT_FRAME_NUM_W<VAD_PARAM0_SPEC> {
        PARAM_INIT_FRAME_NUM_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM0_SPEC;
impl crate::RegisterSpec for VAD_PARAM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param0::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param0::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM0 to value 0x00c8_1388"]
impl crate::Resettable for VAD_PARAM0_SPEC {
    const RESET_VALUE: u32 = 0x00c8_1388;
}
