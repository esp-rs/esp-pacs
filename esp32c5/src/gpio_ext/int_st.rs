#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `COMP_NEG_0_INT_ST` reader - analog comparator pos edge interrupt status"]
pub type COMP_NEG_0_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP_POS_0_INT_ST` reader - analog comparator neg edge interrupt status"]
pub type COMP_POS_0_INT_ST_R = crate::BitReader;
#[doc = "Field `COMP_ALL_0_INT_ST` reader - analog comparator neg or pos edge interrupt status"]
pub type COMP_ALL_0_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn comp_neg_0_int_st(&self) -> COMP_NEG_0_INT_ST_R {
        COMP_NEG_0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn comp_pos_0_int_st(&self) -> COMP_POS_0_INT_ST_R {
        COMP_POS_0_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn comp_all_0_int_st(&self) -> COMP_ALL_0_INT_ST_R {
        COMP_ALL_0_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("comp_neg_0_int_st", &self.comp_neg_0_int_st())
            .field("comp_pos_0_int_st", &self.comp_pos_0_int_st())
            .field("comp_all_0_int_st", &self.comp_all_0_int_st())
            .finish()
    }
}
#[doc = "GPIO_EXT interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
