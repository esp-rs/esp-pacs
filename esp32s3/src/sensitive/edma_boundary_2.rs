#[doc = "Register `EDMA_BOUNDARY_2` reader"]
pub type R = crate::R<EDMA_BOUNDARY_2_SPEC>;
#[doc = "Register `EDMA_BOUNDARY_2` writer"]
pub type W = crate::W<EDMA_BOUNDARY_2_SPEC>;
#[doc = "Field `EDMA_BOUNDARY_2` reader - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_2_R = crate::FieldReader<u16>;
#[doc = "Field `EDMA_BOUNDARY_2` writer - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
pub type EDMA_BOUNDARY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_2(&self) -> EDMA_BOUNDARY_2_R {
        EDMA_BOUNDARY_2_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_BOUNDARY_2")
            .field("edma_boundary_2", &self.edma_boundary_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - This field is used to configure the boundary 2 of external RAM. The unit is 4K. For example, set this field to 0x80, then the address boundary 0 would be 0x3C080000 (0x3C000000 + 0x80 * 4K)."]
    #[inline(always)]
    pub fn edma_boundary_2(&mut self) -> EDMA_BOUNDARY_2_W<EDMA_BOUNDARY_2_SPEC> {
        EDMA_BOUNDARY_2_W::new(self, 0)
    }
}
#[doc = "EDMA boundary 2 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`edma_boundary_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edma_boundary_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDMA_BOUNDARY_2_SPEC;
impl crate::RegisterSpec for EDMA_BOUNDARY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edma_boundary_2::R`](R) reader structure"]
impl crate::Readable for EDMA_BOUNDARY_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edma_boundary_2::W`](W) writer structure"]
impl crate::Writable for EDMA_BOUNDARY_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDMA_BOUNDARY_2 to value 0x2000"]
impl crate::Resettable for EDMA_BOUNDARY_2_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
