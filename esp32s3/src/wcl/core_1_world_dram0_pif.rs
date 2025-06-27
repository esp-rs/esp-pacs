#[doc = "Register `Core_1_World_DRam0_PIF` reader"]
pub type R = crate::R<CORE_1_WORLD_DRAM0_PIF_SPEC>;
#[doc = "Register `Core_1_World_DRam0_PIF` writer"]
pub type W = crate::W<CORE_1_WORLD_DRAM0_PIF_SPEC>;
#[doc = "Field `CORE_1_WORLD_DRAM0_PIF` reader - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_1_WORLD_DRAM0_PIF_R = crate::FieldReader;
#[doc = "Field `CORE_1_WORLD_DRAM0_PIF` writer - this field is used to read current world of Dram0 bus and PIF bus"]
pub type CORE_1_WORLD_DRAM0_PIF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn core_1_world_dram0_pif(&self) -> CORE_1_WORLD_DRAM0_PIF_R {
        CORE_1_WORLD_DRAM0_PIF_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_World_DRam0_PIF")
            .field("core_1_world_dram0_pif", &self.core_1_world_dram0_pif())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn core_1_world_dram0_pif(
        &mut self,
    ) -> CORE_1_WORLD_DRAM0_PIF_W<CORE_1_WORLD_DRAM0_PIF_SPEC> {
        CORE_1_WORLD_DRAM0_PIF_W::new(self, 0)
    }
}
#[doc = "Core_1 dram0 and PIF world register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_world_dram0_pif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_world_dram0_pif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_WORLD_DRAM0_PIF_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_DRAM0_PIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_world_dram0_pif::R`](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_DRAM0_PIF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_world_dram0_pif::W`](W) writer structure"]
impl crate::Writable for CORE_1_WORLD_DRAM0_PIF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_1_World_DRam0_PIF to value 0"]
impl crate::Resettable for CORE_1_WORLD_DRAM0_PIF_SPEC {}
