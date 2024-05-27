///Register `CORE_0_REGION_PMS_CONSTRAIN_8` reader
pub type R = crate::R<CORE_0_REGION_PMS_CONSTRAIN_8_SPEC>;
///Register `CORE_0_REGION_PMS_CONSTRAIN_8` writer
pub type W = crate::W<CORE_0_REGION_PMS_CONSTRAIN_8_SPEC>;
///Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_5` reader - Region 4 end address and Region 5 start address for core0.
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_R = crate::FieldReader<u32>;
///Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_5` writer - Region 4 end address and Region 5 start address for core0.
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Region 4 end address and Region 5 start address for core0.
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_5(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_REGION_PMS_CONSTRAIN_8")
            .field(
                "core_0_region_pms_constrain_addr_5",
                &self.core_0_region_pms_constrain_addr_5(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Region 4 end address and Region 5 start address for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_region_pms_constrain_addr_5(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_W<CORE_0_REGION_PMS_CONSTRAIN_8_SPEC> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_5_W::new(self, 0)
    }
}
/**Core0 region permission register 8.

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_REGION_PMS_CONSTRAIN_8_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_region_pms_constrain_8::R`](R) reader structure
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_8_SPEC {}
///`write(|w| ..)` method takes [`core_0_region_pms_constrain_8::W`](W) writer structure
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_8 to value 0
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_8_SPEC {
    const RESET_VALUE: u32 = 0;
}
