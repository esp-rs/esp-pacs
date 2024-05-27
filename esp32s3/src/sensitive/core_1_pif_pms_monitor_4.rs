///Register `CORE_1_PIF_PMS_MONITOR_4` reader
pub type R = crate::R<CORE_1_PIF_PMS_MONITOR_4_SPEC>;
///Register `CORE_1_PIF_PMS_MONITOR_4` writer
pub type W = crate::W<CORE_1_PIF_PMS_MONITOR_4_SPEC>;
///Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR` reader - Set 1 to clear interrupt that core1 initiate unsupported access type.
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_R = crate::BitReader;
///Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR` writer - Set 1 to clear interrupt that core1 initiate unsupported access type.
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN` reader - Set 1 to enable interrupt that core1 initiate unsupported access type.
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_R = crate::BitReader;
///Field `CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN` writer - Set 1 to enable interrupt that core1 initiate unsupported access type.
pub type CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to clear interrupt that core1 initiate unsupported access type.
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_nonword_violate_clr(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_R {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable interrupt that core1 initiate unsupported access type.
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_nonword_violate_en(
        &self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_R {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_MONITOR_4")
            .field(
                "core_1_pif_pms_monitor_nonword_violate_clr",
                &self.core_1_pif_pms_monitor_nonword_violate_clr(),
            )
            .field(
                "core_1_pif_pms_monitor_nonword_violate_en",
                &self.core_1_pif_pms_monitor_nonword_violate_en(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to clear interrupt that core1 initiate unsupported access type.
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_monitor_nonword_violate_clr(
        &mut self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_W<CORE_1_PIF_PMS_MONITOR_4_SPEC> {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to enable interrupt that core1 initiate unsupported access type.
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_monitor_nonword_violate_en(
        &mut self,
    ) -> CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_W<CORE_1_PIF_PMS_MONITOR_4_SPEC> {
        CORE_1_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN_W::new(self, 1)
    }
}
/**core1 permission report register 4.

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_PIF_PMS_MONITOR_4_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_MONITOR_4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_pif_pms_monitor_4::R`](R) reader structure
impl crate::Readable for CORE_1_PIF_PMS_MONITOR_4_SPEC {}
///`write(|w| ..)` method takes [`core_1_pif_pms_monitor_4::W`](W) writer structure
impl crate::Writable for CORE_1_PIF_PMS_MONITOR_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_PIF_PMS_MONITOR_4 to value 0x03
impl crate::Resettable for CORE_1_PIF_PMS_MONITOR_4_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
