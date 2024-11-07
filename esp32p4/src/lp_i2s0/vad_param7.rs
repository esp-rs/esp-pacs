#[doc = "Register `VAD_PARAM7` reader"]
pub type R = crate::R<VAD_PARAM7_SPEC>;
#[doc = "Register `VAD_PARAM7` writer"]
pub type W = crate::W<VAD_PARAM7_SPEC>;
#[doc = "Field `PARAM_THRES_UPD_BASE` reader - VAD parameter"]
pub type PARAM_THRES_UPD_BASE_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_THRES_UPD_BASE` writer - VAD parameter"]
pub type PARAM_THRES_UPD_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARAM_THRES_UPD_VARY` reader - VAD parameter"]
pub type PARAM_THRES_UPD_VARY_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_THRES_UPD_VARY` writer - VAD parameter"]
pub type PARAM_THRES_UPD_VARY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_thres_upd_base(&self) -> PARAM_THRES_UPD_BASE_R {
        PARAM_THRES_UPD_BASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_thres_upd_vary(&self) -> PARAM_THRES_UPD_VARY_R {
        PARAM_THRES_UPD_VARY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM7")
            .field("param_thres_upd_base", &self.param_thres_upd_base())
            .field("param_thres_upd_vary", &self.param_thres_upd_vary())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_thres_upd_base(&mut self) -> PARAM_THRES_UPD_BASE_W<VAD_PARAM7_SPEC> {
        PARAM_THRES_UPD_BASE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_thres_upd_vary(&mut self) -> PARAM_THRES_UPD_VARY_W<VAD_PARAM7_SPEC> {
        PARAM_THRES_UPD_VARY_W::new(self, 16)
    }
}
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_param7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vad_param7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM7_SPEC;
impl crate::RegisterSpec for VAD_PARAM7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param7::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param7::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM7 to value 0x0148_7eb8"]
impl crate::Resettable for VAD_PARAM7_SPEC {
    const RESET_VALUE: u32 = 0x0148_7eb8;
}
