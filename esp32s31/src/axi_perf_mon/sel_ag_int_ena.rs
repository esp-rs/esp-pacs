#[doc = "Register `SEL_AG_INT_ENA` reader"]
pub type R = crate::R<SEL_AG_INT_ENA_SPEC>;
#[doc = "Register `SEL_AG_INT_ENA` writer"]
pub type W = crate::W<SEL_AG_INT_ENA_SPEC>;
#[doc = "Field `SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA` reader - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA` writer - Write 1 to enable instantaneous bandwidth test data under limit int"]
pub type SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag0_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag1_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag2_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag3_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag4_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag5_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag6_record_cnt_under_limit_int_ena(
        &self,
    ) -> SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_R {
        SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG_INT_ENA")
            .field(
                "sel_ag0_record_cnt_under_limit_int_ena",
                &self.sel_ag0_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag1_record_cnt_under_limit_int_ena",
                &self.sel_ag1_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag2_record_cnt_under_limit_int_ena",
                &self.sel_ag2_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag3_record_cnt_under_limit_int_ena",
                &self.sel_ag3_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag4_record_cnt_under_limit_int_ena",
                &self.sel_ag4_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag5_record_cnt_under_limit_int_ena",
                &self.sel_ag5_record_cnt_under_limit_int_ena(),
            )
            .field(
                "sel_ag6_record_cnt_under_limit_int_ena",
                &self.sel_ag6_record_cnt_under_limit_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag0_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG0_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag1_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG1_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag2_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG2_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag3_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG3_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag4_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG4_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag5_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG5_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable instantaneous bandwidth test data under limit int"]
    #[inline(always)]
    pub fn sel_ag6_record_cnt_under_limit_int_ena(
        &mut self,
    ) -> SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_W<'_, SEL_AG_INT_ENA_SPEC> {
        SEL_AG6_RECORD_CNT_UNDER_LIMIT_INT_ENA_W::new(self, 6)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG_INT_ENA_SPEC;
impl crate::RegisterSpec for SEL_AG_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag_int_ena::R`](R) reader structure"]
impl crate::Readable for SEL_AG_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag_int_ena::W`](W) writer structure"]
impl crate::Writable for SEL_AG_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG_INT_ENA to value 0"]
impl crate::Resettable for SEL_AG_INT_ENA_SPEC {}
