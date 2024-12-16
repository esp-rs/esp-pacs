#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_7` reader"]
pub type R = crate::R<CORE_0_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_7` writer"]
pub type W = crate::W<CORE_0_REGION_PMS_CONSTRAIN_7_SPEC>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_4` reader - Region 3 end address and Region 4 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_4` writer - Region 3 end address and Region 4 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 3 end address and Region 4 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_4(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_REGION_PMS_CONSTRAIN_7")
            .field(
                "core_0_region_pms_constrain_addr_4",
                &self.core_0_region_pms_constrain_addr_4(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 3 end address and Region 4 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_4(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_W<CORE_0_REGION_PMS_CONSTRAIN_7_SPEC> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_4_W::new(self, 0)
    }
}
#[doc = "Core0 region permission register 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_region_pms_constrain_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_REGION_PMS_CONSTRAIN_7_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_region_pms_constrain_7::R`](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_region_pms_constrain_7::W`](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_7 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
