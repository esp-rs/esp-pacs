///Register `CORE_0_REGION_PMS_CONSTRAIN_3` reader
pub type R = crate::R<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
///Register `CORE_0_REGION_PMS_CONSTRAIN_3` writer
pub type W = crate::W<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC>;
///Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_0` reader - Region 0 start address for core0.
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R = crate::FieldReader<u32>;
///Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_0` writer - Region 0 start address for core0.
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - Region 0 start address for core0.
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_0(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_REGION_PMS_CONSTRAIN_3")
            .field(
                "core_0_region_pms_constrain_addr_0",
                &self.core_0_region_pms_constrain_addr_0(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:29 - Region 0 start address for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_region_pms_constrain_addr_0(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W<CORE_0_REGION_PMS_CONSTRAIN_3_SPEC> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_0_W::new(self, 0)
    }
}
/**Core0 region permission register 3.

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_REGION_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_region_pms_constrain_3::R`](R) reader structure
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {}
///`write(|w| ..)` method takes [`core_0_region_pms_constrain_3::W`](W) writer structure
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_3 to value 0
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
