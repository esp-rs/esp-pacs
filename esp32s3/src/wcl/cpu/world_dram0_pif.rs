#[doc = "Register `WORLD_DRAM0_PIF` reader"]
pub type R = crate::R<WORLD_DRAM0_PIF_SPEC>;
#[doc = "Register `WORLD_DRAM0_PIF` writer"]
pub type W = crate::W<WORLD_DRAM0_PIF_SPEC>;
#[doc = "Field `WORLD_DRAM0_PIF` reader - this field is used to read current world of Dram0 bus and PIF bus"]
pub type WORLD_DRAM0_PIF_R = crate::FieldReader;
#[doc = "Field `WORLD_DRAM0_PIF` writer - this field is used to read current world of Dram0 bus and PIF bus"]
pub type WORLD_DRAM0_PIF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn world_dram0_pif(&self) -> WORLD_DRAM0_PIF_R {
        WORLD_DRAM0_PIF_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORLD_DRAM0_PIF")
            .field("world_dram0_pif", &self.world_dram0_pif())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - this field is used to read current world of Dram0 bus and PIF bus"]
    #[inline(always)]
    pub fn world_dram0_pif(&mut self) -> WORLD_DRAM0_PIF_W<WORLD_DRAM0_PIF_SPEC> {
        WORLD_DRAM0_PIF_W::new(self, 0)
    }
}
#[doc = "Core_0 dram0 and PIF world register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_dram0_pif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`world_dram0_pif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORLD_DRAM0_PIF_SPEC;
impl crate::RegisterSpec for WORLD_DRAM0_PIF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`world_dram0_pif::R`](R) reader structure"]
impl crate::Readable for WORLD_DRAM0_PIF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`world_dram0_pif::W`](W) writer structure"]
impl crate::Writable for WORLD_DRAM0_PIF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORLD_DRAM0_PIF to value 0"]
impl crate::Resettable for WORLD_DRAM0_PIF_SPEC {}
