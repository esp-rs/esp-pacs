#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_0` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_0` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LOCK` reader - backup_bus_pms_constrain_lock"]
pub type BACKUP_BUS_PMS_CONSTRAIN_LOCK_R = crate::BitReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_LOCK` writer - backup_bus_pms_constrain_lock"]
pub type BACKUP_BUS_PMS_CONSTRAIN_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - backup_bus_pms_constrain_lock"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_lock(&self) -> BACKUP_BUS_PMS_CONSTRAIN_LOCK_R {
        BACKUP_BUS_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_0")
            .field(
                "backup_bus_pms_constrain_lock",
                &format_args!("{}", self.backup_bus_pms_constrain_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - backup_bus_pms_constrain_lock"]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_lock(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_LOCK_W<BACKUP_BUS_PMS_CONSTRAIN_0_SPEC, 0> {
        BACKUP_BUS_PMS_CONSTRAIN_LOCK_W::new(self)
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
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_0::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_0::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_0 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
