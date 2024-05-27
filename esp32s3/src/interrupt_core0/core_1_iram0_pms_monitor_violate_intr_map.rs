///Register `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader
pub type R = crate::R<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
///Register `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer
pub type W = crate::W<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>;
///Field `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` reader - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R = crate::FieldReader;
///Field `CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP` writer - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt
pub type CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt
    #[inline(always)]
    pub fn core_1_iram0_pms_monitor_violate_intr_map(
        &self,
    ) -> CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R {
        CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP")
            .field(
                "core_1_iram0_pms_monitor_violate_intr_map",
                &self.core_1_iram0_pms_monitor_violate_intr_map(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map core1_IRam0_pms_monitor_violatile interrupt to one of core0's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn core_1_iram0_pms_monitor_violate_intr_map(
        &mut self,
    ) -> CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W<CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>
    {
        CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_W::new(self, 0)
    }
}
/**core1_IRam0_pms_monitor_violatile interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_violate_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_violate_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_iram0_pms_monitor_violate_intr_map::R`](R) reader structure
impl crate::Readable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`core_1_iram0_pms_monitor_violate_intr_map::W`](W) writer structure
impl crate::Writable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP to value 0x10
impl crate::Resettable for CORE_1_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
