#[doc = "Register `TCM_INT_CLR` writer"]
pub type W = crate::W<TCM_INT_CLR_SPEC>;
#[doc = "Field `TCM_PARITY_ERR_INT_CLR` writer - need_des"]
pub type TCM_PARITY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TCM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn tcm_parity_err_int_clr(&mut self) -> TCM_PARITY_ERR_INT_CLR_W<TCM_INT_CLR_SPEC> {
        TCM_PARITY_ERR_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_INT_CLR_SPEC;
impl crate::RegisterSpec for TCM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcm_int_clr::W`](W) writer structure"]
impl crate::Writable for TCM_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INT_CLR to value 0"]
impl crate::Resettable for TCM_INT_CLR_SPEC {}
