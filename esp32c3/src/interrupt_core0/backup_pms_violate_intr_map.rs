#[doc = "Register `BACKUP_PMS_VIOLATE_INTR_MAP` reader"]
pub type R = crate::R<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Register `BACKUP_PMS_VIOLATE_INTR_MAP` writer"]
pub type W = crate::W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "Field `BACKUP_PMS_VIOLATE_INTR_MAP` reader - reg_core0_backup_pms_violate_intr_map"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `BACKUP_PMS_VIOLATE_INTR_MAP` writer - reg_core0_backup_pms_violate_intr_map"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_backup_pms_violate_intr_map"]
    #[inline(always)]
    pub fn backup_pms_violate_intr_map(&self) -> BACKUP_PMS_VIOLATE_INTR_MAP_R {
        BACKUP_PMS_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_PMS_VIOLATE_INTR_MAP")
            .field(
                "backup_pms_violate_intr_map",
                &format_args!("{}", self.backup_pms_violate_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_backup_pms_violate_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn backup_pms_violate_intr_map(
        &mut self,
    ) -> BACKUP_PMS_VIOLATE_INTR_MAP_W<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC> {
        BACKUP_PMS_VIOLATE_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_pms_violate_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_pms_violate_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_PMS_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_pms_violate_intr_map::R`](R) reader structure"]
impl crate::Readable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_pms_violate_intr_map::W`](W) writer structure"]
impl crate::Writable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_PMS_VIOLATE_INTR_MAP to value 0"]
impl crate::Resettable for BACKUP_PMS_VIOLATE_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
