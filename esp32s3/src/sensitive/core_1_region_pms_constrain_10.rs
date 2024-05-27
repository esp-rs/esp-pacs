///Register `CORE_1_REGION_PMS_CONSTRAIN_10` reader
pub type R = crate::R<CORE_1_REGION_PMS_CONSTRAIN_10_SPEC>;
///Register `CORE_1_REGION_PMS_CONSTRAIN_10` writer
pub type W = crate::W<CORE_1_REGION_PMS_CONSTRAIN_10_SPEC>;
///Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_7` reader - Region 6 end address and Region 7 start address for core1.
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_R = crate::FieldReader<u32>;
///Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_7` writer - Region 6 end address and Region 7 start address for core1.
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Region 6 end address and Region 7 start address for core1.
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_7(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_10")
            .field(
                "core_1_region_pms_constrain_addr_7",
                &self.core_1_region_pms_constrain_addr_7(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Region 6 end address and Region 7 start address for core1.
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_addr_7(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_W<CORE_1_REGION_PMS_CONSTRAIN_10_SPEC> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_7_W::new(self, 0)
    }
}
/**core1 region permission register 10.

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_region_pms_constrain_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_REGION_PMS_CONSTRAIN_10_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_10_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_region_pms_constrain_10::R`](R) reader structure
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_10_SPEC {}
///`write(|w| ..)` method takes [`core_1_region_pms_constrain_10::W`](W) writer structure
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_10 to value 0
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_10_SPEC {
    const RESET_VALUE: u32 = 0;
}
