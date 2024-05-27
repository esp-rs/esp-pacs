///Register `CORE_X_IRAM0_PMS_CONSTRAIN_0` reader
pub type R = crate::R<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
///Register `CORE_X_IRAM0_PMS_CONSTRAIN_0` writer
pub type W = crate::W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC>;
///Field `CORE_X_IRAM0_PMS_CONSTRAIN_LOCK` reader - Set 1 to lock corex iram0 permission configuration register
pub type CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R = crate::BitReader;
///Field `CORE_X_IRAM0_PMS_CONSTRAIN_LOCK` writer - Set 1 to lock corex iram0 permission configuration register
pub type CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to lock corex iram0 permission configuration register
    #[inline(always)]
    pub fn core_x_iram0_pms_constrain_lock(&self) -> CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R {
        CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_PMS_CONSTRAIN_0")
            .field(
                "core_x_iram0_pms_constrain_lock",
                &self.core_x_iram0_pms_constrain_lock(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to lock corex iram0 permission configuration register
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_pms_constrain_lock(
        &mut self,
    ) -> CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W<CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC> {
        CORE_X_IRAM0_PMS_CONSTRAIN_LOCK_W::new(self, 0)
    }
}
/**corex iram0 permission configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_pms_constrain_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_pms_constrain_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_x_iram0_pms_constrain_0::R`](R) reader structure
impl crate::Readable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {}
///`write(|w| ..)` method takes [`core_x_iram0_pms_constrain_0::W`](W) writer structure
impl crate::Writable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_X_IRAM0_PMS_CONSTRAIN_0 to value 0
impl crate::Resettable for CORE_X_IRAM0_PMS_CONSTRAIN_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
