#[doc = "Register `CORE_1_AREA_DRAM0_0_MAX` reader"]
pub type R = crate::R<CORE_1_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Register `CORE_1_AREA_DRAM0_0_MAX` writer"]
pub type W = crate::W<CORE_1_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_MAX` reader - Core1 dram0 region0 end addr"]
pub type CORE_1_AREA_DRAM0_0_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_MAX` writer - Core1 dram0 region0 end addr"]
pub type CORE_1_AREA_DRAM0_0_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core1 dram0 region0 end addr"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_max(&self) -> CORE_1_AREA_DRAM0_0_MAX_R {
        CORE_1_AREA_DRAM0_0_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_AREA_DRAM0_0_MAX")
            .field("core_1_area_dram0_0_max", &self.core_1_area_dram0_0_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Core1 dram0 region0 end addr"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_max(
        &mut self,
    ) -> CORE_1_AREA_DRAM0_0_MAX_W<CORE_1_AREA_DRAM0_0_MAX_SPEC> {
        CORE_1_AREA_DRAM0_0_MAX_W::new(self, 0)
    }
}
#[doc = "Core1 dram0 region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_dram0_0_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_dram0_0_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_AREA_DRAM0_0_MAX_SPEC;
impl crate::RegisterSpec for CORE_1_AREA_DRAM0_0_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_area_dram0_0_max::R`](R) reader structure"]
impl crate::Readable for CORE_1_AREA_DRAM0_0_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_area_dram0_0_max::W`](W) writer structure"]
impl crate::Writable for CORE_1_AREA_DRAM0_0_MAX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_AREA_DRAM0_0_MAX to value 0"]
impl crate::Resettable for CORE_1_AREA_DRAM0_0_MAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
