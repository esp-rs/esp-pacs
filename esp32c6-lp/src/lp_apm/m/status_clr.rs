///Register `STATUS_CLR` writer
pub type W = crate::W<STATUS_CLR_SPEC>;
///Field `REGION_STATUS_CLR` writer - Clear exception status
pub type REGION_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear exception status
    #[inline(always)]
    #[must_use]
    pub fn region_status_clr(&mut self) -> REGION_STATUS_CLR_W<STATUS_CLR_SPEC> {
        REGION_STATUS_CLR_W::new(self, 0)
    }
}
/**M0 status clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_CLR_SPEC;
impl crate::RegisterSpec for STATUS_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`status_clr::W`](W) writer structure
impl crate::Writable for STATUS_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS_CLR to value 0
impl crate::Resettable for STATUS_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
