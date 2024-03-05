#[doc = "Register `SW_INT_CLR` writer"]
pub type W = crate::W<SW_INT_CLR_SPEC>;
#[doc = "Field `LP_SW_INT_CLR` writer - need_des"]
pub type LP_SW_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sw_int_clr(&mut self) -> LP_SW_INT_CLR_W<SW_INT_CLR_SPEC> {
        LP_SW_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_INT_CLR_SPEC;
impl crate::RegisterSpec for SW_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sw_int_clr::W`](W) writer structure"]
impl crate::Writable for SW_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_INT_CLR to value 0"]
impl crate::Resettable for SW_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
