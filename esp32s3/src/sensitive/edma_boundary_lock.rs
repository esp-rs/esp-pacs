#[doc = "Register `EDMA_BOUNDARY_LOCK` reader"]
pub type R = crate::R<EDMA_BOUNDARY_LOCK_SPEC>;
#[doc = "Register `EDMA_BOUNDARY_LOCK` writer"]
pub type W = crate::W<EDMA_BOUNDARY_LOCK_SPEC>;
#[doc = "Field `EDMA_BOUNDARY_LOCK` reader - Set 1 to lock EDMA boundary registers."]
pub type EDMA_BOUNDARY_LOCK_R = crate::BitReader;
#[doc = "Field `EDMA_BOUNDARY_LOCK` writer - Set 1 to lock EDMA boundary registers."]
pub type EDMA_BOUNDARY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock EDMA boundary registers."]
    #[inline(always)]
    pub fn edma_boundary_lock(&self) -> EDMA_BOUNDARY_LOCK_R {
        EDMA_BOUNDARY_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_BOUNDARY_LOCK")
            .field(
                "edma_boundary_lock",
                &format_args!("{}", self.edma_boundary_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDMA_BOUNDARY_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock EDMA boundary registers."]
    #[inline(always)]
    #[must_use]
    pub fn edma_boundary_lock(&mut self) -> EDMA_BOUNDARY_LOCK_W<EDMA_BOUNDARY_LOCK_SPEC> {
        EDMA_BOUNDARY_LOCK_W::new(self, 0)
    }
}
#[doc = "EDMA boundary lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_boundary_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_boundary_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDMA_BOUNDARY_LOCK_SPEC;
impl crate::RegisterSpec for EDMA_BOUNDARY_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edma_boundary_lock::R`](R) reader structure"]
impl crate::Readable for EDMA_BOUNDARY_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edma_boundary_lock::W`](W) writer structure"]
impl crate::Writable for EDMA_BOUNDARY_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDMA_BOUNDARY_LOCK to value 0"]
impl crate::Resettable for EDMA_BOUNDARY_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
