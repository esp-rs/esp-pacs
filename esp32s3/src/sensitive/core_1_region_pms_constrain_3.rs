#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_3` reader"]
pub type R = crate::R<CORE_1_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_3` writer"]
pub type W = crate::W<CORE_1_REGION_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_0` reader - Region 0 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_0` writer - Region 0 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 0 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_0(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_3")
            .field(
                "core_1_region_pms_constrain_addr_0",
                &self.core_1_region_pms_constrain_addr_0(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 0 start address for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_addr_0(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_W<CORE_1_REGION_PMS_CONSTRAIN_3_SPEC> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_0_W::new(self, 0)
    }
}
#[doc = "core1 region permission register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_region_pms_constrain_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_REGION_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_region_pms_constrain_3::R`](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_region_pms_constrain_3::W`](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_3 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
