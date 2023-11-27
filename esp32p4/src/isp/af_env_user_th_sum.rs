#[doc = "Register `AF_ENV_USER_TH_SUM` reader"]
pub type R = crate::R<AF_ENV_USER_TH_SUM_SPEC>;
#[doc = "Register `AF_ENV_USER_TH_SUM` writer"]
pub type W = crate::W<AF_ENV_USER_TH_SUM_SPEC>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_SUM` reader - this field configures user setup env detect sum threshold"]
pub type AF_ENV_USER_THRESHOLD_SUM_R = crate::FieldReader<u32>;
#[doc = "Field `AF_ENV_USER_THRESHOLD_SUM` writer - this field configures user setup env detect sum threshold"]
pub type AF_ENV_USER_THRESHOLD_SUM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - this field configures user setup env detect sum threshold"]
    #[inline(always)]
    pub fn af_env_user_threshold_sum(&self) -> AF_ENV_USER_THRESHOLD_SUM_R {
        AF_ENV_USER_THRESHOLD_SUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_ENV_USER_TH_SUM")
            .field(
                "af_env_user_threshold_sum",
                &format_args!("{}", self.af_env_user_threshold_sum().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_ENV_USER_TH_SUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - this field configures user setup env detect sum threshold"]
    #[inline(always)]
    #[must_use]
    pub fn af_env_user_threshold_sum(
        &mut self,
    ) -> AF_ENV_USER_THRESHOLD_SUM_W<AF_ENV_USER_TH_SUM_SPEC> {
        AF_ENV_USER_THRESHOLD_SUM_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "af monitor user sum threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_env_user_th_sum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_env_user_th_sum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_ENV_USER_TH_SUM_SPEC;
impl crate::RegisterSpec for AF_ENV_USER_TH_SUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_env_user_th_sum::R`](R) reader structure"]
impl crate::Readable for AF_ENV_USER_TH_SUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_env_user_th_sum::W`](W) writer structure"]
impl crate::Writable for AF_ENV_USER_TH_SUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AF_ENV_USER_TH_SUM to value 0"]
impl crate::Resettable for AF_ENV_USER_TH_SUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
