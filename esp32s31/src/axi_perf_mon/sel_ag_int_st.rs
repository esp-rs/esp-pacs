#[doc = "Register `SEL_AG_INT_ST` reader"]
pub type R = crate::R<SEL_AG_INT_ST_SPEC>;
#[doc = "Field `SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
#[doc = "Field `SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ST` reader - The masked interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag0_record_cnt_under_limit_int_st(&self) -> SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag1_record_cnt_under_limit_int_st(&self) -> SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag2_record_cnt_under_limit_int_st(&self) -> SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag3_record_cnt_under_limit_int_st(&self) -> SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag4_record_cnt_under_limit_int_st(&self) -> SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag5_record_cnt_under_limit_int_st(&self) -> SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag6_record_cnt_under_limit_int_st(&self) -> SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ST_R {
        SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG_INT_ST")
            .field(
                "sel_ag0_record_cnt_under_limit_int_st",
                &self.sel_ag0_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag1_record_cnt_under_limit_int_st",
                &self.sel_ag1_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag2_record_cnt_under_limit_int_st",
                &self.sel_ag2_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag3_record_cnt_under_limit_int_st",
                &self.sel_ag3_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag4_record_cnt_under_limit_int_st",
                &self.sel_ag4_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag5_record_cnt_under_limit_int_st",
                &self.sel_ag5_record_cnt_under_limit_int_st(),
            )
            .field(
                "sel_ag6_record_cnt_under_limit_int_st",
                &self.sel_ag6_record_cnt_under_limit_int_st(),
            )
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG_INT_ST_SPEC;
impl crate::RegisterSpec for SEL_AG_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag_int_st::R`](R) reader structure"]
impl crate::Readable for SEL_AG_INT_ST_SPEC {}
#[doc = "`reset()` method sets SEL_AG_INT_ST to value 0"]
impl crate::Resettable for SEL_AG_INT_ST_SPEC {}
