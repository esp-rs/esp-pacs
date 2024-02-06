#[doc = "Register `LP_ANA_LP_INT_ST` reader"]
pub type R = crate::R<LP_ANA_LP_INT_ST_SPEC>;
#[doc = "Field `LP_ANA_BOD_MODE0_LP_INT_ST` reader - need_des"]
pub type LP_ANA_BOD_MODE0_LP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_bod_mode0_lp_int_st(&self) -> LP_ANA_BOD_MODE0_LP_INT_ST_R {
        LP_ANA_BOD_MODE0_LP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_LP_INT_ST")
            .field(
                "lp_ana_bod_mode0_lp_int_st",
                &format_args!("{}", self.lp_ana_bod_mode0_lp_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_LP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_lp_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_ANA_LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_ANA_LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_ANA_LP_INT_ST to value 0"]
impl crate::Resettable for LP_ANA_LP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
