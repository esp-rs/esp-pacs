///Register `BACKUP_BUS_PMS_MONITOR_1` reader
pub type R = crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
///Register `BACKUP_BUS_PMS_MONITOR_1` writer
pub type W = crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
///Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` reader - Set 1 to clear interrupt that BackUp initiate illegal access.
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R = crate::BitReader;
///Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` writer - Set 1 to clear interrupt that BackUp initiate illegal access.
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` reader - Set 1 to enable interrupt that BackUp initiate illegal access.
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R = crate::BitReader;
///Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` writer - Set 1 to enable interrupt that BackUp initiate illegal access.
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to clear interrupt that BackUp initiate illegal access.
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_clr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable interrupt that BackUp initiate illegal access.
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_en(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_MONITOR_1")
            .field(
                "backup_bus_pms_monitor_violate_clr",
                &self.backup_bus_pms_monitor_violate_clr(),
            )
            .field(
                "backup_bus_pms_monitor_violate_en",
                &self.backup_bus_pms_monitor_violate_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to clear interrupt that BackUp initiate illegal access.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_monitor_violate_clr(
        &mut self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<BACKUP_BUS_PMS_MONITOR_1_SPEC> {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to enable interrupt that BackUp initiate illegal access.
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_monitor_violate_en(
        &mut self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<BACKUP_BUS_PMS_MONITOR_1_SPEC> {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W::new(self, 1)
    }
}
/**BackUp permission report register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_monitor_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_monitor_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BACKUP_BUS_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`backup_bus_pms_monitor_1::R`](R) reader structure
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_1_SPEC {}
///`write(|w| ..)` method takes [`backup_bus_pms_monitor_1::W`](W) writer structure
impl crate::Writable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BACKUP_BUS_PMS_MONITOR_1 to value 0x03
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
