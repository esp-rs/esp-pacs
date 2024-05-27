///Register `CORE_0_VECBASE_OVERRIDE_0` reader
pub type R = crate::R<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
///Register `CORE_0_VECBASE_OVERRIDE_0` writer
pub type W = crate::W<CORE_0_VECBASE_OVERRIDE_0_SPEC>;
///Field `CORE_0_VECBASE_WORLD_MASK` reader - Set 1 to mask world, then only world0_value will work.
pub type CORE_0_VECBASE_WORLD_MASK_R = crate::BitReader;
///Field `CORE_0_VECBASE_WORLD_MASK` writer - Set 1 to mask world, then only world0_value will work.
pub type CORE_0_VECBASE_WORLD_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to mask world, then only world0_value will work.
    #[inline(always)]
    pub fn core_0_vecbase_world_mask(&self) -> CORE_0_VECBASE_WORLD_MASK_R {
        CORE_0_VECBASE_WORLD_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_VECBASE_OVERRIDE_0")
            .field(
                "core_0_vecbase_world_mask",
                &self.core_0_vecbase_world_mask(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to mask world, then only world0_value will work.
    #[inline(always)]
    #[must_use]
    pub fn core_0_vecbase_world_mask(
        &mut self,
    ) -> CORE_0_VECBASE_WORLD_MASK_W<CORE_0_VECBASE_OVERRIDE_0_SPEC> {
        CORE_0_VECBASE_WORLD_MASK_W::new(self, 0)
    }
}
/**core0 vecbase override configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_vecbase_override_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_vecbase_override_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_VECBASE_OVERRIDE_0_SPEC;
impl crate::RegisterSpec for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_vecbase_override_0::R`](R) reader structure
impl crate::Readable for CORE_0_VECBASE_OVERRIDE_0_SPEC {}
///`write(|w| ..)` method takes [`core_0_vecbase_override_0::W`](W) writer structure
impl crate::Writable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_VECBASE_OVERRIDE_0 to value 0x01
impl crate::Resettable for CORE_0_VECBASE_OVERRIDE_0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
