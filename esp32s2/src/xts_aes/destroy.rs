///Register `DESTROY` writer
pub type W = crate::W<DESTROY_SPEC>;
///Field `DESTROY` writer - Set to destroy encrypted result.
pub type DESTROY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTROY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set to destroy encrypted result.
    #[inline(always)]
    #[must_use]
    pub fn destroy(&mut self) -> DESTROY_W<DESTROY_SPEC> {
        DESTROY_W::new(self, 0)
    }
}
/**Destroys control

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destroy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DESTROY_SPEC;
impl crate::RegisterSpec for DESTROY_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`destroy::W`](W) writer structure
impl crate::Writable for DESTROY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DESTROY to value 0
impl crate::Resettable for DESTROY_SPEC {
    const RESET_VALUE: u32 = 0;
}
