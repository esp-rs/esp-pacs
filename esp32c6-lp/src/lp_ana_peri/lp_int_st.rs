#[doc = "Register `LP_INT_ST` reader"]
pub struct R(crate::R<LP_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOD_MODE0_LP_INT_ST` reader - need_des"]
pub type BOD_MODE0_LP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_lp_int_st(&self) -> BOD_MODE0_LP_INT_ST_R {
        BOD_MODE0_LP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field(
                "bod_mode0_lp_int_st",
                &format_args!("{}", self.bod_mode0_lp_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_st](index.html) module"]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_int_st::R](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
