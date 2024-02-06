#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `COMP0_NEG_INT_ST` reader - analog comparator pos edge interrupt status"]
pub type COMP0_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP0_POS_INT_ST` reader - analog comparator neg edge interrupt status"]
pub type COMP0_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP0_ALL_INT_ST` reader - analog comparator neg or pos edge interrupt status"]
pub type COMP0_ALL_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP1_NEG_INT_ST` reader - analog comparator pos edge interrupt status"]
pub type COMP1_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP1_POS_INT_ST` reader - analog comparator neg edge interrupt status"]
pub type COMP1_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP1_ALL_INT_ST` reader - analog comparator neg or pos edge interrupt status"]
pub type COMP1_ALL_INT_ST_R = crate::BitReader;
#[doc = "Field `BISTOK_INT_ST` reader - pad bistok interrupt status"]
pub type BISTOK_INT_ST_R = crate::BitReader;
#[doc = "Field `BISTFAIL_INT_ST` reader - pad bistfail interrupt status"]
pub type BISTFAIL_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn comp0_neg_int_st(&self) -> COMP0_NEG_INT_ST_R {
        COMP0_NEG_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn comp0_pos_int_st(&self) -> COMP0_POS_INT_ST_R {
        COMP0_POS_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn comp0_all_int_st(&self) -> COMP0_ALL_INT_ST_R {
        COMP0_ALL_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn comp1_neg_int_st(&self) -> COMP1_NEG_INT_ST_R {
        COMP1_NEG_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn comp1_pos_int_st(&self) -> COMP1_POS_INT_ST_R {
        COMP1_POS_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn comp1_all_int_st(&self) -> COMP1_ALL_INT_ST_R {
        COMP1_ALL_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pad bistok interrupt status"]
    #[inline(always)]
    pub fn bistok_int_st(&self) -> BISTOK_INT_ST_R {
        BISTOK_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pad bistfail interrupt status"]
    #[inline(always)]
    pub fn bistfail_int_st(&self) -> BISTFAIL_INT_ST_R {
        BISTFAIL_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "comp0_neg_int_st",
                &format_args!("{}", self.comp0_neg_int_st().bit()),
            )
            .field(
                "comp0_pos_int_st",
                &format_args!("{}", self.comp0_pos_int_st().bit()),
            )
            .field(
                "comp0_all_int_st",
                &format_args!("{}", self.comp0_all_int_st().bit()),
            )
            .field(
                "comp1_neg_int_st",
                &format_args!("{}", self.comp1_neg_int_st().bit()),
            )
            .field(
                "comp1_pos_int_st",
                &format_args!("{}", self.comp1_pos_int_st().bit()),
            )
            .field(
                "comp1_all_int_st",
                &format_args!("{}", self.comp1_all_int_st().bit()),
            )
            .field(
                "bistok_int_st",
                &format_args!("{}", self.bistok_int_st().bit()),
            )
            .field(
                "bistfail_int_st",
                &format_args!("{}", self.bistfail_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "analog comparator interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
