#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` reader - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` writer - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` reader - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` writer - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_l(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_h(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_6")
            .field(
                "backup_bus_pms_constrain_rtcfast_l",
                &self.backup_bus_pms_constrain_rtcfast_l(),
            )
            .field(
                "backup_bus_pms_constrain_rtcfast_h",
                &self.backup_bus_pms_constrain_rtcfast_h(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_l(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<'_, BACKUP_BUS_PMS_CONSTRAIN_6_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_h(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<'_, BACKUP_BUS_PMS_CONSTRAIN_6_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W::new(self, 3)
    }
}
#[doc = "BackUp access peripherals permission configuration register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_bus_pms_constrain_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_6::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_6::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_6 to value 0x3f"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
