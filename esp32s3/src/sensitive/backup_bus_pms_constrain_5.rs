#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_5` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_5` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR` reader - BackUp access rtcfast_spltaddr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR` writer - BackUp access rtcfast_spltaddr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W<'a, REG> =
    crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - BackUp access rtcfast_spltaddr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_spltaddr(
        &self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_5")
            .field(
                "backup_bus_pms_constrain_rtcfast_spltaddr",
                &self.backup_bus_pms_constrain_rtcfast_spltaddr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - BackUp access rtcfast_spltaddr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_spltaddr(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W<BACKUP_BUS_PMS_CONSTRAIN_5_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_SPLTADDR_W::new(self, 0)
    }
}
#[doc = "BackUp access peripherals permission configuration register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_bus_pms_constrain_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_5::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_5::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_5 to value 0x07ff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: u32 = 0x07ff;
}
