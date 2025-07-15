#[doc = "Register `AREA_PIF_1_MAX` reader"]
pub type R = crate::R<AREA_PIF_1_MAX_SPEC>;
#[doc = "Register `AREA_PIF_1_MAX` writer"]
pub type W = crate::W<AREA_PIF_1_MAX_SPEC>;
#[doc = "Field `CORE_0_AREA_PIF_1_MAX` reader - reg_core_0_area_pif_1_max"]
pub type CORE_0_AREA_PIF_1_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_PIF_1_MAX` writer - reg_core_0_area_pif_1_max"]
pub type CORE_0_AREA_PIF_1_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_area_pif_1_max"]
    #[inline(always)]
    pub fn core_0_area_pif_1_max(&self) -> CORE_0_AREA_PIF_1_MAX_R {
        CORE_0_AREA_PIF_1_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AREA_PIF_1_MAX")
            .field("core_0_area_pif_1_max", &self.core_0_area_pif_1_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core_0_area_pif_1_max"]
    #[inline(always)]
    pub fn core_0_area_pif_1_max(&mut self) -> CORE_0_AREA_PIF_1_MAX_W<AREA_PIF_1_MAX_SPEC> {
        CORE_0_AREA_PIF_1_MAX_W::new(self, 0)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`area_pif_1_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`area_pif_1_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AREA_PIF_1_MAX_SPEC;
impl crate::RegisterSpec for AREA_PIF_1_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`area_pif_1_max::R`](R) reader structure"]
impl crate::Readable for AREA_PIF_1_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`area_pif_1_max::W`](W) writer structure"]
impl crate::Writable for AREA_PIF_1_MAX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AREA_PIF_1_MAX to value 0"]
impl crate::Resettable for AREA_PIF_1_MAX_SPEC {}
