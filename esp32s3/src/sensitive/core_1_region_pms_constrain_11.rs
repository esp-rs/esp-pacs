#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_11` reader"]
pub type R = crate::R<CORE_1_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_11` writer"]
pub type W = crate::W<CORE_1_REGION_PMS_CONSTRAIN_11_SPEC>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_8` reader - Region 7 end address and Region 8 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_8` writer - Region 7 end address and Region 8 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 7 end address and Region 8 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_8(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_11")
            .field(
                "core_1_region_pms_constrain_addr_8",
                &self.core_1_region_pms_constrain_addr_8(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 7 end address and Region 8 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_8(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_W<CORE_1_REGION_PMS_CONSTRAIN_11_SPEC> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_8_W::new(self, 0)
    }
}
#[doc = "core1 region permission register 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_region_pms_constrain_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_region_pms_constrain_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_REGION_PMS_CONSTRAIN_11_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_region_pms_constrain_11::R`](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_region_pms_constrain_11::W`](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_11 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_11_SPEC {}
