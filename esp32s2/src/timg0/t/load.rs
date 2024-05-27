///Register `LOAD` writer
pub type W = crate::W<LOAD_SPEC>;
///Field `LOAD` writer - Write any value to trigger a timer %s time-base counter reload.
pub type LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Write any value to trigger a timer %s time-base counter reload.
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<LOAD_SPEC> {
        LOAD_W::new(self, 0)
    }
}
/**Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`load::W`](W) writer structure
impl crate::Writable for LOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOAD to value 0
impl crate::Resettable for LOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
