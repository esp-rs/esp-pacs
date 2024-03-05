#[doc = "Register `CORE_X_DRAM0_PMS_CONSTRAIN_0` reader"]
pub type R = crate::R<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Register `CORE_X_DRAM0_PMS_CONSTRAIN_0` writer"]
pub type W = crate::W<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_LOCK` reader - core_x_dram0_pms_constrain_lock"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_R = crate::BitReader;
#[doc = "Field `CORE_X_DRAM0_PMS_CONSTRAIN_LOCK` writer - core_x_dram0_pms_constrain_lock"]
pub type CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - core_x_dram0_pms_constrain_lock"]
    #[inline(always)]
    pub fn core_x_dram0_pms_constrain_lock(&self) -> CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_R {
        CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_DRAM0_PMS_CONSTRAIN_0")
            .field(
                "core_x_dram0_pms_constrain_lock",
                &format_args!("{}", self.core_x_dram0_pms_constrain_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - core_x_dram0_pms_constrain_lock"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_dram0_pms_constrain_lock(
        &mut self,
    ) -> CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_W<CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC> {
        CORE_X_DRAM0_PMS_CONSTRAIN_LOCK_W::new(self, 0)
    }
}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_dram0_pms_constrain_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_dram0_pms_constrain_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_dram0_pms_constrain_0::R`](R) reader structure"]
impl crate::Readable for CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_dram0_pms_constrain_0::W`](W) writer structure"]
impl crate::Writable for CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_X_DRAM0_PMS_CONSTRAIN_0 to value 0"]
impl crate::Resettable for CORE_X_DRAM0_PMS_CONSTRAIN_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
