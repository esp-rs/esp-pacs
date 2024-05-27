#[doc = "Register `AF_ENV_USER_TH_LUM` reader"]
pub type R = crate::R<AF_ENV_USER_TH_LUM_SPEC>;
#[doc = "Register `AF_ENV_USER_TH_LUM` writer"]
pub type W = crate::W<AF_ENV_USER_TH_LUM_SPEC>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_LUM` reader - this field configures user setup env detect lum threshold"]
pub type AF_ENV_USER_THRESHOLD_LUM_R = crate::FieldReader<u32>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_LUM` writer - this field configures user setup env detect lum threshold"]
pub type AF_ENV_USER_THRESHOLD_LUM_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - this field configures user setup env detect lum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_lum(&self) -> AF_ENV_USER_THRESHOLD_LUM_R {
        AF_ENV_USER_THRESHOLD_LUM_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_ENV_USER_TH_LUM")
            .field(
                "af_env_user_threshold_lum",
                &self.af_env_user_threshold_lum(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - this field configures user setup env detect lum threshold"]
    #[inline(always)]
    #[must_use]
    pub fn af_env_user_threshold_lum(
        &mut self,
    ) -> AF_ENV_USER_THRESHOLD_LUM_W<AF_ENV_USER_TH_LUM_SPEC> {
        AF_ENV_USER_THRESHOLD_LUM_W::new(self, 0)
    }
}
#[doc = "af monitor user lum threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_env_user_th_lum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_env_user_th_lum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_ENV_USER_TH_LUM_SPEC;
impl crate::RegisterSpec for AF_ENV_USER_TH_LUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_env_user_th_lum::R`](R) reader structure"]
impl crate::Readable for AF_ENV_USER_TH_LUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_env_user_th_lum::W`](W) writer structure"]
impl crate::Writable for AF_ENV_USER_TH_LUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_ENV_USER_TH_LUM to value 0"]
impl crate::Resettable for AF_ENV_USER_TH_LUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
