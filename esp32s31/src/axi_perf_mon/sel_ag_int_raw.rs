#[doc = "Register `SEL_AG_INT_RAW` reader"]
pub type R = crate::R<SEL_AG_INT_RAW_SPEC>;
#[doc = "Register `SEL_AG_INT_RAW` writer"]
pub type W = crate::W<SEL_AG_INT_RAW_SPEC>;
#[doc = "Field `SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW` reader - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW` writer - The raw interrupt status of instantaneous bandwidth test data under limit int"]
pub type SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag0_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag1_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag2_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag3_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag4_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag5_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag6_record_cnt_under_limit_int_raw(
        &self,
    ) -> SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_R {
        SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG_INT_RAW")
            .field(
                "sel_ag0_record_cnt_under_limit_int_raw",
                &self.sel_ag0_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag1_record_cnt_under_limit_int_raw",
                &self.sel_ag1_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag2_record_cnt_under_limit_int_raw",
                &self.sel_ag2_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag3_record_cnt_under_limit_int_raw",
                &self.sel_ag3_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag4_record_cnt_under_limit_int_raw",
                &self.sel_ag4_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag5_record_cnt_under_limit_int_raw",
                &self.sel_ag5_record_cnt_under_limit_int_raw(),
            )
            .field(
                "sel_ag6_record_cnt_under_limit_int_raw",
                &self.sel_ag6_record_cnt_under_limit_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag0_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag1_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag2_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag3_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag4_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag5_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag6_record_cnt_under_limit_int_raw(
        &mut self,
    ) -> SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_W<'_, SEL_AG_INT_RAW_SPEC> {
        SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_RAW_W::new(self, 6)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG_INT_RAW_SPEC;
impl crate::RegisterSpec for SEL_AG_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag_int_raw::R`](R) reader structure"]
impl crate::Readable for SEL_AG_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag_int_raw::W`](W) writer structure"]
impl crate::Writable for SEL_AG_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG_INT_RAW to value 0"]
impl crate::Resettable for SEL_AG_INT_RAW_SPEC {}
