#[doc = "Register `SW_INT_ST` reader"]
pub type R = crate::R<SW_INT_ST_SPEC>;
#[doc = "Field `LP_SW_INT_ST` reader - need_des"]
pub type LP_SW_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sw_int_st(&self) -> LP_SW_INT_ST_R {
        LP_SW_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_INT_ST")
            .field(
                "lp_sw_int_st",
                &format_args!("{}", self.lp_sw_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_INT_ST_SPEC;
impl crate::RegisterSpec for SW_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_int_st::R`](R) reader structure"]
impl crate::Readable for SW_INT_ST_SPEC {}
#[doc = "`reset()` method sets SW_INT_ST to value 0"]
impl crate::Resettable for SW_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
