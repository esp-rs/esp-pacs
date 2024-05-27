///Register `TCM_INT_CLR` writer
pub type W = crate::W<TCM_INT_CLR_SPEC>;
///Field `TCM_PARITY_ERR_INT_CLR` writer - need_des
pub type TCM_PARITY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TCM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tcm_parity_err_int_clr(&mut self) -> TCM_PARITY_ERR_INT_CLR_W<TCM_INT_CLR_SPEC> {
        TCM_PARITY_ERR_INT_CLR_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcm_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCM_INT_CLR_SPEC;
impl crate::RegisterSpec for TCM_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tcm_int_clr::W`](W) writer structure
impl crate::Writable for TCM_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TCM_INT_CLR to value 0
impl crate::Resettable for TCM_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
